//! Installing, turning on and off, reviewing and removing plugins against real folders.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};

use bonsai_api::replay::{Fixture, Interaction, RecordedRequest, RecordedResponse, Replay};
use bonsai_api::{Base, RateLimiter, Secret};
use bonsai_core::account::{AccountList, AccountMeta, CookieStore, MemoryCookies};
use bonsai_core::cache::{Cache, Kind};
use bonsai_core::commands::{CommandId, PluginCmd, Tier};
use bonsai_core::launcher::RecordingOpener;
use bonsai_core::plugins::{PluginInfo, Status};
use bonsai_core::{AccountId, Core, CoreConfig, Paths, StoreData};
use parking_lot::Mutex;

fn start(dir: &Path) -> Core {
    let cfg = CoreConfig {
        paths: Paths::in_dir(dir),
        base: Base::Replay("http://127.0.0.1:9".into()),
        cookies: Arc::new(MemoryCookies::default()),
        opener: Arc::new(RecordingOpener::default()),
        limiter: Arc::new(RateLimiter::default()),
        background: false,
        quick_poll: Duration::from_millis(10),
    };
    Core::start(cfg, Arc::new(|| {}))
}

/// Writes a plugin folder named `id` under `parent`.
fn plugin(parent: &Path, id: &str, permissions: &str, main: &str) -> std::path::PathBuf {
    let dir = parent.join(id);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("plugin.toml"),
        format!(
            "id = \"{id}\"\nname = \"Test {id}\"\nversion = \"1.0.0\"\nentry = \"main.luau\"\n\
             permissions = [{permissions}]\nactions = [\"user.unfriend\", \"account.remove\", \"nav.home\"]\n"
        ),
    )
    .unwrap();
    std::fs::write(dir.join("main.luau"), main).unwrap();
    dir
}

const MAIN: &str = r#"
bonsai.command { id = "hello", title = "Say hello", run = function() return { toast = "hi" } end }
"#;

fn wait(core: &Core, what: &str, f: impl Fn(&[PluginInfo]) -> bool) -> Vec<PluginInfo> {
    let start = Instant::now();
    loop {
        let p = core.plugins();
        if f(&p) {
            return p;
        }
        assert!(
            start.elapsed() < Duration::from_secs(10),
            "timed out waiting for {what}: {p:#?}; toasts: {:?}",
            core.store().toasts
        );
        std::thread::sleep(Duration::from_millis(20));
    }
}

fn status<'a>(p: &'a [PluginInfo], id: &str) -> Option<&'a Status> {
    p.iter().find(|x| x.id == id).map(|x| &x.status)
}

fn running(p: &[PluginInfo], id: &str) -> bool {
    matches!(status(p, id), Some(Status::Running { .. }))
}

#[test]
fn install_run_disable_remove() {
    let home = tempfile::tempdir().unwrap();
    let src = tempfile::tempdir().unwrap();
    let dir = plugin(src.path(), "hello", "\"read.public\"", MAIN);
    let core = start(home.path());

    core.stage_plugin_install(dir.to_str().unwrap(), false).unwrap();
    let pending = core.store().plugin_install.clone().unwrap();
    assert_eq!(pending.manifest.id, "hello");
    // Tier 4 and non-writes are refused before the user sees the dialog.
    assert_eq!(pending.manifest.actions, vec!["user.unfriend"]);
    let refused: Vec<_> = pending.manifest.refused.iter().map(|r| r.what.as_str()).collect();
    assert_eq!(refused, vec!["account.remove", "nav.home"]);
    assert_eq!(pending.added, vec!["read.public", "user.unfriend"]);
    core.finish_plugin_install(Vec::new()).unwrap();
    assert!(core.store().plugin_install.is_none());

    let p = wait(&core, "the plugin to run", |p| running(p, "hello"));
    let Some(Status::Running { commands, .. }) = status(&p, "hello") else {
        unreachable!()
    };
    assert_eq!(commands[0].title, "Say hello");
    assert_eq!(core.plugin_commands().len(), 1);
    let installed = home.path().join("config/plugins/hello");
    assert!(installed.join("main.luau").is_file());
    assert!(!p[0].unpacked);

    // Settings survive a restart.
    drop(core);
    let core = start(home.path());
    wait(&core, "the plugin to start again", |p| running(p, "hello"));

    core.set_plugin_enabled("hello", false).unwrap();
    wait(&core, "the plugin to stop", |p| {
        status(p, "hello") == Some(&Status::Off)
    });
    assert!(core.plugin_commands().is_empty());

    core.remove_plugin("hello").unwrap();
    wait(&core, "the plugin to go", |p| p.is_empty());
    assert!(!installed.exists());
    // The source folder is untouched.
    assert!(dir.join("main.luau").is_file());
}

#[test]
fn unpacked_plugins_load_in_place_and_reload() {
    let home = tempfile::tempdir().unwrap();
    let src = tempfile::tempdir().unwrap();
    let dir = plugin(src.path(), "dev", "", MAIN);
    let core = start(home.path());
    core.stage_plugin_install(dir.join("plugin.toml").to_str().unwrap(), true)
        .unwrap();
    core.finish_plugin_install(Vec::new()).unwrap();
    wait(&core, "the plugin to run", |p| running(p, "dev"));
    assert!(!home.path().join("config/plugins/dev").exists());

    // A broken edit fails the plugin; fixing it and reloading brings it back.
    std::fs::write(dir.join("main.luau"), "error('broken')").unwrap();
    core.reload_plugin("dev").unwrap();
    wait(
        &core,
        "the plugin to fail",
        |p| matches!(status(p, "dev"), Some(Status::Failed(e)) if e.contains("broken")),
    );
    std::fs::write(dir.join("main.luau"), MAIN).unwrap();
    core.reload_plugin("dev").unwrap();
    wait(&core, "the plugin to run again", |p| running(p, "dev"));

    // Installing a copy of a plugin that's loaded unpacked is refused.
    assert!(core.stage_plugin_install(dir.to_str().unwrap(), false).is_err());

    // Removing an unpacked plugin forgets it and leaves its folder alone.
    core.remove_plugin("dev").unwrap();
    wait(&core, "the plugin to go", |p| p.is_empty());
    assert!(dir.join("main.luau").is_file());
}

#[test]
fn asking_for_more_waits_for_review() {
    let home = tempfile::tempdir().unwrap();
    let src = tempfile::tempdir().unwrap();
    let dir = plugin(src.path(), "grow", "\"read.public\"", MAIN);
    let core = start(home.path());
    core.stage_plugin_install(dir.to_str().unwrap(), true).unwrap();
    core.finish_plugin_install(Vec::new()).unwrap();
    wait(&core, "the plugin to run", |p| running(p, "grow"));

    // The manifest changes on disk to ask for account reads.
    plugin(src.path(), "grow", "\"read.public\", \"read.account\"", MAIN);
    core.reload_plugin("grow").unwrap();
    wait(&core, "the plugin to wait for review", |p| {
        status(p, "grow") == Some(&Status::NeedsReview(vec!["read.account".into()]))
    });
    assert!(core.plugin_commands().is_empty());

    // Turning it on opens the dialog instead, listing only what's new.
    core.set_plugin_enabled("grow", true).unwrap();
    let pending = core.store().plugin_install.clone().unwrap();
    assert_eq!(pending.added, vec!["read.account"]);
    assert_eq!(pending.replaces.as_deref(), Some("1.0.0"));
    core.finish_plugin_install(Vec::new()).unwrap();
    wait(&core, "the plugin to run again", |p| running(p, "grow"));
}

#[test]
fn broken_folders_show_up_failed() {
    let home = tempfile::tempdir().unwrap();
    let plugins = home.path().join("config/plugins");
    std::fs::create_dir_all(plugins.join("no-manifest")).unwrap();
    let wrong = plugin(&plugins, "right", "", MAIN);
    std::fs::rename(&wrong, plugins.join("wrong")).unwrap();
    let core = start(home.path());
    let p = wait(&core, "both folders", |p| p.len() == 2);
    for x in &p {
        assert!(matches!(x.status, Status::Failed(_)), "{x:?}");
        assert!(!x.enabled);
    }
    assert!(core.set_plugin_enabled("missing", true).is_err());
}

const ME: AccountId = AccountId(1);
const COOKIE: &str = "_|WARNING:-DO-NOT-SHARE-THIS.--Sharing-this-will-allow-someone-to-log-in-as-you-and-to-steal-your-ROBUX-and-items.|_FAKEFAKEFAKEFAKEFAKEFAKEFAKEFAKE";

const RUNS: &str = r#"
bonsai.command { id = "hello", title = "Say hello", run = function() return { toast = "hi" } end }
bonsai.command { id = "unfriend", title = "Unfriend", arg = "user", context = "user", run = function(ctx, id)
  return { actions = { { id = "user.unfriend", arg = tostring(id) } } }
end }
bonsai.command { id = "fav", title = "Favorite", run = function()
  return { actions = { { id = "market.favorite", arg = "item:1" } } }
end }
bonsai.command { id = "open", title = "Open", run = function() return { open = { kind = "user", id = 5 } } end }
bonsai.command { id = "who", title = "Who", run = function(ctx) return { toast = tostring(ctx.account) } end }
bonsai.command { id = "slow", title = "Slow", run = function() while true do end end }
"#;

fn wait_store(core: &Core, what: &str, f: impl Fn(&StoreData) -> bool) {
    let start = Instant::now();
    while !f(&core.store()) {
        assert!(
            start.elapsed() < Duration::from_secs(10),
            "timed out waiting for {what}; toasts: {:?}",
            core.store().toasts
        );
        std::thread::sleep(Duration::from_millis(20));
    }
}

fn toasted(core: &Core, text: &str) -> bool {
    core.store().toasts.iter().any(|t| t.text.contains(text))
}

type Ran = Arc<Mutex<Vec<(AccountId, CommandId, Option<String>)>>>;

/// A core with one account and the `runs` plugin installed and allowed to use it.
fn running_plugin(home: &Path, src: &Path) -> (Core, Ran) {
    AccountList {
        accounts: vec![AccountMeta {
            id: ME,
            name: "TestUser".into(),
            display_name: "Test User".into(),
            ..Default::default()
        }],
        active: Some(ME),
        previous: None,
    }
    .save(&Paths::in_dir(home).accounts_file())
    .unwrap();
    let dir = src.join("runs");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("plugin.toml"),
        "id = \"runs\"\nname = \"Runs\"\nentry = \"main.luau\"\nactions = [\"user.unfriend\", \"market.favorite\"]\n",
    )
    .unwrap();
    std::fs::write(dir.join("main.luau"), RUNS).unwrap();
    let core = start(home);
    let ran: Ran = Arc::default();
    let r = ran.clone();
    core.set_plugin_runner(Arc::new(move |a, id, arg| {
        r.lock().push((a, id, arg));
        Ok(())
    }));
    core.stage_plugin_install(dir.to_str().unwrap(), false).unwrap();
    core.finish_plugin_install(vec![ME]).unwrap();
    wait(&core, "the plugin to run", |p| running(p, "runs"));
    (core, ran)
}

fn cmd(id: &str) -> PluginCmd {
    PluginCmd {
        plugin: "runs".into(),
        id: id.into(),
    }
}

#[test]
fn commands_toast_open_and_propose_actions() {
    let home = tempfile::tempdir().unwrap();
    let src = tempfile::tempdir().unwrap();
    let (core, ran) = running_plugin(home.path(), src.path());
    let titles: Vec<_> = core
        .plugin_commands()
        .iter()
        .map(|c| c.command.title.clone())
        .collect();
    assert!(titles.contains(&"Say hello".to_owned()));

    core.run_plugin_command(&cmd("hello"), None, Some(ME), None)
        .unwrap();
    wait_store(&core, "the toast", |_| toasted(&core, "Runs: hi"));

    core.run_plugin_command(&cmd("open"), None, Some(ME), None)
        .unwrap();
    wait_store(&core, "the open", |s| s.plugin_open.is_some());
    let open = core.store().plugin_open.clone().unwrap();
    assert_eq!((open.command, open.arg.as_str()), (CommandId::OpenProfile, "5"));

    // Tier 2 waits for the user. The argument comes from the page it ran on.
    core.run_plugin_command(&cmd("unfriend"), None, Some(ME), Some("user:7"))
        .unwrap();
    wait_store(&core, "the action dialog", |s| s.plugin_actions.is_some());
    let pending = core.store().plugin_actions.clone().unwrap();
    assert_eq!(pending.account, ME);
    assert_eq!(pending.actions[0].id, CommandId::Unfriend);
    assert_eq!(pending.actions[0].arg.as_deref(), Some("7"));
    assert_eq!(pending.actions[0].tier, Tier::Social);
    assert!(ran.lock().is_empty());
    core.confirm_plugin_actions().unwrap();
    wait_store(&core, "the action to run", |_| {
        toasted(&core, "Runs: Ran 1 action.")
    });
    assert_eq!(ran.lock()[0], (ME, CommandId::Unfriend, Some("7".into())));

    // Cancelling runs nothing.
    core.run_plugin_command(&cmd("unfriend"), Some("8".into()), Some(ME), None)
        .unwrap();
    wait_store(&core, "the action dialog", |s| s.plugin_actions.is_some());
    core.cancel_plugin_actions();
    assert!(core.store().plugin_actions.is_none());

    // Tier 1 runs straight away.
    core.run_plugin_command(&cmd("fav"), None, Some(ME), None)
        .unwrap();
    wait_store(&core, "the favorite", |_| ran.lock().len() == 2);
    assert_eq!(
        ran.lock()[1],
        (ME, CommandId::FavoriteItem, Some("item:1".into()))
    );
    assert!(core.store().plugin_actions.is_none());
}

#[test]
fn commands_are_refused_without_grants_and_time_out() {
    let home = tempfile::tempdir().unwrap();
    let src = tempfile::tempdir().unwrap();
    let (core, ran) = running_plugin(home.path(), src.path());

    core.run_plugin_command(&cmd("slow"), None, Some(ME), None)
        .unwrap();
    wait_store(&core, "the timeout", |_| toasted(&core, "Runs: Took longer than"));

    core.run_plugin_command(&cmd("who"), None, Some(ME), None)
        .unwrap();
    wait_store(&core, "the account", |_| toasted(&core, "Runs: 1"));

    // Without the account it sees 0 and can't act.
    core.set_plugin_account("runs", ME, false).unwrap();
    wait(&core, "the grant to go", |p| p[0].accounts.is_empty());
    core.run_plugin_command(&cmd("who"), None, Some(ME), None)
        .unwrap();
    wait_store(&core, "no account", |_| toasted(&core, "Runs: 0"));
    core.run_plugin_command(&cmd("fav"), None, Some(ME), None)
        .unwrap();
    wait_store(&core, "the refusal", |_| {
        toasted(&core, "may not use this account")
    });

    // A pending batch is checked again when confirmed.
    core.set_plugin_account("runs", ME, true).unwrap();
    wait(&core, "the grant", |p| !p[0].accounts.is_empty());
    core.run_plugin_command(&cmd("unfriend"), Some("9".into()), Some(ME), None)
        .unwrap();
    wait_store(&core, "the action dialog", |s| s.plugin_actions.is_some());
    core.set_plugin_enabled("runs", false).unwrap();
    wait(&core, "the plugin to stop", |p| p[0].status == Status::Off);
    assert!(core.confirm_plugin_actions().is_err());
    assert!(
        core.run_plugin_command(&cmd("hello"), None, Some(ME), None)
            .is_err()
    );
    assert!(ran.lock().is_empty());
}

const READS: &str = r#"
bonsai.command { id = "user", title = "User", run = function(ctx)
  local u = ctx.user(5)
  return { toast = u.display_name .. " " .. tostring(u.created) .. " " .. tostring(u.friends) }
end }
bonsai.command { id = "presence", title = "Presence", run = function(ctx)
  local p = ctx.presence({ 9 })[1]
  return { toast = p.status .. " " .. p.location }
end }
bonsai.command { id = "friends", title = "Friends", run = function(ctx)
  return { toast = "friends " .. #ctx.friends() }
end }
"#;

fn get(host: &str, path: &str, body: serde_json::Value) -> Interaction {
    serve("GET", host, path, body)
}

fn serve(method: &str, host: &str, path: &str, body: serde_json::Value) -> Interaction {
    Interaction {
        request: RecordedRequest {
            method: method.into(),
            host: host.into(),
            path: path.into(),
            ..Default::default()
        },
        response: RecordedResponse {
            status: 200,
            body,
            ..Default::default()
        },
    }
}

#[test]
fn reads_check_grants_and_use_the_cache() {
    use serde_json::json;
    let rt = tokio::runtime::Runtime::new().unwrap();
    // Each answers once, so a second fetch of the user would 404.
    let fixture = Fixture {
        source: "synthetic".into(),
        interactions: vec![
            get(
                "users.roblox.com",
                "/v1/users/5",
                json!({"id":5,"name":"five","displayName":"Five","created":"2020-01-02T00:00:00Z","hasVerifiedBadge":false,"isBanned":false}),
            ),
            get(
                "friends.roblox.com",
                "/v1/users/5/friends/count",
                json!({"count":3}),
            ),
            get(
                "friends.roblox.com",
                "/v1/users/5/followers/count",
                json!({"count":4}),
            ),
            get(
                "friends.roblox.com",
                "/v1/users/5/followings/count",
                json!({"count":6}),
            ),
            serve(
                "POST",
                "presence.roblox.com",
                "/v1/presence/users",
                json!({"userPresences":[{"userId":9,"userPresenceType":2,"lastLocation":"Somewhere"}]}),
            ),
        ],
        ..Default::default()
    };
    let replay = rt.block_on(Replay::from_fixture(&fixture));

    let home = tempfile::tempdir().unwrap();
    let src = tempfile::tempdir().unwrap();
    let paths = Paths::in_dir(home.path());
    AccountList {
        accounts: vec![AccountMeta {
            id: ME,
            name: "TestUser".into(),
            display_name: "Test User".into(),
            ..Default::default()
        }],
        active: Some(ME),
        previous: None,
    }
    .save(&paths.accounts_file())
    .unwrap();
    std::fs::create_dir_all(&paths.cache).unwrap();
    Cache::open(&paths.db_file())
        .unwrap()
        .put(Kind::Friends, ME, "friends", &vec![1_u64, 156]);
    let cookies = Arc::new(MemoryCookies::default());
    cookies.set(ME, &Secret::new(COOKIE)).unwrap();
    let core = Core::start(
        CoreConfig {
            paths,
            base: Base::Replay(replay.server.uri()),
            cookies,
            opener: Arc::new(RecordingOpener::default()),
            limiter: Arc::new(RateLimiter::default()),
            background: false,
            quick_poll: Duration::from_millis(10),
        },
        Arc::new(|| {}),
    );
    let dir = src.path().join("reads");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("plugin.toml"),
        "id = \"reads\"\nname = \"Reads\"\nentry = \"main.luau\"\npermissions = [\"read.public\", \"read.account\"]\n",
    )
    .unwrap();
    std::fs::write(dir.join("main.luau"), READS).unwrap();
    core.stage_plugin_install(dir.to_str().unwrap(), false).unwrap();
    core.finish_plugin_install(vec![ME]).unwrap();
    wait(&core, "the plugin to run", |p| running(p, "reads"));
    let run = |id: &str| {
        let c = PluginCmd {
            plugin: "reads".into(),
            id: id.into(),
        };
        core.run_plugin_command(&c, None, Some(ME), None).unwrap();
    };

    // The same text replaces its toast, so look for a newer one.
    let newest = |text: &str| {
        core.store()
            .toasts
            .iter()
            .filter(|t| t.text.contains(text))
            .map(|t| t.id)
            .max()
            .unwrap_or(0)
    };
    const USER: &str = "Reads: Five 1577923200 3";
    run("user");
    wait_store(&core, "the user", |_| newest(USER) > 0);
    let first = newest(USER);
    run("user");
    wait_store(&core, "the cached user", |_| newest(USER) > first);
    run("presence");
    wait_store(&core, "the presence", |_| {
        toasted(&core, "Reads: in_game Somewhere")
    });
    run("friends");
    wait_store(&core, "the friends", |_| toasted(&core, "Reads: friends 2"));

    // Without the account, private reads are refused and public ones still work.
    core.set_plugin_account("reads", ME, false).unwrap();
    wait(&core, "the grant to go", |p| p[0].accounts.is_empty());
    run("friends");
    wait_store(&core, "the refusal", |_| {
        toasted(&core, "may not use this account")
    });
    let before = newest(USER);
    run("user");
    wait_store(&core, "the public read", |_| newest(USER) > before);
}

const WATCH: &str = r#"
bonsai.on("chat", function(ctx, e)
  local n = (ctx.store.get("chats") or 0) + 1
  ctx.store.set("chats", n)
  if n == 1 then
    return { alert = { title = "Chat", body = "from " .. tostring(e.from) .. " in " .. e.conversation } }
  end
  -- Even tier 1 waits for the user when nobody ran a command.
  return { actions = { { id = "market.favorite", arg = "item:1" } } }
end)
bonsai.command { id = "count", title = "Count", run = function(ctx)
  return { toast = tostring(ctx.store.get("chats")) .. " " .. ctx.settings.greeting .. " " .. tostring(ctx.settings.loud) }
end }
bonsai.command { id = "own", title = "Own", run = function(ctx)
  return { toast = "own " .. tostring(ctx.store.get("chats")) }
end }
"#;

#[test]
fn events_storage_and_settings() {
    use bonsai_api::realtime::Event;
    let home = tempfile::tempdir().unwrap();
    let src = tempfile::tempdir().unwrap();
    AccountList {
        accounts: vec![AccountMeta {
            id: ME,
            name: "TestUser".into(),
            display_name: "Test User".into(),
            ..Default::default()
        }],
        active: Some(ME),
        previous: None,
    }
    .save(&Paths::in_dir(home.path()).accounts_file())
    .unwrap();
    let dir = src.path().join("watch");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("plugin.toml"),
        r#"id = "watch"
name = "Watch"
entry = "main.luau"
permissions = ["read.account", "notify"]
actions = ["market.favorite"]

[[settings]]
key = "greeting"
kind = "text"
default = "hi"

[[settings]]
key = "loud"
kind = "bool"
"#,
    )
    .unwrap();
    std::fs::write(dir.join("main.luau"), WATCH).unwrap();
    let core = start(home.path());
    let ran: Ran = Arc::default();
    let r = ran.clone();
    core.set_plugin_runner(Arc::new(move |a, id, arg| {
        r.lock().push((a, id, arg));
        Ok(())
    }));
    core.stage_plugin_install(dir.to_str().unwrap(), false).unwrap();
    core.finish_plugin_install(vec![ME]).unwrap();
    let p = wait(&core, "the plugin to run", |p| running(p, "watch"));
    let Some(Status::Running { events, timers, .. }) = status(&p, "watch") else {
        unreachable!()
    };
    assert_eq!(
        (events.as_slice(), timers.as_slice()),
        (&["chat".to_owned()][..], &[][..])
    );
    let cmd = |id: &str| PluginCmd {
        plugin: "watch".into(),
        id: id.into(),
    };

    // A chat message reaches the plugin, which keeps a count and raises a desktop alert.
    let chat = Event {
        account: ME,
        namespace: "ChatNotifications".into(),
        detail: serde_json::json!({"Type": "NewMessage", "ConversationId": "conv-1", "SenderUserId": 7}),
    };
    core.apply_realtime(&chat);
    let start = Instant::now();
    let alerts = loop {
        // Bonsai's own chat alert comes too.
        let a: Vec<_> = core
            .take_alerts()
            .into_iter()
            .filter(|a| a.kind == "plugins")
            .collect();
        if !a.is_empty() {
            break a;
        }
        assert!(
            start.elapsed() < Duration::from_secs(10),
            "no alert; toasts: {:?}",
            core.store().toasts
        );
        std::thread::sleep(Duration::from_millis(20));
    };
    assert_eq!(alerts[0].kind, "plugins");
    assert_eq!(alerts[0].title, "Watch: Chat");
    assert_eq!(alerts[0].body, "from 7 in conv-1");

    // The second time it proposes a tier 1 action, which waits in the dialog.
    core.apply_realtime(&chat);
    wait_store(&core, "the action dialog", |s| s.plugin_actions.is_some());
    assert!(ran.lock().is_empty());
    core.cancel_plugin_actions();

    // Storage is per account, and kept on disk.
    core.run_plugin_command(&cmd("count"), None, Some(ME), None)
        .unwrap();
    wait_store(&core, "the count", |_| toasted(&core, "Watch: 2 hi false"));
    let file = home.path().join("config/plugin-data/watch/1.json");
    assert!(std::fs::read_to_string(&file).unwrap().contains("\"chats\":2"));
    core.run_plugin_command(&cmd("own"), None, None, None).unwrap();
    wait_store(&core, "its own storage", |_| toasted(&core, "Watch: own nil"));

    // Settings come from the manifest until changed, and reset with no value.
    assert!(core.set_plugin_setting("watch", "loud", "maybe").is_err());
    assert!(core.set_plugin_setting("watch", "nope", "on").is_err());
    core.set_plugin_setting("watch", "greeting", "hello there")
        .unwrap();
    core.set_plugin_setting("watch", "loud", "on").unwrap();
    let values: Vec<_> = core
        .plugin_settings("watch")
        .into_iter()
        .map(|(s, v)| (s.key, v))
        .collect();
    assert_eq!(
        values,
        vec![
            ("greeting".to_owned(), serde_json::json!("hello there")),
            ("loud".to_owned(), serde_json::json!(true))
        ]
    );
    core.run_plugin_command(&cmd("count"), None, Some(ME), None)
        .unwrap();
    wait_store(&core, "the new settings", |_| {
        toasted(&core, "Watch: 2 hello there true")
    });
    core.set_plugin_setting("watch", "greeting", "").unwrap();
    assert_eq!(core.plugin_settings("watch")[0].1, serde_json::json!("hi"));

    // Without the account it hears nothing.
    core.set_plugin_account("watch", ME, false).unwrap();
    wait(&core, "the grant to go", |p| p[0].accounts.is_empty());
    core.apply_realtime(&chat);
    std::thread::sleep(Duration::from_millis(200));
    assert!(!core.take_alerts().iter().any(|a| a.kind == "plugins"));
    assert!(core.store().plugin_actions.is_none());

    // Removing it deletes what it kept.
    core.remove_plugin("watch").unwrap();
    wait(&core, "the plugin to go", |p| p.is_empty());
    assert!(!home.path().join("config/plugin-data/watch").exists());
}

//! Fixture plugins against a fake host: escapes fail, limits trip, undeclared actions are refused.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;
use std::time::Duration;

use bonsai_plugin::{Action, Arg, Ctx, Error, Host, Limits, Manifest, Outcome, Plugin, Read, TradeStatus};

#[derive(Default)]
struct FakeHost {
    runs: RefCell<Vec<(u64, String, Option<String>)>>,
    logs: RefCell<Vec<String>>,
    reads: RefCell<Vec<(u64, Read)>>,
    /// What every read returns.
    data: serde_json::Value,
    /// How long `run` and `read` take, to check host time isn't counted.
    delay: Duration,
    fail: Option<String>,
    /// `ctx.store`, by account and key.
    store: RefCell<HashMap<(u64, String), serde_json::Value>>,
    settings: serde_json::Map<String, serde_json::Value>,
}

impl Host for FakeHost {
    fn run(&self, _plugin: &str, account: u64, action: &str, arg: Option<&str>) -> Result<(), String> {
        std::thread::sleep(self.delay);
        if let Some(e) = &self.fail {
            return Err(e.clone());
        }
        self.runs
            .borrow_mut()
            .push((account, action.to_owned(), arg.map(str::to_owned)));
        Ok(())
    }
    fn read(&self, _plugin: &str, account: u64, read: &Read) -> Result<serde_json::Value, String> {
        std::thread::sleep(self.delay);
        if let Some(e) = &self.fail {
            return Err(e.clone());
        }
        self.reads.borrow_mut().push((account, read.clone()));
        Ok(self.data.clone())
    }
    fn log(&self, _plugin: &str, line: &str) {
        self.logs.borrow_mut().push(line.to_owned());
    }
    fn store_get(&self, _plugin: &str, account: u64, key: &str) -> Result<serde_json::Value, String> {
        Ok(self
            .store
            .borrow()
            .get(&(account, key.to_owned()))
            .cloned()
            .unwrap_or_default())
    }
    fn store_set(
        &self,
        _plugin: &str,
        account: u64,
        key: &str,
        value: serde_json::Value,
    ) -> Result<(), String> {
        let mut s = self.store.borrow_mut();
        if value.is_null() {
            s.remove(&(account, key.to_owned()));
        } else {
            s.insert((account, key.to_owned()), value);
        }
        Ok(())
    }
    fn settings(&self, _plugin: &str) -> serde_json::Map<String, serde_json::Value> {
        self.settings.clone()
    }
}

const MANIFEST: &str = r#"
id = "test"
entry = "main.luau"
actions = ["user.unfriend", "user.decline_friend"]
"#;

const CTX: Ctx = Ctx {
    account: 7,
    now: 1_800_000_000,
};

fn limits() -> Limits {
    Limits {
        call: Duration::from_millis(50),
        load: Duration::from_millis(250),
        memory: 8 << 20,
    }
}

struct Fixture {
    _dir: tempfile::TempDir,
    host: Rc<FakeHost>,
    result: Result<Plugin, Error>,
}

fn write(dir: &Path, files: &[(&str, &str)]) {
    for (name, text) in files {
        let path = dir.join(name);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    }
}

fn load_with(main: &str, files: &[(&str, &str)], host: FakeHost) -> Fixture {
    load_manifest(MANIFEST, main, files, host)
}

fn load_manifest(manifest: &str, main: &str, files: &[(&str, &str)], host: FakeHost) -> Fixture {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join("test");
    write(&dir, &[("main.luau", main)]);
    write(&dir, files);
    let host = Rc::new(host);
    let manifest = Manifest::parse(manifest).unwrap();
    let result = Plugin::load(&dir, manifest, host.clone(), limits());
    Fixture {
        _dir: tmp,
        host,
        result,
    }
}

fn load(main: &str) -> Result<Plugin, Error> {
    load_with(main, &[], FakeHost::default()).result
}

/// A plugin with one command `c` whose body is `body`.
fn command(body: &str) -> Plugin {
    let main = format!(
        "bonsai.command {{ id = 'c', title = 'C', arg = 'text', run = function(ctx, arg)\n{body}\nend }}"
    );
    load(&main).unwrap()
}

fn run(body: &str) -> Result<Outcome, Error> {
    command(body).run_command("c", CTX, None)
}

fn script_error(r: Result<impl std::fmt::Debug, Error>) -> String {
    match r {
        Err(Error::Script(m)) => m,
        other => panic!("expected a script error, got {other:?}"),
    }
}

#[test]
fn registers_and_runs_commands() {
    let p = load(
        r#"
        bonsai.command {
          id = "unfriend-inactive",
          title = "Unfriend people offline more than 6 months",
          keywords = { "friends", "cleanup" },
          arg = "user",
          context = "user",
          run = function(ctx, arg)
            return { actions = { { id = "user.unfriend", arg = arg }, { id = "user.decline_friend", arg = "12" } } }
          end,
        }
        bonsai.command { id = "hello", title = "Hello", run = function(ctx) return { toast = "hi " .. ctx.account } end }
        "#,
    )
    .unwrap();
    let cmds = p.commands();
    assert_eq!(cmds.len(), 2);
    assert_eq!(cmds[0].keywords, vec!["friends", "cleanup"]);
    assert_eq!(
        (cmds[0].arg, cmds[0].context.as_deref()),
        (Arg::User, Some("user"))
    );
    assert_eq!(
        p.run_command("unfriend-inactive", CTX, Some("156")).unwrap(),
        Outcome::Actions(vec![
            Action {
                id: "user.unfriend".into(),
                arg: Some("156".into())
            },
            Action {
                id: "user.decline_friend".into(),
                arg: Some("12".into())
            },
        ])
    );
    assert_eq!(
        p.run_command("hello", CTX, None).unwrap(),
        Outcome::Toast("hi 7".into())
    );
    assert!(matches!(
        p.run_command("nope", CTX, None),
        Err(Error::NoCommand(_))
    ));
    assert!(matches!(
        p.run_command("unfriend-inactive", CTX, Some("bob")),
        Err(Error::Script(_))
    ));
}

#[test]
fn other_outcomes() {
    assert_eq!(run("return nil").unwrap(), Outcome::Nothing);
    assert_eq!(
        run("return { open = { kind = 'user', id = 1 } }").unwrap(),
        Outcome::Open {
            kind: "user".into(),
            id: 1
        }
    );
    assert_eq!(
        run("return { alert = { title = 'T', body = 'B', command = 'plugin.test.c' } }").unwrap(),
        Outcome::Alert {
            title: "T".into(),
            body: "B".into(),
            command: Some("plugin.test.c".into())
        }
    );
    script_error(run("return { open = { kind = 'file', id = 1 } }"));
    script_error(run("return 5"));
    script_error(run("return { what = 1 }"));
}

#[test]
fn bad_registrations_fail_to_load() {
    for main in [
        "bonsai.command { id = 'Bad', title = 'x', run = function() end }",
        "bonsai.command { id = 'a', title = '', run = function() end }",
        "bonsai.command { id = 'a', title = 'x' }",
        "bonsai.command { id = 'a', title = 'x', arg = 'file', run = function() end }",
        "bonsai.command { id = 'a', title = 'x', context = 'trade', run = function() end }",
        "for i = 1, 2 do bonsai.command { id = 'a', title = 'x', run = function() end } end",
        "for i = 1, 101 do bonsai.command { id = 'a' .. i, title = 'x', run = function() end } end",
        "error('boom')",
        "local x = ",
    ] {
        assert!(matches!(load(main), Err(Error::Script(_))), "{main}");
    }
    // Only while loading.
    let p = command("bonsai.command { id = 'late', title = 'x', run = function() end }");
    assert!(script_error(p.run_command("c", CTX, None)).contains("while the plugin loads"));
}

#[test]
fn folder_must_match_id() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join("other");
    write(&dir, &[("main.luau", "")]);
    let m = Manifest::parse(MANIFEST).unwrap();
    assert!(matches!(
        Plugin::load(&dir, m, Rc::new(FakeHost::default()), limits()),
        Err(Error::Manifest(_))
    ));
}

#[test]
fn dangerous_globals_are_gone() {
    for name in [
        "os",
        "io",
        "debug",
        "getfenv",
        "setfenv",
        "loadstring",
        "load",
        "dofile",
        "loadfile",
        "package",
        "ffi",
        "jit",
        "collectgarbage_full",
    ] {
        assert_eq!(
            run(&format!("return {{ toast = type({name}) }}")).unwrap(),
            Outcome::Toast("nil".into()),
            "{name}"
        );
    }
}

#[test]
fn libraries_and_metatables_are_read_only() {
    for body in [
        "string.upper = function() end",
        "getmetatable('').__index = {}",
        "getmetatable('').__index.upper = nil",
        "table.insert = nil",
        "math.pi = 3",
        "bonsai.command = nil",
        "bonsai.id = 'someone-else'",
        "ctx.account = 1",
        "ctx.run = nil",
        "setmetatable(string, {})",
    ] {
        let err = script_error(run(body));
        assert!(err.contains("readonly"), "{body}: {err}");
    }
    // Globals a script sets don't leak into the shared ones.
    let p = command("x = (x or 0) + 1 return { toast = tostring(x) }");
    assert_eq!(p.run_command("c", CTX, None).unwrap(), Outcome::Toast("1".into()));
    assert_eq!(p.run_command("c", CTX, None).unwrap(), Outcome::Toast("2".into()));
}

#[test]
fn require_stays_in_the_folder() {
    let f = load_with(
        r#"
        local util = require("lib/util")
        bonsai.command { id = "c", title = "C", run = function() return { toast = util.hi() } end }
        bonsai.command { id = "up", title = "Up", arg = "text", run = function(ctx, m) return require(m) end }
        "#,
        &[("lib/util.luau", "return { hi = function() return 'hi' end }")],
        FakeHost::default(),
    );
    let p = f.result.unwrap();
    assert_eq!(
        p.run_command("c", CTX, None).unwrap(),
        Outcome::Toast("hi".into())
    );
    for path in [
        "../secret",
        "/etc/passwd",
        "lib/../main",
        "lib\\util",
        "Lib/Util",
        "missing",
    ] {
        assert!(
            matches!(p.run_command("up", CTX, Some(path)), Err(Error::Script(_))),
            "{path}"
        );
    }
}

#[cfg(unix)]
#[test]
fn require_refuses_symlinks_out() {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::write(tmp.path().join("secret.luau"), "return { toast = 'leaked' }").unwrap();
    let dir = tmp.path().join("test");
    write(&dir, &[("main.luau", "local s = require('secret')")]);
    std::os::unix::fs::symlink(tmp.path().join("secret.luau"), dir.join("secret.luau")).unwrap();
    let m = Manifest::parse(MANIFEST).unwrap();
    let err = Plugin::load(&dir, m, Rc::new(FakeHost::default()), limits())
        .err()
        .unwrap();
    assert!(err.to_string().contains("outside the plugin folder"), "{err}");
}

#[test]
fn bytecode_is_not_loaded() {
    let bytecode = mlua::chunk::Compiler::new()
        .compile("return { toast = 'bytecode' }")
        .unwrap();
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join("test");
    write(&dir, &[("main.luau", "local m = require('evil')")]);
    std::fs::write(dir.join("evil.luau"), bytecode).unwrap();
    let m = Manifest::parse(MANIFEST).unwrap();
    assert!(Plugin::load(&dir, m, Rc::new(FakeHost::default()), limits()).is_err());
}

#[test]
fn time_limit_trips_even_if_caught() {
    assert!(matches!(run("while true do end"), Err(Error::Timeout(50))));
    assert!(matches!(
        run("for i = 1, 1e9 do pcall(function() while true do end end) end"),
        Err(Error::Timeout(50))
    ));
    assert!(matches!(
        run("local co = coroutine.create(function() while true do end end) coroutine.resume(co)"),
        Err(Error::Timeout(50)) | Err(Error::Script(_))
    ));
    assert!(matches!(load("while true do end"), Err(Error::Timeout(250))));
    // The VM still works afterwards.
    let p = command("if arg == 'spin' then while true do end end return { toast = 'ok' }");
    assert!(matches!(
        p.run_command("c", CTX, Some("spin")),
        Err(Error::Timeout(_))
    ));
    assert_eq!(
        p.run_command("c", CTX, None).unwrap(),
        Outcome::Toast("ok".into())
    );
}

#[test]
fn memory_limit_trips() {
    assert!(matches!(
        run("local s = string.rep('x', 64 * 1024 * 1024)"),
        Err(Error::Memory(8))
    ));
    assert!(matches!(
        run("local t = {} for i = 1, 1e7 do t[i] = { i } end"),
        Err(Error::Memory(8)) | Err(Error::Timeout(_))
    ));
    let p = command(
        "if arg == 'big' then local s = string.rep('x', 64 * 1024 * 1024) end return { toast = 'ok' }",
    );
    assert!(matches!(
        p.run_command("c", CTX, Some("big")),
        Err(Error::Memory(_))
    ));
    assert_eq!(
        p.run_command("c", CTX, None).unwrap(),
        Outcome::Toast("ok".into())
    );
}

#[test]
fn undeclared_actions_are_refused() {
    // Returned: the whole result is refused.
    assert!(matches!(
        run("return { actions = { { id = 'user.unfriend', arg = 1 }, { id = 'market.buy' } } }"),
        Err(Error::Undeclared(a)) if a == "market.buy"
    ));
    // Through ctx.run: a Luau error the plugin can catch, and the host never sees it.
    let f = load_with(
        r#"bonsai.command { id = "c", title = "C", run = function(ctx)
            local ok, err = pcall(ctx.run, "account.sign_out")
            ctx.run("user.unfriend", 156)
            return { toast = tostring(ok) .. " " .. tostring(err) }
        end }"#,
        &[],
        FakeHost::default(),
    );
    let p = f.result.unwrap();
    match p.run_command("c", CTX, None).unwrap() {
        Outcome::Toast(t) => assert!(t.starts_with("false") && t.contains("account.sign_out"), "{t}"),
        other => panic!("{other:?}"),
    }
    assert_eq!(
        *f.host.runs.borrow(),
        vec![(7, "user.unfriend".to_owned(), Some("156".to_owned()))]
    );
}

#[test]
fn host_errors_reach_the_plugin_and_host_time_is_free() {
    let main = r#"bonsai.command { id = "c", title = "C", run = function(ctx)
        for i = 1, 3 do ctx.run("user.unfriend", i) end
        return { toast = "done" }
    end }"#;
    // Three 40 ms host calls go past the 50 ms limit, but only Luau time counts.
    let slow = FakeHost {
        delay: Duration::from_millis(40),
        ..FakeHost::default()
    };
    let f = load_with(main, &[], slow);
    assert_eq!(
        f.result.unwrap().run_command("c", CTX, None).unwrap(),
        Outcome::Toast("done".into())
    );
    assert_eq!(f.host.runs.borrow().len(), 3);

    let failing = FakeHost {
        fail: Some("Automation is off for this account.".into()),
        ..FakeHost::default()
    };
    let f = load_with(main, &[], failing);
    let err = script_error(f.result.unwrap().run_command("c", CTX, None));
    assert!(err.contains("Automation is off"), "{err}");
}

#[test]
fn print_goes_to_the_host_log() {
    let f = load_with("print('loaded', 1, nil, true)", &[], FakeHost::default());
    assert!(f.result.is_ok());
    assert_eq!(*f.host.logs.borrow(), vec!["loaded\t1\tnil\ttrue"]);
}

const READER: &str = r#"
id = "test"
entry = "main.luau"
permissions = ["read.public", "read.account"]
"#;

/// Runs command `c` with `body` in a plugin that may read, against `host`.
fn read_with(manifest: &str, body: &str, host: FakeHost) -> (Result<Outcome, Error>, Vec<(u64, Read)>) {
    let main = format!("bonsai.command {{ id = 'c', title = 'C', run = function(ctx)\n{body}\nend }}");
    let f = load_manifest(manifest, &main, &[], host);
    let out = f.result.unwrap().run_command("c", CTX, None);
    let reads = f.host.reads.borrow().clone();
    (out, reads)
}

#[test]
fn reads_reach_the_host_as_the_account() {
    let body = r#"
        ctx.user(1); ctx.game("920587237"); ctx.item(2); ctx.item(3, "bundle")
        ctx.presence({ 4, 5 }); ctx.friends()
        ctx.inventory(); ctx.inventory({ user = 6, asset_type = 8, limit = 500 })
        ctx.trades(); ctx.trades({ status = "completed", limit = 100 })
        ctx.friend_requests()
    "#;
    let (out, reads) = read_with(READER, body, FakeHost::default());
    assert_eq!(out.unwrap(), Outcome::Nothing);
    let inv = |user, asset_type, limit| Read::Inventory {
        user,
        asset_type,
        bundle_type: None,
        limit,
    };
    let want = vec![
        Read::User(1),
        Read::Game(920587237),
        Read::Item { id: 2, bundle: false },
        Read::Item { id: 3, bundle: true },
        Read::Presence(vec![4, 5]),
        Read::Friends,
        inv(None, None, 100),
        inv(Some(6), Some(8), 500),
        Read::Trades {
            status: TradeStatus::Inbound,
            limit: 25,
        },
        Read::Trades {
            status: TradeStatus::Completed,
            limit: 100,
        },
        Read::FriendRequests { limit: 50 },
    ];
    assert_eq!(
        reads,
        want.into_iter().map(|r| (CTX.account, r)).collect::<Vec<_>>()
    );
}

#[test]
fn host_data_becomes_tables() {
    let host = FakeHost {
        data: serde_json::json!({ "id": 1, "name": "a", "tags": ["x", "y"], "gone": null, "ok": true }),
        ..FakeHost::default()
    };
    let body = r#"
        local u = ctx.user(1)
        assert(u.id == 1 and u.name == "a" and #u.tags == 2 and u.tags[2] == "y")
        assert(u.gone == nil and u.ok == true)
        return { toast = u.name }
    "#;
    assert_eq!(
        read_with(READER, body, host).0.unwrap(),
        Outcome::Toast("a".into())
    );
    let (out, _) = read_with(READER, "assert(ctx.user(1) == nil)", FakeHost::default());
    assert_eq!(out.unwrap(), Outcome::Nothing);
}

#[test]
fn reads_need_the_permission_in_the_manifest() {
    let public_only = r#"
id = "test"
entry = "main.luau"
permissions = ["read.public"]
"#;
    let (out, reads) = read_with(public_only, "ctx.user(1); ctx.friends()", FakeHost::default());
    assert!(script_error(out).contains("\"read.account\""));
    assert_eq!(reads.len(), 1);
    let (out, reads) = read_with(MANIFEST, "ctx.item(1)", FakeHost::default());
    assert!(script_error(out).contains("\"read.public\""));
    assert!(reads.is_empty());
}

#[test]
fn bad_read_args_are_errors() {
    for (body, want) in [
        ("ctx.user(0)", "ctx.user takes an id"),
        ("ctx.user(1.5)", "ctx.user takes an id"),
        ("ctx.user('abc')", "ctx.user takes an id"),
        ("ctx.item(1, 'hat')", "kind is asset or bundle"),
        ("ctx.inventory({ asset_type = 8, bundle_type = 1 })", "not both"),
        (
            "ctx.inventory({ limit = 5000 })",
            "limit is a whole number from 1 to 1000",
        ),
        ("ctx.trades({ status = 'mine' })", "status is inbound"),
    ] {
        let (out, reads) = read_with(READER, body, FakeHost::default());
        let err = script_error(out);
        assert!(err.contains(want), "{body}: {err}");
        assert!(reads.is_empty(), "{body}");
    }
    let many: Vec<String> = (1..=101).map(|i| i.to_string()).collect();
    let body = format!("ctx.presence({{ {} }})", many.join(", "));
    assert!(script_error(read_with(READER, &body, FakeHost::default()).0).contains("at most 100 ids"));
}

#[test]
fn reads_are_capped_per_call_and_off_the_clock() {
    let (out, reads) = read_with(READER, "for i = 1, 300 do ctx.user(i) end", FakeHost::default());
    assert!(script_error(out).contains("at most 200 reads"));
    assert_eq!(reads.len(), 200);
    // Three 40 ms reads go past the 50 ms limit, but only Luau time counts.
    let slow = FakeHost {
        delay: Duration::from_millis(40),
        ..FakeHost::default()
    };
    let (out, _) = read_with(READER, "for i = 1, 3 do ctx.user(i) end", slow);
    assert_eq!(out.unwrap(), Outcome::Nothing);
    let failing = FakeHost {
        fail: Some("Not signed in.".into()),
        ..FakeHost::default()
    };
    let body = "local ok, e = pcall(ctx.friends); return { toast = tostring(e) }";
    let Outcome::Toast(t) = read_with(READER, body, failing).0.unwrap() else {
        panic!("expected a toast");
    };
    assert!(t.contains("Not signed in."), "{t}");
}

const LISTENER: &str = r#"
id = "test"
entry = "main.luau"
permissions = ["read.account"]
actions = ["user.decline_friend"]
"#;

#[test]
fn events_and_timers_register_and_run() {
    let main = r#"
        bonsai.on("friend_request", function(ctx, req)
          return { actions = { { id = "user.decline_friend", arg = req.from } } }
        end)
        bonsai.on("presence", function(ctx, list) return { toast = list[1].status .. " " .. ctx.account } end)
        bonsai.every(300, function(ctx) return { toast = "tick " .. ctx.account } end)
        bonsai.every(60, function(ctx) end)
    "#;
    let f = load_manifest(LISTENER, main, &[], FakeHost::default());
    let p = f.result.unwrap();
    assert_eq!(p.events(), vec!["friend_request", "presence"]);
    assert_eq!(
        p.timers(),
        vec![Duration::from_secs(300), Duration::from_secs(60)]
    );
    assert_eq!(
        p.run_event("friend_request", CTX, &serde_json::json!({ "from": 156 }))
            .unwrap(),
        Outcome::Actions(vec![Action {
            id: "user.decline_friend".into(),
            arg: Some("156".into())
        }])
    );
    assert_eq!(
        p.run_event("presence", CTX, &serde_json::json!([{ "status": "in_game" }]))
            .unwrap(),
        Outcome::Toast("in_game 7".into())
    );
    assert_eq!(
        p.run_event("trade", CTX, &serde_json::json!({})).unwrap(),
        Outcome::Nothing
    );
    assert_eq!(p.run_timer(0, CTX).unwrap(), Outcome::Toast("tick 7".into()));
    assert_eq!(p.run_timer(1, CTX).unwrap(), Outcome::Nothing);
}

#[test]
fn bad_events_and_timers_fail_to_load() {
    for (manifest, main, want) in [
        (
            MANIFEST,
            "bonsai.on('presence', function() end)",
            "needs \"read.account\"",
        ),
        (
            LISTENER,
            "bonsai.on('cookie', function() end)",
            "no event \"cookie\"",
        ),
        (
            LISTENER,
            "bonsai.on('chat', function() end) bonsai.on('chat', function() end)",
            "registered twice",
        ),
        (LISTENER, "bonsai.every(59, function() end)", "from 60"),
        (LISTENER, "bonsai.every(90.5, function() end)", "whole seconds"),
        (
            LISTENER,
            "for i = 1, 11 do bonsai.every(60, function() end) end",
            "at most 10 timers",
        ),
    ] {
        let err = script_error(
            load_manifest(manifest, main, &[], FakeHost::default())
                .result
                .map(|_| ()),
        );
        assert!(err.contains(want), "{main}: {err}");
    }
    // Registering later, from a handler, is refused.
    let p = load_manifest(
        LISTENER,
        "bonsai.command { id = 'c', title = 'C', run = function() bonsai.every(60, function() end) end }",
        &[],
        FakeHost::default(),
    )
    .result
    .unwrap();
    assert!(script_error(p.run_command("c", CTX, None)).contains("only works while the plugin loads"));
}

#[test]
fn store_keeps_values_per_account() {
    let main = r#"
        bonsai.command { id = 'put', title = 'Put', run = function(ctx)
          ctx.store.set("seen", { ids = { 1, 2, 3 }, note = "hi", ok = true, n = 1.5 })
          ctx.store.set("gone", 1)
          ctx.store.set("gone", nil)
        end }
        bonsai.command { id = 'get', title = 'Get', run = function(ctx)
          local v = ctx.store.get("seen")
          if v == nil then return { toast = "none" } end
          assert(ctx.store.get("gone") == nil)
          return { toast = v.note .. #v.ids .. v.ids[3] .. tostring(v.ok) .. v.n }
        end }
        bonsai.command { id = 'bad', title = 'Bad', arg = 'text', run = function(ctx, what)
          if what == "fn" then ctx.store.set("x", { f = print }) end
          if what == "mixed" then ctx.store.set("x", { 1, a = 2 }) end
          if what == "key" then ctx.store.set(5, 1) end
          if what == "many" then for i = 1, 101 do ctx.store.set("k" .. i, i) end end
          if what == "deep" then
            local t = {}
            for i = 1, 20 do t = { t } end
            ctx.store.set("x", t)
          end
        end }
    "#;
    let f = load_manifest(MANIFEST, main, &[], FakeHost::default());
    let p = f.result.unwrap();
    p.run_command("put", CTX, None).unwrap();
    assert_eq!(
        p.run_command("get", CTX, None).unwrap(),
        Outcome::Toast("hi33true1.5".into())
    );
    let other = Ctx { account: 8, ..CTX };
    assert_eq!(
        p.run_command("get", other, None).unwrap(),
        Outcome::Toast("none".into())
    );
    assert_eq!(
        f.host.store.borrow().get(&(7, "seen".to_owned())),
        Some(&serde_json::json!({ "ids": [1, 2, 3], "note": "hi", "ok": true, "n": 1.5 }))
    );
    for (what, want) in [
        ("fn", "can't keep a function"),
        ("mixed", "keys that aren't strings"),
        ("key", "keys are strings"),
        ("many", "at most 100"),
        ("deep", "nested more than 16"),
    ] {
        let err = script_error(p.run_command("bad", CTX, Some(what)));
        assert!(err.contains(want), "{what}: {err}");
    }
}

#[test]
fn settings_are_read_only_values() {
    let mut settings = serde_json::Map::new();
    settings.insert("min_age".into(), serde_json::json!(7));
    settings.insert("mode".into(), serde_json::json!("loud"));
    let host = FakeHost {
        settings,
        ..FakeHost::default()
    };
    let main = r#"
        bonsai.command { id = 'c', title = 'C', run = function(ctx)
          local ok = pcall(function() ctx.settings.mode = "quiet" end)
          return { toast = ctx.settings.mode .. ctx.settings.min_age .. tostring(ok) }
        end }
    "#;
    let p = load_manifest(MANIFEST, main, &[], host).result.unwrap();
    assert_eq!(
        p.run_command("c", CTX, None).unwrap(),
        Outcome::Toast("loud7false".into())
    );
}

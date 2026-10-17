//! Installing, turning on and off, reviewing and removing plugins against real folders.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};

use bonsai_api::{Base, RateLimiter};
use bonsai_core::account::MemoryCookies;
use bonsai_core::launcher::RecordingOpener;
use bonsai_core::plugins::{PluginInfo, Status};
use bonsai_core::{Core, CoreConfig, Paths};

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
    let Some(Status::Running { commands }) = status(&p, "hello") else {
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

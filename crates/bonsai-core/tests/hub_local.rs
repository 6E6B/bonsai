//! Opt-in round trip against the local Worker emulator. No Roblox account or website cookie.
#![allow(clippy::unwrap_used, clippy::expect_used)]
use bonsai_api::{Base, RateLimiter};
use bonsai_core::hub::{Publish, Target};
use bonsai_core::themes::OnClash;
use bonsai_core::{Core, CoreConfig, Paths, account::MemoryCookies, launcher::RecordingOpener};
use bonsai_hub_api::{Kind, Query};
use std::sync::Arc;
use std::time::{Duration, Instant};
#[tokio::test]
#[ignore = "requires local backend plus BONSAI_HUB_URL and a local publisher session"]
async fn browse_install_publish_round_trip() {
    assert!(
        std::env::var("BONSAI_HUB_URL")
            .unwrap()
            .starts_with("http://127.0.0.1:")
    );
    let dir = tempfile::tempdir().unwrap();
    let core = Arc::new(Core::start(
        CoreConfig {
            paths: Paths::in_dir(dir.path()),
            base: Base::Production,
            cookies: Arc::new(MemoryCookies::default()),
            opener: Arc::new(RecordingOpener::default()),
            limiter: Arc::new(RateLimiter::default()),
            background: false,
            quick_poll: Duration::from_secs(5),
        },
        Arc::new(|| {}),
    ));
    let start = Instant::now();
    loop {
        let state = core.hub_snapshot(Query::default());
        if !state.catalog.loading && state.user.is_some() {
            assert!(state.catalog.error.is_none(), "{:?}", state.catalog.error);
            assert!(!state.catalog.page.items.is_empty());
            break;
        }
        assert!(start.elapsed() < Duration::from_secs(15));
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
    let entry = core.hub_catalog(Query::default()).page.items[0].clone();
    core.hub_install(Target {
        kind: Kind::Themes,
        id: entry.id.clone(),
        version: entry.version.clone(),
    })
    .unwrap();
    let start = Instant::now();
    while core.store().theme_import.is_none() {
        assert!(start.elapsed() < Duration::from_secs(15));
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
    let id = core.finish_theme_import(OnClash::KeepBoth).unwrap();
    assert!(
        core.themes()
            .iter()
            .any(|(theme, builtin)| theme.id == id && !*builtin)
    );
    let hub_id = format!("round-trip-{}", std::process::id());
    core.hub_publish_theme(Publish {
        local_id: id,
        hub_id: hub_id.clone(),
        name: "Round trip".into(),
        description: "Local smoke test".into(),
        tags: vec!["test".into()],
        version: "1.0.0".into(),
    })
    .unwrap();
    let start = Instant::now();
    loop {
        let state = core.hub_snapshot(Query::default());
        if !state.busy {
            assert!(state.error.is_none(), "{:?}", state.error);
            break;
        }
        assert!(start.elapsed() < Duration::from_secs(15));
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
    let start = Instant::now();
    loop {
        let page = core.hub_catalog(Query {
            q: "Round trip".into(),
            ..Default::default()
        });
        if !page.loading {
            assert!(page.page.items.iter().any(|entry| entry.id == hub_id));
            break;
        }
        assert!(start.elapsed() < Duration::from_secs(15));
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
}

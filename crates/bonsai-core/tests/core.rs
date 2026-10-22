//! Core against replayed traffic: sign-in, switching, launching, expiry and friends in servers.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;
use std::time::{Duration, Instant};

use bonsai_api::replay::{self, Fixture, Interaction, Replay};
use bonsai_api::{AccountId, Base, RateLimiter, Secret};
use bonsai_core::account::{AccountList, CookieStore, MemoryCookies};
use bonsai_core::cache::{Cache, Kind};
use bonsai_core::launcher::{JoinTarget, RecordingOpener};
use bonsai_core::me::ServersTab;
use bonsai_core::search::{Group, Scope, Target};
use bonsai_core::store::{LoginState, PresenceKind, SessionState, Unread};
use bonsai_core::thumbs::{DiskCache, ThumbKey, ThumbKind, ThumbState};
use bonsai_core::{AccountMeta, Cmd, Core, CoreConfig, Paths, StoreData};

const ME: AccountId = AccountId(1_000_001);
const ALT: AccountId = AccountId(1_000_002);
const COOKIE: &str = "_|WARNING:-DO-NOT-SHARE-THIS.--Sharing-this-will-allow-someone-to-log-in-as-you-and-to-steal-your-ROBUX-and-items.|_FAKEFAKEFAKEFAKEFAKEFAKEFAKEFAKE";

struct Rig {
    core: Core,
    opener: Arc<RecordingOpener>,
    cookies: Arc<MemoryCookies>,
    _dir: tempfile::TempDir,
}

fn rig(base: Base, dir: tempfile::TempDir, cookies: Arc<MemoryCookies>) -> Rig {
    let opener = Arc::new(RecordingOpener::default());
    let cfg = CoreConfig {
        paths: Paths::in_dir(dir.path()),
        base,
        cookies: cookies.clone(),
        opener: opener.clone(),
        limiter: Arc::new(RateLimiter::default()),
        background: false,
        quick_poll: Duration::from_millis(10),
    };
    Rig {
        core: Core::start(cfg, Arc::new(|| {})),
        opener,
        cookies,
        _dir: dir,
    }
}

async fn wait_for(core: &Core, what: &str, f: impl Fn(&StoreData) -> bool) {
    let start = Instant::now();
    while !f(&core.store()) {
        assert!(
            start.elapsed() < Duration::from_secs(10),
            "timed out waiting for {what}; toasts: {:?}",
            core.store().toasts
        );
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
}

/// Two saved accounts with cookies and some cached data, no network.
fn seeded(dir: &tempfile::TempDir) -> Arc<MemoryCookies> {
    let paths = Paths::in_dir(dir.path());
    let list = AccountList {
        accounts: vec![
            AccountMeta {
                id: ME,
                name: "TestUser".into(),
                display_name: "Test User".into(),
                ..Default::default()
            },
            AccountMeta {
                id: ALT,
                name: "AltUser".into(),
                display_name: "Alt".into(),
                ..Default::default()
            },
        ],
        active: Some(ME),
        previous: None,
    };
    list.save(&paths.accounts_file()).unwrap();
    std::fs::create_dir_all(&paths.cache).unwrap();
    let cache = Cache::open(&paths.db_file()).unwrap();
    cache.put(Kind::Robux, ME, "robux", &10_i64);
    cache.put(Kind::Robux, ALT, "robux", &20_i64);
    cache.put(
        Kind::Unread,
        ALT,
        "unread",
        &Unread {
            messages: 3,
            ..Default::default()
        },
    );
    cache.put(Kind::Friends, ALT, "friends", &vec![1_u64, 156]);
    drop(cache);
    let cookies = Arc::new(MemoryCookies::default());
    cookies.set(ME, &Secret::new(COOKIE)).unwrap();
    cookies.set(ALT, &Secret::new(COOKIE)).unwrap();
    cookies
}

fn png() -> Vec<u8> {
    let mut buf = std::io::Cursor::new(Vec::new());
    image::RgbaImage::from_pixel(4, 4, image::Rgba([9, 9, 9, 255]))
        .write_to(&mut buf, image::ImageFormat::Png)
        .unwrap();
    buf.into_inner()
}

#[tokio::test(flavor = "multi_thread")]
async fn paste_cookie_validates_then_confirms() {
    let replay = Replay::start(&["core_signin"]).await;
    let dir = tempfile::tempdir().unwrap();
    // The headshot bytes are already on disk, so no CDN fetch is needed.
    let f = replay::load("core_signin");
    let url = f.interactions.last().unwrap().response.body["data"][0]["imageUrl"]
        .as_str()
        .unwrap()
        .to_string();
    DiskCache::new(&Paths::in_dir(dir.path()).thumbs_dir()).put(&url, &png());
    let r = rig(Base::Replay(replay.server.uri()), dir, Arc::default());

    assert!(!r.core.submit_cookie("not a cookie"));
    assert!(matches!(r.core.store().login, Some(LoginState::Failed(_))));

    assert!(r.core.submit_cookie(&format!(".ROBLOSECURITY={COOKIE}; other=1")));
    wait_for(&r.core, "confirm step", |s| {
        matches!(s.login, Some(LoginState::Confirm { .. }))
    })
    .await;
    match r.core.store().login.clone() {
        Some(LoginState::Confirm { user, already_added }) => {
            assert_eq!(user.id, ME.0);
            assert_eq!(user.name, "TestUser");
            assert!(!already_added);
        }
        other => panic!("{other:?}"),
    }
    // Nothing is saved before the user confirms.
    assert!(r.cookies.get(ME).unwrap().is_none());
    assert!(r.core.store().accounts.accounts.is_empty());

    r.core.send(Cmd::ConfirmLogin);
    wait_for(&r.core, "unread counts and friends", |s| {
        s.account_data(ME)
            .is_some_and(|d| d.unread.friend_requests == 1 && d.friends.data.is_some())
    })
    .await;
    let s = r.core.store();
    assert_eq!(s.active_id(), Some(ME));
    assert!(s.login.is_none());
    let d = s.account_data(ME).unwrap();
    assert_eq!(d.session, SessionState::Ok);
    assert_eq!(d.robux, Some(0));
    assert_eq!(d.unread.messages, 1);
    assert_eq!(d.friends.data.as_deref(), Some(&[][..]));
    assert!(matches!(
        s.thumbs.get(&ThumbKey::large(ThumbKind::Headshot, ME.0)),
        Some(ThumbState::Ready(_))
    ));
    drop(s);
    assert_eq!(r.cookies.get(ME).unwrap().unwrap().expose(), COOKIE);
    // The account list on disk has no cookie in it.
    let saved = std::fs::read_to_string(r.core.paths().accounts_file()).unwrap();
    assert!(saved.contains("1000001") && !saved.contains("FAKEFAKE"));
}

#[tokio::test(flavor = "multi_thread")]
async fn sign_out_ends_the_session_then_forgets_the_account() {
    let replay = Replay::from_fixture(&replay::load("sign_out")).await;
    let dir = tempfile::tempdir().unwrap();
    let cookies = seeded(&dir);
    let r = rig(Base::Replay(replay.server.uri()), dir, cookies);
    r.core.send(Cmd::SignOut(ALT));
    wait_for(&r.core, "signed out", |s| {
        s.toasts.iter().any(|t| t.text == "Signed out.")
    })
    .await;
    assert!(r.core.store().accounts.get(ALT).is_none());
    assert!(r.cookies.get(ALT).unwrap().is_none());
    let reqs = replay.requests().await;
    assert!(reqs.iter().any(|q| q.url.path() == "/auth.roblox.com/v2/logout"));
}

#[tokio::test(flavor = "multi_thread")]
async fn switching_renders_from_cache_without_network() {
    let dir = tempfile::tempdir().unwrap();
    let cookies = seeded(&dir);
    // Point at a dead address: every refresh fails, cached data must still be there.
    let r = rig(Base::Replay("http://127.0.0.1:9".into()), dir, cookies);
    let assert_alt_cache = || {
        let s = r.core.store();
        assert_eq!(s.account_data(ALT).unwrap().robux, Some(20));
        assert_eq!(s.account_data(ALT).unwrap().unread.messages, 3);
        assert_eq!(
            s.account_data(ALT).unwrap().friends.data.as_deref(),
            Some(&[1_u64, 156][..])
        );
    };
    assert_eq!(r.core.store().active_id(), Some(ME));
    assert_alt_cache();
    assert!(r.core.switch(ALT));
    // Check the visible state as soon as switch returns, without awaiting a refresh.
    // A frame-sized wall-clock budget also measures disk and scheduler delays in CI.
    assert_eq!(r.core.store().active_id(), Some(ALT));
    assert_alt_cache();
    wait_for(&r.core, "failed offline refresh", |s| s.offline).await;
    assert_eq!(r.core.store().active_id(), Some(ALT));
    assert_alt_cache();
    assert!(r.core.switch_last());
    assert_eq!(r.core.store().active_id(), Some(ME));
    assert_eq!(r.core.store().account_data(ME).unwrap().robux, Some(10));
    assert!(!r.core.switch(AccountId(42)));
    assert_eq!(r.core.store().active_id(), Some(ME));
    // Survives a restart.
    let list = AccountList::load(&r.core.paths().accounts_file());
    assert_eq!(list.active, Some(ME));
    assert_eq!(list.previous, Some(ALT));
}

#[tokio::test(flavor = "multi_thread")]
async fn website_links_open_only_roblox_pages() {
    let dir = tempfile::tempdir().unwrap();
    let cookies = seeded(&dir);
    let r = rig(Base::Replay("http://127.0.0.1:9".into()), dir, cookies);
    let pass = bonsai_core::links::game_pass(9);
    r.core.open_web(&pass).unwrap();
    assert!(r.core.open_web("https://example.com/games/1").is_err());
    assert!(r.core.open_web("https://roblox.com.example.com/").is_err());
    assert_eq!(*r.opener.web.lock(), [pass]);
    // Website links never go to the player.
    assert!(r.opener.opened.lock().is_empty());
}

fn repeat(f: &Fixture, n: usize) -> Fixture {
    let mut out = f.clone();
    out.interactions = (0..n).flat_map(|_| f.interactions.clone()).collect();
    out
}

#[tokio::test(flavor = "multi_thread")]
async fn launch_as_each_account_in_each_mode() {
    let replay = Replay::from_fixture(&repeat(&replay::load("auth_ticket"), 5)).await;
    let dir = tempfile::tempdir().unwrap();
    let cookies = seeded(&dir);
    let r = rig(Base::Replay(replay.server.uri()), dir, cookies);
    let modes = [
        (ME, JoinTarget::Place { place_id: 1818 }, "RequestGame%26"),
        (
            ALT,
            JoinTarget::Server {
                place_id: 1818,
                job_id: "7b9b36ef".into(),
            },
            "gameId%3D7b9b36ef",
        ),
        (ME, JoinTarget::FollowUser { user_id: 156 }, "RequestFollowUser"),
        (
            ALT,
            JoinTarget::PrivateServer {
                place_id: 1818,
                link_code: Some("55".into()),
                access_code: None,
            },
            "linkCode%3D55",
        ),
    ];
    for (i, (acc, target, needle)) in modes.iter().enumerate() {
        r.core.send(Cmd::Launch {
            account: *acc,
            target: target.clone(),
        });
        let start = Instant::now();
        while r.opener.opened.lock().len() <= i {
            assert!(
                start.elapsed() < Duration::from_secs(10),
                "launch {i} did not open"
            );
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
        let uri = r.opener.opened.lock()[i].expose().to_string();
        assert!(
            uri.starts_with("roblox-player:1+launchmode:play+gameinfo:ticket-redacted+"),
            "{uri}"
        );
        assert!(uri.contains(needle), "{needle} not in {uri}");
    }
    wait_for(&r.core, "toasts", |s| s.toasts.len() == 4).await;
    assert!(
        r.core
            .store()
            .toasts
            .iter()
            .all(|t| !t.error && t.text.starts_with("Starting Roblox as"))
    );
    // An account with no cookie gets an error, not a launch.
    r.core.send(Cmd::Launch {
        account: AccountId(42),
        target: JoinTarget::Place { place_id: 1 },
    });
    wait_for(&r.core, "error toast", |s| s.toasts.iter().any(|t| t.error)).await;
    assert_eq!(r.opener.opened.lock().len(), 4);
}

#[tokio::test(flavor = "multi_thread")]
async fn expired_session_marks_only_that_account() {
    let unauthorized = Interaction {
        request: replay::RecordedRequest {
            method: "GET".into(),
            host: "users.roblox.com".into(),
            path: "/v1/users/authenticated".into(),
            ..Default::default()
        },
        response: replay::RecordedResponse {
            status: 401,
            body: serde_json::json!({"errors":[{"code":0,"message":"Authorization has been denied for this request."}]}),
            ..Default::default()
        },
    };
    let f = Fixture {
        source: "synthetic".into(),
        interactions: vec![unauthorized],
        ..Default::default()
    };
    let replay = Replay::from_fixture(&f).await;
    let dir = tempfile::tempdir().unwrap();
    let cookies = seeded(&dir);
    // The alt's cookie is gone from the keychain (e.g. Secret Service reset).
    cookies.delete(ALT).unwrap();
    let r = rig(Base::Replay(replay.server.uri()), dir, cookies);
    wait_for(&r.core, "expired", |s| {
        s.account_data(ME)
            .is_some_and(|d| d.session == SessionState::Expired)
    })
    .await;
    assert_eq!(
        r.core.store().account_data(ALT).unwrap().session,
        SessionState::NoCookie
    );
    // Cached data stays visible.
    assert_eq!(r.core.store().account_data(ME).unwrap().robux, Some(10));
}

#[tokio::test(flavor = "multi_thread")]
async fn friends_presence_marks_their_server() {
    let place = replay::load("core_place");
    let servers = place
        .interactions
        .iter()
        .find(|i| i.request.path.ends_with("/servers/0"))
        .unwrap();
    let job = servers.response.body["data"][0]["id"]
        .as_str()
        .unwrap()
        .to_string();
    let mut f = replay::load("core_signin");
    for i in &mut f.interactions {
        if i.request.path.ends_with("/friends") {
            i.response.body = serde_json::json!({"data":[{"id":1,"name":"","displayName":""},{"id":156,"name":"","displayName":""}]});
        }
    }
    let post = |host: &str, path: &str, body: serde_json::Value| Interaction {
        request: replay::RecordedRequest {
            method: "POST".into(),
            host: host.into(),
            path: path.into(),
            ..Default::default()
        },
        response: replay::RecordedResponse {
            status: 200,
            body,
            ..Default::default()
        },
    };
    f.interactions.push(post(
        "users.roblox.com",
        "/v1/users",
        serde_json::json!({"data":[{"id":1,"name":"Roblox","displayName":"Roblox","hasVerifiedBadge":true},{"id":156,"name":"builderman","displayName":"builderman","hasVerifiedBadge":true}]}),
    ));
    f.interactions.push(post(
        "presence.roblox.com",
        "/v1/presence/users",
        serde_json::json!({"userPresences":[
            {"userPresenceType":2,"lastLocation":"Crossroads","placeId":1818,"rootPlaceId":1818,"gameId":job,"universeId":13058,"userId":1},
            {"userPresenceType":1,"lastLocation":"Website","placeId":null,"rootPlaceId":null,"gameId":null,"universeId":null,"userId":156}
        ]}),
    ));
    f.interactions.extend(
        place
            .interactions
            .into_iter()
            .filter(|i| !i.request.path.contains("multiget")),
    );
    let replay = Replay::from_fixture(&f).await;
    let r = rig(
        Base::Replay(replay.server.uri()),
        tempfile::tempdir().unwrap(),
        Arc::default(),
    );
    r.core.submit_cookie(COOKIE);
    wait_for(&r.core, "confirm", |s| {
        matches!(s.login, Some(LoginState::Confirm { .. }))
    })
    .await;
    r.core.send(Cmd::ConfirmLogin);
    wait_for(&r.core, "presence", |s| {
        s.account_data(ME).is_some_and(|d| d.presence.len() == 2)
    })
    .await;
    wait_for(&r.core, "game details", |s| s.games.contains_key(&13058)).await;
    {
        let s = r.core.store();
        assert_eq!(s.user_title(156), "builderman");
        let seen = &s.account_data(ME).unwrap().last_seen;
        assert!(seen.contains_key(&1) && seen.contains_key(&156));
        let in_game = s.friends_in_game(ME);
        assert_eq!(in_game.len(), 1);
        assert_eq!(in_game[0].1.kind, PresenceKind::InGame);
        let g = s.game_for_place(1818).unwrap();
        assert_eq!(g.name, "Crossroads");
        assert!(g.up_votes.is_some());
    }
    r.core.send(Cmd::LoadServers {
        account: ME,
        place_id: 1818,
        more: false,
    });
    wait_for(&r.core, "servers", |s| {
        s.servers.get(&1818).is_some_and(|l| l.data.is_some())
    })
    .await;
    let s = r.core.store();
    let list = s.servers[&1818].data.as_ref().unwrap();
    let srv = list.iter().find(|x| x.job_id == job).unwrap();
    assert_eq!(srv.friends, vec![1]);
    assert!(
        list.iter()
            .filter(|x| x.job_id != job)
            .all(|x| x.friends.is_empty())
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn friend_missing_from_a_fresh_list_is_reported() {
    let mut f = replay::load("core_signin");
    for i in &mut f.interactions {
        if i.request.path.ends_with("/friends") {
            i.response.body = serde_json::json!({"data":[{"id":1,"name":"Roblox","displayName":"Roblox"}]});
        }
    }
    let replay = Replay::from_fixture(&f).await;
    let dir = tempfile::tempdir().unwrap();
    let cookies = seeded(&dir);
    {
        let cache = Cache::open(&Paths::in_dir(dir.path()).db_file()).unwrap();
        cache.put(Kind::Friends, ME, "friends", &vec![1_u64, 156]);
    }
    let r = rig(Base::Replay(replay.server.uri()), dir, cookies);
    wait_for(&r.core, "unfriend notice", |s| {
        s.toasts
            .iter()
            .any(|t| t.account == Some(ME) && t.text == "User 156 is no longer your friend.")
    })
    .await;
    let s = r.core.store();
    let d = s.account_data(ME).unwrap();
    assert_eq!(d.friends.data.as_deref(), Some(&[1_u64][..]));
    assert!(d.unfriended.contains_key(&156));
    assert!(!d.unfriended.contains_key(&1));
    assert!(
        d.feed
            .iter()
            .any(|n| n.user_id == Some(156) && n.category == "friends" && !n.read),
        "listed with notifications"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn experience_page_loads_without_sign_in() {
    let mut f = replay::load("place_universe");
    f.interactions.extend(
        replay::load("core_place")
            .interactions
            .into_iter()
            .filter(|i| !i.request.path.contains("multiget")),
    );
    let replay = Replay::from_fixture(&f).await;
    let r = rig(
        Base::Replay(replay.server.uri()),
        tempfile::tempdir().unwrap(),
        Arc::default(),
    );
    r.core.send(Cmd::LoadPlace {
        account: AccountId::ANONYMOUS,
        place_id: 1818,
    });
    wait_for(&r.core, "game details", |s| s.games.contains_key(&13058)).await;
    r.core.send(Cmd::LoadServers {
        account: AccountId::ANONYMOUS,
        place_id: 1818,
        more: false,
    });
    wait_for(&r.core, "servers", |s| {
        s.servers.get(&1818).is_some_and(|l| l.data.is_some())
    })
    .await;
    let s = r.core.store();
    let g = s.game_for_place(1818).unwrap();
    assert_eq!(g.name, "Crossroads");
    assert_eq!(
        (g.genre_l1.as_str(), g.genre_l2.as_str()),
        ("Action", "Battlegrounds & Fighting")
    );
    assert_eq!(g.avatar_type, "Player choice");
    assert!(!g.copying_allowed && g.gear.is_empty());
    assert!(s.toasts.iter().all(|t| !t.error));
}

/// The UI asks for missing games on every store change. A load that is in flight or just
/// failed must not be queued again, or jobs pile up while the rate limiter drains them.
#[tokio::test(flavor = "multi_thread")]
async fn wanted_universes_load_once_while_pending_or_failed() {
    let replay = Replay::from_fixture(&Fixture::default()).await;
    let r = rig(
        Base::Replay(replay.server.uri()),
        tempfile::tempdir().unwrap(),
        Arc::default(),
    );
    for _ in 0..50 {
        r.core.want_universes(&[13058, 13058, 0]);
        assert!(r.core.store().jobs <= 1);
    }
    wait_for(&r.core, "the load to finish", |s| s.jobs == 0).await;
    r.core.want_universes(&[13058]);
    tokio::time::sleep(Duration::from_millis(100)).await;
    let games = replay
        .requests()
        .await
        .into_iter()
        .filter(|q| q.url.path() == "/games.roblox.com/v1/games")
        .count();
    assert_eq!(games, 1);
}

#[tokio::test(flavor = "multi_thread")]
async fn home_sorts_signed_out() {
    let replay = Replay::from_fixture(&replay::load("explore_sorts")).await;
    let r = rig(
        Base::Replay(replay.server.uri()),
        tempfile::tempdir().unwrap(),
        Arc::default(),
    );
    r.core.ensure_home(AccountId::ANONYMOUS);
    // A second call while loading sends nothing (the fixture answers once).
    r.core.ensure_home(AccountId::ANONYMOUS);
    wait_for(&r.core, "sorts", |s| {
        s.account_data(AccountId::ANONYMOUS)
            .is_some_and(|d| d.home.data.is_some())
    })
    .await;
    let s = r.core.store();
    let home = s
        .account_data(AccountId::ANONYMOUS)
        .unwrap()
        .home
        .data
        .clone()
        .unwrap();
    assert!(home.len() >= 3, "{home:?}");
    assert!(home.iter().any(|s| s.title == "Top Trending"));
    assert!(
        home.iter()
            .all(|s| !s.games.is_empty() && s.games.iter().all(|g| g.place_id > 0))
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn remote_search_finds_games() {
    let mut f = replay::load("omni_search");
    // People and community search for the same query come back empty.
    for (host, path) in [
        ("users.roblox.com", "/v1/users/search"),
        ("groups.roblox.com", "/v1/groups/search"),
    ] {
        f.interactions.push(Interaction {
            request: replay::RecordedRequest {
                method: "GET".into(),
                host: host.into(),
                path: path.into(),
                ..Default::default()
            },
            response: replay::RecordedResponse {
                status: 200,
                body: serde_json::json!({"data": []}),
                ..Default::default()
            },
        });
    }
    // The search results page asks again, for experiences only.
    let omni = f
        .interactions
        .iter()
        .find(|i| i.request.host == "apis.roblox.com")
        .cloned()
        .unwrap();
    f.interactions.push(omni);
    let replay = Replay::from_fixture(&f).await;
    let r = rig(
        Base::Replay(replay.server.uri()),
        tempfile::tempdir().unwrap(),
        Arc::default(),
    );
    // The command bar asks on every store change; only the first ask searches (each
    // recorded response is served once).
    r.core.ensure_remote_search(AccountId::ANONYMOUS, " adopt me");
    r.core.ensure_remote_search(AccountId::ANONYMOUS, "adopt me ");
    wait_for(&r.core, "results", |s| s.remote.data.is_some()).await;
    assert_eq!(r.core.store().remote.data.as_ref().unwrap().query, "adopt me");
    {
        let s = r.core.store();
        let rows = &s.remote.data.as_ref().unwrap().rows;
        let adopt = rows
            .iter()
            .find(|e| {
                e.target
                    == Target::Experience {
                        universe_id: 383310974,
                        place_id: 920587237,
                    }
            })
            .unwrap_or_else(|| panic!("{rows:?}"));
        assert_eq!(adopt.group, Group::Experiences);
        assert!(adopt.subtitle.contains("playing"), "{}", adopt.subtitle);
    }
    let page = poll(|| {
        r.core
            .search_results(AccountId::ANONYMOUS, Scope::Experiences, " adopt me")
    })
    .await;
    assert!(!page.is_empty() && page.iter().all(|e| e.group == Group::Experiences));
    // Recent searches: newest first, each once whatever its case.
    let a = AccountId::ANONYMOUS;
    for q in ["adopt me", "obby", "Adopt Me", "  "] {
        r.core.note_search(a, q);
    }
    assert_eq!(r.core.recent_searches(a), vec!["Adopt Me", "obby"]);
    r.core.clear_searches(a);
    assert!(r.core.recent_searches(a).is_empty());
}

#[tokio::test(flavor = "multi_thread")]
async fn people_profile_lists_and_relation() {
    let mut f = replay::load("people");
    // The username lookup runs twice: once for an action, once as a resource.
    let lookup = f
        .interactions
        .iter()
        .find(|i| i.request.path == "/v1/usernames/users")
        .cloned()
        .unwrap();
    f.interactions.push(lookup);
    let replay = Replay::from_fixture(&f).await;
    let dir = tempfile::tempdir().unwrap();
    let cookies = seeded(&dir);
    let r = rig(Base::Replay(replay.server.uri()), dir, cookies);
    let c = &r.core;
    // A username is looked up (captured 2026-10-14), then the action runs with its id.
    let found = Arc::new(parking_lot::Mutex::new(None));
    let f2 = found.clone();
    c.with_user("@BuilderMan", move |_, id| *f2.lock() = Some(id))
        .unwrap();
    let start = Instant::now();
    while found.lock().is_none() {
        assert!(start.elapsed() < Duration::from_secs(10), "lookup timed out");
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    assert_eq!(*found.lock(), Some(156));
    assert!(c.with_user("  ", |_, _| {}).is_err());
    // The same lookup as a resource, for screens that take a name.
    assert_eq!(*poll(|| c.user_by_name("BuilderMan")).await, 156);
    // Ask every frame until it lands, like a screen does.
    let p = poll(|| c.profile(156)).await;
    assert_eq!(p.name, "builderman");
    assert!(p.verified && p.followers > 1_000_000);
    let rel = poll(|| c.relation(ME, 1)).await;
    assert_eq!(rel.friend, bonsai_core::people::FriendState::NotFriends);
    assert!(!rel.following && !rel.blocked);
    let reqs = poll(|| c.friend_requests(ME, false)).await;
    assert_eq!(reqs.items.len(), 1);
    assert_eq!(reqs.items[0].user_id, 2_000_001);
    assert_eq!(reqs.items[0].origin, "InGame");
    let followers = poll(|| c.followers(ME, 156, false)).await;
    assert_eq!(followers.items.len(), 100);
    assert!(followers.next.is_some());
    assert!(poll(|| c.followings(ME, 156, false)).await.items.len() > 10);
    assert!(poll(|| c.friends_of(156, false)).await.items.is_empty());
    assert!(poll(|| c.username_history(156)).await.is_empty());
    let groups = poll(|| c.user_groups(1)).await;
    assert!(groups.iter().any(|g| g.name == "Roblox Wiki"));
    let badges = poll(|| c.user_badges(ME, 156, false)).await;
    assert!(!badges.items.is_empty() && badges.items.iter().all(|b| b.place_id > 0));
    let games = poll(|| c.user_creations(1)).await;
    assert!(games.iter().any(|g| g.name == "Chaos Canyon"));
    assert!(poll(|| c.blocked_users(ME)).await.is_empty());
    // Names and types come with the look, so items not in the catalog (a classic T-shirt) still show.
    let look = poll(|| c.user_look(156)).await;
    assert_eq!(look.rig, "R6");
    assert_eq!(
        look.assets.iter().map(|a| (a.id, a.type_id)).collect::<Vec<_>>(),
        vec![(1_080_951, 8), (1_142_116, 2)]
    );
    let outfits = poll(|| c.user_outfits(156, false)).await;
    assert_eq!(outfits.items.len(), 5);
    assert!(outfits.next.is_none());
    let outfit = poll(|| c.outfit(outfits.items[3].id)).await;
    assert!(outfit.assets.iter().any(|a| a.name == "Roblox 2.0 Torso"));
    assert!(outfit.emotes.is_empty());
    // Yourself is left out of the batch.
    let follows = poll(|| c.follows(ME, &[156, 2_000_001, ME.0])).await;
    assert_eq!(follows.get(&156), Some(&true));
    assert_eq!(follows.get(&2_000_001), Some(&false));
    // Nicknames and notes stay on this computer, per account.
    c.set_nickname(ME, 156, "  Builder ");
    c.set_note(ME, 156, "Met in Crossroads");
    let n = c.user_note(ME, 156);
    assert_eq!(
        (n.nickname.as_str(), n.note.as_str()),
        ("Builder", "Met in Crossroads")
    );
    assert!(c.user_note(AccountId(2), 156).nickname.is_empty());
    c.set_nickname(ME, 156, "");
    assert_eq!(c.user_note(ME, 156).note, "Met in Crossroads");
    c.set_note(ME, 156, "");
    assert!(c.user_notes(ME).is_empty());
}

#[tokio::test(flavor = "multi_thread")]
async fn player_stats_from_rolimons_only_when_values_are_on() {
    use bonsai_core::values::BadgeGroup;
    let replay = Replay::from_fixture(&replay::load("player_values")).await;
    let dir = tempfile::tempdir().unwrap();
    let cookies = seeded(&dir);
    let r = rig(Base::Replay(replay.server.uri()), dir, cookies);
    let c = &r.core;
    // Off by default: no request, no data.
    assert!(c.player_stats(156).data.is_none());
    c.update_settings(|s| s.item_values = true);
    let b = poll(|| c.player_stats(156)).await;
    assert!(b.tracked && b.value.unwrap() > 0 && b.rank.is_some());
    assert!(b.last_online.is_some() && b.stats_updated.is_some());
    let roblox = poll(|| c.player_stats(1)).await;
    assert_eq!(roblox.rank, Some(1));
    assert_eq!(roblox.badges[0].group, BadgeGroup::Value);
    assert!(roblox.badges.iter().all(|b| b.group != BadgeGroup::Unknown));
    // Never scanned: untracked, not an error.
    let none = poll(|| c.player_stats(5_000_000_123)).await;
    assert!(!none.tracked && none.badges.is_empty());
    let reqs = replay.requests().await;
    let third: Vec<_> = reqs
        .iter()
        .filter(|q| q.url.path().starts_with("/api.rolimons.com"))
        .collect();
    assert_eq!(third.len(), 3);
    assert!(third.iter().all(|q| !q.headers.contains_key("cookie")));
}

#[tokio::test(flavor = "multi_thread")]
async fn wiki_articles_only_when_turned_on() {
    use bonsai_core::wiki::{About, Block};
    let replay = Replay::from_fixture(&replay::load("wiki")).await;
    let dir = tempfile::tempdir().unwrap();
    let cookies = seeded(&dir);
    let r = rig(Base::Replay(replay.server.uri()), dir, cookies);
    let c = &r.core;
    let dominus = |id| About::Item {
        id,
        name: "Dominus Infernus".into(),
    };
    // Off by default: no request, no data.
    assert!(c.wiki_about(&dominus(31_101_391)).data.is_none());
    assert!(c.wiki_search("roblox", "dominus").data.is_none());
    assert!(c.wiki_page("w:c:jailbreak:Torero").data.is_none());
    c.update_settings(|s| s.wiki = true);
    let p = poll(|| c.wiki_about(&dominus(31_101_391))).await;
    let a = p.article.as_ref().unwrap();
    assert_eq!(a.title, "Catalog:Dominus Infernus");
    assert_eq!(a.revision, 1_833_149);
    assert_eq!(a.infobox_title, "Dominus Infernus");
    assert!(
        a.categories.contains(&"Hats".to_owned()) && !a.categories.iter().any(|c| c.starts_with("Pages ")),
        "hidden categories dropped: {:?}",
        a.categories
    );
    assert!(
        a.extract()
            .starts_with("Dominus Infernus is a [[Limited unique|limited unique]] [[Hat|hat]]")
    );
    let titles: Vec<_> = a.sections.iter().map(|s| s.title.as_str()).collect();
    assert_eq!(
        titles,
        ["History", "Release history", "Appearance", "Contents", "Trivia"]
    );
    assert!(matches!(a.sections[1].blocks[..], [Block::Table { .. }]));
    // The same page is cached: a different item with that name isn't this one.
    let other = c.wiki_about(&dominus(5));
    assert!(other.data.unwrap().article.is_none());
    let missing = poll(|| {
        c.wiki_about(&About::Item {
            id: 9,
            name: "No Such Hat".into(),
        })
    })
    .await;
    assert!(missing.article.is_none());
    let hits = poll(|| c.wiki_search("roblox", "  dominus ")).await;
    assert_eq!(hits.len(), 50);
    assert_eq!(hits[0].title, "Dominus (series)");
    assert!(hits.iter().any(|h| h.title == "Catalog:Dominus Infernus"));
    let a = poll(|| c.wiki_about(&dominus(31_101_391))).await;
    let a = a.article.as_ref().unwrap();
    let image = a.image.as_ref().unwrap();
    assert!(
        image
            .src
            .starts_with("roblox/images/0/01/Dominus_Infernus.png/revision/latest/")
    );
    // Links to other sites open only when the article has them. (Turning the wiki on swapped in
    // the system opener, so the accepted link isn't opened here.)
    let link = "https://create.roblox.com/store/asset/31100020/";
    assert!(a.links.iter().any(|l| l == link), "{:?}", a.links);
    assert!(
        c.open_wiki_link("Catalog:Dominus Infernus", "https://example.com/")
            .is_err()
    );
    assert!(c.open_wiki_link("Catalog:No Such Hat", link).is_err());

    // An experience's own wiki: same API on its own host, titled `w:c:<wiki>:`.
    let t = poll(|| c.wiki_page("w:c:jailbreak:Torero")).await;
    let t = t.article.as_ref().unwrap();
    assert_eq!(
        (t.title.as_str(), t.site.as_str(), t.main),
        ("w:c:jailbreak:Torero", "Jailbreak Wiki", false)
    );
    assert!(
        t.image
            .as_ref()
            .unwrap()
            .src
            .starts_with("rblx-jailbreak/images/")
    );
    assert!(
        t.infobox
            .iter()
            .any(|f| f.label == "Successor" && f.value == "[[w:c:jailbreak:Snake|Snake]]"),
        "links stay on that wiki: {:?}",
        t.infobox
    );
    let hits = poll(|| c.wiki_search("jailbreak", "torero")).await;
    assert_eq!(hits.len(), 50);
    assert_eq!(hits[0].title, "w:c:jailbreak:Torero");
    // Not a wiki name: nothing to fetch.
    assert!(c.wiki_search("evil.com/x", "torero").data.is_none());

    let reqs = replay.requests().await;
    for (host, n) in [("/roblox.fandom.com", 3), ("/jailbreak.fandom.com", 3)] {
        let third: Vec<_> = reqs.iter().filter(|q| q.url.path().starts_with(host)).collect();
        assert_eq!(third.len(), n, "{host}");
        assert!(third.iter().all(|q| !q.headers.contains_key("cookie")));
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn friends_search_pages() {
    let replay = Replay::from_fixture(&replay::load("friends_search")).await;
    let dir = tempfile::tempdir().unwrap();
    let cookies = seeded(&dir);
    let r = rig(Base::Replay(replay.server.uri()), dir, cookies);
    let c = &r.core;
    // Case and spaces don't make a new search.
    let first = poll(|| c.search_friends_of(ME, 156, " BU ", false)).await;
    assert_eq!(first.items, vec![9_000_000_101, 9_000_000_102]);
    assert!(first.next.is_some());
    let start = Instant::now();
    loop {
        let p = c.search_friends_of(ME, 156, "bu", true);
        if let Some(d) = p.data.filter(|d| d.items.len() == 3) {
            assert!(d.next.is_none());
            break;
        }
        assert!(p.error.is_none(), "error: {:?}", p.error);
        assert!(start.elapsed() < Duration::from_secs(10), "second page timed out");
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}

/// Poll a resource until it has data.
async fn poll<T>(mut f: impl FnMut() -> bonsai_core::res::Res<T>) -> Arc<T> {
    let start = Instant::now();
    loop {
        let r = f();
        if let Some(d) = r.data {
            return d;
        }
        assert!(r.error.is_none(), "error: {:?}", r.error);
        assert!(start.elapsed() < Duration::from_secs(10), "timed out");
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn people_actions_toast_and_report_errors() {
    let mut f = replay::load("friends_actions");
    f.interactions.extend(replay::load("blocking").interactions);
    let replay = Replay::from_fixture(&f).await;
    let dir = tempfile::tempdir().unwrap();
    let cookies = seeded(&dir);
    let r = rig(Base::Replay(replay.server.uri()), dir, cookies);
    let c = &r.core;
    let toast = |text: &'static str| move |s: &StoreData| s.toasts.iter().any(|t| t.text == text);
    c.add_friend(ME, 156);
    wait_for(c, "request sent", toast("Friend request sent")).await;
    c.accept_friend(ME, 2_000_001);
    wait_for(c, "accepted", toast("Friend request accepted")).await;
    c.decline_friend(ME, 2_000_002);
    wait_for(
        c,
        "decline error",
        toast("Could not decline the request: Friend request does not exist"),
    )
    .await;
    c.decline_all_friends(ME);
    wait_for(c, "declined all", toast("All friend requests declined")).await;
    c.unfriend(ME, 156);
    wait_for(c, "unfriended", toast("Removed from friends")).await;
    c.follow(ME, 1);
    c.unfollow(ME, 1);
    c.block(ME, 156);
    wait_for(c, "blocked", toast("Blocked")).await;
    c.unblock(ME, 156);
    wait_for(c, "unblocked", toast("Unblocked")).await;
    wait_for(c, "idle", |s| s.jobs == 0).await;
    let posts = replay
        .requests()
        .await
        .into_iter()
        .filter(|q| q.method.as_str() == "POST")
        .count();
    assert!(posts >= 9, "{posts}");
    // Nothing marked the account expired.
    assert_ne!(c.store().account_data(ME).unwrap().session, SessionState::Expired);
}

#[tokio::test(flavor = "multi_thread")]
async fn experience_page_resources() {
    let replay = Replay::from_fixture(&replay::load("experience")).await;
    let dir = tempfile::tempdir().unwrap();
    let cookies = seeded(&dir);
    let r = rig(Base::Replay(replay.server.uri()), dir, cookies);
    let c = &r.core;
    const U: u64 = 383_310_974;
    const P: u64 = 920_587_237;
    let media = poll(|| c.game_media(U)).await;
    assert!(!media.is_empty() && media.iter().all(|m| m.image_id > 0));
    let me = poll(|| c.my_game_state(ME, U)).await;
    assert!(!me.can_vote && me.vote.is_none());
    assert_eq!(me.vote_hint, "Play it first to vote.");
    assert!(!me.favorited && !me.following && me.can_follow);
    let m = poll(|| c.maturity(U)).await;
    assert!(!m.label.is_empty(), "{m:?}");
    let passes = poll(|| c.game_passes(ME, U, false)).await;
    assert!(!passes.items.is_empty());
    assert!(passes.items.iter().any(|p| p.price.is_some()));
    assert!(passes.items.iter().all(|p| !p.owned));
    let badges = poll(|| c.game_badges(U, false)).await;
    assert!(!badges.items.is_empty());
    let ids: Vec<u64> = badges.items.iter().map(|b| b.id).collect();
    assert!(poll(|| c.badges_earned(ME, &ids)).await.is_empty());
    let events = poll(|| c.game_events(ME, U)).await;
    assert!(events.iter().all(|e| !e.going && !e.id.is_empty()));
    let rec = poll(|| c.recommended_games(U)).await;
    assert!(!rec.is_empty() && rec.iter().all(|g| g.place_id > 0));
    assert!(poll(|| c.private_servers(ME, P, false)).await.items.is_empty());
    let offer = poll(|| c.private_server_offer(ME, U, P)).await;
    assert!(offer.enabled && offer.can_create);
    assert_eq!(offer.price, Some(0));

    // Reversible actions recorded live: favorite, notify, event RSVP on and off.
    c.favorite(ME, U, true);
    assert!(c.my_game_state(ME, U).data.unwrap().favorited, "optimistic");
    c.favorite(ME, U, false);
    c.follow_game(ME, U, true);
    c.follow_game(ME, U, false);
    // The recorded event. Not `events.first()`: that list drops ended events by wall clock.
    const EVENT: &str = "8203642938119553696";
    c.rsvp(ME, U, EVENT, true);
    c.rsvp(ME, U, EVENT, false);
    wait_for(c, "idle", |s| s.jobs == 0).await;
    assert!(
        c.store().toasts.iter().all(|t| !t.error),
        "{:?}",
        c.store().toasts
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn friends_servers_list_the_friends_in_each() {
    let replay = Replay::from_fixture(&replay::load("friend_servers")).await;
    let dir = tempfile::tempdir().unwrap();
    let cookies = seeded(&dir);
    let r = rig(Base::Replay(replay.server.uri()), dir, cookies);
    let c = &r.core;
    assert!(c.friend_servers(AccountId::ANONYMOUS, 920_587_237).data.is_none());
    let list = poll(|| c.friend_servers(ME, 920_587_237)).await;
    assert_eq!(list.len(), 1, "the server without a job id is dropped");
    assert_eq!(list[0].job_id, "00000000-0000-0000-0000-000000000001");
    assert_eq!(list[0].friends, vec![156]);
    assert_eq!(c.store().users.get(&156).unwrap().display_name, "builderman");
}

#[tokio::test(flavor = "multi_thread")]
async fn experience_vote_and_private_server_actions() {
    let replay = Replay::from_fixture(&replay::load("experience_actions")).await;
    let dir = tempfile::tempdir().unwrap();
    let cookies = seeded(&dir);
    let r = rig(Base::Replay(replay.server.uri()), dir, cookies);
    let c = &r.core;
    let toast = |text: &'static str| move |s: &StoreData| s.toasts.iter().any(|t| t.text == text);
    c.vote(ME, 383_310_974, true);
    wait_for(c, "vote error", toast("Could not vote: Play game required")).await;
    c.vote(ME, 13058, false);
    c.buy_private_server(ME, 383_310_974, "Test server".into(), 0);
    wait_for(c, "bought", toast("Private server bought.")).await;
    let cfg = poll(|| c.server_config(ME, 5_000_001)).await;
    assert!(cfg.active && cfg.renews && !cfg.friends_allowed);
    assert_eq!(cfg.expires, "2026-11-12T00:00:00Z");
    use bonsai_core::experience::ServerChange;
    assert!(c.server_link(ME, 5_000_001).is_ok());
    c.change_server(ME, 5_000_001, ServerChange::Rename("New".into()));
    // Toggles flip the loaded state and show it before the request finishes.
    c.toggle_server(ME, 5_000_001, ServerChange::FriendsAllowed)
        .unwrap();
    let now = c.server_config(ME, 5_000_001).data.unwrap();
    assert!(now.friends_allowed);
    assert_eq!(now.name, "New");
    c.change_server(ME, 5_000_001, ServerChange::NewLink);
    wait_for(c, "new link", toast("New link made. The old one stops working.")).await;
    c.toggle_server(ME, 5_000_001, ServerChange::Renew).unwrap();
    wait_for(
        c,
        "renew off",
        toast("It won't renew. It stays until it expires."),
    )
    .await;
    wait_for(c, "idle", |s| s.jobs == 0).await;
    let reqs = replay.requests().await;
    let n = |m: &str, p: &str| {
        reqs.iter()
            .filter(|q| q.method.as_str() == m && q.url.path() == format!("/games.roblox.com{p}"))
            .count()
    };
    assert_eq!(n("PATCH", "/v1/games/13058/user-votes"), 1);
    assert_eq!(n("POST", "/v1/games/vip-servers/383310974"), 1);
    assert_eq!(n("PATCH", "/v1/vip-servers/5000001"), 2);
    assert_eq!(n("PATCH", "/v1/vip-servers/5000001/permissions"), 1);
    assert_eq!(n("PATCH", "/v1/vip-servers/5000001/subscription"), 1);
    let buy = reqs
        .iter()
        .find(|q| q.url.path() == "/games.roblox.com/v1/games/vip-servers/383310974")
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&buy.body).unwrap();
    assert_eq!(body["expectedPrice"], 0);
    assert_eq!(body["isPurchaseConfirmed"], true);
}

#[tokio::test(flavor = "multi_thread")]
async fn buying_items_resale_copies_and_passes() {
    use bonsai_core::purchase::Seller;
    let replay = Replay::from_fixture(&replay::load("purchases")).await;
    let dir = tempfile::tempdir().unwrap();
    let cookies = seeded(&dir);
    let r = rig(Base::Replay(replay.server.uri()), dir, cookies);
    let c = &r.core;
    let toast = |text: &'static str| move |s: &StoreData| s.toasts.iter().any(|t| t.text == text);
    let silly = "c04aa0b7-af31-4805-afa5-773274f31add";
    let valk = "0d1d5570-68e5-49d8-8c9e-fdb777863bd0";

    // From the creator: sale details first, then the purchase, then the new balance.
    c.buy_item(ME, silly.into(), "Silly Fun".into(), 0, Seller::Creator);
    wait_for(c, "bought item", toast("Bought Silly Fun.")).await;
    wait_for(c, "balance", |s| {
        s.account_data(ME).and_then(|d| d.robux) == Some(90)
    })
    .await;

    // The cheapest resale copy, refused for the balance.
    c.buy_item(ME, valk.into(), "Valkyrie Helm".into(), 279_900, Seller::Cheapest);
    wait_for(
        c,
        "no robux",
        toast("Could not buy the item: You don't have enough Robux."),
    )
    .await;

    c.buy_pass(ME, 3_714_256_301, "1.25x Candy Multiplier".into(), 50);
    wait_for(c, "bought pass", toast("Bought 1.25x Candy Multiplier.")).await;
    wait_for(c, "balance", |s| {
        s.account_data(ME).and_then(|d| d.robux) == Some(40)
    })
    .await;
    c.buy_pass(ME, 3_714_256_301, "1.25x Candy Multiplier".into(), 50);
    wait_for(
        c,
        "2sv",
        toast("Could not buy the pass: Roblox wants you to verify this purchase. Buy it on the website instead."),
    )
    .await;
    wait_for(c, "idle", |s| s.jobs == 0).await;

    let reqs = replay.requests().await;
    let body = |p: &str| -> serde_json::Value {
        let q = reqs
            .iter()
            .rfind(|q| q.method.as_str() == "POST" && q.url.path() == format!("/apis.roblox.com{p}"))
            .unwrap_or_else(|| panic!("no POST {p}"));
        serde_json::from_slice(&q.body).unwrap()
    };
    let b = body(&format!("/marketplace-sales/v1/item/{silly}/purchase-item"));
    assert_eq!(b["collectibleProductId"], "a9d8fd64-463e-4cf8-b0af-2d842a550b11");
    assert_eq!(b["expectedPrice"], 0);
    assert_eq!(b["expectedCurrency"], 1);
    assert_eq!(b["expectedSellerId"], 1);
    assert_eq!(b["expectedSellerType"], "User");
    assert_eq!(b["expectedPurchaserId"], ME.0);
    assert_eq!(b["expectedPurchaserType"], "User");
    assert_eq!(b["idempotencyKey"].as_str().unwrap().len(), 36);
    assert!(b.get("collectibleItemInstanceId").is_none());

    let b = body(&format!("/marketplace-sales/v1/item/{valk}/purchase-resale"));
    assert_eq!(
        b["collectibleItemInstanceId"],
        "3c662a1f-409b-40a9-a3d8-787ebabc8e0d"
    );
    assert_eq!(b["collectibleProductId"], "b7ae0bff-18ca-4970-82a5-e739fbaf1296");
    assert_eq!(b["expectedSellerId"], 9_000_000_005_i64);
    assert_eq!(b["expectedPrice"], 279_900);

    let b = body("/game-passes/v1/game-passes/3714256301/purchase");
    assert_eq!(b["expectedPrice"], 50);
    assert!(b["idempotencyKey"].is_string());
    // A new key for each attempt, so a retry by the user isn't taken for the same purchase.
    let keys: std::collections::HashSet<_> = reqs
        .iter()
        .filter(|q| q.url.path().ends_with("/purchase"))
        .map(|q| serde_json::from_slice::<serde_json::Value>(&q.body).unwrap()["idempotencyKey"].clone())
        .filter(|k| k.is_string())
        .collect();
    assert_eq!(keys.len(), 2);
}

#[tokio::test(flavor = "multi_thread")]
async fn buying_refuses_a_changed_price_without_spending() {
    use bonsai_core::purchase::Seller;
    let replay = Replay::from_fixture(&replay::load("purchases")).await;
    let dir = tempfile::tempdir().unwrap();
    let cookies = seeded(&dir);
    let r = rig(Base::Replay(replay.server.uri()), dir, cookies);
    let c = &r.core;
    // The user saw 5 Robux; Roblox now says it's free. Nothing is bought.
    c.buy_item(
        ME,
        "c04aa0b7-af31-4805-afa5-773274f31add".into(),
        "Silly Fun".into(),
        5,
        Seller::Creator,
    );
    wait_for(c, "refused", |s| {
        s.toasts
            .iter()
            .any(|t| t.text == "Could not buy the item: The price is now free. Look again before buying.")
    })
    .await;
    wait_for(c, "idle", |s| s.jobs == 0).await;
    let reqs = replay.requests().await;
    assert!(!reqs.iter().any(|q| q.url.path().contains("/purchase")));
}

#[tokio::test(flavor = "multi_thread")]
async fn discover_charts_feed_and_favorites() {
    let replay = Replay::from_fixture(&replay::load("discover")).await;
    let dir = tempfile::tempdir().unwrap();
    let cookies = seeded(&dir);
    let r = rig(Base::Replay(replay.server.uri()), dir, cookies);
    let c = &r.core;
    let first = poll(|| c.charts(ME, "computer", "all", false)).await;
    let filters = first.items.iter().find(|s| !s.filters.is_empty()).unwrap();
    let device = filters.filters.iter().find(|f| f.kind == "device").unwrap();
    assert_eq!(device.selected, "computer");
    assert!(device.options.iter().any(|o| o.0 == "console"));
    assert!(
        first
            .items
            .iter()
            .any(|s| s.id == "top-trending" && !s.games.is_empty())
    );
    assert!(first.next.is_some());
    let _ = c.charts(ME, "computer", "all", true);
    let both = poll(|| {
        let r = c.charts(ME, "computer", "all", false);
        if r.data
            .as_ref()
            .is_some_and(|p| p.next.is_none() || p.items.len() > first.items.len())
        {
            r
        } else {
            Default::default()
        }
    })
    .await;
    assert!(both.items.iter().any(|s| s.id == "top-rated"));
    let top = poll(|| c.chart(ME, "top-trending", "computer", "all")).await;
    assert!(!top.games.is_empty() && !top.info.is_empty());
    assert!(top.games.iter().all(|g| !g.genre.is_empty()));
    let feed = poll(|| c.home_feed(ME)).await;
    assert!(feed.iter().any(|s| s.title == "Recommended For You"));
    // Home folds this topic into Continue.
    assert!(
        feed.iter()
            .any(|s| s.id == bonsai_core::discover::CONTINUE_TOPIC && s.title == "Continue")
    );
    let titles: std::collections::HashSet<_> = feed.iter().map(|s| &s.title).collect();
    assert_eq!(titles.len(), feed.len(), "no duplicate topics");
    assert!(feed.iter().all(|s| !s.games.is_empty()));
    let fav = poll(|| c.favorite_games(156, false)).await;
    assert!(fav.items.iter().all(|g| g.place_id > 0 && !g.name.is_empty()));
    assert!(!fav.items.is_empty());
    assert!(poll(|| c.favorite_games(ME.0, false)).await.items.is_empty());
    assert!(poll(|| c.followed_games(ME)).await.is_empty());
}

#[tokio::test(flavor = "multi_thread")]
async fn private_messages_and_notifications() {
    let replay = Replay::from_fixture(&replay::load("messages")).await;
    let dir = tempfile::tempdir().unwrap();
    let cookies = seeded(&dir);
    let r = rig(Base::Replay(replay.server.uri()), dir, cookies);
    let c = &r.core;
    let inbox = poll(|| c.messages(ME, "inbox", false)).await;
    let m = inbox.items.first().unwrap();
    assert!(m.system && !m.subject.is_empty());
    assert!(!m.body.contains('<'), "body is plain text");
    let _ = poll(|| c.messages(ME, "news", false)).await;
    let n = poll(|| c.notifications(ME, false)).await;
    assert!(n.items.iter().all(|x| !x.id.is_empty()));
}

#[tokio::test(flavor = "multi_thread")]
async fn notification_stream_categories() {
    let replay = Replay::from_fixture(&replay::load("notifications")).await;
    let dir = tempfile::tempdir().unwrap();
    let cookies = seeded(&dir);
    let r = rig(Base::Replay(replay.server.uri()), dir, cookies);
    let n = poll(|| r.core.notifications(ME, false)).await;
    let cat = |c: &str| n.items.iter().filter(|x| x.category == c).count();
    assert!(cat("friends") > 0 && cat("account") > 0);
    assert_eq!(cat("other"), 0, "every recorded entry has a category");
    let accepted = n
        .items
        .iter()
        .find(|x| x.kind == "FriendRequestAccepted")
        .unwrap();
    assert!(accepted.user_id.is_some() && accepted.body.contains("accepted"));
}

#[tokio::test(flavor = "multi_thread")]
async fn marketplace_item_page_and_public_inventory() {
    use bonsai_core::inventory::InvSlot;
    use bonsai_core::market::{CatalogQuery, CatalogSort, ItemKind, ItemRef, Limited};
    let mut f = replay::load("market");
    f.interactions.extend(replay::load("values").interactions);
    let replay = Replay::from_fixture(&f).await;
    let dir = tempfile::tempdir().unwrap();
    let cookies = seeded(&dir);
    let r = rig(Base::Replay(replay.server.uri()), dir, cookies);
    let c = &r.core;

    let cats = poll(|| c.catalog_categories()).await;
    assert_eq!(cats[0].name, "All");
    assert!(
        cats.iter()
            .any(|c| c.name == "Clothing" && !c.subcategories.is_empty())
    );
    let q = CatalogQuery {
        taxonomy: cats[0].taxonomy.clone(),
        sort: CatalogSort::Favorited,
        creator: "Roblox".into(),
        ..Default::default()
    };
    let page = poll(|| c.catalog_search(&q, false)).await;
    assert!(!page.items.is_empty() && page.next.is_some());
    assert!(
        page.items
            .iter()
            .all(|i| i.creator == "Roblox" && !i.name.is_empty())
    );

    // A classic limited: off sale, sold only by resellers.
    let helm = ItemRef::asset(1_365_767);
    let d = poll(|| c.item(helm)).await;
    assert_eq!(d.card.name, "Valkyrie Helm");
    assert_eq!(d.card.limited, Limited::Limited);
    assert!(d.card.off_sale && d.has_resellers);
    assert!(d.card.buy_price().is_some_and(|p| p > 0));
    assert_eq!(d.card.type_name(), "Hat");
    assert_eq!(d.updated, "2016-11-16T20:10:05.697Z");
    assert_eq!(d.genres, ["Medieval", "Fighting"]);
    assert!(d.tags.contains(&"Head Accessories".to_string()), "{:?}", d.tags);
    assert!(d.tags.contains(&"Tradable".to_string()), "{:?}", d.tags);
    assert!(d.tags.contains(&"helmet".to_string()), "{:?}", d.tags);
    let bundle = poll(|| c.item(ItemRef::bundle(299_652))).await;
    assert_eq!(bundle.card.item.kind, ItemKind::Bundle);
    assert_eq!(bundle.card.price_label(), "Free");
    assert!(bundle.contents.len() >= 2, "{:?}", bundle.contents);
    assert!(
        bundle.tags.contains(&"Adjustable head".to_string()),
        "{:?}",
        bundle.tags
    );
    assert!(bundle.tags.contains(&"Animated".to_string()), "{:?}", bundle.tags);

    let recs = poll(|| c.item_recommendations(helm, 8)).await;
    assert!(!recs.is_empty() && recs.iter().all(|i| i.item.kind == ItemKind::Asset));
    let sellers = poll(|| c.item_resellers(&d.card.collectible_id, false)).await;
    assert!(!sellers.items.is_empty());
    assert!(
        sellers.items.windows(2).all(|w| w[0].price <= w[1].price),
        "cheapest first"
    );
    let h = poll(|| c.item_price_history(&d.card.collectible_id)).await;
    assert!(h.rap.is_some() && h.prices.len() == 60);
    assert!(h.prices.windows(2).all(|w| w[0].0 < w[1].0), "oldest first");
    let owners = poll(|| c.item_owners(ME, helm.id, false)).await;
    assert!(!owners.items.is_empty());

    // Builderman's inventory.
    let inv_cats = poll(|| c.inventory_categories(156)).await;
    assert!(
        inv_cats
            .iter()
            .any(|c| c.slots.contains(&("Head".into(), InvSlot::Asset(8))))
    );
    let slots: Vec<InvSlot> = inv_cats.iter().flat_map(|c| &c.slots).map(|s| s.1).collect();
    assert!(
        slots.iter().all(|s| !matches!(s, InvSlot::Asset(21 | 34 | 9))),
        "badges, passes and places are listed elsewhere"
    );
    for s in [
        InvSlot::Badges,
        InvSlot::Passes,
        InvSlot::Places,
        InvSlot::Asset(13),
        InvSlot::Asset(3),
    ] {
        assert!(slots.contains(&s), "{s:?} in {slots:?}");
    }
    assert!(*poll(|| c.can_view_inventory(ME, 156)).await);
    let col = poll(|| c.collectibles(ME, 156)).await;
    assert!(!col.is_empty() && col.iter().any(|i| i.rap.is_some()));
    let hats = poll(|| c.inventory(ME, 156, InvSlot::Asset(8), false, false)).await;
    assert!(!hats.items.is_empty() && hats.items.iter().all(|i| i.copy_id > 0));
    let bodies = poll(|| c.inventory(ME, 156, InvSlot::Bundle(1), false, false)).await;
    assert!(bodies.items.iter().all(|i| i.item.kind == ItemKind::Bundle));

    // Values are off by default: no request, no data.
    assert!(c.item_values().data.is_none());
    c.update_settings(|s| s.item_values = true);
    let v = poll(|| c.item_values()).await;
    let t = bonsai_core::inventory::totals(&col, Some(&v));
    assert!(t.value.unwrap() >= t.rap / 2, "{t:?}");
    let reqs = replay.requests().await;
    assert_eq!(
        reqs.iter()
            .filter(|q| q.url.path().starts_with("/api.rolimons.com"))
            .count(),
        1
    );
    assert!(
        reqs.iter()
            .filter(|q| q.url.path().starts_with("/api.rolimons.com"))
            .all(|q| !q.headers.contains_key("cookie"))
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn inventory_passes_badges_places_and_asset_info() {
    let replay = Replay::from_fixture(&replay::load("inventory")).await;
    let dir = tempfile::tempdir().unwrap();
    let cookies = seeded(&dir);
    let r = rig(Base::Replay(replay.server.uri()), dir, cookies);
    let c = &r.core;

    let passes = poll(|| c.user_passes(156, false)).await;
    assert!(
        passes.items.is_empty() && passes.next.is_none(),
        "builderman owns none"
    );
    let badges = poll(|| c.user_badges(ME, 156, false)).await;
    assert!(!badges.items.is_empty());
    let places = poll(|| c.user_places(156, false)).await;
    assert!(places.items.len() > 5, "{}", places.items.len());
    assert!(places.next.is_none(), "one page of 100");
    let martian = &places.items[0];
    assert_eq!((martian.place_id, martian.universe_id), (458_486, 63_778));
    assert_eq!(martian.name, "Martian Invasion");
    assert_eq!(
        (martian.creator.as_str(), martian.creator_is_group),
        ("builderman", false)
    );
    assert!(
        places
            .items
            .iter()
            .all(|p| p.place_id > 0 && p.price.is_none_or(|x| x > 0))
    );

    // A decal: not in the catalog, so the drawer reads economy details.
    let decal = poll(|| c.asset_info(41_372_326)).await;
    assert_eq!(decal.asset_type, 13);
    assert_eq!(decal.name, "builderman");
    assert!(!decal.creator.is_empty() && decal.creator_id > 0 && !decal.creator_is_group);
    assert_eq!(decal.price, None);
    assert!(decal.created.starts_with("2010-12-02"));
    assert!(decal.public_domain && !decal.premium_only);

    // A pass: its own page reads product info.
    let pass = poll(|| c.pass_info(1)).await;
    assert_eq!(pass.name, "Never get banned");
    assert_eq!((pass.creator.as_str(), pass.creator_is_group), ("user1", false));
    assert_eq!(pass.product_id, 3_613_243_303);
    assert_eq!(pass.price, None);
    assert!(pass.created.starts_with("2017-05-24"));
}

#[tokio::test(flavor = "multi_thread")]
async fn trades_lists_details_and_actions() {
    use bonsai_core::market::ItemRef;
    use bonsai_core::trades::{Draft, TradeBox, Warning, evaluate};
    let replay = Replay::from_fixture(&replay::load("trades")).await;
    let dir = tempfile::tempdir().unwrap();
    let cookies = seeded(&dir);
    let r = rig(Base::Replay(replay.server.uri()), dir, cookies);
    let c = &r.core;
    let toast = |text: &'static str| move |s: &StoreData| s.toasts.iter().any(|t| t.text == text);

    let inbound = poll(|| c.trades(ME, TradeBox::Inbound, false)).await;
    assert_eq!(inbound.items.len(), 2);
    assert_eq!(inbound.items[0].partner, 2_000_001);
    assert_eq!(inbound.items[0].status, "Open");
    // Partner names are known without another call.
    assert_eq!(c.store().users.get(&2_000_001).unwrap().display_name, "User One");

    let t = poll(|| c.trade(ME, 7001)).await;
    assert_eq!(t.mine.user_id, ME.0, "participant B is us");
    assert_eq!(t.mine.items.len(), 2);
    assert_eq!(t.theirs.robux, 500);
    assert_eq!(t.theirs.items[0].item, ItemRef::asset(1_365_767));
    let rules = poll(|| c.trade_rules(ME)).await;
    assert_eq!(rules.max_items, 4);
    let e = evaluate(
        &t.mine.items,
        t.mine.robux,
        &t.theirs.items,
        t.theirs.robux,
        None,
        &rules,
    );
    assert_eq!(e.get.robux, 350, "Robux after the 30% fee");
    assert!(e.warnings.iter().any(|w| matches!(w, Warning::OnHold(_))));
    assert!(poll(|| c.can_trade(ME)).await.can_trade);
    let with = poll(|| c.can_trade_with(ME, 2_000_003)).await;
    assert!(!with.can_trade && with.reason == "They can't trade right now.");
    let mine = poll(|| c.tradable_items(ME, ME.0, false)).await;
    assert_eq!(mine.items.len(), 2, "one row per copy");
    assert!(
        mine.items
            .iter()
            .all(|i| i.name == "The Classic ROBLOX Fedora" && i.rap == Some(406_464))
    );
    assert!(mine.items[1].on_hold);

    // The composer's draft stays local until sent.
    c.start_trade(ME, 2_000_001);
    assert_eq!(
        c.send_trade_draft(ME).unwrap_err(),
        "One side of the trade is empty."
    );
    assert!(
        c.toggle_draft_item(ME, true, &mine.items[1].instance_id)
            .unwrap_err()
            .contains("on hold")
    );
    c.toggle_draft_item(ME, true, &mine.items[0].instance_id).unwrap();
    assert_eq!(c.trade_draft(ME).unwrap().give.len(), 1);
    c.toggle_draft_item(ME, true, &mine.items[0].instance_id).unwrap();
    assert!(
        c.trade_draft(ME).unwrap().give.is_empty(),
        "a second click takes it off"
    );
    c.start_counter(ME, 7001).unwrap();
    let d = c.trade_draft(ME).unwrap();
    assert_eq!(
        (d.counter_of, d.partner, d.give.len()),
        (Some(7001), 2_000_001, 2)
    );
    c.discard_trade_draft(ME);
    assert!(c.trade_draft(ME).is_none() && c.trade_draft(ALT).is_none());

    c.accept_trade(ME, 7001);
    // Gone from the list at once.
    assert!(
        c.trades(ME, TradeBox::Inbound, false)
            .data
            .unwrap()
            .items
            .iter()
            .all(|t| t.id != 7001)
    );
    wait_for(c, "accepted", toast("Trade accepted")).await;
    wait_for(c, "unread trades", |s| {
        s.account_data(ME).unwrap().unread.trades == 1
    })
    .await;
    c.decline_trade(ME, 7002);
    wait_for(
        c,
        "decline error",
        toast("Could not decline the trade: The trade is inactive."),
    )
    .await;
    let draft = Draft {
        partner: 2_000_001,
        give: vec![mine.items[0].clone()],
        get: t.theirs.items.clone(),
        give_robux: 0,
        get_robux: 100,
        counter_of: None,
    };
    c.send_trade(ME, draft.clone());
    wait_for(c, "sent", toast("Trade sent")).await;
    c.send_trade(
        ME,
        Draft {
            counter_of: Some(7002),
            ..draft
        },
    );
    wait_for(c, "countered", toast("Counter offer sent")).await;
    wait_for(c, "idle", |s| s.jobs == 0).await;
    let reqs = replay.requests().await;
    let send = reqs
        .iter()
        .find(|q| q.url.path() == "/trades.roblox.com/v2/trades/send")
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&send.body).unwrap();
    assert_eq!(body["senderOffer"]["userId"], ME.0);
    assert_eq!(
        body["senderOffer"]["collectibleItemInstanceIds"][0],
        "bbbbbbbb-0000-0000-0000-000000000001"
    );
    assert_eq!(body["recipientOffer"]["userId"], 2_000_001);
    assert_eq!(body["recipientOffer"]["robux"], 100);
}

#[tokio::test(flavor = "multi_thread")]
async fn item_ownership_favorites_wishlist_and_delete() {
    use bonsai_core::market::{CatalogItem, ItemRef};
    let replay = Replay::from_fixture(&replay::load("trades")).await;
    let dir = tempfile::tempdir().unwrap();
    let cookies = seeded(&dir);
    let r = rig(Base::Replay(replay.server.uri()), dir, cookies);
    let c = &r.core;
    let helm = ItemRef::asset(1_365_767);
    assert!(*poll(|| c.owns(ME, helm)).await);
    assert!(
        !*poll(|| c.item_favorited(ME, helm)).await,
        "a null body means not favorited"
    );
    c.favorite_item(ME, helm, true);
    c.favorite_item(ME, helm, false);
    c.delete_inventory_item(ME, 1_028_606);
    wait_for(c, "deleted", |s| {
        s.toasts.iter().any(|t| t.text == "Deleted from your inventory")
    })
    .await;
    wait_for(c, "idle", |s| s.jobs == 0).await;
    assert!(
        c.store().toasts.iter().all(|t| !t.error),
        "{:?}",
        c.store().toasts
    );

    // The wishlist is local and per account.
    let card = CatalogItem {
        item: helm,
        name: "Valkyrie Helm".into(),
        lowest_resale: Some(279_900),
        limited: bonsai_core::market::Limited::Limited,
        off_sale: true,
        ..Default::default()
    };
    assert!(c.toggle_wishlist(ME, &card));
    assert!(c.in_wishlist(ME, helm) && !c.in_wishlist(ALT, helm));
    assert_eq!(c.wishlist(ME)[0].price_then, Some(279_900));
    assert!(!c.toggle_wishlist(ME, &card));
    assert!(c.wishlist(ME).is_empty());
}

#[tokio::test(flavor = "multi_thread")]
async fn failed_chat_send_is_kept_for_resend() {
    let mut f = replay::load("chat");
    f.interactions
        .retain(|i| !i.request.path.ends_with("/send-messages"));
    let replay = Replay::from_fixture(&f).await;
    let dir = tempfile::tempdir().unwrap();
    let cookies = seeded(&dir);
    let r = rig(Base::Replay(replay.server.uri()), dir, cookies);
    let c = &r.core;
    let _ = poll(|| c.chat_messages(ME, "conv-1", false)).await;
    c.send_chat(ME, "conv-1", "lost");
    wait_for(c, "send failed", |s| s.jobs == 0).await;
    let failed = c.failed_chat(ME, "conv-1");
    assert_eq!(failed.len(), 1);
    assert!(failed[0].failed && !failed[0].pending && failed[0].text == "lost");
    assert!(c.failed_chat(ME, "conv-2").is_empty(), "kept per conversation");
    assert!(!c.discard_chat(ME, "conv-1", "nope"));
    assert!(c.discard_chat(ME, "conv-1", &failed[0].id));
    assert!(c.failed_chat(ME, "conv-1").is_empty());
}

#[tokio::test(flavor = "multi_thread")]
async fn chat_conversations_send_typing_and_manage() {
    use bonsai_api::realtime::Event;
    use bonsai_core::chat::ConvKind;
    let replay = Replay::from_fixture(&replay::load("chat")).await;
    let dir = tempfile::tempdir().unwrap();
    let cookies = seeded(&dir);
    let r = rig(Base::Replay(replay.server.uri()), dir, cookies);
    let c = &r.core;
    let toast = |text: &'static str| move |s: &StoreData| s.toasts.iter().any(|t| t.text == text);

    let convs = poll(|| c.conversations(ME, false)).await;
    assert_eq!(convs.items.len(), 2);
    let one = &convs.items[0];
    assert_eq!(
        (one.kind, one.partner(ME.0), one.unread),
        (ConvKind::OneToOne, Some(2_000_001), 2)
    );
    assert_eq!(one.title(ME.0), "User One");
    assert_eq!(convs.items[1].title(ME.0), "Squad");
    assert_eq!(convs.items[1].kind, ConvKind::Group);
    assert_eq!(c.store().users.get(&2_000_002).unwrap().display_name, "User Two");
    let s = poll(|| c.chat_settings(ME)).await;
    assert!(s.enabled && s.group_enabled);

    let t = poll(|| c.chat_messages(ME, "conv-1", false)).await;
    assert_eq!(t.items.len(), 3, "newest first");
    assert!(t.items[2].moderated);
    assert_eq!(bonsai_core::text::linked_places(&t.items[0].text), vec![1818]);

    c.mark_chat_read(ME, "conv-1");
    assert_eq!(
        c.conversations(ME, false).data.unwrap().items[0].unread,
        0,
        "read at once"
    );
    c.send_chat(ME, "conv-1", "  on my way  ");
    let shown = c.chat_messages(ME, "conv-1", false).data.unwrap();
    assert!(
        shown.items[0].pending && shown.items[0].text == "on my way",
        "shows before the server answers"
    );
    wait_for(c, "sent", |s| s.jobs == 0).await;
    let start = Instant::now();
    loop {
        let p = poll(|| c.chat_messages(ME, "conv-1", false)).await;
        if p.items[0].id == "m4" && !p.items[0].pending {
            break;
        }
        assert!(
            start.elapsed() < Duration::from_secs(10),
            "server copy replaces the pending one"
        );
        tokio::time::sleep(Duration::from_millis(20)).await;
    }

    // An existing one-to-one chat opens without a request; a new person starts one.
    c.start_chat(ME, vec![2_000_001, ME.0], None);
    assert_eq!(c.take_chat_focus(ME).as_deref(), Some("conv-1"));
    c.start_chat(ME, vec![2_000_002], None);
    let start = Instant::now();
    while c
        .store()
        .account_data(ME)
        .and_then(|d| d.chat_focus.clone())
        .is_none()
    {
        assert!(start.elapsed() < Duration::from_secs(10), "new chat focus");
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    assert_eq!(c.take_chat_focus(ME).as_deref(), Some("conv-3"));
    assert_eq!(c.take_chat_focus(ME), None, "focus is taken once");

    c.rename_chat(ME, "conv-2", "Squad 2");
    wait_for(c, "renamed", toast("Renamed")).await;
    c.add_to_chat(ME, "conv-2", vec![2_000_003]);
    wait_for(c, "added", toast("Added to the chat")).await;
    c.remove_from_chat(ME, "conv-2", 2_000_003);
    wait_for(c, "removed", toast("Removed from the chat")).await;
    c.chat_typing(ME, "conv-1");
    c.chat_typing(ME, "conv-1");

    // Realtime: typing shows, a new message from them clears it.
    let ev = |kind: &str| Event {
        account: ME,
        namespace: "ChatNotifications".into(),
        detail: serde_json::json!({"Type": kind, "ConversationId": "conv-1", "UserId": 2_000_001, "SenderUserId": 2_000_001, "IsTyping": true}),
    };
    c.apply_realtime(&ev("ParticipantTyping"));
    assert_eq!(c.typing_in(ME, "conv-1"), vec![2_000_001]);
    assert!(c.typing_in(ME, "conv-2").is_empty());
    c.apply_realtime(&ev("NewMessage"));
    assert!(c.typing_in(ME, "conv-1").is_empty());

    c.toggle_pin_chat(ME, "conv-2");
    assert_eq!(c.pinned_chats(ME), vec!["conv-2".to_string()]);
    c.toggle_pin_chat(ME, "conv-2");
    assert!(c.pinned_chats(ME).is_empty());

    wait_for(c, "idle", |s| s.jobs == 0).await;
    assert!(
        !c.store().toasts.iter().any(|t| t.error),
        "{:?}",
        c.store().toasts
    );
    // Typing status is fire-and-forget; wait for it to land.
    let start = Instant::now();
    while !replay
        .requests()
        .await
        .iter()
        .any(|q| q.url.path().ends_with("/update-typing-status") && q.headers.contains_key("x-csrf-token"))
    {
        assert!(start.elapsed() < Duration::from_secs(10), "typing status sent");
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    let reqs = replay.requests().await;
    let body = |p: &str| -> serde_json::Value {
        let q = reqs
            .iter()
            .find(|q| q.url.path() == format!("/apis.roblox.com/platform-chat-api/v1/{p}"))
            .unwrap_or_else(|| panic!("{p} was not called"));
        serde_json::from_slice(&q.body).unwrap()
    };
    assert_eq!(body("send-messages")["messages"][0]["content"], "on my way");
    assert_eq!(body("send-messages")["conversation_id"], "conv-1");
    assert_eq!(
        body("create-conversations")["conversations"][0]["type"],
        "one_to_one"
    );
    assert_eq!(
        body("create-conversations")["conversations"][0]["participant_user_ids"],
        serde_json::json!([2_000_002])
    );
    assert_eq!(
        body("update-conversations")["conversations"][0]["name"],
        "Squad 2"
    );
    assert_eq!(
        body("mark-conversations")["conversation_ids"],
        serde_json::json!(["conv-1"])
    );
    let typing = reqs
        .iter()
        .filter(|q| q.url.path().ends_with("/update-typing-status") && q.headers.contains_key("x-csrf-token"))
        .count();
    assert_eq!(typing, 1, "typing status is throttled");
}

#[tokio::test(flavor = "multi_thread")]
async fn group_chat_creation_deduplicates_people_and_keeps_accounts_separate() {
    let replay = Replay::from_fixture(&replay::load("chat")).await;
    let dir = tempfile::tempdir().unwrap();
    let cookies = seeded(&dir);
    let r = rig(Base::Replay(replay.server.uri()), dir, cookies);
    let c = &r.core;
    let _ = poll(|| c.chat_settings(ME)).await;
    c.start_chat(
        ME,
        vec![ME.0, 0, 2_000_001, 2_000_002, 2_000_001],
        Some("  Squad  ".into()),
    );
    wait_for(c, "group focus", |s| {
        s.account_data(ME).is_some_and(|d| d.chat_focus.is_some())
    })
    .await;
    assert!(c.take_chat_focus(AccountId(9_000_001)).is_none());
    assert_eq!(c.take_chat_focus(ME).as_deref(), Some("conv-3"));
    let requests = replay.requests().await;
    let create = requests
        .iter()
        .find(|q| q.url.path().ends_with("/create-conversations") && q.headers.contains_key("x-csrf-token"))
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&create.body).unwrap();
    assert_eq!(body["conversations"][0]["type"], "group");
    assert_eq!(body["conversations"][0]["name"], "Squad");
    assert_eq!(
        body["conversations"][0]["participant_user_ids"],
        serde_json::json!([2_000_001, 2_000_002])
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn group_chat_creation_checks_privacy_before_writing_without_cached_settings() {
    let mut fixture = replay::load("chat");
    for interaction in &mut fixture.interactions {
        if interaction.request.path.ends_with("/metadata") {
            interaction.response.body["isGroupChatEnabledByPrivacySetting"] = serde_json::json!("disabled");
        }
    }
    let replay = Replay::from_fixture(&fixture).await;
    let dir = tempfile::tempdir().unwrap();
    let cookies = seeded(&dir);
    let r = rig(Base::Replay(replay.server.uri()), dir, cookies);
    r.core.start_chat(ME, vec![2_000_001, 2_000_002], None);
    wait_for(&r.core, "privacy error", |s| {
        s.toasts
            .iter()
            .any(|t| t.error && t.text.contains("Group chat is off"))
    })
    .await;
    assert!(r.core.take_chat_focus(ME).is_none());
    assert!(replay.requests().await.iter().all(|r| r.method == "GET"));
}

#[tokio::test(flavor = "multi_thread")]
async fn leaving_a_group_removes_only_the_acting_account() {
    let replay = Replay::from_fixture(&replay::load("chat")).await;
    let dir = tempfile::tempdir().unwrap();
    let cookies = seeded(&dir);
    let r = rig(Base::Replay(replay.server.uri()), dir, cookies);
    let _ = poll(|| r.core.conversations(ME, false)).await;
    r.core.leave_chat(ME, "conv-2");
    assert!(
        r.core
            .conversations(ME, false)
            .data
            .unwrap()
            .items
            .iter()
            .all(|c| c.id != "conv-2")
    );
    wait_for(&r.core, "left group", |s| {
        s.toasts.iter().any(|t| t.text == "You left the chat")
    })
    .await;
    let requests = replay.requests().await;
    let remove = requests
        .iter()
        .find(|q| q.url.path().ends_with("/remove-users") && q.headers.contains_key("x-csrf-token"))
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&remove.body).unwrap();
    assert_eq!(body["conversation_id"], "conv-2");
    assert_eq!(body["user_ids"], serde_json::json!([ME.0]));
}

#[tokio::test(flavor = "multi_thread")]
async fn group_chat_management_rejects_direct_messages_and_non_owner_removal() {
    let mut fixture = replay::load("chat");
    for interaction in &mut fixture.interactions {
        if interaction.request.path.ends_with("/get-user-conversations") {
            interaction.response.body["conversations"][1]["created_by"] = serde_json::json!(2_000_001);
        }
    }
    let replay = Replay::from_fixture(&fixture).await;
    let dir = tempfile::tempdir().unwrap();
    let cookies = seeded(&dir);
    let r = rig(Base::Replay(replay.server.uri()), dir, cookies);
    let c = &r.core;
    let _ = poll(|| c.conversations(ME, false)).await;
    c.rename_chat(ME, "conv-1", "Oops");
    c.add_to_chat(ME, "conv-1", vec![2_000_002]);
    c.leave_chat(ME, "conv-1");
    c.remove_from_chat(ME, "conv-2", 2_000_002);
    assert!(
        c.store()
            .toasts
            .iter()
            .any(|t| t.text == "Only the group creator can remove people.")
    );
    assert_eq!(c.conversations(ME, false).data.unwrap().items.len(), 2);
    assert!(replay.requests().await.iter().all(|r| r.method == "GET"));
}

#[tokio::test(flavor = "multi_thread")]
async fn communities_member_admin_forums_and_public_pages() {
    use bonsai_core::communities::{Affiliation, Setting};
    let mut f = replay::load("communities");
    f.interactions
        .extend(replay::load("communities_public").interactions);
    let replay = Replay::from_fixture(&f).await;
    let dir = tempfile::tempdir().unwrap();
    let cookies = seeded(&dir);
    let r = rig(Base::Replay(replay.server.uri()), dir, cookies);
    let c = &r.core;
    let g = 7001;

    let d = poll(|| c.community(g)).await;
    assert_eq!(
        (d.name.as_str(), d.owner.as_str(), d.members),
        ("Test Community", "User Four", 4)
    );
    assert!(!d.public_entry);
    let shout = d.shout.clone().unwrap();
    assert_eq!(
        (
            shout.text.as_str(),
            shout.poster.as_str(),
            shout.poster_username.as_str()
        ),
        ("Event Friday at 6!", "User One", "user1")
    );

    let m = poll(|| c.membership(ME, g)).await;
    assert!(m.is_member() && !m.owner && !m.primary);
    assert_eq!(m.role.as_ref().unwrap().name, "Admin");
    let p = &m.perms;
    assert!(p.post_shout && p.change_rank && p.invite_members && p.remove_members && p.view_audit_log);
    assert!(p.view_payouts && p.manage_settings && p.create_posts && p.remove_comments);
    assert!(!p.ban_members && !p.spend_funds);

    let roles = poll(|| c.community_roles(g)).await;
    let names: Vec<&str> = roles.iter().map(|r| r.name.as_str()).collect();
    assert_eq!(names, ["Owner", "Admin", "Member"], "top rank first, no Guest");
    assert_eq!(
        poll(|| c.community_members(ME, g, 70, false)).await.items.len(),
        2
    );

    let mine = poll(|| c.user_groups(ME.0)).await;
    assert_eq!(mine[0].group_id, 7002, "primary first");
    let mine_here = c.my_accounts_in(g);
    assert_eq!(mine_here.len(), 1);
    assert_eq!((mine_here[0].account, mine_here[0].role.as_str()), (ME, "Admin"));
    assert_eq!(poll(|| c.pending_communities(ME)).await[0].id, 7003);

    let reqs = poll(|| c.join_requests(ME, g, false)).await;
    assert_eq!(
        reqs.items.iter().map(|x| x.user_id).collect::<Vec<_>>(),
        [2_000_003, 2_000_002]
    );
    let log = poll(|| c.audit_log(ME, g, false)).await;
    assert_eq!(log.items[0].action, "ChangeRank");
    assert_eq!(log.items[0].text, "changed user1's rank from Member to Admin");
    assert_eq!(log.items[0].actor_role, "Owner");
    assert_eq!(log.items[1].action, "PostStatus");
    assert_eq!(log.items[1].text, "posted a shout: “Event Friday at 6!”");
    let s = poll(|| c.community_settings(ME, g)).await;
    assert!(s.approval_required && !s.enemies_allowed && s.games_visible);
    assert_eq!(*poll(|| c.community_funds(ME, g)).await, 12_345);
    let pay = poll(|| c.community_payouts(ME, g)).await;
    assert_eq!(pay.iter().map(|p| p.percent).collect::<Vec<_>>(), [60, 10]);
    assert_eq!(poll(|| c.community_links(ME, g)).await[0].kind, "Discord");

    let cats = poll(|| c.forum_categories(ME, g)).await;
    let names: Vec<&str> = cats.iter().map(|c| c.name.as_str()).collect();
    assert_eq!(names, ["General", "Off topic"], "ranked first, archived hidden");
    let posts = poll(|| c.forum_posts(ME, g, "cat-1", false)).await;
    assert_eq!(
        posts.items.iter().map(|p| p.id.as_str()).collect::<Vec<_>>(),
        ["post-1", "post-2"]
    );
    assert!(posts.items[0].pinned && posts.items[0].locked);
    let p2 = posts.items[1].clone();
    assert!(!p2.updated.is_empty());
    assert_eq!(
        (p2.replies, p2.unread, p2.text.as_str(), p2.first_comment.as_str()),
        (2, true, "Share yours.", "c-2")
    );
    assert_eq!(p2.author, "User One");
    assert_eq!(
        poll(|| c.forum_comments(ME, g, "cat-1", "post-2", false))
            .await
            .items
            .len(),
        3
    );

    c.mark_shout_seen(ME, g, &shout.updated);
    assert_eq!(c.seen_shouts(ME).get("7001"), Some(&shout.updated));

    // Every write once.
    c.post_shout(ME, g, "New shout");
    c.answer_join_requests(ME, g, vec![2_000_003], true);
    c.answer_join_requests(ME, g, vec![2_000_002, 2_000_004], false);
    assert!(
        c.join_requests(ME, g, false).data.unwrap().items.is_empty(),
        "answered requests leave the list at once"
    );
    c.set_member_role(ME, g, 2_000_001, 71);
    c.kick_member(ME, g, 2_000_002);
    c.set_primary_community(ME, Some(g));
    c.set_community_setting(ME, g, Setting::ApprovalRequired, false);
    assert!(!c.community_settings(ME, g).data.unwrap().approval_required);
    c.create_forum_post(ME, g, "cat-1", "New post", "Hello all");
    c.reply_forum_post(ME, g, "cat-1", &p2, "Thanks");
    c.delete_forum_comment(ME, g, "cat-1", "post-2", "c-3");
    c.delete_forum_post(ME, g, "cat-1", "post-1");
    c.join_community(ME, 7003);
    c.leave_community(ME, 7002);
    c.pay_out(ME, g, 2_000_001, 45);
    assert_eq!(
        *c.community_funds(ME, g).data.unwrap(),
        12_300,
        "funds drop at once"
    );
    assert!(
        c.set_recurring_payouts(ME, g, vec![(2_000_001, 80), (2_000_002, 30)])
            .is_err(),
        "more than 100% is refused before sending"
    );
    c.set_recurring_payouts(ME, g, vec![(2_000_001, 70), (2_000_002, 30)])
        .unwrap();
    assert_eq!(
        c.community_payouts(ME, g)
            .data
            .unwrap()
            .iter()
            .map(|p| p.percent)
            .collect::<Vec<_>>(),
        [70, 30]
    );
    c.add_affiliate(ME, g, Affiliation::Ally, 7004);
    c.remove_affiliate(ME, g, Affiliation::Ally, 7005);
    c.add_affiliate(ME, g, Affiliation::Enemy, 7006);
    c.remove_affiliate(ME, g, Affiliation::Enemy, 7007);
    wait_for(c, "idle", |s| s.jobs == 0).await;
    assert!(
        !c.store().toasts.iter().any(|t| t.error),
        "{:?}",
        c.store().toasts
    );
    let reqs = replay.requests().await;
    let body = |m: &str, p: &str| -> serde_json::Value {
        let q = reqs
            .iter()
            .find(|q| q.method.as_str() == m && q.url.path() == p)
            .unwrap_or_else(|| panic!("{m} {p} was not called"));
        serde_json::from_slice(&q.body).unwrap_or(serde_json::Value::Null)
    };
    assert_eq!(
        body("PATCH", "/groups.roblox.com/v1/groups/7001/status")["message"],
        "New shout"
    );
    assert_eq!(
        body("DELETE", "/groups.roblox.com/v1/groups/7001/join-requests")["UserIds"],
        serde_json::json!([2_000_002, 2_000_004])
    );
    assert_eq!(
        body("PATCH", "/groups.roblox.com/v1/groups/7001/users/2000001")["roleId"],
        71
    );
    assert_eq!(
        body("POST", "/groups.roblox.com/v1/user/groups/primary")["groupId"],
        7001
    );
    assert_eq!(
        body("PATCH", "/groups.roblox.com/v1/groups/7001/settings")["isApprovalRequired"],
        false
    );
    let post = body("POST", "/groups.roblox.com/v1/groups/7001/forums/cat-1/posts");
    assert_eq!(
        (post["title"].as_str(), post["content"].as_str()),
        (Some("New post"), Some("Hello all"))
    );
    assert_eq!(
        body(
            "POST",
            "/groups.roblox.com/v1/groups/7001/forums/cat-1/posts/post-2/comments"
        )["repliesToPostCommentId"],
        "c-2"
    );
    let pay = body("POST", "/groups.roblox.com/v1/groups/7001/payouts");
    assert_eq!(pay["PayoutType"], 1, "FixedAmount");
    assert_eq!(
        (
            &pay["Recipients"][0]["recipientId"],
            &pay["Recipients"][0]["amount"]
        ),
        (&serde_json::json!(2_000_001), &serde_json::json!(45))
    );
    assert_eq!(pay["IdempotencyKey"].as_str().map(str::len), Some(36));
    let split = body("POST", "/groups.roblox.com/v1/groups/7001/payouts/recurring");
    assert_eq!(split["PayoutType"], 2, "Percentage");
    assert_eq!(split["Recipients"][1]["amount"], 30);
    for (m, p) in [
        ("POST", "Allies/7004"),
        ("DELETE", "Allies/7005"),
        ("POST", "Enemies/7006"),
        ("DELETE", "Enemies/7007"),
    ] {
        body(m, &format!("/groups.roblox.com/v1/groups/7001/relationships/{p}"));
    }

    // Public pages, signed out data shared by every account.
    let big = 3_959_677;
    assert!(poll(|| c.community(big)).await.members > 1_000_000);
    assert!(!poll(|| c.community_games(big)).await.is_empty());
    let store = poll(|| c.community_store(big, false)).await;
    assert!(!store.items.is_empty() && store.items.iter().all(|i| i.creator_id == big));
    assert!(
        !poll(|| c.search_communities("BIG Games", false))
            .await
            .items
            .is_empty()
    );
    let roles = poll(|| c.community_roles(big)).await;
    let biggest = roles.iter().max_by_key(|r| r.members).unwrap().id;
    assert_eq!(
        poll(|| c.community_members(ME, big, biggest, false))
            .await
            .items
            .len(),
        100
    );
    let a = poll(|| c.community_affiliates(big)).await;
    assert!(!a.allies.is_empty() && a.enemies.is_empty());
    assert!(
        poll(|| c.community_name_history(big))
            .await
            .iter()
            .all(|n| !n.is_empty())
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn avatar_wear_outfits_and_rig() {
    use bonsai_core::avatar::Rig;
    let replay = Replay::from_fixture(&replay::load("avatar")).await;
    let dir = tempfile::tempdir().unwrap();
    let cookies = seeded(&dir);
    let r = rig(Base::Replay(replay.server.uri()), dir, cookies);
    let c = &r.core;
    let toast = |text: &'static str| move |s: &StoreData| s.toasts.iter().any(|t| t.text == text);
    let idle = |s: &StoreData| s.jobs == 0;

    let look = poll(|| c.avatar(ME)).await;
    assert_eq!(look.rig, "R6");
    assert_eq!(look.assets.len(), 2);
    assert_eq!(
        (look.assets[0].id, look.assets[0].type_name.as_str()),
        (1_080_951, "Hat")
    );
    let outfits = poll(|| c.outfits(ME, false)).await;
    assert_eq!(outfits.items.len(), 5);
    assert!(outfits.next.is_none(), "an empty token is the last page");
    assert_eq!(outfits.items[3].id, 24_477_597);

    // Taking the hat off sends what is worn, minus the hat.
    let rules = poll(|| c.avatar_rules()).await;
    assert!(rules.slots.iter().any(|s| s.type_id == 8 && s.max == 3));
    c.toggle_wear(ME, 1_080_951, Some(8));
    wait_for(c, "worn", idle).await;
    let reqs = replay.requests().await;
    let wear = reqs
        .iter()
        .find(|q| q.url.path() == "/avatar.roblox.com/v2/avatar/set-wearing-assets")
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&wear.body).unwrap();
    assert_eq!(body["assets"].as_array().unwrap().len(), 1);
    assert_eq!(body["assets"][0]["id"], 1_142_116);

    c.wear_outfit(ME, 24_477_597);
    wait_for(c, "outfit", toast("Outfit on.")).await;
    let reqs = replay.requests().await;
    let colors = reqs
        .iter()
        .find(|q| q.url.path() == "/avatar.roblox.com/v2/avatar/set-body-colors")
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&colors.body).unwrap();
    assert_eq!(body["torsoColor3"], "111111");

    assert!(c.save_outfit(ME, "  ").is_err());
    c.save_outfit(ME, "Work").unwrap();
    wait_for(c, "saved", toast("Outfit saved.")).await;
    let reqs = replay.requests().await;
    let create = reqs
        .iter()
        .find(|q| q.url.path() == "/avatar.roblox.com/v3/outfits/create")
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&create.body).unwrap();
    assert_eq!(body["name"], "Work");
    assert_eq!(body["playerAvatarType"], "R6");
    assert_eq!(body["bodyColor3s"]["headColor3"], "F5CD30");
    assert_eq!(body["assets"].as_array().unwrap().len(), 2);

    c.set_rig(ME, Rig::R15);
    wait_for(c, "rig", idle).await;
    // One T-shirt at a time: a new one replaces the old.
    c.toggle_wear(ME, 777, Some(2));
    wait_for(c, "swapped", idle).await;
    let reqs = replay.requests().await;
    let wear = reqs
        .iter()
        .rfind(|q| q.url.path() == "/avatar.roblox.com/v2/avatar/set-wearing-assets")
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&wear.body).unwrap();
    let ids: Vec<i64> = body["assets"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| a["id"].as_i64().unwrap())
        .collect();
    assert_eq!(ids, [1_080_951, 777]);
    c.toggle_wear(ME, 99, None);
    wait_for(
        c,
        "refused",
        toast("Could not change your avatar: Roblox would not wear Some Hat."),
    )
    .await;

    let updates = async || -> Vec<serde_json::Value> {
        replay
            .requests()
            .await
            .iter()
            .filter(|q| q.url.path() == "/avatar.roblox.com/v3/outfits/24477597")
            .map(|q| serde_json::from_slice(&q.body).unwrap())
            .collect()
    };
    // Replacing keeps the outfit's name and takes what is worn now.
    c.update_outfit(ME, 24_477_597);
    wait_for(c, "updated", toast("Outfit updated.")).await;
    let sent = updates().await;
    assert_eq!(sent[0]["name"], "2015");
    assert_eq!(sent[0]["assets"].as_array().unwrap().len(), 2);
    assert_eq!(sent[0]["bodyColor3s"]["headColor3"], "F5CD30");

    // Renaming keeps the outfit's contents.
    assert!(c.rename_outfit(ME, 24_477_597, " ").is_err());
    c.rename_outfit(ME, 24_477_597, " Old ").unwrap();
    wait_for(c, "renamed", toast("Outfit renamed.")).await;
    let sent = updates().await;
    assert_eq!(sent[1]["name"], "Old");
    assert_eq!(sent[1], serde_json::json!({"name": "Old"}));

    c.delete_outfit(ME, 24_477_597);
    wait_for(c, "deleted", toast("Outfit deleted.")).await;
    c.redraw_avatar(ME);
    wait_for(c, "redrawn", toast("Avatar redrawn.")).await;

    assert!(c.set_emote(ME, 9, None).is_err());
    c.set_emote(ME, 2, Some(3_333_499_508)).unwrap();
    wait_for(c, "emote equipped", toast("Emote equipped.")).await;
    c.set_emote(ME, 2, None).unwrap();
    wait_for(c, "emote cleared", toast("Emote slot cleared.")).await;

    let recent = poll(|| c.recent_items(ME)).await;
    assert_eq!(recent.len(), 2);
    assert!(!recent[0].outfit && recent[1].outfit);

    // Body colors: only the named parts change; colors outside the palette are refused.
    use bonsai_core::avatar::{Part, Scale};
    assert_eq!(look.colors.head, 226);
    assert!(rules.palette.iter().any(|s| s.id == 1003 && !s.basic));
    assert!(rules.palette.iter().any(|s| s.id == 364 && s.basic));
    assert!(c.set_body_color(ME, vec![], 9_999).is_err());
    c.set_body_color(ME, vec![Part::LeftArm, Part::RightArm], 1003)
        .unwrap();
    wait_for(c, "painted", idle).await;
    let reqs = replay.requests().await;
    let colors = reqs
        .iter()
        .rfind(|q| q.url.path() == "/avatar.roblox.com/v2/avatar/set-body-colors")
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&colors.body).unwrap();
    assert_eq!(
        (body["leftArmColor3"].as_str(), body["rightArmColor3"].as_str()),
        (Some("111111"), Some("111111"))
    );
    assert_eq!(body["headColor3"], "F5CD30");
    assert!(c.set_body_color_hex(ME, vec![], "#12345").is_err());
    c.set_body_color_hex(ME, vec![Part::Head], "123456").unwrap();
    wait_for(c, "custom color", idle).await;
    let reqs = replay.requests().await;
    let painted = reqs
        .iter()
        .rfind(|q| q.url.path() == "/avatar.roblox.com/v2/avatar/set-body-colors")
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&painted.body).unwrap();
    assert_eq!(body["headColor3"], "123456");
    assert_eq!(body["torsoColor3"], "0D69AC");

    // Scales snap to the range Roblox allows.
    c.set_scale(ME, Scale::Height, 1.5).unwrap();
    wait_for(c, "scaled", idle).await;
    let reqs = replay.requests().await;
    let scales = reqs
        .iter()
        .rfind(|q| q.url.path() == "/avatar.roblox.com/v1/avatar/set-scales")
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&scales.body).unwrap();
    assert_eq!(body["height"], 1.05);
    assert_eq!(body["width"], 1.0);

    // Going to R6 names what won't show there.
    c.set_rig(ME, Rig::R6);
    wait_for(c, "r6", toast("R6 does not show Denim Jacket.")).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn settings_account_privacy_and_private_servers() {
    let replay = Replay::from_fixture(&replay::load("settings")).await;
    let dir = tempfile::tempdir().unwrap();
    let cookies = seeded(&dir);
    let r = rig(Base::Replay(replay.server.uri()), dir, cookies);
    let c = &r.core;
    let toast = |text: &'static str| move |s: &StoreData| s.toasts.iter().any(|t| t.text == text);

    let info = poll(|| c.account_info(ME)).await;
    assert_eq!(info.email, "t*****@example.com");
    assert!(info.email_verified);
    assert!(info.phone.is_empty() && !info.phone_verified);
    assert_eq!(info.birthdate, Some((2000, 1, 2)));
    assert_eq!(info.premium, Some(false));
    assert_eq!(info.plan, None);
    assert_eq!(info.age_verified, Some(false));
    let v = info.voice.clone().unwrap();
    assert!(v.voice_enabled && v.voice_eligible && v.voice_verified && !v.banned);
    assert!(!v.camera_enabled && v.camera_eligible);

    // Newest first, the current session on top; token and IP never reach the row.
    let sessions = poll(|| c.sessions(ME)).await;
    assert_eq!(sessions.len(), 2);
    assert!(sessions[0].current);
    assert_eq!(sessions[0].device, "Browser on Linux");
    assert_eq!(sessions[0].location, "Springfield, Example, United States");
    assert_eq!(sessions[1].device, "App on Windows");
    assert_eq!(sessions[1].last_active, 1_792_200_000);

    // Only settings with a plain value; unknown ones are kept and tolerated.
    let privacy = poll(|| c.privacy(ME)).await;
    let keys: Vec<&str> = privacy.iter().map(|s| s.key.as_str()).collect();
    assert!(keys.contains(&"whoCanSeeMyInventory") && keys.contains(&"someFutureSetting"));
    assert!(!keys.contains(&"doNotDisturbTimeWindow") && !keys.contains(&"dailyScreenTimeLimit"));
    let trade = privacy.iter().find(|s| s.key == "whoCanTradeWithMe").unwrap();
    assert!(
        !trade
            .options
            .iter()
            .find(|o| o.value == "AllUsers")
            .unwrap()
            .allowed
    );
    assert!(c.set_privacy(ME, "whoCanTradeWithMe", "AllUsers").is_err());
    assert!(c.set_privacy(ME, "whoCanTradeWithMe", "Nope").is_err());
    c.set_privacy(ME, "whoCanSeeMyInventory", "Friends").unwrap();
    let now = c.privacy(ME).data.unwrap();
    let inv = now.iter().find(|s| s.key == "whoCanSeeMyInventory").unwrap();
    assert_eq!(inv.value, "Friends", "optimistic");

    let owned = poll(|| c.my_private_servers(ME, ServersTab::Owned, false)).await;
    assert_eq!(owned.items.len(), 1);
    let s = &owned.items[0];
    assert_eq!(
        (s.id, s.place_id, s.game.as_str()),
        (5_000_001, 920_587_237, "Adopt Me!")
    );
    assert!(s.active && s.renews && s.price.is_none());
    let invited = poll(|| c.my_private_servers(ME, ServersTab::Invited, false)).await;
    assert_eq!(invited.items[0].owner_id, 156);
    assert_eq!(invited.items[0].price, Some(100));

    let cfg = poll(|| c.server_config(ME, 5_000_001)).await;
    assert_eq!(cfg.users, vec![156]);
    assert_eq!(c.store().users.get(&156).unwrap().name, "builderman");
    use bonsai_core::experience::ServerChange;
    c.change_server(ME, 5_000_001, ServerChange::RemoveUser(156));
    assert!(
        c.server_config(ME, 5_000_001).data.unwrap().users.is_empty(),
        "optimistic"
    );
    wait_for(c, "removed", toast("Removed from the server.")).await;
    c.change_server(ME, 5_000_001, ServerChange::AddUser(1));
    wait_for(c, "added", toast("Added to the server.")).await;

    assert!(c.set_display_name(ME, "  ").is_err());
    c.set_display_name(ME, "A display name that is far too long")
        .unwrap();
    wait_for(
        c,
        "name refused",
        toast("Could not change the display name: Display name is too long"),
    )
    .await;
    c.set_display_name(ME, "Good Name").unwrap();
    wait_for(c, "name changed", toast("Display name changed.")).await;
    assert_eq!(c.store().accounts.get(ME).unwrap().display_name, "Good Name");
    wait_for(c, "idle", |s| s.jobs == 0).await;

    let reqs = replay.requests().await;
    let post = reqs
        .iter()
        .find(|q| q.url.path() == "/apis.roblox.com/user-settings-api/v1/user-settings")
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&post.body).unwrap();
    assert_eq!(body, serde_json::json!({ "whoCanSeeMyInventory": "Friends" }));
    let perms: Vec<serde_json::Value> = reqs
        .iter()
        .filter(|q| q.url.path() == "/games.roblox.com/v1/vip-servers/5000001/permissions")
        .map(|q| serde_json::from_slice(&q.body).unwrap())
        .collect();
    assert_eq!(perms[0]["usersToRemove"], serde_json::json!([156]));
    assert_eq!(perms[1]["usersToAdd"], serde_json::json!([1]));
    let patches = reqs
        .iter()
        .filter(|q| q.url.path() == "/users.roblox.com/v1/users/1000001/display-names")
        .count();
    assert_eq!(patches, 1, "a refused name is never sent");
}

#[tokio::test(flavor = "multi_thread")]
async fn thumbnail_seen_before_shows_without_network() {
    let dir = tempfile::tempdir().unwrap();
    let paths = Paths::in_dir(dir.path());
    let key = ThumbKey::small(ThumbKind::GameIcon, 42);
    let url = "https://tr.rbxcdn.com/icon";
    DiskCache::new(&paths.thumbs_dir()).put(url, &png());
    std::fs::create_dir_all(&paths.cache).unwrap();
    Cache::open(&paths.db_file())
        .unwrap()
        .put(Kind::ThumbUrl, AccountId::ANONYMOUS, &key.request_id(), &url);
    // Nothing listens here: any lookup or CDN fetch would fail.
    let r = rig(Base::Replay("http://127.0.0.1:9".into()), dir, Arc::default());
    r.core.want_thumbs(&[key]);
    wait_for(
        &r.core,
        "icon from disk",
        |s| matches!(s.thumbs.get(&key), Some(ThumbState::Ready(i)) if i.mime == "image/png"),
    )
    .await;
}

#[tokio::test(flavor = "multi_thread")]
async fn disk_cache_measures_and_clears_but_keeps_local_data() {
    let dir = tempfile::tempdir().unwrap();
    let paths = Paths::in_dir(dir.path());
    DiskCache::new(&paths.thumbs_dir()).put("https://tr.rbxcdn.com/x", &png());
    let cookies = seeded(&dir);
    {
        let cache = Cache::open(&paths.db_file()).unwrap();
        cache.put(Kind::Local, ME, "drafts", &"keep me");
    }
    let r = rig(Base::Replay("http://127.0.0.1:9".into()), dir, cookies);
    let c = &r.core;
    c.measure_cache();
    wait_for(c, "measured", |s| {
        s.cache_usage.is_some_and(|u| u.images > 0 && u.data > 0)
    })
    .await;
    c.clear_cache();
    wait_for(c, "cleared", |s| s.cache_usage.is_some_and(|u| u.images == 0)).await;
    let cache = Cache::open(&paths.db_file()).unwrap();
    assert!(cache.get::<String>(Kind::Local, ME, "drafts").is_some());
    assert!(cache.get::<i64>(Kind::Robux, ME, "robux").is_none());
}

#[tokio::test(flavor = "multi_thread")]
async fn quick_sign_in_shows_code_then_confirms() {
    let mut f = replay::load("quick_sign_in_approved");
    f.interactions.extend(replay::load("core_signin").interactions);
    let replay = Replay::from_fixture(&f).await;
    let r = rig(
        Base::Replay(replay.server.uri()),
        tempfile::tempdir().unwrap(),
        Arc::default(),
    );

    r.core.start_quick_sign_in();
    wait_for(&r.core, "approved by TestUser", |s| {
        matches!(&s.login, Some(LoginState::QuickCode { code, linked: Some(n), .. }) if code == "ABC123" && n == "TestUser")
            || matches!(s.login, Some(LoginState::Confirm { .. }))
    })
    .await;
    wait_for(&r.core, "confirm step", |s| {
        matches!(s.login, Some(LoginState::Confirm { .. }))
    })
    .await;
    // The cookie from the login response is what got checked, and nothing is saved yet.
    let reqs = replay.requests().await;
    let me = reqs
        .iter()
        .find(|q| q.url.path() == "/users.roblox.com/v1/users/authenticated")
        .unwrap();
    let cookie = me.headers.get("cookie").unwrap().to_str().unwrap();
    assert!(cookie.contains("quick-sign-in-cookie"), "{cookie}");
    assert!(r.cookies.get(ME).unwrap().is_none());
}

#[tokio::test(flavor = "multi_thread")]
async fn cancelling_quick_sign_in_cancels_the_code() {
    let fixture = replay::load("quick_sign_in");
    let replay = Replay::from_fixture(&fixture).await;
    // The captured flow eventually returns Cancelled on its own. Keep it pending here:
    // this test must be the thing that cancels it, regardless of how many polls run.
    let pending = fixture
        .interactions
        .iter()
        .find(|i| i.request.path.ends_with("/login/status") && i.response.body["status"] == "Created")
        .unwrap();
    replay.repeat(pending).await;
    let r = rig(
        Base::Replay(replay.server.uri()),
        tempfile::tempdir().unwrap(),
        Arc::default(),
    );

    r.core.start_quick_sign_in();
    wait_for(&r.core, "code", |s| {
        matches!(s.login, Some(LoginState::QuickCode { .. }))
    })
    .await;
    // Let multiple status polls finish before cancelling. The code must remain visible.
    let start = Instant::now();
    while replay
        .requests()
        .await
        .iter()
        .filter(|q| q.url.path().ends_with("/login/status"))
        .count()
        < 3
    {
        assert!(start.elapsed() < Duration::from_secs(10), "no status polls");
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    assert!(matches!(r.core.store().login, Some(LoginState::QuickCode { .. })));
    r.core.cancel_login();
    assert!(r.core.store().login.is_none());
    let start = Instant::now();
    loop {
        let reqs = replay.requests().await;
        if reqs.iter().any(|q| q.url.path().ends_with("/login/cancel")) {
            break;
        }
        assert!(start.elapsed() < Duration::from_secs(10), "no cancel request");
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    // Polling stopped: the state stays cleared.
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert!(r.core.store().login.is_none());
}

#[tokio::test(flavor = "multi_thread")]
async fn sign_in_window_session_goes_to_confirm() {
    let replay = Replay::start(&["core_signin"]).await;
    let r = rig(
        Base::Replay(replay.server.uri()),
        tempfile::tempdir().unwrap(),
        Arc::default(),
    );

    // Nothing is waiting on the window: a session is ignored.
    assert!(!r.core.submit_session(Secret::new(COOKIE)));
    r.core.browser_sign_in_started();
    assert_eq!(r.core.store().login, Some(LoginState::Browser));
    // Closing the window without signing in ends the flow.
    r.core.browser_sign_in_closed();
    assert!(r.core.store().login.is_none());

    r.core.browser_sign_in_started();
    assert!(r.core.submit_session(Secret::new(COOKIE)));
    // The window closing after handing over the session doesn't cancel the check.
    r.core.browser_sign_in_closed();
    wait_for(&r.core, "confirm step", |s| {
        matches!(s.login, Some(LoginState::Confirm { .. }))
    })
    .await;
    r.core.send(Cmd::ConfirmLogin);
    wait_for(&r.core, "saved", |s| s.active_id() == Some(ME)).await;
    assert_eq!(r.cookies.get(ME).unwrap().unwrap().expose(), COOKIE);
}

#[tokio::test(flavor = "multi_thread")]
async fn rate_limit_emits_countdown_toast() {
    let mut headers = std::collections::BTreeMap::new();
    headers.insert("retry-after".into(), "1".into());
    let limited = Interaction {
        request: replay::RecordedRequest {
            method: "GET".into(),
            host: "users.roblox.com".into(),
            path: "/v1/users/1".into(),
            ..Default::default()
        },
        response: replay::RecordedResponse {
            status: 429,
            headers,
            body: serde_json::json!({"errors":[{"code":0,"message":"TooManyRequests"}]}),
        },
    };
    let ok = Interaction {
        request: replay::RecordedRequest {
            method: "GET".into(),
            host: "users.roblox.com".into(),
            path: "/v1/users/1".into(),
            ..Default::default()
        },
        response: replay::RecordedResponse {
            status: 200,
            body: serde_json::json!({"id":1,"name":"Roblox","displayName":"Roblox"}),
            ..Default::default()
        },
    };
    let replay = Replay::from_fixture(&Fixture {
        interactions: vec![limited, ok],
        ..Default::default()
    })
    .await;
    let r = rig(
        Base::Replay(replay.server.uri()),
        tempfile::tempdir().unwrap(),
        Arc::default(),
    );

    let _ = r.core.profile(1);
    wait_for(&r.core, "rate limit toast", |s| {
        s.toasts
            .iter()
            .any(|t| t.host.as_deref() == Some("users.roblox.com") && t.retry_after_secs.is_some())
    })
    .await;
    let t = r
        .core
        .store()
        .toasts
        .iter()
        .find(|t| t.host.as_deref() == Some("users.roblox.com"))
        .cloned()
        .unwrap();
    assert_eq!(t.text, "Rate limited (users.roblox.com)");
    assert!(!t.error);
    assert!(t.retry_after_secs.unwrap() >= 1);
}

#[tokio::test(flavor = "multi_thread")]
async fn transactions_ledger_syncs_backfills_and_sums() {
    use bonsai_core::transactions::{Filter, Period, Scope, TxKind, sums};
    let replay = Replay::from_fixture(&replay::load("transactions_ledger")).await;
    let dir = tempfile::tempdir().unwrap();
    let cookies = seeded(&dir);
    let r = rig(Base::Replay(replay.server.uri()), dir, cookies);
    let c = &r.core;
    let settle = |want: &dyn Fn(&bonsai_core::transactions::Ledger) -> bool| {
        let start = Instant::now();
        loop {
            let l = c.ledger(ME, Scope::Me, false);
            assert!(l.error.is_none(), "error: {:?}", l.error);
            if !l.syncing && !l.loading_older && want(&l) {
                return l;
            }
            assert!(start.elapsed() < Duration::from_secs(10), "timed out");
            std::thread::sleep(Duration::from_millis(10));
        }
    };

    // First sync: one page per type; sales have more.
    let l = settle(&|l| l.synced_at > 0);
    let ids: Vec<&str> = l.rows.iter().map(|t| t.id.as_str()).collect();
    assert_eq!(ids, ["p1", "s-pend", "s-old", "st1"], "newest first across types");
    assert!(l.has_older() && !l.complete());
    let p = &l.rows[0];
    assert_eq!(p.kind, TxKind::Purchase);
    assert_eq!(p.experience().map(|e| e.universe_id), Some(10_767_201_394));
    assert!(p.agent.as_ref().is_some_and(|a| a.group));
    let s = sums(l.rows.iter());
    assert_eq!((s.incoming, s.outgoing, s.pending), (485, -15, 70));

    // Older history until every type is complete.
    c.ledger_load_older(ME, Scope::Me).unwrap();
    let l = settle(&|l| l.complete());
    assert_eq!(l.rows.last().map(|t| t.id.as_str()), Some("s-oldest"));
    assert_eq!(
        c.ledger_load_older(ME, Scope::Me),
        Err("All history is already here.".into())
    );

    // A resync stops at known rows, adds the new sale and releases the pending one.
    let before = l.synced_at;
    drop(c.ledger(ME, Scope::Me, true));
    let l = settle(&|l| l.synced_at >= before && l.rows.len() == 6);
    assert!(l.rows.iter().any(|t| t.id == "s-new" && t.pending));
    assert!(l.rows.iter().any(|t| t.id == "s-pend" && !t.pending));
    let sales = Filter {
        kinds: vec![TxKind::Sale],
        ..Default::default()
    };
    let s = sums(l.rows.iter().filter(|t| sales.matches(t)));
    assert_eq!((s.count, s.incoming, s.pending), (4, 155, 35));
    let jul = bonsai_core::fmt::parse_iso("2026-08-01T00:00:00Z").unwrap();
    let sep = bonsai_core::fmt::parse_iso("2026-09-01T00:00:00Z").unwrap();
    let august = Filter {
        from: Some(jul),
        to: Some(sep),
        ..Default::default()
    };
    assert_eq!(l.rows.iter().filter(|t| august.matches(t)).count(), 1);

    // Roblox's own totals.
    let t = poll(|| c.tx_totals(ME, Scope::Me, Period::Month)).await;
    assert_eq!((t.incoming, t.outgoing, t.pending), (485, -15, 105));

    // A community: its summary, and its ledger with a refused type skipped.
    let g = Scope::Group(4_000_001);
    let t = poll(|| c.tx_totals(ME, g, Period::Month)).await;
    assert_eq!((t.incoming, t.outgoing, t.pending), (700, -100, 300));
    let start = Instant::now();
    let l = loop {
        let l = c.ledger(ME, g, false);
        if !l.syncing && l.synced_at > 0 {
            break l;
        }
        assert!(start.elapsed() < Duration::from_secs(10), "timed out");
        tokio::time::sleep(Duration::from_millis(10)).await;
    };
    assert!(l.error.is_none(), "error: {:?}", l.error);
    assert_eq!(l.rows.len(), 2);

    let csv = bonsai_core::transactions::csv(l.rows.iter());
    assert_eq!(csv.lines().count(), 3);
}

#[tokio::test(flavor = "multi_thread")]
async fn themes_apply_cycle_delete_and_persist() {
    use bonsai_core::settings::Mode;
    use bonsai_core::themes::{self, Theme};
    let dir = tempfile::tempdir().unwrap();
    let paths = Paths::in_dir(dir.path());
    let r = rig(Base::Replay("http://127.0.0.1:9".into()), dir, Arc::default());
    let c = &r.core;
    assert_eq!(c.appearance_theme(themes::Base::Dark).id, "dark");

    let mut mine = Theme::parse(r#"{"schema":1,"id":"mine","base":"light","name":"Mine"}"#).unwrap();
    c.save_theme(mine.clone()).unwrap();
    mine.name = "Mine 2".into();
    c.save_theme(mine.clone()).unwrap();
    assert!(
        c.save_theme(Theme {
            id: "dark".into(),
            ..mine.clone()
        })
        .is_err()
    );

    // Applying sets the slot for the theme's base and switches to that base.
    c.apply_theme("mine").unwrap();
    let a = c.settings().appearance;
    assert_eq!(
        (a.mode, a.light.as_str(), a.dark.as_str()),
        (Mode::Light, "mine", "dark")
    );
    assert!(c.apply_theme("nope").is_err());

    // Cycling walks built-ins, then user themes, and wraps.
    c.cycle_theme();
    assert_eq!(c.settings().appearance.dark, "dark");
    assert_eq!(c.settings().appearance.mode, Mode::Dark);
    c.cycle_theme();
    assert_eq!(c.settings().appearance.light, "light");

    // Written by the worker; a fresh load sees it.
    let file = paths.themes_dir().join("mine.json");
    let start = Instant::now();
    while themes::Themes::load(&paths.themes_dir())
        .get("mine")
        .map(|t| t.name.as_str())
        != Some("Mine 2")
    {
        assert!(start.elapsed() < Duration::from_secs(10), "theme never written");
        tokio::time::sleep(Duration::from_millis(20)).await;
    }

    // Deleting puts a slot that used it back on the built-in.
    c.apply_theme("mine").unwrap();
    assert!(c.delete_theme("light").is_err());
    c.delete_theme("mine").unwrap();
    assert_eq!(c.settings().appearance.light, "light");
    assert!(c.themes().iter().all(|(t, _)| t.id != "mine"));
    let start = Instant::now();
    while file.exists() {
        assert!(start.elapsed() < Duration::from_secs(10), "theme never deleted");
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn theme_import_export_and_clashes() {
    use bonsai_core::themes::{OnClash, Theme};
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("out.bonsaitheme");
    let r = rig(Base::Replay("http://127.0.0.1:9".into()), dir, Arc::default());
    let c = &r.core;

    // A code of a built-in imports as a copy, since built-ins can't be replaced.
    let ember = c.theme_or_shown("ember").unwrap();
    c.stage_theme_import(&ember.share_code()).unwrap();
    let p = c.store().theme_import.clone().unwrap();
    assert_eq!((p.source.as_str(), p.clash), ("", Some(("Ember".into(), true))));
    let id = c.finish_theme_import(OnClash::Replace).unwrap();
    assert_eq!(id, "ember-copy");
    assert!(c.store().theme_import.is_none());
    assert!(c.finish_theme_import(OnClash::Replace).is_err());

    // Exported to a file and read back: same id, so it can replace or sit beside it.
    c.export_theme("ember-copy", &out).unwrap();
    assert_eq!(
        Theme::decode(&std::fs::read_to_string(&out).unwrap()).unwrap().id,
        "ember-copy"
    );
    c.stage_theme_import(&out.to_string_lossy()).unwrap();
    let p = c.store().theme_import.clone().unwrap();
    assert_eq!(p.source, "out.bonsaitheme");
    assert_eq!(p.clash, Some(("Ember".into(), false)));
    assert_eq!(c.finish_theme_import(OnClash::Replace).unwrap(), "ember-copy");
    c.stage_theme_import(&out.to_string_lossy()).unwrap();
    assert_eq!(c.finish_theme_import(OnClash::KeepBoth).unwrap(), "ember-copy-2");

    // Cancelling installs nothing; bad input never opens the dialog.
    c.stage_theme_import(r#"{"schema":1,"id":"fresh","base":"dark"}"#)
        .unwrap();
    assert_eq!(c.store().theme_import.as_ref().unwrap().clash, None);
    c.cancel_theme_import();
    assert!(c.themes().iter().all(|(t, _)| t.id != "fresh"));
    assert!(c.stage_theme_import("/no/such/file.bonsaitheme").is_err());
    assert!(c.stage_theme_import("bonsai-theme:1:zz").is_err());
    assert!(c.store().theme_import.is_none());

    // With no id, the theme on screen.
    assert_eq!(c.theme_or_shown("").unwrap().base, c.shown_base());
}

#[tokio::test(flavor = "multi_thread")]
async fn google_font_copies_a_builtin_and_downloads_its_files() {
    use bonsai_core::settings::Mode;
    use bonsai_core::themes::{self, FontSlot};
    let replay = Replay::from_fixture(&replay::load("fonts")).await;
    let dir = tempfile::tempdir().unwrap();
    let paths = Paths::in_dir(dir.path());
    let r = rig(Base::Replay(replay.server.uri()), dir, Arc::default());
    let c = &r.core;
    c.update_settings(|s| s.appearance.mode = Mode::Dark);

    // Bundled fonts need no catalog; unknown names do.
    assert!(c.set_theme_font(FontSlot::Sans, "Nope Sans").is_err());
    let catalog = poll(|| c.font_catalog()).await;
    assert_eq!(catalog[0].family, "Inter");
    assert!(
        catalog.iter().all(|f| f.family != "Molle"),
        "italic-only families are dropped"
    );
    assert!(c.set_theme_font(FontSlot::Sans, "Nope Sans").is_err());

    // A built-in theme is copied, and the copy is used.
    c.set_theme_font(FontSlot::Sans, "space grotesk").unwrap();
    let shown = c.appearance_theme(themes::Base::Dark);
    assert_eq!(
        (shown.id.as_str(), shown.name.as_str()),
        ("dark-copy", "Dark copy")
    );
    assert_eq!(shown.google_font(FontSlot::Sans), Some("Space Grotesk"));
    assert_eq!(c.appearance_fonts(), vec!["Space Grotesk".to_owned()]);
    assert!(
        c.themes()
            .iter()
            .any(|(t, b)| t.id == "dark" && *b && t.google_font(FontSlot::Sans).is_none())
    );

    // The files land on disk, one per weight.
    let files = poll(|| c.google_font("Space Grotesk")).await;
    let weights: Vec<u16> = files.files.iter().map(|f| f.weight).collect();
    assert_eq!(weights, vec![400, 500, 600, 700]);
    let bytes = std::fs::read(paths.fonts_dir().join("space-grotesk/700.ttf")).unwrap();
    assert!(bytes.ends_with(b"space-grotesk-700"));

    // Switching back to a bundled font edits the copy in place.
    c.set_theme_font(FontSlot::Sans, "Inter").unwrap();
    assert_eq!(c.appearance_theme(themes::Base::Dark).id, "dark-copy");
    assert!(c.appearance_fonts().is_empty());

    // In System mode the theme the OS shows is changed, and the mode stays.
    c.update_settings(|s| s.appearance.mode = Mode::System);
    c.set_system_dark(false);
    c.set_theme_font(FontSlot::Mono, "System").unwrap();
    assert_eq!(c.settings().appearance.mode, Mode::System);
    assert_eq!(c.appearance_theme(themes::Base::Light).id, "light-copy");
    assert_eq!(c.appearance_theme(themes::Base::Dark).id, "dark-copy");

    let reqs = replay.requests().await;
    assert!(reqs.iter().all(|q| !q.headers.contains_key("cookie")));
}

#[tokio::test(flavor = "multi_thread")]
async fn approving_a_code_from_another_device() {
    use bonsai_core::signin::ApproveStep;
    let replay = Replay::start(&["quick_sign_in_approve"]).await;
    let dir = tempfile::tempdir().unwrap();
    let cookies = seeded(&dir);
    let r = rig(Base::Replay(replay.server.uri()), dir, cookies);
    let c = &r.core;
    let step = |s: &StoreData| {
        s.approve
            .as_ref()
            .filter(|p| !p.busy)
            .map(|p| (p.step.clone(), p.error.clone()))
    };

    assert_eq!(
        c.approve_enter(ME, "abc"),
        Err("Enter the 6-character code.".into())
    );
    // Entered, then confirmed: the code is approved.
    let seq = c.approve_enter(ME, "abc 123").unwrap();
    assert_eq!(c.store().approve.as_ref().unwrap().code, "ABC123");
    wait_for(c, "device shown", |s| {
        matches!(step(s), Some((ApproveStep::Confirm { device, location }, None))
            if device == "Chrome on Windows" && location == "Springfield, United States")
    })
    .await;
    c.approve_confirm(seq).unwrap();
    wait_for(c, "approved", |s| {
        matches!(step(s), Some((ApproveStep::Done, None)))
    })
    .await;
    c.approve_close(seq);
    assert!(c.store().approve.is_none());

    // Entered, then closed: the code is cancelled so the other device can't use it.
    let seq = c.approve_enter(ME, "DEF456").unwrap();
    wait_for(c, "device shown", |s| {
        matches!(step(s), Some((ApproveStep::Confirm { .. }, _)))
    })
    .await;
    c.approve_close(seq);
    let start = Instant::now();
    while !replay
        .requests()
        .await
        .iter()
        .any(|q| q.url.path().ends_with("/login/cancel"))
    {
        assert!(start.elapsed() < Duration::from_secs(10), "no cancel request");
        tokio::time::sleep(Duration::from_millis(20)).await;
    }

    // A wrong code stays on the entry step with a message.
    c.approve_enter(ME, "ZZZZZZ").unwrap();
    wait_for(
        c,
        "error",
        |s| matches!(step(s), Some((ApproveStep::Enter, Some(e))) if e.starts_with("That code didn't work")),
    )
    .await;
    assert_ne!(c.store().account_data(ME).unwrap().session, SessionState::Expired);
}

#[tokio::test]
async fn premium_plan_and_renewal() {
    let replay = Replay::from_fixture(&replay::load("account_extras")).await;
    let dir = tempfile::tempdir().unwrap();
    let cookies = seeded(&dir);
    let r = rig(Base::Replay(replay.server.uri()), dir, cookies);
    let info = poll(|| r.core.account_info(ME)).await;
    assert_eq!(info.premium, Some(true));
    let plan = info.plan.clone().unwrap();
    assert_eq!(plan.name, "Premium 1000");
    assert_eq!(plan.renews.as_deref(), Some("2026-11-01T00:00:00Z"));
    assert_eq!(plan.expires.as_deref(), Some("2026-12-01T00:00:00Z"));
}

#[tokio::test(flavor = "multi_thread")]
async fn security_reads_methods_and_walks_the_two_step_dialogs() {
    use bonsai_core::security::{SecurityStep, SecurityTask};
    let replay = Replay::from_fixture(&replay::load("security")).await;
    let dir = tempfile::tempdir().unwrap();
    let cookies = seeded(&dir);
    let r = rig(Base::Replay(replay.server.uri()), dir, cookies);
    let info = poll(|| r.core.security(ME)).await;
    assert!(info.email && !info.authenticator && !info.sms);
    assert_eq!(info.recovery_codes, Some(0));
    assert_eq!(info.passkeys, vec!["Laptop".to_owned()]);

    // Authenticator: password, then a wrong code, then the right one.
    let seq = r.core.security_open(ME, SecurityTask::Authenticator).unwrap();
    assert!(
        r.core.security_code(seq, "123456").is_err(),
        "code before the key"
    );
    r.core.security_password(seq, "hunter22").unwrap();
    let step = |s: &StoreData| {
        s.security
            .as_ref()
            .map(|f| (f.step.clone(), f.busy, f.error.clone()))
    };
    wait_for(&r.core, "key", |s| {
        matches!(step(s), Some((SecurityStep::Key { .. }, false, _)))
    })
    .await;
    assert_eq!(
        step(&r.core.store()).unwrap().0,
        SecurityStep::Key {
            manual_key: "ABCDEFGHIJKLMNOP".into()
        }
    );
    assert!(r.core.security_code(seq, "12345").is_err());
    r.core.security_code(seq, "000 000").unwrap();
    wait_for(&r.core, "wrong code", |s| {
        matches!(step(s), Some((_, false, Some(_))))
    })
    .await;
    r.core.security_code(seq, "123456").unwrap();
    wait_for(&r.core, "done", |s| {
        matches!(step(s), Some((SecurityStep::Done { .. }, ..)))
    })
    .await;
    assert_eq!(
        step(&r.core.store()).unwrap().0,
        SecurityStep::Done {
            codes: vec!["1111 2222".into(), "3333 4444".into()]
        }
    );
    r.core.security_close(seq);
    assert!(r.core.store().security.is_none());

    // New recovery codes; a stale dialog can't answer.
    let seq2 = r.core.security_open(ME, SecurityTask::RecoveryCodes).unwrap();
    assert!(r.core.security_password(seq, "hunter22").is_err());
    r.core.security_password(seq2, "hunter22").unwrap();
    wait_for(
        &r.core,
        "codes",
        |s| matches!(step(s), Some((SecurityStep::Done { codes, .. }, ..)) if codes.len() == 2),
    )
    .await;
    r.core.security_close(seq2);

    // Quick changes report with toasts.
    assert!(r.core.change_password(ME, "hunter22", "short").is_err());
    assert!(r.core.change_password(ME, "hunter22", "hunter22").is_err());
    r.core.change_password(ME, "hunter22", "correct horse").unwrap();
    let toast = |text: &'static str| move |s: &StoreData| s.toasts.iter().any(|t| t.text == text);
    wait_for(
        &r.core,
        "password",
        toast("Password changed. Other sessions were signed out."),
    )
    .await;
    r.core.set_email_codes(ME, false, "correct horse").unwrap();
    wait_for(&r.core, "email off", toast("Email codes are off.")).await;
    assert!(r.core.remove_passkey(ME, "  ").is_err());
    r.core.remove_passkey(ME, "Laptop").unwrap();
    wait_for(&r.core, "passkey", toast("Passkey removed.")).await;
    assert!(r.core.store().toasts.iter().all(|t| !t.error));
}

#[tokio::test(flavor = "multi_thread")]
async fn quick_settings_changes_keep_the_newest() {
    let dir = tempfile::tempdir().unwrap();
    let file = Paths::in_dir(dir.path()).settings_file();
    let r = rig(Base::Replay("http://127.0.0.1:9".into()), dir, Arc::default());
    // Saves run side by side; an older one finishing last must not win.
    for days in 1..=50 {
        r.core.update_settings(|s| s.cache_days = days);
    }
    assert_eq!(r.core.settings().cache_days, 50);
    let start = Instant::now();
    while bonsai_core::settings::Settings::load(&file).cache_days != 50 {
        assert!(start.elapsed() < Duration::from_secs(10), "not saved");
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    tokio::time::sleep(Duration::from_millis(200)).await;
    assert_eq!(r.core.settings().cache_days, 50);
    assert_eq!(bonsai_core::settings::Settings::load(&file).cache_days, 50);
}

#[tokio::test(flavor = "multi_thread")]
async fn backgrounds_belong_to_the_theme_and_share_without_files() {
    use bonsai_core::themes::{self, Theme};
    let r = rig(
        Base::Replay("http://127.0.0.1:9".into()),
        tempfile::tempdir().unwrap(),
        Arc::default(),
    );
    let c = &r.core;
    assert!(c.set_background("/tmp/image.png").is_err());
    c.set_background("https://example.com/wallpaper.png?size=large")
        .unwrap();
    c.set_background_level("blur 12").unwrap();
    let theme = c.appearance_theme(c.shown_base());
    assert_ne!(theme.id, "dark");
    assert_eq!(theme.background.as_ref().unwrap().blur, 12);
    assert_eq!(Theme::decode(&theme.share_code()).unwrap(), theme);
    assert!(!theme.to_json().contains("\"file\""));
    c.apply_theme("light").unwrap();
    assert!(c.appearance_theme(themes::Base::Light).background.is_none());
    c.apply_theme(&theme.id).unwrap();
    assert_eq!(c.appearance_theme(c.shown_base()).background, theme.background);
    c.clear_background().unwrap();
    assert!(c.appearance_theme(c.shown_base()).background.is_none());
}

#[tokio::test(flavor = "multi_thread")]
async fn theme_background_download_is_cached_for_offline_reuse() {
    let replay = Replay::start(&["background_image"]).await;
    let r = rig(
        Base::Replay(replay.server.uri()),
        tempfile::tempdir().unwrap(),
        Arc::default(),
    );
    let url = "https://images.example.com/cat.gif";
    // Only the background of a theme in use is served.
    assert!(r.core.background_image(url).await.is_err());
    r.core.set_background(url).unwrap();
    let (bytes, mime) = r.core.background_image(url).await.unwrap();
    assert_eq!(bytes, b"GIF89a-background");
    assert_eq!(mime, "image/gif");
    drop(replay);
    let (cached, cached_mime) = r.core.background_image(url).await.unwrap();
    assert_eq!(cached, bytes);
    assert_eq!(cached_mime, mime);
    assert!(r.core.background_image("file:///tmp/a.png").await.is_err());
}

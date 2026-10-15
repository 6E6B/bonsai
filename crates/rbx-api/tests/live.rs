//! Live tests against the real API with a throwaway account.
//!
//! `RBX_TEST_COOKIE=... cargo test -p rbx-api --features live -- --test-threads=1`
//!
//! With `RBX_RECORD=1` they also (re)write the fixtures under `tests/fixtures/`, scrubbed.
//! Never run in CI.
#![cfg(feature = "live")]

use std::sync::Arc;

use rbx_api::generated::*;
use rbx_api::replay::{Recorder, Scrubber};
use rbx_api::{AccountId, Client, Secret};

struct Live {
    client: Client,
    rec: Arc<Recorder>,
    scrub: Scrubber,
}

async fn live() -> Live {
    let cookie = std::env::var("RBX_TEST_COOKIE").expect("RBX_TEST_COOKIE");
    let rec = Arc::new(Recorder::default());
    let anon = Client::builder(AccountId::ANONYMOUS).build().unwrap();
    let me = users::v1::get_users_authenticated(
        &Client::builder(AccountId(1))
            .cookie(Secret::new(&cookie))
            .build()
            .unwrap(),
    )
    .await
    .expect("cookie must be valid");
    drop(anon);
    let id = me.id.unwrap_or_default();
    let client = rec
        .attach(Client::builder(AccountId(id as u64)))
        .cookie(Secret::new(cookie))
        .browser_tracker(123_456_789_012)
        .build()
        .unwrap();
    let scrub = Scrubber::new()
        .keep_ids(&[1, 156, 1818])
        .map_id(id, 1_000_001)
        .replace(me.name.clone().unwrap_or_default(), "TestUser")
        .replace(me.display_name.clone().unwrap_or_default(), "Test User")
        .replace("Roblox", "Roblox");
    Live { client, rec, scrub }
}

fn record() -> bool {
    std::env::var("RBX_RECORD").is_ok_and(|v| v == "1")
}

impl Live {
    /// An anonymous client whose traffic goes into the same fixture.
    fn anon(&self) -> Client {
        self.rec
            .attach(Client::builder(AccountId::ANONYMOUS))
            .build()
            .unwrap()
    }

    fn save(&self, name: &str, note: &str) {
        if record() {
            self.rec.save(name, note, &self.scrub);
        }
    }
}

#[tokio::test]
async fn csrf_retry_on_auth_ticket() {
    let l = live().await;
    let resp = l
        .client
        .send(
            auth::undocumented::post_authentication_ticket_request(
                &auth::undocumented::PostAuthenticationTicketParams {
                    referer: "https://www.roblox.com/".into(),
                    rbx_authentication_negotiation: Some("1".into()),
                },
            )
            .unwrap(),
        )
        .await
        .unwrap();
    assert!(
        resp.header("rbx-authentication-ticket")
            .is_some_and(|t| !t.is_empty())
    );
    l.save(
        "auth_ticket",
        "POST without a token gets 403 + x-csrf-token, the retry gets the ticket header.",
    );
}

#[tokio::test]
async fn servers_cursor_pages() {
    let l = live().await;
    let p = games::v1::GetGamesByPlaceIdServersByServerTypeParams {
        limit: Some(10),
        ..Default::default()
    };
    let first = games::v1::get_games_by_place_id_servers_by_server_type(&l.client, 920587237, 0, &p)
        .await
        .unwrap();
    let cursor = first
        .next_page_cursor
        .clone()
        .expect("a big game has a second page");
    let p2 = games::v1::GetGamesByPlaceIdServersByServerTypeParams {
        cursor: Some(cursor),
        ..p
    };
    let second = games::v1::get_games_by_place_id_servers_by_server_type(&l.client, 920587237, 0, &p2)
        .await
        .unwrap();
    assert!(!second.data.unwrap_or_default().is_empty());
    l.save(
        "servers_two_pages",
        "Public servers of Adopt Me (920587237), two cursor pages of 10.",
    );
}

#[tokio::test]
async fn messages_page_numbers() {
    let l = live().await;
    for page in 0..2 {
        let p = privatemessages::v1::GetMessagesParams {
            page_number: Some(page),
            page_size: Some(2),
            ..Default::default()
        };
        privatemessages::v1::get_messages(&l.client, &p).await.unwrap();
    }
    l.save(
        "messages_pages",
        "Private messages inbox, page numbers 0 and 1 of size 2.",
    );
}

#[tokio::test]
async fn authenticated_user() {
    let l = live().await;
    let me = users::v1::get_users_authenticated(&l.client).await.unwrap();
    assert!(me.id.is_some());
    l.save(
        "users_authenticated",
        "Cookie validation call used by cookie-paste sign-in.",
    );
}

/// The calls rbx-core makes when a cookie is pasted and confirmed, in order.
#[tokio::test]
async fn core_signin_calls() {
    let l = live().await;
    let anon = l.anon();
    let c = &l.client;
    let me = users::v1::get_users_authenticated(c).await.unwrap();
    users::v1::get_users_authenticated(c).await.unwrap();
    economy::v1::get_user_currency(c).await.unwrap();
    privatemessages::v1::get_messages_unread_count(c).await.unwrap();
    friends::v1::get_user_friend_requests_count(c).await.unwrap();
    trades::v1::get_trades_by_trade_status_type_count(c, 1)
        .await
        .unwrap();
    notifications::v2::get_stream_notifications_unread_count(c)
        .await
        .unwrap();
    friends::v1::get_users_by_user_id_friends(c, me.id.unwrap(), &Default::default())
        .await
        .unwrap();
    let id = me.id.unwrap();
    let r = thumbnails::undocumented::post_batch(
        &anon,
        &vec![thumbnails::undocumented::BatchRequest {
            request_id: Some(format!("AvatarHeadShot:{id}:150x150")),
            target_id: Some(id),
            type_: Some("AvatarHeadShot".into()),
            size: Some("150x150".into()),
            format: Some("webp".into()),
            is_circular: Some(false),
        }],
    )
    .await
    .unwrap();
    assert_eq!(r.data.unwrap_or_default().len(), 1);
    l.save(
        "core_signin",
        "Cookie validation, then the account refresh: me, Robux, unread counts, friends, headshot.",
    );
}

/// The calls rbx-core makes to open an experience page and its server list.
#[tokio::test]
async fn core_place_calls() {
    let l = live().await;
    let anon = l.anon();
    let d = games::v1::get_games_multiget_place_details(
        &l.client,
        &games::v1::GetGamesMultigetPlaceDetailsParams {
            place_ids: vec![1818],
        },
    )
    .await
    .unwrap();
    let u = d[0].universe_id.unwrap();
    games::v1::get_games(
        &anon,
        &games::v1::GetGamesParams {
            universe_ids: vec![u],
        },
    )
    .await
    .unwrap();
    games::v1::get_games_votes(
        &anon,
        &games::v1::GetGamesVotesParams {
            universe_ids: vec![u],
        },
    )
    .await
    .unwrap();
    let p = games::v1::GetGamesByPlaceIdServersByServerTypeParams {
        sort_order: Some(2),
        exclude_full_games: Some(false),
        limit: Some(100),
        cursor: None,
    };
    games::v1::get_games_by_place_id_servers_by_server_type(&l.client, 1818, 0, &p)
        .await
        .unwrap();
    l.save(
        "core_place",
        "Crossroads (place 1818): place to universe, details, votes, public servers.",
    );
}

#[tokio::test]
async fn place_universe() {
    let l = live().await;
    let r = apis::undocumented::get_place_universe(&l.anon(), 1818)
        .await
        .unwrap();
    assert_eq!(r.universe_id, Some(13058));
    l.save("place_universe", "Place 1818 to its universe, no sign-in.");
}

const SESSION: &str = "00000000-0000-4000-8000-000000000000";

#[tokio::test]
async fn explore_sorts() {
    let l = live().await;
    let p = apis::undocumented::GetExploreSortsParams {
        session_id: SESSION.into(),
        device: Some("computer".into()),
        country: Some("all".into()),
        ..Default::default()
    };
    let r = apis::undocumented::get_explore_sorts(&l.client, &p)
        .await
        .unwrap();
    let sorts = r.sorts.unwrap_or_default();
    assert!(
        sorts
            .iter()
            .any(|s| !s.games.clone().unwrap_or_default().is_empty())
    );
    l.save("explore_sorts", "Home game sorts, signed in.");
}

#[tokio::test]
async fn omni_search() {
    let l = live().await;
    let p = apis::undocumented::GetOmniSearchParams {
        search_query: "adopt me".into(),
        session_id: SESSION.into(),
        page_type: Some("games".into()),
        ..Default::default()
    };
    let r = apis::undocumented::get_omni_search(&l.client, &p).await.unwrap();
    let games: Vec<_> = r
        .search_results
        .unwrap_or_default()
        .into_iter()
        .flat_map(|g| g.contents.unwrap_or_default())
        .collect();
    assert!(games.iter().any(|g| g.root_place_id == Some(920587237)));
    l.save("omni_search", "Game search for \"adopt me\".");
}

/// Profile reads for builderman (156) and Roblox (1), plus the test account's requests,
/// relation to Roblox and block list.
#[tokio::test]
async fn people() {
    let l = live().await;
    let c = &l.client;
    let me = users::v1::get_users_authenticated(c).await.unwrap().id.unwrap();
    l.rec.clear();
    users::v1::get_users_by_user_id(c, 156).await.unwrap();
    friends::v1::get_users_by_user_id_friends_count(c, 156)
        .await
        .unwrap();
    friends::v1::get_users_by_target_user_id_followers_count(c, 156)
        .await
        .unwrap();
    friends::v1::get_users_by_target_user_id_followings_count(c, 156)
        .await
        .unwrap();
    let st = friends::v1::GetUsersByUserIdFriendsStatusesParams { user_ids: vec![1] };
    friends::v1::get_users_by_user_id_friends_statuses(c, me, &st)
        .await
        .unwrap();
    let body = friends::v1::FollowingExistsRequestModel {
        target_user_ids: Some(vec![1]),
    };
    friends::v1::post_user_following_exists(c, &body).await.unwrap();
    assert!(!apis::undocumented::get_is_blocked(c, 1).await.unwrap());
    let p = friends::v1::GetMyFriendsRequestsParams {
        limit: Some(50),
        ..Default::default()
    };
    let reqs = friends::v1::get_my_friends_requests(c, &p).await.unwrap();
    for (i, r) in reqs.data.unwrap_or_default().iter().enumerate() {
        if let Some(id) = r.id {
            l.scrub.alias(id, 2_000_001 + i as i64);
        }
    }
    let p = friends::v1::GetUsersByUserIdFriendsFindParams {
        limit: Some(50),
        ..Default::default()
    };
    friends::v1::get_users_by_user_id_friends_find(c, 156, &p)
        .await
        .unwrap();
    let p = friends::v1::GetUsersByTargetUserIdFollowersParams {
        limit: Some(100),
        ..Default::default()
    };
    friends::v1::get_users_by_target_user_id_followers(c, 156, &p)
        .await
        .unwrap();
    let p = friends::v1::GetUsersByTargetUserIdFollowingsParams {
        limit: Some(100),
        ..Default::default()
    };
    friends::v1::get_users_by_target_user_id_followings(c, 156, &p)
        .await
        .unwrap();
    let p = users::v1::GetUsersByUserIdUsernameHistoryParams {
        limit: Some(50),
        ..Default::default()
    };
    users::v1::get_users_by_user_id_username_history(c, 156, &p)
        .await
        .unwrap();
    let p = groups::v1::GetUsersByUserIdGroupsRolesParams::default();
    groups::v1::get_users_by_user_id_groups_roles(c, 1, &p)
        .await
        .unwrap();
    let p = badges::v1::GetUsersByUserIdBadgesParams {
        limit: Some(100),
        cursor: None,
        sort_order: Some(badges::v1::GetUniversesByUniverseIdBadgesSortOrder::Desc),
    };
    badges::v1::get_users_by_user_id_badges(c, 156, &p).await.unwrap();
    let p = games::v2::GetUsersByUserIdGamesParams {
        limit: Some(50),
        ..Default::default()
    };
    games::v2::get_users_by_user_id_games(c, 1, &p).await.unwrap();
    let p = apis::undocumented::GetBlockedUsersParams {
        cursor: None,
        count: 49,
    };
    apis::undocumented::get_blocked_users(c, &p).await.unwrap();
    l.save(
        "people",
        "Profile of builderman (156) and Roblox (1) as the test account.",
    );
}

/// Block builderman and unblock right away; the account ends as it started.
#[tokio::test]
async fn blocking() {
    let l = live().await;
    let c = &l.client;
    let r = apis::undocumented::post_block_user(c, 156).await;
    let blocked = apis::undocumented::get_is_blocked(c, 156).await;
    // Unblock before asserting anything so a failure never leaves the block in place.
    let u = apis::undocumented::post_unblock_user(c, 156).await;
    assert!(r.unwrap().starts_with("Successfully blocked"));
    assert!(blocked.unwrap());
    assert!(u.unwrap().starts_with("Successfully unblocked"));
    assert!(!apis::undocumented::get_is_blocked(c, 156).await.unwrap());
    l.save("blocking", "Block then unblock builderman.");
}

/// Adopt Me (universe 383310974, place 920587237) as the core's experience page loads it,
/// plus favorite/notify/event-RSVP each turned on and off again (the account ends as it
/// started).
#[tokio::test]
async fn experience() {
    const U: i64 = 383_310_974;
    const P: i64 = 920_587_237;
    let l = live().await;
    let c = &l.client;
    let anon = l.anon();
    let me = users::v1::get_users_authenticated(c).await.unwrap().id.unwrap();
    l.rec.clear();
    apis::undocumented::get_place_universe(&anon, P).await.unwrap();
    games::v1::get_games(
        &anon,
        &games::v1::GetGamesParams {
            universe_ids: vec![U],
        },
    )
    .await
    .unwrap();
    games::v1::get_games_votes(
        &anon,
        &games::v1::GetGamesVotesParams {
            universe_ids: vec![U],
        },
    )
    .await
    .unwrap();
    games::v2::get_games_by_universe_id_media(&anon, U).await.unwrap();
    let p = games::v1::GetGamesByPlaceIdServersByServerTypeParams {
        sort_order: Some(2),
        exclude_full_games: Some(false),
        limit: Some(100),
        cursor: None,
    };
    games::v1::get_games_by_place_id_servers_by_server_type(c, P, 0, &p)
        .await
        .unwrap();
    games::v1::get_games_by_universe_id_votes_user(c, U)
        .await
        .unwrap();
    let fav = games::v1::get_games_by_universe_id_favorites(c, U).await.unwrap();
    assert_eq!(fav.is_favorited, Some(false), "start unfavorited");
    let st = followings::v1::get_users_by_user_id_universes_by_universe_id_status(c, me, U)
        .await
        .unwrap();
    assert_eq!(st.is_following, Some(false), "start not following");
    let body = apis::undocumented::AgeRecommendationRequest {
        universe_id: Some(U.to_string()),
    };
    apis::undocumented::post_age_recommendation(&anon, &body)
        .await
        .unwrap();
    let p = apis::undocumented::GetUniverseGamePassesParams {
        pass_view: Some("Full".into()),
        page_size: Some(50),
        page_token: None,
    };
    apis::undocumented::get_universe_game_passes(c, U, &p)
        .await
        .unwrap();
    let p = badges::v1::GetUniversesByUniverseIdBadgesParams {
        limit: Some(100),
        ..Default::default()
    };
    let b = badges::v1::get_universes_by_universe_id_badges(&anon, U, &p)
        .await
        .unwrap();
    let ids: Vec<i64> = b
        .data
        .unwrap_or_default()
        .iter()
        .filter_map(|b| b.id)
        .take(100)
        .collect();
    let p = badges::v1::GetUsersByUserIdBadgesAwardedDatesParams { badge_ids: ids };
    badges::v1::get_users_by_user_id_badges_awarded_dates(c, me, &p)
        .await
        .unwrap();
    let p = apis::undocumented::GetUniverseEventsParams {
        from_utc: Some("2026-09-12T00:00:00Z".into()),
        limit: Some(25),
        cursor: None,
    };
    let ev = apis::undocumented::get_universe_events(c, U, &p).await.unwrap();
    let p = games::v1::GetGamesRecommendationsGameByUniverseIdParams {
        max_rows: 12,
        ..Default::default()
    };
    games::v1::get_games_recommendations_game_by_universe_id(&anon, U, &p)
        .await
        .unwrap();
    let p = games::v1::GetGamesByPlaceIdPrivateServersParams {
        limit: Some(25),
        cursor: None,
        sort_order: None,
    };
    games::v1::get_games_by_place_id_private_servers(c, P, &p)
        .await
        .unwrap();
    games::v1::get_private_servers_enabled_in_universe_by_universe_id(c, U)
        .await
        .unwrap();
    let page = c
        .send(www::undocumented::page_game_request(P).unwrap())
        .await
        .unwrap();
    assert!(String::from_utf8_lossy(&page.body).contains("data-private-server-price"));

    // Reversible writes.
    let on = games::v1::GameFavoritesRequest {
        is_favorited: Some(true),
    };
    games::v1::post_games_by_universe_id_favorites(c, U, &on)
        .await
        .unwrap();
    let off = games::v1::GameFavoritesRequest {
        is_favorited: Some(false),
    };
    games::v1::post_games_by_universe_id_favorites(c, U, &off)
        .await
        .unwrap();
    followings::v1::post_users_by_user_id_universes_by_universe_id(c, me, U)
        .await
        .unwrap();
    followings::v1::delete_users_by_user_id_universes_by_universe_id(c, me, U)
        .await
        .unwrap();
    let now = "2026-10-12T00:00:00";
    if let Some(e) = ev.data.unwrap_or_default().into_iter().find(|e| {
        e.event_time
            .as_ref()
            .and_then(|t| t.start_utc.as_deref())
            .is_some_and(|s| s > now)
    }) {
        let id = e.id.unwrap();
        let going = apis::undocumented::RsvpRequest {
            rsvp_status: Some("going".into()),
        };
        apis::undocumented::post_event_rsvp(c, &id, &going).await.unwrap();
        let not = apis::undocumented::RsvpRequest {
            rsvp_status: Some("notGoing".into()),
        };
        apis::undocumented::post_event_rsvp(c, &id, &not).await.unwrap();
    }
    l.save(
        "experience",
        "Adopt Me (universe 383310974, place 920587237): page data, store, badges, events, \
         private servers; favorite, notify and event RSVP on then off. The game page HTML is \
         trimmed to its private-server data attributes.",
    );
    if record() {
        trim_game_page("experience");
    }
}

/// Keep only the private-server data attributes of a recorded www game page.
fn trim_game_page(name: &str) {
    let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/{name}.json"));
    let mut f: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&p).unwrap()).unwrap();
    for i in f["interactions"].as_array_mut().unwrap() {
        if i["request"]["host"].as_str() == Some("www.roblox.com")
            && let Some(html) = i["response"]["body"].as_str()
        {
            let keep: Vec<&str> = html
                .split_whitespace()
                .filter(|w| w.starts_with("data-private-server") || w.starts_with("data-can-create-server"))
                .collect();
            i["response"]["body"] =
                format!("<div id=\"game-detail-meta-data\" {}></div>", keep.join(" ")).into();
        }
    }
    let mut t = serde_json::to_string_pretty(&f).unwrap();
    t.push('\n');
    std::fs::write(&p, t).unwrap();
}

/// Charts (two sort pages, one "see all"), the signed-in Home feed, favorites of builderman
/// and of the test account, and the test account's followed experiences.
#[tokio::test]
async fn discover() {
    let l = live().await;
    let c = &l.client;
    let me = users::v1::get_users_authenticated(c).await.unwrap().id.unwrap();
    l.rec.clear();
    let session = "00000000-0000-4000-8000-000000000000".to_string();
    let p = apis::undocumented::GetExploreSortsParams {
        session_id: session.clone(),
        device: Some("computer".into()),
        country: Some("all".into()),
        sorts_page_token: None,
    };
    let first = apis::undocumented::get_explore_sorts(c, &p).await.unwrap();
    let p = apis::undocumented::GetExploreSortsParams {
        sorts_page_token: first.next_sorts_page_token.clone(),
        ..p
    };
    apis::undocumented::get_explore_sorts(c, &p).await.unwrap();
    let p = apis::undocumented::GetExploreSortContentParams {
        session_id: session.clone(),
        sort_id: "top-trending".into(),
        device: Some("computer".into()),
        country: Some("all".into()),
    };
    apis::undocumented::get_explore_sort_content(c, &p).await.unwrap();
    let body = apis::undocumented::OmniRecommendationRequest {
        page_type: Some("Home".into()),
        session_id: Some(session),
    };
    apis::undocumented::post_omni_recommendation(c, &body)
        .await
        .unwrap();
    let p = games::v2::GetUsersByUserIdFavoriteGamesParams {
        limit: Some(50),
        ..Default::default()
    };
    #[allow(deprecated)]
    {
        games::v2::get_users_by_user_id_favorite_games(c, 156, &p)
            .await
            .unwrap();
        games::v2::get_users_by_user_id_favorite_games(c, me, &p)
            .await
            .unwrap();
    }
    followings::v2::get_users_by_user_id_universes(c, me)
        .await
        .unwrap();
    l.save(
        "discover",
        "Charts sorts (2 pages) and top-trending, the Home feed (omni-recommendation), favorites \
         of builderman (156) and the test account, followed experiences. Trimmed to 12 games per \
         sort/topic.",
    );
    if record() {
        trim_discover("discover");
    }
}

/// Keep 12 games per sort and topic so the fixture stays small.
fn trim_discover(name: &str) {
    use serde_json::Value;
    let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/{name}.json"));
    let mut f: Value = serde_json::from_str(&std::fs::read_to_string(&p).unwrap()).unwrap();
    for i in f["interactions"].as_array_mut().unwrap() {
        let Some(b) = i.pointer_mut("/response/body").and_then(Value::as_object_mut) else {
            continue;
        };
        let mut keep = std::collections::HashSet::new();
        if let Some(sorts) = b.get_mut("sorts").and_then(Value::as_array_mut) {
            for s in sorts.iter_mut() {
                if let Some(g) = s.get_mut("games").and_then(Value::as_array_mut) {
                    g.truncate(12);
                }
                if let Some(r) = s.get_mut("recommendationList").and_then(Value::as_array_mut) {
                    r.truncate(12);
                    for x in r.iter() {
                        keep.insert(x["contentId"].to_string());
                    }
                }
                if let Some(fs) = s.get_mut("filters").and_then(Value::as_array_mut) {
                    for f in fs.iter_mut() {
                        if let Some(o) = f.get_mut("filterOptions").and_then(Value::as_array_mut) {
                            o.truncate(12);
                        }
                    }
                }
            }
        }
        if let Some(meta) = b.get_mut("contentMetadata").and_then(Value::as_object_mut) {
            for (k, v) in meta.iter_mut() {
                if let Some(m) = v.as_object_mut() {
                    if k == "Game" {
                        m.retain(|id, _| keep.contains(id));
                    } else {
                        m.clear();
                    }
                }
            }
        }
        for k in ["contentMetadataByStringId", "sdui", "DebugInfoGroups"] {
            b.remove(k);
        }
        if let Some(g) = b.get_mut("games").and_then(Value::as_array_mut) {
            g.truncate(12);
        }
    }
    let mut t = serde_json::to_string_pretty(&f).unwrap();
    t.push('\n');
    std::fs::write(&p, t).unwrap();
}

/// The realtime hub accepts the account and stays connected (no fixture; see realtime.json).
#[tokio::test]
async fn realtime_connects() {
    let l = live().await;
    let r = tokio::time::timeout(
        std::time::Duration::from_secs(20),
        rbx_api::realtime::run(&l.client, |_| {}),
    )
    .await;
    assert!(r.is_err(), "the hub closed or failed early: {r:?}");
}

/// Private messages (tabs, detail, news), the notification stream and chat (read-only: the
/// test account has no conversations). Mark unread/read and archive/unarchive run on the
/// account's own welcome message and leave it as it was.
#[tokio::test]
async fn messages() {
    use privatemessages::v1 as pm;
    let l = live().await;
    let c = &l.client;
    l.rec.clear();
    let tab = |t| pm::GetMessagesParams {
        page_number: Some(0),
        page_size: Some(20),
        message_tab: Some(t),
    };
    let inbox = pm::get_messages(c, &tab(pm::GetMessagesMessageTab::Inbox))
        .await
        .unwrap();
    pm::get_messages(c, &tab(pm::GetMessagesMessageTab::Sent))
        .await
        .unwrap();
    let first = inbox
        .collection
        .unwrap_or_default()
        .into_iter()
        .next()
        .expect("welcome message");
    let id = first.id.unwrap();
    pm::get_messages_by_message_id(c, id).await.unwrap();
    pm::get_announcements(c).await.unwrap();
    let ids = pm::BatchMessagesRequest {
        message_ids: Some(vec![id]),
    };
    pm::post_messages_mark_unread(c, &ids).await.unwrap();
    pm::post_messages_mark_read(c, &ids).await.unwrap();
    if first.is_read == Some(false) {
        pm::post_messages_mark_unread(c, &ids).await.unwrap();
    }
    pm::post_messages_archive(c, &ids).await.unwrap();
    pm::get_messages(c, &tab(pm::GetMessagesMessageTab::Archive))
        .await
        .unwrap();
    pm::post_messages_unarchive(c, &ids).await.unwrap();
    let p = notifications::v2::GetStreamNotificationsGetRecentParams {
        start_index: Some(0),
        max_rows: Some(20),
    };
    notifications::v2::get_stream_notifications_get_recent(c, &p)
        .await
        .unwrap();
    notifications::v2::post_stream_notifications_clear_unread(c)
        .await
        .unwrap();
    let p = apis::undocumented::GetChatConversationsParams {
        include_user_data: Some(true),
        cursor: None,
        page_size: Some(20),
    };
    apis::undocumented::get_chat_conversations(c, &p).await.unwrap();
    apis::undocumented::get_chat_unread(c).await.unwrap();
    apis::undocumented::get_chat_settings(c).await.unwrap();
    l.save(
        "messages",
        "Private messages: inbox, sent, detail, news, mark unread/read, archive, archive tab, \
         unarchive; the notification stream and clear-unread; chat conversations (none), unread \
         counts and settings.",
    );
}

/// A recording client with no cookie, for public data. Needs no RBX_TEST_COOKIE.
fn anon_live() -> Live {
    let rec = Arc::new(Recorder::default());
    let client = rec.attach(Client::builder(AccountId::ANONYMOUS)).build().unwrap();
    // Builderman (156) is a public Roblox account; Roblox (1) creates the items.
    let scrub = Scrubber::new().keep_ids(&[1, 156]);
    Live { client, rec, scrub }
}

/// The calls rbx-core makes for the Marketplace, an item page and a public inventory, signed out.
/// The same order and parameters as `rbx-core`'s `market` and `inventory` modules.
#[tokio::test]
async fn market() {
    let l = anon_live();
    let c = &l.client;
    let menu = catalog::undocumented::get_navigation_menu_items(c).await.unwrap();
    let all = menu.categories.unwrap()[0].taxonomy.clone().unwrap();
    use catalog::v2::{
        GetSearchItemsDetailsSalesTypeFilter as Sales, GetSearchItemsDetailsSortAggregation as Agg,
        GetSearchItemsDetailsSortType as Sort,
    };
    let p = catalog::v2::GetSearchItemsDetailsParams {
        taxonomy: Some(all),
        sort_type: Some(Sort::Favorited),
        sort_aggregation: Some(Agg::AllTime),
        sales_type_filter: Some(Sales::All),
        creator_name: Some("Roblox".into()),
        limit: Some(60),
        ..Default::default()
    };
    let found = catalog::v2::get_search_items_details(c, &p).await.unwrap();
    assert!(!found.data.unwrap().is_empty());
    for (id, t) in [(1365767, "Asset"), (299652, "Bundle")] {
        let p = catalog::undocumented::GetItemDetailsParams { item_type: t.into() };
        catalog::undocumented::get_item_details(c, id, &p).await.unwrap();
        if t == "Asset" {
            economy::undocumented::get_asset_economy_details(c, id)
                .await
                .unwrap();
        }
        let topic_req = catalog::v1::TopicRequestModel {
            gender_type: Some(catalog::v1::TopicRequestModelGenderType::UnknownValue),
            input_query: Some(String::new()),
            items: Some(vec![catalog::v1::AvatarItem {
                item_type: Some(if t == "Bundle" {
                    catalog::v1::AvatarItemItemType::Bundle
                } else {
                    catalog::v1::AvatarItemItemType::Asset
                }),
                target_id: Some(id),
            }]),
            max_result: Some(10),
            select_topics: Some(Vec::new()),
        };
        catalog::v1::post_topic_get_topics(c, &topic_req).await.unwrap();
    }
    let p = catalog::undocumented::GetAssetRecommendationsParams {
        asset_type_id: 8,
        context_asset_id: 1365767,
        num_items: Some(30),
    };
    let recs = catalog::undocumented::get_asset_recommendations(c, &p)
        .await
        .unwrap();
    let body = catalog::v1::MultigetItemDetailsRequestModel {
        items: Some(
            recs.data
                .unwrap()
                .into_iter()
                .map(|id| catalog::v1::MultigetItemDetailsRequestItem {
                    id: Some(id),
                    item_type: Some(catalog::v1::CatalogSearchDetailedCompleteResponseItemItemType::Asset),
                })
                .collect(),
        ),
    };
    catalog::v1::post_catalog_items_details(c, &body).await.unwrap();
    let helm = "0d1d5570-68e5-49d8-8c9e-fdb777863bd0";
    let p = apis::undocumented::GetItemResellersParams {
        limit: Some(30),
        cursor: None,
    };
    apis::undocumented::get_item_resellers(c, helm, &p).await.unwrap();
    apis::undocumented::get_item_resale_data(c, helm).await.unwrap();
    let p = inventory::v2::GetAssetsByAssetIdOwnersParams {
        limit: Some(100),
        cursor: None,
        sort_order: Some(inventory::v2::GetAssetsByAssetIdOwnersSortOrder::Asc),
    };
    inventory::v2::get_assets_by_asset_id_owners(c, 1365767, &p)
        .await
        .unwrap();
    // Builderman's public inventory.
    inventory::v1::get_users_by_user_id_categories(c, 156)
        .await
        .unwrap();
    inventory::v1::get_users_by_user_id_can_view_inventory(c, 156)
        .await
        .unwrap();
    let p = inventory::v1::GetUsersByUserIdAssetsCollectiblesParams {
        limit: Some(100),
        sort_order: Some(inventory::v1::GetUsersByUserIdAssetsCollectiblesSortOrder::Desc),
        ..Default::default()
    };
    inventory::v1::get_users_by_user_id_assets_collectibles(c, 156, &p)
        .await
        .unwrap();
    let p = inventory::v2::GetUsersByUserIdInventoryByAssetTypeIdParams {
        limit: Some(100),
        cursor: None,
        sort_order: Some(inventory::v2::GetAssetsByAssetIdOwnersSortOrder::Desc),
    };
    inventory::v2::get_users_by_user_id_inventory_by_asset_type_id(c, 156, 8, &p)
        .await
        .unwrap();
    let p = catalog::v1::GetUsersByUserIdBundlesByBundleTypeParams {
        cursor: String::new(),
        limit: Some(100),
        sort_order: Some(catalog::v1::GetUsersByUserIdBundlesByBundleTypeSortOrder::Desc),
    };
    catalog::v1::get_users_by_user_id_bundles_by_bundle_type(
        c,
        156,
        catalog::v1::GetUsersByUserIdBundlesByBundleTypeBundleType::BodyParts,
        &p,
    )
    .await
    .unwrap();
    l.save(
        "market",
        "Signed out: marketplace categories, a search (Roblox's items, most favorited), Valkyrie Helm \
         (1365767) and Silly Fun (bundle 299652) item pages with recommendations, resellers, resale \
         history and owners, and builderman's (156) public inventory. Resale history trimmed to 60 days by hand.",
    );
    let v = api_rolimons_com::undocumented::get_item_values(c).await.unwrap();
    assert!(v.items.unwrap().len() > 100);
    l.save(
        "values",
        "Rolimons item values (third party, opt-in). Trimmed by hand to the items the market fixture uses.",
    );
}

/// Rolimons player stats (third party, opt-in), signed out: builderman (156), Roblox (1) with
/// every badge, and an id Rolimons never scanned (404).
#[tokio::test]
async fn player_values() {
    let l = anon_live();
    let c = &l.client;
    let b = api_rolimons_com::undocumented::get_player_info(c, 156)
        .await
        .unwrap();
    assert_eq!(b.name.as_deref(), Some("builderman"));
    assert!(b.value.unwrap() > 0 && b.rank.is_some());
    let r = api_rolimons_com::undocumented::get_player_info(c, 1)
        .await
        .unwrap();
    assert!(r.rolibadges.unwrap().len() > 10);
    let missing = api_rolimons_com::undocumented::get_player_info(c, 5_000_000_123).await;
    assert!(
        matches!(missing, Err(rbx_api::Error::Status { status: 404, .. })),
        "{missing:?}"
    );
    l.save(
        "player_values",
        "Rolimons player stats (third party, opt-in) for builderman, Roblox and an unscanned id.",
    );
}

/// The public calls rbx-core's `communities` module makes for a community page, signed out:
/// details, roles, members of the biggest role, search, experiences, store, allies and enemies,
/// and past names. Same order and parameters as core. BIG Games Pets (3959677) has a store and
/// experiences.
#[tokio::test]
async fn communities_public() {
    let l = anon_live();
    let c = &l.client;
    let g = 3959677;
    let d = groups::v1::get_groups_by_group_id(c, g).await.unwrap();
    assert_eq!(d.id, Some(g));
    let p = groups::v1::GetGroupsByGroupIdRolesParams {
        include_private: false,
    };
    let roles = groups::v1::get_groups_by_group_id_roles(c, g, &p).await.unwrap();
    let biggest = roles
        .roles
        .unwrap()
        .into_iter()
        .filter(|r| r.rank.unwrap_or(0) > 0)
        .max_by_key(|r| r.member_count.unwrap_or(0))
        .unwrap();
    let p = groups::v1::GetGroupsByGroupIdRolesByRoleSetIdUsersParams {
        limit: Some(100),
        cursor: None,
        sort_order: Some(groups::v1::GetGroupsByGroupIdAuditLogSortOrder::Desc),
    };
    let members =
        groups::v1::get_groups_by_group_id_roles_by_role_set_id_users(c, g, biggest.id.unwrap(), &p)
            .await
            .unwrap();
    assert!(!members.data.unwrap().is_empty());
    let p = groups::v1::GetGroupsSearchParams {
        keyword: "BIG Games".into(),
        prioritize_exact_match: Some(true),
        limit: Some(25),
        cursor: None,
    };
    assert!(
        !groups::v1::get_groups_search(c, &p)
            .await
            .unwrap()
            .data
            .unwrap()
            .is_empty()
    );
    let p = games::v2::GetGroupsByGroupIdGamesV2Params {
        access_filter: Some(2),
        limit: Some(50),
        ..Default::default()
    };
    assert!(
        !games::v2::get_groups_by_group_id_games_v2(c, g, &p)
            .await
            .unwrap()
            .data
            .unwrap()
            .is_empty()
    );
    use catalog::v2::{GetSearchItemsDetailsCreatorType as Creator, GetSearchItemsDetailsSortType as Sort};
    let p = catalog::v2::GetSearchItemsDetailsParams {
        creator_type: Some(Creator::Group),
        creator_target_id: Some(g),
        sort_type: Some(Sort::Updated),
        limit: Some(30),
        ..Default::default()
    };
    assert!(
        !catalog::v2::get_search_items_details(c, &p)
            .await
            .unwrap()
            .data
            .unwrap()
            .is_empty()
    );
    let p = groups::v1::GetGroupsByGroupIdRelationshipsByGroupRelationshipTypeParams {
        start_row_index: 0,
        max_rows: 100,
    };
    for t in ["Allies", "Enemies"] {
        let _ =
            groups::v1::get_groups_by_group_id_relationships_by_group_relationship_type(c, g, t, &p).await;
    }
    let p = groups::v1::GetGroupsByGroupIdNameHistoryParams {
        limit: Some(50),
        ..Default::default()
    };
    groups::v1::get_groups_by_group_id_name_history(c, g, &p)
        .await
        .unwrap();
    l.save(
        "communities_public",
        "Signed out: BIG Games Pets (3959677) details, roles, the biggest role's first 100 members, \
         a search for \"BIG Games\", its public experiences, its store (30 newest items), allies, \
         enemies and past names. Members and the owner are scrubbed.",
    );
}

//! Replay tests for the transport and generated client. Fixtures in `tests/fixtures/`.

use rbx_api::generated::*;
use rbx_api::paging::{CursorPage, collect_cursor, collect_pages};
use rbx_api::replay::Replay;
use rbx_api::rt::ChallengeAnswer;
use rbx_api::{AccountId, Error};

#[tokio::test]
async fn csrf_token_is_fetched_and_request_retried_once() {
    let r = Replay::start(&["auth_ticket"]).await;
    let c = r.client();
    let req = auth::undocumented::post_authentication_ticket_request(
        &auth::undocumented::PostAuthenticationTicketParams {
            referer: "https://www.roblox.com/".into(),
            ..Default::default()
        },
    )
    .unwrap();
    let resp = c.send(req.clone()).await.unwrap();
    assert_eq!(resp.header("rbx-authentication-ticket"), Some("ticket-redacted"));
    let reqs = r.requests().await;
    assert_eq!(reqs.len(), 2);
    assert!(!reqs[0].headers.contains_key("x-csrf-token"));
    assert_eq!(
        reqs[1].headers.get("x-csrf-token").unwrap(),
        "csrf-token-redacted"
    );
    assert_eq!(reqs[1].headers.get("referer").unwrap(), "https://www.roblox.com/");
    assert!(
        reqs[1]
            .headers
            .get("cookie")
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with(".ROBLOSECURITY=")
    );
}

#[tokio::test]
async fn rate_limit_honors_retry_after_then_backs_off() {
    let r = Replay::start(&["rate_limited"]).await;
    let c = r.client();
    let u = users::v1::get_users_by_user_id(&c, 1).await.unwrap();
    assert_eq!(u.name.as_deref(), Some("Roblox"));
    assert_eq!(r.requests().await.len(), 3);
}

#[tokio::test]
async fn rate_limit_gives_up_with_typed_error() {
    let r = Replay::start(&["rate_limited"]).await;
    let c = r
        .builder(AccountId(1))
        .retry(rbx_api::RetryPolicy {
            max_429: 1,
            base_delay: std::time::Duration::from_millis(1),
            ..Default::default()
        })
        .build()
        .unwrap();
    let e = users::v1::get_users_by_user_id(&c, 1).await.unwrap_err();
    assert!(matches!(e, Error::RateLimited { .. }), "{e:?}");
}

#[tokio::test]
async fn server_errors_retry_reads_but_never_writes() {
    let r = Replay::start(&["server_errors"]).await;
    let c = r.client();
    let u = users::v1::get_users_by_user_id(&c, 156).await.unwrap();
    assert_eq!(u.name.as_deref(), Some("builderman"));
    let e = users::v1::post_usernames_users(
        &c,
        &users::v1::MultiGetByUsernameRequest {
            usernames: Some(vec!["x".into()]),
            exclude_banned_users: None,
        },
    )
    .await
    .unwrap_err();
    assert_eq!(e.status(), Some(500));
    // 3 GETs + exactly one POST
    assert_eq!(r.requests().await.len(), 4);
}

#[tokio::test]
async fn challenge_is_a_typed_error_and_can_be_answered() {
    let r = Replay::start(&["challenge"]).await;
    let c = r.client();
    let req = auth::undocumented::post_authentication_ticket_request(&Default::default()).unwrap();
    let e = c.send(req.clone()).await.unwrap_err();
    let Error::Challenge(ch) = e else {
        panic!("expected challenge, got {e:?}")
    };
    assert_eq!(ch.kind, "twostepverification");
    assert_eq!(ch.metadata["actionType"], "Generic");
    let resp = c
        .send(req.with_challenge(ChallengeAnswer {
            id: ch.id.clone(),
            kind: ch.kind.clone(),
            metadata: "e30=".into(),
        }))
        .await
        .unwrap();
    assert_eq!(resp.status, 200);
}

#[tokio::test]
async fn unauthorized_is_typed() {
    let r = Replay::start(&["unauthorized"]).await;
    let e = users::v1::get_users_authenticated(&r.client()).await.unwrap_err();
    assert!(matches!(e, Error::Unauthorized));
}

#[tokio::test]
async fn cursor_pagination_follows_next_page_cursor() {
    let r = Replay::start(&["servers_two_pages"]).await;
    let c = r.client();
    let servers = collect_cursor(
        |cursor| {
            let c = c.clone();
            async move {
                let p = games::v1::GetGamesByPlaceIdServersByServerTypeParams {
                    limit: Some(10),
                    cursor,
                    ..Default::default()
                };
                let page =
                    games::v1::get_games_by_place_id_servers_by_server_type(&c, 920587237, 0, &p).await?;
                Ok(CursorPage {
                    items: page.data.unwrap_or_default(),
                    next: page.next_page_cursor,
                })
            }
        },
        2,
    )
    .await
    .unwrap();
    assert_eq!(servers.len(), 20);
    assert!(servers.iter().all(|s| s.id.is_some() && s.max_players.is_some()));
}

#[tokio::test]
async fn page_number_pagination_stops_at_short_page() {
    let r = Replay::start(&["messages_pages"]).await;
    let c = r.client();
    let msgs = collect_pages(
        |page| {
            let c = c.clone();
            async move {
                let p = privatemessages::v1::GetMessagesParams {
                    page_number: Some(page as i32 - 1),
                    page_size: Some(2),
                    ..Default::default()
                };
                Ok(privatemessages::v1::get_messages(&c, &p)
                    .await?
                    .collection
                    .unwrap_or_default())
            }
        },
        2,
        2,
    )
    .await
    .unwrap();
    assert!(!msgs.is_empty());
}

#[tokio::test]
async fn authenticated_user_decodes() {
    let r = Replay::start(&["users_authenticated"]).await;
    let me = users::v1::get_users_authenticated(&r.client()).await.unwrap();
    assert_eq!(me.id, Some(1_000_001));
    assert_eq!(me.name.as_deref(), Some("TestUser"));
}

/// The realtime client: handshake, cookie and origin headers, notification decoding, close.
#[tokio::test]
#[allow(clippy::result_large_err)] // the closure's error type is tungstenite's, not ours
async fn realtime_hub_frames() {
    use futures::{SinkExt, StreamExt};
    use tokio_tungstenite::tungstenite::Message;
    let fx: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(rbx_api::replay::fixtures_dir().join("realtime.json")).unwrap(),
    )
    .unwrap();
    let frames: Vec<String> = fx["frames"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| f.as_str().unwrap().to_string())
        .collect();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (sock, _) = listener.accept().await.unwrap();
        let seen = std::sync::Arc::new(parking_lot::Mutex::new((
            String::new(),
            String::new(),
            String::new(),
        )));
        let s2 = seen.clone();
        let cb = move |req: &tokio_tungstenite::tungstenite::handshake::server::Request,
                       resp: tokio_tungstenite::tungstenite::handshake::server::Response| {
            let h = |k: &str| {
                req.headers()
                    .get(k)
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("")
                    .to_string()
            };
            *s2.lock() = (req.uri().path().to_string(), h("cookie"), h("origin"));
            Ok(resp)
        };
        let mut ws = tokio_tungstenite::accept_hdr_async(sock, cb).await.unwrap();
        let hello = ws.next().await.unwrap().unwrap();
        assert_eq!(
            hello.into_text().unwrap().as_str(),
            "{\"protocol\":\"json\",\"version\":1}\u{1e}"
        );
        for f in frames {
            ws.send(Message::Text(f.into())).await.unwrap();
        }
        let _ = ws.close(None).await;
        seen.lock().clone()
    });
    let c = rbx_api::Client::builder(AccountId(1_000_001))
        .cookie(rbx_api::Secret::new("test-cookie"))
        .base(rbx_api::Base::Replay(format!("http://{addr}")))
        .build()
        .unwrap();
    let mut got = Vec::new();
    rbx_api::realtime::run(&c, |e| got.push(e)).await.unwrap();
    let (path, cookie, origin) = server.await.unwrap();
    assert_eq!(path, "/realtime-signalr.roblox.com/userhub");
    assert_eq!(cookie, ".ROBLOSECURITY=test-cookie");
    assert_eq!(origin, "https://www.roblox.com");
    let kinds: Vec<(String, String)> = got.iter().map(|e| (e.namespace.clone(), e.kind())).collect();
    assert_eq!(
        kinds,
        [
            ("ChatNotifications".into(), "newmessage".into()),
            ("ChatNotifications".into(), "participanttyping".into()),
            ("NotificationStream".into(), "newnotification".into()),
            ("FriendshipNotifications".into(), "friendshiprequested".into()),
        ]
    );
    assert!(got.iter().all(|e| e.account == AccountId(1_000_001)));
    assert_eq!(got[1].detail["UserId"], 2_000_001);
}

#[tokio::test]
async fn realtime_needs_sign_in() {
    let c = rbx_api::Client::builder(AccountId::ANONYMOUS).build().unwrap();
    assert!(matches!(
        rbx_api::realtime::run(&c, |_| {}).await,
        Err(Error::Unauthorized)
    ));
}

#[tokio::test]
async fn credentials_never_leave_roblox_hosts() {
    use rbx_api::replay::{Fixture, Interaction};
    let ok = |host: &str, path: &str, method: &str| {
        let mut i = Interaction::default();
        i.request.method = method.into();
        i.request.host = host.into();
        i.request.path = path.into();
        i.response.status = 200;
        i.response.body = serde_json::json!({"success": true, "items": {}});
        i
    };
    // A third party that tries to hand us a CSRF token gets no retry and no token stored.
    let mut grab = ok("api.rolimons.com", "/items/v2/itemdetails", "GET");
    grab.response.status = 200;
    grab.response.headers.insert(
        "set-cookie".into(),
        ".ROBLOSECURITY=evil; domain=roblox.com".into(),
    );
    let f = Fixture {
        interactions: vec![grab, ok("users.roblox.com", "/v1/users/1", "GET")],
        ..Default::default()
    };
    let r = Replay::from_fixture(&f).await;
    let c = r
        .builder(AccountId(1))
        .cookie(rbx_api::Secret::new("replay-cookie"))
        .browser_tracker(1)
        .build()
        .unwrap();
    api_rolimons_com::undocumented::get_item_values(&c).await.unwrap();
    users::v1::get_users_by_user_id(&c, 1).await.unwrap();
    let reqs = r.requests().await;
    let third = reqs
        .iter()
        .find(|q| q.url.path().starts_with("/api.rolimons.com"))
        .unwrap();
    assert!(!third.headers.contains_key("cookie"), "{:?}", third.headers);
    assert!(!third.headers.contains_key("x-csrf-token"));
    let rbx = reqs
        .iter()
        .find(|q| q.url.path().starts_with("/users.roblox.com"))
        .unwrap();
    let cookie = rbx.headers.get("cookie").unwrap().to_str().unwrap();
    // Still the account's own cookie, not the one the third party tried to set.
    assert!(cookie.starts_with(".ROBLOSECURITY=replay-cookie"), "{cookie}");
}

#[tokio::test]
async fn quick_sign_in_code_status_and_cancel() {
    let r = Replay::start(&["quick_sign_in"]).await;
    let c = r.builder(AccountId::ANONYMOUS).build().unwrap();
    let code = apis::undocumented::post_quick_login_create(&c).await.unwrap();
    assert_eq!(code.code.as_deref(), Some("ABC123"));
    assert_eq!(code.status.as_deref(), Some("Created"));
    let key = apis::undocumented::QuickLoginKey {
        code: code.code.clone(),
        private_key: code.private_key.clone(),
    };
    let s = apis::undocumented::post_quick_login_status(&c, &key)
        .await
        .unwrap();
    assert_eq!(s.status.as_deref(), Some("Created"));
    assert!(s.account_name.is_none());
    apis::undocumented::post_quick_login_cancel(
        &c,
        &apis::undocumented::QuickLoginCancel { code: code.code },
    )
    .await
    .unwrap();
    let s = apis::undocumented::post_quick_login_status(&c, &key)
        .await
        .unwrap();
    assert_eq!(s.status.as_deref(), Some("Cancelled"));
    // Create went without a token; status got one on its first 403 and kept it.
    let reqs = r.requests().await;
    assert!(!reqs[0].headers.contains_key("x-csrf-token"));
    assert_eq!(reqs.len(), 5);
}

#[tokio::test]
async fn quick_sign_in_login_sets_the_cookie() {
    let r = Replay::start(&["quick_sign_in_approved"]).await;
    let got = std::sync::Arc::new(parking_lot::Mutex::new(None));
    let sink = got.clone();
    let c = r
        .builder(AccountId::ANONYMOUS)
        .on_cookie_rotated(move |_, s| *sink.lock() = Some(s.clone()))
        .build()
        .unwrap();
    let code = apis::undocumented::post_quick_login_create(&c).await.unwrap();
    let key = apis::undocumented::QuickLoginKey {
        code: code.code.clone(),
        private_key: code.private_key.clone(),
    };
    let s = apis::undocumented::post_quick_login_status(&c, &key)
        .await
        .unwrap();
    assert_eq!(s.status.as_deref(), Some("UserLinked"));
    assert_eq!(s.account_name.as_deref(), Some("TestUser"));
    let s = apis::undocumented::post_quick_login_status(&c, &key)
        .await
        .unwrap();
    assert_eq!(s.status.as_deref(), Some("Validated"));
    let me = auth::v2::post_login(
        &c,
        &auth::v2::LoginRequest {
            ctype: Some(auth::v2::LoginRequestCtype::AuthToken),
            cvalue: code.code,
            password: code.private_key,
            ..Default::default()
        },
    )
    .await
    .unwrap();
    assert_eq!(me.user.and_then(|u| u.id), Some(1000001));
    let cookie = got.lock().clone().unwrap();
    assert!(cookie.expose().ends_with("quick-sign-in-cookie"));
    let login = r.requests().await.pop().unwrap();
    let body: serde_json::Value = serde_json::from_slice(&login.body).unwrap();
    assert_eq!(body["ctype"], 4);
    assert_eq!(body["cvalue"], "ABC123");
    assert!(body.get("secureAuthenticationIntent").is_none());
}

#[tokio::test]
async fn asset_delivery_finds_a_shirts_model_and_template() {
    let r = Replay::start(&["assetdelivery"]).await;
    let c = r.client();
    let shirt = assetdelivery::v2::get_asset_id_by_asset_id(&c, 1000011, &Default::default())
        .await
        .unwrap();
    assert_eq!(shirt.asset_type_id, Some(11));
    let url = shirt.locations.unwrap()[0].location.clone().unwrap();
    let model = String::from_utf8(c.fetch_cdn(&url).await.unwrap()).unwrap();
    assert!(model.contains("ShirtTemplate"));
    assert!(model.contains("?id=1000012"));
    let image = assetdelivery::v2::get_asset_id_by_asset_id(&c, 1000012, &Default::default())
        .await
        .unwrap();
    assert_eq!(image.asset_type_id, Some(1));
    assert!(image.errors.is_none());
    assert_eq!(
        image.locations.unwrap()[0].location.as_deref(),
        Some("https://sc0.rbxcdn.com/shirt-image-redacted")
    );
}

/// Transactions recorded live: type flags, totals, and purchases over two cursor pages.
#[tokio::test]
async fn transactions_decode_and_page() {
    use apis::undocumented as a;
    let r = Replay::start(&["transactions"]).await;
    let c = r.client();
    let flags = a::get_user_transaction_types(&c, 1_000_001).await.unwrap();
    assert_eq!(flags.get("HasPurchase"), Some(&true));
    let t = a::get_user_transaction_totals(
        &c,
        1_000_001,
        &a::GetUserTransactionTotalsParams {
            time_frame: "Month".into(),
            transaction_type: "summary".into(),
        },
    )
    .await
    .unwrap();
    assert!(t.outgoing_robux_total.is_some_and(|n| n < 0));
    let page = |cursor| a::GetUserTransactionsParams {
        transaction_type: "Purchase".into(),
        limit: Some(10),
        cursor,
        item_pricing_type: None,
    };
    let first = a::get_user_transactions(&c, 1_000_001, &page(None))
        .await
        .unwrap();
    let rows = first.data.unwrap();
    assert_eq!(rows.len(), 10);
    assert!(
        rows.iter()
            .all(|x| x.id_hash.is_some() && x.currency.as_ref().and_then(|c| c.amount).is_some())
    );
    assert!(
        rows.iter()
            .any(|x| x.details.as_ref().and_then(|d| d.place.as_ref()).is_some())
    );
    let second = a::get_user_transactions(&c, 1_000_001, &page(first.next_page_cursor))
        .await
        .unwrap();
    assert!(!second.data.unwrap().is_empty());
    assert!(second.next_page_cursor.is_none());
}

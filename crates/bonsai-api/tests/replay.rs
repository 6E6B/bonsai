//! Replay tests for the transport and generated client. Fixtures in `tests/fixtures/`.

use bonsai_api::generated::*;
use bonsai_api::paging::{CursorPage, collect_cursor, collect_pages};
use bonsai_api::replay::Replay;
use bonsai_api::rt::ChallengeAnswer;
use bonsai_api::{AccountId, Error};

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
        .retry(bonsai_api::RetryPolicy {
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
async fn rate_limit_headers_pace_and_time_the_retry() {
    let r = Replay::start(&["rate_limit_headers"]).await;
    let c = r
        .builder(AccountId(1))
        .retry(bonsai_api::RetryPolicy {
            max_429: 0,
            ..Default::default()
        })
        .build()
        .unwrap();
    users::v1::get_users_by_user_id(&c, 1).await.unwrap();
    // The first answer used up the window, so the second waits for it to reset.
    let t = std::time::Instant::now();
    users::v1::get_users_by_user_id(&c, 1).await.unwrap();
    assert!(t.elapsed() >= std::time::Duration::from_millis(900));
    // A 429 without Retry-After waits for the window instead of a guessed backoff.
    let e = users::v1::get_users_by_user_id(&c, 1).await.unwrap_err();
    assert!(matches!(e, Error::RateLimited { retry_after_secs: 2 }), "{e:?}");
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
        &std::fs::read_to_string(bonsai_api::replay::fixtures_dir().join("realtime.json")).unwrap(),
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
    let c = bonsai_api::Client::builder(AccountId(1_000_001))
        .cookie(bonsai_api::Secret::new("test-cookie"))
        .base(bonsai_api::Base::Replay(format!("http://{addr}")))
        .build()
        .unwrap();
    let mut got = Vec::new();
    bonsai_api::realtime::run(&c, |e| got.push(e)).await.unwrap();
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
    let c = bonsai_api::Client::builder(AccountId::ANONYMOUS).build().unwrap();
    assert!(matches!(
        bonsai_api::realtime::run(&c, |_| {}).await,
        Err(Error::Unauthorized)
    ));
}

#[tokio::test]
async fn credentials_never_leave_roblox_hosts() {
    use bonsai_api::replay::{Fixture, Interaction};
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
        .cookie(bonsai_api::Secret::new("replay-cookie"))
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
async fn quick_sign_in_approve_from_signed_in_side() {
    use apis::undocumented as a;
    let r = Replay::start(&["quick_sign_in_approve"]).await;
    let anon = r.builder(AccountId::ANONYMOUS).build().unwrap();
    let me = r.client();
    let first = a::post_quick_login_create(&anon).await.unwrap();
    let code = a::QuickLoginCancel {
        code: first.code.clone(),
    };
    let key = a::QuickLoginKey {
        code: first.code.clone(),
        private_key: first.private_key.clone(),
    };
    let d = a::post_quick_login_enter_code(&me, &code).await.unwrap();
    assert_eq!(d.device_info.as_deref(), Some("Chrome on Windows"));
    assert_eq!(d.location.as_deref(), Some("Springfield, United States"));
    let s = a::post_quick_login_status(&anon, &key).await.unwrap();
    assert_eq!(s.status.as_deref(), Some("UserLinked"));
    a::post_quick_login_validate_code(&me, &code).await.unwrap();
    let s = a::post_quick_login_status(&anon, &key).await.unwrap();
    assert_eq!(s.status.as_deref(), Some("Validated"));

    let second = a::post_quick_login_create(&anon).await.unwrap();
    let code = a::QuickLoginCancel { code: second.code };
    a::post_quick_login_enter_code(&me, &code).await.unwrap();
    a::post_quick_login_cancel(&me, &code).await.unwrap();
    let key = a::QuickLoginKey {
        code: code.code.clone(),
        private_key: second.private_key,
    };
    let s = a::post_quick_login_status(&anon, &key).await.unwrap();
    assert_eq!(s.status.as_deref(), Some("Cancelled"));

    // A wrong code answers 400 with a bare JSON string.
    let bad = a::QuickLoginCancel {
        code: Some("ZZZZZZ".into()),
    };
    let e = a::post_quick_login_enter_code(&me, &bad).await.unwrap_err();
    assert!(
        matches!(&e, Error::Status { status: 400, body, .. } if body == "\"CodeInvalid\""),
        "{e:?}"
    );
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
    // The 3D viewer picks a mesh variant with a format header and a representation list.
    let params = assetdelivery::v2::GetAssetIdByAssetIdParams {
        roblox_asset_format: "avatar_meshpart_accessory".into(),
        content_representation_priority_list: Some(
            "W3siZm9ybWF0IjoibWVzaCIsIm1ham9yVmVyc2lvbiI6IjUifV0=".into(),
        ),
        ..Default::default()
    };
    let mesh = assetdelivery::v2::get_asset_id_by_asset_id(&c, 1000013, &params)
        .await
        .unwrap();
    let url = mesh.locations.unwrap()[0].location.clone().unwrap();
    assert!(c.fetch_cdn(&url).await.unwrap().starts_with(b"version 5.00"));
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

/// Google Fonts (third party): the catalog, a family's CSS and its files, with no credentials.
#[tokio::test]
async fn google_fonts_catalog_css_and_files() {
    use fonts_google_com::undocumented as catalog;
    use fonts_googleapis_com::undocumented as css;
    let r = Replay::start(&["fonts"]).await;
    let c = r.client();
    let list = catalog::get_font_catalog(&c)
        .await
        .unwrap()
        .family_metadata_list
        .unwrap();
    let space = list
        .iter()
        .find(|f| f.family.as_deref() == Some("Space Grotesk"))
        .unwrap();
    assert!(space.fonts.as_ref().unwrap().contains_key("700"));
    let params = css::GetFontCssParams {
        family: "Space Grotesk:wght@400;500;600;700".into(),
    };
    let body = c
        .send(css::get_font_css_request(&params).unwrap())
        .await
        .unwrap()
        .body;
    let body = String::from_utf8(body).unwrap();
    let url = body
        .split("url(")
        .nth(1)
        .and_then(|s| s.split(')').next())
        .unwrap()
        .to_owned();
    assert!(url.starts_with("https://fonts.gstatic.com/"));
    let file = c.fetch_font(&url).await.unwrap();
    assert_eq!(&file[..4], &[0, 1, 0, 0]);
    assert!(c.fetch_font("https://evil.example/x.ttf").await.is_err());
    assert!(c.fetch_cdn(&url).await.is_err());
    for q in r.requests().await {
        assert!(q.headers.get("cookie").is_none());
    }
}

/// A background image from a pasted link: any https host, no credentials, capped in size.
#[tokio::test]
async fn user_image_from_any_https_host() {
    let r = Replay::start(&["background_image"]).await;
    let c = r.client();
    let gif = c
        .fetch_user_image("https://images.example.com/cat.gif", 1 << 20)
        .await
        .unwrap();
    assert!(gif.starts_with(b"GIF89a"));
    assert!(
        c.fetch_user_image("https://images.example.com/big.png", 16)
            .await
            .is_err()
    );
    assert!(
        c.fetch_user_image("https://images.example.com/gone.png", 1 << 20)
            .await
            .is_err()
    );
    assert!(
        c.fetch_user_image("http://images.example.com/cat.gif", 1 << 20)
            .await
            .is_err()
    );
    assert!(c.fetch_user_image("file:///etc/passwd", 1 << 20).await.is_err());
    for q in r.requests().await {
        assert!(q.headers.get("cookie").is_none());
    }
}

#[tokio::test]
async fn challenge_hook_answers_and_the_write_is_resent() {
    let r = Replay::start(&["challenge"]).await;
    let seen = std::sync::Arc::new(parking_lot::Mutex::new(Vec::new()));
    let sink = seen.clone();
    let c = r
        .builder(AccountId(1))
        .cookie(bonsai_api::Secret::new("replay-cookie"))
        .on_challenge(move |acc, ch| {
            sink.lock()
                .push((acc, ch.kind.clone(), ch.metadata["challengeId"].clone()));
            let answer =
                ChallengeAnswer::new(&ch.id, &ch.kind, &serde_json::json!({ "verificationToken": "t" }));
            async move { Some(answer) }
        })
        .build()
        .unwrap();
    let req = auth::undocumented::post_authentication_ticket_request(&Default::default()).unwrap();
    let resp = c.send(req).await.unwrap();
    assert_eq!(resp.status, 200);
    assert_eq!(
        seen.lock().as_slice(),
        &[(
            AccountId(1),
            "twostepverification".to_owned(),
            serde_json::json!("abc")
        )]
    );
    let reqs = r.requests().await;
    let retry = reqs.last().unwrap();
    let meta = retry
        .headers
        .get("rblx-challenge-metadata")
        .unwrap()
        .to_str()
        .unwrap();
    use base64::Engine as _;
    let meta = base64::engine::general_purpose::STANDARD.decode(meta).unwrap();
    let meta: serde_json::Value = serde_json::from_slice(&meta).unwrap();
    assert_eq!(meta["verificationToken"], "t");
}

#[tokio::test]
async fn challenge_hook_declining_fails_the_write() {
    let r = Replay::start(&["challenge"]).await;
    let c = r
        .builder(AccountId(1))
        .cookie(bonsai_api::Secret::new("replay-cookie"))
        .on_challenge(|_, _| async { None })
        .build()
        .unwrap();
    let req = auth::undocumented::post_authentication_ticket_request(&Default::default()).unwrap();
    let e = c.send(req).await.unwrap_err();
    assert!(matches!(e, Error::Challenge(_)), "{e:?}");
    assert_eq!(r.requests().await.len(), 1);
}

#[tokio::test]
async fn captcha_keys_and_challenge_continue() {
    let r = Replay::start(&["challenge_captcha"]).await;
    let c = r.client();
    let m = apis::undocumented::get_captcha_metadata(&c).await.unwrap();
    let keys = m.fun_captcha_public_keys.unwrap();
    assert_eq!(
        keys["ACTION_TYPE_WEB_GROUP_JOIN"],
        "00000000-0000-4000-8000-00000000000B"
    );
    apis::undocumented::post_challenge_continue(
        &c,
        &apis::undocumented::ChallengeContinueRequest {
            challenge_id: Some("11111111-2222-3333-4444-555555555555".into()),
            challenge_type: Some("captcha".into()),
            challenge_metadata: Some(
                r#"{"unifiedCaptchaId":"u","captchaToken":"x","actionType":"Generic"}"#.into(),
            ),
        },
    )
    .await
    .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&r.requests().await.pop().unwrap().body).unwrap();
    assert_eq!(body["challengeID"], "11111111-2222-3333-4444-555555555555");
    assert!(body["challengeMetadata"].is_string());
}

#[tokio::test]
async fn account_pin_status_unlock_and_lock() {
    let r = Replay::start(&["account_pin"]).await;
    let c = r.client();
    let s = auth::v1::get_account_pin(&c).await.unwrap();
    assert_eq!(s.is_enabled, Some(true));
    assert_eq!(s.unlocked_until, None);
    auth::v1::post_account_pin_unlock(
        &c,
        &auth::v1::AccountPinRequest {
            pin: Some("1234".into()),
            ..Default::default()
        },
    )
    .await
    .unwrap();
    auth::v1::post_account_pin_lock(&c).await.unwrap();
    let reqs = r.requests().await;
    let unlock = reqs
        .iter()
        .find(|q| q.url.path().ends_with("/v1/account/pin/unlock"))
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&unlock.body).unwrap();
    assert_eq!(body["pin"], "1234");
}

#[tokio::test]
async fn sign_out_others_rotates_the_cookie() {
    let r = Replay::start(&["sign_out_others"]).await;
    let got = std::sync::Arc::new(parking_lot::Mutex::new(None));
    let sink = got.clone();
    let c = r
        .builder(AccountId(1))
        .on_cookie_rotated(move |_, s| *sink.lock() = Some(s.clone()))
        .build()
        .unwrap();
    auth::v2::post_logoutfromallsessionsandreauthenticate(&c, &Default::default())
        .await
        .unwrap();
    let cookie = got.lock().clone().unwrap();
    assert!(cookie.expose().ends_with("reauthenticated-cookie"));
}

#[tokio::test]
async fn two_step_methods_send_and_verify() {
    use twostepverification::v1 as tsv;
    let r = Replay::start(&["two_step"]).await;
    let c = r.client();
    let cfg = tsv::get_users_by_user_id_configuration(&c, 1000001, &Default::default())
        .await
        .unwrap();
    assert_eq!(cfg.primary_media_type, Some(2));
    let methods = cfg.methods.unwrap();
    assert_eq!(methods.len(), 4);
    assert_eq!(
        methods[1].media_type,
        Some(tsv::UserConfigurationMethodMediaType::Authenticator)
    );
    // An unknown media type still decodes.
    assert!(methods[3].media_type.is_some());
    let send = tsv::SendCodeRequest {
        challenge_id: Some("abc".into()),
        ..Default::default()
    };
    tsv::post_users_by_user_id_challenges_email_send_code(&c, 1000001, &send)
        .await
        .unwrap();
    let code = |c: &str| tsv::VerifyCodeRequest {
        challenge_id: Some("abc".into()),
        code: Some(c.into()),
        ..Default::default()
    };
    let e = tsv::post_users_by_user_id_challenges_authenticator_verify(&c, 1000001, &code("000000"))
        .await
        .unwrap_err();
    assert_eq!(e.code(), Some(10));
    let ok = tsv::post_users_by_user_id_challenges_email_verify(&c, 1000001, &code("123456"))
        .await
        .unwrap();
    assert_eq!(
        ok.verification_token.as_deref(),
        Some("verification-token-redacted")
    );
}

#[tokio::test]
async fn authenticator_setup_hands_back_a_key_then_recovery_codes() {
    use twostepverification::v1 as tsv;
    let r = Replay::start(&["security"]).await;
    let c = r.client();
    let pw = || Some("hunter22".to_owned());
    let codes = tsv::get_users_by_user_id_recovery_codes(&c, 1000001)
        .await
        .unwrap();
    assert_eq!(codes.active_count, Some(0));
    let keys =
        auth::v1::post_passkey_list_credentials(&c, &auth::v1::ListPasskeysRequest { all: Some(true) })
            .await
            .unwrap();
    assert_eq!(keys.credentials.unwrap()[0].nickname.as_deref(), Some("Laptop"));
    let body = tsv::EnableTwoStepVerificationRequest {
        password: pw(),
        ..Default::default()
    };
    let setup = tsv::post_users_by_user_id_configuration_authenticator_enable(&c, 1000001, &body)
        .await
        .unwrap();
    assert_eq!(setup.manual_entry_key.as_deref(), Some("ABCDEFGHIJKLMNOP"));
    let verify = |code: &str| tsv::EnableVerifyAuthenticatorRequest {
        code: Some(code.into()),
        password: pw(),
        setup_token: setup.setup_token.clone(),
        ..Default::default()
    };
    let e =
        tsv::post_users_by_user_id_configuration_authenticator_enable_verify(&c, 1000001, &verify("000000"))
            .await
            .unwrap_err();
    assert_eq!(e.status(), Some(400));
    let ok =
        tsv::post_users_by_user_id_configuration_authenticator_enable_verify(&c, 1000001, &verify("123456"))
            .await
            .unwrap();
    assert_eq!(ok.recovery_codes.unwrap().len(), 2);
}

#[tokio::test]
async fn wiki_parse_search_and_missing_page() {
    use roblox_fandom_com::undocumented::{ActionApiParams, action_api};
    let r = Replay::start(&["wiki"]).await;
    // A signed-in client, to check its cookie never goes to Fandom.
    let c = r.client();
    let parse = |page: &str| ActionApiParams {
        action: "parse".into(),
        format: "json".into(),
        formatversion: Some("2".into()),
        page: Some(page.into()),
        redirects: Some("1".into()),
        ..Default::default()
    };
    let p = action_api(&c, &parse("Catalog:Dominus Infernus")).await.unwrap();
    let page = p.parse.unwrap();
    assert_eq!(page.title.as_deref(), Some("Catalog:Dominus Infernus"));
    assert!(page.text.unwrap().contains("portable-infobox"));
    assert_eq!(page.categories.unwrap()[2].hidden, Some(true));
    let missing = action_api(&c, &parse("Catalog:No Such Hat")).await.unwrap();
    assert_eq!(missing.error.unwrap().code.as_deref(), Some("missingtitle"));
    let s = action_api(
        &c,
        &ActionApiParams {
            action: "query".into(),
            format: "json".into(),
            list: Some("search".into()),
            srsearch: Some("dominus".into()),
            srnamespace: Some("0|112|114".into()),
            ..Default::default()
        },
    )
    .await
    .unwrap();
    assert_eq!(s.query.unwrap().search.unwrap().len(), 50);
    for q in r.requests().await {
        assert!(!q.headers.contains_key("cookie"), "{:?}", q.headers);
        assert!(!q.headers.contains_key("x-csrf-token"));
    }
}

#![allow(clippy::unwrap_used, clippy::expect_used)]
use bonsai_api::{RateLimiter, Secret};
use bonsai_hub_api::{Client, Kind, Query, verify};
use serde_json::json;
use std::sync::Arc;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path, query_param},
};
fn client(origin: &str) -> Client {
    bonsai_api::transport::init_tls();
    Client::new(origin, Arc::new(RateLimiter::default())).unwrap()
}
#[test]
fn origin_and_integrity_boundaries() {
    for bad in [
        "http://evil.test",
        "http://localhost.evil.test",
        "https://user:pass@hub.test",
        "https://hub.test/path",
        "https://hub.test/?secret=x",
        "https://hub.test/#x",
        "file:///tmp/hub",
    ] {
        assert!(
            Client::new(bad, Arc::new(RateLimiter::default())).is_err(),
            "{bad}"
        );
    }
    assert!(verify(b"x", &"0".repeat(64)).is_err());
    assert!(
        verify(
            b"abc",
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        )
        .is_ok()
    );
}
#[tokio::test]
async fn browse_contract_and_query_encoding() {
    let server = MockServer::start().await;
    Mock::given(method("GET")).and(path("/v1/themes")).and(query_param("q","a & b")).respond_with(ResponseTemplate::new(200).set_body_json(json!({"items":[{"id":"theme","kind":"themes","version":"1.0.0","sha256":"a".repeat(64),"futureField":true}],"nextOffset":24}))).mount(&server).await;
    let page = client(&server.uri())
        .browse(&Query {
            q: "a & b".into(),
            ..Default::default()
        })
        .await
        .unwrap();
    assert_eq!(page.items[0].id, "theme");
    assert_eq!(page.next_offset, Some(24));
    let requests = server.received_requests().await.unwrap();
    assert!(!requests[0].headers.contains_key("cookie"));
    assert!(!requests[0].headers.contains_key("authorization"));
}
#[tokio::test]
async fn rejects_wrong_identity_and_redirects() {
    let server = MockServer::start().await;
    Mock::given(path("/v1/plugins/example"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"id":"different","kind":"plugins","sha256":"a".repeat(64)})),
        )
        .mount(&server)
        .await;
    assert!(
        client(&server.uri())
            .entry(Kind::Plugins, "example", None)
            .await
            .is_err()
    );
    Mock::given(path("/v1/themes"))
        .respond_with(ResponseTemplate::new(302).insert_header("location", "https://evil.test"))
        .mount(&server)
        .await;
    assert!(client(&server.uri()).browse(&Query::default()).await.is_err());
}
#[tokio::test]
async fn private_session_is_sent_only_as_bearer() {
    let server = MockServer::start().await;
    Mock::given(path("/v1/auth/me"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"id":"publisher","name":"Publisher"})))
        .mount(&server)
        .await;
    let secret = Secret::new("private-session");
    assert_eq!(format!("{secret:?}"), "<redacted>");
    assert_eq!(client(&server.uri()).me(&secret).await.unwrap().id, "publisher");
    let requests = server.received_requests().await.unwrap();
    assert_eq!(
        requests[0].headers.get("authorization").unwrap(),
        "Bearer private-session"
    );
    assert!(!requests[0].headers.contains_key("cookie"));
}
#[test]
fn vendored_contract_version_matches() {
    let contract: serde_json::Value =
        serde_json::from_str(include_str!("../../../docs/hub-openapi.json")).unwrap();
    assert_eq!(contract["info"]["version"], bonsai_hub_api::CONTRACT);
    for p in ["/v1/themes", "/v1/plugins", "/v1/auth/start", "/v1/revocations"] {
        assert!(contract["paths"][p].is_object());
    }
}

#[tokio::test]
async fn version_paths_loopback_and_revocations_are_validated() {
    for bad in [".", "..", "1..0", ".1", "1.", "-1", "1-", "../x"] {
        assert!(!bonsai_hub_api::valid_version_path(bad), "{bad}");
        let mut entry = bonsai_hub_api::Entry {
            id: "ok".into(),
            kind: Kind::Plugins,
            version: bad.into(),
            sha256: "a".repeat(64),
            ..Default::default()
        };
        assert!(client("https://hub.6e6b.dev").download(&entry).await.is_err());
        entry.version = "1.0.0".into();
    }
    for ok in ["1.0.0", "1.2.3-rc.1", "0.1.0+build1"] {
        assert!(bonsai_hub_api::valid_version_path(ok), "{ok}");
    }
    for local in [
        "http://127.0.0.1:8787",
        "http://localhost:8787",
        "http://[::1]:8787",
        "http://localhost",
        "http://127.0.0.1",
    ] {
        assert!(client(local).is_loopback_http(), "{local}");
    }
    assert!(!client("https://hub.6e6b.dev").is_loopback_http());

    let server = MockServer::start().await;
    Mock::given(path("/v1/revocations"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"items":[{"id":"../evil","version":"..","sha256":"nope"}]})),
        )
        .mount(&server)
        .await;
    assert!(client(&server.uri()).revocations().await.is_err());
}

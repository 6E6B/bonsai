//! The site's realtime hub: SignalR's JSON protocol over a WebSocket.
//!
//! [`run`] connects for one account, does the handshake, answers pings and hands every
//! `notification` invocation to a callback until the socket closes. Reconnecting is the
//! caller's job (bonsai-core does it with backoff).

use std::time::Duration;

use futures::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http::HeaderValue;

use crate::generated::realtime_signalr::undocumented as hub;
use crate::{AccountId, Client, Error, Result};

/// SignalR record separator.
const RS: char = '\u{1e}';
const PING_EVERY: Duration = Duration::from_secs(15);
/// The hub pings about every 15 s; give up after this long without hearing anything.
const SILENT_FOR: Duration = Duration::from_secs(60);

/// One push from the hub.
#[derive(Clone, Debug, PartialEq)]
pub struct Event {
    pub account: AccountId,
    /// `ChatNotifications`, `FriendshipNotifications`, `NotificationStream`, ...
    pub namespace: String,
    /// The detail payload (the hub sends it as a JSON string; decoded here).
    pub detail: serde_json::Value,
}

impl Event {
    /// `Type` field of the detail, lowercased (`newmessage`, `participanttyping`, ...).
    pub fn kind(&self) -> String {
        ["Type", "type", "eventType"]
            .iter()
            .find_map(|k| self.detail.get(*k).and_then(|v| v.as_str()))
            .unwrap_or_default()
            .to_ascii_lowercase()
    }
}

/// Parse one WebSocket text frame into its SignalR records.
pub fn parse_frame(account: AccountId, text: &str) -> (Vec<Event>, bool) {
    let mut out = Vec::new();
    let mut close = false;
    for rec in text.split(RS).filter(|r| !r.trim().is_empty()) {
        let Ok(v) = serde_json::from_str::<serde_json::Value>(rec) else {
            continue;
        };
        match v.get("type").and_then(|t| t.as_i64()) {
            Some(1) if v.get("target").and_then(|t| t.as_str()) == Some("notification") => {
                let args = v.get("arguments").and_then(|a| a.as_array());
                let namespace = args
                    .and_then(|a| a.first())
                    .and_then(|n| n.as_str())
                    .unwrap_or_default()
                    .to_string();
                let detail = match args.and_then(|a| a.get(1)) {
                    Some(serde_json::Value::String(s)) => {
                        serde_json::from_str(s).unwrap_or(serde_json::Value::String(s.clone()))
                    }
                    Some(other) => other.clone(),
                    None => serde_json::Value::Null,
                };
                out.push(Event {
                    account,
                    namespace,
                    detail,
                });
            }
            Some(7) => close = true,
            _ => {}
        }
    }
    (out, close)
}

/// Connect and deliver events until the connection ends. Returns `Ok(())` when the hub
/// closes cleanly.
pub async fn run(client: &Client, mut on_event: impl FnMut(Event) + Send) -> Result<()> {
    if !client.signed_in() {
        return Err(Error::Unauthorized);
    }
    let account = client.account();
    let (url, cookie) = client.ws_parts(&hub::get_userhub_request()?);
    let mut req = url
        .into_client_request()
        .map_err(|e| Error::Network(format!("realtime: {e}")))?;
    let h = req.headers_mut();
    h.insert("origin", HeaderValue::from_static("https://www.roblox.com"));
    if let Some(c) = cookie {
        let mut v = HeaderValue::try_from(c.expose())
            .map_err(|_| Error::Encode("cookie contains invalid characters".into()))?;
        v.set_sensitive(true);
        h.insert("cookie", v);
    }
    let connector = if req.uri().scheme_str() == Some("wss") {
        crate::transport::init_tls();
        use rustls_platform_verifier::ConfigVerifierExt;
        let cfg = rustls::ClientConfig::with_platform_verifier()
            .map_err(|e| Error::Network(format!("realtime tls: {e}")))?;
        Some(tokio_tungstenite::Connector::Rustls(std::sync::Arc::new(cfg)))
    } else {
        None
    };
    let (ws, _) = tokio::time::timeout(
        Duration::from_secs(15),
        tokio_tungstenite::connect_async_tls_with_config(req, None, false, connector),
    )
    .await
    .map_err(|_| Error::Network("realtime: connect timed out".into()))?
    .map_err(|e| Error::Network(format!("realtime: {}", redact(&e.to_string()))))?;
    let (mut tx, mut rx) = ws.split();
    let send = |m: String| Message::Text(m.into());
    tx.send(send(format!("{{\"protocol\":\"json\",\"version\":1}}{RS}")))
        .await
        .map_err(|e| Error::Network(format!("realtime: {e}")))?;
    let mut ping = tokio::time::interval(PING_EVERY);
    ping.tick().await;
    loop {
        tokio::select! {
            _ = ping.tick() => {
                tx.send(send(format!("{{\"type\":6}}{RS}")))
                    .await
                    .map_err(|e| Error::Network(format!("realtime: {e}")))?;
            }
            m = tokio::time::timeout(SILENT_FOR, rx.next()) => {
                let m = m.map_err(|_| Error::Network("realtime: hub went quiet".into()))?;
                match m {
                    None => return Ok(()),
                    Some(Err(e)) => return Err(Error::Network(format!("realtime: {e}"))),
                    Some(Ok(Message::Text(t))) => {
                        let (events, close) = parse_frame(account, &t);
                        for e in events {
                            on_event(e);
                        }
                        if close {
                            return Ok(());
                        }
                    }
                    Some(Ok(Message::Close(_))) => return Ok(()),
                    Some(Ok(_)) => {}
                }
            }
        }
    }
}

/// Connection errors can echo request headers; never let a cookie through.
fn redact(s: &str) -> String {
    match s.find("ROBLOSECURITY") {
        Some(i) => format!("{}<redacted>", &s[..i]),
        None => s.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_notifications_pings_and_close() {
        let a = AccountId(1);
        let frame = "{\"type\":1,\"target\":\"notification\",\"arguments\":[\"ChatNotifications\",\"{\\\"Type\\\":\\\"NewMessage\\\",\\\"ConversationId\\\":\\\"c1\\\"}\",3]}\u{1e}{\"type\":6}\u{1e}";
        let (ev, close) = parse_frame(a, frame);
        assert!(!close);
        assert_eq!(ev.len(), 1);
        assert_eq!(ev[0].namespace, "ChatNotifications");
        assert_eq!(ev[0].kind(), "newmessage");
        assert_eq!(ev[0].detail["ConversationId"], "c1");
        let (ev, close) = parse_frame(a, "{\"type\":7}\u{1e}");
        assert!(ev.is_empty() && close);
        let (ev, _) = parse_frame(
            a,
            "not json\u{1e}{\"type\":1,\"target\":\"subscriptionStatus\",\"arguments\":[]}\u{1e}",
        );
        assert!(ev.is_empty());
    }

    #[test]
    fn redacts_cookie() {
        assert_eq!(redact("bad header .ROBLOSECURITY=abc"), "bad header .<redacted>");
    }
}

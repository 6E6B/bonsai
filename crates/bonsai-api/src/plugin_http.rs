//! Isolated plugin HTTPS. This client has no account, cookie jar, CSRF or Roblox transport.
use std::sync::Arc;
use std::time::Duration;

use reqwest::Url;
use serde::Serialize;

use crate::transport::PublicOnly;
use crate::{Error, RateLimiter, Result};

pub const MAX_REQUEST: usize = 64 * 1024;
pub const MAX_RESPONSE: usize = 1024 * 1024;

#[derive(Clone)]
pub struct PluginHttp {
    http: reqwest::Client,
    limiter: Arc<RateLimiter>,
}

#[derive(Serialize)]
pub struct PluginResponse {
    pub status: u16,
    pub body: String,
    /// Parsed JSON when the response is JSON; null otherwise.
    pub json: serde_json::Value,
}

fn denied() -> Error {
    Error::Message("Plugin HTTP requires HTTPS on port 443 to an exact granted public DNS host outside Roblox, without credentials or fragments.".into())
}

fn checked_url(raw: &str, hosts: &[String]) -> Result<Url> {
    if raw.len() > 8192 {
        return Err(denied());
    }
    let url = Url::parse(raw).map_err(|_| denied())?;
    let host = url.host_str().ok_or_else(denied)?;
    if url.scheme() != "https"
        || url.port_or_known_default() != Some(443)
        || !url.username().is_empty()
        || url.password().is_some()
        || url.fragment().is_some()
        || url.domain().is_none()
        || !host.contains('.')
        || host.ends_with('.')
        || !hosts.iter().any(|h| h == host)
        || ["roblox.com", "roblox.cn", "rbxcdn.com", "rbx.com"]
            .iter()
            .any(|d| host == *d || host.ends_with(&format!(".{d}")))
    {
        return Err(denied());
    }
    Ok(url)
}

impl PluginHttp {
    pub fn new(limiter: Arc<RateLimiter>) -> Result<Self> {
        crate::transport::init_tls();
        let http = reqwest::Client::builder()
            .no_proxy()
            .dns_resolver(PublicOnly)
            .redirect(reqwest::redirect::Policy::none())
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(15))
            .build()
            .map_err(|_| Error::Message("Cannot initialize plugin HTTPS.".into()))?;
        Ok(Self { http, limiter })
    }

    /// Never retries, including POST and 429, so a webhook cannot be sent twice by a retry.
    pub async fn request(
        &self,
        hosts: &[String],
        raw: &str,
        method: &str,
        body: Option<&str>,
    ) -> Result<PluginResponse> {
        let url = checked_url(raw, hosts)?;
        if !matches!(method, "GET" | "POST")
            || (method == "GET" && body.is_some())
            || body.is_some_and(|b| b.len() > MAX_REQUEST)
        {
            return Err(Error::Message(
                "Plugin HTTP takes GET or POST with a body up to 64 KiB.".into(),
            ));
        }
        let host = url.host_str().ok_or_else(denied)?.to_owned();
        let mut request = match method {
            "POST" => self.http.post(url),
            _ => self.http.get(url),
        };
        if let Some(body) = body {
            request = request
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .body(body.to_owned());
        }
        // Timeout includes waiting for the machine limiter, DNS, TLS, and the streamed body.
        tokio::time::timeout(Duration::from_secs(15), self.exchange(request, &host))
            .await
            .map_err(|_| Error::Message("Plugin HTTP timed out.".into()))?
    }

    async fn exchange(&self, request: reqwest::RequestBuilder, host: &str) -> Result<PluginResponse> {
        self.limiter.acquire(host).await;
        let mut response = request
            .send()
            .await
            .map_err(|_| Error::Message("Plugin HTTPS request failed.".into()))?;
        let status = response.status().as_u16();
        if status == 429 {
            let wait = response
                .headers()
                .get(reqwest::header::RETRY_AFTER)
                .and_then(|h| h.to_str().ok())
                .and_then(|h| h.parse::<u64>().ok())
                .unwrap_or(30)
                .clamp(1, 300);
            self.limiter.block(host, Duration::from_secs(wait));
        }
        if response.content_length().is_some_and(|n| n > MAX_RESPONSE as u64) {
            return Err(Error::Message("Plugin HTTP response exceeds 1 MiB.".into()));
        }
        let mut bytes = Vec::new();
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|_| Error::Message("Could not read plugin HTTP response.".into()))?
        {
            if chunk.len() > MAX_RESPONSE.saturating_sub(bytes.len()) {
                return Err(Error::Message("Plugin HTTP response exceeds 1 MiB.".into()));
            }
            bytes.extend_from_slice(&chunk);
        }
        let body = String::from_utf8(bytes)
            .map_err(|_| Error::Message("Plugin HTTP response must be UTF-8 text.".into()))?;
        let json = serde_json::from_str(&body).unwrap_or_default();
        Ok(PluginResponse { status, body, json })
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{method, path},
    };

    #[test]
    fn exact_https_hosts_only() {
        let hosts = vec!["example.com".into(), "www.roblox.com".into()];
        for url in [
            "http://example.com/",
            "https://example.com:444/",
            "https://example.com.evil.org/",
            "https://sub.example.com/",
            "https://example.com./",
            "https://user:pass@example.com/",
            "https://example.com/#x",
            "https://www.roblox.com/",
            "https://127.1/",
            "https://[::1]/",
        ] {
            assert!(checked_url(url, &hosts).is_err(), "{url}");
        }
        assert!(checked_url("https://example.com/a?b=c", &hosts).is_ok());
    }

    fn local() -> PluginHttp {
        crate::transport::init_tls();
        PluginHttp {
            http: reqwest::Client::builder()
                .no_proxy()
                .redirect(reqwest::redirect::Policy::none())
                .build()
                .unwrap(),
            limiter: Arc::new(RateLimiter::default()),
        }
    }

    #[tokio::test]
    async fn sends_json_without_credentials_and_never_follows_redirects() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/hook"))
            .respond_with(
                ResponseTemplate::new(302).insert_header("location", format!("{}/secret", server.uri())),
            )
            .expect(1)
            .mount(&server)
            .await;
        let client = local();
        let r = client
            .exchange(
                client.http.post(format!("{}/hook", server.uri())).body("{}"),
                "example.com",
            )
            .await
            .unwrap();
        assert_eq!(r.status, 302);
        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests.len(), 1);
        for h in [
            "cookie",
            "authorization",
            "proxy-authorization",
            "x-csrf-token",
            "roblox-id",
        ] {
            assert!(!requests[0].headers.contains_key(h));
        }
    }

    #[tokio::test]
    async fn resolver_refuses_loopback_and_post_is_not_retried() {
        use reqwest::dns::Resolve;
        assert!(PublicOnly.resolve("localhost".parse().unwrap()).await.is_err());
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(503))
            .expect(1)
            .mount(&server)
            .await;
        let client = local();
        assert_eq!(
            client
                .exchange(client.http.post(server.uri()), "example.com")
                .await
                .unwrap()
                .status,
            503
        );
    }

    #[tokio::test]
    async fn streamed_body_without_content_length_is_capped() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut request = [0; 4096];
            assert!(socket.read(&mut request).await.unwrap() > 0);
            socket
                .write_all(b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n")
                .await
                .unwrap();
            let bytes = vec![b'x'; MAX_RESPONSE + 1];
            let header = format!("{:x}\r\n", bytes.len());
            socket.write_all(header.as_bytes()).await.unwrap();
            let _ = socket.write_all(&bytes).await;
            let _ = socket.write_all(b"\r\n0\r\n\r\n").await;
        });
        let client = local();
        let error = client
            .exchange(client.http.get(format!("http://{addr}")), "example.com")
            .await
            .err()
            .unwrap();
        assert!(error.to_string().contains("exceeds 1 MiB"));
        server.await.unwrap();
    }

    #[tokio::test]
    async fn caps_response_and_returns_json() {
        let server = MockServer::start().await;
        Mock::given(path("/large"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(vec![b'x'; MAX_RESPONSE + 1]))
            .mount(&server)
            .await;
        Mock::given(path("/json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"ok":true})))
            .mount(&server)
            .await;
        let c = local();
        assert!(
            c.exchange(c.http.get(format!("{}/large", server.uri())), "example.com")
                .await
                .is_err()
        );
        let r = c
            .exchange(c.http.get(format!("{}/json", server.uri())), "example.com")
            .await
            .unwrap();
        assert_eq!(r.json["ok"], true);
    }
}

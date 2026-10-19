//! Hand-written HTTP transport: one [`Client`] per account, CSRF retry, challenge errors,
//! shared per-host and per-route rate limits paced by the `x-ratelimit-*` headers Roblox sends,
//! `Retry-After`, and backoff with jitter.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use base64::Engine as _;
use parking_lot::{Mutex, RwLock};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

use crate::error::{ApiError, Challenge, Error, Result};
use crate::rt::{Body, ChallengeAnswer, Method, Request};
use crate::secret::Secret;

/// Which signed-in account a request belongs to. `AccountId::ANONYMOUS` is for public data
/// fetched without a cookie.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default, serde::Serialize, serde::Deserialize,
)]
pub struct AccountId(pub u64);

impl AccountId {
    pub const ANONYMOUS: AccountId = AccountId(0);
    pub fn is_anonymous(self) -> bool {
        self.0 == 0
    }
}

impl std::fmt::Display for AccountId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Whether `host` is Roblox's. Only these hosts ever see the cookie, the browser tracker or a
/// CSRF token, and only their responses may rotate them. Third-party hosts (item values) get a
/// bare request.
pub fn is_roblox_host(host: &str) -> bool {
    host == "roblox.com" || host.ends_with(".roblox.com")
}

/// Where requests go. Tests point every host at a local mock server.
#[derive(Clone, Debug)]
pub enum Base {
    Production,
    /// `{url}/{host}{path}`
    Replay(String),
}

impl Base {
    fn url(&self, host: &str, path: &str) -> String {
        match self {
            Base::Production => format!("https://{host}{path}"),
            Base::Replay(u) => format!("{}/{host}{path}", u.trim_end_matches('/')),
        }
    }
}

/// Retry timing. Production uses [`RetryPolicy::default`]; tests shrink the delays.
#[derive(Clone, Debug)]
pub struct RetryPolicy {
    pub max_429: u32,
    pub max_5xx: u32,
    pub base_delay: Duration,
    pub max_delay: Duration,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_429: 3,
            max_5xx: 3,
            base_delay: Duration::from_millis(400),
            max_delay: Duration::from_secs(30),
        }
    }
}

impl RetryPolicy {
    /// Exponential backoff with full jitter.
    pub fn backoff(&self, attempt: u32) -> Duration {
        let exp = self.base_delay.saturating_mul(1u32 << attempt.min(10));
        let cap = exp.min(self.max_delay);
        let ms = cap.as_millis() as u64;
        Duration::from_millis(ms / 2 + fastrand::u64(0..=ms / 2))
    }
}

#[derive(Debug)]
struct Bucket {
    tokens: f64,
    last: Instant,
    blocked_until: Option<Instant>,
}

type BlockHook = Box<dyn Fn(&str, Duration) + Send + Sync>;

/// Per-host token buckets, shared by every account in the process, so five accounts do not
/// look like five times the traffic from one machine. A route (host and path prefix) can have
/// its own bucket: a 429 there holds only that route, not the whole host.
pub struct RateLimiter {
    buckets: Mutex<HashMap<String, Bucket>>,
    /// requests per second, per host
    rate: f64,
    burst: f64,
    /// By bucket key: a host, or a route's `host/prefix`.
    overrides: HashMap<String, (f64, f64)>,
    /// `(host, path prefix)`, longest prefix first.
    routes: Vec<(String, String)>,
    on_block: Mutex<Option<BlockHook>>,
}

impl std::fmt::Debug for RateLimiter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RateLimiter")
            .field("rate", &self.rate)
            .field("burst", &self.burst)
            .field("overrides", &self.overrides)
            .field("routes", &self.routes)
            .finish()
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new(6.0, 12.0)
    }
}

impl RateLimiter {
    pub fn new(rate_per_sec: f64, burst: f64) -> Self {
        Self {
            buckets: Mutex::new(HashMap::new()),
            rate: rate_per_sec,
            burst,
            overrides: HashMap::new(),
            routes: Vec::new(),
            on_block: Mutex::new(None),
        }
    }

    /// Slower limits for hosts that are known to be strict.
    pub fn with_host(mut self, host: &str, rate_per_sec: f64, burst: f64) -> Self {
        self.overrides.insert(host.to_string(), (rate_per_sec, burst));
        self
    }

    /// A bucket for paths under `prefix` on `host`, apart from the rest of the host. Meant for
    /// endpoints that answer 429 after a few calls: the limiter paces them, and a 429 there is
    /// not retried by [`Client::send`] (retrying only extends the block).
    pub fn with_route(mut self, host: &str, prefix: &str, rate_per_sec: f64, burst: f64) -> Self {
        self.overrides
            .insert(format!("{host}{prefix}"), (rate_per_sec, burst));
        self.routes.push((host.to_string(), prefix.to_string()));
        self.routes.sort_by_key(|r| std::cmp::Reverse(r.1.len()));
        self
    }

    /// The bucket a request to `host` and `path` draws from, and whether it is a route's.
    pub fn bucket(&self, host: &str, path: &str) -> (String, bool) {
        match self
            .routes
            .iter()
            .find(|(h, p)| h == host && path.starts_with(p.as_str()))
        {
            Some((h, p)) => (format!("{h}{p}"), true),
            None => (host.to_string(), false),
        }
    }

    /// Called whenever a host answers 429 and is blocked for a duration.
    pub fn set_on_block(&self, f: impl Fn(&str, Duration) + Send + Sync + 'static) {
        *self.on_block.lock() = Some(Box::new(f));
    }

    fn limits(&self, host: &str) -> (f64, f64) {
        self.overrides
            .get(host)
            .copied()
            .unwrap_or((self.rate, self.burst))
    }

    /// How long to wait before the next request on bucket `host` may go out; takes the token.
    fn reserve(&self, host: &str) -> Duration {
        let (rate, burst) = self.limits(host);
        let now = Instant::now();
        let mut map = self.buckets.lock();
        let b = map.entry(host.to_string()).or_insert(Bucket {
            tokens: burst,
            last: now,
            blocked_until: None,
        });
        b.tokens = (b.tokens + now.duration_since(b.last).as_secs_f64() * rate).min(burst);
        b.last = now;
        let mut wait = Duration::ZERO;
        if let Some(u) = b.blocked_until {
            if u > now {
                wait = u - now;
            } else {
                b.blocked_until = None;
            }
        }
        b.tokens -= 1.0;
        if b.tokens < 0.0 {
            wait = wait.max(Duration::from_secs_f64(-b.tokens / rate));
        }
        wait
    }

    pub async fn acquire(&self, host: &str) {
        let wait = self.reserve(host);
        if !wait.is_zero() {
            tokio::time::sleep(wait).await;
        }
    }

    /// A response said `remaining` requests are left on its tightest window, which resets in
    /// `reset`. This only ever slows the bucket down: a bucket covers a whole host, and the
    /// server may count some of its endpoints apart.
    pub fn observe(&self, host: &str, remaining: u64, reset: Duration) {
        let (_, burst) = self.limits(host);
        let now = Instant::now();
        let mut map = self.buckets.lock();
        let b = map.entry(host.to_string()).or_insert(Bucket {
            tokens: burst,
            last: now,
            blocked_until: None,
        });
        if remaining == 0 {
            let until = now + reset;
            b.blocked_until = Some(b.blocked_until.map_or(until, |u| u.max(until)));
        } else {
            b.tokens = b.tokens.min(remaining as f64);
        }
    }

    /// The server said 429: hold every account's requests on this bucket.
    pub fn block(&self, host: &str, d: Duration) {
        let remaining = {
            let mut map = self.buckets.lock();
            let now = Instant::now();
            let b = map.entry(host.to_string()).or_insert(Bucket {
                tokens: 0.0,
                last: now,
                blocked_until: None,
            });
            let until = b.blocked_until.map_or(now + d, |u| u.max(now + d));
            b.blocked_until = Some(until);
            until.saturating_duration_since(now)
        };
        if let Some(f) = &*self.on_block.lock() {
            f(host, remaining);
        }
    }
}

/// A completed response.
#[derive(Debug)]
pub struct Response {
    pub status: u16,
    pub headers: HeaderMap,
    pub body: Vec<u8>,
}

impl Response {
    pub fn json<T: serde::de::DeserializeOwned>(self) -> Result<T> {
        let bytes: &[u8] = if self.body.is_empty() { b"null" } else { &self.body };
        serde_json::from_slice(bytes).map_err(|e| Error::Decode {
            message: e.to_string(),
            body: String::from_utf8_lossy(&self.body).chars().take(300).collect(),
        })
    }
    pub fn unit(self) -> Result<()> {
        Ok(())
    }
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers.get(name).and_then(|v| v.to_str().ok())
    }
}

type CookieHook = Box<dyn Fn(AccountId, &Secret) + Send + Sync>;
/// `(request, response, csrf token was sent)`
type ResponseHook = Box<dyn Fn(&Request, &Response, bool) + Send + Sync>;
type ChallengeHook = Box<
    dyn Fn(AccountId, Challenge) -> futures::future::BoxFuture<'static, Option<ChallengeAnswer>>
        + Send
        + Sync,
>;

/// How many challenges one request may go through (a captcha can lead to two-step, and so on).
const MAX_CHALLENGES: u32 = 3;

struct Inner {
    http: reqwest::Client,
    account: AccountId,
    cookie: RwLock<Option<Secret>>,
    csrf: RwLock<Option<Secret>>,
    limiter: Arc<RateLimiter>,
    base: Base,
    retry: RetryPolicy,
    on_cookie_rotated: Option<CookieHook>,
    on_response: Option<ResponseHook>,
    on_challenge: Option<ChallengeHook>,
    browser_tracker: Option<u64>,
}

/// HTTP client for one account (or anonymous). Cheap to clone.
#[derive(Clone)]
pub struct Client {
    inner: Arc<Inner>,
}

impl std::fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client")
            .field("account", &self.inner.account)
            .field("signed_in", &self.inner.cookie.read().is_some())
            .field("base", &self.inner.base)
            .finish()
    }
}

/// Install the TLS crypto provider once. Safe to call more than once.
pub fn init_tls() {
    let _ = rustls::crypto::ring::default_provider().install_default();
}

pub struct ClientBuilder {
    account: AccountId,
    cookie: Option<Secret>,
    limiter: Option<Arc<RateLimiter>>,
    base: Base,
    retry: RetryPolicy,
    hook: Option<CookieHook>,
    on_response: Option<ResponseHook>,
    on_challenge: Option<ChallengeHook>,
    user_agent: String,
    browser_tracker: Option<u64>,
}

impl ClientBuilder {
    pub fn cookie(mut self, c: Secret) -> Self {
        self.cookie = Some(c);
        self
    }
    pub fn limiter(mut self, l: Arc<RateLimiter>) -> Self {
        self.limiter = Some(l);
        self
    }
    pub fn base(mut self, b: Base) -> Self {
        self.base = b;
        self
    }
    pub fn retry(mut self, r: RetryPolicy) -> Self {
        self.retry = r;
        self
    }
    /// Called when Roblox rotates the `.ROBLOSECURITY` cookie in a `Set-Cookie` header.
    pub fn on_cookie_rotated(mut self, f: impl Fn(AccountId, &Secret) + Send + Sync + 'static) -> Self {
        self.hook = Some(Box::new(f));
        self
    }
    /// Observe every final response (used by the fixture recorder).
    pub fn on_response(mut self, f: impl Fn(&Request, &Response, bool) + Send + Sync + 'static) -> Self {
        self.on_response = Some(Box::new(f));
        self
    }
    /// Asked when a signed-in write gets a challenge (two-step, captcha). The hook shows it to
    /// the user and returns their answer, and the request is sent again with it; `None` gives
    /// up and the request fails with [`Error::Challenge`]. Reads never ask: the site doesn't
    /// answer challenges on GET either.
    pub fn on_challenge<F, Fut>(mut self, f: F) -> Self
    where
        F: Fn(AccountId, Challenge) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Option<ChallengeAnswer>> + Send + 'static,
    {
        self.on_challenge = Some(Box::new(move |a, c| Box::pin(f(a, c))));
        self
    }
    /// Send the site's `RBXEventTrackerV2` cookie with this browser id. Some APIs (blocking)
    /// refuse requests without one. It identifies the install, not the user.
    pub fn browser_tracker(mut self, id: u64) -> Self {
        self.browser_tracker = Some(id);
        self
    }
    pub fn user_agent(mut self, ua: impl Into<String>) -> Self {
        self.user_agent = ua.into();
        self
    }
    pub fn build(self) -> Result<Client> {
        init_tls();
        let http = reqwest::Client::builder()
            .user_agent(self.user_agent)
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| Error::Network(e.to_string()))?;
        Ok(Client {
            inner: Arc::new(Inner {
                http,
                account: self.account,
                cookie: RwLock::new(self.cookie),
                csrf: RwLock::new(None),
                limiter: self.limiter.unwrap_or_default(),
                base: self.base,
                retry: self.retry,
                on_cookie_rotated: self.hook,
                on_response: self.on_response,
                on_challenge: self.on_challenge,
                browser_tracker: self.browser_tracker,
            }),
        })
    }
}

const COOKIE_NAME: &str = ".ROBLOSECURITY";
/// How long a strict route is held after a 429 without `Retry-After`.
const STRICT_BLOCK: Duration = Duration::from_secs(10);

impl Client {
    pub fn builder(account: AccountId) -> ClientBuilder {
        ClientBuilder {
            account,
            cookie: None,
            limiter: None,
            base: Base::Production,
            retry: RetryPolicy::default(),
            hook: None,
            on_response: None,
            on_challenge: None,
            user_agent: format!("Bonsai/{}", env!("CARGO_PKG_VERSION")),
            browser_tracker: None,
        }
    }

    pub fn account(&self) -> AccountId {
        self.inner.account
    }

    pub fn signed_in(&self) -> bool {
        self.inner.cookie.read().is_some()
    }

    pub fn limiter(&self) -> &Arc<RateLimiter> {
        &self.inner.limiter
    }

    /// Send a request. Handles CSRF, 429 and 5xx retries; returns typed errors for 401 and
    /// challenges. A challenge on a signed-in write goes to the `on_challenge` hook first.
    pub async fn send(&self, mut req: Request) -> Result<Response> {
        let mut csrf_retried = false;
        let mut n429 = 0;
        let mut n5xx = 0;
        let mut challenges = 0;
        let (bucket, strict) = self.inner.limiter.bucket(req.host, &req.path);
        loop {
            self.inner.limiter.acquire(&bucket).await;
            let roblox = is_roblox_host(req.host);
            let csrf_sent = roblox && req.method != Method::GET && self.inner.csrf.read().is_some();
            let resp = match self.build(&req)?.send().await {
                Ok(r) => r,
                Err(e) => {
                    if req.method.idempotent() && n5xx < self.inner.retry.max_5xx {
                        tokio::time::sleep(self.inner.retry.backoff(n5xx)).await;
                        n5xx += 1;
                        continue;
                    }
                    return Err(Error::Network(e.without_url().to_string()));
                }
            };
            let status = resp.status().as_u16();
            let headers = resp.headers().clone();
            if roblox {
                self.capture_rotated_cookie(&headers);
            }
            let body = resp
                .bytes()
                .await
                .map_err(|e| Error::Network(e.without_url().to_string()))?
                .to_vec();
            let response = Response {
                status,
                headers,
                body,
            };
            if let Some(h) = &self.inner.on_response {
                h(&req, &response, csrf_sent);
            }
            let quota = if roblox { quota(&response.headers) } else { None };
            if let Some((remaining, reset)) = quota {
                self.inner.limiter.observe(&bucket, remaining, reset);
            }

            if let Some(ch) = challenge_from(&response.headers) {
                let hook =
                    self.inner.on_challenge.as_ref().filter(|_| {
                        req.method != Method::GET && self.signed_in() && challenges < MAX_CHALLENGES
                    });
                let Some(hook) = hook else {
                    return Err(Error::Challenge(Box::new(ch)));
                };
                match hook(self.inner.account, ch.clone()).await {
                    Some(answer) => {
                        req.challenge = Some(answer);
                        challenges += 1;
                        continue;
                    }
                    None => return Err(Error::Challenge(Box::new(ch))),
                }
            }
            if status == 403
                && roblox
                && !csrf_retried
                && let Some(tok) = response.header("x-csrf-token")
            {
                *self.inner.csrf.write() = Some(Secret::new(tok));
                csrf_retried = true;
                continue;
            }
            if status == 429 {
                tracing::debug!(
                    host = req.host,
                    path = %req.path,
                    limit = response.header("x-ratelimit-limit").unwrap_or("-"),
                    remaining = response.header("x-ratelimit-remaining").unwrap_or("-"),
                    reset = response.header("x-ratelimit-reset").unwrap_or("-"),
                    "429"
                );
                let reset = quota.map(|q| q.1).filter(|d| !d.is_zero());
                let wait = retry_after(&response.headers).or(reset).unwrap_or_else(|| {
                    if strict {
                        STRICT_BLOCK
                    } else {
                        self.inner.retry.backoff(n429)
                    }
                });
                self.inner.limiter.block(&bucket, wait);
                if !strict && n429 < self.inner.retry.max_429 && wait <= self.inner.retry.max_delay {
                    n429 += 1;
                    continue;
                }
                return Err(Error::RateLimited {
                    retry_after_secs: wait.as_secs().max(1),
                });
            }
            if status >= 500 && req.method.idempotent() && n5xx < self.inner.retry.max_5xx {
                tokio::time::sleep(self.inner.retry.backoff(n5xx)).await;
                n5xx += 1;
                continue;
            }
            if status == 401 {
                return Err(Error::Unauthorized);
            }
            if !(200..300).contains(&status) {
                return Err(status_error(status, &response.body));
            }
            return Ok(response);
        }
    }

    /// Download an image from the Roblox CDN (a URL returned by the thumbnails API). Only
    /// `https://*.rbxcdn.com` and `https://contentdelivery.roblox.com` URLs are allowed (asset
    /// delivery sometimes returns the latter); no cookie is sent.
    pub async fn fetch_cdn(&self, url: &str) -> Result<Vec<u8>> {
        self.fetch_file(url, |h| {
            h == "rbxcdn.com" || h.ends_with(".rbxcdn.com") || h == "contentdelivery.roblox.com"
        })
        .await
    }

    /// Download an image from a Fandom wiki article (see [`crate::fandom::IMAGE_HOST`]). Only
    /// that host is allowed; no cookie is sent.
    pub async fn fetch_wiki_image(&self, url: &str) -> Result<Vec<u8>> {
        self.fetch_file(url, |h| h == crate::fandom::IMAGE_HOST).await
    }

    /// Download a font file (a URL from Google Fonts' CSS API). Only `https://fonts.gstatic.com`
    /// URLs are allowed; no cookie is sent.
    pub async fn fetch_font(&self, url: &str) -> Result<Vec<u8>> {
        self.fetch_file(url, |h| h == "fonts.gstatic.com").await
    }

    /// Download an image the user pasted a link to, for the app background. Any `https` host,
    /// since the user picked it; no cookie is sent. Fails past `max` bytes without reading on.
    /// The caller checks that the bytes are an image.
    pub async fn fetch_user_image(&self, url: &str, max: usize) -> Result<Vec<u8>> {
        let host = https_host(url);
        if host.is_empty() || host.contains(['@', '\\']) {
            return Err(Error::Encode("only https links work".into()));
        }
        let url = match &self.inner.base {
            Base::Production => url.to_string(),
            Base::Replay(_) => self.inner.base.url(host, &url[8 + host.len()..]),
        };
        self.inner.limiter.acquire(host).await;
        let mut resp = self
            .inner
            .http
            .get(url)
            .send()
            .await
            .map_err(|e| Error::Network(e.without_url().to_string()))?;
        let status = resp.status().as_u16();
        if !(200..300).contains(&status) {
            return Err(Error::Message(format!("The link answered {status}.")));
        }
        let too_big = || Error::Message(format!("The image is over {} MB.", max >> 20));
        if resp.content_length().is_some_and(|n| n > max as u64) {
            return Err(too_big());
        }
        let mut body = Vec::new();
        while let Some(chunk) = resp
            .chunk()
            .await
            .map_err(|e| Error::Network(e.without_url().to_string()))?
        {
            if body.len() + chunk.len() > max {
                return Err(too_big());
            }
            body.extend_from_slice(&chunk);
        }
        Ok(body)
    }

    /// GET a public file by URL with no credentials, if `allowed` accepts its host.
    async fn fetch_file(&self, url: &str, allowed: impl Fn(&str) -> bool) -> Result<Vec<u8>> {
        let host = https_host(url);
        if !allowed(host) {
            return Err(Error::Encode(format!("refusing to fetch url host {host}")));
        }
        let url = match &self.inner.base {
            Base::Production => url.to_string(),
            Base::Replay(_) => self.inner.base.url(host, &url[8 + host.len()..]),
        };
        self.inner.limiter.acquire(host).await;
        let resp = self
            .inner
            .http
            .get(url)
            .send()
            .await
            .map_err(|e| Error::Network(e.without_url().to_string()))?;
        let status = resp.status().as_u16();
        let body = resp
            .bytes()
            .await
            .map_err(|e| Error::Network(e.without_url().to_string()))?
            .to_vec();
        if !(200..300).contains(&status) {
            return Err(status_error(status, &body));
        }
        Ok(body)
    }

    /// URL (ws/wss) and cookie header value for a WebSocket upgrade of `req`. Used only by
    /// [`crate::realtime`]; the cookie stays a [`Secret`].
    pub(crate) fn ws_parts(&self, req: &Request) -> (String, Option<Secret>) {
        let url = self.inner.base.url(req.host, &req.path);
        let url = if let Some(rest) = url.strip_prefix("https://") {
            format!("wss://{rest}")
        } else if let Some(rest) = url.strip_prefix("http://") {
            format!("ws://{rest}")
        } else {
            url
        };
        let cookie = self
            .inner
            .cookie
            .read()
            .as_ref()
            .map(|c| Secret::new(format!("{COOKIE_NAME}={}", c.expose())));
        (url, cookie)
    }

    fn build(&self, req: &Request) -> Result<reqwest::RequestBuilder> {
        let method = match req.method {
            Method::GET => reqwest::Method::GET,
            Method::POST => reqwest::Method::POST,
            Method::PUT => reqwest::Method::PUT,
            Method::PATCH => reqwest::Method::PATCH,
            Method::DELETE => reqwest::Method::DELETE,
        };
        let mut url = self.inner.base.url(req.host, &req.path);
        if !req.query.is_empty() {
            url.push('?');
            url.push_str(&crate::rt::encode_query(&req.query));
        }
        let mut b = self
            .inner
            .http
            .request(method, url)
            .header("accept", "application/json");
        for (k, v) in &req.headers {
            if let (Ok(k), Ok(v)) = (
                HeaderName::try_from(k.as_str()),
                HeaderValue::try_from(v.as_str()),
            ) {
                b = b.header(k, v);
            }
        }
        // Credentials go to Roblox only.
        let roblox = is_roblox_host(req.host);
        let tracker = self
            .inner
            .browser_tracker
            .filter(|_| roblox)
            .map(|id| format!("RBXEventTrackerV2=CreateDate=1/1/2026 12:00:00 AM&rbxid=&browserid={id}"));
        if let Some(c) = self.inner.cookie.read().as_ref().filter(|_| roblox) {
            let mut text = format!("{COOKIE_NAME}={}", c.expose());
            if let Some(t) = &tracker {
                text.push_str("; ");
                text.push_str(t);
            }
            let mut v = HeaderValue::try_from(text)
                .map_err(|_| Error::Encode("cookie contains invalid characters".into()))?;
            v.set_sensitive(true);
            b = b.header("cookie", v);
        } else if let Some(t) = tracker
            && let Ok(v) = HeaderValue::try_from(t)
        {
            b = b.header("cookie", v);
        }
        if roblox
            && req.method != Method::GET
            && let Some(t) = self.inner.csrf.read().as_ref()
            && let Ok(mut v) = HeaderValue::try_from(t.expose())
        {
            v.set_sensitive(true);
            b = b.header("x-csrf-token", v);
        }
        if let Some(ch) = &req.challenge {
            b = b
                .header("rblx-challenge-id", ch.id.as_str())
                .header("rblx-challenge-type", ch.kind.as_str())
                .header("rblx-challenge-metadata", ch.metadata.as_str());
        }
        b = match &req.body {
            Body::None => {
                if req.method == Method::GET {
                    b
                } else {
                    // Some services answer 415 to a body-less POST without a content type.
                    b.header("content-type", "application/json")
                        .header("content-length", "0")
                }
            }
            Body::Json(bytes) => b.header("content-type", "application/json").body(bytes.clone()),
            Body::Form(f) => b.form(f),
            Body::Multipart { text, files } => {
                let mut form = reqwest::multipart::Form::new();
                for (k, v) in text {
                    form = form.text(k.clone(), v.clone());
                }
                for (k, f) in files {
                    let part = reqwest::multipart::Part::bytes(f.bytes.clone())
                        .file_name(f.file_name.clone())
                        .mime_str(&f.mime)
                        .map_err(|e| Error::Encode(e.to_string()))?;
                    form = form.part(k.clone(), part);
                }
                b.multipart(form)
            }
        };
        Ok(b)
    }

    fn capture_rotated_cookie(&self, headers: &HeaderMap) {
        for v in headers.get_all("set-cookie") {
            let Ok(s) = v.to_str() else { continue };
            let Some(rest) = s.strip_prefix(COOKIE_NAME).and_then(|r| r.strip_prefix('=')) else {
                continue;
            };
            let value = rest.split(';').next().unwrap_or("").trim();
            if value.is_empty() {
                continue;
            }
            let new = Secret::new(value);
            let changed = self
                .inner
                .cookie
                .read()
                .as_ref()
                .is_none_or(|old| old.expose() != new.expose());
            if changed {
                *self.inner.cookie.write() = Some(new.clone());
                if let Some(h) = &self.inner.on_cookie_rotated {
                    h(self.inner.account, &new);
                }
            }
        }
    }
}

/// The host of an `https://` URL, or empty.
fn https_host(url: &str) -> &str {
    url.strip_prefix("https://")
        .and_then(|r| r.split(['/', '?', '#']).next())
        .unwrap_or("")
}

fn status_error(status: u16, body: &[u8]) -> Error {
    #[derive(serde::Deserialize, Default)]
    #[serde(default)]
    struct Errs {
        errors: Vec<ApiError>,
    }
    let mut errors = serde_json::from_slice::<Errs>(body)
        .map(|e| e.errors)
        .unwrap_or_default();
    // Some services answer with a bare `{"code":0,"message":"..."}`.
    if errors.is_empty()
        && let Ok(one) = serde_json::from_slice::<ApiError>(body)
        && !one.message.is_empty()
    {
        errors.push(one);
    }
    Error::Status {
        status,
        errors,
        body: String::from_utf8_lossy(body).chars().take(300).collect(),
    }
}

fn retry_after(h: &HeaderMap) -> Option<Duration> {
    let v = h.get("retry-after")?.to_str().ok()?;
    v.trim().parse::<u64>().ok().map(Duration::from_secs)
}

/// Longest `x-ratelimit-reset` we believe. Roblox's windows are a minute at most; anything far
/// past that is more likely a timestamp than a duration.
const MAX_RESET: Duration = Duration::from_secs(300);

/// `(remaining, reset)` from Roblox's `x-ratelimit-remaining` and `x-ratelimit-reset`, which
/// describe the window closest to running out (`x-ratelimit-limit: 25, 25;w=1, 10000;w=60`).
fn quota(h: &HeaderMap) -> Option<(u64, Duration)> {
    let num = |n: &str| -> Option<u64> { h.get(n)?.to_str().ok()?.split(',').next()?.trim().parse().ok() };
    let reset = Duration::from_secs(num("x-ratelimit-reset")?);
    (reset <= MAX_RESET).then_some((num("x-ratelimit-remaining")?, reset))
}

fn challenge_from(h: &HeaderMap) -> Option<Challenge> {
    let get = |n: &str| h.get(n).and_then(|v| v.to_str().ok()).map(str::to_string);
    let id = get("rblx-challenge-id")?;
    let kind = get("rblx-challenge-type").unwrap_or_default();
    let metadata_raw = get("rblx-challenge-metadata").unwrap_or_default();
    let metadata = base64::engine::general_purpose::STANDARD
        .decode(metadata_raw.as_bytes())
        .ok()
        .and_then(|b| serde_json::from_slice(&b).ok())
        .unwrap_or(serde_json::Value::Null);
    Some(Challenge {
        id,
        kind,
        metadata,
        metadata_raw,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backoff_grows_and_is_capped() {
        let p = RetryPolicy {
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_millis(1000),
            ..Default::default()
        };
        for a in 0..12 {
            let d = p.backoff(a);
            assert!(d <= Duration::from_millis(1000));
        }
        assert!(p.backoff(0) <= Duration::from_millis(100));
        assert!(p.backoff(3) >= Duration::from_millis(400));
    }

    #[test]
    fn limiter_is_shared_per_host() {
        let l = RateLimiter::new(1.0, 2.0);
        assert!(l.reserve("a").is_zero());
        assert!(l.reserve("a").is_zero());
        assert!(!l.reserve("a").is_zero());
        assert!(l.reserve("b").is_zero());
        l.block("b", Duration::from_secs(5));
        assert!(l.reserve("b") >= Duration::from_secs(4));
    }

    #[test]
    fn quota_headers_slow_the_bucket() {
        let l = RateLimiter::new(10.0, 10.0);
        l.observe("a", 1, Duration::from_secs(1));
        assert!(l.reserve("a").is_zero());
        assert!(!l.reserve("a").is_zero());
        l.observe("b", 0, Duration::from_secs(5));
        assert!(l.reserve("b") >= Duration::from_secs(4));
        // Plenty left never speeds a bucket past its own limit.
        l.observe("c", 1000, Duration::from_secs(1));
        for _ in 0..10 {
            assert!(l.reserve("c").is_zero());
        }
        assert!(!l.reserve("c").is_zero());
    }

    #[test]
    fn parses_quota_headers() {
        let mut h = HeaderMap::new();
        assert_eq!(quota(&h), None);
        h.insert(
            "x-ratelimit-limit",
            HeaderValue::from_static("25, 25;w=1, 10000;w=60"),
        );
        h.insert("x-ratelimit-remaining", HeaderValue::from_static("24"));
        h.insert("x-ratelimit-reset", HeaderValue::from_static("1"));
        assert_eq!(quota(&h), Some((24, Duration::from_secs(1))));
        h.insert("x-ratelimit-reset", HeaderValue::from_static("1760000000"));
        assert_eq!(quota(&h), None);
    }

    #[test]
    fn routes_get_their_own_bucket() {
        let l = RateLimiter::new(10.0, 10.0)
            .with_route("api.x", "/slow/", 1.0, 1.0)
            .with_route("api.x", "/slow/v1/fast/", 10.0, 10.0);
        assert_eq!(l.bucket("api.x", "/other"), ("api.x".to_string(), false));
        assert_eq!(l.bucket("api.y", "/slow/a"), ("api.y".to_string(), false));
        let (slow, strict) = l.bucket("api.x", "/slow/v1/users");
        assert_eq!((slow.as_str(), strict), ("api.x/slow/", true));
        assert_eq!(l.bucket("api.x", "/slow/v1/fast/a").0, "api.x/slow/v1/fast/");
        assert!(l.reserve(&slow).is_zero());
        assert!(!l.reserve(&slow).is_zero(), "the route's own burst");
        l.block(&slow, Duration::from_secs(5));
        assert!(
            l.reserve("api.x").is_zero(),
            "a 429 on the route leaves the host alone"
        );
    }
}

//! Version 1 of the Bonsai hub contract. No Roblox cookie store, redirects or webview HTTP.
use bonsai_api::{RateLimiter, Secret};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use std::time::Duration;
use url::Url;

pub const DEFAULT_ORIGIN: &str = "https://hub.6e6b.dev";

pub const CONTRACT: &str = "1.0.0";
pub const MAX_PACKAGE: usize = 8 * 1024 * 1024;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    #[default]
    Themes,
    Plugins,
    #[serde(other)]
    Unknown,
}
impl Kind {
    pub fn path(self) -> Result<&'static str, String> {
        match self {
            Self::Themes => Ok("themes"),
            Self::Plugins => Ok("plugins"),
            Self::Unknown => Err("Unknown hub content type.".into()),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Entry {
    pub id: String,
    pub kind: Kind,
    pub owner_id: String,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub base: Option<String>,
    pub updated_at: u64,
    pub version: String,
    pub sha256: String,
    pub document: Option<String>,
    pub permissions: Vec<String>,
    pub api_version: Option<u32>,
    pub size: usize,
    pub release_id: String,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Version {
    pub version: String,
    pub sha256: String,
    pub created_at: u64,
    pub permissions: Vec<String>,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Page {
    pub items: Vec<Entry>,
    pub next_offset: Option<u32>,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct User {
    pub id: String,
    pub name: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthStart {
    pub authorization_url: String,
    pub poll_token: String,
    pub expires_in: u64,
}
// Deliberately not Debug/Serialize: bearer tokens never cross IPC or enter logs.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub token: String,
    pub expires_at: u64,
    pub user: User,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Submission {
    pub id: String,
    pub status: String,
    pub reason: Option<String>,
    pub kind: Kind,
    pub entry_id: String,
    pub version: String,
    pub release_id: String,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Revocation {
    pub id: String,
    pub version: String,
    pub sha256: String,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Query {
    pub kind: Kind,
    pub q: String,
    pub base: String,
    pub sort: String,
    pub offset: u32,
}

#[derive(Clone)]
pub struct Client {
    http: reqwest::Client,
    base: Url,
    limiter: Arc<RateLimiter>,
}
impl Client {
    /// HTTPS for remote hosts; literal loopback HTTP only for local development. No embedded
    /// credentials, query, fragment or path. Configured by the Rust host, never by hub content.
    pub fn new(base: &str, limiter: Arc<RateLimiter>) -> Result<Self, String> {
        bonsai_api::transport::init_tls();
        let base = Url::parse(base).map_err(|_| "Invalid hub URL.".to_owned())?;
        let local = matches!(base.host_str(), Some("localhost" | "127.0.0.1" | "[::1]"));
        if !(base.scheme() == "https" || base.scheme() == "http" && local)
            || !base.username().is_empty()
            || base.password().is_some()
            || base.query().is_some()
            || base.fragment().is_some()
            || base.path() != "/"
        {
            return Err("Hub URL must be an HTTPS origin, or loopback HTTP.".into());
        }
        let http = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .timeout(Duration::from_secs(25))
            .user_agent("Bonsai-Hub/1")
            .build()
            .map_err(|_| "Could not start hub client.".to_owned())?;
        Ok(Self { http, base, limiter })
    }
    pub fn origin(&self) -> String {
        self.base.origin().ascii_serialization()
    }
    async fn send(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<Vec<u8>>,
        session: Option<&Secret>,
        metadata: Option<&Metadata>,
        max: usize,
    ) -> Result<(u16, Vec<u8>), String> {
        self.limiter
            .acquire(self.base.host_str().unwrap_or_default())
            .await;
        let url = self
            .base
            .join(path)
            .map_err(|_| "Invalid hub route.".to_owned())?;
        let mut request = self.http.request(method, url);
        if let Some(token) = session {
            request = request.bearer_auth(token.expose());
        }
        if let Some(meta) = metadata {
            let json = serde_json::to_string(meta).map_err(|_| "Invalid metadata.".to_owned())?;
            request = request
                .header(
                    "X-Bonsai-Metadata",
                    url::form_urlencoded::byte_serialize(json.as_bytes()).collect::<String>(),
                )
                .header("Content-Type", "application/octet-stream");
        } else {
            request = request.header("Content-Type", "application/json");
        }
        if let Some(body) = body {
            request = request.body(body);
        }
        let mut response = request
            .send()
            .await
            .map_err(|_| "Could not reach the Bonsai hub.".to_owned())?;
        let status = response.status().as_u16();
        if !(200..300).contains(&status) {
            return Err(match status {
                401 => "Sign into Bonsai to continue.",
                403 => "You cannot change this publisher's content.",
                404 => "This hub release is unavailable.",
                409 => "That version already exists or its status changed.",
                413 => "This package is too large.",
                422 => "The hub refused invalid content.",
                429 => "Too many requests. Try again later.",
                503 => "The hub is not configured for this operation.",
                _ => "The Bonsai hub request failed.",
            }
            .into());
        }
        if response.content_length().is_some_and(|n| n > max as u64) {
            return Err("Hub response is too large.".into());
        }
        let mut bytes = Vec::new();
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|_| "Could not read hub response.".to_owned())?
        {
            if bytes.len() + chunk.len() > max {
                return Err("Hub response is too large.".into());
            }
            bytes.extend_from_slice(&chunk);
        }
        Ok((status, bytes))
    }
    async fn json<T: DeserializeOwned>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<serde_json::Value>,
        session: Option<&Secret>,
    ) -> Result<T, String> {
        let body = body
            .map(|x| serde_json::to_vec(&x))
            .transpose()
            .map_err(|_| "Invalid hub request.".to_owned())?;
        let (_, bytes) = self
            .send(method, path, body, session, None, 2 * 1024 * 1024)
            .await?;
        serde_json::from_slice(&bytes).map_err(|_| "Hub returned an incompatible response.".into())
    }
    pub async fn browse(&self, q: &Query) -> Result<Page, String> {
        let mut url = self
            .base
            .join(&format!("v1/{}", q.kind.path()?))
            .map_err(|_| "Invalid hub route.".to_owned())?;
        {
            let mut pairs = url.query_pairs_mut();
            pairs
                .append_pair("q", &q.q)
                .append_pair("offset", &q.offset.to_string())
                .append_pair("limit", "24")
                .append_pair("sort", if q.sort == "name" { "name" } else { "updated" });
            if matches!(q.base.as_str(), "dark" | "light") {
                pairs.append_pair("base", &q.base);
            }
            if q.kind == Kind::Plugins {
                pairs.append_pair("apiVersion", "1");
            }
        }
        let page: Page = self
            .json(
                reqwest::Method::GET,
                &format!("{}?{}", url.path(), url.query().unwrap_or_default()),
                None,
                None,
            )
            .await?;
        if page.items.len() > 24 || page.items.iter().any(|e| !valid_entry(e) || e.kind != q.kind) {
            return Err("Hub returned invalid listing data.".into());
        }
        Ok(page)
    }
    pub async fn entry(&self, kind: Kind, id: &str, version: Option<&str>) -> Result<Entry, String> {
        if !valid_id(id) {
            return Err("Invalid hub ID.".into());
        }
        let mut path = format!("v1/{}/{id}", kind.path()?);
        if let Some(version) = version {
            path.push_str("?version=");
            path.push_str(&url::form_urlencoded::byte_serialize(version.as_bytes()).collect::<String>());
        }
        let entry: Entry = self.json(reqwest::Method::GET, &path, None, None).await?;
        if entry.id != id
            || entry.kind != kind
            || !valid_entry(&entry)
            || version.is_some_and(|v| v != entry.version)
        {
            return Err("Hub returned a different release.".into());
        }
        Ok(entry)
    }
    pub async fn versions(&self, kind: Kind, id: &str) -> Result<Vec<Version>, String> {
        if !valid_id(id) {
            return Err("Invalid hub ID.".into());
        }
        #[derive(Deserialize)]
        struct List {
            items: Vec<Version>,
        }
        let result = self
            .json::<List>(
                reqwest::Method::GET,
                &format!("v1/{}/{id}/versions", kind.path()?),
                None,
                None,
            )
            .await?;
        if result.items.len() > 100
            || result
                .items
                .iter()
                .any(|v| !valid_version_path(&v.version) || !valid_hash(&v.sha256))
        {
            return Err("Hub returned invalid version history.".into());
        }
        Ok(result.items)
    }
    pub async fn download(&self, e: &Entry) -> Result<Vec<u8>, String> {
        if e.kind != Kind::Plugins || !valid_id(&e.id) || !valid_version_path(&e.version) {
            return Err("Invalid plugin release.".into());
        }
        let (_, bytes) = self
            .send(
                reqwest::Method::GET,
                &format!("v1/plugins/{}/download/{}", e.id, e.version),
                None,
                None,
                None,
                MAX_PACKAGE,
            )
            .await?;
        verify(&bytes, &e.sha256)?;
        Ok(bytes)
    }
    pub async fn publish_theme(
        &self,
        meta: &Metadata,
        document: serde_json::Value,
        session: &Secret,
    ) -> Result<serde_json::Value, String> {
        self.json(reqwest::Method::POST,"v1/themes",Some(serde_json::json!({"name":meta.name,"description":meta.description,"tags":meta.tags,"document":document})),Some(session)).await
    }
    pub async fn publish_plugin(
        &self,
        meta: &Metadata,
        bytes: Vec<u8>,
        session: &Secret,
    ) -> Result<(), String> {
        if bytes.len() > MAX_PACKAGE {
            return Err("Package exceeds 8 MiB.".into());
        }
        self.send(
            reqwest::Method::POST,
            "v1/plugins",
            Some(bytes),
            Some(session),
            Some(meta),
            65536,
        )
        .await?;
        Ok(())
    }
    pub async fn start_auth(&self, challenge: &str) -> Result<AuthStart, String> {
        self.json(
            reqwest::Method::POST,
            "v1/auth/start",
            Some(serde_json::json!({"challenge":challenge})),
            None,
        )
        .await
    }
    pub async fn exchange(&self, poll: &str, verifier: &str) -> Result<Option<Session>, String> {
        let body = serde_json::to_vec(&serde_json::json!({"pollToken":poll,"verifier":verifier}))
            .map_err(|_| "Invalid sign-in.".to_owned())?;
        let (status, bytes) = self
            .send(
                reqwest::Method::POST,
                "v1/auth/exchange",
                Some(body),
                None,
                None,
                65536,
            )
            .await?;
        if status == 202 {
            return Ok(None);
        }
        serde_json::from_slice(&bytes)
            .map(Some)
            .map_err(|_| "Invalid session response.".into())
    }
    pub async fn me(&self, session: &Secret) -> Result<User, String> {
        let user: User = self
            .json(reqwest::Method::GET, "v1/auth/me", None, Some(session))
            .await?;
        if user.id.is_empty() || user.id.len() > 64 || user.name.len() > 256 {
            return Err("Hub returned an invalid identity.".into());
        }
        Ok(user)
    }
    pub async fn logout(&self, session: &Secret) -> Result<(), String> {
        self.send(
            reqwest::Method::POST,
            "v1/auth/logout",
            None,
            Some(session),
            None,
            65536,
        )
        .await?;
        Ok(())
    }
    pub async fn submissions(&self, session: &Secret) -> Result<Vec<Submission>, String> {
        #[derive(Deserialize)]
        struct List {
            items: Vec<Submission>,
        }
        Ok(self
            .json::<List>(
                reqwest::Method::GET,
                "v1/account/submissions",
                None,
                Some(session),
            )
            .await?
            .items)
    }
    pub async fn revocations(&self) -> Result<Vec<Revocation>, String> {
        #[derive(Deserialize)]
        struct List {
            items: Vec<Revocation>,
        }
        Ok(self
            .json::<List>(reqwest::Method::GET, "v1/revocations", None, None)
            .await?
            .items)
    }
    pub async fn unlist(&self, kind: Kind, id: &str, session: &Secret) -> Result<(), String> {
        if !valid_id(id) {
            return Err("Invalid hub ID.".into());
        }
        self.json::<serde_json::Value>(
            reqwest::Method::PATCH,
            &format!("v1/{}/{id}", kind.path()?),
            Some(serde_json::json!({"listed":false})),
            Some(session),
        )
        .await?;
        Ok(())
    }
    pub async fn delete(&self, kind: Kind, id: &str, session: &Secret) -> Result<(), String> {
        if !valid_id(id) {
            return Err("Invalid hub ID.".into());
        }
        self.send(
            reqwest::Method::DELETE,
            &format!("v1/{}/{id}", kind.path()?),
            None,
            Some(session),
            None,
            65536,
        )
        .await?;
        Ok(())
    }
    pub async fn report(&self, kind: Kind, id: &str, reason: &str, session: &Secret) -> Result<(), String> {
        if !valid_id(id) {
            return Err("Invalid hub ID.".into());
        }
        self.json::<serde_json::Value>(
            reqwest::Method::POST,
            &format!("v1/{}/{id}/report", kind.path()?),
            Some(serde_json::json!({"reason":reason})),
            Some(session),
        )
        .await?;
        Ok(())
    }
}
fn valid_entry(e: &Entry) -> bool {
    valid_id(&e.id)
        && valid_version_path(&e.version)
        && valid_hash(&e.sha256)
        && e.name.len() <= 256
        && e.description.len() <= 8192
        && e.tags.len() <= 8
        && e.permissions.len() <= 128
        && e.document.as_ref().is_none_or(|d| d.len() <= 32768)
        && e.size <= MAX_PACKAGE
}
pub fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 64
        && !id.starts_with('-')
        && id
            .bytes()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-')
}
fn valid_version_path(v: &str) -> bool {
    !v.is_empty() && v.len() <= 64 && v.bytes().all(|c| c.is_ascii_alphanumeric() || b".-".contains(&c))
}
fn valid_hash(hash: &str) -> bool {
    hash.len() == 64 && hash.bytes().all(|c| c.is_ascii_hexdigit())
}
pub fn verify(bytes: &[u8], hash: &str) -> Result<(), String> {
    let actual: String = Sha256::digest(bytes).iter().map(|b| format!("{b:02x}")).collect();
    if !valid_hash(hash) || actual != hash {
        return Err("Hub content failed its SHA-256 integrity check.".into());
    }
    Ok(())
}

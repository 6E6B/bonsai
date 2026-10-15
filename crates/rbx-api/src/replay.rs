//! Record-and-replay test harness.
//!
//! Fixtures live in `crates/rbx-api/tests/fixtures/*.json`:
//!
//! ```json
//! { "captured": "2026-10-12", "source": "live", "note": "...",
//!   "interactions": [
//!     { "request": { "method": "GET", "host": "users.roblox.com", "path": "/v1/users/1",
//!                    "query": { "limit": "10" }, "headers": { "x-csrf-token": "abc" },
//!                    "absent_headers": ["x-csrf-token"] },
//!       "response": { "status": 200, "headers": { "x-csrf-token": "abc" }, "body": {} } } ] }
//! ```
//!
//! Interactions replay in order; each answers once. A [`Recorder`] attached to a live client
//! writes the same format, with identifying values scrubbed.

use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

use crate::rt::Request;
use crate::transport::{AccountId, Base, Client, ClientBuilder, Response, RetryPolicy};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Fixture {
    pub captured: String,
    /// `live` (recorded) or `synthetic` (hand-written for transport behaviour)
    pub source: String,
    pub note: String,
    pub interactions: Vec<Interaction>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Interaction {
    pub request: RecordedRequest,
    pub response: RecordedResponse,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct RecordedRequest {
    pub method: String,
    pub host: String,
    pub path: String,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub query: BTreeMap<String, String>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub headers: BTreeMap<String, String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub absent_headers: Vec<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct RecordedResponse {
    pub status: u16,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub headers: BTreeMap<String, String>,
    pub body: Value,
}

pub fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

#[allow(clippy::panic)]
pub fn load(name: &str) -> Fixture {
    let p = fixtures_dir().join(format!("{name}.json"));
    let text = std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("fixture {}: {e}", p.display()));
    serde_json::from_str(&text).unwrap_or_else(|e| panic!("fixture {}: {e}", p.display()))
}

struct Absent(String);
impl wiremock::Match for Absent {
    fn matches(&self, r: &wiremock::Request) -> bool {
        !r.headers.contains_key(self.0.as_str())
    }
}

/// A running mock server serving one or more fixtures.
pub struct Replay {
    pub server: MockServer,
}

impl Replay {
    pub async fn start(names: &[&str]) -> Replay {
        let server = MockServer::start().await;
        for name in names {
            for i in load(name).interactions {
                mount(&server, &i).await;
            }
        }
        Replay { server }
    }

    pub async fn from_fixture(f: &Fixture) -> Replay {
        let server = MockServer::start().await;
        for i in &f.interactions {
            mount(&server, i).await;
        }
        Replay { server }
    }

    /// A client builder pointed at this server, with fast retries.
    pub fn builder(&self, account: AccountId) -> ClientBuilder {
        Client::builder(account)
            .base(Base::Replay(self.server.uri()))
            .retry(RetryPolicy {
                base_delay: std::time::Duration::from_millis(5),
                max_delay: std::time::Duration::from_secs(2),
                ..Default::default()
            })
    }

    #[allow(clippy::expect_used)] // test harness
    pub fn client(&self) -> Client {
        self.builder(AccountId(1))
            .cookie(crate::Secret::new("replay-cookie"))
            .build()
            .expect("client")
    }

    pub async fn requests(&self) -> Vec<wiremock::Request> {
        self.server.received_requests().await.unwrap_or_default()
    }
}

/// A recorded query value that matches anything.
pub const ANY: &str = "*";
/// Query parameters that are random per run and saved as [`ANY`].
const RANDOM_QUERY: &[&str] = &["sessionId", "fromUtc"];

struct HasQuery(String);

impl wiremock::Match for HasQuery {
    fn matches(&self, req: &wiremock::Request) -> bool {
        req.url.query_pairs().any(|(k, _)| k == self.0.as_str())
    }
}

async fn mount(server: &MockServer, i: &Interaction) {
    let r = &i.request;
    // Like Roblox: a write without a CSRF token gets 403 plus a token, whatever order
    // concurrent requests arrive in. Lower priority than recorded interactions.
    if r.headers.keys().any(|k| k.eq_ignore_ascii_case("x-csrf-token")) {
        Mock::given(method(r.method.as_str()))
            .and(path(format!("/{}{}", r.host, r.path)))
            .and(Absent("x-csrf-token".into()))
            .respond_with(
                ResponseTemplate::new(403)
                    .insert_header("x-csrf-token", CSRF_PLACEHOLDER)
                    .set_body_json(
                        serde_json::json!({"errors": [{"code": 0, "message": "Token Validation Failed"}]}),
                    ),
            )
            .with_priority(10)
            .mount(server)
            .await;
    }
    let mut m = Mock::given(method(r.method.as_str())).and(path(format!("/{}{}", r.host, r.path)));
    for (k, v) in &r.query {
        if v == ANY {
            m = m.and(HasQuery(k.clone()));
        } else {
            m = m.and(query_param(k.as_str(), v.as_str()));
        }
    }
    for (k, v) in &r.headers {
        m = m.and(header(k.as_str(), v.as_str()));
    }
    for h in &r.absent_headers {
        m = m.and(Absent(h.to_lowercase()));
    }
    let mut t = ResponseTemplate::new(i.response.status);
    for (k, v) in &i.response.headers {
        t = t.insert_header(k.as_str(), v.as_str());
    }
    let json = i
        .response
        .headers
        .iter()
        .any(|(k, v)| k.eq_ignore_ascii_case("content-type") && v.contains("json"));
    match &i.response.body {
        Value::Null => {}
        // A JSON response whose body is a bare string, like `"Successfully blocked ..."`.
        Value::String(s) if !json => t = t.set_body_string(s.clone()),
        b => t = t.set_body_json(b),
    }
    m.respond_with(t).up_to_n_times(1).mount(server).await;
}

const CSRF_PLACEHOLDER: &str = "csrf-token-redacted";

/// Response headers worth keeping in a fixture.
const KEEP_HEADERS: &[&str] = &[
    "content-type",
    "retry-after",
    "rblx-challenge-id",
    "rblx-challenge-type",
    "rblx-challenge-metadata",
];

/// Captures live traffic for fixtures. Attach with [`Recorder::attach`].
#[derive(Default)]
pub struct Recorder {
    items: Mutex<Vec<Interaction>>,
}

impl Recorder {
    pub fn attach(self: &Arc<Self>, b: ClientBuilder) -> ClientBuilder {
        let me = self.clone();
        b.on_response(move |req: &Request, resp: &Response, csrf: bool| me.push(req, resp, csrf))
    }

    fn push(&self, req: &Request, resp: &Response, csrf_sent: bool) {
        let mut headers = BTreeMap::new();
        for k in KEEP_HEADERS {
            if let Some(v) = resp.header(k) {
                headers.insert((*k).to_string(), v.to_string());
            }
        }
        // The token is a secret: keep only the fact that one was issued and sent.
        if resp.header("x-csrf-token").is_some() {
            headers.insert("x-csrf-token".into(), CSRF_PLACEHOLDER.into());
        }
        if resp.header("rbx-authentication-ticket").is_some() {
            headers.insert("rbx-authentication-ticket".into(), "ticket-redacted".into());
        }
        let mut req_headers = BTreeMap::new();
        let mut absent = Vec::new();
        if req.method != crate::rt::Method::GET {
            if csrf_sent {
                req_headers.insert("x-csrf-token".to_string(), CSRF_PLACEHOLDER.to_string());
            } else {
                absent.push("x-csrf-token".to_string());
            }
        }
        let body = serde_json::from_slice(&resp.body)
            .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&resp.body).into_owned()));
        self.items.lock().push(Interaction {
            request: RecordedRequest {
                method: req.method.as_str().to_string(),
                host: req.host.to_string(),
                path: req.path.clone(),
                query: req.query.iter().cloned().collect(),
                headers: req_headers,
                absent_headers: absent,
            },
            response: RecordedResponse {
                status: resp.status,
                headers,
                body,
            },
        });
    }

    /// Forget what was recorded so far (setup calls that should not be in the fixture).
    pub fn clear(&self) {
        self.items.lock().clear();
    }

    /// Write a fixture, scrubbing identifying data first.
    pub fn save(&self, name: &str, note: &str, scrub: &Scrubber) {
        let mut interactions = self.items.lock().clone();
        for i in &mut interactions {
            scrub.apply(&mut i.response.body);
            for (k, v) in i.request.query.iter_mut() {
                *v = if RANDOM_QUERY.contains(&k.as_str()) {
                    ANY.to_string()
                } else {
                    scrub.text(v)
                };
            }
            i.request.path = scrub.text(&i.request.path);
        }
        let f = Fixture {
            captured: today(),
            source: "live".into(),
            note: note.into(),
            interactions,
        };
        let p = fixtures_dir().join(format!("{name}.json"));
        let mut text = serde_json::to_string_pretty(&f).unwrap_or_default();
        text.push('\n');
        std::fs::write(&p, text).unwrap_or_else(|e| panic!("writing {}: {e}", p.display()));
        self.items.lock().clear();
    }
}

fn today() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    // Civil date from days since epoch (Howard Hinnant's algorithm).
    let z = (secs / 86_400) as i64 + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    format!("{y:04}-{m:02}-{d:02}")
}

/// Replaces anything that could identify a real person: user ids and names in user-shaped
/// objects, player tokens, and any exact strings you register (the test account's name).
#[derive(Default)]
pub struct Scrubber {
    /// Every user id seen and its fake.
    ids: Mutex<HashMap<i64, i64>>,
    /// Ids registered with [`Scrubber::map_id`]; also replaced inside strings and paths.
    mapped: Mutex<HashMap<i64, i64>>,
    names: Mutex<HashMap<String, String>>,
    /// ids that are public and fine to keep (e.g. Roblox's own account 1, game ids)
    keep_ids: Vec<i64>,
    exact: Vec<(String, String)>,
}

const USER_NAME_KEYS: &[&str] = &[
    "name",
    "displayName",
    "username",
    "Username",
    "DisplayName",
    "Name",
    "senderName",
];
const USER_ID_KEYS: &[&str] = &[
    "id",
    "userId",
    "Id",
    "UserId",
    "targetId",
    "senderId",
    "builderId",
    "sellerId",
];

impl Scrubber {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn keep_ids(mut self, ids: &[i64]) -> Self {
        self.keep_ids.extend_from_slice(ids);
        self
    }
    /// Replace this exact string everywhere (case-sensitive).
    pub fn replace(mut self, from: impl Into<String>, to: impl Into<String>) -> Self {
        let from = from.into();
        if !from.is_empty() {
            self.exact.push((from, to.into()));
        }
        self
    }
    /// Map this real user id to a fixed fake one everywhere, including inside strings.
    pub fn map_id(self, real: i64, fake: i64) -> Self {
        self.alias(real, fake);
        self
    }

    /// Like [`Scrubber::map_id`] for an id found while recording.
    pub fn alias(&self, real: i64, fake: i64) {
        self.ids.lock().insert(real, fake);
        self.mapped.lock().insert(real, fake);
    }

    fn fake_id(&self, id: i64) -> i64 {
        if self.keep_ids.contains(&id) {
            return id;
        }
        let mut m = self.ids.lock();
        let n = m.len() as i64;
        *m.entry(id).or_insert(9_000_000_000 + n + 1)
    }

    fn fake_name(&self, s: &str) -> String {
        if let Some((_, to)) = self.exact.iter().find(|(from, _)| from == s) {
            return to.clone();
        }
        let mut m = self.names.lock();
        let n = m.len() + 1;
        m.entry(s.to_string())
            .or_insert_with(|| format!("user{n}"))
            .clone()
    }

    pub fn text(&self, s: &str) -> String {
        // Avatar image URLs identify the avatar; keep the shape, drop the hash.
        if let Some(rest) = s.strip_prefix("https://tr.rbxcdn.com/")
            && let Some((first, tail)) = rest.split_once('/')
            && first.contains("Avatar")
        {
            let kind = first.split('-').nth(1).unwrap_or("Avatar");
            return format!("https://tr.rbxcdn.com/30DAY-{kind}-redacted/{tail}");
        }
        if let Some(c) = self.cursor(s) {
            return c;
        }
        self.plain_text(s)
    }

    fn plain_text(&self, s: &str) -> String {
        let mut out = s.to_string();
        for (a, b) in &self.exact {
            out = out.replace(a, b);
        }
        for (real, fake) in self.mapped.lock().iter() {
            out = out.replace(&real.to_string(), &fake.to_string());
        }
        out
    }

    /// Some cursors are base64 text that names the account (`userId:...`). Scrub inside and
    /// encode again, so the recorded cursor and the request that sends it back still match.
    fn cursor(&self, s: &str) -> Option<String> {
        use base64::Engine;
        use base64::engine::general_purpose::{STANDARD, STANDARD_NO_PAD};
        if s.len() < 24 || s.contains(|c: char| c.is_whitespace() || c == '-' || c == '_' || c == '.') {
            return None;
        }
        let (bytes, engine) = match STANDARD.decode(s) {
            Ok(b) => (b, STANDARD),
            Err(_) => (STANDARD_NO_PAD.decode(s).ok()?, STANDARD_NO_PAD),
        };
        let text = String::from_utf8(bytes).ok()?;
        let scrubbed = self.plain_text(&text);
        (scrubbed != text).then(|| engine.encode(scrubbed))
    }

    pub fn apply(&self, v: &mut Value) {
        match v {
            Value::Object(m) => {
                // Friend and follower lists: bare `{"id": n}` (maybe with hasVerifiedBadge).
                let bare_user =
                    m.contains_key("id") && m.keys().all(|k| matches!(k.as_str(), "id" | "hasVerifiedBadge"));
                // Groups, badges, items and games also have `name` + `displayName` or
                // `hasVerifiedBadge`; these keys never appear on users.
                let group_like = [
                    "memberCount",
                    "isBuildersClubOnly",
                    "publicEntryAllowed",
                    "shout",
                    "statistics",
                    "awarder",
                    "iconImageId",
                    "assetType",
                    "itemType",
                    "price",
                    "universeId",
                    "categoryType",
                ]
                .iter()
                .any(|k| m.contains_key(*k));
                // A user always has a numeric id; a group creator says so.
                let has_id = USER_ID_KEYS.iter().any(|k| m.get(*k).is_some_and(Value::is_i64));
                let group_typed = ["type", "creatorType", "sellerType"]
                    .iter()
                    .any(|k| m.get(*k).and_then(Value::as_str) == Some("Group"));
                // Transaction agents: `{"id", "type": "User", "name"}`.
                let user_typed =
                    m.get("type").and_then(Value::as_str) == Some("User") && m.contains_key("name");
                let user_like = has_id
                    && !group_like
                    && !group_typed
                    && (bare_user
                        || user_typed
                        || m.contains_key("displayName")
                        || m.contains_key("username")
                        || m.contains_key("hasVerifiedBadge")
                            && m.contains_key("name")
                            && !m.contains_key("rootPlaceId"));
                let kept = USER_ID_KEYS
                    .iter()
                    .filter_map(|k| m.get(*k).and_then(Value::as_i64))
                    .any(|id| self.keep_ids.contains(&id));
                if user_like && !kept {
                    for k in USER_ID_KEYS {
                        if let Some(Value::Number(n)) = m.get(*k)
                            && let Some(id) = n.as_i64()
                        {
                            m.insert((*k).to_string(), Value::from(self.fake_id(id)));
                        }
                    }
                    for k in USER_NAME_KEYS {
                        if let Some(Value::String(s)) = m.get(*k) {
                            let f = self.fake_name(s);
                            m.insert((*k).to_string(), Value::String(f));
                        }
                    }
                }
                // Catalog items name their creator inline; UGC creators are people.
                if m.get("creatorType").and_then(Value::as_str) == Some("User")
                    && let Some(id) = m.get("creatorTargetId").and_then(Value::as_i64)
                    && !self.keep_ids.contains(&id)
                {
                    m.insert("creatorTargetId".into(), Value::from(self.fake_id(id)));
                    if let Some(Value::String(n)) = m.get("creatorName") {
                        let f = self.fake_name(n);
                        m.insert("creatorName".into(), Value::String(f));
                    }
                }
                if let Some(Value::Array(toks)) = m.get_mut("playerTokens") {
                    for (i, t) in toks.iter_mut().enumerate() {
                        *t = Value::String(format!("token{i}"));
                    }
                }
                for (k, val) in m.iter_mut() {
                    if matches!(k.as_str(), "userId" | "senderId" | "targetId")
                        && let Some(id) = val.as_i64()
                    {
                        *val = Value::from(self.fake_id(id));
                    }
                    self.apply(val);
                }
            }
            Value::Array(a) => a.iter_mut().for_each(|v| self.apply(v)),
            Value::String(s) => *s = self.text(s),
            Value::Number(n) => {
                if let Some(id) = n.as_i64()
                    && let Some(f) = self.mapped.lock().get(&id)
                {
                    *v = Value::from(*f);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn scrubs_sellers_and_ugc_creators_but_not_roblox() {
        let s = Scrubber::new().keep_ids(&[1]);
        let mut v = json!({"data": [
            {"price": 5, "seller": {"sellerId": 4700575552_i64, "sellerType": "User", "name": "someone", "hasVerifiedBadge": false}},
            {"price": 6, "seller": {"sellerId": 77, "sellerType": "Group", "name": "Some Group", "hasVerifiedBadge": false}},
            {"itemType": "Asset", "creatorType": "User", "creatorTargetId": 123, "creatorName": "ugcperson"},
            {"itemType": "Asset", "creatorType": "User", "creatorTargetId": 1, "creatorName": "Roblox"},
            {"categoryType": "Hat", "displayName": "Head", "id": 8, "name": "Head", "type": "AssetType"}
        ]});
        s.apply(&mut v);
        let d = &v["data"];
        assert_ne!(d[0]["seller"]["name"], "someone");
        assert_ne!(d[0]["seller"]["sellerId"], 4700575552_i64);
        assert_eq!(d[1]["seller"]["name"], "Some Group");
        assert_ne!(d[2]["creatorName"], "ugcperson");
        assert_ne!(d[2]["creatorTargetId"], 123);
        assert_eq!(d[3]["creatorName"], "Roblox");
        assert_eq!(d[4]["name"], "Head", "inventory categories aren't users");
        assert_eq!(d[4]["id"], 8);
    }

    #[test]
    fn scrubs_transaction_agents_and_cursors_naming_the_account() {
        use base64::Engine;
        use base64::engine::general_purpose::STANDARD;
        let s = Scrubber::new().keep_ids(&[1]).map_id(111, 1_000_001);
        let cursor = STANDARD.encode("{\"discriminator\":\"userId:111transactionType:Purchase\"}");
        let mut v = json!({"nextPageCursor": cursor, "data": [
            {"agent": {"id": 4700575552_i64, "type": "User", "name": "someone"}},
            {"agent": {"id": 77, "type": "Group", "name": "Some Group"}},
            {"agent": {"id": 1, "type": "User", "name": "Roblox"}}
        ]});
        s.apply(&mut v);
        let d = &v["data"];
        assert_ne!(d[0]["agent"]["name"], "someone");
        assert_ne!(d[0]["agent"]["id"], 4700575552_i64);
        assert_eq!(d[1]["agent"]["name"], "Some Group");
        assert_eq!(d[2]["agent"]["name"], "Roblox");
        let c = v["nextPageCursor"].as_str().unwrap();
        let text = String::from_utf8(STANDARD.decode(c).unwrap()).unwrap();
        assert!(text.contains("userId:1000001") && !text.contains("userId:111t"));
        assert_eq!(
            s.text(&cursor),
            c,
            "the request sending it back scrubs the same way"
        );
    }
}

//! Runtime pieces the generated code builds on: request description, parameter encoding and
//! the tolerant enum macro.

use serde::Serialize;

pub use crate::error::{Error, Result};
pub use crate::transport::{Client, Response};

/// HTTP method of an operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

impl Method {
    /// Safe to repeat after a server error.
    pub fn idempotent(self) -> bool {
        matches!(self, Method::GET | Method::PUT | Method::DELETE)
    }
    pub fn as_str(self) -> &'static str {
        match self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::PATCH => "PATCH",
            Method::DELETE => "DELETE",
        }
    }
}

/// A file field in a multipart form.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FilePart {
    pub file_name: String,
    pub mime: String,
    pub bytes: Vec<u8>,
}

#[derive(Clone, Debug, Default)]
pub enum Body {
    #[default]
    None,
    Json(Vec<u8>),
    Form(Vec<(String, String)>),
    Multipart {
        text: Vec<(String, String)>,
        files: Vec<(String, FilePart)>,
    },
}

/// Answer to a challenge, attached to the retried request.
#[derive(Clone)]
pub struct ChallengeAnswer {
    pub id: String,
    pub kind: String,
    /// base64 JSON, as returned by the challenge continue step.
    pub metadata: String,
}

impl std::fmt::Debug for ChallengeAnswer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChallengeAnswer")
            .field("kind", &self.kind)
            .finish_non_exhaustive()
    }
}

/// A fully described request. Built by generated `*_request` functions; sent by [`Client::send`].
#[derive(Clone, Debug)]
pub struct Request {
    pub method: Method,
    pub host: &'static str,
    pub path: String,
    pub query: Vec<(String, String)>,
    pub headers: Vec<(String, String)>,
    pub body: Body,
    pub challenge: Option<ChallengeAnswer>,
}

impl Request {
    pub fn new(method: Method, host: &'static str, path: String) -> Self {
        Self {
            method,
            host,
            path,
            query: Vec::new(),
            headers: Vec::new(),
            body: Body::None,
            challenge: None,
        }
    }

    pub fn query(&mut self, key: &str, v: &impl ParamValue) {
        let mut vals = Vec::new();
        v.param_values(&mut vals);
        for s in vals {
            self.query.push((key.to_string(), s));
        }
    }

    pub fn header(&mut self, key: &str, v: &impl ParamValue) {
        let mut vals = Vec::new();
        v.param_values(&mut vals);
        if !vals.is_empty() {
            self.headers.push((key.to_string(), vals.join(",")));
        }
    }

    pub fn form_text(&mut self, key: &str, v: &impl ParamValue) {
        let mut vals = Vec::new();
        v.param_values(&mut vals);
        match &mut self.body {
            Body::Form(f) | Body::Multipart { text: f, .. } => {
                f.extend(vals.into_iter().map(|s| (key.to_string(), s)))
            }
            _ => self.body = Body::Form(vals.into_iter().map(|s| (key.to_string(), s)).collect()),
        }
    }

    pub fn form_file(&mut self, key: &str, v: &impl FileValue) {
        let Some(file) = v.file() else { return };
        let (text, mut files) = match std::mem::take(&mut self.body) {
            Body::Form(t) => (t, Vec::new()),
            Body::Multipart { text, files } => (text, files),
            _ => (Vec::new(), Vec::new()),
        };
        files.push((key.to_string(), file.clone()));
        self.body = Body::Multipart { text, files };
    }

    pub fn json(&mut self, body: &impl Serialize) -> Result<()> {
        self.body = Body::Json(serde_json::to_vec(body).map_err(|e| Error::Encode(e.to_string()))?);
        Ok(())
    }

    /// Attach a solved challenge (two-step, etc.) for the retry.
    pub fn with_challenge(mut self, answer: ChallengeAnswer) -> Self {
        self.challenge = Some(answer);
        self
    }

    /// Absolute URL on the production site, with the query string. For links and launch URIs.
    pub fn url(&self) -> String {
        let mut u = format!("https://{}{}", self.host, self.path);
        if !self.query.is_empty() {
            u.push('?');
            u.push_str(&encode_query(&self.query));
        }
        u
    }
}

pub fn encode_query(q: &[(String, String)]) -> String {
    q.iter()
        .map(|(k, v)| format!("{}={}", encode(k), encode(v)))
        .collect::<Vec<_>>()
        .join("&")
}

/// Percent-encode everything except RFC 3986 unreserved characters.
pub fn encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~') {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

/// Values that can go in a query string, header or form field.
pub trait ParamValue {
    fn param_values(&self, out: &mut Vec<String>);
}

macro_rules! display_param {
    ($($t:ty),*) => {$(
        impl ParamValue for $t {
            fn param_values(&self, out: &mut Vec<String>) {
                out.push(self.to_string());
            }
        }
        impl PathValue for $t {
            fn path_string(&self) -> String {
                encode(&self.to_string())
            }
        }
    )*};
}
display_param!(i32, i64, u32, u64, f64, bool, String, &str);

impl<T: ParamValue> ParamValue for Option<T> {
    fn param_values(&self, out: &mut Vec<String>) {
        if let Some(v) = self {
            v.param_values(out);
        }
    }
}

impl<T: ParamValue> ParamValue for Vec<T> {
    fn param_values(&self, out: &mut Vec<String>) {
        for v in self {
            v.param_values(out);
        }
    }
}

impl ParamValue for serde_json::Value {
    fn param_values(&self, out: &mut Vec<String>) {
        match self {
            serde_json::Value::Null => {}
            serde_json::Value::String(s) => out.push(s.clone()),
            v => out.push(v.to_string()),
        }
    }
}

/// Values that can fill a `{param}` in a path.
pub trait PathValue {
    fn path_string(&self) -> String;
}

pub fn path_value(v: &impl PathValue) -> String {
    v.path_string()
}

pub trait FileValue {
    fn file(&self) -> Option<&FilePart>;
}
impl FileValue for FilePart {
    fn file(&self) -> Option<&FilePart> {
        Some(self)
    }
}
impl FileValue for Option<FilePart> {
    fn file(&self) -> Option<&FilePart> {
        self.as_ref()
    }
}

/// Wire form used while deserializing enums: Roblox sends either the number or the name.
#[doc(hidden)]
#[derive(serde::Deserialize)]
#[serde(untagged)]
pub enum RawEnum {
    Int(i64),
    Float(f64),
    Str(String),
    Other(serde::de::IgnoredAny),
}

/// Generate an enum that never fails to deserialize: unknown numbers or names become `Unknown`.
///
/// `int` enums serialize as their number, `str` enums as their name. Both accept either form,
/// because Roblox services disagree about which one they send. `Unknown` serializes as `null`.
#[macro_export]
macro_rules! api_enum {
    ($kind:ident $(#[$m:meta])* $name:ident { $($v:ident = $n:literal => $s:literal),* $(,)? }) => {
        $(#[$m])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
        pub enum $name {
            $($v,)*
            /// A value this build does not know about.
            #[default]
            Unknown,
        }

        impl $name {
            pub const ALL: &'static [$name] = &[$($name::$v),*];
            /// Numeric value on the wire.
            pub fn value(self) -> Option<i64> {
                match self {
                    $($name::$v => Some($n),)*
                    $name::Unknown => None,
                }
            }
            /// Name on the wire.
            pub fn name(self) -> Option<&'static str> {
                match self {
                    $($name::$v => Some($s),)*
                    $name::Unknown => None,
                }
            }
            pub fn from_value(n: i64) -> Self {
                match n {
                    $($n => $name::$v,)*
                    _ => $name::Unknown,
                }
            }
            pub fn from_name(s: &str) -> Self {
                $(if s.eq_ignore_ascii_case($s) { return $name::$v; })*
                match s.parse::<i64>() {
                    Ok(n) => Self::from_value(n),
                    Err(_) => $name::Unknown,
                }
            }
            fn wire(self) -> Option<String> {
                $crate::api_enum!(@wire $kind self)
            }
        }

        impl serde::Serialize for $name {
            fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
                $crate::api_enum!(@ser $kind self s)
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
                Ok(match <$crate::rt::RawEnum as serde::Deserialize>::deserialize(d)? {
                    $crate::rt::RawEnum::Int(n) => Self::from_value(n),
                    $crate::rt::RawEnum::Float(f) => Self::from_value(f as i64),
                    $crate::rt::RawEnum::Str(s) => Self::from_name(&s),
                    $crate::rt::RawEnum::Other(_) => $name::Unknown,
                })
            }
        }

        impl $crate::rt::ParamValue for $name {
            fn param_values(&self, out: &mut Vec<String>) {
                if let Some(w) = self.wire() {
                    out.push(w);
                }
            }
        }

        impl $crate::rt::PathValue for $name {
            fn path_string(&self) -> String {
                self.wire().map(|w| $crate::rt::encode(&w)).unwrap_or_default()
            }
        }
    };
    (@wire int $self:ident) => { $self.value().map(|n| n.to_string()) };
    (@wire str $self:ident) => { $self.name().map(str::to_string) };
    (@ser int $self:ident $s:ident) => {
        match $self.value() { Some(n) => $s.serialize_i64(n), None => $s.serialize_none() }
    };
    (@ser str $self:ident $s:ident) => {
        match $self.name() { Some(n) => $s.serialize_str(n), None => $s.serialize_none() }
    };
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    crate::api_enum! {
        int
        /// test
        Privacy {
            NoOne = 1 => "NoOne",
            Friends = 2 => "Friends",
        }
    }
    crate::api_enum! {
        str
        Sort {
            Asc = 0 => "Asc",
            Desc = 1 => "Desc",
        }
    }

    #[test]
    fn enums_accept_numbers_names_and_unknowns() {
        let v: Vec<Privacy> = serde_json::from_str(r#"[1, "Friends", 99, "Nope", null, 2.0]"#).unwrap();
        assert_eq!(
            v,
            vec![
                Privacy::NoOne,
                Privacy::Friends,
                Privacy::Unknown,
                Privacy::Unknown,
                Privacy::Unknown,
                Privacy::Friends
            ]
        );
        assert_eq!(serde_json::to_string(&Privacy::Friends).unwrap(), "2");
        assert_eq!(serde_json::to_string(&Sort::Desc).unwrap(), "\"Desc\"");
        assert_eq!(serde_json::to_string(&Sort::Unknown).unwrap(), "null");
        let s: Sort = serde_json::from_str("1").unwrap();
        assert_eq!(s, Sort::Desc);
    }

    #[test]
    fn request_encoding() {
        let mut r = Request::new(Method::GET, "games.roblox.com", "/v1/games".into());
        r.query("universeIds", &vec![1i64, 2]);
        r.query("cursor", &None::<String>);
        r.query("q", &Some("a b&c".to_string()));
        r.query("sort", &Some(Sort::Asc));
        assert_eq!(
            r.url(),
            "https://games.roblox.com/v1/games?universeIds=1&universeIds=2&q=a%20b%26c&sort=Asc"
        );
        assert_eq!(path_value(&"a/b"), "a%2Fb");
        assert_eq!(path_value(&Privacy::Friends), "2");
    }
}

/// Decode an optional field, treating a value of the wrong type as missing. Used for integer
/// enums the spec gives no names for, which Roblox sometimes sends as strings.
pub fn lenient<'de, D, T>(d: D) -> std::result::Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::de::DeserializeOwned,
{
    let v = <serde_json::Value as serde::Deserialize>::deserialize(d)?;
    Ok(serde_json::from_value(v).ok())
}

#[cfg(test)]
mod lenient_tests {
    #[derive(serde::Deserialize, Default)]
    #[serde(default)]
    struct S {
        #[serde(deserialize_with = "super::lenient")]
        kind: Option<i32>,
    }

    #[test]
    fn wrong_type_is_none() {
        let s: S = serde_json::from_str(r#"{"kind":"User"}"#).unwrap_or_default();
        assert_eq!(s.kind, None);
        let s: S = serde_json::from_str(r#"{"kind":2}"#).unwrap_or_default();
        assert_eq!(s.kind, Some(2));
        let s: S = serde_json::from_str(r#"{}"#).unwrap_or_default();
        assert_eq!(s.kind, None);
    }
}

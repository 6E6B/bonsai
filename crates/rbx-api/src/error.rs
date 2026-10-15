//! Errors from the API layer.

use serde::Deserialize;

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// One entry of Roblox's `{"errors":[{"code":..,"message":..}]}` body.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct ApiError {
    pub code: i64,
    pub message: String,
    #[serde(rename = "userFacingMessage")]
    pub user_facing_message: Option<String>,
}

/// The server wants the user to pass a challenge (captcha, two-step, etc.) before it will act.
/// The UI must show it; it is never solved automatically.
#[derive(Clone, PartialEq, Eq)]
pub struct Challenge {
    pub id: String,
    /// e.g. `twostepverification`, `captcha`, `reauthentication`, `forcetwostepverification`
    pub kind: String,
    /// Decoded `rblx-challenge-metadata` (base64 JSON), if it decoded.
    pub metadata: serde_json::Value,
    /// The raw base64 metadata, needed to continue the challenge.
    pub metadata_raw: String,
}

impl std::fmt::Debug for Challenge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Challenge")
            .field("kind", &self.kind)
            .finish_non_exhaustive()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("network error: {0}")]
    Network(String),
    /// 401: the cookie is missing, expired or revoked.
    #[error("session expired or not signed in")]
    Unauthorized,
    #[error("challenge required: {}", .0.kind)]
    Challenge(Box<Challenge>),
    #[error("rate limited by Roblox, retry in {retry_after_secs} s")]
    RateLimited { retry_after_secs: u64 },
    #[error("{status}: {}", summary(.errors, .body))]
    Status {
        status: u16,
        errors: Vec<ApiError>,
        body: String,
    },
    #[error("unexpected response: {message}")]
    Decode { message: String, body: String },
    #[error("cannot encode request: {0}")]
    Encode(String),
    /// A plain message for the user, not from an HTTP status (e.g. "This account is not signed in").
    #[error("{0}")]
    Message(String),
}

fn summary(errors: &[ApiError], body: &str) -> String {
    match errors.first() {
        Some(e) => e
            .user_facing_message
            .clone()
            .filter(|m| !m.is_empty())
            .unwrap_or_else(|| e.message.clone()),
        None => body.chars().take(200).collect(),
    }
}

impl Error {
    pub fn status(&self) -> Option<u16> {
        match self {
            Error::Status { status, .. } => Some(*status),
            Error::Unauthorized => Some(401),
            Error::RateLimited { .. } => Some(429),
            _ => None,
        }
    }

    /// First Roblox error code in the body, if any.
    pub fn code(&self) -> Option<i64> {
        match self {
            Error::Status { errors, .. } => errors.first().map(|e| e.code),
            _ => None,
        }
    }

    /// Plain message for the UI.
    pub fn user_message(&self) -> String {
        match self {
            Error::Network(_) => "Can't reach Roblox. Check your connection.".into(),
            Error::Unauthorized => "Session expired. Paste a new cookie for this account.".into(),
            Error::Challenge(c) => format!("Roblox wants you to confirm this ({}).", c.kind),
            Error::RateLimited { retry_after_secs } => {
                format!("Too many requests. Try again in {retry_after_secs} s.")
            }
            Error::Status { errors, body, .. } => summary(errors, body),
            Error::Decode { .. } => "Roblox sent something unexpected.".into(),
            Error::Encode(e) => e.clone(),
            Error::Message(m) => m.clone(),
        }
    }
}

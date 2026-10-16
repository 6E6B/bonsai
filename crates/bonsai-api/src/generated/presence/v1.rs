//! `presence.roblox.com` v1 — generated from `vendor/roblox-api-tracker/presence/v1.json`.
//!
//! Do not edit. Change the spec or `bonsai-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "presence.roblox.com";

/// `Error`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ErrorModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// `ErrorResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ErrorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ErrorModel>>,
}

/// `PresenceType`
pub type PresenceType = i32;

/// `UserPresence`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UserPresence {
    #[serde(rename = "gameId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_id: Option<String>,
    #[serde(rename = "lastLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_location: Option<String>,
    #[serde(rename = "placeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_id: Option<i64>,
    #[serde(rename = "rootPlaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_place_id: Option<i64>,
    #[serde(rename = "universeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_id: Option<i64>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(rename = "userPresenceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_presence_type: Option<PresenceType>,
}

/// `UserPresenceRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UserPresenceRequest {
    #[serde(rename = "userIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<i64>>,
}

/// `UserPresencesResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UserPresencesResponse {
    #[serde(rename = "userPresences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_presences: Option<Vec<UserPresence>>,
}

/// `POST /v1/presence/users`
pub fn post_presence_users_request(body: &UserPresenceRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/presence/users".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/presence/users`
pub async fn post_presence_users(
    client: &Client,
    body: &UserPresenceRequest,
) -> Result<UserPresencesResponse> {
    client.send(post_presence_users_request(body)?).await?.json()
}

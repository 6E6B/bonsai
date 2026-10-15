//! `followings.roblox.com` v2 — generated from `vendor/roblox-api-tracker/followings/v2.json`.
//!
//! Do not edit. Change the spec or `rbx-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "followings.roblox.com";

/// `Roblox.Followings.Api.Models.FollowsByTypeResponse`
///
/// Data model containing collection of all followed sources of a specific type.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FollowsByTypeResponse {
    /// Followed sources: map of (source ID =\> follow date)
    #[serde(rename = "followedSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followed_sources: Option<HashMap<String, String>>,
    /// ID of the follower entity.
    #[serde(rename = "followerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follower_id: Option<i64>,
    /// Type of the follower entity.
    #[serde(rename = "followerType")]
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follower_type: Option<i32>,
    /// Type of the source entity.
    #[serde(rename = "sourceType")]
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<i32>,
}

/// `GET /v2/users/{userId}/universes`
///
/// Gets all universes followed by a user.
pub fn get_users_by_user_id_universes_request(user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v2/users/{}/universes", crate::rt::path_value(&user_id)),
    );
    Ok(r)
}

/// `GET /v2/users/{userId}/universes`
///
/// Gets all universes followed by a user.
pub async fn get_users_by_user_id_universes(client: &Client, user_id: i64) -> Result<FollowsByTypeResponse> {
    client
        .send(get_users_by_user_id_universes_request(user_id)?)
        .await?
        .json()
}

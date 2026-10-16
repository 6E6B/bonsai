//! `followings.roblox.com` v1 — generated from `vendor/roblox-api-tracker/followings/v1.json`.
//!
//! Do not edit. Change the spec or `bonsai-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "followings.roblox.com";

/// `Roblox.Followings.Api.Models.UserFollowingUniverseResponse`
///
/// Model for a user following a universe controller responses
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UserFollowingUniverseResponse {
    /// The id of the universe being followed
    #[serde(rename = "universeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_id: Option<i64>,
    /// The id of the user that is following
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}

/// `Roblox.Followings.Api.Models.UserFollowingUniverseStatusResponse`
///
/// Model for a user following a universe state controller responses
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UserFollowingUniverseStatusResponse {
    /// If the user can follow the universe.
    #[serde(rename = "CanFollow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_follow: Option<bool>,
    /// The number of followings between this user and a universe.
    #[serde(rename = "FollowingCountByType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub following_count_by_type: Option<i32>,
    /// The limit to the number of followings between a user and a universe for a specific user.
    #[serde(rename = "FollowingLimitByType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub following_limit_by_type: Option<i32>,
    /// If the user is currently following the universe.
    #[serde(rename = "IsFollowing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_following: Option<bool>,
    /// The id of the universe.
    #[serde(rename = "UniverseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_id: Option<i64>,
    /// The id of the user.
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}

/// `GET /v1/users/{userId}/universes`
///
/// Gets all the followings between a user with userId and universes
pub fn get_users_by_user_id_universes_request(user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/users/{}/universes", crate::rt::path_value(&user_id)),
    );
    Ok(r)
}

/// `GET /v1/users/{userId}/universes`
///
/// Gets all the followings between a user with userId and universes
pub async fn get_users_by_user_id_universes(
    client: &Client,
    user_id: i64,
) -> Result<Vec<UserFollowingUniverseResponse>> {
    client
        .send(get_users_by_user_id_universes_request(user_id)?)
        .await?
        .json()
}

/// `POST /v1/users/{userId}/universes/{universeId}`
///
/// Creates the following between a user with userId and universe with universeId
pub fn post_users_by_user_id_universes_by_universe_id_request(
    user_id: i64,
    universe_id: i64,
) -> Result<Request> {
    let r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/users/{}/universes/{}",
            crate::rt::path_value(&user_id),
            crate::rt::path_value(&universe_id)
        ),
    );
    Ok(r)
}

/// `POST /v1/users/{userId}/universes/{universeId}`
///
/// Creates the following between a user with userId and universe with universeId
pub async fn post_users_by_user_id_universes_by_universe_id(
    client: &Client,
    user_id: i64,
    universe_id: i64,
) -> Result<UserFollowingUniverseResponse> {
    client
        .send(post_users_by_user_id_universes_by_universe_id_request(
            user_id,
            universe_id,
        )?)
        .await?
        .json()
}

/// `DELETE /v1/users/{userId}/universes/{universeId}`
///
/// Deletes the following between a user with userId and universe with universeId
pub fn delete_users_by_user_id_universes_by_universe_id_request(
    user_id: i64,
    universe_id: i64,
) -> Result<Request> {
    let r = Request::new(
        Method::DELETE,
        HOST,
        format!(
            "/v1/users/{}/universes/{}",
            crate::rt::path_value(&user_id),
            crate::rt::path_value(&universe_id)
        ),
    );
    Ok(r)
}

/// `DELETE /v1/users/{userId}/universes/{universeId}`
///
/// Deletes the following between a user with userId and universe with universeId
pub async fn delete_users_by_user_id_universes_by_universe_id(
    client: &Client,
    user_id: i64,
    universe_id: i64,
) -> Result<UserFollowingUniverseResponse> {
    client
        .send(delete_users_by_user_id_universes_by_universe_id_request(
            user_id,
            universe_id,
        )?)
        .await?
        .json()
}

/// `GET /v1/users/{userId}/universes/{universeId}/status`
///
/// Gets the status of a following relationship between a user and a universe.
pub fn get_users_by_user_id_universes_by_universe_id_status_request(
    user_id: i64,
    universe_id: i64,
) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/users/{}/universes/{}/status",
            crate::rt::path_value(&user_id),
            crate::rt::path_value(&universe_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/users/{userId}/universes/{universeId}/status`
///
/// Gets the status of a following relationship between a user and a universe.
pub async fn get_users_by_user_id_universes_by_universe_id_status(
    client: &Client,
    user_id: i64,
    universe_id: i64,
) -> Result<UserFollowingUniverseStatusResponse> {
    client
        .send(get_users_by_user_id_universes_by_universe_id_status_request(
            user_id,
            universe_id,
        )?)
        .await?
        .json()
}

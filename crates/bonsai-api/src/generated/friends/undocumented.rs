//! `friends.roblox.com` undocumented — generated from `specs/undocumented/friends.roblox.com.yaml`.
//!
//! Do not edit. Change the spec or `bonsai-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "friends.roblox.com";

/// `Empty`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Empty {}

/// `FriendshipRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FriendshipRequest {
    /// UserProfile, PlayerSearch, ...
    #[serde(rename = "friendshipOriginSourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friendship_origin_source_type: Option<String>,
}

/// `FriendshipResult`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FriendshipResult {
    #[serde(rename = "isCaptchaRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_captcha_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

/// `TrustedFriendStatus`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TrustedFriendStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// `GET /v1/my/trusted-friends/{targetUserId}/status`
///
/// Read the signed-in account's trusted friendship with a user.
pub fn get_trusted_friend_status_request(target_user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/my/trusted-friends/{}/status",
            crate::rt::path_value(&target_user_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/my/trusted-friends/{targetUserId}/status`
///
/// Read the signed-in account's trusted friendship with a user.
pub async fn get_trusted_friend_status(client: &Client, target_user_id: i64) -> Result<TrustedFriendStatus> {
    client
        .send(get_trusted_friend_status_request(target_user_id)?)
        .await?
        .json()
}

/// `POST /v1/user/friend-requests/decline-all`
///
/// Decline every pending friend request.
///
/// Undocumented; captured 2026-10-12, www.roblox.com/users/friends#!/friend-requests.
pub fn post_decline_all_friend_requests_request() -> Result<Request> {
    let r = Request::new(
        Method::POST,
        HOST,
        "/v1/user/friend-requests/decline-all".to_string(),
    );
    Ok(r)
}

/// `POST /v1/user/friend-requests/decline-all`
///
/// Decline every pending friend request.
///
/// Undocumented; captured 2026-10-12, www.roblox.com/users/friends#!/friend-requests.
pub async fn post_decline_all_friend_requests(client: &Client) -> Result<Empty> {
    client
        .send(post_decline_all_friend_requests_request()?)
        .await?
        .json()
}

/// `POST /v1/users/{requesterUserId}/accept-friend-request`
///
/// Accept a friend request.
///
/// Undocumented; captured 2026-10-12, www.roblox.com/users/friends#!/friend-requests.
pub fn post_accept_friend_request_request(requester_user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/users/{}/accept-friend-request",
            crate::rt::path_value(&requester_user_id)
        ),
    );
    Ok(r)
}

/// `POST /v1/users/{requesterUserId}/accept-friend-request`
///
/// Accept a friend request.
///
/// Undocumented; captured 2026-10-12, www.roblox.com/users/friends#!/friend-requests.
pub async fn post_accept_friend_request(client: &Client, requester_user_id: i64) -> Result<Empty> {
    client
        .send(post_accept_friend_request_request(requester_user_id)?)
        .await?
        .json()
}

/// `POST /v1/users/{requesterUserId}/decline-friend-request`
///
/// Decline a friend request.
///
/// Undocumented; captured 2026-10-12, www.roblox.com/users/friends#!/friend-requests.
pub fn post_decline_friend_request_request(requester_user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/users/{}/decline-friend-request",
            crate::rt::path_value(&requester_user_id)
        ),
    );
    Ok(r)
}

/// `POST /v1/users/{requesterUserId}/decline-friend-request`
///
/// Decline a friend request.
///
/// Undocumented; captured 2026-10-12, www.roblox.com/users/friends#!/friend-requests.
pub async fn post_decline_friend_request(client: &Client, requester_user_id: i64) -> Result<Empty> {
    client
        .send(post_decline_friend_request_request(requester_user_id)?)
        .await?
        .json()
}

/// `POST /v1/users/{targetUserId}/accept-trusted-friend-request`
///
/// Accept an incoming trusted friend request.
pub fn post_accept_trusted_friend_request_request(target_user_id: i64, body: &Empty) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/users/{}/accept-trusted-friend-request",
            crate::rt::path_value(&target_user_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/users/{targetUserId}/accept-trusted-friend-request`
///
/// Accept an incoming trusted friend request.
pub async fn post_accept_trusted_friend_request(
    client: &Client,
    target_user_id: i64,
    body: &Empty,
) -> Result<Empty> {
    client
        .send(post_accept_trusted_friend_request_request(target_user_id, body)?)
        .await?
        .json()
}

/// `POST /v1/users/{targetUserId}/ignore-trusted-friend-request`
///
/// Decline an incoming trusted friend request, keeping the friendship.
pub fn post_ignore_trusted_friend_request_request(target_user_id: i64, body: &Empty) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/users/{}/ignore-trusted-friend-request",
            crate::rt::path_value(&target_user_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/users/{targetUserId}/ignore-trusted-friend-request`
///
/// Decline an incoming trusted friend request, keeping the friendship.
pub async fn post_ignore_trusted_friend_request(
    client: &Client,
    target_user_id: i64,
    body: &Empty,
) -> Result<Empty> {
    client
        .send(post_ignore_trusted_friend_request_request(target_user_id, body)?)
        .await?
        .json()
}

/// `POST /v1/users/{targetUserId}/remove-trusted-friend`
///
/// Remove trusted status or cancel an outgoing trusted request, keeping the friendship.
pub fn post_remove_trusted_friend_request(target_user_id: i64, body: &Empty) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/users/{}/remove-trusted-friend",
            crate::rt::path_value(&target_user_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/users/{targetUserId}/remove-trusted-friend`
///
/// Remove trusted status or cancel an outgoing trusted request, keeping the friendship.
pub async fn post_remove_trusted_friend(client: &Client, target_user_id: i64, body: &Empty) -> Result<Empty> {
    client
        .send(post_remove_trusted_friend_request(target_user_id, body)?)
        .await?
        .json()
}

/// `POST /v1/users/{targetUserId}/request-friendship`
///
/// Send a friend request (user action only).
///
/// Undocumented; captured 2026-10-12, www.roblox.com/users/{id}/profile.
pub fn post_request_friendship_request(target_user_id: i64, body: &FriendshipRequest) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/users/{}/request-friendship",
            crate::rt::path_value(&target_user_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/users/{targetUserId}/request-friendship`
///
/// Send a friend request (user action only).
///
/// Undocumented; captured 2026-10-12, www.roblox.com/users/{id}/profile.
pub async fn post_request_friendship(
    client: &Client,
    target_user_id: i64,
    body: &FriendshipRequest,
) -> Result<FriendshipResult> {
    client
        .send(post_request_friendship_request(target_user_id, body)?)
        .await?
        .json()
}

/// `POST /v1/users/{targetUserId}/send-trusted-friend-request`
///
/// Send a trusted friend request to an existing friend.
pub fn post_send_trusted_friend_request_request(target_user_id: i64, body: &Empty) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/users/{}/send-trusted-friend-request",
            crate::rt::path_value(&target_user_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/users/{targetUserId}/send-trusted-friend-request`
///
/// Send a trusted friend request to an existing friend.
pub async fn post_send_trusted_friend_request(
    client: &Client,
    target_user_id: i64,
    body: &Empty,
) -> Result<Empty> {
    client
        .send(post_send_trusted_friend_request_request(target_user_id, body)?)
        .await?
        .json()
}

/// `POST /v1/users/{targetUserId}/unfriend`
///
/// Remove a friend.
///
/// Undocumented; captured 2026-10-12, www.roblox.com/users/{id}/profile.
pub fn post_unfriend_request(target_user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::POST,
        HOST,
        format!("/v1/users/{}/unfriend", crate::rt::path_value(&target_user_id)),
    );
    Ok(r)
}

/// `POST /v1/users/{targetUserId}/unfriend`
///
/// Remove a friend.
///
/// Undocumented; captured 2026-10-12, www.roblox.com/users/{id}/profile.
pub async fn post_unfriend(client: &Client, target_user_id: i64) -> Result<Empty> {
    client.send(post_unfriend_request(target_user_id)?).await?.json()
}

//! `usermoderation.roblox.com` undocumented — generated from `specs/undocumented/usermoderation.roblox.com.yaml`.
//!
//! Do not edit. Change the spec or `bonsai-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "usermoderation.roblox.com";

/// `Empty`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Empty {}

/// `NotApproved`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct NotApproved {
    /// RFC 3339
    #[serde(rename = "beginDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_date: Option<String>,
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_: Option<String>,
    /// RFC 3339; set for bans
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "messageToUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_to_user: Option<String>,
    #[serde(rename = "punishedUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub punished_user_id: Option<i64>,
    /// "Warn", "Delete" (terminated), "Ban 7 Days", or empty
    #[serde(rename = "punishmentTypeDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub punishment_type_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// `GET /v1/not-approved`
///
/// The punishment of the signed-in account. Empty punishment type means locked until a human check.
///
/// Undocumented; captured 2026-11-09, www.roblox.com/not-approved.
pub fn get_not_approved_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/not-approved".to_string());
    Ok(r)
}

/// `GET /v1/not-approved`
///
/// The punishment of the signed-in account. Empty punishment type means locked until a human check.
///
/// Undocumented; captured 2026-11-09, www.roblox.com/not-approved.
pub async fn get_not_approved(client: &Client) -> Result<NotApproved> {
    client.send(get_not_approved_request()?).await?.json()
}

/// `POST /v1/not-approved/reactivate`
///
/// Accept a warning or an ended ban so the account works again (user action only).
///
/// Undocumented; captured 2026-11-09, www.roblox.com/not-approved.
pub fn post_reactivate_request() -> Result<Request> {
    let r = Request::new(Method::POST, HOST, "/v1/not-approved/reactivate".to_string());
    Ok(r)
}

/// `POST /v1/not-approved/reactivate`
///
/// Accept a warning or an ended ban so the account works again (user action only).
///
/// Undocumented; captured 2026-11-09, www.roblox.com/not-approved.
pub async fn post_reactivate(client: &Client) -> Result<Empty> {
    client.send(post_reactivate_request()?).await?.json()
}

//! `thumbnails.roblox.com` undocumented — generated from `specs/undocumented/thumbnails.roblox.com.yaml`.
//!
//! Do not edit. Change the spec or `bonsai-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "thumbnails.roblox.com";

/// `BatchItem`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BatchItem {
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "imageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Completed, Pending, Blocked, Error, InReview, TemporarilyUnavailable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "targetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// `BatchRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BatchRequest {
    /// webp or png
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "isCircular")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_circular: Option<bool>,
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// e.g. 48x48, 150x150, 420x420
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(rename = "targetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<i64>,
    /// AvatarHeadShot, Avatar, GameIcon, GameThumbnail, Asset, BundleThumbnail, GroupIcon, BadgeIcon, GamePass, Outfit
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// `BatchResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BatchResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<BatchItem>>,
}

/// `POST /v1/batch`
///
/// Thumbnails of mixed types in one request (up to 100).
///
/// Undocumented; captured 2026-10-12, avatar headshots and game icons on www.roblox.com/home.
pub fn post_batch_request(body: &Vec<BatchRequest>) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/batch".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/batch`
///
/// Thumbnails of mixed types in one request (up to 100).
///
/// Undocumented; captured 2026-10-12, avatar headshots and game icons on www.roblox.com/home.
pub async fn post_batch(client: &Client, body: &Vec<BatchRequest>) -> Result<BatchResponse> {
    client.send(post_batch_request(body)?).await?.json()
}

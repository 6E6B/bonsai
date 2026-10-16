//! `avatar.roblox.com` undocumented — generated from `specs/undocumented/avatar.roblox.com.yaml`.
//!
//! Do not edit. Change the spec or `rbx-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "avatar.roblox.com";

/// `Empty`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Empty {}

/// `POST /v1/emotes/{assetId}/{position}`
///
/// Put an emote the user owns in a hotbar slot (1 to 8).
///
/// Undocumented; captured 2026-10-16, www.roblox.com/my/avatar.
pub fn post_emote_equip_request(asset_id: i64, position: i32) -> Result<Request> {
    let r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/emotes/{}/{}",
            crate::rt::path_value(&asset_id),
            crate::rt::path_value(&position)
        ),
    );
    Ok(r)
}

/// `POST /v1/emotes/{assetId}/{position}`
///
/// Put an emote the user owns in a hotbar slot (1 to 8).
///
/// Undocumented; captured 2026-10-16, www.roblox.com/my/avatar.
pub async fn post_emote_equip(client: &Client, asset_id: i64, position: i32) -> Result<Empty> {
    client
        .send(post_emote_equip_request(asset_id, position)?)
        .await?
        .json()
}

/// `DELETE /v1/emotes/{position}`
///
/// Clear a hotbar slot.
///
/// Undocumented; captured 2026-10-16, www.roblox.com/my/avatar.
pub fn delete_emote_unequip_request(position: i32) -> Result<Request> {
    let r = Request::new(
        Method::DELETE,
        HOST,
        format!("/v1/emotes/{}", crate::rt::path_value(&position)),
    );
    Ok(r)
}

/// `DELETE /v1/emotes/{position}`
///
/// Clear a hotbar slot.
///
/// Undocumented; captured 2026-10-16, www.roblox.com/my/avatar.
pub async fn delete_emote_unequip(client: &Client, position: i32) -> Result<Empty> {
    client.send(delete_emote_unequip_request(position)?).await?.json()
}

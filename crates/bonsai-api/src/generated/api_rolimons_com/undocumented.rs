//! `api.rolimons.com` undocumented — generated from `specs/undocumented/api.rolimons.com.yaml`.
//!
//! Do not edit. Change the spec or `bonsai-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "api.rolimons.com";

/// `ItemValues`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ItemValues {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<HashMap<String, Vec<serde_json::Value>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

/// `PlayerInfo`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PlayerInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_online: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rap: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rolibadges: Option<HashMap<String, i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats_updated: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}

/// `GET /items/v2/itemdetails`
///
/// Every tracked limited, by asset id.
///
/// Each value is \[name, acronym, rap, value, default_value, demand, trend, projected, hyped, rare, limited_version, ...\]; -1 means none. Acronym is "" when there is none. default_value is value when set, else RAP. Demand 0-4 (terrible, low, normal, high, amazing), trend 0-4 (lowering, unstable, stable, raising, fluctuating), projected, hyped and rare are 1 when set. limited_version is 1 (Limited 1.0) or 2 (Limited 2.0).
///
/// Undocumented; captured 2026-10-13, https://api.rolimons.com/items/v2/itemdetails.
pub fn get_item_values_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/items/v2/itemdetails".to_string());
    Ok(r)
}

/// `GET /items/v2/itemdetails`
///
/// Every tracked limited, by asset id.
///
/// Each value is \[name, acronym, rap, value, default_value, demand, trend, projected, hyped, rare, limited_version, ...\]; -1 means none. Acronym is "" when there is none. default_value is value when set, else RAP. Demand 0-4 (terrible, low, normal, high, amazing), trend 0-4 (lowering, unstable, stable, raising, fluctuating), projected, hyped and rare are 1 when set. limited_version is 1 (Limited 1.0) or 2 (Limited 2.0).
///
/// Undocumented; captured 2026-10-13, https://api.rolimons.com/items/v2/itemdetails.
pub async fn get_item_values(client: &Client) -> Result<ItemValues> {
    client.send(get_item_values_request()?).await?.json()
}

/// `GET /players/v1/playerinfo/{playerId}`
///
/// A player's inventory value, RAP, rank and Rolimons badges.
///
/// Times are unix seconds. rank is null for a player without value. last_online and last_location are when and where Rolimons last saw them, not Roblox's own record. rolibadges maps a badge key (value_1m, own_1_dominus, ...) to when it was earned.
///
/// Undocumented; captured 2026-10-14, https://www.rolimons.com/player/156.
pub fn get_player_info_request(player_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/players/v1/playerinfo/{}", crate::rt::path_value(&player_id)),
    );
    Ok(r)
}

/// `GET /players/v1/playerinfo/{playerId}`
///
/// A player's inventory value, RAP, rank and Rolimons badges.
///
/// Times are unix seconds. rank is null for a player without value. last_online and last_location are when and where Rolimons last saw them, not Roblox's own record. rolibadges maps a badge key (value_1m, own_1_dominus, ...) to when it was earned.
///
/// Undocumented; captured 2026-10-14, https://www.rolimons.com/player/156.
pub async fn get_player_info(client: &Client, player_id: i64) -> Result<PlayerInfo> {
    client.send(get_player_info_request(player_id)?).await?.json()
}

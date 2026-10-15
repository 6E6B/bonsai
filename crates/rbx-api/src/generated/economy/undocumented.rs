//! `economy.roblox.com` undocumented — generated from `specs/undocumented/economy.roblox.com.yaml`.
//!
//! Do not edit. Change the spec or `rbx-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "economy.roblox.com";

/// `AssetEconomyCreator`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AssetEconomyCreator {
    #[serde(rename = "CreatorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_type: Option<String>,
    #[serde(rename = "HasVerifiedBadge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_verified_badge: Option<bool>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `AssetEconomyDetails`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AssetEconomyDetails {
    #[serde(rename = "AssetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<i64>,
    #[serde(rename = "AssetTypeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type_id: Option<i32>,
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "Creator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<AssetEconomyCreator>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IsForSale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_for_sale: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PriceInRobux")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_in_robux: Option<i64>,
    #[serde(rename = "Sales")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales: Option<i64>,
    #[serde(rename = "Updated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

/// `GroupFunds`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupFunds {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robux: Option<i64>,
}

/// `GET /v1/groups/{groupId}/currency`
///
/// A community's Robux, for members who may see the funds.
///
/// Undocumented; captured 2026-10-13, www.roblox.com/communities/{id}.
pub fn get_group_funds_request(group_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/groups/{}/currency", crate::rt::path_value(&group_id)),
    );
    Ok(r)
}

/// `GET /v1/groups/{groupId}/currency`
///
/// A community's Robux, for members who may see the funds.
///
/// Undocumented; captured 2026-10-13, www.roblox.com/communities/{id}.
pub async fn get_group_funds(client: &Client, group_id: i64) -> Result<GroupFunds> {
    client.send(get_group_funds_request(group_id)?).await?.json()
}

/// `GET /v2/assets/{assetId}/details`
///
/// Economy metadata for an asset, including Created and Updated timestamps.
///
/// Undocumented; captured 2026-10-14, www.roblox.com/catalog/1365767.
pub fn get_asset_economy_details_request(asset_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v2/assets/{}/details", crate::rt::path_value(&asset_id)),
    );
    Ok(r)
}

/// `GET /v2/assets/{assetId}/details`
///
/// Economy metadata for an asset, including Created and Updated timestamps.
///
/// Undocumented; captured 2026-10-14, www.roblox.com/catalog/1365767.
pub async fn get_asset_economy_details(client: &Client, asset_id: i64) -> Result<AssetEconomyDetails> {
    client
        .send(get_asset_economy_details_request(asset_id)?)
        .await?
        .json()
}

//! `catalog.roblox.com` undocumented — generated from `specs/undocumented/catalog.roblox.com.yaml`.
//!
//! Do not edit. Change the spec or `rbx-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "catalog.roblox.com";

/// `AssetRecommendations`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AssetRecommendations {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<i64>>,
}

/// `BundledItem`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BundledItem {
    #[serde(rename = "assetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "supportsHeadShapes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_head_shapes: Option<bool>,
    /// Asset or UserOutfit
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// `ItemTaxonomy`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ItemTaxonomy {
    #[serde(rename = "taxonomyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxonomy_id: Option<String>,
    #[serde(rename = "taxonomyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxonomy_name: Option<String>,
}

/// `ItemDetails`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ItemDetails {
    #[serde(rename = "assetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<i32>,
    #[serde(rename = "bundleType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_type: Option<i32>,
    #[serde(rename = "bundledItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundled_items: Option<Vec<BundledItem>>,
    #[serde(rename = "collectibleItemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collectible_item_id: Option<String>,
    #[serde(rename = "creatorHasVerifiedBadge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_has_verified_badge: Option<bool>,
    #[serde(rename = "creatorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_name: Option<String>,
    #[serde(rename = "creatorTargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_target_id: Option<i64>,
    /// User or Group
    #[serde(rename = "creatorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "favoriteCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favorite_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genres: Option<Vec<String>>,
    #[serde(rename = "hasResellers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_resellers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "isOffSale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_off_sale: Option<bool>,
    #[serde(rename = "isPurchasable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_purchasable: Option<bool>,
    #[serde(rename = "itemCreatedUtc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_created_utc: Option<String>,
    /// Limited, LimitedUnique, Collectible, Live, ...
    #[serde(rename = "itemRestrictions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_restrictions: Option<Vec<String>>,
    /// Asset or Bundle
    #[serde(rename = "itemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_type: Option<String>,
    #[serde(rename = "itemUpdatedUtc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_updated_utc: Option<String>,
    #[serde(rename = "lowestPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lowest_price: Option<i64>,
    #[serde(rename = "lowestResalePrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lowest_resale_price: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "offSaleDeadline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_sale_deadline: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owned: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
    /// Free, Off Sale, ... when there is no price to show
    #[serde(rename = "priceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_status: Option<String>,
    #[serde(rename = "renderProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_properties: Option<Vec<String>>,
    #[serde(rename = "saleLocationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_location_type: Option<String>,
    #[serde(rename = "supportsHeadShapes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_head_shapes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxonomy: Option<Vec<ItemTaxonomy>>,
    #[serde(rename = "totalQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_quantity: Option<i64>,
    #[serde(rename = "unitsAvailableForConsumption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units_available_for_consumption: Option<i64>,
}

/// `MenuSubcategory`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MenuSubcategory {
    #[serde(rename = "assetTypeIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type_ids: Option<Vec<i32>>,
    #[serde(rename = "bundleTypeIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_type_ids: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "shortName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subcategory: Option<String>,
    #[serde(rename = "subcategoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subcategory_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxonomy: Option<String>,
}

/// `MenuCategory`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MenuCategory {
    #[serde(rename = "assetTypeIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type_ids: Option<Vec<i32>>,
    #[serde(rename = "bundleTypeIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_type_ids: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "categoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<i32>,
    #[serde(rename = "isSearchable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_searchable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "orderIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subcategories: Option<Vec<MenuSubcategory>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxonomy: Option<String>,
}

/// `NavigationMenu`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct NavigationMenu {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<MenuCategory>>,
}

/// Query, header and form inputs for [`get_item_details`].
#[derive(Debug, Clone, Default)]
pub struct GetItemDetailsParams {
    /// Asset or Bundle
    pub item_type: String,
}

/// Query, header and form inputs for [`get_asset_recommendations`].
#[derive(Debug, Clone, Default)]
pub struct GetAssetRecommendationsParams {
    pub asset_type_id: i32,
    pub context_asset_id: i64,
    pub num_items: Option<i32>,
}

/// `GET /v1/catalog/items/{itemId}/details`
///
/// One asset or bundle with price, resale and collectible details.
///
/// Undocumented; captured 2026-10-13, www.roblox.com/catalog/1365767.
pub fn get_item_details_request(item_id: i64, params: &GetItemDetailsParams) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/catalog/items/{}/details", crate::rt::path_value(&item_id)),
    );
    r.query("itemType", &params.item_type);
    Ok(r)
}

/// `GET /v1/catalog/items/{itemId}/details`
///
/// One asset or bundle with price, resale and collectible details.
///
/// Undocumented; captured 2026-10-13, www.roblox.com/catalog/1365767.
pub async fn get_item_details(
    client: &Client,
    item_id: i64,
    params: &GetItemDetailsParams,
) -> Result<ItemDetails> {
    client
        .send(get_item_details_request(item_id, params)?)
        .await?
        .json()
}

/// `GET /v1/search/navigation-menu-items`
///
/// Marketplace categories and their subcategories, with the taxonomy id search takes.
///
/// Undocumented; captured 2026-10-13, www.roblox.com/catalog.
pub fn get_navigation_menu_items_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/search/navigation-menu-items".to_string());
    Ok(r)
}

/// `GET /v1/search/navigation-menu-items`
///
/// Marketplace categories and their subcategories, with the taxonomy id search takes.
///
/// Undocumented; captured 2026-10-13, www.roblox.com/catalog.
pub async fn get_navigation_menu_items(client: &Client) -> Result<NavigationMenu> {
    client.send(get_navigation_menu_items_request()?).await?.json()
}

/// `GET /v2/recommendations/assets`
///
/// Asset ids similar to an asset.
///
/// Undocumented; captured 2026-10-13, www.roblox.com/catalog/1365767.
pub fn get_asset_recommendations_request(params: &GetAssetRecommendationsParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v2/recommendations/assets".to_string());
    r.query("assetTypeId", &params.asset_type_id);
    r.query("contextAssetId", &params.context_asset_id);
    r.query("numItems", &params.num_items);
    Ok(r)
}

/// `GET /v2/recommendations/assets`
///
/// Asset ids similar to an asset.
///
/// Undocumented; captured 2026-10-13, www.roblox.com/catalog/1365767.
pub async fn get_asset_recommendations(
    client: &Client,
    params: &GetAssetRecommendationsParams,
) -> Result<AssetRecommendations> {
    client
        .send(get_asset_recommendations_request(params)?)
        .await?
        .json()
}

//! `itemconfiguration.roblox.com` v1 — generated from `vendor/roblox-api-tracker/itemconfiguration/v1.json`.
//!
//! Do not edit. Change the spec or `rbx-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "itemconfiguration.roblox.com";

/// `Roblox.ItemConfiguration.Api.AssetCreationsDetailsRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AssetCreationsDetailsRequest {
    #[serde(rename = "AssetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_ids: Option<Vec<i64>>,
}

crate::api_enum! {
    str
    /// The creator type. \['Unknown' = 0, 'User' = 1, 'Group' = 2\]
    AssetCreationsDetailsResponseCreatorType {
    UnknownValue = 0 => "Unknown",
    User = 1 => "User",
    Group = 2 => "Group",
    }
}

crate::api_enum! {
    str
    /// The asset status. \['Unknown' = 0, 'ReviewPending' = 1, 'Moderated' = 2, 'ReviewApproved' = 3, 'OnSale' = 4, 'OffSale' = 5, 'DelayedRelease' = 6, 'Free' = 7\]
    AssetCreationsDetailsResponseStatus {
    UnknownValue = 0 => "Unknown",
    ReviewPending = 1 => "ReviewPending",
    Moderated = 2 => "Moderated",
    ReviewApproved = 3 => "ReviewApproved",
    OnSale = 4 => "OnSale",
    OffSale = 5 => "OffSale",
    DelayedRelease = 6 => "DelayedRelease",
    Free = 7 => "Free",
    }
}

/// `Roblox.ItemConfiguration.Api.AssetCreationsDetailsResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AssetCreationsDetailsResponse {
    /// The asset Id.
    #[serde(rename = "assetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<i64>,
    /// Type of the asset.
    #[serde(rename = "assetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<String>,
    /// Date asset was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// The creator target Id.
    #[serde(rename = "creatorTargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_target_id: Option<i64>,
    /// The creator type. \['Unknown' = 0, 'User' = 1, 'Group' = 2\]
    #[serde(rename = "creatorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_type: Option<AssetCreationsDetailsResponseCreatorType>,
    /// The asset description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Is the asset archived.
    #[serde(rename = "isArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// The asset name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The asset status. \['Unknown' = 0, 'ReviewPending' = 1, 'Moderated' = 2, 'ReviewApproved' = 3, 'OnSale' = 4, 'OffSale' = 5, 'DelayedRelease' = 6, 'Free' = 7\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AssetCreationsDetailsResponseStatus>,
    /// Date asset was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

/// `Roblox.ItemConfiguration.Api.AssetCreationsResponse`
///
/// Asset Status response model.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AssetCreationsResponse {
    /// The asset Id.
    #[serde(rename = "assetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<i64>,
    /// The asset name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

crate::api_enum! {
    int
    /// Publishing type for the collectible. Currently can be either Limited or NonLimited. \['Invalid' = 0, 'Limited' = 1, 'NonLimited' = 2\]
    CreatePublishingPreferencesRequestPublishingType {
    Invalid = 0 => "Invalid",
    Limited = 1 => "Limited",
    NonLimited = 2 => "NonLimited",
    }
}

crate::api_enum! {
    int
    /// Represents possible SaleLocation types of the Collectibles System. \['SALE_LOCATION_TYPE_INVALID' = 0, 'SALE_LOCATION_TYPE_SHOP_AND_ALL_EXPERIENCES' = 1, 'SALE_LOCATION_EXPERIENCES_AND_DEV_API_ONLY' = 2, 'SALE_LOCATION_TYPE_SHOP_ONLY' = 3, 'SALE_LOCATION_TYPE_SHOP_AND_EXPERIENCES_BY_ID' = 4\]
    CreatePublishingPreferencesRequestSaleLocationType {
    SaleLocationTypeInvalid = 0 => "SALE_LOCATION_TYPE_INVALID",
    SaleLocationTypeShopAndAllExperiences = 1 => "SALE_LOCATION_TYPE_SHOP_AND_ALL_EXPERIENCES",
    SaleLocationExperiencesAndDevApiOnly = 2 => "SALE_LOCATION_EXPERIENCES_AND_DEV_API_ONLY",
    SaleLocationTypeShopOnly = 3 => "SALE_LOCATION_TYPE_SHOP_ONLY",
    SaleLocationTypeShopAndExperiencesById = 4 => "SALE_LOCATION_TYPE_SHOP_AND_EXPERIENCES_BY_ID",
    }
}

/// `Roblox.ItemConfiguration.Api.Models.Request.PublishingPreferences.CreatePublishingPreferencesRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CreatePublishingPreferencesRequest {
    #[serde(rename = "autoPublishEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_publish_enabled: Option<bool>,
    #[serde(rename = "creatorGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_group_id: Option<i64>,
    #[serde(rename = "creatorUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_user_id: Option<i64>,
    #[serde(rename = "enableRegionalPricing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_regional_pricing: Option<bool>,
    #[serde(rename = "isFree")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_free: Option<bool>,
    #[serde(rename = "isRentalOptIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rental_opt_in: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub places: Option<Vec<i64>>,
    #[serde(rename = "priceInRobux")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_in_robux: Option<i64>,
    #[serde(rename = "priceOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_offset: Option<i64>,
    /// Publishing type for the collectible. Currently can be either Limited or NonLimited. \['Invalid' = 0, 'Limited' = 1, 'NonLimited' = 2\]
    #[serde(rename = "publishingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publishing_type: Option<CreatePublishingPreferencesRequestPublishingType>,
    /// Represents possible SaleLocation types of the Collectibles System. \['SALE_LOCATION_TYPE_INVALID' = 0, 'SALE_LOCATION_TYPE_SHOP_AND_ALL_EXPERIENCES' = 1, 'SALE_LOCATION_EXPERIENCES_AND_DEV_API_ONLY' = 2, 'SALE_LOCATION_TYPE_SHOP_ONLY' = 3, 'SALE_LOCATION_TYPE_SHOP_AND_EXPERIENCES_BY_ID' = 4\]
    #[serde(rename = "saleLocationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_location_type: Option<CreatePublishingPreferencesRequestSaleLocationType>,
}

/// `Roblox.ItemConfiguration.Api.Models.Request.PublishingPreferences.UpdatePublishingPreferencesRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdatePublishingPreferencesRequest {
    #[serde(rename = "autoPublishEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_publish_enabled: Option<bool>,
    #[serde(rename = "creatorGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_group_id: Option<i64>,
    #[serde(rename = "creatorUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_user_id: Option<i64>,
    #[serde(rename = "enableRegionalPricing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_regional_pricing: Option<bool>,
    #[serde(rename = "isFree")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_free: Option<bool>,
    #[serde(rename = "isRentalOptIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rental_opt_in: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub places: Option<Vec<i64>>,
    #[serde(rename = "priceInRobux")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_in_robux: Option<i64>,
    #[serde(rename = "priceOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_offset: Option<i64>,
    #[serde(rename = "publishingType")]
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publishing_type: Option<i32>,
    #[serde(rename = "saleLocationType")]
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_location_type: Option<i32>,
}

/// `Roblox.ItemConfiguration.Api.Models.Response.PublishingPreferences.PublishingPreferencesResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PublishingPreferencesResponse {
    #[serde(rename = "autoPublishEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_publish_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "creatorGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_group_id: Option<i64>,
    #[serde(rename = "creatorUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_user_id: Option<i64>,
    #[serde(rename = "enableRegionalPricing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_regional_pricing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isFree")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_free: Option<bool>,
    #[serde(rename = "isRentalOptIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rental_opt_in: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub places: Option<Vec<i64>>,
    #[serde(rename = "priceInRobux")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_in_robux: Option<i64>,
    #[serde(rename = "priceOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_offset: Option<i64>,
    /// Publishing type for the collectible. Currently can be either Limited or NonLimited. \['Invalid' = 0, 'Limited' = 1, 'NonLimited' = 2\]
    #[serde(rename = "publishingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publishing_type: Option<CreatePublishingPreferencesRequestPublishingType>,
    /// Represents possible SaleLocation types of the Collectibles System. \['SALE_LOCATION_TYPE_INVALID' = 0, 'SALE_LOCATION_TYPE_SHOP_AND_ALL_EXPERIENCES' = 1, 'SALE_LOCATION_EXPERIENCES_AND_DEV_API_ONLY' = 2, 'SALE_LOCATION_TYPE_SHOP_ONLY' = 3, 'SALE_LOCATION_TYPE_SHOP_AND_EXPERIENCES_BY_ID' = 4\]
    #[serde(rename = "saleLocationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_location_type: Option<CreatePublishingPreferencesRequestSaleLocationType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

/// `Roblox.Web.WebAPI.Models.ApiPageResponse\[Roblox.ItemConfiguration.Api.AssetCreationsResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiPageResponseAssetCreationsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<AssetCreationsResponse>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
}

/// Query, header and form inputs for [`get_creations_get_assets`].
#[derive(Debug, Clone, Default)]
pub struct GetCreationsGetAssetsParams {
    pub asset_type: String,
    pub is_archived: Option<bool>,
    pub group_id: Option<i64>,
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
}

/// Query, header and form inputs for [`get_preferences_publishing`].
#[derive(Debug, Clone, Default)]
pub struct GetPreferencesPublishingParams {
    pub group_id: Option<i64>,
}

/// Query, header and form inputs for [`delete_preferences_publishing`].
#[derive(Debug, Clone, Default)]
pub struct DeletePreferencesPublishingParams {
    pub group_id: Option<i64>,
}

/// `POST /v1/creations/get-asset-details`
///
/// Gets the asset status and other configuration details for the given assetIds list.
pub fn post_creations_get_asset_details_request(body: &AssetCreationsDetailsRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/creations/get-asset-details".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/creations/get-asset-details`
///
/// Gets the asset status and other configuration details for the given assetIds list.
pub async fn post_creations_get_asset_details(
    client: &Client,
    body: &AssetCreationsDetailsRequest,
) -> Result<Vec<AssetCreationsDetailsResponse>> {
    client
        .send(post_creations_get_asset_details_request(body)?)
        .await?
        .json()
}

/// `GET /v1/creations/get-assets`
///
/// Gets the user created asset information filtered by the given asset type.
pub fn get_creations_get_assets_request(params: &GetCreationsGetAssetsParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/creations/get-assets".to_string());
    r.query("assetType", &params.asset_type);
    r.query("isArchived", &params.is_archived);
    r.query("groupId", &params.group_id);
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    Ok(r)
}

/// `GET /v1/creations/get-assets`
///
/// Gets the user created asset information filtered by the given asset type.
pub async fn get_creations_get_assets(
    client: &Client,
    params: &GetCreationsGetAssetsParams,
) -> Result<ApiPageResponseAssetCreationsResponse> {
    client
        .send(get_creations_get_assets_request(params)?)
        .await?
        .json()
}

/// `GET /v1/preferences/publishing`
pub fn get_preferences_publishing_request(params: &GetPreferencesPublishingParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/preferences/publishing".to_string());
    r.query("groupId", &params.group_id);
    Ok(r)
}

/// `GET /v1/preferences/publishing`
pub async fn get_preferences_publishing(
    client: &Client,
    params: &GetPreferencesPublishingParams,
) -> Result<PublishingPreferencesResponse> {
    client
        .send(get_preferences_publishing_request(params)?)
        .await?
        .json()
}

/// `POST /v1/preferences/publishing`
pub fn post_preferences_publishing_request(body: &CreatePublishingPreferencesRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/preferences/publishing".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/preferences/publishing`
pub async fn post_preferences_publishing(
    client: &Client,
    body: &CreatePublishingPreferencesRequest,
) -> Result<PublishingPreferencesResponse> {
    client
        .send(post_preferences_publishing_request(body)?)
        .await?
        .json()
}

/// `PATCH /v1/preferences/publishing`
pub fn patch_preferences_publishing_request(body: &UpdatePublishingPreferencesRequest) -> Result<Request> {
    let mut r = Request::new(Method::PATCH, HOST, "/v1/preferences/publishing".to_string());
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/preferences/publishing`
pub async fn patch_preferences_publishing(
    client: &Client,
    body: &UpdatePublishingPreferencesRequest,
) -> Result<PublishingPreferencesResponse> {
    client
        .send(patch_preferences_publishing_request(body)?)
        .await?
        .json()
}

/// `DELETE /v1/preferences/publishing`
pub fn delete_preferences_publishing_request(params: &DeletePreferencesPublishingParams) -> Result<Request> {
    let mut r = Request::new(Method::DELETE, HOST, "/v1/preferences/publishing".to_string());
    r.query("groupId", &params.group_id);
    Ok(r)
}

/// `DELETE /v1/preferences/publishing`
pub async fn delete_preferences_publishing(
    client: &Client,
    params: &DeletePreferencesPublishingParams,
) -> Result<()> {
    client
        .send(delete_preferences_publishing_request(params)?)
        .await?
        .unit()
}

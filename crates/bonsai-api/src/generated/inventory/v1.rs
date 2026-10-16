//! `inventory.roblox.com` v1 — generated from `vendor/roblox-api-tracker/inventory/v1.json`.
//!
//! Do not edit. Change the spec or `bonsai-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "inventory.roblox.com";

/// `Roblox.Inventory.Api.Models.AssetIdListModel`
///
/// A model that contains a list of asset ids
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AssetIdListModel {
    /// The asset ids
    #[serde(rename = "assetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_ids: Option<Vec<i64>>,
}

crate::api_enum! {
    int
    /// Describes what type an AssetsExplorerCategoryItemModel contains \['AssetType' = 0, 'Bundle' = 1, 'Outfit' = 2, 'Set' = 3, 'Avatar' = 4\]
    AssetsExplorerCategoryItemModelType {
    AssetType = 0 => "AssetType",
    Bundle = 1 => "Bundle",
    Outfit = 2 => "Outfit",
    Set = 3 => "Set",
    Avatar = 4 => "Avatar",
    }
}

/// `Roblox.Inventory.Api.Models.AssetsExplorerCategoryItemModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AssetsExplorerCategoryItemModel {
    #[serde(rename = "categoryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_type: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Describes what type an AssetsExplorerCategoryItemModel contains \['AssetType' = 0, 'Bundle' = 1, 'Outfit' = 2, 'Set' = 3, 'Avatar' = 4\]
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<AssetsExplorerCategoryItemModelType>,
}

/// `Roblox.Inventory.Api.Models.AssetsExplorerCategoryModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AssetsExplorerCategoryModel {
    #[serde(rename = "categoryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_type: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<AssetsExplorerCategoryItemModel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.Inventory.Api.Models.CanViewInventoryResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CanViewInventoryResponse {
    /// Boolean describing if the user's inventory can be viewed
    #[serde(rename = "canView")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_view: Option<bool>,
}

/// `Roblox.Inventory.Api.Models.CategoriesModel`
///
/// Model class that contains the categories of the Inventory or Favorites page
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CategoriesModel {
    /// Categories to show up in Inventory or Favorites page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<AssetsExplorerCategoryModel>>,
}

crate::api_enum! {
    int
    /// The recent average price of the user asset \['None' = 0, 'BC' = 1, 'TBC' = 2, 'OBC' = 3, 'RobloxPremium' = 4\]
    CollectibleUserAssetModelBuildersClubMembershipType {
    None = 0 => "None",
    Bc = 1 => "BC",
    Tbc = 2 => "TBC",
    Obc = 3 => "OBC",
    RobloxPremium = 4 => "RobloxPremium",
    }
}

/// `Roblox.Inventory.Api.Models.CollectibleUserAssetModel`
///
/// A model containing information about a Roblox.Platform.AssetOwnership.UserAsset
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CollectibleUserAssetModel {
    /// The asset id of the user asset
    #[serde(rename = "assetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<i64>,
    /// The recent average price of the user asset
    #[serde(rename = "assetStock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_stock: Option<i64>,
    /// The recent average price of the user asset \['None' = 0, 'BC' = 1, 'TBC' = 2, 'OBC' = 3, 'RobloxPremium' = 4\]
    #[serde(rename = "buildersClubMembershipType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builders_club_membership_type: Option<CollectibleUserAssetModelBuildersClubMembershipType>,
    /// Whether the user asset has an active hold.
    #[serde(rename = "isOnHold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_on_hold: Option<bool>,
    /// The asset name of the asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The original price of the asset
    #[serde(rename = "originalPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_price: Option<i64>,
    /// The recent average price of the asset
    #[serde(rename = "recentAveragePrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recent_average_price: Option<i64>,
    /// The serial number of the user asset
    #[serde(rename = "serialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<i64>,
    /// The user asset id
    #[serde(rename = "userAssetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_asset_id: Option<i64>,
}

crate::api_enum! {
    int
    /// \['User' = 1, 'Group' = 2, 'Experience' = 3\]
    CreatorModelType {
    User = 1 => "User",
    Group = 2 => "Group",
    Experience = 3 => "Experience",
    }
}

/// `Roblox.Inventory.Api.Models.CreatorModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CreatorModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// \['User' = 1, 'Group' = 2, 'Experience' = 3\]
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<CreatorModelType>,
}

crate::api_enum! {
    int
    /// The type of the item \['Asset' = 0, 'GamePass' = 1, 'Badge' = 2, 'Bundle' = 3, 'Avatar' = 4\]
    IItemModelType {
    Asset = 0 => "Asset",
    GamePass = 1 => "GamePass",
    Badge = 2 => "Badge",
    Bundle = 3 => "Bundle",
    Avatar = 4 => "Avatar",
    }
}

/// `Roblox.Inventory.Api.Models.IItemModel`
///
/// Model representing an inventory item
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct IItemModel {
    /// The ID of the item
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The instance id of the item if applicable
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<i64>,
    /// The name of the item
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The type of the item \['Asset' = 0, 'GamePass' = 1, 'Badge' = 2, 'Bundle' = 3, 'Avatar' = 4\]
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<IItemModelType>,
}

/// `Roblox.Inventory.Api.Models.PlaceModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PlaceModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<CreatorModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "placeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_id: Option<i64>,
    #[serde(rename = "priceInRobux")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_in_robux: Option<i64>,
    #[serde(rename = "universeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_id: Option<i64>,
}

/// `Roblox.Web.WebAPI.ApiEmptyResponseModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiEmptyResponseModel {}

/// `Roblox.Web.WebAPI.Models.ApiPageResponse\[Roblox.Inventory.Api.Models.CollectibleUserAssetModel\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiPageResponseCollectibleUserAssetModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<CollectibleUserAssetModel>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
}

/// `Roblox.Web.WebAPI.Models.ApiPageResponse\[Roblox.Inventory.Api.Models.IItemModel\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiPageResponseIItemModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<IItemModel>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
}

/// `Roblox.Web.WebAPI.Models.ApiPageResponse\[Roblox.Inventory.Api.Models.PlaceModel\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiPageResponsePlaceModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<PlaceModel>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
}

crate::api_enum! {
    str
    /// Sorted by userAssetId
    GetUsersByUserIdAssetsCollectiblesSortOrder {
    Asc = 0 => "Asc",
    Desc = 1 => "Desc",
    }
}

/// Query, header and form inputs for [`get_users_by_user_id_assets_collectibles`].
#[derive(Debug, Clone, Default)]
pub struct GetUsersByUserIdAssetsCollectiblesParams {
    /// The asset type for the collectibles you're trying to get.
    pub asset_type: Option<i32>,
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// Sorted by userAssetId
    pub sort_order: Option<GetUsersByUserIdAssetsCollectiblesSortOrder>,
}

/// Query, header and form inputs for [`get_users_by_user_id_places_inventory`].
#[derive(Debug, Clone, Default)]
pub struct GetUsersByUserIdPlacesInventoryParams {
    pub places_tab: i32,
    pub items_per_page: i32,
    pub cursor: i32,
}

/// `POST /v1/collections/items/{itemType}/{itemTargetId}`
///
/// Adds an item to the appropriate collection
pub fn post_collections_items_by_item_type_by_item_target_id_request(
    item_type: i32,
    item_target_id: i64,
) -> Result<Request> {
    let r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/collections/items/{}/{}",
            crate::rt::path_value(&item_type),
            crate::rt::path_value(&item_target_id)
        ),
    );
    Ok(r)
}

/// `POST /v1/collections/items/{itemType}/{itemTargetId}`
///
/// Adds an item to the appropriate collection
pub async fn post_collections_items_by_item_type_by_item_target_id(
    client: &Client,
    item_type: i32,
    item_target_id: i64,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_collections_items_by_item_type_by_item_target_id_request(
            item_type,
            item_target_id,
        )?)
        .await?
        .json()
}

/// `DELETE /v1/collections/items/{itemType}/{itemTargetId}`
///
/// Removes an item to the appropriate collection
pub fn delete_collections_items_by_item_type_by_item_target_id_request(
    item_type: i32,
    item_target_id: i64,
) -> Result<Request> {
    let r = Request::new(
        Method::DELETE,
        HOST,
        format!(
            "/v1/collections/items/{}/{}",
            crate::rt::path_value(&item_type),
            crate::rt::path_value(&item_target_id)
        ),
    );
    Ok(r)
}

/// `DELETE /v1/collections/items/{itemType}/{itemTargetId}`
///
/// Removes an item to the appropriate collection
pub async fn delete_collections_items_by_item_type_by_item_target_id(
    client: &Client,
    item_type: i32,
    item_target_id: i64,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(delete_collections_items_by_item_type_by_item_target_id_request(
            item_type,
            item_target_id,
        )?)
        .await?
        .json()
}

/// `GET /v1/packages/{packageId}/assets`
///
/// Given a package ID, returns the list of asset IDs for that package
#[deprecated(note = "marked deprecated by Roblox")]
pub fn get_packages_by_package_id_assets_request(package_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/packages/{}/assets", crate::rt::path_value(&package_id)),
    );
    Ok(r)
}

/// `GET /v1/packages/{packageId}/assets`
///
/// Given a package ID, returns the list of asset IDs for that package
#[deprecated(note = "marked deprecated by Roblox")]
pub async fn get_packages_by_package_id_assets(client: &Client, package_id: i64) -> Result<AssetIdListModel> {
    client
        .send(get_packages_by_package_id_assets_request(package_id)?)
        .await?
        .json()
}

/// `GET /v1/users/{userId}/assets/collectibles`
///
/// Gets all collectible assets owned by the specified user.
pub fn get_users_by_user_id_assets_collectibles_request(
    user_id: i64,
    params: &GetUsersByUserIdAssetsCollectiblesParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/users/{}/assets/collectibles",
            crate::rt::path_value(&user_id)
        ),
    );
    r.query("assetType", &params.asset_type);
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    r.query("sortOrder", &params.sort_order);
    Ok(r)
}

/// `GET /v1/users/{userId}/assets/collectibles`
///
/// Gets all collectible assets owned by the specified user.
pub async fn get_users_by_user_id_assets_collectibles(
    client: &Client,
    user_id: i64,
    params: &GetUsersByUserIdAssetsCollectiblesParams,
) -> Result<ApiPageResponseCollectibleUserAssetModel> {
    client
        .send(get_users_by_user_id_assets_collectibles_request(user_id, params)?)
        .await?
        .json()
}

/// `GET /v1/users/{userId}/can-view-inventory`
///
/// Gets whether the specified user's inventory can be viewed.
pub fn get_users_by_user_id_can_view_inventory_request(user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/users/{}/can-view-inventory", crate::rt::path_value(&user_id)),
    );
    Ok(r)
}

/// `GET /v1/users/{userId}/can-view-inventory`
///
/// Gets whether the specified user's inventory can be viewed.
pub async fn get_users_by_user_id_can_view_inventory(
    client: &Client,
    user_id: i64,
) -> Result<CanViewInventoryResponse> {
    client
        .send(get_users_by_user_id_can_view_inventory_request(user_id)?)
        .await?
        .json()
}

/// `GET /v1/users/{userId}/categories`
///
/// Return inventory categories for a user
pub fn get_users_by_user_id_categories_request(user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/users/{}/categories", crate::rt::path_value(&user_id)),
    );
    Ok(r)
}

/// `GET /v1/users/{userId}/categories`
///
/// Return inventory categories for a user
pub async fn get_users_by_user_id_categories(client: &Client, user_id: i64) -> Result<CategoriesModel> {
    client
        .send(get_users_by_user_id_categories_request(user_id)?)
        .await?
        .json()
}

/// `GET /v1/users/{userId}/categories/favorites`
///
/// Return favorites categories for a user
pub fn get_users_by_user_id_categories_favorites_request(user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/users/{}/categories/favorites",
            crate::rt::path_value(&user_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/users/{userId}/categories/favorites`
///
/// Return favorites categories for a user
pub async fn get_users_by_user_id_categories_favorites(
    client: &Client,
    user_id: i64,
) -> Result<CategoriesModel> {
    client
        .send(get_users_by_user_id_categories_favorites_request(user_id)?)
        .await?
        .json()
}

/// `GET /v1/users/{userId}/items/{itemType}/{itemTargetId}`
///
/// Gets owned items of the specified item type. Game Servers can make requests for any user, but can only make requests for game passes that belong to the place sending the request. Place creators can make requests as if they were the Game Server.
pub fn get_users_by_user_id_items_by_item_type_by_item_target_id_request(
    user_id: i64,
    item_type: i32,
    item_target_id: i64,
) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/users/{}/items/{}/{}",
            crate::rt::path_value(&user_id),
            crate::rt::path_value(&item_type),
            crate::rt::path_value(&item_target_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/users/{userId}/items/{itemType}/{itemTargetId}`
///
/// Gets owned items of the specified item type. Game Servers can make requests for any user, but can only make requests for game passes that belong to the place sending the request. Place creators can make requests as if they were the Game Server.
pub async fn get_users_by_user_id_items_by_item_type_by_item_target_id(
    client: &Client,
    user_id: i64,
    item_type: i32,
    item_target_id: i64,
) -> Result<ApiPageResponseIItemModel> {
    client
        .send(get_users_by_user_id_items_by_item_type_by_item_target_id_request(
            user_id,
            item_type,
            item_target_id,
        )?)
        .await?
        .json()
}

/// `GET /v1/users/{userId}/items/{itemType}/{itemTargetId}/is-owned`
///
/// Gets whether a user owns an item of type itemType with id itemTargetId.
pub fn get_users_by_user_id_items_by_item_type_by_item_target_id_is_owned_request(
    user_id: i64,
    item_type: i32,
    item_target_id: i64,
) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/users/{}/items/{}/{}/is-owned",
            crate::rt::path_value(&user_id),
            crate::rt::path_value(&item_type),
            crate::rt::path_value(&item_target_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/users/{userId}/items/{itemType}/{itemTargetId}/is-owned`
///
/// Gets whether a user owns an item of type itemType with id itemTargetId.
pub async fn get_users_by_user_id_items_by_item_type_by_item_target_id_is_owned(
    client: &Client,
    user_id: i64,
    item_type: i32,
    item_target_id: i64,
) -> Result<bool> {
    client
        .send(
            get_users_by_user_id_items_by_item_type_by_item_target_id_is_owned_request(
                user_id,
                item_type,
                item_target_id,
            )?,
        )
        .await?
        .json()
}

/// `GET /v1/users/{userId}/places/inventory`
///
/// Gets Created, MyGames, or OtherGames places inventory for a user
pub fn get_users_by_user_id_places_inventory_request(
    user_id: i64,
    params: &GetUsersByUserIdPlacesInventoryParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/users/{}/places/inventory", crate::rt::path_value(&user_id)),
    );
    r.query("placesTab", &params.places_tab);
    r.query("itemsPerPage", &params.items_per_page);
    r.query("cursor", &params.cursor);
    Ok(r)
}

/// `GET /v1/users/{userId}/places/inventory`
///
/// Gets Created, MyGames, or OtherGames places inventory for a user
pub async fn get_users_by_user_id_places_inventory(
    client: &Client,
    user_id: i64,
    params: &GetUsersByUserIdPlacesInventoryParams,
) -> Result<ApiPageResponsePlaceModel> {
    client
        .send(get_users_by_user_id_places_inventory_request(user_id, params)?)
        .await?
        .json()
}

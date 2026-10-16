//! `inventory.roblox.com` v2 — generated from `vendor/roblox-api-tracker/inventory/v2.json`.
//!
//! Do not edit. Change the spec or `bonsai-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "inventory.roblox.com";

/// `Roblox.Inventory.Api.Models.UserModel`
///
/// A model representing data about an Roblox.Platform.Membership.IUser
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UserModel {
    /// The user's builders club membership type \['None' = 0, 'BC' = 1, 'TBC' = 2, 'OBC' = 3, 'RobloxPremium' = 4\]
    #[serde(rename = "buildersClubMembershipType")]
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builders_club_membership_type: Option<i32>,
    /// The user id
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// `Roblox.Inventory.Api.Models.InventoryItemModel`
///
/// A model containing information about an inventory item.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct InventoryItemModel {
    /// The asset id of the user asset
    #[serde(rename = "assetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<i64>,
    /// The asset name of the user asset
    #[serde(rename = "assetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_name: Option<String>,
    /// The id of the corresponding collectible item
    #[serde(rename = "collectibleItemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collectible_item_id: Option<String>,
    /// The id of the corresponding collectible item instance
    #[serde(rename = "collectibleItemInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collectible_item_instance_id: Option<String>,
    /// The creation date of the user asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// Expiration timestamp for transient items
    #[serde(rename = "expireAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<UserModel>,
    /// The serial number of the user asset
    #[serde(rename = "serialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<i64>,
    /// The updated date of the user asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    /// The user asset id
    #[serde(rename = "userAssetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_asset_id: Option<i64>,
}

crate::api_enum! {
    int
    /// \['User' = 1, 'Group' = 2\]
    RelatedEntityTypeResponseAssociatedEntityTypeType {
    User = 1 => "User",
    Group = 2 => "Group",
    }
}

/// `Roblox.Web.Responses.RelatedEntityTypeResponse\[Roblox.Users.Client.AssociatedEntityType\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RelatedEntityTypeResponseAssociatedEntityType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// \['User' = 1, 'Group' = 2\]
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<RelatedEntityTypeResponseAssociatedEntityTypeType>,
}

/// `Roblox.Inventory.Api.V2.AssetOwnerResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AssetOwnerResponse {
    #[serde(rename = "collectibleItemInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collectible_item_instance_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<RelatedEntityTypeResponseAssociatedEntityType>,
    #[serde(rename = "serialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

/// `Roblox.Inventory.Api.V2.CollectibleItemOwnerResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CollectibleItemOwnerResponse {
    #[serde(rename = "collectibleItemInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collectible_item_instance_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<RelatedEntityTypeResponseAssociatedEntityType>,
    #[serde(rename = "serialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<i64>,
}

crate::api_enum! {
    int
    /// The asset type id of asset with id Roblox.Inventory.Api.V2.UserAssetItemModelV2.AssetId. \['Image' = 1, 'TShirt' = 2, 'Audio' = 3, 'Mesh' = 4, 'Lua' = 5, 'HTML' = 6, 'Text' = 7, 'Hat' = 8, 'Place' = 9, 'Model' = 10, 'Shirt' = 11, 'Pants' = 12, 'Decal' = 13, 'Avatar' = 16, 'Head' = 17, 'Face' = 18, 'Gear' = 19, 'Badge' = 21, 'GroupEmblem' = 22, 'Animation' = 24, 'Arms' = 25, 'Legs' = 26, 'Torso' = 27, 'RightArm' = 28, 'LeftArm' = 29, 'LeftLeg' = 30, 'RightLeg' = 31, 'Package' = 32, 'YouTubeVideo' = 33, 'GamePass' = 34, 'App' = 35, 'Code' = 37, 'Plugin' = 38, 'SolidModel' = 39, 'MeshPart' = 40, 'HairAccessory' = 41, 'FaceAccessory' = 42, 'NeckAccessory' = 43, 'ShoulderAccessory' = 44, 'FrontAccessory' = 45, 'BackAccessory' = 46, 'WaistAccessory' = 47, 'ClimbAnimation' = 48, 'DeathAnimation' = 49, 'FallAnimation' = 50, 'IdleAnimation' = 51, 'JumpAnimation' = 52, 'RunAnimation' = 53, 'SwimAnimation' = 54, 'WalkAnimation' = 55, 'PoseAnimation' = 56, 'LocalizationTableManifest' = 59, 'LocalizationTableTranslation' = 60, 'EmoteAnimation' = 61, 'Video' = 62, 'TexturePack' = 63, 'TShirtAccessory' = 64, 'ShirtAccessory' = 65, 'PantsAccessory' = 66, 'JacketAccessory' = 67, 'SweaterAccessory' = 68, 'ShortsAccessory' = 69, 'LeftShoeAccessory' = 70, 'RightShoeAccessory' = 71, 'DressSkirtAccessory' = 72, 'FontFamily' = 73, 'FontFace' = 74, 'MeshHiddenSurfaceRemoval' = 75, 'EyebrowAccessory' = 76, 'EyelashAccessory' = 77, 'MoodAnimation' = 78, 'DynamicHead' = 79, 'CodeSnippet' = 80, 'AdsVideo' = 81, 'OtaUpdate' = 82, 'Screenshot' = 83, 'RuntimePropertySet' = 84, 'StorePreviewVideo' = 85, 'GamePreviewVideo' = 86, 'CreatorExperienceConfig' = 87, 'FaceMakeup' = 88, 'LipMakeup' = 89, 'EyeMakeup' = 90, 'VoxelFragment' = 91, 'AvatarBackground' = 92, 'TextDocument' = 93, 'Post' = 94, 'AnimatedImage' = 95, 'ComputeFunction' = 96\]
    UserAssetItemModelV2AssetType {
    Image = 1 => "Image",
    TShirt = 2 => "TShirt",
    Audio = 3 => "Audio",
    Mesh = 4 => "Mesh",
    Lua = 5 => "Lua",
    Html = 6 => "HTML",
    Text = 7 => "Text",
    Hat = 8 => "Hat",
    Place = 9 => "Place",
    Model = 10 => "Model",
    Shirt = 11 => "Shirt",
    Pants = 12 => "Pants",
    Decal = 13 => "Decal",
    Avatar = 16 => "Avatar",
    Head = 17 => "Head",
    Face = 18 => "Face",
    Gear = 19 => "Gear",
    Badge = 21 => "Badge",
    GroupEmblem = 22 => "GroupEmblem",
    Animation = 24 => "Animation",
    Arms = 25 => "Arms",
    Legs = 26 => "Legs",
    Torso = 27 => "Torso",
    RightArm = 28 => "RightArm",
    LeftArm = 29 => "LeftArm",
    LeftLeg = 30 => "LeftLeg",
    RightLeg = 31 => "RightLeg",
    Package = 32 => "Package",
    YouTubeVideo = 33 => "YouTubeVideo",
    GamePass = 34 => "GamePass",
    App = 35 => "App",
    Code = 37 => "Code",
    Plugin = 38 => "Plugin",
    SolidModel = 39 => "SolidModel",
    MeshPart = 40 => "MeshPart",
    HairAccessory = 41 => "HairAccessory",
    FaceAccessory = 42 => "FaceAccessory",
    NeckAccessory = 43 => "NeckAccessory",
    ShoulderAccessory = 44 => "ShoulderAccessory",
    FrontAccessory = 45 => "FrontAccessory",
    BackAccessory = 46 => "BackAccessory",
    WaistAccessory = 47 => "WaistAccessory",
    ClimbAnimation = 48 => "ClimbAnimation",
    DeathAnimation = 49 => "DeathAnimation",
    FallAnimation = 50 => "FallAnimation",
    IdleAnimation = 51 => "IdleAnimation",
    JumpAnimation = 52 => "JumpAnimation",
    RunAnimation = 53 => "RunAnimation",
    SwimAnimation = 54 => "SwimAnimation",
    WalkAnimation = 55 => "WalkAnimation",
    PoseAnimation = 56 => "PoseAnimation",
    LocalizationTableManifest = 59 => "LocalizationTableManifest",
    LocalizationTableTranslation = 60 => "LocalizationTableTranslation",
    EmoteAnimation = 61 => "EmoteAnimation",
    Video = 62 => "Video",
    TexturePack = 63 => "TexturePack",
    TShirtAccessory = 64 => "TShirtAccessory",
    ShirtAccessory = 65 => "ShirtAccessory",
    PantsAccessory = 66 => "PantsAccessory",
    JacketAccessory = 67 => "JacketAccessory",
    SweaterAccessory = 68 => "SweaterAccessory",
    ShortsAccessory = 69 => "ShortsAccessory",
    LeftShoeAccessory = 70 => "LeftShoeAccessory",
    RightShoeAccessory = 71 => "RightShoeAccessory",
    DressSkirtAccessory = 72 => "DressSkirtAccessory",
    FontFamily = 73 => "FontFamily",
    FontFace = 74 => "FontFace",
    MeshHiddenSurfaceRemoval = 75 => "MeshHiddenSurfaceRemoval",
    EyebrowAccessory = 76 => "EyebrowAccessory",
    EyelashAccessory = 77 => "EyelashAccessory",
    MoodAnimation = 78 => "MoodAnimation",
    DynamicHead = 79 => "DynamicHead",
    CodeSnippet = 80 => "CodeSnippet",
    AdsVideo = 81 => "AdsVideo",
    OtaUpdate = 82 => "OtaUpdate",
    Screenshot = 83 => "Screenshot",
    RuntimePropertySet = 84 => "RuntimePropertySet",
    StorePreviewVideo = 85 => "StorePreviewVideo",
    GamePreviewVideo = 86 => "GamePreviewVideo",
    CreatorExperienceConfig = 87 => "CreatorExperienceConfig",
    FaceMakeup = 88 => "FaceMakeup",
    LipMakeup = 89 => "LipMakeup",
    EyeMakeup = 90 => "EyeMakeup",
    VoxelFragment = 91 => "VoxelFragment",
    AvatarBackground = 92 => "AvatarBackground",
    TextDocument = 93 => "TextDocument",
    Post = 94 => "Post",
    AnimatedImage = 95 => "AnimatedImage",
    ComputeFunction = 96 => "ComputeFunction",
    }
}

/// `Roblox.Inventory.Api.V2.UserAssetItemModelV2`
///
/// The user asset item model for V2 controllers.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UserAssetItemModelV2 {
    /// The asset id of the user asset.
    #[serde(rename = "assetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<i64>,
    /// The asset type id of asset with id Roblox.Inventory.Api.V2.UserAssetItemModelV2.AssetId. \['Image' = 1, 'TShirt' = 2, 'Audio' = 3, 'Mesh' = 4, 'Lua' = 5, 'HTML' = 6, 'Text' = 7, 'Hat' = 8, 'Place' = 9, 'Model' = 10, 'Shirt' = 11, 'Pants' = 12, 'Decal' = 13, 'Avatar' = 16, 'Head' = 17, 'Face' = 18, 'Gear' = 19, 'Badge' = 21, 'GroupEmblem' = 22, 'Animation' = 24, 'Arms' = 25, 'Legs' = 26, 'Torso' = 27, 'RightArm' = 28, 'LeftArm' = 29, 'LeftLeg' = 30, 'RightLeg' = 31, 'Package' = 32, 'YouTubeVideo' = 33, 'GamePass' = 34, 'App' = 35, 'Code' = 37, 'Plugin' = 38, 'SolidModel' = 39, 'MeshPart' = 40, 'HairAccessory' = 41, 'FaceAccessory' = 42, 'NeckAccessory' = 43, 'ShoulderAccessory' = 44, 'FrontAccessory' = 45, 'BackAccessory' = 46, 'WaistAccessory' = 47, 'ClimbAnimation' = 48, 'DeathAnimation' = 49, 'FallAnimation' = 50, 'IdleAnimation' = 51, 'JumpAnimation' = 52, 'RunAnimation' = 53, 'SwimAnimation' = 54, 'WalkAnimation' = 55, 'PoseAnimation' = 56, 'LocalizationTableManifest' = 59, 'LocalizationTableTranslation' = 60, 'EmoteAnimation' = 61, 'Video' = 62, 'TexturePack' = 63, 'TShirtAccessory' = 64, 'ShirtAccessory' = 65, 'PantsAccessory' = 66, 'JacketAccessory' = 67, 'SweaterAccessory' = 68, 'ShortsAccessory' = 69, 'LeftShoeAccessory' = 70, 'RightShoeAccessory' = 71, 'DressSkirtAccessory' = 72, 'FontFamily' = 73, 'FontFace' = 74, 'MeshHiddenSurfaceRemoval' = 75, 'EyebrowAccessory' = 76, 'EyelashAccessory' = 77, 'MoodAnimation' = 78, 'DynamicHead' = 79, 'CodeSnippet' = 80, 'AdsVideo' = 81, 'OtaUpdate' = 82, 'Screenshot' = 83, 'RuntimePropertySet' = 84, 'StorePreviewVideo' = 85, 'GamePreviewVideo' = 86, 'CreatorExperienceConfig' = 87, 'FaceMakeup' = 88, 'LipMakeup' = 89, 'EyeMakeup' = 90, 'VoxelFragment' = 91, 'AvatarBackground' = 92, 'TextDocument' = 93, 'Post' = 94, 'AnimatedImage' = 95, 'ComputeFunction' = 96\]
    #[serde(rename = "assetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<UserAssetItemModelV2AssetType>,
    /// The created date time of the user asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// The name of asset with id Roblox.Inventory.Api.V2.UserAssetItemModelV2.AssetId.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.Web.WebAPI.ApiEmptyResponseModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiEmptyResponseModel {}

/// `Roblox.Web.WebAPI.Models.ApiPageResponse\[Roblox.Inventory.Api.Models.InventoryItemModel\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiPageResponseInventoryItemModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<InventoryItemModel>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
}

/// `Roblox.Web.WebAPI.Models.ApiPageResponse\[Roblox.Inventory.Api.V2.AssetOwnerResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiPageResponseAssetOwnerResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<AssetOwnerResponse>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
}

/// `Roblox.Web.WebAPI.Models.ApiPageResponse\[Roblox.Inventory.Api.V2.CollectibleItemOwnerResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiPageResponseCollectibleItemOwnerResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<CollectibleItemOwnerResponse>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
}

/// `Roblox.Web.WebAPI.Models.ApiPageResponse\[Roblox.Inventory.Api.V2.UserAssetItemModelV2\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiPageResponseUserAssetItemModelV2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<UserAssetItemModelV2>>,
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
    GetAssetsByAssetIdOwnersSortOrder {
    Asc = 0 => "Asc",
    Desc = 1 => "Desc",
    }
}

/// Query, header and form inputs for [`get_assets_by_asset_id_owners`].
#[derive(Debug, Clone, Default)]
pub struct GetAssetsByAssetIdOwnersParams {
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// Sorted by userAssetId
    pub sort_order: Option<GetAssetsByAssetIdOwnersSortOrder>,
}

/// Query, header and form inputs for [`get_collectible_items_by_collectible_item_id_owners`].
#[derive(Debug, Clone, Default)]
pub struct GetCollectibleItemsByCollectibleItemIdOwnersParams {
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// The order the results are sorted in.
    pub sort_order: Option<i32>,
}

/// Query, header and form inputs for [`get_users_by_user_id_inventory`].
#[derive(Debug, Clone, Default)]
pub struct GetUsersByUserIdInventoryParams {
    /// The asset types to query.
    pub asset_types: Vec<UserAssetItemModelV2AssetType>,
    /// Filters moderated assets when enabled.
    pub filter_disapproved_assets: Option<bool>,
    /// Filters moderated assets and assets pending review when enabled.
    pub show_approved_only: Option<bool>,
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// The order the results are sorted in.
    pub sort_order: Option<GetAssetsByAssetIdOwnersSortOrder>,
}

/// Query, header and form inputs for [`get_users_by_user_id_inventory_by_asset_type_id`].
#[derive(Debug, Clone, Default)]
pub struct GetUsersByUserIdInventoryByAssetTypeIdParams {
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// The order the results are sorted in.
    pub sort_order: Option<GetAssetsByAssetIdOwnersSortOrder>,
}

/// `GET /v2/assets/{assetId}/owners`
///
/// Gets a list of owners of an asset.
pub fn get_assets_by_asset_id_owners_request(
    asset_id: i64,
    params: &GetAssetsByAssetIdOwnersParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v2/assets/{}/owners", crate::rt::path_value(&asset_id)),
    );
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    r.query("sortOrder", &params.sort_order);
    Ok(r)
}

/// `GET /v2/assets/{assetId}/owners`
///
/// Gets a list of owners of an asset.
pub async fn get_assets_by_asset_id_owners(
    client: &Client,
    asset_id: i64,
    params: &GetAssetsByAssetIdOwnersParams,
) -> Result<ApiPageResponseAssetOwnerResponse> {
    client
        .send(get_assets_by_asset_id_owners_request(asset_id, params)?)
        .await?
        .json()
}

/// `GET /v2/collectible-items/{collectibleItemId}/owners`
///
/// Gets a list of owners of a collectible item.
pub fn get_collectible_items_by_collectible_item_id_owners_request(
    collectible_item_id: &str,
    params: &GetCollectibleItemsByCollectibleItemIdOwnersParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v2/collectible-items/{}/owners",
            crate::rt::path_value(&collectible_item_id)
        ),
    );
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    r.query("sortOrder", &params.sort_order);
    Ok(r)
}

/// `GET /v2/collectible-items/{collectibleItemId}/owners`
///
/// Gets a list of owners of a collectible item.
pub async fn get_collectible_items_by_collectible_item_id_owners(
    client: &Client,
    collectible_item_id: &str,
    params: &GetCollectibleItemsByCollectibleItemIdOwnersParams,
) -> Result<ApiPageResponseCollectibleItemOwnerResponse> {
    client
        .send(get_collectible_items_by_collectible_item_id_owners_request(
            collectible_item_id,
            params,
        )?)
        .await?
        .json()
}

/// `DELETE /v2/inventory/asset/{assetId}`
///
/// Give up an asset owned by the authenticated user. Assets that are created by Roblox user or are limited edition are not eligible for deletion and will return NotEligibleForDelete.
pub fn delete_inventory_asset_by_asset_id_request(asset_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::DELETE,
        HOST,
        format!("/v2/inventory/asset/{}", crate::rt::path_value(&asset_id)),
    );
    Ok(r)
}

/// `DELETE /v2/inventory/asset/{assetId}`
///
/// Give up an asset owned by the authenticated user. Assets that are created by Roblox user or are limited edition are not eligible for deletion and will return NotEligibleForDelete.
pub async fn delete_inventory_asset_by_asset_id(
    client: &Client,
    asset_id: i64,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(delete_inventory_asset_by_asset_id_request(asset_id)?)
        .await?
        .json()
}

/// `GET /v2/users/{userId}/inventory`
///
/// Get user's inventory by multiple Roblox.Platform.Assets.AssetType.
///
/// GamePass and Badges not allowed.
pub fn get_users_by_user_id_inventory_request(
    user_id: i64,
    params: &GetUsersByUserIdInventoryParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v2/users/{}/inventory", crate::rt::path_value(&user_id)),
    );
    r.query("assetTypes", &params.asset_types);
    r.query("filterDisapprovedAssets", &params.filter_disapproved_assets);
    r.query("showApprovedOnly", &params.show_approved_only);
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    r.query("sortOrder", &params.sort_order);
    Ok(r)
}

/// `GET /v2/users/{userId}/inventory`
///
/// Get user's inventory by multiple Roblox.Platform.Assets.AssetType.
///
/// GamePass and Badges not allowed.
pub async fn get_users_by_user_id_inventory(
    client: &Client,
    user_id: i64,
    params: &GetUsersByUserIdInventoryParams,
) -> Result<ApiPageResponseUserAssetItemModelV2> {
    client
        .send(get_users_by_user_id_inventory_request(user_id, params)?)
        .await?
        .json()
}

/// `GET /v2/users/{userId}/inventory/{assetTypeId}`
///
/// Gets user's inventory based on specific asset type
pub fn get_users_by_user_id_inventory_by_asset_type_id_request(
    user_id: i64,
    asset_type_id: i32,
    params: &GetUsersByUserIdInventoryByAssetTypeIdParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v2/users/{}/inventory/{}",
            crate::rt::path_value(&user_id),
            crate::rt::path_value(&asset_type_id)
        ),
    );
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    r.query("sortOrder", &params.sort_order);
    Ok(r)
}

/// `GET /v2/users/{userId}/inventory/{assetTypeId}`
///
/// Gets user's inventory based on specific asset type
pub async fn get_users_by_user_id_inventory_by_asset_type_id(
    client: &Client,
    user_id: i64,
    asset_type_id: i32,
    params: &GetUsersByUserIdInventoryByAssetTypeIdParams,
) -> Result<ApiPageResponseInventoryItemModel> {
    client
        .send(get_users_by_user_id_inventory_by_asset_type_id_request(
            user_id,
            asset_type_id,
            params,
        )?)
        .await?
        .json()
}

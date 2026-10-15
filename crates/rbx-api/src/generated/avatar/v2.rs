//! `avatar.roblox.com` v2 — generated from `vendor/roblox-api-tracker/avatar/v2.json`.
//!
//! Do not edit. Change the spec or `rbx-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "avatar.roblox.com";

/// `Roblox.Avatarcore.Shared.V3.AssetPosition`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AssetPosition {
    #[serde(rename = "X")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(rename = "Y")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[serde(rename = "Z")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub z: Option<f64>,
}

/// `Roblox.Avatarcore.Shared.V3.AssetRotation`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AssetRotation {
    #[serde(rename = "X")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(rename = "Y")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[serde(rename = "Z")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub z: Option<f64>,
}

/// `Roblox.Avatarcore.Shared.V3.AssetScale`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AssetScale {
    #[serde(rename = "X")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(rename = "Y")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[serde(rename = "Z")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub z: Option<f64>,
}

/// `Roblox.Api.Avatar.Models.AssetMetaModelV1`
///
/// Exhaustive model denoting all possible metadata fields of an asset
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AssetMetaModelV1 {
    /// Layered-clothing order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<AssetPosition>,
    /// Layered-clothing puffiness
    #[serde(skip_serializing_if = "Option::is_none")]
    pub puffiness: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<AssetRotation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<AssetScale>,
    /// Client-authoritative meta model format version - default is always 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}

/// `Roblox.Api.Avatar.Models.AssetTypeModel`
///
/// A model containing details about an asset type
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AssetTypeModel {
    /// The id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.Api.Avatar.Models.AssetModelV2`
///
/// A model containing details about an asset - V2: adds CurrentVersionId, AssetMetaModel
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AssetModelV2 {
    #[serde(rename = "assetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<AssetTypeModel>,
    /// Id of the current version of asset
    #[serde(rename = "currentVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version_id: Option<i64>,
    /// The id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<AssetMetaModelV1>,
    /// The name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.Api.Avatar.Models.AssetWearModel`
///
/// A model which contains - an asset id - AssetMetaModel
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AssetWearModel {
    /// An asset id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<AssetMetaModelV1>,
}

/// `Roblox.Api.Avatar.Models.AvatarApiSuccessResponse`
///
/// Success response class
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AvatarApiSuccessResponse {
    /// Gets or sets a value indicating whether the request was a success.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

/// `Roblox.Api.Avatar.Models.BodyColors3Model`
///
/// A model containing RGB hex colors for each body part.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BodyColors3Model {
    /// The RGB hex color for head color, e.g. #FFFFFF
    #[serde(rename = "headColor3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_color3: Option<String>,
    /// The RGB hex color for left arm color, e.g. #FFFFFF
    #[serde(rename = "leftArmColor3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_arm_color3: Option<String>,
    /// The RGB hex color for left leg color, e.g. #FFFFFF
    #[serde(rename = "leftLegColor3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_leg_color3: Option<String>,
    /// The RGB hex color for right arm color, e.g. #FFFFFF
    #[serde(rename = "rightArmColor3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_arm_color3: Option<String>,
    /// The RGB hex color for right leg color, e.g. #FFFFFF
    #[serde(rename = "rightLegColor3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_leg_color3: Option<String>,
    /// The RGB hex color for torso color, e.g. #FFFFFF
    #[serde(rename = "torsoColor3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub torso_color3: Option<String>,
}

/// `Roblox.Api.Avatar.Models.EmoteResponseModel`
///
/// Response object representing a user's emote
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EmoteResponseModel {
    /// The asset id of the emote
    #[serde(rename = "assetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<i64>,
    /// The name of the emote
    #[serde(rename = "assetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_name: Option<String>,
    /// The position the emote is equipped to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
}

crate::api_enum! {
    int
    /// The avatar type \['R6' = 1, 'R15' = 3\]
    AvatarModelV3PlayerAvatarType {
    R6 = 1 => "R6",
    R15 = 3 => "R15",
    }
}

/// `Roblox.Web.Responses.Avatar.ScaleModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ScaleModel {
    #[serde(rename = "bodyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_type: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proportion: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
}

/// `Roblox.Api.Avatar.Models.AvatarModelV3`
///
/// A model containing details about an avatar
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AvatarModelV3 {
    /// The assets worn on the character
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<AssetModelV2>>,
    #[serde(rename = "bodyColor3s")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_color3s: Option<BodyColors3Model>,
    /// Whether default clothing has been applied to this avatar.
    #[serde(rename = "defaultPantsApplied")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_pants_applied: Option<bool>,
    /// Whether default clothing has been applied to this avatar.
    #[serde(rename = "defaultShirtApplied")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_shirt_applied: Option<bool>,
    /// The emotes on the character
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emotes: Option<Vec<EmoteResponseModel>>,
    /// The avatar type \['R6' = 1, 'R15' = 3\]
    #[serde(rename = "playerAvatarType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_avatar_type: Option<AvatarModelV3PlayerAvatarType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scales: Option<ScaleModel>,
}

/// `Roblox.Api.Avatar.Models.OutfitModel`
///
/// A slim model for user outfits.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct OutfitModel {
    /// The id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Whether the outfit can be modified by the user.
    #[serde(rename = "isEditable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_editable: Option<bool>,
    /// The name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The type of the Outfit.
    #[serde(rename = "outfitType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outfit_type: Option<String>,
}

/// `Roblox.Api.Avatar.Models.AvatarPageResponse\[Roblox.Api.Avatar.Models.OutfitModel\]`
///
/// Avatar page response.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AvatarPageResponseOutfitModel {
    /// The data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<OutfitModel>>,
    /// A non empty string indicates that there is more data available than this response contains. An empty string indicates the last page.
    #[serde(rename = "paginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
}

/// `Roblox.Api.Avatar.Models.BodyColorsModel`
///
/// A model container BrickColor ids for each body part.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BodyColorsModel {
    /// The BrickColor id for head color
    #[serde(rename = "headColorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_color_id: Option<i32>,
    /// The BrickColor id for left arm color
    #[serde(rename = "leftArmColorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_arm_color_id: Option<i32>,
    /// The BrickColor id for left leg color
    #[serde(rename = "leftLegColorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_leg_color_id: Option<i32>,
    /// The BrickColor id for right arm color
    #[serde(rename = "rightArmColorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_arm_color_id: Option<i32>,
    /// The BrickColor id for right leg color
    #[serde(rename = "rightLegColorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_leg_color_id: Option<i32>,
    /// The BrickColor id for torso color
    #[serde(rename = "torsoColorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub torso_color_id: Option<i32>,
}

crate::api_enum! {
    int
    /// The type of outfit \['Invalid' = 0, 'Avatar' = 1, 'DynamicHead' = 2, 'InExperienceBody' = 3\]
    OutfitUpdateModelV2OutfitType {
    Invalid = 0 => "Invalid",
    Avatar = 1 => "Avatar",
    DynamicHead = 2 => "DynamicHead",
    InExperienceBody = 3 => "InExperienceBody",
    }
}

/// `Roblox.Api.Avatar.Models.OutfitUpdateModelV2`
///
/// A model containing details needed to update or create an outfit
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct OutfitUpdateModelV2 {
    /// Array of assets
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<AssetWearModel>>,
    #[serde(rename = "bodyColors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_colors: Option<BodyColorsModel>,
    /// The outfit name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The type of outfit \['Invalid' = 0, 'Avatar' = 1, 'DynamicHead' = 2, 'InExperienceBody' = 3\]
    #[serde(rename = "outfitType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outfit_type: Option<OutfitUpdateModelV2OutfitType>,
    /// The avatar scale
    #[serde(rename = "playerAvatarType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_avatar_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<ScaleModel>,
}

/// `Roblox.Api.Avatar.Models.WearRequestModel`
///
/// A model that contains a list of AssetWear models
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct WearRequestModel {
    /// The asset ids
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<AssetWearModel>>,
}

/// `Roblox.Api.Avatar.Models.WearResponseModel`
///
/// A model for wear outfit responses
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct WearResponseModel {
    /// The asset ids that could not be worn
    #[serde(rename = "invalidAssetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_asset_ids: Option<Vec<i64>>,
    /// The assets that could not be worn Unlike invalidAssetIds, only contains assets that are wearable types
    #[serde(rename = "invalidAssets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_assets: Option<Vec<AssetModelV2>>,
    /// Whether or not all the outfit contents were successfully worn
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

/// `Roblox.Platform.Avatar.BodyColorsModelV2`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BodyColorsModelV2 {
    #[serde(rename = "headColor3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_color3: Option<String>,
    #[serde(rename = "leftArmColor3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_arm_color3: Option<String>,
    #[serde(rename = "leftLegColor3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_leg_color3: Option<String>,
    #[serde(rename = "rightArmColor3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_arm_color3: Option<String>,
    #[serde(rename = "rightLegColor3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_leg_color3: Option<String>,
    #[serde(rename = "torsoColor3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub torso_color3: Option<String>,
}

/// `Roblox.Web.WebAPI.ApiEmptyResponseModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiEmptyResponseModel {}

/// Query, header and form inputs for [`get_avatar_users_by_user_id_outfits`].
#[derive(Debug, Clone, Default)]
pub struct GetAvatarUsersByUserIdOutfitsParams {
    /// The token received from the response to get the next page. For the first request, this value should be empty. Note : If no value is sent the 1st page will be returned.
    pub pagination_token: Option<String>,
    /// The outfit type being searched for, null will return all outfitTypes
    pub outfit_type: Option<String>,
    /// The page number of the current page of requests, default is 1.
    pub page: Option<i32>,
    /// The max number of outfits that can be returned.
    pub items_per_page: Option<i32>,
    /// Whether the outfits are editable. A null value will lead to no filtering.
    pub is_editable: Option<bool>,
}

/// `GET /v2/avatar/avatar`
///
/// Returns details about the authenticated user's avatar
pub fn get_avatar_avatar_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v2/avatar/avatar".to_string());
    Ok(r)
}

/// `GET /v2/avatar/avatar`
///
/// Returns details about the authenticated user's avatar
pub async fn get_avatar_avatar(client: &Client) -> Result<AvatarModelV3> {
    client.send(get_avatar_avatar_request()?).await?.json()
}

/// `POST /v2/avatar/set-body-colors`
///
/// Sets the authenticated user's body colors
pub fn post_avatar_set_body_colors_request(body: &BodyColorsModelV2) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v2/avatar/set-body-colors".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/avatar/set-body-colors`
///
/// Sets the authenticated user's body colors
pub async fn post_avatar_set_body_colors(
    client: &Client,
    body: &BodyColorsModelV2,
) -> Result<AvatarApiSuccessResponse> {
    client
        .send(post_avatar_set_body_colors_request(body)?)
        .await?
        .json()
}

/// `POST /v2/avatar/set-wearing-assets`
///
/// Sets the avatar's current assets to the list
///
/// Only allows items that you own, are not expired, and are wearable asset types. Any assets being worn before this method is called are automatically removed.
pub fn post_avatar_set_wearing_assets_request(body: &WearRequestModel) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v2/avatar/set-wearing-assets".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/avatar/set-wearing-assets`
///
/// Sets the avatar's current assets to the list
///
/// Only allows items that you own, are not expired, and are wearable asset types. Any assets being worn before this method is called are automatically removed.
pub async fn post_avatar_set_wearing_assets(
    client: &Client,
    body: &WearRequestModel,
) -> Result<WearResponseModel> {
    client
        .send(post_avatar_set_wearing_assets_request(body)?)
        .await?
        .json()
}

/// `GET /v2/avatar/users/{userId}/avatar`
///
/// Returns details about a specified user's avatar
///
/// Includes assets, bodycolors, and playerAvatarType.
pub fn get_avatar_users_by_user_id_avatar_request(user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v2/avatar/users/{}/avatar", crate::rt::path_value(&user_id)),
    );
    Ok(r)
}

/// `GET /v2/avatar/users/{userId}/avatar`
///
/// Returns details about a specified user's avatar
///
/// Includes assets, bodycolors, and playerAvatarType.
pub async fn get_avatar_users_by_user_id_avatar(client: &Client, user_id: i64) -> Result<AvatarModelV3> {
    client
        .send(get_avatar_users_by_user_id_avatar_request(user_id)?)
        .await?
        .json()
}

/// `GET /v2/avatar/users/{userId}/outfits`
///
/// Gets a list of outfits for the specified user.
pub fn get_avatar_users_by_user_id_outfits_request(
    user_id: i64,
    params: &GetAvatarUsersByUserIdOutfitsParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v2/avatar/users/{}/outfits", crate::rt::path_value(&user_id)),
    );
    r.query("paginationToken", &params.pagination_token);
    r.query("outfitType", &params.outfit_type);
    r.query("page", &params.page);
    r.query("itemsPerPage", &params.items_per_page);
    r.query("isEditable", &params.is_editable);
    Ok(r)
}

/// `GET /v2/avatar/users/{userId}/outfits`
///
/// Gets a list of outfits for the specified user.
pub async fn get_avatar_users_by_user_id_outfits(
    client: &Client,
    user_id: i64,
    params: &GetAvatarUsersByUserIdOutfitsParams,
) -> Result<AvatarPageResponseOutfitModel> {
    client
        .send(get_avatar_users_by_user_id_outfits_request(user_id, params)?)
        .await?
        .json()
}

/// `POST /v2/outfits/create`
///
/// Creates a new outfit.
///
/// Fails if any of the assetIds are not owned by the user, or not wearable types. The name property of the request is optional as one will be auto-generated when the request has a null name.
pub fn post_outfits_create_request(body: &OutfitUpdateModelV2) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v2/outfits/create".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/outfits/create`
///
/// Creates a new outfit.
///
/// Fails if any of the assetIds are not owned by the user, or not wearable types. The name property of the request is optional as one will be auto-generated when the request has a null name.
pub async fn post_outfits_create(
    client: &Client,
    body: &OutfitUpdateModelV2,
) -> Result<ApiEmptyResponseModel> {
    client.send(post_outfits_create_request(body)?).await?.json()
}

/// `PATCH /v2/outfits/{userOutfitId}`
///
/// Updates the contents of an outfit.
///
/// Fails if the user does not own any of the assetIds or if they are not wearable asset types. Accepts partial updates.
#[deprecated(note = "marked deprecated by Roblox")]
pub fn patch_outfits_by_user_outfit_id_request(
    user_outfit_id: i64,
    body: &OutfitUpdateModelV2,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!("/v2/outfits/{}", crate::rt::path_value(&user_outfit_id)),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v2/outfits/{userOutfitId}`
///
/// Updates the contents of an outfit.
///
/// Fails if the user does not own any of the assetIds or if they are not wearable asset types. Accepts partial updates.
#[deprecated(note = "marked deprecated by Roblox")]
pub async fn patch_outfits_by_user_outfit_id(
    client: &Client,
    user_outfit_id: i64,
    body: &OutfitUpdateModelV2,
) -> Result<OutfitModel> {
    client
        .send(patch_outfits_by_user_outfit_id_request(user_outfit_id, body)?)
        .await?
        .json()
}

/// `POST /v2/outfits/{userOutfitId}/update`
///
/// Updates the contents of the outfit.
///
/// Fails if the user does not own any of the assetIds or if they are not wearable asset types.
pub fn post_outfits_by_user_outfit_id_update_request(
    user_outfit_id: i64,
    body: &OutfitUpdateModelV2,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!("/v2/outfits/{}/update", crate::rt::path_value(&user_outfit_id)),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/outfits/{userOutfitId}/update`
///
/// Updates the contents of the outfit.
///
/// Fails if the user does not own any of the assetIds or if they are not wearable asset types.
pub async fn post_outfits_by_user_outfit_id_update(
    client: &Client,
    user_outfit_id: i64,
    body: &OutfitUpdateModelV2,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_outfits_by_user_outfit_id_update_request(
            user_outfit_id,
            body,
        )?)
        .await?
        .json()
}

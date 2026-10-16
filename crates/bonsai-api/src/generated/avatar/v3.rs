//! `avatar.roblox.com` v3 — generated from `vendor/roblox-api-tracker/avatar/v3.json`.
//!
//! Do not edit. Change the spec or `bonsai-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "avatar.roblox.com";

/// `Roblox.Api.Avatar.Models.AssetPosition`
///
/// A model which contains accessory position coordinates.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AssetPosition {
    /// X coordinate of the position.
    #[serde(rename = "X")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    /// y coordinate of the position.
    #[serde(rename = "Y")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    /// Z coordinate of the position.
    #[serde(rename = "Z")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub z: Option<f64>,
}

/// `Roblox.Api.Avatar.Models.AssetRotation`
///
/// A model which contains accessory rotation coordinates.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AssetRotation {
    /// X degrees of the rotation.
    #[serde(rename = "X")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    /// Y degrees of the rotation.
    #[serde(rename = "Y")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    /// Z degrees of the rotation.
    #[serde(rename = "Z")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub z: Option<f64>,
}

/// `Roblox.Api.Avatar.Models.AssetScale`
///
/// A model which contains accessory scale.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AssetScale {
    /// X scale multiplier of accessory.
    #[serde(rename = "X")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    /// Y scale multiplier of accessory.
    #[serde(rename = "Y")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    /// Z scale multiplier of accessory.
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
    /// Head Shape selected for the asset id. Applicable for dynamic head assets.
    #[serde(rename = "headShape")]
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_shape: Option<i32>,
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
    /// Indicates user choice for facial animation. staticFacialAnimation=false, implies the toggle as on and face will animate. Applicable for dynamic head assets.
    #[serde(rename = "staticFacialAnimation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_facial_animation: Option<bool>,
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
    pub id: Option<i32>,
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
    /// Asset availability status.
    #[serde(rename = "availabilityStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_status: Option<String>,
    /// Id of the current version of asset
    #[serde(rename = "currentVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version_id: Option<i64>,
    /// For rental assets only. (Future) ownership expiration time of the asset.
    #[serde(rename = "expirationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    /// The id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<AssetMetaModelV1>,
    /// The name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// If the "Id" is swappable, applicable for DH assets.
    #[serde(rename = "supportsHeadShapes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_head_shapes: Option<bool>,
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

/// `Roblox.Api.Avatar.Models.OutfitDetailsModelV2`
///
/// A model containing details about a user outfit
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct OutfitDetailsModelV2 {
    /// A list of assetIds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<AssetModelV2>>,
    #[serde(rename = "bodyColor3s")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_color3s: Option<BodyColors3Model>,
    /// The bundle ID, currently only returned for in-experience created outfits.
    #[serde(rename = "bundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<i64>,
    /// The id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The inventory type of the outfit.
    #[serde(rename = "inventoryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_type: Option<String>,
    /// Whether the outfit can be edited by the user
    #[serde(rename = "isEditable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_editable: Option<bool>,
    /// The moderation status of the outfit, not applicable when outfit is created outside experience
    #[serde(rename = "moderationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation_status: Option<String>,
    /// The name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The outfit type of the outfit
    #[serde(rename = "outfitType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outfit_type: Option<String>,
    /// The player avatar type - this can be R6 or R15.
    #[serde(rename = "playerAvatarType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_avatar_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<ScaleModel>,
    /// The universe id of the outfit, null when outfit is not created in-experience
    #[serde(rename = "universeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_id: Option<i64>,
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

/// `Roblox.Api.Avatar.Models.OutfitUpdateModelV3`
///
/// A model containing details needed to update or create an outfit.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct OutfitUpdateModelV3 {
    /// Array of assets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<AssetWearModel>>,
    #[serde(rename = "bodyColor3s")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_color3s: Option<BodyColors3Model>,
    /// The outfit name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The type of outfit.
    #[serde(rename = "outfitType")]
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outfit_type: Option<i32>,
    /// The avatar scale.
    #[serde(rename = "playerAvatarType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_avatar_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<ScaleModel>,
}

/// Query, header and form inputs for [`post_outfits_create`].
#[derive(Debug, Clone, Default)]
pub struct PostOutfitsCreateParams {
    pub roblox_place_id: Option<i64>,
}

/// Query, header and form inputs for [`patch_outfits_by_user_outfit_id`].
#[derive(Debug, Clone, Default)]
pub struct PatchOutfitsByUserOutfitIdParams {
    pub roblox_place_id: Option<i64>,
}

/// Query, header and form inputs for [`get_outfits_by_user_outfit_id_details`].
#[derive(Debug, Clone, Default)]
pub struct GetOutfitsByUserOutfitIdDetailsParams {
    pub roblox_place_id: Option<i64>,
    /// Whether to return assets with availability status.
    pub check_asset_availability: Option<bool>,
}

/// `POST /v3/outfits/create`
///
/// Creates a new outfit.
///
/// Fails if any of the assetIds are not owned by the user, or not wearable types. The name property of the request is optional as one will be auto-generated when the request has a null name.
#[deprecated(note = "marked deprecated by Roblox")]
pub fn post_outfits_create_request(
    params: &PostOutfitsCreateParams,
    body: &OutfitUpdateModelV3,
) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v3/outfits/create".to_string());
    r.header("Roblox-Place-Id", &params.roblox_place_id);
    r.json(body)?;
    Ok(r)
}

/// `POST /v3/outfits/create`
///
/// Creates a new outfit.
///
/// Fails if any of the assetIds are not owned by the user, or not wearable types. The name property of the request is optional as one will be auto-generated when the request has a null name.
#[deprecated(note = "marked deprecated by Roblox")]
pub async fn post_outfits_create(
    client: &Client,
    params: &PostOutfitsCreateParams,
    body: &OutfitUpdateModelV3,
) -> Result<OutfitModel> {
    client
        .send(post_outfits_create_request(params, body)?)
        .await?
        .json()
}

/// `PATCH /v3/outfits/{userOutfitId}`
///
/// Updates the contents of an outfit.
///
/// Fails if the user does not own any of the assetIds or if they are not wearable asset types. Accepts partial updates.
#[deprecated(note = "marked deprecated by Roblox")]
pub fn patch_outfits_by_user_outfit_id_request(
    user_outfit_id: i64,
    params: &PatchOutfitsByUserOutfitIdParams,
    body: &OutfitUpdateModelV3,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!("/v3/outfits/{}", crate::rt::path_value(&user_outfit_id)),
    );
    r.header("Roblox-Place-Id", &params.roblox_place_id);
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v3/outfits/{userOutfitId}`
///
/// Updates the contents of an outfit.
///
/// Fails if the user does not own any of the assetIds or if they are not wearable asset types. Accepts partial updates.
#[deprecated(note = "marked deprecated by Roblox")]
pub async fn patch_outfits_by_user_outfit_id(
    client: &Client,
    user_outfit_id: i64,
    params: &PatchOutfitsByUserOutfitIdParams,
    body: &OutfitUpdateModelV3,
) -> Result<OutfitModel> {
    client
        .send(patch_outfits_by_user_outfit_id_request(
            user_outfit_id,
            params,
            body,
        )?)
        .await?
        .json()
}

/// `GET /v3/outfits/{userOutfitId}/details`
///
/// Gets details about the contents of an outfit.
#[deprecated(note = "marked deprecated by Roblox")]
pub fn get_outfits_by_user_outfit_id_details_request(
    user_outfit_id: i64,
    params: &GetOutfitsByUserOutfitIdDetailsParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v3/outfits/{}/details", crate::rt::path_value(&user_outfit_id)),
    );
    r.header("Roblox-Place-Id", &params.roblox_place_id);
    r.query("checkAssetAvailability", &params.check_asset_availability);
    Ok(r)
}

/// `GET /v3/outfits/{userOutfitId}/details`
///
/// Gets details about the contents of an outfit.
#[deprecated(note = "marked deprecated by Roblox")]
pub async fn get_outfits_by_user_outfit_id_details(
    client: &Client,
    user_outfit_id: i64,
    params: &GetOutfitsByUserOutfitIdDetailsParams,
) -> Result<OutfitDetailsModelV2> {
    client
        .send(get_outfits_by_user_outfit_id_details_request(
            user_outfit_id,
            params,
        )?)
        .await?
        .json()
}

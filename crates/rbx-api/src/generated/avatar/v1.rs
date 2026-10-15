//! `avatar.roblox.com` v1 — generated from `vendor/roblox-api-tracker/avatar/v1.json`.
//!
//! Do not edit. Change the spec or `rbx-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "avatar.roblox.com";

/// `Roblox.Api.Avatar.Controllers.V1.AvatarController.EmptyResponse`
///
/// Empty response, will be in WebAPI base soon
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EmptyResponse {}

/// `Roblox.Api.Avatar.Models.AssetIdListModel`
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

/// `Roblox.Api.Avatar.Models.AssetTypeRulesModel`
///
/// A model containing details about an asset type and its business rules
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AssetTypeRulesModel {
    /// The id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The max number of this asset type that can be worn
    #[serde(rename = "maxNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_number: Option<i32>,
    /// The name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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

/// `Roblox.Api.Avatar.Models.AvatarFilteredPageResponse\[Roblox.Api.Avatar.Models.OutfitModel\]`
///
/// Filtered page response
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AvatarFilteredPageResponseOutfitModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<OutfitModel>>,
    /// Number of !:TPagedObject filtered.
    #[serde(rename = "filteredCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filtered_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

/// `Roblox.Api.Avatar.Models.AvatarMetadataModel`
///
/// A model containing website metadata for avatars
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AvatarMetadataModel {
    /// Flag for both web UI and App, name is fixed due to sharing, do not change
    #[serde(rename = "LCEnabledInEditorAndCatalog")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lc_enabled_in_editor_and_catalog: Option<bool>,
    /// Whether or not 3D thumbnails are shown
    #[serde(rename = "areThreeDeeThumbsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub are_three_dee_thumbs_enabled: Option<bool>,
    /// Whether or not to show the Default Clothing message
    #[serde(rename = "enableDefaultClothingMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_default_clothing_message: Option<bool>,
    /// Whether or not the Scales is embedded in the tab
    #[serde(rename = "isAvatarScaleEmbeddedInTab")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_avatar_scale_embedded_in_tab: Option<bool>,
    /// Does the frontend lock avatar editor input until the wearing call returns
    #[serde(rename = "isAvatarWearingApiCallsLockingOnFrontendEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_avatar_wearing_api_calls_locking_on_frontend_enabled: Option<bool>,
    /// Whether or not the Boby Type scale is embedded in the tab
    #[serde(rename = "isBodyTypeScaleOutOfTab")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_body_type_scale_out_of_tab: Option<bool>,
    /// Determines whether Category Reorg is released
    #[serde(rename = "isCategoryReorgEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_category_reorg_enabled: Option<bool>,
    /// Determines whether a bunch of UI improvements are released
    #[serde(rename = "isJustinUiChangesEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_justin_ui_changes_enabled: Option<bool>,
    /// Useful for the time between enabling Jackets for most users and all LC types for everyone, meanwhile Soothsayers need all types at all times
    #[serde(rename = "isLCCompletelyEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_lc_completely_enabled: Option<bool>,
    /// Does the frontend lock avatar editor input until the wearing call returns
    #[serde(rename = "isOutfitHandlingOnFrontendEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_outfit_handling_on_frontend_enabled: Option<bool>,
    /// How much the body type scaler should increment by
    #[serde(rename = "scaleBodyTypeIncrement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_body_type_increment: Option<f64>,
    /// How much the head scaler should increment by
    #[serde(rename = "scaleHeadIncrement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_head_increment: Option<f64>,
    /// How much the height scaler should increment by
    #[serde(rename = "scaleHeightIncrement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_height_increment: Option<f64>,
    /// How much the proportion scaler should increment by
    #[serde(rename = "scaleProportionIncrement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_proportion_increment: Option<f64>,
    /// How much the width scaler should increment by
    #[serde(rename = "scaleWidthIncrement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_width_increment: Option<f64>,
    /// Whether or not to show the default clothing message when the page loads
    #[serde(rename = "showDefaultClothingMessageOnPageLoad")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_default_clothing_message_on_page_load: Option<bool>,
    /// Whether or not to support proportion and body type
    #[serde(rename = "supportProportionAndBodyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_proportion_and_body_type: Option<bool>,
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
    AvatarModelV2PlayerAvatarType {
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

/// `Roblox.Api.Avatar.Models.AvatarModelV2`
///
/// A model containing details about an avatar
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AvatarModelV2 {
    /// The assets worn on the character
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<AssetModelV2>>,
    #[serde(rename = "bodyColors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_colors: Option<BodyColorsModel>,
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
    pub player_avatar_type: Option<AvatarModelV2PlayerAvatarType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scales: Option<ScaleModel>,
}

/// `Roblox.Avatarcore.Common.Models.AccessoryPositionModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AccessoryPositionModel {
    #[serde(rename = "xPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_position: Option<f64>,
    #[serde(rename = "yPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_position: Option<f64>,
    #[serde(rename = "zPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub z_position: Option<f64>,
}

/// `Roblox.Avatarcore.Common.Models.AccessoryRotationModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AccessoryRotationModel {
    #[serde(rename = "xRotation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_rotation: Option<f64>,
    #[serde(rename = "yRotation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_rotation: Option<f64>,
    #[serde(rename = "zRotation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub z_rotation: Option<f64>,
}

/// `Roblox.Avatarcore.Common.Models.AccessoryScaleModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AccessoryScaleModel {
    #[serde(rename = "xScale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_scale: Option<f64>,
    #[serde(rename = "yScale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_scale: Option<f64>,
    #[serde(rename = "zScale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub z_scale: Option<f64>,
}

/// `Roblox.Avatarcore.Common.Models.AccessoryRefinementModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AccessoryRefinementModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<AccessoryPositionModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<AccessoryRotationModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<AccessoryScaleModel>,
}

/// The lower bounds for accessory refinement settings.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AvatarRulesModelAccessoryRefinementLowerBounds {
    #[serde(rename = "AdsVideo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads_video: Option<AccessoryRefinementModel>,
    #[serde(rename = "Animation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<AccessoryRefinementModel>,
    #[serde(rename = "App")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<AccessoryRefinementModel>,
    #[serde(rename = "Arms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arms: Option<AccessoryRefinementModel>,
    #[serde(rename = "Audio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<AccessoryRefinementModel>,
    #[serde(rename = "Avatar")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<AccessoryRefinementModel>,
    #[serde(rename = "BackAccessory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back_accessory: Option<AccessoryRefinementModel>,
    #[serde(rename = "Badge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge: Option<AccessoryRefinementModel>,
    #[serde(rename = "ClimbAnimation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub climb_animation: Option<AccessoryRefinementModel>,
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<AccessoryRefinementModel>,
    #[serde(rename = "CodeSnippet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_snippet: Option<AccessoryRefinementModel>,
    #[serde(rename = "DeathAnimation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub death_animation: Option<AccessoryRefinementModel>,
    #[serde(rename = "Decal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decal: Option<AccessoryRefinementModel>,
    #[serde(rename = "DressSkirtAccessory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dress_skirt_accessory: Option<AccessoryRefinementModel>,
    #[serde(rename = "DynamicHead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_head: Option<AccessoryRefinementModel>,
    #[serde(rename = "EmoteAnimation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emote_animation: Option<AccessoryRefinementModel>,
    #[serde(rename = "EyebrowAccessory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eyebrow_accessory: Option<AccessoryRefinementModel>,
    #[serde(rename = "EyelashAccessory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eyelash_accessory: Option<AccessoryRefinementModel>,
    #[serde(rename = "Face")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<AccessoryRefinementModel>,
    #[serde(rename = "FaceAccessory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_accessory: Option<AccessoryRefinementModel>,
    #[serde(rename = "FallAnimation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fall_animation: Option<AccessoryRefinementModel>,
    #[serde(rename = "FontFace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_face: Option<AccessoryRefinementModel>,
    #[serde(rename = "FontFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<AccessoryRefinementModel>,
    #[serde(rename = "FrontAccessory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front_accessory: Option<AccessoryRefinementModel>,
    #[serde(rename = "GamePass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_pass: Option<AccessoryRefinementModel>,
    #[serde(rename = "Gear")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gear: Option<AccessoryRefinementModel>,
    #[serde(rename = "GroupEmblem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_emblem: Option<AccessoryRefinementModel>,
    #[serde(rename = "HTML")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<AccessoryRefinementModel>,
    #[serde(rename = "HairAccessory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hair_accessory: Option<AccessoryRefinementModel>,
    #[serde(rename = "Hat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hat: Option<AccessoryRefinementModel>,
    #[serde(rename = "Head")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<AccessoryRefinementModel>,
    #[serde(rename = "IdleAnimation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_animation: Option<AccessoryRefinementModel>,
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<AccessoryRefinementModel>,
    #[serde(rename = "JacketAccessory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jacket_accessory: Option<AccessoryRefinementModel>,
    #[serde(rename = "JumpAnimation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jump_animation: Option<AccessoryRefinementModel>,
    #[serde(rename = "LeftArm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_arm: Option<AccessoryRefinementModel>,
    #[serde(rename = "LeftLeg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_leg: Option<AccessoryRefinementModel>,
    #[serde(rename = "LeftShoeAccessory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_shoe_accessory: Option<AccessoryRefinementModel>,
    #[serde(rename = "Legs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legs: Option<AccessoryRefinementModel>,
    #[serde(rename = "LocalizationTableManifest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localization_table_manifest: Option<AccessoryRefinementModel>,
    #[serde(rename = "LocalizationTableTranslation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localization_table_translation: Option<AccessoryRefinementModel>,
    #[serde(rename = "Lua")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lua: Option<AccessoryRefinementModel>,
    #[serde(rename = "Mesh")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh: Option<AccessoryRefinementModel>,
    #[serde(rename = "MeshHiddenSurfaceRemoval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_hidden_surface_removal: Option<AccessoryRefinementModel>,
    #[serde(rename = "MeshPart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_part: Option<AccessoryRefinementModel>,
    #[serde(rename = "Model")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<AccessoryRefinementModel>,
    #[serde(rename = "MoodAnimation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mood_animation: Option<AccessoryRefinementModel>,
    #[serde(rename = "NeckAccessory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neck_accessory: Option<AccessoryRefinementModel>,
    #[serde(rename = "OtaUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ota_update: Option<AccessoryRefinementModel>,
    #[serde(rename = "Package")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<AccessoryRefinementModel>,
    #[serde(rename = "Pants")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pants: Option<AccessoryRefinementModel>,
    #[serde(rename = "PantsAccessory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pants_accessory: Option<AccessoryRefinementModel>,
    #[serde(rename = "Place")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place: Option<AccessoryRefinementModel>,
    #[serde(rename = "Plugin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin: Option<AccessoryRefinementModel>,
    #[serde(rename = "PoseAnimation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pose_animation: Option<AccessoryRefinementModel>,
    #[serde(rename = "RightArm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_arm: Option<AccessoryRefinementModel>,
    #[serde(rename = "RightLeg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_leg: Option<AccessoryRefinementModel>,
    #[serde(rename = "RightShoeAccessory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_shoe_accessory: Option<AccessoryRefinementModel>,
    #[serde(rename = "RunAnimation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_animation: Option<AccessoryRefinementModel>,
    #[serde(rename = "Shirt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shirt: Option<AccessoryRefinementModel>,
    #[serde(rename = "ShirtAccessory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shirt_accessory: Option<AccessoryRefinementModel>,
    #[serde(rename = "ShortsAccessory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shorts_accessory: Option<AccessoryRefinementModel>,
    #[serde(rename = "ShoulderAccessory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shoulder_accessory: Option<AccessoryRefinementModel>,
    #[serde(rename = "SolidModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solid_model: Option<AccessoryRefinementModel>,
    #[serde(rename = "SweaterAccessory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sweater_accessory: Option<AccessoryRefinementModel>,
    #[serde(rename = "SwimAnimation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swim_animation: Option<AccessoryRefinementModel>,
    #[serde(rename = "TShirt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_shirt: Option<AccessoryRefinementModel>,
    #[serde(rename = "TShirtAccessory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_shirt_accessory: Option<AccessoryRefinementModel>,
    #[serde(rename = "Text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<AccessoryRefinementModel>,
    #[serde(rename = "TexturePack")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texture_pack: Option<AccessoryRefinementModel>,
    #[serde(rename = "Torso")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub torso: Option<AccessoryRefinementModel>,
    #[serde(rename = "Video")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<AccessoryRefinementModel>,
    #[serde(rename = "WaistAccessory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waist_accessory: Option<AccessoryRefinementModel>,
    #[serde(rename = "WalkAnimation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub walk_animation: Option<AccessoryRefinementModel>,
    #[serde(rename = "YouTubeVideo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub you_tube_video: Option<AccessoryRefinementModel>,
}

crate::api_enum! {
    int
    /// \['Image' = 1, 'TShirt' = 2, 'Audio' = 3, 'Mesh' = 4, 'Lua' = 5, 'HTML' = 6, 'Text' = 7, 'Hat' = 8, 'Place' = 9, 'Model' = 10, 'Shirt' = 11, 'Pants' = 12, 'Decal' = 13, 'Avatar' = 16, 'Head' = 17, 'Face' = 18, 'Gear' = 19, 'Badge' = 21, 'GroupEmblem' = 22, 'Animation' = 24, 'Arms' = 25, 'Legs' = 26, 'Torso' = 27, 'RightArm' = 28, 'LeftArm' = 29, 'LeftLeg' = 30, 'RightLeg' = 31, 'Package' = 32, 'YouTubeVideo' = 33, 'GamePass' = 34, 'App' = 35, 'Code' = 37, 'Plugin' = 38, 'SolidModel' = 39, 'MeshPart' = 40, 'HairAccessory' = 41, 'FaceAccessory' = 42, 'NeckAccessory' = 43, 'ShoulderAccessory' = 44, 'FrontAccessory' = 45, 'BackAccessory' = 46, 'WaistAccessory' = 47, 'ClimbAnimation' = 48, 'DeathAnimation' = 49, 'FallAnimation' = 50, 'IdleAnimation' = 51, 'JumpAnimation' = 52, 'RunAnimation' = 53, 'SwimAnimation' = 54, 'WalkAnimation' = 55, 'PoseAnimation' = 56, 'LocalizationTableManifest' = 59, 'LocalizationTableTranslation' = 60, 'EmoteAnimation' = 61, 'Video' = 62, 'TexturePack' = 63, 'TShirtAccessory' = 64, 'ShirtAccessory' = 65, 'PantsAccessory' = 66, 'JacketAccessory' = 67, 'SweaterAccessory' = 68, 'ShortsAccessory' = 69, 'LeftShoeAccessory' = 70, 'RightShoeAccessory' = 71, 'DressSkirtAccessory' = 72, 'FontFamily' = 73, 'FontFace' = 74, 'MeshHiddenSurfaceRemoval' = 75, 'EyebrowAccessory' = 76, 'EyelashAccessory' = 77, 'MoodAnimation' = 78, 'DynamicHead' = 79, 'CodeSnippet' = 80, 'AdsVideo' = 81, 'OtaUpdate' = 82\]
    AvatarRulesModelAccessoryRefinementTypes {
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
    }
}

/// The lower bounds for accessory refinement settings.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AvatarRulesModelAccessoryRefinementUpperBounds {
    #[serde(rename = "AdsVideo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads_video: Option<AccessoryRefinementModel>,
    #[serde(rename = "Animation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<AccessoryRefinementModel>,
    #[serde(rename = "App")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<AccessoryRefinementModel>,
    #[serde(rename = "Arms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arms: Option<AccessoryRefinementModel>,
    #[serde(rename = "Audio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<AccessoryRefinementModel>,
    #[serde(rename = "Avatar")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<AccessoryRefinementModel>,
    #[serde(rename = "BackAccessory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back_accessory: Option<AccessoryRefinementModel>,
    #[serde(rename = "Badge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge: Option<AccessoryRefinementModel>,
    #[serde(rename = "ClimbAnimation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub climb_animation: Option<AccessoryRefinementModel>,
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<AccessoryRefinementModel>,
    #[serde(rename = "CodeSnippet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_snippet: Option<AccessoryRefinementModel>,
    #[serde(rename = "DeathAnimation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub death_animation: Option<AccessoryRefinementModel>,
    #[serde(rename = "Decal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decal: Option<AccessoryRefinementModel>,
    #[serde(rename = "DressSkirtAccessory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dress_skirt_accessory: Option<AccessoryRefinementModel>,
    #[serde(rename = "DynamicHead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_head: Option<AccessoryRefinementModel>,
    #[serde(rename = "EmoteAnimation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emote_animation: Option<AccessoryRefinementModel>,
    #[serde(rename = "EyebrowAccessory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eyebrow_accessory: Option<AccessoryRefinementModel>,
    #[serde(rename = "EyelashAccessory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eyelash_accessory: Option<AccessoryRefinementModel>,
    #[serde(rename = "Face")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<AccessoryRefinementModel>,
    #[serde(rename = "FaceAccessory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_accessory: Option<AccessoryRefinementModel>,
    #[serde(rename = "FallAnimation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fall_animation: Option<AccessoryRefinementModel>,
    #[serde(rename = "FontFace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_face: Option<AccessoryRefinementModel>,
    #[serde(rename = "FontFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<AccessoryRefinementModel>,
    #[serde(rename = "FrontAccessory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front_accessory: Option<AccessoryRefinementModel>,
    #[serde(rename = "GamePass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_pass: Option<AccessoryRefinementModel>,
    #[serde(rename = "Gear")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gear: Option<AccessoryRefinementModel>,
    #[serde(rename = "GroupEmblem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_emblem: Option<AccessoryRefinementModel>,
    #[serde(rename = "HTML")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<AccessoryRefinementModel>,
    #[serde(rename = "HairAccessory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hair_accessory: Option<AccessoryRefinementModel>,
    #[serde(rename = "Hat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hat: Option<AccessoryRefinementModel>,
    #[serde(rename = "Head")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<AccessoryRefinementModel>,
    #[serde(rename = "IdleAnimation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_animation: Option<AccessoryRefinementModel>,
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<AccessoryRefinementModel>,
    #[serde(rename = "JacketAccessory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jacket_accessory: Option<AccessoryRefinementModel>,
    #[serde(rename = "JumpAnimation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jump_animation: Option<AccessoryRefinementModel>,
    #[serde(rename = "LeftArm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_arm: Option<AccessoryRefinementModel>,
    #[serde(rename = "LeftLeg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_leg: Option<AccessoryRefinementModel>,
    #[serde(rename = "LeftShoeAccessory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_shoe_accessory: Option<AccessoryRefinementModel>,
    #[serde(rename = "Legs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legs: Option<AccessoryRefinementModel>,
    #[serde(rename = "LocalizationTableManifest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localization_table_manifest: Option<AccessoryRefinementModel>,
    #[serde(rename = "LocalizationTableTranslation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localization_table_translation: Option<AccessoryRefinementModel>,
    #[serde(rename = "Lua")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lua: Option<AccessoryRefinementModel>,
    #[serde(rename = "Mesh")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh: Option<AccessoryRefinementModel>,
    #[serde(rename = "MeshHiddenSurfaceRemoval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_hidden_surface_removal: Option<AccessoryRefinementModel>,
    #[serde(rename = "MeshPart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_part: Option<AccessoryRefinementModel>,
    #[serde(rename = "Model")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<AccessoryRefinementModel>,
    #[serde(rename = "MoodAnimation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mood_animation: Option<AccessoryRefinementModel>,
    #[serde(rename = "NeckAccessory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neck_accessory: Option<AccessoryRefinementModel>,
    #[serde(rename = "OtaUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ota_update: Option<AccessoryRefinementModel>,
    #[serde(rename = "Package")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<AccessoryRefinementModel>,
    #[serde(rename = "Pants")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pants: Option<AccessoryRefinementModel>,
    #[serde(rename = "PantsAccessory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pants_accessory: Option<AccessoryRefinementModel>,
    #[serde(rename = "Place")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place: Option<AccessoryRefinementModel>,
    #[serde(rename = "Plugin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin: Option<AccessoryRefinementModel>,
    #[serde(rename = "PoseAnimation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pose_animation: Option<AccessoryRefinementModel>,
    #[serde(rename = "RightArm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_arm: Option<AccessoryRefinementModel>,
    #[serde(rename = "RightLeg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_leg: Option<AccessoryRefinementModel>,
    #[serde(rename = "RightShoeAccessory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_shoe_accessory: Option<AccessoryRefinementModel>,
    #[serde(rename = "RunAnimation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_animation: Option<AccessoryRefinementModel>,
    #[serde(rename = "Shirt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shirt: Option<AccessoryRefinementModel>,
    #[serde(rename = "ShirtAccessory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shirt_accessory: Option<AccessoryRefinementModel>,
    #[serde(rename = "ShortsAccessory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shorts_accessory: Option<AccessoryRefinementModel>,
    #[serde(rename = "ShoulderAccessory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shoulder_accessory: Option<AccessoryRefinementModel>,
    #[serde(rename = "SolidModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solid_model: Option<AccessoryRefinementModel>,
    #[serde(rename = "SweaterAccessory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sweater_accessory: Option<AccessoryRefinementModel>,
    #[serde(rename = "SwimAnimation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swim_animation: Option<AccessoryRefinementModel>,
    #[serde(rename = "TShirt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_shirt: Option<AccessoryRefinementModel>,
    #[serde(rename = "TShirtAccessory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_shirt_accessory: Option<AccessoryRefinementModel>,
    #[serde(rename = "Text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<AccessoryRefinementModel>,
    #[serde(rename = "TexturePack")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texture_pack: Option<AccessoryRefinementModel>,
    #[serde(rename = "Torso")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub torso: Option<AccessoryRefinementModel>,
    #[serde(rename = "Video")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<AccessoryRefinementModel>,
    #[serde(rename = "WaistAccessory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waist_accessory: Option<AccessoryRefinementModel>,
    #[serde(rename = "WalkAnimation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub walk_animation: Option<AccessoryRefinementModel>,
    #[serde(rename = "YouTubeVideo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub you_tube_video: Option<AccessoryRefinementModel>,
}

/// `Roblox.Api.Avatar.Models.BodyColorModel`
///
/// A model container BrickColor ids for each body part.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BodyColorModel {
    /// The BrickColor id
    #[serde(rename = "brickColorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brick_color_id: Option<i32>,
    /// The hex color, e.g. #FFFFFF
    #[serde(rename = "hexColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex_color: Option<String>,
    /// The name of the BrickColor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.Api.Avatar.Models.DefaultClothingAssets`
///
/// A model containing details about avatar-related business rules
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DefaultClothingAssets {
    /// List of asset Ids used to equip pants for default clothing when the avatar appears nude.
    #[serde(rename = "defaultPantAssetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_pant_asset_ids: Option<Vec<i64>>,
    /// List of asset Ids used to equip shirts for default clothing when the avatar appears nude.
    #[serde(rename = "defaultShirtAssetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_shirt_asset_ids: Option<Vec<i64>>,
}

/// `Roblox.Api.Avatar.Models.ScaleRulesModel`
///
/// A model that contains information about the max/mins for each scale
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ScaleRulesModel {
    /// The increment of the scale
    #[serde(skip_serializing_if = "Option::is_none")]
    pub increment: Option<f64>,
    /// The max scale
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    /// The min scale
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
}

/// `Roblox.Api.Avatar.Models.AvatarRulesModel`
///
/// A model containing details about avatar-related business rules
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AvatarRulesModel {
    /// The lower bounds for accessory refinement settings.
    #[serde(rename = "accessoryRefinementLowerBounds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessory_refinement_lower_bounds: Option<AvatarRulesModelAccessoryRefinementLowerBounds>,
    /// The list of asset types for Accessory Refinement.
    #[serde(rename = "accessoryRefinementTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessory_refinement_types: Option<Vec<AvatarRulesModelAccessoryRefinementTypes>>,
    /// The lower bounds for accessory refinement settings.
    #[serde(rename = "accessoryRefinementUpperBounds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessory_refinement_upper_bounds: Option<AvatarRulesModelAccessoryRefinementUpperBounds>,
    /// The basic set of body colors
    #[serde(rename = "basicBodyColorsPalette")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_body_colors_palette: Option<Vec<BodyColorModel>>,
    /// The full set of usable body colors
    #[serde(rename = "bodyColorsPalette")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_colors_palette: Option<Vec<BodyColorModel>>,
    /// Whether or not bundles are enabled for the specific user
    #[serde(rename = "bundlesEnabledForUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundles_enabled_for_user: Option<bool>,
    #[serde(rename = "defaultClothingAssetLists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_clothing_asset_lists: Option<DefaultClothingAssets>,
    /// Whether or not emotes are enabled
    #[serde(rename = "emotesEnabledForUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emotes_enabled_for_user: Option<bool>,
    /// The minimum Delta-E difference in body colors for default clothing not to be applied
    #[serde(rename = "minimumDeltaEBodyColorDifference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_delta_e_body_color_difference: Option<f64>,
    /// The avatar type
    #[serde(rename = "playerAvatarTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_avatar_types: Option<Vec<AvatarModelV2PlayerAvatarType>>,
    /// Whether proportion and bodyType scales are allowed to be set by the authenticated user
    #[serde(rename = "proportionsAndBodyTypeEnabledForUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proportions_and_body_type_enabled_for_user: Option<bool>,
    /// The scales
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scales: Option<HashMap<String, ScaleRulesModel>>,
    /// The assets worn on the character
    #[serde(rename = "wearableAssetTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wearable_asset_types: Option<Vec<AssetTypeRulesModel>>,
}

/// `Roblox.Api.Avatar.Models.UniverseAvatarAssetOverrideResponseModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UniverseAvatarAssetOverrideResponseModel {
    #[serde(rename = "assetID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<i64>,
    #[serde(rename = "assetTypeID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type_id: Option<i32>,
    #[serde(rename = "isPlayerChoice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_player_choice: Option<bool>,
}

/// `Roblox.Api.Avatar.Models.GameStartInfoResponse`
///
/// The game start info
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GameStartInfoResponse {
    /// Custom animation enabled
    #[serde(rename = "allowCustomAnimations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_custom_animations: Option<String>,
    /// Avatar Type
    #[serde(rename = "gameAvatarType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_avatar_type: Option<String>,
    /// Joing positioning type
    #[serde(rename = "jointPositioningType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joint_positioning_type: Option<String>,
    /// Mesasge
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Moderation status
    #[serde(rename = "moderationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation_status: Option<String>,
    /// asset overrides for the univers
    #[serde(rename = "universeAvatarAssetOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_avatar_asset_overrides: Option<Vec<UniverseAvatarAssetOverrideResponseModel>>,
    /// Body type for the univers
    #[serde(rename = "universeAvatarBodyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_avatar_body_type: Option<String>,
    /// collision type for the univers
    #[serde(rename = "universeAvatarCollisionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_avatar_collision_type: Option<String>,
    #[serde(rename = "universeAvatarMaxScales")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_avatar_max_scales: Option<ScaleModel>,
    #[serde(rename = "universeAvatarMinScales")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_avatar_min_scales: Option<ScaleModel>,
}

/// `Roblox.Api.Avatar.Models.OutfitDetailsModel`
///
/// A model containing details about a user outfit
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct OutfitDetailsModel {
    /// A list of assetIds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<AssetModelV2>>,
    #[serde(rename = "bodyColors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_colors: Option<BodyColorsModel>,
    /// The id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
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

crate::api_enum! {
    int
    /// The type of outfit \['Invalid' = 0, 'Avatar' = 1, 'DynamicHead' = 2, 'InExperienceBody' = 3\]
    OutfitUpdateModelV1OutfitType {
    Invalid = 0 => "Invalid",
    Avatar = 1 => "Avatar",
    DynamicHead = 2 => "DynamicHead",
    InExperienceBody = 3 => "InExperienceBody",
    }
}

/// `Roblox.Api.Avatar.Models.OutfitUpdateModelV1`
///
/// A model containing details needed to update or create an outfit
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct OutfitUpdateModelV1 {
    /// The outfit asset IDs
    #[serde(rename = "assetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_ids: Option<Vec<i64>>,
    #[serde(rename = "bodyColors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_colors: Option<BodyColorsModel>,
    /// The outfit name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The type of outfit \['Invalid' = 0, 'Avatar' = 1, 'DynamicHead' = 2, 'InExperienceBody' = 3\]
    #[serde(rename = "outfitType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outfit_type: Option<OutfitUpdateModelV1OutfitType>,
    /// The avatar scale
    #[serde(rename = "playerAvatarType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_avatar_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<ScaleModel>,
}

/// `Roblox.Api.Avatar.Models.PlayerAvatarTypeModel`
///
/// A model that contains a playerAvatarType
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PlayerAvatarTypeModel {
    /// The playerAvatarType \['R6' = 1, 'R15' = 3\]
    #[serde(rename = "playerAvatarType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_avatar_type: Option<AvatarModelV2PlayerAvatarType>,
}

/// `Roblox.Api.Avatar.Models.RecentItemModel`
///
/// A model containing details about a recent item
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RecentItemModel {
    #[serde(rename = "assetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<AssetTypeModel>,
    /// The id. Check the type to figure out what kind of recent item this is.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The outfit's editability (only included if it is an outfit)
    #[serde(rename = "isEditable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_editable: Option<bool>,
    /// The name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The recent item's type. Can be Asset or Outfit \['Asset' = 1, 'Outfit' = 2\]
    #[serde(rename = "type")]
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<i32>,
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

/// `Roblox.Web.WebAPI.Models.ApiLegacyPageResponse\[Roblox.Api.Avatar.Models.RecentItemModel\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiLegacyPageResponseRecentItemModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<RecentItemModel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

/// Query, header and form inputs for [`get_game_start_info`].
#[derive(Debug, Clone, Default)]
pub struct GetGameStartInfoParams {
    pub universe_id: i64,
}

/// Query, header and form inputs for [`get_users_by_user_id_outfits`].
#[derive(Debug, Clone, Default)]
pub struct GetUsersByUserIdOutfitsParams {
    /// The outfit type being searched for, null will return all outfitTypes
    pub outfit_type: Option<String>,
    /// The page number of the current page of requests, default is 1.
    pub page: Option<i32>,
    /// The max number of outfits that can be returned.
    pub items_per_page: Option<i32>,
    /// Whether the outfits are editable. A null value will lead to no filtering.
    pub is_editable: Option<bool>,
}

/// `GET /v1/avatar`
///
/// Returns details about the authenticated user's avatar
pub fn get_avatar_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/avatar".to_string());
    Ok(r)
}

/// `GET /v1/avatar`
///
/// Returns details about the authenticated user's avatar
pub async fn get_avatar(client: &Client) -> Result<AvatarModelV2> {
    client.send(get_avatar_request()?).await?.json()
}

/// `GET /v1/avatar-rules`
///
/// Returns the business rules related to avatars
///
/// BodyColorsPalette is a list of valid brickColors you can choose for your avatar. WearableAssetTypes contains a list of asset types with names, ids, and the maximum number that you can wear at a time. Does not include packages because they cannot be worn on your avatar directly. PlayerAvatarTypes are the types of avatars you can choose between.
pub fn get_avatar_rules_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/avatar-rules".to_string());
    Ok(r)
}

/// `GET /v1/avatar-rules`
///
/// Returns the business rules related to avatars
///
/// BodyColorsPalette is a list of valid brickColors you can choose for your avatar. WearableAssetTypes contains a list of asset types with names, ids, and the maximum number that you can wear at a time. Does not include packages because they cannot be worn on your avatar directly. PlayerAvatarTypes are the types of avatars you can choose between.
pub async fn get_avatar_rules(client: &Client) -> Result<AvatarRulesModel> {
    client.send(get_avatar_rules_request()?).await?.json()
}

/// `POST /v1/avatar/assets/{assetId}/remove`
///
/// Removes the asset from the authenticated user's avatar.
#[deprecated(note = "marked deprecated by Roblox")]
pub fn post_avatar_assets_by_asset_id_remove_request(asset_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::POST,
        HOST,
        format!("/v1/avatar/assets/{}/remove", crate::rt::path_value(&asset_id)),
    );
    Ok(r)
}

/// `POST /v1/avatar/assets/{assetId}/remove`
///
/// Removes the asset from the authenticated user's avatar.
#[deprecated(note = "marked deprecated by Roblox")]
pub async fn post_avatar_assets_by_asset_id_remove(
    client: &Client,
    asset_id: i64,
) -> Result<AvatarApiSuccessResponse> {
    client
        .send(post_avatar_assets_by_asset_id_remove_request(asset_id)?)
        .await?
        .json()
}

/// `POST /v1/avatar/assets/{assetId}/wear`
///
/// Puts the asset on the authenticated user's avatar. - Flagged as obsolete, does not support layered clothing meta params
#[deprecated(note = "marked deprecated by Roblox")]
pub fn post_avatar_assets_by_asset_id_wear_request(asset_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::POST,
        HOST,
        format!("/v1/avatar/assets/{}/wear", crate::rt::path_value(&asset_id)),
    );
    Ok(r)
}

/// `POST /v1/avatar/assets/{assetId}/wear`
///
/// Puts the asset on the authenticated user's avatar. - Flagged as obsolete, does not support layered clothing meta params
#[deprecated(note = "marked deprecated by Roblox")]
pub async fn post_avatar_assets_by_asset_id_wear(
    client: &Client,
    asset_id: i64,
) -> Result<AvatarApiSuccessResponse> {
    client
        .send(post_avatar_assets_by_asset_id_wear_request(asset_id)?)
        .await?
        .json()
}

/// `GET /v1/avatar/metadata`
///
/// Returns metadata used by the avatar page of the website
pub fn get_avatar_metadata_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/avatar/metadata".to_string());
    Ok(r)
}

/// `GET /v1/avatar/metadata`
///
/// Returns metadata used by the avatar page of the website
pub async fn get_avatar_metadata(client: &Client) -> Result<AvatarMetadataModel> {
    client.send(get_avatar_metadata_request()?).await?.json()
}

/// `POST /v1/avatar/redraw-thumbnail`
///
/// Requests the authenticated user's thumbnail be redrawn
pub fn post_avatar_redraw_thumbnail_request() -> Result<Request> {
    let r = Request::new(Method::POST, HOST, "/v1/avatar/redraw-thumbnail".to_string());
    Ok(r)
}

/// `POST /v1/avatar/redraw-thumbnail`
///
/// Requests the authenticated user's thumbnail be redrawn
pub async fn post_avatar_redraw_thumbnail(client: &Client) -> Result<EmptyResponse> {
    client.send(post_avatar_redraw_thumbnail_request()?).await?.json()
}

/// `POST /v1/avatar/set-body-colors`
///
/// Sets the authenticated user's body colors
pub fn post_avatar_set_body_colors_request(body: &BodyColorsModel) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/avatar/set-body-colors".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/avatar/set-body-colors`
///
/// Sets the authenticated user's body colors
pub async fn post_avatar_set_body_colors(
    client: &Client,
    body: &BodyColorsModel,
) -> Result<AvatarApiSuccessResponse> {
    client
        .send(post_avatar_set_body_colors_request(body)?)
        .await?
        .json()
}

/// `POST /v1/avatar/set-player-avatar-type`
///
/// Sets the authenticated user's player avatar type (e.g. R6 or R15).
///
/// This is the avatar type chosen on the Avatar page. Some games can override this and force your character to be R6 or R15.
pub fn post_avatar_set_player_avatar_type_request(body: &PlayerAvatarTypeModel) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        "/v1/avatar/set-player-avatar-type".to_string(),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/avatar/set-player-avatar-type`
///
/// Sets the authenticated user's player avatar type (e.g. R6 or R15).
///
/// This is the avatar type chosen on the Avatar page. Some games can override this and force your character to be R6 or R15.
pub async fn post_avatar_set_player_avatar_type(
    client: &Client,
    body: &PlayerAvatarTypeModel,
) -> Result<AvatarApiSuccessResponse> {
    client
        .send(post_avatar_set_player_avatar_type_request(body)?)
        .await?
        .json()
}

/// `POST /v1/avatar/set-scales`
///
/// Sets the authenticated user's scales
pub fn post_avatar_set_scales_request(body: &ScaleModel) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/avatar/set-scales".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/avatar/set-scales`
///
/// Sets the authenticated user's scales
pub async fn post_avatar_set_scales(client: &Client, body: &ScaleModel) -> Result<AvatarApiSuccessResponse> {
    client.send(post_avatar_set_scales_request(body)?).await?.json()
}

/// `POST /v1/avatar/set-wearing-assets`
///
/// Sets the avatar's current assets to the list - Flagged as obsolete, does not support layered clothing meta params
///
/// Only allows items that you own, are not expired, and are wearable asset types. Any assets being worn before this method is called are automatically removed.
#[deprecated(note = "marked deprecated by Roblox")]
pub fn post_avatar_set_wearing_assets_request(body: &AssetIdListModel) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/avatar/set-wearing-assets".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/avatar/set-wearing-assets`
///
/// Sets the avatar's current assets to the list - Flagged as obsolete, does not support layered clothing meta params
///
/// Only allows items that you own, are not expired, and are wearable asset types. Any assets being worn before this method is called are automatically removed.
#[deprecated(note = "marked deprecated by Roblox")]
pub async fn post_avatar_set_wearing_assets(
    client: &Client,
    body: &AssetIdListModel,
) -> Result<WearResponseModel> {
    client
        .send(post_avatar_set_wearing_assets_request(body)?)
        .await?
        .json()
}

/// `GET /v1/game-start-info`
///
/// The server will call this on game server start to request general information about the universe This is version 1.1, which returns an entry from the UniverseAvatarType enum. During mixed mode this may return unreliable results.
pub fn get_game_start_info_request(params: &GetGameStartInfoParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/game-start-info".to_string());
    r.query("universeId", &params.universe_id);
    Ok(r)
}

/// `GET /v1/game-start-info`
///
/// The server will call this on game server start to request general information about the universe This is version 1.1, which returns an entry from the UniverseAvatarType enum. During mixed mode this may return unreliable results.
pub async fn get_game_start_info(
    client: &Client,
    params: &GetGameStartInfoParams,
) -> Result<GameStartInfoResponse> {
    client.send(get_game_start_info_request(params)?).await?.json()
}

/// `POST /v1/outfits/create`
///
/// Creates a new outfit.
///
/// Fails if any of the assetIds are not owned by the user, or not wearable types. The name property of the request is optional as one will be auto-generated when the request has a null name.
#[deprecated(note = "marked deprecated by Roblox")]
pub fn post_outfits_create_request(body: &OutfitUpdateModelV1) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/outfits/create".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/outfits/create`
///
/// Creates a new outfit.
///
/// Fails if any of the assetIds are not owned by the user, or not wearable types. The name property of the request is optional as one will be auto-generated when the request has a null name.
#[deprecated(note = "marked deprecated by Roblox")]
pub async fn post_outfits_create(
    client: &Client,
    body: &OutfitUpdateModelV1,
) -> Result<AvatarApiSuccessResponse> {
    client.send(post_outfits_create_request(body)?).await?.json()
}

/// `PATCH /v1/outfits/{userOutfitId}`
///
/// Updates the contents of an outfit. Deprecated. There are no calls to this API
///
/// Fails if the user does not own any of the assetIds or if they are not wearable asset types. Accepts partial updates.
#[deprecated(note = "marked deprecated by Roblox")]
pub fn patch_outfits_by_user_outfit_id_request(
    user_outfit_id: i64,
    body: &OutfitUpdateModelV1,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!("/v1/outfits/{}", crate::rt::path_value(&user_outfit_id)),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/outfits/{userOutfitId}`
///
/// Updates the contents of an outfit. Deprecated. There are no calls to this API
///
/// Fails if the user does not own any of the assetIds or if they are not wearable asset types. Accepts partial updates.
#[deprecated(note = "marked deprecated by Roblox")]
pub async fn patch_outfits_by_user_outfit_id(
    client: &Client,
    user_outfit_id: i64,
    body: &OutfitUpdateModelV1,
) -> Result<OutfitModel> {
    client
        .send(patch_outfits_by_user_outfit_id_request(user_outfit_id, body)?)
        .await?
        .json()
}

/// `POST /v1/outfits/{userOutfitId}/delete`
///
/// Deletes the outfit
///
/// You are only allowed to delete outfits you created.
pub fn post_outfits_by_user_outfit_id_delete_request(user_outfit_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::POST,
        HOST,
        format!("/v1/outfits/{}/delete", crate::rt::path_value(&user_outfit_id)),
    );
    Ok(r)
}

/// `POST /v1/outfits/{userOutfitId}/delete`
///
/// Deletes the outfit
///
/// You are only allowed to delete outfits you created.
pub async fn post_outfits_by_user_outfit_id_delete(
    client: &Client,
    user_outfit_id: i64,
) -> Result<AvatarApiSuccessResponse> {
    client
        .send(post_outfits_by_user_outfit_id_delete_request(user_outfit_id)?)
        .await?
        .json()
}

/// `GET /v1/outfits/{userOutfitId}/details`
pub fn get_outfits_by_user_outfit_id_details_request(user_outfit_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/outfits/{}/details", crate::rt::path_value(&user_outfit_id)),
    );
    Ok(r)
}

/// `GET /v1/outfits/{userOutfitId}/details`
pub async fn get_outfits_by_user_outfit_id_details(
    client: &Client,
    user_outfit_id: i64,
) -> Result<OutfitDetailsModel> {
    client
        .send(get_outfits_by_user_outfit_id_details_request(user_outfit_id)?)
        .await?
        .json()
}

/// `POST /v1/outfits/{userOutfitId}/update`
///
/// Updates the contents of the outfit. Deprecated. There are no calls to this API
///
/// Fails if the user does not own any of the assetIds or if they are not wearable asset types.
#[deprecated(note = "marked deprecated by Roblox")]
pub fn post_outfits_by_user_outfit_id_update_request(
    user_outfit_id: i64,
    body: &OutfitUpdateModelV1,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!("/v1/outfits/{}/update", crate::rt::path_value(&user_outfit_id)),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/outfits/{userOutfitId}/update`
///
/// Updates the contents of the outfit. Deprecated. There are no calls to this API
///
/// Fails if the user does not own any of the assetIds or if they are not wearable asset types.
#[deprecated(note = "marked deprecated by Roblox")]
pub async fn post_outfits_by_user_outfit_id_update(
    client: &Client,
    user_outfit_id: i64,
    body: &OutfitUpdateModelV1,
) -> Result<AvatarApiSuccessResponse> {
    client
        .send(post_outfits_by_user_outfit_id_update_request(
            user_outfit_id,
            body,
        )?)
        .await?
        .json()
}

/// `GET /v1/recent-items/{recentItemListType}/list`
///
/// Returns a list of recent items Recent items can be Assets or Outfits
pub fn get_recent_items_by_recent_item_list_type_list_request(recent_item_list_type: i32) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/recent-items/{}/list",
            crate::rt::path_value(&recent_item_list_type)
        ),
    );
    Ok(r)
}

/// `GET /v1/recent-items/{recentItemListType}/list`
///
/// Returns a list of recent items Recent items can be Assets or Outfits
pub async fn get_recent_items_by_recent_item_list_type_list(
    client: &Client,
    recent_item_list_type: i32,
) -> Result<ApiLegacyPageResponseRecentItemModel> {
    client
        .send(get_recent_items_by_recent_item_list_type_list_request(
            recent_item_list_type,
        )?)
        .await?
        .json()
}

/// `GET /v1/users/{userId}/avatar`
///
/// Returns details about a specified user's avatar
///
/// Includes assets, bodycolors, and playerAvatarType.
pub fn get_users_by_user_id_avatar_request(user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/users/{}/avatar", crate::rt::path_value(&user_id)),
    );
    Ok(r)
}

/// `GET /v1/users/{userId}/avatar`
///
/// Returns details about a specified user's avatar
///
/// Includes assets, bodycolors, and playerAvatarType.
pub async fn get_users_by_user_id_avatar(client: &Client, user_id: i64) -> Result<AvatarModelV2> {
    client
        .send(get_users_by_user_id_avatar_request(user_id)?)
        .await?
        .json()
}

/// `GET /v1/users/{userId}/currently-wearing`
///
/// Gets a list of asset ids that the user is currently wearing
pub fn get_users_by_user_id_currently_wearing_request(user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/users/{}/currently-wearing", crate::rt::path_value(&user_id)),
    );
    Ok(r)
}

/// `GET /v1/users/{userId}/currently-wearing`
///
/// Gets a list of asset ids that the user is currently wearing
pub async fn get_users_by_user_id_currently_wearing(
    client: &Client,
    user_id: i64,
) -> Result<AssetIdListModel> {
    client
        .send(get_users_by_user_id_currently_wearing_request(user_id)?)
        .await?
        .json()
}

/// `GET /v1/users/{userId}/outfits`
///
/// Deprecated, user v2. Gets a list of outfits for the specified user.
pub fn get_users_by_user_id_outfits_request(
    user_id: i64,
    params: &GetUsersByUserIdOutfitsParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/users/{}/outfits", crate::rt::path_value(&user_id)),
    );
    r.query("outfitType", &params.outfit_type);
    r.query("page", &params.page);
    r.query("itemsPerPage", &params.items_per_page);
    r.query("isEditable", &params.is_editable);
    Ok(r)
}

/// `GET /v1/users/{userId}/outfits`
///
/// Deprecated, user v2. Gets a list of outfits for the specified user.
pub async fn get_users_by_user_id_outfits(
    client: &Client,
    user_id: i64,
    params: &GetUsersByUserIdOutfitsParams,
) -> Result<AvatarFilteredPageResponseOutfitModel> {
    client
        .send(get_users_by_user_id_outfits_request(user_id, params)?)
        .await?
        .json()
}

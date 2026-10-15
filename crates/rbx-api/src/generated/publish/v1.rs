//! `publish.roblox.com` v1 — generated from `vendor/roblox-api-tracker/publish/v1.json`.
//!
//! Do not edit. Change the spec or `rbx-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "publish.roblox.com";

/// `Roblox.Publish.Api.AssetQuota`
///
/// Model for asset quota.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AssetQuota {
    /// Capacity of the quota.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
    /// Duration type of the quota.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// Expiration time of current usage limit.
    #[serde(rename = "expirationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    /// Current usage of the quota.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<i64>,
}

/// `Roblox.Publish.Api.AssetQuotasResponse`
///
/// Response model for asset quotas.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AssetQuotasResponse {
    /// A list of quotas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quotas: Option<Vec<AssetQuota>>,
}

/// `Roblox.Publish.Api.PublishAudioResponse`
///
/// Response model for publish audio.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PublishAudioResponse {
    /// Id of the published asset.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Name of the published asset.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.Publish.Api.UploadAudioRequest`
///
/// A request model for uploading an audio file.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UploadAudioRequest {
    /// The asset privacy of the audio asset.
    #[serde(rename = "assetPrivacy")]
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_privacy: Option<i32>,
    /// Estimated duration of the audio file in seconds.
    #[serde(rename = "estimatedDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_duration: Option<f64>,
    /// Estimated file size of the audio file in bytes.
    #[serde(rename = "estimatedFileSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_file_size: Option<i64>,
    /// File to be uploaded. Formatted as a base64 string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    /// Id of the group you are publishing the audio asset for. Null if not publishing under a group.
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i64>,
    /// Name for the audio file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The source of funds for payment. User: Use personal funds of authenticated user. Group: Use group funds from Roblox.Publish.Api.UploadAudioRequest.GroupId. Null/Empty: Will default to authenticated user funds.
    #[serde(rename = "paymentSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_source: Option<String>,
}

/// `Roblox.Publish.Api.UploadResponse`
///
/// A response used when an upload has completed.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UploadResponse {
    /// The target Id of the uploaded item.
    #[serde(rename = "targetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<i64>,
}

/// `Roblox.Publish.Api.VerifyAudioRequest`
///
/// Request model to publish an audio asset.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct VerifyAudioRequest {
    /// Gets or sets the duration of the audio in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    /// File to be uploaded. Formatted as a base64 string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    /// Gets or sets the size of the audio file in bytes.
    #[serde(rename = "fileSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    /// Gets or sets the ID of the group if applicable. Optional.
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i64>,
    /// Gets or sets the name of the audio asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Gets or sets the payment source. 'User' or 'Group'. Required if Group ID is set.
    #[serde(rename = "paymentSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_source: Option<String>,
}

/// `Roblox.Publish.Api.VerifyAudioResponse`
///
/// Response model for verify audio endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct VerifyAudioResponse {
    /// User's current Robux balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<i64>,
    /// Boolean, true if the user can afford to purchase the publishing of the audio file.
    #[serde(rename = "canAfford")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_afford: Option<bool>,
    /// Name of the audio file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Price in robux to publish the audio file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
}

/// Query, header and form inputs for [`get_asset_quotas`].
#[derive(Debug, Clone, Default)]
pub struct GetAssetQuotasParams {
    /// Resource type of the asset quota
    pub resource_type: String,
    /// Asset type of the asset quota
    pub asset_type: String,
    /// Use dummy data for testing. This is for internal use only
    pub use_dummy_data: Option<bool>,
}

/// Query, header and form inputs for [`post_badges_by_badge_id_icon`].
#[derive(Debug, Clone, Default)]
pub struct PostBadgesByBadgeIdIconParams {
    pub files: Option<FilePart>,
}

/// Query, header and form inputs for [`post_games_by_game_id_thumbnail_image`].
#[derive(Debug, Clone, Default)]
pub struct PostGamesByGameIdThumbnailImageParams {
    pub files: Option<FilePart>,
}

/// Query, header and form inputs for [`post_plugins_by_plugin_id_icon`].
#[derive(Debug, Clone, Default)]
pub struct PostPluginsByPluginIdIconParams {
    pub files: Option<FilePart>,
}

/// `GET /v1/asset-quotas`
///
/// List asset quotas of the given resource type and asset type.
pub fn get_asset_quotas_request(params: &GetAssetQuotasParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/asset-quotas".to_string());
    r.query("resourceType", &params.resource_type);
    r.query("assetType", &params.asset_type);
    r.query("useDummyData", &params.use_dummy_data);
    Ok(r)
}

/// `GET /v1/asset-quotas`
///
/// List asset quotas of the given resource type and asset type.
pub async fn get_asset_quotas(client: &Client, params: &GetAssetQuotasParams) -> Result<AssetQuotasResponse> {
    client.send(get_asset_quotas_request(params)?).await?.json()
}

/// `POST /v1/audio`
///
/// Published an audio file and returns the new asset info.
pub fn post_audio_request(body: &UploadAudioRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/audio".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/audio`
///
/// Published an audio file and returns the new asset info.
pub async fn post_audio(client: &Client, body: &UploadAudioRequest) -> Result<PublishAudioResponse> {
    client.send(post_audio_request(body)?).await?.json()
}

/// `POST /v1/audio/verify`
///
/// Verifies an audio file and returns a product that you can purchase to publish the audio file.
pub fn post_audio_verify_request(body: &VerifyAudioRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/audio/verify".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/audio/verify`
///
/// Verifies an audio file and returns a product that you can purchase to publish the audio file.
pub async fn post_audio_verify(client: &Client, body: &VerifyAudioRequest) -> Result<VerifyAudioResponse> {
    client.send(post_audio_verify_request(body)?).await?.json()
}

/// `POST /v1/badges/{badgeId}/icon`
///
/// Overwrites a badge icon with a new one.
pub fn post_badges_by_badge_id_icon_request(
    badge_id: i64,
    params: &PostBadgesByBadgeIdIconParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!("/v1/badges/{}/icon", crate::rt::path_value(&badge_id)),
    );
    r.form_file("Files", &params.files);
    Ok(r)
}

/// `POST /v1/badges/{badgeId}/icon`
///
/// Overwrites a badge icon with a new one.
pub async fn post_badges_by_badge_id_icon(
    client: &Client,
    badge_id: i64,
    params: &PostBadgesByBadgeIdIconParams,
) -> Result<UploadResponse> {
    client
        .send(post_badges_by_badge_id_icon_request(badge_id, params)?)
        .await?
        .json()
}

/// `POST /v1/games/{gameId}/thumbnail/image`
///
/// Uploads a game thumbnail.
pub fn post_games_by_game_id_thumbnail_image_request(
    game_id: i64,
    params: &PostGamesByGameIdThumbnailImageParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!("/v1/games/{}/thumbnail/image", crate::rt::path_value(&game_id)),
    );
    r.form_file("Files", &params.files);
    Ok(r)
}

/// `POST /v1/games/{gameId}/thumbnail/image`
///
/// Uploads a game thumbnail.
pub async fn post_games_by_game_id_thumbnail_image(
    client: &Client,
    game_id: i64,
    params: &PostGamesByGameIdThumbnailImageParams,
) -> Result<UploadResponse> {
    client
        .send(post_games_by_game_id_thumbnail_image_request(game_id, params)?)
        .await?
        .json()
}

/// `POST /v1/plugins/{pluginId}/icon`
///
/// Overwrites a plugin icon with a new one.
pub fn post_plugins_by_plugin_id_icon_request(
    plugin_id: i64,
    params: &PostPluginsByPluginIdIconParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!("/v1/plugins/{}/icon", crate::rt::path_value(&plugin_id)),
    );
    r.form_file("Files", &params.files);
    Ok(r)
}

/// `POST /v1/plugins/{pluginId}/icon`
///
/// Overwrites a plugin icon with a new one.
pub async fn post_plugins_by_plugin_id_icon(
    client: &Client,
    plugin_id: i64,
    params: &PostPluginsByPluginIdIconParams,
) -> Result<UploadResponse> {
    client
        .send(post_plugins_by_plugin_id_icon_request(plugin_id, params)?)
        .await?
        .json()
}

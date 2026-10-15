//! `clientsettings.roblox.com` v2 — generated from `vendor/roblox-api-tracker/clientsettings/v2.json`.
//!
//! Do not edit. Change the spec or `rbx-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "clientsettings.roblox.com";

/// `Roblox.ClientSettings.Api.Models.Response.AndroidBinaryLibraryNames`
///
/// Contains the names of the libraries in an Android Binary Module.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AndroidBinaryLibraryNames {
    /// Name of the engine library.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
}

/// `Roblox.ClientSettings.Api.Models.Response.AndroidBinaryResponse`
///
/// Contains information about an Android Binary Module for a given channel. including its module name, the names of its libraries, and whether the channel supports Android binaries.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AndroidBinaryResponse {
    #[serde(rename = "libraryNames")]
    pub library_names: AndroidBinaryLibraryNames,
    /// The name of the Android Binary Module.
    #[serde(rename = "moduleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_name: Option<String>,
    /// True if the channel supports android binaries. False otherwise.
    #[serde(rename = "supportsAndroidBinaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_android_binaries: Option<bool>,
}

/// `Roblox.ClientSettings.Api.Models.Response.BetaProgramInfo`
///
/// Beta program information included in the user channel response.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BetaProgramInfo {
    /// The ID of the beta program.
    pub id: String,
    /// The display name of the beta program.
    pub name: String,
}

/// `Roblox.ClientSettings.Api.Models.Response.ClientVersionResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ClientVersionResponse {
    #[serde(rename = "bootstrapperVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootstrapper_version: Option<String>,
    #[serde(rename = "clientVersionUpload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_version_upload: Option<String>,
    #[serde(rename = "forceInstall")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_install: Option<bool>,
    #[serde(rename = "nextClientVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_client_version: Option<String>,
    #[serde(rename = "nextClientVersionUpload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_client_version_upload: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// `Roblox.ClientSettings.Api.Models.Response.OtaVersionResponse`
///
/// Response for endpoints returning ota information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct OtaVersionResponse {
    /// The application storage reset ID for the LuaApp OTA.
    #[serde(rename = "appStorageResetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_storage_reset_id: Option<String>,
    /// The asset ID of the LuaApp OTA.
    #[serde(rename = "assetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    /// The asset version of the LuaApp OTA.
    #[serde(rename = "assetVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_version: Option<String>,
    /// Inlined assets manifest for the LuaApp OTA.
    #[serde(rename = "assetsManifest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets_manifest: Option<HashMap<String, String>>,
    /// URL to download the ota asset
    #[serde(rename = "downloadUrl")]
    pub download_url: String,
    /// Indicates whether the LuaApp OTA is a development configuration.
    #[serde(rename = "isDevelopmentConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_development_config: Option<bool>,
    /// Indicates whether the LuaApp OTA is a forced update.
    #[serde(rename = "isForcedUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_forced_update: Option<bool>,
    /// Refers to whether the plugin is core to Studio functions, and is used to determine when it is loaded.
    #[serde(rename = "isStandalone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_standalone: Option<bool>,
    /// The local asset URI for the LuaApp OTA.
    #[serde(rename = "localAssetURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_asset_uri: Option<String>,
    /// The maximum application version compatible with the LuaApp OTA.
    #[serde(rename = "maxAppVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_app_version: Option<String>,
    /// Ota library/plugin name
    pub name: String,
    /// The name of the tryout for the LuaApp OTA.
    #[serde(rename = "tryoutName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tryout_name: Option<String>,
    /// Version number of asset
    pub version: String,
    /// VersionV2 of the OTA asset.
    #[serde(rename = "versionV2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_v2: Option<i64>,
}

crate::api_enum! {
    int
    /// How the user was bound to the channel. If the user is not assigned to any channel, this is omitted. \['None' = 0, 'PerMille' = 1, 'BoundToPrivateChannel' = 2, 'BoundToPublicChannel' = 3, 'OptedInToBetaProgramWithPrivateChannel' = 4, 'OptedInToBetaProgramWithPublicChannel' = 5\]
    UserChannelResponseChannelAssignmentType {
    None = 0 => "None",
    PerMille = 1 => "PerMille",
    BoundToPrivateChannel = 2 => "BoundToPrivateChannel",
    BoundToPublicChannel = 3 => "BoundToPublicChannel",
    OptedInToBetaProgramWithPrivateChannel = 4 => "OptedInToBetaProgramWithPrivateChannel",
    OptedInToBetaProgramWithPublicChannel = 5 => "OptedInToBetaProgramWithPublicChannel",
    }
}

/// `Roblox.ClientSettings.Api.Models.Response.UserChannelResponse`
///
/// Response data object for getting a user's channel information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UserChannelResponse {
    /// How the user was bound to the channel. If the user is not assigned to any channel, this is omitted. \['None' = 0, 'PerMille' = 1, 'BoundToPrivateChannel' = 2, 'BoundToPublicChannel' = 3, 'OptedInToBetaProgramWithPrivateChannel' = 4, 'OptedInToBetaProgramWithPublicChannel' = 5\]
    #[serde(rename = "channelAssignmentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_assignment_type: Option<UserChannelResponseChannelAssignmentType>,
    /// Name of the channel.
    #[serde(rename = "channelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    /// Whether the channel is flag-only. A flag-only channel uses production binaries. If the channel is not flag-only, this is omitted.
    #[serde(rename = "isFlagOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_flag_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program: Option<BetaProgramInfo>,
    /// JWT token. If the channel is not private, this is omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

/// Query, header and form inputs for [`get_ota_version_by_binary_type`].
#[derive(Debug, Clone, Default)]
pub struct GetOtaVersionByBinaryTypeParams {
    /// Channel name. If not provided, production is assumed.
    pub channel: Option<String>,
    /// Application version
    pub version: Option<String>,
    /// Tag to filter results by. Only applicable to non-studio application types.
    pub tag: Option<String>,
    /// Name to filter results by. Only applicable to non-studio application types.
    pub name: Option<String>,
}

/// Query, header and form inputs for [`get_user_channel`].
#[derive(Debug, Clone, Default)]
pub struct GetUserChannelParams {
    pub binary_type: Option<String>,
}

/// `GET /v2/android-binaries/{version}/channels/{channelName}`
///
/// Retrieve the Android binary information for a given version and channel name.
pub fn get_android_binaries_by_version_channels_by_channel_name_request(
    version: &str,
    channel_name: &str,
) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v2/android-binaries/{}/channels/{}",
            crate::rt::path_value(&version),
            crate::rt::path_value(&channel_name)
        ),
    );
    Ok(r)
}

/// `GET /v2/android-binaries/{version}/channels/{channelName}`
///
/// Retrieve the Android binary information for a given version and channel name.
pub async fn get_android_binaries_by_version_channels_by_channel_name(
    client: &Client,
    version: &str,
    channel_name: &str,
) -> Result<AndroidBinaryResponse> {
    client
        .send(get_android_binaries_by_version_channels_by_channel_name_request(
            version,
            channel_name,
        )?)
        .await?
        .json()
}

/// `GET /v2/client-version/{binaryType}`
///
/// Get client version information for specific binary type
pub fn get_client_version_by_binary_type_request(binary_type: &str) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v2/client-version/{}", crate::rt::path_value(&binary_type)),
    );
    Ok(r)
}

/// `GET /v2/client-version/{binaryType}`
///
/// Get client version information for specific binary type
pub async fn get_client_version_by_binary_type(
    client: &Client,
    binary_type: &str,
) -> Result<ClientVersionResponse> {
    client
        .send(get_client_version_by_binary_type_request(binary_type)?)
        .await?
        .json()
}

/// `GET /v2/client-version/{binaryType}/channel/{channelName}`
///
/// Get client version information for specific binary type
pub fn get_client_version_by_binary_type_channel_by_channel_name_request(
    binary_type: &str,
    channel_name: &str,
) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v2/client-version/{}/channel/{}",
            crate::rt::path_value(&binary_type),
            crate::rt::path_value(&channel_name)
        ),
    );
    Ok(r)
}

/// `GET /v2/client-version/{binaryType}/channel/{channelName}`
///
/// Get client version information for specific binary type
pub async fn get_client_version_by_binary_type_channel_by_channel_name(
    client: &Client,
    binary_type: &str,
    channel_name: &str,
) -> Result<ClientVersionResponse> {
    client
        .send(get_client_version_by_binary_type_channel_by_channel_name_request(
            binary_type,
            channel_name,
        )?)
        .await?
        .json()
}

/// `GET /v2/compression-dictionaries`
///
/// Returns a listing of all known compression dictionaries, including their SHA256 and creation date. This will be sorted by creation date, with the most recent dictionaries first.
pub fn get_compression_dictionaries_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v2/compression-dictionaries".to_string());
    Ok(r)
}

/// `GET /v2/compression-dictionaries`
///
/// Returns a listing of all known compression dictionaries, including their SHA256 and creation date. This will be sorted by creation date, with the most recent dictionaries first.
pub async fn get_compression_dictionaries(client: &Client) -> Result<()> {
    client.send(get_compression_dictionaries_request()?).await?.unit()
}

/// `GET /v2/compression-dictionaries/{dictionarySha256}`
///
/// Returns the specified compression dictionary as a file download.
pub fn get_compression_dictionaries_by_dictionary_sha256_request(dictionary_sha256: &str) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v2/compression-dictionaries/{}",
            crate::rt::path_value(&dictionary_sha256)
        ),
    );
    Ok(r)
}

/// `GET /v2/compression-dictionaries/{dictionarySha256}`
///
/// Returns the specified compression dictionary as a file download.
pub async fn get_compression_dictionaries_by_dictionary_sha256(
    client: &Client,
    dictionary_sha256: &str,
) -> Result<()> {
    client
        .send(get_compression_dictionaries_by_dictionary_sha256_request(
            dictionary_sha256,
        )?)
        .await?
        .unit()
}

/// `GET /v2/ota-version/{binaryType}`
///
/// Get OTA information for a specific binary type with a given version on some channel. Returns empty list if no updates are found or channel/application with the given version does not exist in CVS.
pub fn get_ota_version_by_binary_type_request(
    binary_type: &str,
    params: &GetOtaVersionByBinaryTypeParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v2/ota-version/{}", crate::rt::path_value(&binary_type)),
    );
    r.query("channel", &params.channel);
    r.query("version", &params.version);
    r.query("tag", &params.tag);
    r.query("name", &params.name);
    Ok(r)
}

/// `GET /v2/ota-version/{binaryType}`
///
/// Get OTA information for a specific binary type with a given version on some channel. Returns empty list if no updates are found or channel/application with the given version does not exist in CVS.
pub async fn get_ota_version_by_binary_type(
    client: &Client,
    binary_type: &str,
    params: &GetOtaVersionByBinaryTypeParams,
) -> Result<Vec<OtaVersionResponse>> {
    client
        .send(get_ota_version_by_binary_type_request(binary_type, params)?)
        .await?
        .json()
}

/// `GET /v2/user-channel`
///
/// Get channel name for currently logged in user
pub fn get_user_channel_request(params: &GetUserChannelParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v2/user-channel".to_string());
    r.query("binaryType", &params.binary_type);
    Ok(r)
}

/// `GET /v2/user-channel`
///
/// Get channel name for currently logged in user
pub async fn get_user_channel(client: &Client, params: &GetUserChannelParams) -> Result<UserChannelResponse> {
    client.send(get_user_channel_request(params)?).await?.json()
}

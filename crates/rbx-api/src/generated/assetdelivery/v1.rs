//! `assetdelivery.roblox.com` v1 — generated from `vendor/roblox-api-tracker/assetdelivery/v1.json`.
//!
//! Do not edit. Change the spec or `rbx-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "assetdelivery.roblox.com";

/// `Roblox.Web.Assets.AssetContentRepresentationSpecifier`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AssetContentRepresentationSpecifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fidelity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "majorVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major_version: Option<String>,
}

crate::api_enum! {
    int
    /// Custom error code for Roblox.Web.Assets.IAssetItemError \['UnknownError' = 0, 'NoPermissionToAsset' = 1, 'AssetPermissionCheckFailed' = 2, 'NotAuthorizedForAgeRecommendation' = 3, 'AgeRecommendationCheckFailed' = 4, 'InvalidPlaceRequestFromGameServer' = 5, 'BlockedAssetTypeRequestedFromInsertService' = 6, 'BlockedAssetTypeRequestedFromGameServer' = 7, 'AssetTypeMismatch' = 8, 'MissingAssetTypeInRequestHeader' = 9, 'AssetNotTrustedForPlace' = 10\]
    IAssetItemErrorCustomErrorCode {
    UnknownError = 0 => "UnknownError",
    NoPermissionToAsset = 1 => "NoPermissionToAsset",
    AssetPermissionCheckFailed = 2 => "AssetPermissionCheckFailed",
    NotAuthorizedForAgeRecommendation = 3 => "NotAuthorizedForAgeRecommendation",
    AgeRecommendationCheckFailed = 4 => "AgeRecommendationCheckFailed",
    InvalidPlaceRequestFromGameServer = 5 => "InvalidPlaceRequestFromGameServer",
    BlockedAssetTypeRequestedFromInsertService = 6 => "BlockedAssetTypeRequestedFromInsertService",
    BlockedAssetTypeRequestedFromGameServer = 7 => "BlockedAssetTypeRequestedFromGameServer",
    AssetTypeMismatch = 8 => "AssetTypeMismatch",
    MissingAssetTypeInRequestHeader = 9 => "MissingAssetTypeInRequestHeader",
    AssetNotTrustedForPlace = 10 => "AssetNotTrustedForPlace",
    }
}

/// `Roblox.Web.Assets.IAssetItemError`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct IAssetItemError {
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    /// Custom error code for Roblox.Web.Assets.IAssetItemError \['UnknownError' = 0, 'NoPermissionToAsset' = 1, 'AssetPermissionCheckFailed' = 2, 'NotAuthorizedForAgeRecommendation' = 3, 'AgeRecommendationCheckFailed' = 4, 'InvalidPlaceRequestFromGameServer' = 5, 'BlockedAssetTypeRequestedFromInsertService' = 6, 'BlockedAssetTypeRequestedFromGameServer' = 7, 'AssetTypeMismatch' = 8, 'MissingAssetTypeInRequestHeader' = 9, 'AssetNotTrustedForPlace' = 10\]
    #[serde(rename = "CustomErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_error_code: Option<IAssetItemErrorCustomErrorCode>,
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// `Roblox.Web.Assets.AssetResponseItemV1`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AssetResponseItemV1 {
    /// Whether the content's copyright is protected.
    #[serde(rename = "IsCopyrightProtected")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_copyright_protected: Option<bool>,
    #[serde(rename = "IsHashDynamic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_hash_dynamic: Option<bool>,
    /// Asset Type.
    #[serde(rename = "assetTypeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type_id: Option<i32>,
    #[serde(rename = "contentRepresentationSpecifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_representation_specifier: Option<AssetContentRepresentationSpecifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<IAssetItemError>>,
    /// Whether the asset has been archived.
    #[serde(rename = "isArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// `Roblox.Web.Assets.BatchAssetRequestItem`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BatchAssetRequestItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept: Option<String>,
    #[serde(rename = "assetFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_format: Option<String>,
    #[serde(rename = "assetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<i64>,
    #[serde(rename = "assetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_name: Option<String>,
    #[serde(rename = "assetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<String>,
    #[serde(rename = "assetVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_version_id: Option<i64>,
    #[serde(rename = "clientInsert")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_insert: Option<bool>,
    #[serde(rename = "contentRepresentationPriorityList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_representation_priority_list: Option<String>,
    #[serde(rename = "doNotFallbackToBaselineRepresentation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub do_not_fallback_to_baseline_representation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(rename = "modulePlaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_place_id: Option<i64>,
    #[serde(rename = "placeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_id: Option<i64>,
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "roblox-assetFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roblox_asset_format: Option<String>,
    #[serde(rename = "scriptInsert")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_insert: Option<bool>,
    #[serde(rename = "serverPlaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_place_id: Option<i64>,
    #[serde(rename = "universeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_id: Option<i64>,
    #[serde(rename = "userAssetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_asset_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}

/// Query, header and form inputs for [`get_alias_by_alias`].
#[derive(Debug, Clone, Default)]
pub struct GetAliasByAliasParams {
    pub accept_encoding: String,
    pub roblox_place_id: i64,
    pub asset_type: String,
    pub accept: String,
    pub asset_format: String,
    pub roblox_asset_format: String,
    pub skip_signing_scripts: Option<bool>,
    pub client_insert: Option<i32>,
    pub scriptinsert: Option<i32>,
    pub module_place_id: Option<i64>,
    pub serverplaceid: Option<i64>,
    pub expected_asset_type: Option<String>,
}

/// Query, header and form inputs for [`get_asset`].
#[derive(Debug, Clone, Default)]
pub struct GetAssetParams {
    pub accept_encoding: String,
    pub roblox_place_id: i64,
    pub asset_type: String,
    pub accept: String,
    pub asset_format: String,
    pub roblox_asset_format: String,
    pub id: Option<i64>,
    pub user_asset_id: Option<i64>,
    pub asset_version_id: Option<i64>,
    pub version: Option<i32>,
    pub universe_id: Option<i64>,
    pub client_insert: Option<i32>,
    pub scriptinsert: Option<i32>,
    pub module_place_id: Option<i64>,
    pub serverplaceid: Option<String>,
    pub asset_name: Option<String>,
    pub hash: Option<String>,
    pub mar_asset_hash: Option<String>,
    pub mar_check_sum: Option<String>,
    pub expected_asset_type: Option<String>,
    pub skip_signing_scripts: Option<bool>,
    pub permission_context: Option<String>,
    pub do_not_fallback_to_baseline_representation: Option<bool>,
    pub content_representation_priority_list: Option<String>,
}

/// Query, header and form inputs for [`get_asset_id_by_asset_id`].
#[derive(Debug, Clone, Default)]
pub struct GetAssetIdByAssetIdParams {
    pub accept_encoding: String,
    pub roblox_place_id: i64,
    pub asset_type: String,
    pub accept: String,
    pub asset_format: String,
    pub roblox_asset_format: String,
    pub skip_signing_scripts: Option<bool>,
    pub client_insert: Option<i32>,
    pub scriptinsert: Option<i32>,
    pub module_place_id: Option<i64>,
    pub serverplaceid: Option<i64>,
    pub expected_asset_type: Option<String>,
    pub do_not_fallback_to_baseline_representation: Option<bool>,
    pub content_representation_priority_list: Option<String>,
}

/// Query, header and form inputs for [`get_asset_id_by_asset_id_version_by_version`].
#[derive(Debug, Clone, Default)]
pub struct GetAssetIdByAssetIdVersionByVersionParams {
    pub accept_encoding: String,
    pub roblox_place_id: i64,
    pub asset_type: String,
    pub accept: String,
    pub asset_format: String,
    pub roblox_asset_format: String,
    pub skip_signing_scripts: Option<bool>,
    pub client_insert: Option<i32>,
    pub scriptinsert: Option<i32>,
    pub module_place_id: Option<i64>,
    pub serverplaceid: Option<i64>,
    pub expected_asset_type: Option<String>,
    pub do_not_fallback_to_baseline_representation: Option<bool>,
    pub content_representation_priority_list: Option<String>,
}

/// Query, header and form inputs for [`post_assets_batch`].
#[derive(Debug, Clone, Default)]
pub struct PostAssetsBatchParams {
    pub roblox_place_id: i64,
    pub accept: String,
    pub roblox_browser_asset_request: String,
}

/// Query, header and form inputs for [`get_mar_asset_hash_by_mar_asset_hash_mar_check_sum_by_mar_check_sum`].
#[derive(Debug, Clone, Default)]
pub struct GetMarAssetHashByMarAssetHashMarCheckSumByMarCheckSumParams {
    pub accept_encoding: String,
    pub roblox_place_id: i64,
    pub asset_type: String,
    pub accept: String,
    pub asset_format: String,
    pub roblox_asset_format: String,
    pub skip_signing_scripts: Option<bool>,
    pub client_insert: Option<i32>,
    pub scriptinsert: Option<i32>,
    pub module_place_id: Option<i64>,
    pub serverplaceid: Option<i64>,
    pub expected_asset_type: Option<String>,
}

/// `GET /v1/alias/{alias}`
///
/// Retrieves an asset by its alias (universeID/name)
pub fn get_alias_by_alias_request(alias: &str, params: &GetAliasByAliasParams) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/alias/{}", crate::rt::path_value(&alias)),
    );
    r.header("Accept-Encoding", &params.accept_encoding);
    r.header("Roblox-Place-Id", &params.roblox_place_id);
    r.header("AssetType", &params.asset_type);
    r.header("Accept", &params.accept);
    r.header("AssetFormat", &params.asset_format);
    r.header("Roblox-AssetFormat", &params.roblox_asset_format);
    r.query("skipSigningScripts", &params.skip_signing_scripts);
    r.query("clientInsert", &params.client_insert);
    r.query("scriptinsert", &params.scriptinsert);
    r.query("modulePlaceId", &params.module_place_id);
    r.query("serverplaceid", &params.serverplaceid);
    r.query("expectedAssetType", &params.expected_asset_type);
    Ok(r)
}

/// `GET /v1/alias/{alias}`
///
/// Retrieves an asset by its alias (universeID/name)
pub async fn get_alias_by_alias(
    client: &Client,
    alias: &str,
    params: &GetAliasByAliasParams,
) -> Result<AssetResponseItemV1> {
    client
        .send(get_alias_by_alias_request(alias, params)?)
        .await?
        .json()
}

/// `GET /v1/asset`
pub fn get_asset_request(params: &GetAssetParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/asset".to_string());
    r.header("Accept-Encoding", &params.accept_encoding);
    r.header("Roblox-Place-Id", &params.roblox_place_id);
    r.header("AssetType", &params.asset_type);
    r.header("Accept", &params.accept);
    r.header("AssetFormat", &params.asset_format);
    r.header("Roblox-AssetFormat", &params.roblox_asset_format);
    r.query("id", &params.id);
    r.query("userAssetId", &params.user_asset_id);
    r.query("assetVersionId", &params.asset_version_id);
    r.query("version", &params.version);
    r.query("universeId", &params.universe_id);
    r.query("clientInsert", &params.client_insert);
    r.query("scriptinsert", &params.scriptinsert);
    r.query("modulePlaceId", &params.module_place_id);
    r.query("serverplaceid", &params.serverplaceid);
    r.query("assetName", &params.asset_name);
    r.query("hash", &params.hash);
    r.query("marAssetHash", &params.mar_asset_hash);
    r.query("marCheckSum", &params.mar_check_sum);
    r.query("expectedAssetType", &params.expected_asset_type);
    r.query("skipSigningScripts", &params.skip_signing_scripts);
    r.query("permissionContext", &params.permission_context);
    r.query(
        "doNotFallbackToBaselineRepresentation",
        &params.do_not_fallback_to_baseline_representation,
    );
    r.query(
        "contentRepresentationPriorityList",
        &params.content_representation_priority_list,
    );
    Ok(r)
}

/// `GET /v1/asset`
pub async fn get_asset(client: &Client, params: &GetAssetParams) -> Result<()> {
    client.send(get_asset_request(params)?).await?.unit()
}

/// `GET /v1/assetId/{assetId}`
///
/// Retrieves an asset by its ID
pub fn get_asset_id_by_asset_id_request(
    asset_id: i64,
    params: &GetAssetIdByAssetIdParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/assetId/{}", crate::rt::path_value(&asset_id)),
    );
    r.header("Accept-Encoding", &params.accept_encoding);
    r.header("Roblox-Place-Id", &params.roblox_place_id);
    r.header("AssetType", &params.asset_type);
    r.header("Accept", &params.accept);
    r.header("AssetFormat", &params.asset_format);
    r.header("Roblox-AssetFormat", &params.roblox_asset_format);
    r.query("skipSigningScripts", &params.skip_signing_scripts);
    r.query("clientInsert", &params.client_insert);
    r.query("scriptinsert", &params.scriptinsert);
    r.query("modulePlaceId", &params.module_place_id);
    r.query("serverplaceid", &params.serverplaceid);
    r.query("expectedAssetType", &params.expected_asset_type);
    r.query(
        "doNotFallbackToBaselineRepresentation",
        &params.do_not_fallback_to_baseline_representation,
    );
    r.query(
        "contentRepresentationPriorityList",
        &params.content_representation_priority_list,
    );
    Ok(r)
}

/// `GET /v1/assetId/{assetId}`
///
/// Retrieves an asset by its ID
pub async fn get_asset_id_by_asset_id(
    client: &Client,
    asset_id: i64,
    params: &GetAssetIdByAssetIdParams,
) -> Result<AssetResponseItemV1> {
    client
        .send(get_asset_id_by_asset_id_request(asset_id, params)?)
        .await?
        .json()
}

/// `GET /v1/assetId/{assetId}/version/{version}`
///
/// Retrieves an asset by its ID and its version.
pub fn get_asset_id_by_asset_id_version_by_version_request(
    asset_id: i64,
    version: i32,
    params: &GetAssetIdByAssetIdVersionByVersionParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/assetId/{}/version/{}",
            crate::rt::path_value(&asset_id),
            crate::rt::path_value(&version)
        ),
    );
    r.header("Accept-Encoding", &params.accept_encoding);
    r.header("Roblox-Place-Id", &params.roblox_place_id);
    r.header("AssetType", &params.asset_type);
    r.header("Accept", &params.accept);
    r.header("AssetFormat", &params.asset_format);
    r.header("Roblox-AssetFormat", &params.roblox_asset_format);
    r.query("skipSigningScripts", &params.skip_signing_scripts);
    r.query("clientInsert", &params.client_insert);
    r.query("scriptinsert", &params.scriptinsert);
    r.query("modulePlaceId", &params.module_place_id);
    r.query("serverplaceid", &params.serverplaceid);
    r.query("expectedAssetType", &params.expected_asset_type);
    r.query(
        "doNotFallbackToBaselineRepresentation",
        &params.do_not_fallback_to_baseline_representation,
    );
    r.query(
        "contentRepresentationPriorityList",
        &params.content_representation_priority_list,
    );
    Ok(r)
}

/// `GET /v1/assetId/{assetId}/version/{version}`
///
/// Retrieves an asset by its ID and its version.
pub async fn get_asset_id_by_asset_id_version_by_version(
    client: &Client,
    asset_id: i64,
    version: i32,
    params: &GetAssetIdByAssetIdVersionByVersionParams,
) -> Result<AssetResponseItemV1> {
    client
        .send(get_asset_id_by_asset_id_version_by_version_request(
            asset_id, version, params,
        )?)
        .await?
        .json()
}

/// `POST /v1/assets/batch`
pub fn post_assets_batch_request(
    params: &PostAssetsBatchParams,
    body: &Vec<BatchAssetRequestItem>,
) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/assets/batch".to_string());
    r.header("Roblox-Place-Id", &params.roblox_place_id);
    r.header("Accept", &params.accept);
    r.header(
        "Roblox-Browser-Asset-Request",
        &params.roblox_browser_asset_request,
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/assets/batch`
pub async fn post_assets_batch(
    client: &Client,
    params: &PostAssetsBatchParams,
    body: &Vec<BatchAssetRequestItem>,
) -> Result<Vec<AssetResponseItemV1>> {
    client
        .send(post_assets_batch_request(params, body)?)
        .await?
        .json()
}

/// `GET /v1/marAssetHash/{marAssetHash}/marCheckSum/{marCheckSum}`
///
/// Retrieves an asset by its mar (moderation agnostic) hash and mar (moderation agnostic) checksum.
pub fn get_mar_asset_hash_by_mar_asset_hash_mar_check_sum_by_mar_check_sum_request(
    mar_asset_hash: &str,
    mar_check_sum: &str,
    params: &GetMarAssetHashByMarAssetHashMarCheckSumByMarCheckSumParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/marAssetHash/{}/marCheckSum/{}",
            crate::rt::path_value(&mar_asset_hash),
            crate::rt::path_value(&mar_check_sum)
        ),
    );
    r.header("Accept-Encoding", &params.accept_encoding);
    r.header("Roblox-Place-Id", &params.roblox_place_id);
    r.header("AssetType", &params.asset_type);
    r.header("Accept", &params.accept);
    r.header("AssetFormat", &params.asset_format);
    r.header("Roblox-AssetFormat", &params.roblox_asset_format);
    r.query("skipSigningScripts", &params.skip_signing_scripts);
    r.query("clientInsert", &params.client_insert);
    r.query("scriptinsert", &params.scriptinsert);
    r.query("modulePlaceId", &params.module_place_id);
    r.query("serverplaceid", &params.serverplaceid);
    r.query("expectedAssetType", &params.expected_asset_type);
    Ok(r)
}

/// `GET /v1/marAssetHash/{marAssetHash}/marCheckSum/{marCheckSum}`
///
/// Retrieves an asset by its mar (moderation agnostic) hash and mar (moderation agnostic) checksum.
pub async fn get_mar_asset_hash_by_mar_asset_hash_mar_check_sum_by_mar_check_sum(
    client: &Client,
    mar_asset_hash: &str,
    mar_check_sum: &str,
    params: &GetMarAssetHashByMarAssetHashMarCheckSumByMarCheckSumParams,
) -> Result<AssetResponseItemV1> {
    client
        .send(
            get_mar_asset_hash_by_mar_asset_hash_mar_check_sum_by_mar_check_sum_request(
                mar_asset_hash,
                mar_check_sum,
                params,
            )?,
        )
        .await?
        .json()
}

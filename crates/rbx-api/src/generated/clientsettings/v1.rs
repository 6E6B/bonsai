//! `clientsettings.roblox.com` v1 — generated from `vendor/roblox-api-tracker/clientsettings/v1.json`.
//!
//! Do not edit. Change the spec or `rbx-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "clientsettings.roblox.com";

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

/// `Roblox.ClientSettings.Api.Models.Response.MobileClientVersionResponseData`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MobileClientVersionResponseData {
    #[serde(rename = "UpgradeAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_action: Option<String>,
}

/// `Roblox.ClientSettings.Api.Models.Response.MobileClientVersionResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MobileClientVersionResponse {
    #[serde(rename = "MD5Sum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md5_sum: Option<String>,
    #[serde(rename = "activeVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<MobileClientVersionResponseData>,
    #[serde(rename = "upgradeSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_source: Option<String>,
}

/// Query, header and form inputs for [`get_mobile_client_version`].
#[derive(Debug, Clone, Default)]
pub struct GetMobileClientVersionParams {
    /// AppiOSV2.13, AppVersioniOS2.0.1, etc
    pub app_version: String,
}

/// `GET /v1/client-version/{binaryType}`
///
/// Get client version information for specific binary type
pub fn get_client_version_by_binary_type_request(binary_type: &str) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/client-version/{}", crate::rt::path_value(&binary_type)),
    );
    Ok(r)
}

/// `GET /v1/client-version/{binaryType}`
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

/// `GET /v1/installer-cdns`
///
/// Get information about which CDNs to use for installation.
pub fn get_installer_cdns_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/installer-cdns".to_string());
    Ok(r)
}

/// `GET /v1/installer-cdns`
///
/// Get information about which CDNs to use for installation.
pub async fn get_installer_cdns(client: &Client) -> Result<()> {
    client.send(get_installer_cdns_request()?).await?.unit()
}

/// `GET /v1/mobile-client-version`
///
/// Get mobile client version information based on app version parameter
pub fn get_mobile_client_version_request(params: &GetMobileClientVersionParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/mobile-client-version".to_string());
    r.query("appVersion", &params.app_version);
    Ok(r)
}

/// `GET /v1/mobile-client-version`
///
/// Get mobile client version information based on app version parameter
pub async fn get_mobile_client_version(
    client: &Client,
    params: &GetMobileClientVersionParams,
) -> Result<MobileClientVersionResponse> {
    client
        .send(get_mobile_client_version_request(params)?)
        .await?
        .json()
}

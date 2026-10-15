//! `metrics.roblox.com` v1 — generated from `vendor/roblox-api-tracker/metrics/v1.json`.
//!
//! Do not edit. Change the spec or `rbx-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "metrics.roblox.com";

crate::api_enum! {
    int
    /// The content type of the bundle. \['Unknown' = 0, 'JavaScript' = 1, 'Css' = 2\]
    RecordBundleLoadRequestBundleContentType {
    UnknownValue = 0 => "Unknown",
    JavaScript = 1 => "JavaScript",
    Css = 2 => "Css",
    }
}

crate::api_enum! {
    int
    /// Whether or not the bundle loaded successfully. \['Unknown' = 0, 'LoadSuccess' = 1, 'LoadFailure' = 2, 'ExecutionFailure' = 3\]
    RecordBundleLoadRequestLoadState {
    UnknownValue = 0 => "Unknown",
    LoadSuccess = 1 => "LoadSuccess",
    LoadFailure = 2 => "LoadFailure",
    ExecutionFailure = 3 => "ExecutionFailure",
    }
}

/// `Roblox.Metrics.Api.RecordBundleLoadRequest`
///
/// Request parameters for tracking a bundle load.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RecordBundleLoadRequest {
    /// The content type of the bundle. \['Unknown' = 0, 'JavaScript' = 1, 'Css' = 2\]
    #[serde(rename = "bundleContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_content_type: Option<RecordBundleLoadRequestBundleContentType>,
    /// The name of the bundle.
    #[serde(rename = "bundleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_name: Option<String>,
    /// The bundle url that loaded.
    #[serde(rename = "bundleUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_url: Option<String>,
    /// The CDN that served the bundle.
    #[serde(rename = "cdnProviderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdn_provider_name: Option<String>,
    /// Whether or not the bundle loaded successfully. \['Unknown' = 0, 'LoadSuccess' = 1, 'LoadFailure' = 2, 'ExecutionFailure' = 3\]
    #[serde(rename = "loadState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_state: Option<RecordBundleLoadRequestLoadState>,
    /// How long it took to load.
    #[serde(rename = "loadTimeInMilliseconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_time_in_milliseconds: Option<i64>,
}

crate::api_enum! {
    int
    /// The load state. Roblox.Metrics.Api.ThumbnailLoadState \['Unknown' = 0, 'Complete' = 1, 'Timeout' = 2\]
    RecordThumbnailLoadRequestLoadState {
    UnknownValue = 0 => "Unknown",
    Complete = 1 => "Complete",
    Timeout = 2 => "Timeout",
    }
}

/// `Roblox.Metrics.Api.RecordThumbnailLoadRequest`
///
/// Request to add measurement.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RecordThumbnailLoadRequest {
    /// The time it took in milliseconds for final thumbnail to load
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    /// The load state. Roblox.Metrics.Api.ThumbnailLoadState \['Unknown' = 0, 'Complete' = 1, 'Timeout' = 2\]
    #[serde(rename = "loadState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_state: Option<RecordThumbnailLoadRequestLoadState>,
    /// The type of thumbnail.
    #[serde(rename = "thumbnailType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_type: Option<String>,
}

/// `Roblox.Metrics.Api.ThumbnailLoadMetadataResponse`
///
/// Response for thumbnail metrics related metadata.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ThumbnailLoadMetadataResponse {
    /// The probability that the metrics get logged
    #[serde(rename = "logRatio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_ratio: Option<f64>,
}

/// `Roblox.Web.WebAPI.ApiEmptyResponseModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiEmptyResponseModel {}

/// `POST /v1/bundle-metrics/report`
///
/// Records bundle load successes.
///
/// Xsrf protection disabled because this endpoint is supposed to be used to record static content. It's possible XSRF can't be retried if: - The XSRF retry JavaScript fails to load (or fails to execute) - CSS fails to load before JavaScript finishes loading.
pub fn post_bundle_metrics_report_request(body: &RecordBundleLoadRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/bundle-metrics/report".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/bundle-metrics/report`
///
/// Records bundle load successes.
///
/// Xsrf protection disabled because this endpoint is supposed to be used to record static content. It's possible XSRF can't be retried if: - The XSRF retry JavaScript fails to load (or fails to execute) - CSS fails to load before JavaScript finishes loading.
pub async fn post_bundle_metrics_report(
    client: &Client,
    body: &RecordBundleLoadRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_bundle_metrics_report_request(body)?)
        .await?
        .json()
}

/// `POST /v1/thumbnails/load`
///
/// Records the time it takes for a thumbnail to get loaded in the UI.
pub fn post_thumbnails_load_request(body: &RecordThumbnailLoadRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/thumbnails/load".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/thumbnails/load`
///
/// Records the time it takes for a thumbnail to get loaded in the UI.
pub async fn post_thumbnails_load(
    client: &Client,
    body: &RecordThumbnailLoadRequest,
) -> Result<ApiEmptyResponseModel> {
    client.send(post_thumbnails_load_request(body)?).await?.json()
}

/// `GET /v1/thumbnails/metadata`
///
/// Get metadata related to logging thumbnail load metrics. e.g. ThumbnailLoadLoggingRatio
pub fn get_thumbnails_metadata_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/thumbnails/metadata".to_string());
    Ok(r)
}

/// `GET /v1/thumbnails/metadata`
///
/// Get metadata related to logging thumbnail load metrics. e.g. ThumbnailLoadLoggingRatio
pub async fn get_thumbnails_metadata(client: &Client) -> Result<ThumbnailLoadMetadataResponse> {
    client.send(get_thumbnails_metadata_request()?).await?.json()
}

//! `fonts.google.com` undocumented — generated from `specs/undocumented/fonts.google.com.yaml`.
//!
//! Do not edit. Change the spec or `bonsai-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "fonts.google.com";

/// `FontFamily`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FontFamily {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fonts: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub popularity: Option<i64>,
}

/// `FontCatalog`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FontCatalog {
    #[serde(rename = "familyMetadataList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_metadata_list: Option<Vec<FontFamily>>,
}

/// `GET /metadata/fonts`
///
/// Every Google Fonts family.
///
/// fonts maps a style key to its metrics: "400" is regular weight 400, "400i" its italic. popularity is a rank, lower is more popular. category is one of Sans Serif, Serif, Display, Handwriting or Monospace.
///
/// Undocumented; captured 2026-10-15, https://fonts.google.com/.
pub fn get_font_catalog_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/metadata/fonts".to_string());
    Ok(r)
}

/// `GET /metadata/fonts`
///
/// Every Google Fonts family.
///
/// fonts maps a style key to its metrics: "400" is regular weight 400, "400i" its italic. popularity is a rank, lower is more popular. category is one of Sans Serif, Serif, Display, Handwriting or Monospace.
///
/// Undocumented; captured 2026-10-15, https://fonts.google.com/.
pub async fn get_font_catalog(client: &Client) -> Result<FontCatalog> {
    client.send(get_font_catalog_request()?).await?.json()
}

//! `fonts.googleapis.com` undocumented — generated from `specs/undocumented/fonts.googleapis.com.yaml`.
//!
//! Do not edit. Change the spec or `rbx-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "fonts.googleapis.com";

/// Query, header and form inputs for [`get_font_css`].
#[derive(Debug, Clone, Default)]
pub struct GetFontCssParams {
    pub family: String,
}

/// `GET /css2`
///
/// The @font-face rules for one family and its weights.
///
/// family is "Name:wght@400;700" with the name as in the catalog. The body is CSS with one @font-face per weight: font-weight N and src url(https://fonts.gstatic.com/...ttf).
///
/// Undocumented; captured 2026-10-15, https://fonts.google.com/specimen/Inter.
pub fn get_font_css_request(params: &GetFontCssParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/css2".to_string());
    r.query("family", &params.family);
    Ok(r)
}

/// `GET /css2`
///
/// The @font-face rules for one family and its weights.
///
/// family is "Name:wght@400;700" with the name as in the catalog. The body is CSS with one @font-face per weight: font-weight N and src url(https://fonts.gstatic.com/...ttf).
///
/// Undocumented; captured 2026-10-15, https://fonts.google.com/specimen/Inter.
pub async fn get_font_css(client: &Client, params: &GetFontCssParams) -> Result<()> {
    client.send(get_font_css_request(params)?).await?.unit()
}

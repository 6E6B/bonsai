//! `thumbnailsresizer.roblox.com` v1 — generated from `vendor/roblox-api-tracker/thumbnailsresizer/v1.json`.
//!
//! Do not edit. Change the spec or `rbx-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "thumbnailsresizer.roblox.com";

/// Query, header and form inputs for [`get_resize_by_hash_by_width_by_height_by_type_by_format_by_filter_type`].
#[derive(Debug, Clone, Default)]
pub struct GetResizeByHashByWidthByHeightByTypeByFormatByFilterTypeParams {
    /// E.g. is output circular
    pub filter_type: String,
    pub should_modify: Option<bool>,
}

/// Query, header and form inputs for [`get_secureresize_by_thumb_print_by_hash_by_width_by_height_by_type_by_format_by_filter_type`].
#[derive(Debug, Clone, Default)]
pub struct GetSecureresizeByThumbPrintByHashByWidthByHeightByTypeByFormatByFilterTypeParams {
    /// E.g. is output circular
    pub filter_type: Option<String>,
}

/// `GET /v1/resize/{hash}/{width}/{height}/{type}/{format}/{filterType}`
///
/// Resizes larger thumbnails to specified size and format
pub fn get_resize_by_hash_by_width_by_height_by_type_by_format_by_filter_type_request(
    hash: &str,
    width: i32,
    height: i32,
    type_: &str,
    format: &str,
    filter_type_: &str,
    params: &GetResizeByHashByWidthByHeightByTypeByFormatByFilterTypeParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/resize/{}/{}/{}/{}/{}/{}",
            crate::rt::path_value(&hash),
            crate::rt::path_value(&width),
            crate::rt::path_value(&height),
            crate::rt::path_value(&type_),
            crate::rt::path_value(&format),
            crate::rt::path_value(&filter_type_)
        ),
    );
    r.query("filterType", &params.filter_type);
    r.query("shouldModify", &params.should_modify);
    Ok(r)
}

/// `GET /v1/resize/{hash}/{width}/{height}/{type}/{format}/{filterType}`
///
/// Resizes larger thumbnails to specified size and format
pub async fn get_resize_by_hash_by_width_by_height_by_type_by_format_by_filter_type(
    client: &Client,
    hash: &str,
    width: i32,
    height: i32,
    type_: &str,
    format: &str,
    filter_type_: &str,
    params: &GetResizeByHashByWidthByHeightByTypeByFormatByFilterTypeParams,
) -> Result<()> {
    client
        .send(
            get_resize_by_hash_by_width_by_height_by_type_by_format_by_filter_type_request(
                hash,
                width,
                height,
                type_,
                format,
                filter_type_,
                params,
            )?,
        )
        .await?
        .unit()
}

/// `GET /v1/secureresize/{thumbPrint}/{hash}/{width}/{height}/{type}/{format}/{filterType}`
///
/// Decrypts and Resizes larger thumbnails to specified size and format
pub fn get_secureresize_by_thumb_print_by_hash_by_width_by_height_by_type_by_format_by_filter_type_request(
    thumb_print: &str,
    hash: &str,
    width: i32,
    height: i32,
    type_: &str,
    format: &str,
    filter_type_: &str,
    params: &GetSecureresizeByThumbPrintByHashByWidthByHeightByTypeByFormatByFilterTypeParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/secureresize/{}/{}/{}/{}/{}/{}/{}",
            crate::rt::path_value(&thumb_print),
            crate::rt::path_value(&hash),
            crate::rt::path_value(&width),
            crate::rt::path_value(&height),
            crate::rt::path_value(&type_),
            crate::rt::path_value(&format),
            crate::rt::path_value(&filter_type_)
        ),
    );
    r.query("filterType", &params.filter_type);
    Ok(r)
}

/// `GET /v1/secureresize/{thumbPrint}/{hash}/{width}/{height}/{type}/{format}/{filterType}`
///
/// Decrypts and Resizes larger thumbnails to specified size and format
pub async fn get_secureresize_by_thumb_print_by_hash_by_width_by_height_by_type_by_format_by_filter_type(
    client: &Client,
    thumb_print: &str,
    hash: &str,
    width: i32,
    height: i32,
    type_: &str,
    format: &str,
    filter_type_: &str,
    params: &GetSecureresizeByThumbPrintByHashByWidthByHeightByTypeByFormatByFilterTypeParams,
) -> Result<()> {
    client.send(get_secureresize_by_thumb_print_by_hash_by_width_by_height_by_type_by_format_by_filter_type_request(thumb_print, hash, width, height, type_, format, filter_type_, params)?).await?.unit()
}

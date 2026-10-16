//! `contacts.roblox.com` v1 — generated from `vendor/roblox-api-tracker/contacts/v1.json`.
//!
//! Do not edit. Change the spec or `bonsai-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "contacts.roblox.com";

/// `Roblox.Contacts.Api.Request.GetUserTagsRequestModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetUserTagsRequestModel {
    #[serde(rename = "targetUserIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_user_ids: Option<Vec<i64>>,
}

/// `Roblox.Contacts.Api.Request.SetUserTagRequestModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SetUserTagRequestModel {
    /// The userId of the target of the userTag.
    #[serde(rename = "targetUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_user_id: Option<i64>,
    /// The userTag to be set
    #[serde(rename = "userTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_tag: Option<String>,
}

/// `Roblox.Contacts.Api.Response.GetUserTagsResponseModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetUserTagsResponseModel {
    #[serde(rename = "targetUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_user_id: Option<i64>,
    #[serde(rename = "targetUserTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_user_tag: Option<String>,
}

crate::api_enum! {
    str
    /// Status message to be sent to the setter of the userTag \['Success' = 0, 'Moderated' = 1\]
    SetUserTagResponseModelStatus {
    Success = 0 => "Success",
    Moderated = 1 => "Moderated",
    }
}

/// `Roblox.Contacts.Api.Response.SetUserTagResponseModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SetUserTagResponseModel {
    /// Status message to be sent to the setter of the userTag \['Success' = 0, 'Moderated' = 1\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SetUserTagResponseModelStatus>,
}

crate::api_enum! {
    str
    /// Status message to be sent to the requester of the userTag validation \['Success' = 0, 'Moderated' = 1, 'TooLong' = 2\]
    ValidateUserTagResponseModelStatus {
    Success = 0 => "Success",
    Moderated = 1 => "Moderated",
    TooLong = 2 => "TooLong",
    }
}

/// `Roblox.Contacts.Api.Response.ValidateUserTagResponseModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ValidateUserTagResponseModel {
    /// Status message to be sent to the requester of the userTag validation \['Success' = 0, 'Moderated' = 1, 'TooLong' = 2\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ValidateUserTagResponseModelStatus>,
}

/// Query, header and form inputs for [`get_user_tag_validate`].
#[derive(Debug, Clone, Default)]
pub struct GetUserTagValidateParams {
    /// The tag to validate
    pub alias: Option<String>,
}

/// `POST /v1/user/get-tags`
///
/// Gets the tags for multiple users
pub fn post_user_get_tags_request(body: &GetUserTagsRequestModel) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/user/get-tags".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/user/get-tags`
///
/// Gets the tags for multiple users
pub async fn post_user_get_tags(
    client: &Client,
    body: &GetUserTagsRequestModel,
) -> Result<Vec<GetUserTagsResponseModel>> {
    client.send(post_user_get_tags_request(body)?).await?.json()
}

/// `POST /v1/user/tag`
///
/// Sets the tag for a user
pub fn post_user_tag_request(body: &SetUserTagRequestModel) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/user/tag".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/user/tag`
///
/// Sets the tag for a user
pub async fn post_user_tag(
    client: &Client,
    body: &SetUserTagRequestModel,
) -> Result<SetUserTagResponseModel> {
    client.send(post_user_tag_request(body)?).await?.json()
}

/// `GET /v1/user/tag/validate`
///
/// Validates the tag for a user
pub fn get_user_tag_validate_request(params: &GetUserTagValidateParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/user/tag/validate".to_string());
    r.query("alias", &params.alias);
    Ok(r)
}

/// `GET /v1/user/tag/validate`
///
/// Validates the tag for a user
pub async fn get_user_tag_validate(
    client: &Client,
    params: &GetUserTagValidateParams,
) -> Result<ValidateUserTagResponseModel> {
    client.send(get_user_tag_validate_request(params)?).await?.json()
}

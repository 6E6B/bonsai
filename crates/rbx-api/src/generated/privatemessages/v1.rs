//! `privatemessages.roblox.com` v1 — generated from `vendor/roblox-api-tracker/privatemessages/v1.json`.
//!
//! Do not edit. Change the spec or `rbx-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "privatemessages.roblox.com";

/// `Roblox.PrivateMessages.Api.Models.VerifiedSkinnyUserResponse`
///
/// A response model representing user basic information and the user's verified badge status.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct VerifiedSkinnyUserResponse {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Whether the user has a verified badge.
    #[serde(rename = "hasVerifiedBadge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_verified_badge: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.PrivateMessages.Api.Models.AnnouncementsDetailsResponse`
///
/// A message details response.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AnnouncementsDetailsResponse {
    /// The body of the message.
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_: Option<String>,
    /// When the message was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// The message's ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<VerifiedSkinnyUserResponse>,
    /// The subject of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// When the message was last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

/// `Roblox.PrivateMessages.Api.Models.AnnouncementsMetadataResponse`
///
/// A message details response.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AnnouncementsMetadataResponse {
    /// Number of incoming news
    #[serde(rename = "numOfAnnouncements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_of_announcements: Option<i32>,
}

/// `Roblox.PrivateMessages.Api.Models.BatchMessagesRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BatchMessagesRequest {
    #[serde(rename = "messageIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_ids: Option<Vec<i64>>,
}

/// `Roblox.PrivateMessages.Api.Models.FailedMessageResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FailedMessageResponse {
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "messageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
}

/// `Roblox.PrivateMessages.Api.Models.BatchMessagesResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BatchMessagesResponse {
    #[serde(rename = "failedMessages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_messages: Option<Vec<FailedMessageResponse>>,
}

/// `Roblox.PrivateMessages.Api.Models.GetAnnouncementsResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetAnnouncementsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection: Option<Vec<AnnouncementsDetailsResponse>>,
    #[serde(rename = "totalCollectionSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_collection_size: Option<i32>,
}

/// `Roblox.PrivateMessages.Api.Models.MessageDetailsResponse`
///
/// A message details response.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MessageDetailsResponse {
    /// The body of the message.
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_: Option<String>,
    /// When the message was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// The message's ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Whether or not the message has been read.
    #[serde(rename = "isRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_read: Option<bool>,
    /// Whether or not the abuse report link is displayed for the message.
    #[serde(rename = "isReportAbuseDisplayed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_report_abuse_displayed: Option<bool>,
    /// Whether or not the message is a system message.
    #[serde(rename = "isSystemMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_system_message: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<VerifiedSkinnyUserResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<VerifiedSkinnyUserResponse>,
    /// The subject of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// When the message was last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

/// `Roblox.PrivateMessages.Api.Models.GetMessagesResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetMessagesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection: Option<Vec<MessageDetailsResponse>>,
    #[serde(rename = "pageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    #[serde(rename = "totalCollectionSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_collection_size: Option<i64>,
    #[serde(rename = "totalPages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<i32>,
}

/// `Roblox.PrivateMessages.Api.Models.UnreadMessagesCountResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UnreadMessagesCountResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

crate::api_enum! {
    str
    GetMessagesMessageTab {
    Inbox = 0 => "Inbox",
    Sent = 1 => "Sent",
    Archive = 2 => "Archive",
    }
}

/// Query, header and form inputs for [`get_messages`].
#[derive(Debug, Clone, Default)]
pub struct GetMessagesParams {
    pub page_number: Option<i32>,
    pub page_size: Option<i32>,
    pub message_tab: Option<GetMessagesMessageTab>,
}

/// `GET /v1/announcements`
///
/// Migrate from RobloxWebsite project, return news notification for Private Message page
pub fn get_announcements_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/announcements".to_string());
    Ok(r)
}

/// `GET /v1/announcements`
///
/// Migrate from RobloxWebsite project, return news notification for Private Message page
pub async fn get_announcements(client: &Client) -> Result<GetAnnouncementsResponse> {
    client.send(get_announcements_request()?).await?.json()
}

/// `GET /v1/announcements/metadata`
pub fn get_announcements_metadata_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/announcements/metadata".to_string());
    Ok(r)
}

/// `GET /v1/announcements/metadata`
pub async fn get_announcements_metadata(client: &Client) -> Result<AnnouncementsMetadataResponse> {
    client.send(get_announcements_metadata_request()?).await?.json()
}

/// `GET /v1/messages`
///
/// Gets a user's messages.
pub fn get_messages_request(params: &GetMessagesParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/messages".to_string());
    r.query("pageNumber", &params.page_number);
    r.query("pageSize", &params.page_size);
    r.query("messageTab", &params.message_tab);
    Ok(r)
}

/// `GET /v1/messages`
///
/// Gets a user's messages.
pub async fn get_messages(client: &Client, params: &GetMessagesParams) -> Result<GetMessagesResponse> {
    client.send(get_messages_request(params)?).await?.json()
}

/// `POST /v1/messages/archive`
///
/// Archives a batch of messages.
pub fn post_messages_archive_request(body: &BatchMessagesRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/messages/archive".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/messages/archive`
///
/// Archives a batch of messages.
pub async fn post_messages_archive(
    client: &Client,
    body: &BatchMessagesRequest,
) -> Result<BatchMessagesResponse> {
    client.send(post_messages_archive_request(body)?).await?.json()
}

/// `POST /v1/messages/mark-read`
///
/// Marks a batch of messages as read.
pub fn post_messages_mark_read_request(body: &BatchMessagesRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/messages/mark-read".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/messages/mark-read`
///
/// Marks a batch of messages as read.
pub async fn post_messages_mark_read(
    client: &Client,
    body: &BatchMessagesRequest,
) -> Result<BatchMessagesResponse> {
    client.send(post_messages_mark_read_request(body)?).await?.json()
}

/// `POST /v1/messages/mark-unread`
///
/// Marks a batch of messages as unread.
pub fn post_messages_mark_unread_request(body: &BatchMessagesRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/messages/mark-unread".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/messages/mark-unread`
///
/// Marks a batch of messages as unread.
pub async fn post_messages_mark_unread(
    client: &Client,
    body: &BatchMessagesRequest,
) -> Result<BatchMessagesResponse> {
    client
        .send(post_messages_mark_unread_request(body)?)
        .await?
        .json()
}

/// `POST /v1/messages/unarchive`
///
/// Unarchives a batch of messages.
pub fn post_messages_unarchive_request(body: &BatchMessagesRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/messages/unarchive".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/messages/unarchive`
///
/// Unarchives a batch of messages.
pub async fn post_messages_unarchive(
    client: &Client,
    body: &BatchMessagesRequest,
) -> Result<BatchMessagesResponse> {
    client.send(post_messages_unarchive_request(body)?).await?.json()
}

/// `GET /v1/messages/unread/count`
///
/// Gets unread messages for the authenticated user.
pub fn get_messages_unread_count_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/messages/unread/count".to_string());
    Ok(r)
}

/// `GET /v1/messages/unread/count`
///
/// Gets unread messages for the authenticated user.
pub async fn get_messages_unread_count(client: &Client) -> Result<UnreadMessagesCountResponse> {
    client.send(get_messages_unread_count_request()?).await?.json()
}

/// `GET /v1/messages/{messageId}`
///
/// Gets a message's details.
pub fn get_messages_by_message_id_request(message_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/messages/{}", crate::rt::path_value(&message_id)),
    );
    Ok(r)
}

/// `GET /v1/messages/{messageId}`
///
/// Gets a message's details.
pub async fn get_messages_by_message_id(client: &Client, message_id: i64) -> Result<MessageDetailsResponse> {
    client
        .send(get_messages_by_message_id_request(message_id)?)
        .await?
        .json()
}

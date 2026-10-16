//! `accountinformation.roblox.com` v1 — generated from `vendor/roblox-api-tracker/accountinformation/v1.json`.
//!
//! Do not edit. Change the spec or `bonsai-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "accountinformation.roblox.com";

/// `Roblox.AccountInformation.Api.Models.BirthdateResponse`
///
/// The birthdate response
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BirthdateResponse {
    /// The birth day
    #[serde(rename = "birthDay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_day: Option<i32>,
    /// The birth month
    #[serde(rename = "birthMonth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_month: Option<i32>,
    /// The birth year
    #[serde(rename = "birthYear")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_year: Option<i32>,
}

/// `Roblox.AccountInformation.Api.Models.DescriptionRequest`
///
/// The description request
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DescriptionRequest {
    /// The description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// `Roblox.AccountInformation.Api.Models.DescriptionResponse`
///
/// The description response
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DescriptionResponse {
    /// The description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// `Roblox.AccountInformation.Api.Models.EmptyRequest`
///
/// For API calls without a request body.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EmptyRequest {}

/// `Roblox.AccountInformation.Api.Models.GenderRequest`
///
/// The gender request
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GenderRequest {
    /// The gender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
}

/// `Roblox.AccountInformation.Api.Models.GenderResponse`
///
/// The gender response
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GenderResponse {
    /// The gender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<i32>,
}

/// `Roblox.AccountInformation.Api.Models.MetadataResponse`
///
/// The metadata response
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MetadataResponse {
    /// The limit on the length used for description
    #[serde(rename = "MaxUserDescriptionLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_user_description_length: Option<i32>,
    /// The account settings policy enabled setting
    #[serde(rename = "isAccountSettingsPolicyEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_account_settings_policy_enabled: Option<bool>,
    /// Switch for account/settings/allowed-notification-destinations endpoint
    #[serde(rename = "isAllowedNotificationsEndpointDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_allowed_notifications_endpoint_disabled: Option<bool>,
    /// Switch for requiring password to change age below 13.
    #[serde(rename = "isPasswordRequiredForAgingDown")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_password_required_for_aging_down: Option<bool>,
    /// Switch for phone endpoints
    #[serde(rename = "isPhoneNumberEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_phone_number_enabled: Option<bool>,
    /// Switch for determining block endpoints to use for the user
    #[serde(rename = "isUserBlockEndpointsUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_user_block_endpoints_updated: Option<bool>,
    /// Switch for determining if user description is enabled
    #[serde(rename = "isUserDescriptionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_user_description_enabled: Option<bool>,
    /// Whether to display Session Management.
    #[serde(rename = "shouldDisplaySessionManagement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_display_session_management: Option<bool>,
    /// Whether to use Persona for ID verification.
    #[serde(rename = "shouldUsePersonaForIdVerification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_use_persona_for_id_verification: Option<bool>,
}

/// `Roblox.AccountInformation.Api.Models.PhoneRequest`
///
/// The phone request
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PhoneRequest {
    /// The country Code
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// Password
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// The Phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// The Phone Prefix
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// Verification Channel
    #[serde(rename = "verificationChannel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_channel: Option<String>,
}

/// `Roblox.AccountInformation.Api.Models.PhoneResponse`
///
/// The phone response
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PhoneResponse {
    /// Whether user needs to provide password to update their phone numbers
    #[serde(rename = "canBypassPasswordForPhoneUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_bypass_password_for_phone_update: Option<bool>,
    /// The country Code
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// Is the phone verified
    #[serde(rename = "isVerified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_verified: Option<bool>,
    /// The Phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// The Phone Prefix
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// Verification Code Length
    #[serde(rename = "verificationCodeLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_code_length: Option<i32>,
}

/// `Roblox.AccountInformation.Api.Models.PromotionChannelsByUserIdResponse`
///
/// The promotion channels response
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PromotionChannelsByUserIdResponse {
    /// The Facebook channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facebook: Option<String>,
    /// The Twitch channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitch: Option<String>,
    /// The Twitter channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter: Option<String>,
    /// The YouTube channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub youtube: Option<String>,
}

/// `Roblox.AccountInformation.Api.Models.PromotionChannelsRequest`
///
/// The promotion channels request
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PromotionChannelsRequest {
    /// The Facebook channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facebook: Option<String>,
    /// The promotion channels visibility privacy level
    #[serde(rename = "promotionChannelsVisibilityPrivacy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_channels_visibility_privacy: Option<String>,
    /// The Twitch channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitch: Option<String>,
    /// The Twitter channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter: Option<String>,
    /// The YouTube channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub youtube: Option<String>,
}

/// `Roblox.AccountInformation.Api.Models.PromotionChannelsResponse`
///
/// The promotion channels response
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PromotionChannelsResponse {
    /// The Facebook channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facebook: Option<String>,
    /// The promotion channels visibility privacy level
    #[serde(rename = "promotionChannelsVisibilityPrivacy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_channels_visibility_privacy: Option<String>,
    /// The Twitch channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitch: Option<String>,
    /// The Twitter channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter: Option<String>,
    /// The YouTube channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub youtube: Option<String>,
}

/// `Roblox.AccountInformation.Api.Models.VerifyEmailRequest`
///
/// Verify Email Request
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct VerifyEmailRequest {
    /// Ticket to verify email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket: Option<String>,
}

/// `Roblox.AccountInformation.Api.Models.VerifyEmailResponse`
///
/// The verify email response
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct VerifyEmailResponse {
    /// Free item id after email verification
    #[serde(rename = "verifiedUserHatAssetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_user_hat_asset_id: Option<i32>,
}

/// `Roblox.AccountInformation.Api.Models.VerifyPhoneRequest`
///
/// Verify Phone Request
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct VerifyPhoneRequest {
    /// Code to verify phone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

/// `Roblox.AccountInformation.Api.RobloxBadgeResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RobloxBadgeResponse {
    /// The description belonging to this Roblox badge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID belonging to this Roblox badge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// The URL corresponding to the image which represents this Roblox badge.
    #[serde(rename = "imageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// The name of this Roblox badge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.Platform.UserPhoneNumberVerification.Models.PendingVerificationResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PendingVerificationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(rename = "verificationChannel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_channel: Option<String>,
}

/// `Roblox.Web.WebAPI.ApiEmptyResponseModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiEmptyResponseModel {}

/// Query, header and form inputs for [`get_promotion_channels`].
#[derive(Debug, Clone, Default)]
pub struct GetPromotionChannelsParams {
    /// Whether all promotion channel links should be returned as full URLs.
    pub always_return_urls: Option<bool>,
    /// Whether all promotion channel links should be filtered.
    pub filter_link: Option<bool>,
    /// Whether all promotion channels links except for Twitter should be returned as full URLs. If false, all promotion channels will be shortened.
    pub only_shorten_twitter: Option<bool>,
}

/// Query, header and form inputs for [`get_users_by_user_id_promotion_channels`].
#[derive(Debug, Clone, Default)]
pub struct GetUsersByUserIdPromotionChannelsParams {
    /// Whether all promotion channel links should be returned as full URLs.
    pub always_return_urls: Option<bool>,
    /// Whether all promotion channel links should be filtered.
    pub filter_link: Option<bool>,
}

/// `GET /v1/birthdate`
///
/// Get the user's birthdate
#[deprecated(note = "marked deprecated by Roblox")]
pub fn get_birthdate_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/birthdate".to_string());
    Ok(r)
}

/// `GET /v1/birthdate`
///
/// Get the user's birthdate
#[deprecated(note = "marked deprecated by Roblox")]
pub async fn get_birthdate(client: &Client) -> Result<BirthdateResponse> {
    client.send(get_birthdate_request()?).await?.json()
}

/// `GET /v1/description`
///
/// Get the user's description
#[deprecated(note = "marked deprecated by Roblox")]
pub fn get_description_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/description".to_string());
    Ok(r)
}

/// `GET /v1/description`
///
/// Get the user's description
#[deprecated(note = "marked deprecated by Roblox")]
pub async fn get_description(client: &Client) -> Result<DescriptionResponse> {
    client.send(get_description_request()?).await?.json()
}

/// `POST /v1/description`
///
/// Update the user's description
#[deprecated(note = "marked deprecated by Roblox")]
pub fn post_description_request(body: &DescriptionRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/description".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/description`
///
/// Update the user's description
#[deprecated(note = "marked deprecated by Roblox")]
pub async fn post_description(client: &Client, body: &DescriptionRequest) -> Result<DescriptionResponse> {
    client.send(post_description_request(body)?).await?.json()
}

/// `POST /v1/email/verify`
///
/// Verify the user's email address from token
pub fn post_email_verify_request(body: &VerifyEmailRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/email/verify".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/email/verify`
///
/// Verify the user's email address from token
pub async fn post_email_verify(client: &Client, body: &VerifyEmailRequest) -> Result<VerifyEmailResponse> {
    client.send(post_email_verify_request(body)?).await?.json()
}

/// `GET /v1/gender`
///
/// Get the user's gender
#[deprecated(note = "marked deprecated by Roblox")]
pub fn get_gender_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/gender".to_string());
    Ok(r)
}

/// `GET /v1/gender`
///
/// Get the user's gender
#[deprecated(note = "marked deprecated by Roblox")]
pub async fn get_gender(client: &Client) -> Result<GenderResponse> {
    client.send(get_gender_request()?).await?.json()
}

/// `POST /v1/gender`
///
/// Update the user's gender
#[deprecated(note = "marked deprecated by Roblox")]
pub fn post_gender_request(body: &GenderRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/gender".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/gender`
///
/// Update the user's gender
#[deprecated(note = "marked deprecated by Roblox")]
pub async fn post_gender(client: &Client, body: &GenderRequest) -> Result<ApiEmptyResponseModel> {
    client.send(post_gender_request(body)?).await?.json()
}

/// `GET /v1/metadata`
///
/// Get the metadata
pub fn get_metadata_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/metadata".to_string());
    Ok(r)
}

/// `GET /v1/metadata`
///
/// Get the metadata
pub async fn get_metadata(client: &Client) -> Result<MetadataResponse> {
    client.send(get_metadata_request()?).await?.json()
}

/// `GET /v1/phone`
///
/// Get Verified Phone Number
pub fn get_phone_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/phone".to_string());
    Ok(r)
}

/// `GET /v1/phone`
///
/// Get Verified Phone Number
pub async fn get_phone(client: &Client) -> Result<PhoneResponse> {
    client.send(get_phone_request()?).await?.json()
}

/// `POST /v1/phone`
///
/// Set Phone Number
pub fn post_phone_request(body: &PhoneRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/phone".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/phone`
///
/// Set Phone Number
pub async fn post_phone(client: &Client, body: &PhoneRequest) -> Result<PendingVerificationResponse> {
    client.send(post_phone_request(body)?).await?.json()
}

/// `POST /v1/phone/delete`
///
/// Delete Phone
pub fn post_phone_delete_request(body: &PhoneRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/phone/delete".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/phone/delete`
///
/// Delete Phone
pub async fn post_phone_delete(client: &Client, body: &PhoneRequest) -> Result<ApiEmptyResponseModel> {
    client.send(post_phone_delete_request(body)?).await?.json()
}

/// `POST /v1/phone/resend`
///
/// Resend Phone code
pub fn post_phone_resend_request(body: &EmptyRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/phone/resend".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/phone/resend`
///
/// Resend Phone code
pub async fn post_phone_resend(client: &Client, body: &EmptyRequest) -> Result<ApiEmptyResponseModel> {
    client.send(post_phone_resend_request(body)?).await?.json()
}

/// `POST /v1/phone/verify`
///
/// Verify Phone
pub fn post_phone_verify_request(body: &VerifyPhoneRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/phone/verify".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/phone/verify`
///
/// Verify Phone
pub async fn post_phone_verify(client: &Client, body: &VerifyPhoneRequest) -> Result<ApiEmptyResponseModel> {
    client.send(post_phone_verify_request(body)?).await?.json()
}

/// `GET /v1/promotion-channels`
///
/// Get the user's promotion channels
pub fn get_promotion_channels_request(params: &GetPromotionChannelsParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/promotion-channels".to_string());
    r.query("alwaysReturnUrls", &params.always_return_urls);
    r.query("filterLink", &params.filter_link);
    r.query("onlyShortenTwitter", &params.only_shorten_twitter);
    Ok(r)
}

/// `GET /v1/promotion-channels`
///
/// Get the user's promotion channels
pub async fn get_promotion_channels(
    client: &Client,
    params: &GetPromotionChannelsParams,
) -> Result<PromotionChannelsResponse> {
    client.send(get_promotion_channels_request(params)?).await?.json()
}

/// `POST /v1/promotion-channels`
///
/// Update the user's promotion channels
pub fn post_promotion_channels_request(body: &PromotionChannelsRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/promotion-channels".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/promotion-channels`
///
/// Update the user's promotion channels
pub async fn post_promotion_channels(
    client: &Client,
    body: &PromotionChannelsRequest,
) -> Result<ApiEmptyResponseModel> {
    client.send(post_promotion_channels_request(body)?).await?.json()
}

/// `GET /v1/users/{userId}/promotion-channels`
///
/// Get promotion channels for a given user ID
pub fn get_users_by_user_id_promotion_channels_request(
    user_id: i64,
    params: &GetUsersByUserIdPromotionChannelsParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/users/{}/promotion-channels", crate::rt::path_value(&user_id)),
    );
    r.query("alwaysReturnUrls", &params.always_return_urls);
    r.query("filterLink", &params.filter_link);
    Ok(r)
}

/// `GET /v1/users/{userId}/promotion-channels`
///
/// Get promotion channels for a given user ID
pub async fn get_users_by_user_id_promotion_channels(
    client: &Client,
    user_id: i64,
    params: &GetUsersByUserIdPromotionChannelsParams,
) -> Result<PromotionChannelsByUserIdResponse> {
    client
        .send(get_users_by_user_id_promotion_channels_request(user_id, params)?)
        .await?
        .json()
}

/// `GET /v1/users/{userId}/roblox-badges`
///
/// Returns a list of Roblox badges belonging to a user.
pub fn get_users_by_user_id_roblox_badges_request(user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/users/{}/roblox-badges", crate::rt::path_value(&user_id)),
    );
    Ok(r)
}

/// `GET /v1/users/{userId}/roblox-badges`
///
/// Returns a list of Roblox badges belonging to a user.
pub async fn get_users_by_user_id_roblox_badges(
    client: &Client,
    user_id: i64,
) -> Result<Vec<RobloxBadgeResponse>> {
    client
        .send(get_users_by_user_id_roblox_badges_request(user_id)?)
        .await?
        .json()
}

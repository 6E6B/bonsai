//! `accountsettings.roblox.com` v1 — generated from `vendor/roblox-api-tracker/accountsettings/v1.json`.
//!
//! Do not edit. Change the spec or `rbx-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "accountsettings.roblox.com";

/// `Roblox.AccountSettings.Api.CurrentEmailsResponse`
///
/// Response model for getting the user's verified and pending email addresses
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CurrentEmailsResponse {
    /// The user's pending (unverified) email address if one exists (masked)
    #[serde(rename = "pendingEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_email: Option<String>,
    /// The user's most recent verified email address (masked)
    #[serde(rename = "verifiedEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_email: Option<String>,
}

/// `Roblox.AccountSettings.Api.EmailResponse`
///
/// Response model for getting the user's email address and verified status
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EmailResponse {
    /// Whether password is required for updating email
    #[serde(rename = "canBypassPasswordForEmailUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_bypass_password_for_email_update: Option<bool>,
    /// The masked and formatted email address
    #[serde(rename = "emailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// The verified status of the email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
}

/// `Roblox.AccountSettings.Api.Models.AccountsSettingsMetadataModel`
///
/// A model containing website metadata for avatars
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AccountsSettingsMetadataModel {
    /// Whether or not account restrictions spam bug fix is enabled
    #[serde(rename = "IsAccountsRestrictionsSpamBugFixEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_accounts_restrictions_spam_bug_fix_enabled: Option<bool>,
    /// Enables/disables the section in the account parental controls page where you can set the content control settings
    #[serde(rename = "IsContentControlsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_content_controls_enabled: Option<bool>,
    /// Enables/disables the section in the account parental controls page where you can set the monthly spend limit
    #[serde(rename = "IsParentalMonthlyLimitInUIEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_parental_monthly_limit_in_ui_enabled: Option<bool>,
    /// Enables/disables the section in the account parental controls page where you can set the parental notifications settings
    #[serde(rename = "IsParentalNotificationSettingsInUIEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_parental_notification_settings_in_ui_enabled: Option<bool>,
    /// The max amount a user can enter as their monthly spending limit in USD
    #[serde(rename = "MaximumParentalControlsMonthlySpendLimitInUSD")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_parental_controls_monthly_spend_limit_in_usd: Option<i32>,
}

/// `Roblox.AccountSettings.Api.Models.Response.UserAccountCountry`
///
///
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UserAccountCountry {
    ///
    #[serde(rename = "countryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_id: Option<i32>,
    ///
    #[serde(rename = "countryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_name: Option<String>,
    ///
    #[serde(rename = "localizedName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localized_name: Option<String>,
    ///
    #[serde(rename = "localizedSubdivision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localized_subdivision: Option<String>,
    ///
    #[serde(rename = "subdivisionIso")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdivision_iso: Option<String>,
}

/// `Roblox.AccountSettings.Api.Models.Response.AccountCountrySettingsResponse`
///
///
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AccountCountrySettingsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<UserAccountCountry>,
}

/// `Roblox.AccountSettings.Api.Models.Response.UpdateAccountCountryResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateAccountCountryResponse {}

/// `Roblox.AccountSettings.Api.SendVerifyEmailRequest`
///
/// Request model for sending a verify email request
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SendVerifyEmailRequest {
    /// Whether the user will see messaging that they received a free item after verifying their email
    #[serde(rename = "freeItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_item: Option<bool>,
    /// Whether the request is coming from ads site
    #[serde(rename = "isAdsAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_ads_account: Option<bool>,
}

/// `Roblox.AccountSettings.Api.ThemeConfigurationRequest`
///
/// Response model for get user's theme type
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ThemeConfigurationRequest {
    /// theme Type
    #[serde(rename = "themeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_type: Option<String>,
}

/// `Roblox.AccountSettings.Api.ThemeConfigurationResponse`
///
/// Response model for get user's theme type
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ThemeConfigurationResponse {
    /// theme Type
    #[serde(rename = "themeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_type: Option<String>,
}

/// `Roblox.AccountSettings.Api.TradePrivacyResponse`
///
/// Response model for getting the user's trade privacy settings
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TradePrivacyResponse {
    /// The current trade privacy setting for the current user
    #[serde(rename = "tradePrivacy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_privacy: Option<String>,
}

crate::api_enum! {
    int
    /// The inventory privacy setting after updating \['NoOne' = 1, 'Friends' = 2, 'FriendsAndFollowing' = 3, 'FriendsFollowingAndFollowers' = 4, 'AllAuthenticatedUsers' = 5, 'AllUsers' = 6\]
    TradePrivacyUpdateResponseInventoryPrivacy {
    NoOne = 1 => "NoOne",
    Friends = 2 => "Friends",
    FriendsAndFollowing = 3 => "FriendsAndFollowing",
    FriendsFollowingAndFollowers = 4 => "FriendsFollowingAndFollowers",
    AllAuthenticatedUsers = 5 => "AllAuthenticatedUsers",
    AllUsers = 6 => "AllUsers",
    }
}

crate::api_enum! {
    int
    /// The response state after updating trade privacy \['Success' = 0, 'Conflict' = 1\]
    TradePrivacyUpdateResponsePrivacySettingResponse {
    Success = 0 => "Success",
    Conflict = 1 => "Conflict",
    }
}

crate::api_enum! {
    int
    /// The trade privacy setting after updating \['Undefined' = 0, 'Disabled' = 1, 'NoOne' = 2, 'Friends' = 3, 'TopFriends' = 4, 'Following' = 5, 'Followers' = 6, 'All' = 7\]
    TradePrivacyUpdateResponseTradePrivacy {
    Undefined = 0 => "Undefined",
    Disabled = 1 => "Disabled",
    NoOne = 2 => "NoOne",
    Friends = 3 => "Friends",
    TopFriends = 4 => "TopFriends",
    Following = 5 => "Following",
    Followers = 6 => "Followers",
    All = 7 => "All",
    }
}

/// `Roblox.AccountSettings.Api.TradePrivacyUpdateResponse`
///
/// Response model for updating the user's trade privacy settings
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TradePrivacyUpdateResponse {
    /// The inventory privacy setting after updating \['NoOne' = 1, 'Friends' = 2, 'FriendsAndFollowing' = 3, 'FriendsFollowingAndFollowers' = 4, 'AllAuthenticatedUsers' = 5, 'AllUsers' = 6\]
    #[serde(rename = "inventoryPrivacy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_privacy: Option<TradePrivacyUpdateResponseInventoryPrivacy>,
    /// The response state after updating trade privacy \['Success' = 0, 'Conflict' = 1\]
    #[serde(rename = "privacySettingResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_setting_response: Option<TradePrivacyUpdateResponsePrivacySettingResponse>,
    /// The trade privacy setting after updating \['Undefined' = 0, 'Disabled' = 1, 'NoOne' = 2, 'Friends' = 3, 'TopFriends' = 4, 'Following' = 5, 'Followers' = 6, 'All' = 7\]
    #[serde(rename = "tradePrivacy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_privacy: Option<TradePrivacyUpdateResponseTradePrivacy>,
}

crate::api_enum! {
    int
    /// The desired trade value setting for the active user \['Undefined' = 0, 'None' = 1, 'Low' = 2, 'Medium' = 3, 'High' = 4\]
    TradeValueRequestTradeValue {
    Undefined = 0 => "Undefined",
    None = 1 => "None",
    Low = 2 => "Low",
    Medium = 3 => "Medium",
    High = 4 => "High",
    }
}

/// `Roblox.AccountSettings.Api.TradeValueRequest`
///
/// Request model for trade value setting update
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TradeValueRequest {
    /// The desired trade value setting for the active user \['Undefined' = 0, 'None' = 1, 'Low' = 2, 'Medium' = 3, 'High' = 4\]
    #[serde(rename = "tradeValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_value: Option<TradeValueRequestTradeValue>,
}

/// `Roblox.AccountSettings.Api.TradeValueResponse`
///
/// Response model for getting the user's trade value settings
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TradeValueResponse {
    /// The current trade value setting for the current user
    #[serde(rename = "tradeValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_value: Option<String>,
}

/// `Roblox.AccountSettings.Api.UpdateAccountCountryRequest`
///
/// Request Model for updating a user's account country
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateAccountCountryRequest {
    /// The targetCountryId
    #[serde(rename = "targetCountryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_country_id: Option<i32>,
}

/// `Roblox.AccountSettings.Api.UpdateEmailRequest`
///
/// Request model for updating an email
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateEmailRequest {
    /// The new email address to set.
    #[serde(rename = "emailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// Whether the request is coming from ads site.
    #[serde(rename = "isAdsAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_ads_account: Option<bool>,
    /// The user's password.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Should the email controller skip sending the verification email.
    #[serde(rename = "skipVerificationEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_verification_email: Option<bool>,
}

/// `Roblox.AccountSettings.Api.UpdateTradePrivacyRequest`
///
/// Request model for trade privacy setting update
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateTradePrivacyRequest {
    /// The desired trade privacy setting for the active user \['Undefined' = 0, 'Disabled' = 1, 'NoOne' = 2, 'Friends' = 3, 'TopFriends' = 4, 'Following' = 5, 'Followers' = 6, 'All' = 7\]
    #[serde(rename = "tradePrivacy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_privacy: Option<TradePrivacyUpdateResponseTradePrivacy>,
}

/// `Roblox.Web.WebAPI.ApiEmptyResponseModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiEmptyResponseModel {}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[System.String\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseString {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<String>>,
}

/// `GET /v1/account/settings/account-country`
///
/// Get a user's current account country setting.
pub fn get_account_settings_account_country_request() -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        "/v1/account/settings/account-country".to_string(),
    );
    Ok(r)
}

/// `GET /v1/account/settings/account-country`
///
/// Get a user's current account country setting.
pub async fn get_account_settings_account_country(client: &Client) -> Result<AccountCountrySettingsResponse> {
    client
        .send(get_account_settings_account_country_request()?)
        .await?
        .json()
}

/// `POST /v1/account/settings/account-country`
///
/// Updates the user's account country.
pub fn post_account_settings_account_country_request(body: &UpdateAccountCountryRequest) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        "/v1/account/settings/account-country".to_string(),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/account/settings/account-country`
///
/// Updates the user's account country.
pub async fn post_account_settings_account_country(
    client: &Client,
    body: &UpdateAccountCountryRequest,
) -> Result<UpdateAccountCountryResponse> {
    client
        .send(post_account_settings_account_country_request(body)?)
        .await?
        .json()
}

/// `GET /v1/account/settings/metadata`
///
/// Returns metadata used by the account settings page
pub fn get_account_settings_metadata_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/account/settings/metadata".to_string());
    Ok(r)
}

/// `GET /v1/account/settings/metadata`
///
/// Returns metadata used by the account settings page
pub async fn get_account_settings_metadata(client: &Client) -> Result<AccountsSettingsMetadataModel> {
    client
        .send(get_account_settings_metadata_request()?)
        .await?
        .json()
}

/// `GET /v1/email`
///
/// Gets the authenticated user's email address and verified status
pub fn get_email_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/email".to_string());
    Ok(r)
}

/// `GET /v1/email`
///
/// Gets the authenticated user's email address and verified status
pub async fn get_email(client: &Client) -> Result<EmailResponse> {
    client.send(get_email_request()?).await?.json()
}

/// `POST /v1/email`
///
/// Updates the authenticated user's email address
pub fn post_email_request(body: &UpdateEmailRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/email".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/email`
///
/// Updates the authenticated user's email address
pub async fn post_email(client: &Client, body: &UpdateEmailRequest) -> Result<ApiEmptyResponseModel> {
    client.send(post_email_request(body)?).await?.json()
}

/// `PATCH /v1/email`
///
/// Updates the authenticated user's email address
pub fn patch_email_request(body: &UpdateEmailRequest) -> Result<Request> {
    let mut r = Request::new(Method::PATCH, HOST, "/v1/email".to_string());
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/email`
///
/// Updates the authenticated user's email address
pub async fn patch_email(client: &Client, body: &UpdateEmailRequest) -> Result<ApiEmptyResponseModel> {
    client.send(patch_email_request(body)?).await?.json()
}

/// `POST /v1/email/verify`
///
/// Send verify email to the authenticated user's email address
pub fn post_email_verify_request(body: &SendVerifyEmailRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/email/verify".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/email/verify`
///
/// Send verify email to the authenticated user's email address
pub async fn post_email_verify(
    client: &Client,
    body: &SendVerifyEmailRequest,
) -> Result<ApiEmptyResponseModel> {
    client.send(post_email_verify_request(body)?).await?.json()
}

/// `GET /v1/emails`
///
/// Gets the authenticated user's verified email and pending (unverified) email if one exists
pub fn get_emails_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/emails".to_string());
    Ok(r)
}

/// `GET /v1/emails`
///
/// Gets the authenticated user's verified email and pending (unverified) email if one exists
pub async fn get_emails(client: &Client) -> Result<CurrentEmailsResponse> {
    client.send(get_emails_request()?).await?.json()
}

/// `GET /v1/themes/types`
///
/// returns all the enabled theme types.
pub fn get_themes_types_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/themes/types".to_string());
    Ok(r)
}

/// `GET /v1/themes/types`
///
/// returns all the enabled theme types.
pub async fn get_themes_types(client: &Client) -> Result<ApiArrayResponseString> {
    client.send(get_themes_types_request()?).await?.json()
}

/// `GET /v1/themes/{consumerType}/{consumerId}`
///
/// returns the theme type for a specific consumer.
pub fn get_themes_by_consumer_type_by_consumer_id_request(
    consumer_type: i32,
    consumer_id: &str,
) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/themes/{}/{}",
            crate::rt::path_value(&consumer_type),
            crate::rt::path_value(&consumer_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/themes/{consumerType}/{consumerId}`
///
/// returns the theme type for a specific consumer.
pub async fn get_themes_by_consumer_type_by_consumer_id(
    client: &Client,
    consumer_type: i32,
    consumer_id: &str,
) -> Result<ThemeConfigurationResponse> {
    client
        .send(get_themes_by_consumer_type_by_consumer_id_request(
            consumer_type,
            consumer_id,
        )?)
        .await?
        .json()
}

/// `PATCH /v1/themes/{consumerType}/{consumerId}`
///
/// Modify the theme type for consumer.
pub fn patch_themes_by_consumer_type_by_consumer_id_request(
    consumer_type: i32,
    consumer_id: i64,
    body: &ThemeConfigurationRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!(
            "/v1/themes/{}/{}",
            crate::rt::path_value(&consumer_type),
            crate::rt::path_value(&consumer_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/themes/{consumerType}/{consumerId}`
///
/// Modify the theme type for consumer.
pub async fn patch_themes_by_consumer_type_by_consumer_id(
    client: &Client,
    consumer_type: i32,
    consumer_id: i64,
    body: &ThemeConfigurationRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(patch_themes_by_consumer_type_by_consumer_id_request(
            consumer_type,
            consumer_id,
            body,
        )?)
        .await?
        .json()
}

/// `GET /v1/trade-privacy`
///
/// Get a user's trade privacy setting
pub fn get_trade_privacy_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/trade-privacy".to_string());
    Ok(r)
}

/// `GET /v1/trade-privacy`
///
/// Get a user's trade privacy setting
pub async fn get_trade_privacy(client: &Client) -> Result<TradePrivacyResponse> {
    client.send(get_trade_privacy_request()?).await?.json()
}

/// `POST /v1/trade-privacy`
///
/// Updates a user's trade privacy setting
pub fn post_trade_privacy_request(body: &UpdateTradePrivacyRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/trade-privacy".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/trade-privacy`
///
/// Updates a user's trade privacy setting
pub async fn post_trade_privacy(
    client: &Client,
    body: &UpdateTradePrivacyRequest,
) -> Result<TradePrivacyUpdateResponse> {
    client.send(post_trade_privacy_request(body)?).await?.json()
}

/// `GET /v1/trade-value`
///
/// Get a user's trade quality filter setting
pub fn get_trade_value_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/trade-value".to_string());
    Ok(r)
}

/// `GET /v1/trade-value`
///
/// Get a user's trade quality filter setting
pub async fn get_trade_value(client: &Client) -> Result<TradeValueResponse> {
    client.send(get_trade_value_request()?).await?.json()
}

/// `POST /v1/trade-value`
///
/// Updates a user's trade quality filter setting
pub fn post_trade_value_request(body: &TradeValueRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/trade-value".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/trade-value`
///
/// Updates a user's trade quality filter setting
pub async fn post_trade_value(client: &Client, body: &TradeValueRequest) -> Result<ApiEmptyResponseModel> {
    client.send(post_trade_value_request(body)?).await?.json()
}

//! `locale.roblox.com` v1 — generated from `vendor/roblox-api-tracker/locale/v1.json`.
//!
//! Do not edit. Change the spec or `rbx-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "locale.roblox.com";

/// `Roblox.Locale.Api.CountryRegion`
///
/// Model for Country Regions
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CountryRegion {
    /// code of country region
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// localized name of country region. Example "Afghanistan"
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// native name of country region
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.Locale.Api.CountryRegionListResponse`
///
/// Returns list of supported country/regions
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CountryRegionListResponse {
    /// List of supported country/regions. Will be empty on error.
    #[serde(rename = "countryRegionList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_region_list: Option<Vec<CountryRegion>>,
}

/// `Roblox.Locale.Api.Language`
///
/// Model for Language
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Language {
    /// id of language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// whether or not the language is read right-to-left
    #[serde(rename = "isRightToLeft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_right_to_left: Option<bool>,
    /// language code of language
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// name of language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// native name of language
    #[serde(rename = "nativeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_name: Option<String>,
}

/// `Roblox.Locale.Api.SetShowRobloxTranslationsRequest`
///
/// Request entity to set the ShowRobloxTranslations field for an account
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SetShowRobloxTranslationsRequest {
    /// Value to set the ShowRobloxTranslations field to
    #[serde(rename = "showRobloxTranslations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_roblox_translations: Option<bool>,
}

/// `Roblox.Locale.Api.SetSupportedLocaleForUserRequest`
///
/// Request entity to set Supported Locale for user
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SetSupportedLocaleForUserRequest {
    /// SupportedLocale code
    #[serde(rename = "supportedLocaleCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_locale_code: Option<String>,
}

/// `Roblox.Locale.Api.SuccessResponse`
///
/// Tells the operation is successful or not
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SuccessResponse {
    /// Returns true on success otherwise false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

/// `Roblox.Locale.Api.SupportedLocale`
///
/// Model for Supported locale
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SupportedLocale {
    /// id of supported locale
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,
    /// locale of supported locale. Example "en-us"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// Name of supported locale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Name of supported locale in native language. Example "English"
    #[serde(rename = "nativeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_name: Option<String>,
}

/// `Roblox.Locale.Api.SupportedLocaleLocus`
///
/// Model for Supported locale with user locus information
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SupportedLocaleLocus {
    /// Is locale enabled for full experience
    #[serde(rename = "isEnabledForFullExperience")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled_for_full_experience: Option<bool>,
    /// Is locale enabled for in game ugc
    #[serde(rename = "isEnabledForInGameUgc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled_for_in_game_ugc: Option<bool>,
    /// Is locale enabled for signup and login
    #[serde(rename = "isEnabledForSignupAndLogin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled_for_signup_and_login: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<SupportedLocale>,
}

/// `Roblox.Locale.Api.SupportedLocalesResponse`
///
/// Returns list of supported locales
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SupportedLocalesResponse {
    /// List of supported locales. Will be empty on error.
    #[serde(rename = "supportedLocales")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_locales: Option<Vec<SupportedLocale>>,
}

/// `Roblox.Locale.Api.UserLocaleResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UserLocaleResponse {
    #[serde(rename = "nativeLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_language: Option<Language>,
    #[serde(rename = "supportedLocale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_locale: Option<SupportedLocale>,
}

/// `Roblox.Locale.Api.UserLocalizationLocusLocalesResponse`
///
/// Returns available Roblox.Locale.Api.SupportedLocale models.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UserLocalizationLocusLocalesResponse {
    #[serde(rename = "generalExperience")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_experience: Option<SupportedLocale>,
    /// Whether Roblox-suggested translations should be shown to the user.
    #[serde(rename = "showRobloxTranslations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_roblox_translations: Option<bool>,
    #[serde(rename = "signupAndLogin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signup_and_login: Option<SupportedLocale>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ugc: Option<SupportedLocale>,
}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.Locale.Api.SupportedLocaleLocus\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseSupportedLocaleLocus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<SupportedLocaleLocus>>,
}

/// Query, header and form inputs for [`get_country_regions`].
#[derive(Debug, Clone, Default)]
pub struct GetCountryRegionsParams {
    pub locale: Option<String>,
}

/// Query, header and form inputs for [`get_locales`].
#[derive(Debug, Clone, Default)]
pub struct GetLocalesParams {
    pub display_value_locale: Option<String>,
}

/// Query, header and form inputs for [`get_locales_supported_locales_for_creators`].
#[derive(Debug, Clone, Default)]
pub struct GetLocalesSupportedLocalesForCreatorsParams {
    pub display_value_locale: Option<String>,
}

/// Query, header and form inputs for [`get_locales_supported_locales_for_feature`].
#[derive(Debug, Clone, Default)]
pub struct GetLocalesSupportedLocalesForFeatureParams {
    pub feature_name: String,
}

/// `GET /v1/country-regions`
///
/// Get list of country regions sorted by localized name
pub fn get_country_regions_request(params: &GetCountryRegionsParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/country-regions".to_string());
    r.query("locale", &params.locale);
    Ok(r)
}

/// `GET /v1/country-regions`
///
/// Get list of country regions sorted by localized name
pub async fn get_country_regions(
    client: &Client,
    params: &GetCountryRegionsParams,
) -> Result<CountryRegionListResponse> {
    client.send(get_country_regions_request(params)?).await?.json()
}

/// `GET /v1/locales`
///
/// Get list of Supported locales with user locus information.
pub fn get_locales_request(params: &GetLocalesParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/locales".to_string());
    r.query("displayValueLocale", &params.display_value_locale);
    Ok(r)
}

/// `GET /v1/locales`
///
/// Get list of Supported locales with user locus information.
pub async fn get_locales(
    client: &Client,
    params: &GetLocalesParams,
) -> Result<ApiArrayResponseSupportedLocaleLocus> {
    client.send(get_locales_request(params)?).await?.json()
}

/// `POST /v1/locales/set-show-roblox-translations`
///
/// Sets whether translations suggested by Roblox will be shown to the user.
pub fn post_locales_set_show_roblox_translations_request(
    body: &SetShowRobloxTranslationsRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        "/v1/locales/set-show-roblox-translations".to_string(),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/locales/set-show-roblox-translations`
///
/// Sets whether translations suggested by Roblox will be shown to the user.
pub async fn post_locales_set_show_roblox_translations(
    client: &Client,
    body: &SetShowRobloxTranslationsRequest,
) -> Result<SuccessResponse> {
    client
        .send(post_locales_set_show_roblox_translations_request(body)?)
        .await?
        .json()
}

/// `POST /v1/locales/set-user-supported-locale`
///
/// Sets user's supported locale. Null supported locale will clear out user's supported locale (set users' supported locale to null)
pub fn post_locales_set_user_supported_locale_request(
    body: &SetSupportedLocaleForUserRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        "/v1/locales/set-user-supported-locale".to_string(),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/locales/set-user-supported-locale`
///
/// Sets user's supported locale. Null supported locale will clear out user's supported locale (set users' supported locale to null)
pub async fn post_locales_set_user_supported_locale(
    client: &Client,
    body: &SetSupportedLocaleForUserRequest,
) -> Result<SuccessResponse> {
    client
        .send(post_locales_set_user_supported_locale_request(body)?)
        .await?
        .json()
}

/// `GET /v1/locales/supported-locales`
///
/// Get list of supported locales sorted by the Native Name property.
pub fn get_locales_supported_locales_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/locales/supported-locales".to_string());
    Ok(r)
}

/// `GET /v1/locales/supported-locales`
///
/// Get list of supported locales sorted by the Native Name property.
pub async fn get_locales_supported_locales(client: &Client) -> Result<SupportedLocalesResponse> {
    client
        .send(get_locales_supported_locales_request()?)
        .await?
        .json()
}

/// `GET /v1/locales/supported-locales-for-creators`
pub fn get_locales_supported_locales_for_creators_request(
    params: &GetLocalesSupportedLocalesForCreatorsParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        "/v1/locales/supported-locales-for-creators".to_string(),
    );
    r.query("displayValueLocale", &params.display_value_locale);
    Ok(r)
}

/// `GET /v1/locales/supported-locales-for-creators`
pub async fn get_locales_supported_locales_for_creators(
    client: &Client,
    params: &GetLocalesSupportedLocalesForCreatorsParams,
) -> Result<ApiArrayResponseSupportedLocaleLocus> {
    client
        .send(get_locales_supported_locales_for_creators_request(params)?)
        .await?
        .json()
}

/// `GET /v1/locales/supported-locales-for-feature`
///
/// Get list of Supported locales for a specific feature.
pub fn get_locales_supported_locales_for_feature_request(
    params: &GetLocalesSupportedLocalesForFeatureParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        "/v1/locales/supported-locales-for-feature".to_string(),
    );
    r.query("featureName", &params.feature_name);
    Ok(r)
}

/// `GET /v1/locales/supported-locales-for-feature`
///
/// Get list of Supported locales for a specific feature.
pub async fn get_locales_supported_locales_for_feature(
    client: &Client,
    params: &GetLocalesSupportedLocalesForFeatureParams,
) -> Result<SupportedLocalesResponse> {
    client
        .send(get_locales_supported_locales_for_feature_request(params)?)
        .await?
        .json()
}

/// `GET /v1/locales/user-locale`
///
/// Gets user locale. If user is absent returns, locale from http request object.
pub fn get_locales_user_locale_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/locales/user-locale".to_string());
    Ok(r)
}

/// `GET /v1/locales/user-locale`
///
/// Gets user locale. If user is absent returns, locale from http request object.
pub async fn get_locales_user_locale(client: &Client) -> Result<UserLocaleResponse> {
    client.send(get_locales_user_locale_request()?).await?.json()
}

/// `GET /v1/locales/user-localization-locus-supported-locales`
///
/// Gets each of a user's localization locus supported locales. A localization locus supported locale is a page (or group of pages) that have been defined by the International team which need independent locale support. If the user is null we will attempt to return the locales appropriate for the user's device language.
pub fn get_locales_user_localization_locus_supported_locales_request() -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        "/v1/locales/user-localization-locus-supported-locales".to_string(),
    );
    Ok(r)
}

/// `GET /v1/locales/user-localization-locus-supported-locales`
///
/// Gets each of a user's localization locus supported locales. A localization locus supported locale is a page (or group of pages) that have been defined by the International team which need independent locale support. If the user is null we will attempt to return the locales appropriate for the user's device language.
pub async fn get_locales_user_localization_locus_supported_locales(
    client: &Client,
) -> Result<UserLocalizationLocusLocalesResponse> {
    client
        .send(get_locales_user_localization_locus_supported_locales_request()?)
        .await?
        .json()
}

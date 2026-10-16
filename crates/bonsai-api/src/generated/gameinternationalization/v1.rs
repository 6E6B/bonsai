//! `gameinternationalization.roblox.com` v1 — generated from `vendor/roblox-api-tracker/gameinternationalization/v1.json`.
//!
//! Do not edit. Change the spec or `bonsai-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "gameinternationalization.roblox.com";

/// `Roblox.GameInternationalization.Api.LocalizationTableGameAssociation`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LocalizationTableGameAssociation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dissociate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// `Roblox.GameInternationalization.Api.AssociateLocalizationTablesToGameRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AssociateLocalizationTablesToGameRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables: Option<Vec<LocalizationTableGameAssociation>>,
}

/// `Roblox.GameInternationalization.Api.AssociateLocalizationTablesToGameResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AssociateLocalizationTablesToGameResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

/// `Roblox.GameInternationalization.Api.AutoLocalizationMetadataResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AutoLocalizationMetadataResponse {
    /// Is Quota UI Enabled for Automatic Translations
    #[serde(rename = "isAutomaticTranslationQuotaUIEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_automatic_translation_quota_ui_enabled: Option<bool>,
    /// Is Toggle UI Enabled for Automatic Translations
    #[serde(rename = "isAutomaticTranslationToggleUIEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_automatic_translation_toggle_ui_enabled: Option<bool>,
    /// Is React Implementation of AutoLocalization Settings Enabled
    #[serde(rename = "isReactVersionEnabledForAutoLocalizationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_react_version_enabled_for_auto_localization_settings: Option<bool>,
    /// Is Tabbed UI Enabled for Configure Localization Page
    #[serde(rename = "isTabbedUIEnabledForConfigureLocalizationPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_tabbed_ui_enabled_for_configure_localization_page: Option<bool>,
}

/// `Roblox.GameInternationalization.Api.AutomaticTranslationStatusTargetLanguage`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AutomaticTranslationStatusTargetLanguage {
    /// Indicates whether or not automatic translation is allowed for the target language.
    #[serde(rename = "isAutomaticTranslationAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_automatic_translation_allowed: Option<bool>,
    /// The language code.
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

/// `Roblox.GameInternationalization.Api.EditAutomaticTranslationStatusForGameAndLanguageResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EditAutomaticTranslationStatusForGameAndLanguageResponse {
    /// The game id.
    #[serde(rename = "gameId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_id: Option<i64>,
    /// Indicates whether or not automatic translation is currently enabled for the game and language.
    #[serde(rename = "isAutomaticTranslationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_automatic_translation_enabled: Option<bool>,
    /// The language code.
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

/// `Roblox.GameInternationalization.Api.EditImageTranslationStatusForGameAndLanguageResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EditImageTranslationStatusForGameAndLanguageResponse {
    /// The game id.
    #[serde(rename = "gameId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_id: Option<i64>,
    /// Indicates whether image translation is currently enabled for the game and language.
    #[serde(rename = "isImageTranslationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_image_translation_enabled: Option<bool>,
    /// The language code.
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

/// `Roblox.GameInternationalization.Api.FailedNameDescription`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FailedNameDescription {
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

/// `Roblox.GameInternationalization.Api.GameAutolocalizationInformationResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GameAutolocalizationInformationResponse {
    #[serde(rename = "assetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<i64>,
    #[serde(rename = "autoLocalizationTableId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_localization_table_id: Option<String>,
    #[serde(rename = "isAutoLocalizationForImageEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_auto_localization_for_image_enabled: Option<bool>,
    #[serde(rename = "isAutolocalizationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_autolocalization_enabled: Option<bool>,
    #[serde(rename = "shouldUseImageLocalizationTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_use_image_localization_table: Option<bool>,
    #[serde(rename = "shouldUseLocalizationTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_use_localization_table: Option<bool>,
}

/// `Roblox.GameInternationalization.Api.GetAllowedAutomaticTranslationStatusForLanguagesResponse`
///
/// A response model for getting the automatic translation allowed status of target languages.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetAllowedAutomaticTranslationStatusForLanguagesResponse {
    /// The source language.
    #[serde(rename = "sourceLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_language: Option<String>,
    /// The target languages with the automatic translation allowed status. The status basically says if automatic translation can be enabled for the source and target.
    #[serde(rename = "targetLanguages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_languages: Option<Vec<AutomaticTranslationStatusTargetLanguage>>,
}

/// `Roblox.GameInternationalization.Api.GetAutomaticTranslationFeatureStatusForGameResponse`
///
/// A response model for getting the automatic translation allowed status of a game.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetAutomaticTranslationFeatureStatusForGameResponse {
    /// The game id.
    #[serde(rename = "gameId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_id: Option<i64>,
    /// Indicates whether or not automatic translation is allowed for the game.
    #[serde(rename = "isAutomaticTranslationAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_automatic_translation_allowed: Option<bool>,
    /// Indicates whether or not automatic translation switches are enabled for the game.
    #[serde(rename = "isAutomaticTranslationSwitchesUIEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_automatic_translation_switches_ui_enabled: Option<bool>,
}

/// `Roblox.GameInternationalization.Api.QuotaModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct QuotaModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining: Option<i64>,
}

/// `Roblox.GameInternationalization.Api.MonthlyQuotaModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MonthlyQuotaModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
    #[serde(rename = "nextRefreshDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_refresh_date: Option<String>,
    #[serde(rename = "previousRefreshDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_refresh_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining: Option<i64>,
}

/// `Roblox.GameInternationalization.Api.GetAutomaticTranslationQuotaForGameResponse`
///
/// A response model for getting the automatic translation quota info of a game.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetAutomaticTranslationQuotaForGameResponse {
    #[serde(rename = "bankQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_quota: Option<QuotaModel>,
    #[serde(rename = "monthlyQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_quota: Option<MonthlyQuotaModel>,
}

crate::api_enum! {
    str
    /// Enum for image status.
    GetBadgeIconResponseState {
    Approved = 0 => "Approved",
    PendingReview = 1 => "PendingReview",
    UnAvailable = 2 => "UnAvailable",
    Rejected = 3 => "Rejected",
    Error = 4 => "Error",
    }
}

/// `Roblox.GameInternationalization.Api.GetBadgeIconResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetBadgeIconResponse {
    #[serde(rename = "imageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "imageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Enum for image status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<GetBadgeIconResponseState>,
}

/// `Roblox.GameInternationalization.Api.GetDeveloperProductIconResponse`
///
/// A response model for getting developer product icon
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetDeveloperProductIconResponse {
    #[serde(rename = "imageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "imageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Enum for image status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<GetBadgeIconResponseState>,
}

/// `Roblox.GameInternationalization.Api.GetGameIconResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetGameIconResponse {
    #[serde(rename = "imageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "imageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Enum for image status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<GetBadgeIconResponseState>,
}

/// `Roblox.GameInternationalization.Api.GetGamePassIconResponse`
///
/// A response model for getting game pass icon
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetGamePassIconResponse {
    #[serde(rename = "imageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "imageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Enum for image status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<GetBadgeIconResponseState>,
}

/// `Roblox.GameInternationalization.Api.MediaAssetResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MediaAssetResponse {
    #[serde(rename = "mediaAssetAltText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_asset_alt_text: Option<String>,
    #[serde(rename = "mediaAssetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_asset_id: Option<String>,
    #[serde(rename = "mediaAssetUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_asset_url: Option<String>,
    /// Enum for image status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<GetBadgeIconResponseState>,
}

/// `Roblox.GameInternationalization.Api.GetGameThumbnailsResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetGameThumbnailsResponse {
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "mediaAssets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_assets: Option<Vec<MediaAssetResponse>>,
}

crate::api_enum! {
    str
    /// \['User' = 0, 'Automation' = 1\]
    TranslatorAgentType {
    User = 0 => "User",
    Automation = 1 => "Automation",
    }
}

/// `Roblox.GameInternationalization.Api.Translator`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Translator {
    /// \['User' = 0, 'Automation' = 1\]
    #[serde(rename = "agentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_type: Option<TranslatorAgentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
}

/// `Roblox.GameInternationalization.Api.TranslationHistory`
///
/// A model containing a translation, translator, and translation time.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TranslationHistory {
    /// The time the translation was provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// The translation provided by the translator.
    #[serde(rename = "translationText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translator: Option<Translator>,
}

/// `Roblox.GameInternationalization.Api.GetNameDescriptionHistoryResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetNameDescriptionHistoryResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<Vec<TranslationHistory>>,
    #[serde(rename = "lastEvaluatedId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_evaluated_id: Option<String>,
}

crate::api_enum! {
    str
    /// The enum representing the type of request while requesting name/description history
    GetNameDescriptionHistoryV2RequestContentType {
    UniverseDisplayNames = 0 => "UniverseDisplayNames",
    UniverseDisplayDescriptions = 1 => "UniverseDisplayDescriptions",
    BadgeDisplayName = 2 => "BadgeDisplayName",
    BadgeDisplayDescription = 3 => "BadgeDisplayDescription",
    DeveloperProductDisplayName = 4 => "DeveloperProductDisplayName",
    DeveloperProductDisplayDescription = 5 => "DeveloperProductDisplayDescription",
    GamePassDisplayName = 6 => "GamePassDisplayName",
    GamePassDisplayDescription = 7 => "GamePassDisplayDescription",
    }
}

crate::api_enum! {
    str
    /// \['Asc' = 1, 'Desc' = 2\]
    GetNameDescriptionHistoryV2RequestSortOrder {
    Asc = 0 => "Asc",
    Desc = 1 => "Desc",
    }
}

/// `Roblox.GameInternationalization.Api.GetNameDescriptionHistoryV2Request`
///
/// A model containing request for getting the translation history of a content source type's name or description.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetNameDescriptionHistoryV2Request {
    #[serde(rename = "contentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_id: Option<i64>,
    /// The enum representing the type of request while requesting name/description history
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<GetNameDescriptionHistoryV2RequestContentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// \['Asc' = 1, 'Desc' = 2\]
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<GetNameDescriptionHistoryV2RequestSortOrder>,
}

crate::api_enum! {
    str
    /// The category of the translation counts.
    TranslationCountCategoryInfoResponseCategory {
    InGameContent = 0 => "InGameContent",
    }
}

crate::api_enum! {
    str
    /// The enum representing a translation status.
    TranslationCountTranslationStatus {
    Approved = 0 => "Approved",
    }
}

crate::api_enum! {
    str
    /// Gets or sets the type of the translator associated with the count, or 'null' if the count is not associated with a specific type of translator.
    TranslationCountTranslatorType {
    Automation = 0 => "Automation",
    User = 1 => "User",
    }
}

/// `Roblox.GameInternationalization.Api.TranslationCount`
///
/// A model that contains the count value of a translation count and relevant metadata.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TranslationCount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// The enum representing a translation status.
    #[serde(rename = "translationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation_status: Option<TranslationCountTranslationStatus>,
    /// Gets or sets the type of the translator associated with the count, or 'null' if the count is not associated with a specific type of translator.
    #[serde(rename = "translatorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translator_type: Option<TranslationCountTranslatorType>,
}

/// `Roblox.GameInternationalization.Api.TranslationCountCategoryInfoResponse`
///
/// A response model that contains category info for getting a translation count.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TranslationCountCategoryInfoResponse {
    /// The category of the translation counts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<TranslationCountCategoryInfoResponseCategory>,
    /// The total number of translatable items. The translation count percentage can be calculated by doing translationCounts / totalTranslatableItemCount.
    #[serde(rename = "totalTranslatableItemCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_translatable_item_count: Option<i32>,
    /// The translation counts.
    #[serde(rename = "translationCounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation_counts: Option<Vec<TranslationCount>>,
}

crate::api_enum! {
    str
    /// An enum to distinguish between locale codes and language codes.
    TranslationCountLanguageOrLocaleResponseLanguageCodeType {
    Language = 0 => "Language",
    Locale = 1 => "Locale",
    }
}

crate::api_enum! {
    str
    /// The status of the response for the requested game.
    TranslationCountLanguageOrLocaleResponseStatus {
    Success = 0 => "Success",
    LanguageOrLocaleNotSupportedForGame = 1 => "LanguageOrLocaleNotSupportedForGame",
    }
}

/// `Roblox.GameInternationalization.Api.TranslationCountLanguageOrLocaleResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TranslationCountLanguageOrLocaleResponse {
    /// Categories of translation counts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<TranslationCountCategoryInfoResponse>>,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// An enum to distinguish between locale codes and language codes.
    #[serde(rename = "languageCodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code_type: Option<TranslationCountLanguageOrLocaleResponseLanguageCodeType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The status of the response for the requested game.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TranslationCountLanguageOrLocaleResponseStatus>,
}

/// `Roblox.GameInternationalization.Api.GetTranslationCountsForGameResponse`
///
/// A response model that contains game info for getting a translation count.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetTranslationCountsForGameResponse {
    /// The game id.
    #[serde(rename = "gameId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_id: Option<i64>,
    /// The list of languages/locales of the game.
    #[serde(rename = "languagesOrLocales")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages_or_locales: Option<Vec<TranslationCountLanguageOrLocaleResponse>>,
}

crate::api_enum! {
    str
    /// The status of the response for the requested game.
    TranslationCountGameInfoResponseStatus {
    LanguageOrLocaleSupportedForGame = 0 => "LanguageOrLocaleSupportedForGame",
    LanguageOrLocaleNotSupportedForGame = 1 => "LanguageOrLocaleNotSupportedForGame",
    LanguageOrLocaleIsSource = 2 => "LanguageOrLocaleIsSource",
    InsufficientPermission = 3 => "InsufficientPermission",
    GameDoesNotExist = 4 => "GameDoesNotExist",
    GameDoesNotHaveTable = 5 => "GameDoesNotHaveTable",
    UnknownError = 6 => "UnknownError",
    }
}

/// `Roblox.GameInternationalization.Api.TranslationCountGameInfoResponse`
///
/// A response model that contains game info for getting a translation count.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TranslationCountGameInfoResponse {
    /// Categories of translation counts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<TranslationCountCategoryInfoResponse>>,
    /// The game id.
    #[serde(rename = "gameId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_id: Option<i64>,
    /// The status of the response for the requested game.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TranslationCountGameInfoResponseStatus>,
}

/// `Roblox.GameInternationalization.Api.GetTranslationCountsForLanguageOrLocaleResponse`
///
/// A response model for getting translation counts for a language or locale.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetTranslationCountsForLanguageOrLocaleResponse {
    /// Info for the requested games, including their translation counts for the specified language or locale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub games: Option<Vec<TranslationCountGameInfoResponse>>,
    /// The code of the language or locale.
    #[serde(rename = "languageOrLocaleCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_or_locale_code: Option<String>,
    /// An indicator that says whether the passed in language/locale code represents a language or locale.
    #[serde(rename = "languageOrLocaleType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_or_locale_type: Option<TranslationCountLanguageOrLocaleResponseLanguageCodeType>,
}

/// `Roblox.GameInternationalization.Api.GetUiConfigurationsResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetUiConfigurationsResponse {
    /// Badge Icon translation management is enabled or not.
    #[serde(rename = "isBadgeIconEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_badge_icon_enabled: Option<bool>,
    /// Developer product translation management is enabled or not.
    #[serde(rename = "isDeveloperProductEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_developer_product_enabled: Option<bool>,
    /// Game pass translation management is enabled or not.
    #[serde(rename = "isGamePassEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_game_pass_enabled: Option<bool>,
    /// Game products translation management page is enabled or not.
    #[serde(rename = "isGameProductsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_game_products_enabled: Option<bool>,
}

/// `Roblox.GameInternationalization.Api.Language`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Language {
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "nativeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_name: Option<String>,
}

/// `Roblox.GameInternationalization.Api.LanguageOrLocale`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LanguageOrLocale {
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// An enum to distinguish between locale codes and language codes.
    #[serde(rename = "languageCodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code_type: Option<TranslationCountLanguageOrLocaleResponseLanguageCodeType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.GameInternationalization.Api.LanguageOrLocaleSettings`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LanguageOrLocaleSettings {
    /// Indicates whether or not automatic translation is currently enabled for the game and language.
    #[serde(rename = "isAutomaticTranslationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_automatic_translation_enabled: Option<bool>,
    /// Indicates whether image translation is currently enabled for the game and language.
    #[serde(rename = "isImageTranslationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_image_translation_enabled: Option<bool>,
    /// The language code.
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// The language code type.
    #[serde(rename = "languageCodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code_type: Option<TranslationCountLanguageOrLocaleResponseLanguageCodeType>,
}

crate::api_enum! {
    str
    /// \['LanguageFamily' = 0, 'SupportedLocale' = 1, 'SourceOrTranslation' = 2\]
    UserUniverseLocalizationSettingValueSettingType {
    LanguageFamily = 0 => "LanguageFamily",
    SupportedLocale = 1 => "SupportedLocale",
    SourceOrTranslation = 2 => "SourceOrTranslation",
    }
}

/// `Roblox.GameLocalization.Client.UserUniverseLocalizationSettingValue`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UserUniverseLocalizationSettingValue {
    #[serde(rename = "settingTargetCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting_target_code: Option<String>,
    #[serde(rename = "settingTargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting_target_id: Option<i64>,
    /// \['LanguageFamily' = 0, 'SupportedLocale' = 1, 'SourceOrTranslation' = 2\]
    #[serde(rename = "settingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting_type: Option<UserUniverseLocalizationSettingValueSettingType>,
}

/// `Roblox.GameInternationalization.Api.Models.Request.SetUserLocalizationSettingsRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SetUserLocalizationSettingsRequest {
    #[serde(rename = "settingValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting_value: Option<UserUniverseLocalizationSettingValue>,
}

/// `Roblox.GameInternationalization.Api.Models.Response.GameNameDescriptionMetadataResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GameNameDescriptionMetadataResponse {
    #[serde(rename = "isNameDescriptionMigrationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_name_description_migration_enabled: Option<bool>,
}

/// `Roblox.GameInternationalization.Api.Models.Response.GetPlayerChoiceUniverseSettingsResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetPlayerChoiceUniverseSettingsResponse {
    /// Checks whether player choice warning should be displayed to users.
    #[serde(rename = "IsPlayerChoiceEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_player_choice_enabled: Option<bool>,
}

/// `Roblox.GameInternationalization.Api.Models.Response.GetUserLocalizationSettingsForUniverseResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetUserLocalizationSettingsForUniverseResponse {
    #[serde(rename = "userUniverseLocalizationSettingValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_universe_localization_setting_value: Option<UserUniverseLocalizationSettingValue>,
}

/// `Roblox.GameInternationalization.Api.Models.Response.SetUserLocalizationSettingsResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SetUserLocalizationSettingsResponse {}

/// `Roblox.GameInternationalization.Api.Models.Response.UploadImageForGameThumbnailResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UploadImageForGameThumbnailResponse {
    #[serde(rename = "mediaAssetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_asset_id: Option<String>,
}

crate::api_enum! {
    str
    NameDescriptionUpdateType {
    Invalid = 0 => "Invalid",
    Name = 1 => "Name",
    Description = 2 => "Description",
    }
}

/// `Roblox.GameInternationalization.Api.NameDescription`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct NameDescription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "updateType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_type: Option<NameDescriptionUpdateType>,
}

/// `Roblox.GameInternationalization.Api.PatchLanguage`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PatchLanguage {
    /// An optional field used to indicate whether a language should be deleted from this game's list of supported languages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<bool>,
    /// The language code for the language to edit.
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// The language code type.
    #[serde(rename = "languageCodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code_type: Option<TranslationCountLanguageOrLocaleResponseLanguageCodeType>,
}

crate::api_enum! {
    str
    /// The report type
    RequestTranslationAnalyticsReportRequestReportType {
    GameTranslationStatus = 0 => "GameTranslationStatus",
    GameTranslationStatusForTranslator = 1 => "GameTranslationStatusForTranslator",
    GameTranslationStatusForTranslatorGroup = 2 => "GameTranslationStatusForTranslatorGroup",
    Test = 3 => "Test",
    }
}

/// `Roblox.GameInternationalization.Api.RequestTranslationAnalyticsReportRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RequestTranslationAnalyticsReportRequest {
    /// The exclusive end dateTime of report in UTC
    #[serde(rename = "endDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<String>,
    /// The report subject target id
    #[serde(rename = "reportSubjectTargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_subject_target_id: Option<i64>,
    /// The report type
    #[serde(rename = "reportType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_type: Option<RequestTranslationAnalyticsReportRequestReportType>,
    /// The inclusive start dateTime of report in UTC
    #[serde(rename = "startDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<String>,
}

crate::api_enum! {
    str
    /// \['InProgress' = 0, 'Ready' = 1, 'Unavailable' = 2\]
    RequestTranslationAnalyticsReportResponseReportGenerationStatus {
    InProgress = 0 => "inProgress",
    Ready = 1 => "ready",
    Unavailable = 2 => "unavailable",
    }
}

/// `Roblox.GameInternationalization.Api.RequestTranslationAnalyticsReportResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RequestTranslationAnalyticsReportResponse {
    /// \['InProgress' = 0, 'Ready' = 1, 'Unavailable' = 2\]
    #[serde(rename = "reportGenerationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_generation_status: Option<RequestTranslationAnalyticsReportResponseReportGenerationStatus>,
}

/// `Roblox.GameInternationalization.Api.SetAutolocalizationSettingsForGameRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SetAutolocalizationSettingsForGameRequest {
    #[serde(rename = "isAutolocalizationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_autolocalization_enabled: Option<bool>,
    #[serde(rename = "shouldUseLocalizationTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_use_localization_table: Option<bool>,
}

/// `Roblox.GameInternationalization.Api.SetAutolocalizationTableForGameRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SetAutolocalizationTableForGameRequest {
    #[serde(rename = "tableId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_id: Option<String>,
}

/// `Roblox.GameInternationalization.Api.SortImageIdsRequest`
///
/// A request model for sorting of image Ids for game thumbnails
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SortImageIdsRequest {
    /// List of thumbnail ids in desired order.
    #[serde(rename = "mediaAssetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_asset_ids: Option<Vec<i64>>,
}

/// `Roblox.Localization.Client.LanguageFamily`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LanguageFamily {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "isRightToLeft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_right_to_left: Option<bool>,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "nativeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_name: Option<String>,
}

crate::api_enum! {
    str
    SupportedLocaleLocale {
    EnUs = 0 => "en_us",
    EsEs = 1 => "es_es",
    FrFr = 2 => "fr_fr",
    IdId = 3 => "id_id",
    ItIt = 4 => "it_it",
    JaJp = 5 => "ja_jp",
    KoKr = 6 => "ko_kr",
    RuRu = 7 => "ru_ru",
    ThTh = 8 => "th_th",
    TrTr = 9 => "tr_tr",
    ViVn = 10 => "vi_vn",
    PtBr = 11 => "pt_br",
    DeDe = 12 => "de_de",
    ZhCn = 13 => "zh_cn",
    ZhTw = 14 => "zh_tw",
    BgBg = 15 => "bg_bg",
    BnBd = 16 => "bn_bd",
    CsCz = 17 => "cs_cz",
    DaDk = 18 => "da_dk",
    ElGr = 19 => "el_gr",
    EtEe = 20 => "et_ee",
    FiFi = 21 => "fi_fi",
    HiIn = 22 => "hi_in",
    HrHr = 23 => "hr_hr",
    HuHu = 24 => "hu_hu",
    KaGe = 25 => "ka_ge",
    KkKz = 26 => "kk_kz",
    KmKh = 27 => "km_kh",
    LtLt = 28 => "lt_lt",
    LvLv = 29 => "lv_lv",
    MsMy = 30 => "ms_my",
    MyMm = 31 => "my_mm",
    NbNo = 32 => "nb_no",
    NlNl = 33 => "nl_nl",
    FilPh = 34 => "fil_ph",
    PlPl = 35 => "pl_pl",
    RoRo = 36 => "ro_ro",
    UkUa = 37 => "uk_ua",
    SiLk = 38 => "si_lk",
    SkSk = 39 => "sk_sk",
    SlSl = 40 => "sl_sl",
    SqAl = 41 => "sq_al",
    BsBa = 42 => "bs_ba",
    SrRs = 43 => "sr_rs",
    SvSe = 44 => "sv_se",
    ZhCjv = 45 => "zh_cjv",
    Ar001 = 46 => "ar_001",
    EnGb = 47 => "en_gb",
    PtPt = 48 => "pt_pt",
    EsMx = 49 => "es_mx",
    FrCa = 50 => "fr_ca",
    }
}

/// `Roblox.Localization.Client.SupportedLocale`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SupportedLocale {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<LanguageFamily>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<SupportedLocaleLocale>,
    #[serde(rename = "localeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "nativeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_name: Option<String>,
}

/// `Roblox.GameInternationalization.Api.SourceLanguageWithLocales`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SourceLanguageWithLocales {
    #[serde(rename = "childLocales")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_locales: Option<Vec<SupportedLocale>>,
    #[serde(rename = "defaultLocale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_locale: Option<SupportedLocale>,
    #[serde(rename = "languageFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_family: Option<Language>,
}

/// `Roblox.GameInternationalization.Api.SupportedLanguagesMetadataResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SupportedLanguagesMetadataResponse {
    #[serde(rename = "areAllLanguagesEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub are_all_languages_enabled: Option<bool>,
    /// Is translation progress for automatic translations enabled on UI
    #[serde(rename = "isAutomaticTranslationProgressUIEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_automatic_translation_progress_ui_enabled: Option<bool>,
    #[serde(rename = "isFeatureEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_feature_enabled: Option<bool>,
    /// Is translation progress for human translations enabled on UI
    #[serde(rename = "isHumanTranslationProgressUIEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_human_translation_progress_ui_enabled: Option<bool>,
    /// Is Language Locales UI Enabled for Supported Languages
    #[serde(rename = "isSupportedLanguagesChildLocalesUIEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_supported_languages_child_locales_ui_enabled: Option<bool>,
    #[serde(rename = "minimumUniverseIdForFeature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_universe_id_for_feature: Option<i64>,
}

/// `Roblox.GameInternationalization.Api.TranslationAnalyticsMetadataResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TranslationAnalyticsMetadataResponse {
    /// Is Translation Analytics feature enabled on UI
    #[serde(rename = "isFeatureEnabledOnUI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_feature_enabled_on_ui: Option<bool>,
    /// Date-Time since the Analytics Reports can be downloaded
    #[serde(rename = "minimumDateTimeForAnalyticsReport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_date_time_for_analytics_report: Option<String>,
    /// Number of seconds to poll the server for report availability
    #[serde(rename = "reportRequestPollingIntervalSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_request_polling_interval_seconds: Option<i32>,
}

/// `Roblox.GameInternationalization.Api.UniverseDisplayInfoAutomaticTranslationSettings`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UniverseDisplayInfoAutomaticTranslationSettings {
    /// Indicates whether or not automatic translation is currently enabled for Game and Place Information for a game and language.
    #[serde(rename = "isUniverseDisplayInfoAutomaticTranslationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_universe_display_info_automatic_translation_enabled: Option<bool>,
    /// The language code.
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

/// `Roblox.GameInternationalization.Api.UpdateBadgeDescriptionRequest`
///
/// A request model for updating badge description
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateBadgeDescriptionRequest {
    /// Badge description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// `Roblox.GameInternationalization.Api.UpdateBadgeDescriptionResponse`
///
/// A response model for updating badge description
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateBadgeDescriptionResponse {
    /// Badge description saved
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// `Roblox.GameInternationalization.Api.UpdateBadgeNameDescriptionRequest`
///
/// A request model for updating badge name and description
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateBadgeNameDescriptionRequest {
    /// Badge description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Badge name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.GameInternationalization.Api.UpdateBadgeNameDescriptionResponse`
///
/// A response model for updating badge name and description
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateBadgeNameDescriptionResponse {
    /// Badge description saved
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Badge name saved
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.GameInternationalization.Api.UpdateBadgeNameRequest`
///
/// A request model for updating badge name
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateBadgeNameRequest {
    /// Badge name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.GameInternationalization.Api.UpdateBadgeNameResponse`
///
/// A response model for updating badge name
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateBadgeNameResponse {
    /// Badge name saved
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.GameInternationalization.Api.UpdateDeveloperProductDescriptionRequest`
///
/// A request model for updating developer product description
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateDeveloperProductDescriptionRequest {
    /// Developer product description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// `Roblox.GameInternationalization.Api.UpdateDeveloperProductDescriptionResponse`
///
/// A response model for updating developer product description
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateDeveloperProductDescriptionResponse {
    /// Developer product description saved
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// `Roblox.GameInternationalization.Api.UpdateDeveloperProductNameDescriptionRequest`
///
/// A request model for updating developer product name and description
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateDeveloperProductNameDescriptionRequest {
    /// Developer product description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Developer product name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.GameInternationalization.Api.UpdateDeveloperProductNameDescriptionResponse`
///
/// A response model for updating developer product name and description
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateDeveloperProductNameDescriptionResponse {
    /// Developer product description saved
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Developer product name saved
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.GameInternationalization.Api.UpdateDeveloperProductNameRequest`
///
/// A request model for updating developer product name
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateDeveloperProductNameRequest {
    /// Developer product name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.GameInternationalization.Api.UpdateDeveloperProductNameResponse`
///
/// A response model for updating developer product name
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateDeveloperProductNameResponse {
    /// Developer product name saved
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.GameInternationalization.Api.UpdateGamePassDescriptionRequest`
///
/// A request model for updating game pass description
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateGamePassDescriptionRequest {
    /// Game pass description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// `Roblox.GameInternationalization.Api.UpdateGamePassDescriptionResponse`
///
/// A response model for updating game pass description
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateGamePassDescriptionResponse {
    /// Game pass description saved
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// `Roblox.GameInternationalization.Api.UpdateGamePassNameDescriptionRequest`
///
/// A request model for updating game pass name and description
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateGamePassNameDescriptionRequest {
    /// Game pass description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Game pass name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.GameInternationalization.Api.UpdateGamePassNameDescriptionResponse`
///
/// A response model for updating game pass name and description
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateGamePassNameDescriptionResponse {
    /// Game pass description saved
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Game pass name saved
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.GameInternationalization.Api.UpdateGamePassNameRequest`
///
/// A request model for updating game pass name
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateGamePassNameRequest {
    /// Game pass name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.GameInternationalization.Api.UpdateGamePassNameResponse`
///
/// A response model for updating game pass name
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateGamePassNameResponse {
    /// Game pass name saved
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.GameInternationalization.Api.UpdateNameDescriptionsRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateNameDescriptionsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<NameDescription>>,
}

/// `Roblox.GameInternationalization.Api.UpdateNameDescriptionsResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateNameDescriptionsResponse {
    #[serde(rename = "failedOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_operations: Option<Vec<FailedNameDescription>>,
    #[serde(rename = "successOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_operations: Option<Vec<NameDescription>>,
}

/// `Roblox.GameInternationalization.Api.UpdateThumbnailAltTextRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateThumbnailAltTextRequest {
    #[serde(rename = "altText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_text: Option<String>,
    #[serde(rename = "thumbnailId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_id: Option<i64>,
}

/// `Roblox.GameInternationalization.Api.UpdateUniverseDisplayInfoAutomaticTranslationSettingsResponse`
///
/// A response model for getting the automatic translation allowed status for game information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateUniverseDisplayInfoAutomaticTranslationSettingsResponse {
    /// The game id.
    #[serde(rename = "gameId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_id: Option<i64>,
    /// Indicates universeDisplayInformation automatic translation is enabled.
    #[serde(rename = "isUniverseDisplayInfoAutomaticTranslationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_universe_display_info_automatic_translation_enabled: Option<bool>,
    /// The language code.
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

/// `Roblox.Web.WebAPI.ApiEmptyResponseModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiEmptyResponseModel {}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.GameInternationalization.Api.GetBadgeIconResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseGetBadgeIconResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GetBadgeIconResponse>>,
}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.GameInternationalization.Api.GetDeveloperProductIconResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseGetDeveloperProductIconResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GetDeveloperProductIconResponse>>,
}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.GameInternationalization.Api.GetGameIconResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseGetGameIconResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GetGameIconResponse>>,
}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.GameInternationalization.Api.GetGamePassIconResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseGetGamePassIconResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GetGamePassIconResponse>>,
}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.GameInternationalization.Api.GetGameThumbnailsResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseGetGameThumbnailsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GetGameThumbnailsResponse>>,
}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.GameInternationalization.Api.LanguageOrLocaleSettings\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseLanguageOrLocaleSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<LanguageOrLocaleSettings>>,
}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.GameInternationalization.Api.LanguageOrLocale\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseLanguageOrLocale {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<LanguageOrLocale>>,
}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.GameInternationalization.Api.NameDescription\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseNameDescription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<NameDescription>>,
}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.GameInternationalization.Api.UniverseDisplayInfoAutomaticTranslationSettings\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseUniverseDisplayInfoAutomaticTranslationSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<UniverseDisplayInfoAutomaticTranslationSettings>>,
}

/// Query, header and form inputs for [`get_automatic_translation_languages_by_language_code_target_languages`].
#[derive(Debug, Clone, Default)]
pub struct GetAutomaticTranslationLanguagesByLanguageCodeTargetLanguagesParams {
    /// Optional target languages. If not passed in, all allowed target languages for the source language will be returned.
    pub target_languages: Option<Vec<String>>,
    /// Optional gameId. If not passed in, we'll return the default list of languages allowed.
    pub game_id: Option<i64>,
}

/// Query, header and form inputs for [`get_badges_by_badge_id_icons`].
#[derive(Debug, Clone, Default)]
pub struct GetBadgesByBadgeIdIconsParams {
    /// The width of the icon to request
    pub width: Option<i32>,
    /// The height of the icon to request
    pub height: Option<i32>,
}

/// Query, header and form inputs for [`post_badges_by_badge_id_icons_language_codes_by_language_code`].
#[derive(Debug, Clone, Default)]
pub struct PostBadgesByBadgeIdIconsLanguageCodesByLanguageCodeParams {
    pub files: Option<FilePart>,
}

/// Query, header and form inputs for [`get_developer_products_by_developer_product_id_icons`].
#[derive(Debug, Clone, Default)]
pub struct GetDeveloperProductsByDeveloperProductIdIconsParams {
    /// The width of the icon to request
    pub width: Option<i32>,
    /// The height of the icon to request
    pub height: Option<i32>,
}

/// Query, header and form inputs for [`post_developer_products_by_developer_product_id_icons_language_codes_by_language_code`].
#[derive(Debug, Clone, Default)]
pub struct PostDeveloperProductsByDeveloperProductIdIconsLanguageCodesByLanguageCodeParams {
    pub files: Option<FilePart>,
}

/// Query, header and form inputs for [`get_game_icon_games_by_game_id`].
#[derive(Debug, Clone, Default)]
pub struct GetGameIconGamesByGameIdParams {
    /// The width of the icon to request
    pub width: Option<i32>,
    /// The height of the icon to request
    pub height: Option<i32>,
}

/// Query, header and form inputs for [`post_game_icon_games_by_game_id_language_codes_by_language_code`].
#[derive(Debug, Clone, Default)]
pub struct PostGameIconGamesByGameIdLanguageCodesByLanguageCodeParams {
    pub files: Option<FilePart>,
}

/// Query, header and form inputs for [`get_game_localization_status_translation_counts_for_language_or_locale`].
#[derive(Debug, Clone, Default)]
pub struct GetGameLocalizationStatusTranslationCountsForLanguageOrLocaleParams {
    /// List of game ids to retrieve translation counts for.
    pub game_ids: Vec<i64>,
    /// The code for the language or locale.
    pub language_or_locale_code: String,
    /// Indicates whether the languageOrLocaleCode represents a language or locale.
    pub language_or_locale_type: TranslationCountLanguageOrLocaleResponseLanguageCodeType,
}

/// Query, header and form inputs for [`get_game_passes_by_game_pass_id_icons`].
#[derive(Debug, Clone, Default)]
pub struct GetGamePassesByGamePassIdIconsParams {
    /// The width of the icon to request
    pub width: Option<i32>,
    /// The height of the icon to request
    pub height: Option<i32>,
}

/// Query, header and form inputs for [`post_game_passes_by_game_pass_id_icons_language_codes_by_language_code`].
#[derive(Debug, Clone, Default)]
pub struct PostGamePassesByGamePassIdIconsLanguageCodesByLanguageCodeParams {
    pub files: Option<FilePart>,
}

/// Query, header and form inputs for [`get_game_thumbnails_games_by_game_id_images`].
#[derive(Debug, Clone, Default)]
pub struct GetGameThumbnailsGamesByGameIdImagesParams {
    /// The width.
    pub width: Option<i32>,
    /// The height.
    pub height: Option<i32>,
}

/// Query, header and form inputs for [`post_game_thumbnails_games_by_game_id_language_codes_by_language_code_image`].
#[derive(Debug, Clone, Default)]
pub struct PostGameThumbnailsGamesByGameIdLanguageCodesByLanguageCodeImageParams {
    pub files: Option<FilePart>,
}

/// Query, header and form inputs for [`patch_source_language_games_by_game_id`].
#[derive(Debug, Clone, Default)]
pub struct PatchSourceLanguageGamesByGameIdParams {
    pub language_code: String,
}

/// Query, header and form inputs for [`get_translation_analytics_games_by_game_id_download_translation_analytics_report`].
#[derive(Debug, Clone, Default)]
pub struct GetTranslationAnalyticsGamesByGameIdDownloadTranslationAnalyticsReportParams {
    /// The inclusive start dateTime of report in UTC
    pub start_date_time: String,
    /// The exclusive end dateTime of report in UTC
    pub end_date_time: String,
    /// The report type
    pub report_type: RequestTranslationAnalyticsReportRequestReportType,
    /// The translator group id
    pub report_subject_target_id: i64,
}

/// `POST /v1/autolocalization/games/{gameId}/autolocalizationtable`
#[deprecated(note = "marked deprecated by Roblox")]
pub fn post_autolocalization_games_by_game_id_autolocalizationtable_request(game_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/autolocalization/games/{}/autolocalizationtable",
            crate::rt::path_value(&game_id)
        ),
    );
    Ok(r)
}

/// `POST /v1/autolocalization/games/{gameId}/autolocalizationtable`
#[deprecated(note = "marked deprecated by Roblox")]
pub async fn post_autolocalization_games_by_game_id_autolocalizationtable(
    client: &Client,
    game_id: i64,
) -> Result<GameAutolocalizationInformationResponse> {
    client
        .send(post_autolocalization_games_by_game_id_autolocalizationtable_request(game_id)?)
        .await?
        .json()
}

/// `PATCH /v1/autolocalization/games/{gameId}/autolocalizationtable`
#[deprecated(note = "marked deprecated by Roblox")]
pub fn patch_autolocalization_games_by_game_id_autolocalizationtable_request(
    game_id: i64,
    body: &SetAutolocalizationTableForGameRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!(
            "/v1/autolocalization/games/{}/autolocalizationtable",
            crate::rt::path_value(&game_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/autolocalization/games/{gameId}/autolocalizationtable`
#[deprecated(note = "marked deprecated by Roblox")]
pub async fn patch_autolocalization_games_by_game_id_autolocalizationtable(
    client: &Client,
    game_id: i64,
    body: &SetAutolocalizationTableForGameRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(patch_autolocalization_games_by_game_id_autolocalizationtable_request(game_id, body)?)
        .await?
        .json()
}

/// `PATCH /v1/autolocalization/games/{gameId}/settings`
///
/// Sets a game's auto-localization related settings
#[deprecated(note = "marked deprecated by Roblox")]
pub fn patch_autolocalization_games_by_game_id_settings_request(
    game_id: i64,
    body: &SetAutolocalizationSettingsForGameRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!(
            "/v1/autolocalization/games/{}/settings",
            crate::rt::path_value(&game_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/autolocalization/games/{gameId}/settings`
///
/// Sets a game's auto-localization related settings
#[deprecated(note = "marked deprecated by Roblox")]
pub async fn patch_autolocalization_games_by_game_id_settings(
    client: &Client,
    game_id: i64,
    body: &SetAutolocalizationSettingsForGameRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(patch_autolocalization_games_by_game_id_settings_request(
            game_id, body,
        )?)
        .await?
        .json()
}

/// `GET /v1/autolocalization/metadata`
///
/// Metadata for AutoLocalization Configuration
#[deprecated(note = "marked deprecated by Roblox")]
pub fn get_autolocalization_metadata_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/autolocalization/metadata".to_string());
    Ok(r)
}

/// `GET /v1/autolocalization/metadata`
///
/// Metadata for AutoLocalization Configuration
#[deprecated(note = "marked deprecated by Roblox")]
pub async fn get_autolocalization_metadata(client: &Client) -> Result<AutoLocalizationMetadataResponse> {
    client
        .send(get_autolocalization_metadata_request()?)
        .await?
        .json()
}

/// `GET /v1/automatic-translation/games/{gameId}/feature-status`
///
/// Checks if automatic translation can be enabled for a game. The user must still have proper permissions for the game to get this info.
pub fn get_automatic_translation_games_by_game_id_feature_status_request(game_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/automatic-translation/games/{}/feature-status",
            crate::rt::path_value(&game_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/automatic-translation/games/{gameId}/feature-status`
///
/// Checks if automatic translation can be enabled for a game. The user must still have proper permissions for the game to get this info.
pub async fn get_automatic_translation_games_by_game_id_feature_status(
    client: &Client,
    game_id: i64,
) -> Result<GetAutomaticTranslationFeatureStatusForGameResponse> {
    client
        .send(get_automatic_translation_games_by_game_id_feature_status_request(
            game_id,
        )?)
        .await?
        .json()
}

/// `GET /v1/automatic-translation/games/{gameId}/quota`
///
/// Returns the automatic translation quota info for a game. The user must still have proper permissions for the game to get this info.
pub fn get_automatic_translation_games_by_game_id_quota_request(game_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/automatic-translation/games/{}/quota",
            crate::rt::path_value(&game_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/automatic-translation/games/{gameId}/quota`
///
/// Returns the automatic translation quota info for a game. The user must still have proper permissions for the game to get this info.
pub async fn get_automatic_translation_games_by_game_id_quota(
    client: &Client,
    game_id: i64,
) -> Result<GetAutomaticTranslationQuotaForGameResponse> {
    client
        .send(get_automatic_translation_games_by_game_id_quota_request(game_id)?)
        .await?
        .json()
}

/// `GET /v1/automatic-translation/languages/{languageCode}/target-languages`
///
/// Checks if the requested target languages are allowed for automatic translation based on the source language. If there are no requested target languages, then all allowed target languages for the source language will be returned.
pub fn get_automatic_translation_languages_by_language_code_target_languages_request(
    language_code: &str,
    params: &GetAutomaticTranslationLanguagesByLanguageCodeTargetLanguagesParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/automatic-translation/languages/{}/target-languages",
            crate::rt::path_value(&language_code)
        ),
    );
    r.query("targetLanguages", &params.target_languages);
    r.query("gameId", &params.game_id);
    Ok(r)
}

/// `GET /v1/automatic-translation/languages/{languageCode}/target-languages`
///
/// Checks if the requested target languages are allowed for automatic translation based on the source language. If there are no requested target languages, then all allowed target languages for the source language will be returned.
pub async fn get_automatic_translation_languages_by_language_code_target_languages(
    client: &Client,
    language_code: &str,
    params: &GetAutomaticTranslationLanguagesByLanguageCodeTargetLanguagesParams,
) -> Result<GetAllowedAutomaticTranslationStatusForLanguagesResponse> {
    client
        .send(
            get_automatic_translation_languages_by_language_code_target_languages_request(
                language_code,
                params,
            )?,
        )
        .await?
        .json()
}

/// `PATCH /v1/badges/{badgeId}/description/language-codes/{languageCode}`
///
/// Update localized description of a badge
pub fn patch_badges_by_badge_id_description_language_codes_by_language_code_request(
    badge_id: i64,
    language_code: &str,
    body: &UpdateBadgeDescriptionRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!(
            "/v1/badges/{}/description/language-codes/{}",
            crate::rt::path_value(&badge_id),
            crate::rt::path_value(&language_code)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/badges/{badgeId}/description/language-codes/{languageCode}`
///
/// Update localized description of a badge
pub async fn patch_badges_by_badge_id_description_language_codes_by_language_code(
    client: &Client,
    badge_id: i64,
    language_code: &str,
    body: &UpdateBadgeDescriptionRequest,
) -> Result<UpdateBadgeDescriptionResponse> {
    client
        .send(
            patch_badges_by_badge_id_description_language_codes_by_language_code_request(
                badge_id,
                language_code,
                body,
            )?,
        )
        .await?
        .json()
}

/// `GET /v1/badges/{badgeId}/icons`
///
/// Get all icons for a badge
pub fn get_badges_by_badge_id_icons_request(
    badge_id: i64,
    params: &GetBadgesByBadgeIdIconsParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/badges/{}/icons", crate::rt::path_value(&badge_id)),
    );
    r.query("width", &params.width);
    r.query("height", &params.height);
    Ok(r)
}

/// `GET /v1/badges/{badgeId}/icons`
///
/// Get all icons for a badge
pub async fn get_badges_by_badge_id_icons(
    client: &Client,
    badge_id: i64,
    params: &GetBadgesByBadgeIdIconsParams,
) -> Result<ApiArrayResponseGetBadgeIconResponse> {
    client
        .send(get_badges_by_badge_id_icons_request(badge_id, params)?)
        .await?
        .json()
}

/// `POST /v1/badges/{badgeId}/icons/language-codes/{languageCode}`
///
/// Update a badge's icon
pub fn post_badges_by_badge_id_icons_language_codes_by_language_code_request(
    badge_id: i64,
    language_code: &str,
    params: &PostBadgesByBadgeIdIconsLanguageCodesByLanguageCodeParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/badges/{}/icons/language-codes/{}",
            crate::rt::path_value(&badge_id),
            crate::rt::path_value(&language_code)
        ),
    );
    r.form_file("Files", &params.files);
    Ok(r)
}

/// `POST /v1/badges/{badgeId}/icons/language-codes/{languageCode}`
///
/// Update a badge's icon
pub async fn post_badges_by_badge_id_icons_language_codes_by_language_code(
    client: &Client,
    badge_id: i64,
    language_code: &str,
    params: &PostBadgesByBadgeIdIconsLanguageCodesByLanguageCodeParams,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(
            post_badges_by_badge_id_icons_language_codes_by_language_code_request(
                badge_id,
                language_code,
                params,
            )?,
        )
        .await?
        .json()
}

/// `DELETE /v1/badges/{badgeId}/icons/language-codes/{languageCode}`
///
/// Delete a localized icon from a badge
pub fn delete_badges_by_badge_id_icons_language_codes_by_language_code_request(
    badge_id: i64,
    language_code: &str,
) -> Result<Request> {
    let r = Request::new(
        Method::DELETE,
        HOST,
        format!(
            "/v1/badges/{}/icons/language-codes/{}",
            crate::rt::path_value(&badge_id),
            crate::rt::path_value(&language_code)
        ),
    );
    Ok(r)
}

/// `DELETE /v1/badges/{badgeId}/icons/language-codes/{languageCode}`
///
/// Delete a localized icon from a badge
pub async fn delete_badges_by_badge_id_icons_language_codes_by_language_code(
    client: &Client,
    badge_id: i64,
    language_code: &str,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(
            delete_badges_by_badge_id_icons_language_codes_by_language_code_request(badge_id, language_code)?,
        )
        .await?
        .json()
}

/// `GET /v1/badges/{badgeId}/name-description`
pub fn get_badges_by_badge_id_name_description_request(badge_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/badges/{}/name-description", crate::rt::path_value(&badge_id)),
    );
    Ok(r)
}

/// `GET /v1/badges/{badgeId}/name-description`
pub async fn get_badges_by_badge_id_name_description(
    client: &Client,
    badge_id: i64,
) -> Result<ApiArrayResponseNameDescription> {
    client
        .send(get_badges_by_badge_id_name_description_request(badge_id)?)
        .await?
        .json()
}

/// `PATCH /v1/badges/{badgeId}/name-description/language-codes/{languageCode}`
///
/// Update localized name and description of a badge
pub fn patch_badges_by_badge_id_name_description_language_codes_by_language_code_request(
    badge_id: i64,
    language_code: &str,
    body: &UpdateBadgeNameDescriptionRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!(
            "/v1/badges/{}/name-description/language-codes/{}",
            crate::rt::path_value(&badge_id),
            crate::rt::path_value(&language_code)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/badges/{badgeId}/name-description/language-codes/{languageCode}`
///
/// Update localized name and description of a badge
pub async fn patch_badges_by_badge_id_name_description_language_codes_by_language_code(
    client: &Client,
    badge_id: i64,
    language_code: &str,
    body: &UpdateBadgeNameDescriptionRequest,
) -> Result<UpdateBadgeNameDescriptionResponse> {
    client
        .send(
            patch_badges_by_badge_id_name_description_language_codes_by_language_code_request(
                badge_id,
                language_code,
                body,
            )?,
        )
        .await?
        .json()
}

/// `DELETE /v1/badges/{badgeId}/name-description/language-codes/{languageCode}`
///
/// Delete localized name and description of a badge
pub fn delete_badges_by_badge_id_name_description_language_codes_by_language_code_request(
    badge_id: i64,
    language_code: &str,
) -> Result<Request> {
    let r = Request::new(
        Method::DELETE,
        HOST,
        format!(
            "/v1/badges/{}/name-description/language-codes/{}",
            crate::rt::path_value(&badge_id),
            crate::rt::path_value(&language_code)
        ),
    );
    Ok(r)
}

/// `DELETE /v1/badges/{badgeId}/name-description/language-codes/{languageCode}`
///
/// Delete localized name and description of a badge
pub async fn delete_badges_by_badge_id_name_description_language_codes_by_language_code(
    client: &Client,
    badge_id: i64,
    language_code: &str,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(
            delete_badges_by_badge_id_name_description_language_codes_by_language_code_request(
                badge_id,
                language_code,
            )?,
        )
        .await?
        .json()
}

/// `PATCH /v1/badges/{badgeId}/name/language-codes/{languageCode}`
///
/// Update localized name of a badge
pub fn patch_badges_by_badge_id_name_language_codes_by_language_code_request(
    badge_id: i64,
    language_code: &str,
    body: &UpdateBadgeNameRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!(
            "/v1/badges/{}/name/language-codes/{}",
            crate::rt::path_value(&badge_id),
            crate::rt::path_value(&language_code)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/badges/{badgeId}/name/language-codes/{languageCode}`
///
/// Update localized name of a badge
pub async fn patch_badges_by_badge_id_name_language_codes_by_language_code(
    client: &Client,
    badge_id: i64,
    language_code: &str,
    body: &UpdateBadgeNameRequest,
) -> Result<UpdateBadgeNameResponse> {
    client
        .send(
            patch_badges_by_badge_id_name_language_codes_by_language_code_request(
                badge_id,
                language_code,
                body,
            )?,
        )
        .await?
        .json()
}

/// `PATCH /v1/developer-products/{developerProductId}/description/language-codes/{languageCode}`
///
/// Update localized description of a developer product
pub fn patch_developer_products_by_developer_product_id_description_language_codes_by_language_code_request(
    developer_product_id: i64,
    language_code: &str,
    body: &UpdateDeveloperProductDescriptionRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!(
            "/v1/developer-products/{}/description/language-codes/{}",
            crate::rt::path_value(&developer_product_id),
            crate::rt::path_value(&language_code)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/developer-products/{developerProductId}/description/language-codes/{languageCode}`
///
/// Update localized description of a developer product
pub async fn patch_developer_products_by_developer_product_id_description_language_codes_by_language_code(
    client: &Client,
    developer_product_id: i64,
    language_code: &str,
    body: &UpdateDeveloperProductDescriptionRequest,
) -> Result<UpdateDeveloperProductDescriptionResponse> {
    client.send(patch_developer_products_by_developer_product_id_description_language_codes_by_language_code_request(developer_product_id, language_code, body)?).await?.json()
}

/// `GET /v1/developer-products/{developerProductId}/icons`
///
/// Get all icons for a developer product
pub fn get_developer_products_by_developer_product_id_icons_request(
    developer_product_id: i64,
    params: &GetDeveloperProductsByDeveloperProductIdIconsParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/developer-products/{}/icons",
            crate::rt::path_value(&developer_product_id)
        ),
    );
    r.query("width", &params.width);
    r.query("height", &params.height);
    Ok(r)
}

/// `GET /v1/developer-products/{developerProductId}/icons`
///
/// Get all icons for a developer product
pub async fn get_developer_products_by_developer_product_id_icons(
    client: &Client,
    developer_product_id: i64,
    params: &GetDeveloperProductsByDeveloperProductIdIconsParams,
) -> Result<ApiArrayResponseGetDeveloperProductIconResponse> {
    client
        .send(get_developer_products_by_developer_product_id_icons_request(
            developer_product_id,
            params,
        )?)
        .await?
        .json()
}

/// `POST /v1/developer-products/{developerProductId}/icons/language-codes/{languageCode}`
///
/// Update a developer product's icon
pub fn post_developer_products_by_developer_product_id_icons_language_codes_by_language_code_request(
    developer_product_id: i64,
    language_code: &str,
    params: &PostDeveloperProductsByDeveloperProductIdIconsLanguageCodesByLanguageCodeParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/developer-products/{}/icons/language-codes/{}",
            crate::rt::path_value(&developer_product_id),
            crate::rt::path_value(&language_code)
        ),
    );
    r.form_file("Files", &params.files);
    Ok(r)
}

/// `POST /v1/developer-products/{developerProductId}/icons/language-codes/{languageCode}`
///
/// Update a developer product's icon
pub async fn post_developer_products_by_developer_product_id_icons_language_codes_by_language_code(
    client: &Client,
    developer_product_id: i64,
    language_code: &str,
    params: &PostDeveloperProductsByDeveloperProductIdIconsLanguageCodesByLanguageCodeParams,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(
            post_developer_products_by_developer_product_id_icons_language_codes_by_language_code_request(
                developer_product_id,
                language_code,
                params,
            )?,
        )
        .await?
        .json()
}

/// `DELETE /v1/developer-products/{developerProductId}/icons/language-codes/{languageCode}`
///
/// Delete a localized icon from a developer product
pub fn delete_developer_products_by_developer_product_id_icons_language_codes_by_language_code_request(
    developer_product_id: i64,
    language_code: &str,
) -> Result<Request> {
    let r = Request::new(
        Method::DELETE,
        HOST,
        format!(
            "/v1/developer-products/{}/icons/language-codes/{}",
            crate::rt::path_value(&developer_product_id),
            crate::rt::path_value(&language_code)
        ),
    );
    Ok(r)
}

/// `DELETE /v1/developer-products/{developerProductId}/icons/language-codes/{languageCode}`
///
/// Delete a localized icon from a developer product
pub async fn delete_developer_products_by_developer_product_id_icons_language_codes_by_language_code(
    client: &Client,
    developer_product_id: i64,
    language_code: &str,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(
            delete_developer_products_by_developer_product_id_icons_language_codes_by_language_code_request(
                developer_product_id,
                language_code,
            )?,
        )
        .await?
        .json()
}

/// `GET /v1/developer-products/{developerProductId}/name-description`
///
/// Get all names and descriptions of a developer product
pub fn get_developer_products_by_developer_product_id_name_description_request(
    developer_product_id: i64,
) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/developer-products/{}/name-description",
            crate::rt::path_value(&developer_product_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/developer-products/{developerProductId}/name-description`
///
/// Get all names and descriptions of a developer product
pub async fn get_developer_products_by_developer_product_id_name_description(
    client: &Client,
    developer_product_id: i64,
) -> Result<ApiArrayResponseNameDescription> {
    client
        .send(get_developer_products_by_developer_product_id_name_description_request(developer_product_id)?)
        .await?
        .json()
}

/// `PATCH /v1/developer-products/{developerProductId}/name-description/language-codes/{languageCode}`
///
/// Update localized name and description of a developer product
pub fn patch_developer_products_by_developer_product_id_name_description_language_codes_by_language_code_request(
    developer_product_id: i64,
    language_code: &str,
    body: &UpdateDeveloperProductNameDescriptionRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!(
            "/v1/developer-products/{}/name-description/language-codes/{}",
            crate::rt::path_value(&developer_product_id),
            crate::rt::path_value(&language_code)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/developer-products/{developerProductId}/name-description/language-codes/{languageCode}`
///
/// Update localized name and description of a developer product
pub async fn patch_developer_products_by_developer_product_id_name_description_language_codes_by_language_code(
    client: &Client,
    developer_product_id: i64,
    language_code: &str,
    body: &UpdateDeveloperProductNameDescriptionRequest,
) -> Result<UpdateDeveloperProductNameDescriptionResponse> {
    client.send(patch_developer_products_by_developer_product_id_name_description_language_codes_by_language_code_request(developer_product_id, language_code, body)?).await?.json()
}

/// `DELETE /v1/developer-products/{developerProductId}/name-description/language-codes/{languageCode}`
///
/// Delete localized name and description of a developer product
pub fn delete_developer_products_by_developer_product_id_name_description_language_codes_by_language_code_request(
    developer_product_id: i64,
    language_code: &str,
) -> Result<Request> {
    let r = Request::new(
        Method::DELETE,
        HOST,
        format!(
            "/v1/developer-products/{}/name-description/language-codes/{}",
            crate::rt::path_value(&developer_product_id),
            crate::rt::path_value(&language_code)
        ),
    );
    Ok(r)
}

/// `DELETE /v1/developer-products/{developerProductId}/name-description/language-codes/{languageCode}`
///
/// Delete localized name and description of a developer product
pub async fn delete_developer_products_by_developer_product_id_name_description_language_codes_by_language_code(
    client: &Client,
    developer_product_id: i64,
    language_code: &str,
) -> Result<ApiEmptyResponseModel> {
    client.send(delete_developer_products_by_developer_product_id_name_description_language_codes_by_language_code_request(developer_product_id, language_code)?).await?.json()
}

/// `PATCH /v1/developer-products/{developerProductId}/name/language-codes/{languageCode}`
///
/// Update localized name of a developer product
pub fn patch_developer_products_by_developer_product_id_name_language_codes_by_language_code_request(
    developer_product_id: i64,
    language_code: &str,
    body: &UpdateDeveloperProductNameRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!(
            "/v1/developer-products/{}/name/language-codes/{}",
            crate::rt::path_value(&developer_product_id),
            crate::rt::path_value(&language_code)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/developer-products/{developerProductId}/name/language-codes/{languageCode}`
///
/// Update localized name of a developer product
pub async fn patch_developer_products_by_developer_product_id_name_language_codes_by_language_code(
    client: &Client,
    developer_product_id: i64,
    language_code: &str,
    body: &UpdateDeveloperProductNameRequest,
) -> Result<UpdateDeveloperProductNameResponse> {
    client
        .send(
            patch_developer_products_by_developer_product_id_name_language_codes_by_language_code_request(
                developer_product_id,
                language_code,
                body,
            )?,
        )
        .await?
        .json()
}

/// `GET /v1/game-icon/games/{gameId}`
///
/// Get all icons for a game
pub fn get_game_icon_games_by_game_id_request(
    game_id: i64,
    params: &GetGameIconGamesByGameIdParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/game-icon/games/{}", crate::rt::path_value(&game_id)),
    );
    r.query("width", &params.width);
    r.query("height", &params.height);
    Ok(r)
}

/// `GET /v1/game-icon/games/{gameId}`
///
/// Get all icons for a game
pub async fn get_game_icon_games_by_game_id(
    client: &Client,
    game_id: i64,
    params: &GetGameIconGamesByGameIdParams,
) -> Result<ApiArrayResponseGetGameIconResponse> {
    client
        .send(get_game_icon_games_by_game_id_request(game_id, params)?)
        .await?
        .json()
}

/// `POST /v1/game-icon/games/{gameId}/language-codes/{languageCode}`
///
/// Update a game's icon
pub fn post_game_icon_games_by_game_id_language_codes_by_language_code_request(
    game_id: i64,
    language_code: &str,
    params: &PostGameIconGamesByGameIdLanguageCodesByLanguageCodeParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/game-icon/games/{}/language-codes/{}",
            crate::rt::path_value(&game_id),
            crate::rt::path_value(&language_code)
        ),
    );
    r.form_file("Files", &params.files);
    Ok(r)
}

/// `POST /v1/game-icon/games/{gameId}/language-codes/{languageCode}`
///
/// Update a game's icon
pub async fn post_game_icon_games_by_game_id_language_codes_by_language_code(
    client: &Client,
    game_id: i64,
    language_code: &str,
    params: &PostGameIconGamesByGameIdLanguageCodesByLanguageCodeParams,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(
            post_game_icon_games_by_game_id_language_codes_by_language_code_request(
                game_id,
                language_code,
                params,
            )?,
        )
        .await?
        .json()
}

/// `DELETE /v1/game-icon/games/{gameId}/language-codes/{languageCode}`
///
/// Delete a localized icon from a game
pub fn delete_game_icon_games_by_game_id_language_codes_by_language_code_request(
    game_id: i64,
    language_code: &str,
) -> Result<Request> {
    let r = Request::new(
        Method::DELETE,
        HOST,
        format!(
            "/v1/game-icon/games/{}/language-codes/{}",
            crate::rt::path_value(&game_id),
            crate::rt::path_value(&language_code)
        ),
    );
    Ok(r)
}

/// `DELETE /v1/game-icon/games/{gameId}/language-codes/{languageCode}`
///
/// Delete a localized icon from a game
pub async fn delete_game_icon_games_by_game_id_language_codes_by_language_code(
    client: &Client,
    game_id: i64,
    language_code: &str,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(
            delete_game_icon_games_by_game_id_language_codes_by_language_code_request(
                game_id,
                language_code,
            )?,
        )
        .await?
        .json()
}

/// `GET /v1/game-localization-status/translation-counts-for-language-or-locale`
///
/// Gets the language translation counts for the specified table. The languages to retrieve must be provided.
pub fn get_game_localization_status_translation_counts_for_language_or_locale_request(
    params: &GetGameLocalizationStatusTranslationCountsForLanguageOrLocaleParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        "/v1/game-localization-status/translation-counts-for-language-or-locale".to_string(),
    );
    r.query("gameIds", &params.game_ids);
    r.query("languageOrLocaleCode", &params.language_or_locale_code);
    r.query("languageOrLocaleType", &params.language_or_locale_type);
    Ok(r)
}

/// `GET /v1/game-localization-status/translation-counts-for-language-or-locale`
///
/// Gets the language translation counts for the specified table. The languages to retrieve must be provided.
pub async fn get_game_localization_status_translation_counts_for_language_or_locale(
    client: &Client,
    params: &GetGameLocalizationStatusTranslationCountsForLanguageOrLocaleParams,
) -> Result<GetTranslationCountsForLanguageOrLocaleResponse> {
    client
        .send(get_game_localization_status_translation_counts_for_language_or_locale_request(params)?)
        .await?
        .json()
}

/// `GET /v1/game-localization-status/{gameId}/translation-counts`
///
/// Gets the language translation counts for all languages of a game
pub fn get_game_localization_status_by_game_id_translation_counts_request(game_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/game-localization-status/{}/translation-counts",
            crate::rt::path_value(&game_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/game-localization-status/{gameId}/translation-counts`
///
/// Gets the language translation counts for all languages of a game
pub async fn get_game_localization_status_by_game_id_translation_counts(
    client: &Client,
    game_id: i64,
) -> Result<GetTranslationCountsForGameResponse> {
    client
        .send(get_game_localization_status_by_game_id_translation_counts_request(game_id)?)
        .await?
        .json()
}

/// `PATCH /v1/game-passes/{gamePassId}/description/language-codes/{languageCode}`
///
/// Update localized description of a game pass
pub fn patch_game_passes_by_game_pass_id_description_language_codes_by_language_code_request(
    game_pass_id: i64,
    language_code: &str,
    body: &UpdateGamePassDescriptionRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!(
            "/v1/game-passes/{}/description/language-codes/{}",
            crate::rt::path_value(&game_pass_id),
            crate::rt::path_value(&language_code)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/game-passes/{gamePassId}/description/language-codes/{languageCode}`
///
/// Update localized description of a game pass
pub async fn patch_game_passes_by_game_pass_id_description_language_codes_by_language_code(
    client: &Client,
    game_pass_id: i64,
    language_code: &str,
    body: &UpdateGamePassDescriptionRequest,
) -> Result<UpdateGamePassDescriptionResponse> {
    client
        .send(
            patch_game_passes_by_game_pass_id_description_language_codes_by_language_code_request(
                game_pass_id,
                language_code,
                body,
            )?,
        )
        .await?
        .json()
}

/// `GET /v1/game-passes/{gamePassId}/icons`
///
/// Get all icons for a game pass
pub fn get_game_passes_by_game_pass_id_icons_request(
    game_pass_id: i64,
    params: &GetGamePassesByGamePassIdIconsParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/game-passes/{}/icons", crate::rt::path_value(&game_pass_id)),
    );
    r.query("width", &params.width);
    r.query("height", &params.height);
    Ok(r)
}

/// `GET /v1/game-passes/{gamePassId}/icons`
///
/// Get all icons for a game pass
pub async fn get_game_passes_by_game_pass_id_icons(
    client: &Client,
    game_pass_id: i64,
    params: &GetGamePassesByGamePassIdIconsParams,
) -> Result<ApiArrayResponseGetGamePassIconResponse> {
    client
        .send(get_game_passes_by_game_pass_id_icons_request(
            game_pass_id,
            params,
        )?)
        .await?
        .json()
}

/// `POST /v1/game-passes/{gamePassId}/icons/language-codes/{languageCode}`
///
/// Update a game pass's icon
pub fn post_game_passes_by_game_pass_id_icons_language_codes_by_language_code_request(
    game_pass_id: i64,
    language_code: &str,
    params: &PostGamePassesByGamePassIdIconsLanguageCodesByLanguageCodeParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/game-passes/{}/icons/language-codes/{}",
            crate::rt::path_value(&game_pass_id),
            crate::rt::path_value(&language_code)
        ),
    );
    r.form_file("Files", &params.files);
    Ok(r)
}

/// `POST /v1/game-passes/{gamePassId}/icons/language-codes/{languageCode}`
///
/// Update a game pass's icon
pub async fn post_game_passes_by_game_pass_id_icons_language_codes_by_language_code(
    client: &Client,
    game_pass_id: i64,
    language_code: &str,
    params: &PostGamePassesByGamePassIdIconsLanguageCodesByLanguageCodeParams,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(
            post_game_passes_by_game_pass_id_icons_language_codes_by_language_code_request(
                game_pass_id,
                language_code,
                params,
            )?,
        )
        .await?
        .json()
}

/// `DELETE /v1/game-passes/{gamePassId}/icons/language-codes/{languageCode}`
///
/// Delete a localized icon from a game pass
pub fn delete_game_passes_by_game_pass_id_icons_language_codes_by_language_code_request(
    game_pass_id: i64,
    language_code: &str,
) -> Result<Request> {
    let r = Request::new(
        Method::DELETE,
        HOST,
        format!(
            "/v1/game-passes/{}/icons/language-codes/{}",
            crate::rt::path_value(&game_pass_id),
            crate::rt::path_value(&language_code)
        ),
    );
    Ok(r)
}

/// `DELETE /v1/game-passes/{gamePassId}/icons/language-codes/{languageCode}`
///
/// Delete a localized icon from a game pass
pub async fn delete_game_passes_by_game_pass_id_icons_language_codes_by_language_code(
    client: &Client,
    game_pass_id: i64,
    language_code: &str,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(
            delete_game_passes_by_game_pass_id_icons_language_codes_by_language_code_request(
                game_pass_id,
                language_code,
            )?,
        )
        .await?
        .json()
}

/// `GET /v1/game-passes/{gamePassId}/name-description`
///
/// Get all names and descriptions of a game pass
pub fn get_game_passes_by_game_pass_id_name_description_request(game_pass_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/game-passes/{}/name-description",
            crate::rt::path_value(&game_pass_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/game-passes/{gamePassId}/name-description`
///
/// Get all names and descriptions of a game pass
pub async fn get_game_passes_by_game_pass_id_name_description(
    client: &Client,
    game_pass_id: i64,
) -> Result<ApiArrayResponseNameDescription> {
    client
        .send(get_game_passes_by_game_pass_id_name_description_request(
            game_pass_id,
        )?)
        .await?
        .json()
}

/// `PATCH /v1/game-passes/{gamePassId}/name-description/language-codes/{languageCode}`
///
/// Update localized name and description of a game pass
pub fn patch_game_passes_by_game_pass_id_name_description_language_codes_by_language_code_request(
    game_pass_id: i64,
    language_code: &str,
    body: &UpdateGamePassNameDescriptionRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!(
            "/v1/game-passes/{}/name-description/language-codes/{}",
            crate::rt::path_value(&game_pass_id),
            crate::rt::path_value(&language_code)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/game-passes/{gamePassId}/name-description/language-codes/{languageCode}`
///
/// Update localized name and description of a game pass
pub async fn patch_game_passes_by_game_pass_id_name_description_language_codes_by_language_code(
    client: &Client,
    game_pass_id: i64,
    language_code: &str,
    body: &UpdateGamePassNameDescriptionRequest,
) -> Result<UpdateGamePassNameDescriptionResponse> {
    client
        .send(
            patch_game_passes_by_game_pass_id_name_description_language_codes_by_language_code_request(
                game_pass_id,
                language_code,
                body,
            )?,
        )
        .await?
        .json()
}

/// `DELETE /v1/game-passes/{gamePassId}/name-description/language-codes/{languageCode}`
///
/// Delete localized name and description of a game pass
pub fn delete_game_passes_by_game_pass_id_name_description_language_codes_by_language_code_request(
    game_pass_id: i64,
    language_code: &str,
) -> Result<Request> {
    let r = Request::new(
        Method::DELETE,
        HOST,
        format!(
            "/v1/game-passes/{}/name-description/language-codes/{}",
            crate::rt::path_value(&game_pass_id),
            crate::rt::path_value(&language_code)
        ),
    );
    Ok(r)
}

/// `DELETE /v1/game-passes/{gamePassId}/name-description/language-codes/{languageCode}`
///
/// Delete localized name and description of a game pass
pub async fn delete_game_passes_by_game_pass_id_name_description_language_codes_by_language_code(
    client: &Client,
    game_pass_id: i64,
    language_code: &str,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(
            delete_game_passes_by_game_pass_id_name_description_language_codes_by_language_code_request(
                game_pass_id,
                language_code,
            )?,
        )
        .await?
        .json()
}

/// `PATCH /v1/game-passes/{gamePassId}/name/language-codes/{languageCode}`
///
/// Update localized name of a game pass
pub fn patch_game_passes_by_game_pass_id_name_language_codes_by_language_code_request(
    game_pass_id: i64,
    language_code: &str,
    body: &UpdateGamePassNameRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!(
            "/v1/game-passes/{}/name/language-codes/{}",
            crate::rt::path_value(&game_pass_id),
            crate::rt::path_value(&language_code)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/game-passes/{gamePassId}/name/language-codes/{languageCode}`
///
/// Update localized name of a game pass
pub async fn patch_game_passes_by_game_pass_id_name_language_codes_by_language_code(
    client: &Client,
    game_pass_id: i64,
    language_code: &str,
    body: &UpdateGamePassNameRequest,
) -> Result<UpdateGamePassNameResponse> {
    client
        .send(
            patch_game_passes_by_game_pass_id_name_language_codes_by_language_code_request(
                game_pass_id,
                language_code,
                body,
            )?,
        )
        .await?
        .json()
}

/// `GET /v1/game-thumbnails/games/{gameId}/images`
///
/// Get the localized image ids in all languages for a game.
pub fn get_game_thumbnails_games_by_game_id_images_request(
    game_id: i64,
    params: &GetGameThumbnailsGamesByGameIdImagesParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/game-thumbnails/games/{}/images",
            crate::rt::path_value(&game_id)
        ),
    );
    r.query("width", &params.width);
    r.query("height", &params.height);
    Ok(r)
}

/// `GET /v1/game-thumbnails/games/{gameId}/images`
///
/// Get the localized image ids in all languages for a game.
pub async fn get_game_thumbnails_games_by_game_id_images(
    client: &Client,
    game_id: i64,
    params: &GetGameThumbnailsGamesByGameIdImagesParams,
) -> Result<ApiArrayResponseGetGameThumbnailsResponse> {
    client
        .send(get_game_thumbnails_games_by_game_id_images_request(
            game_id, params,
        )?)
        .await?
        .json()
}

/// `POST /v1/game-thumbnails/games/{gameId}/language-codes/{languageCode}/alt-text`
///
/// Updates the game thumbnail alt text.
pub fn post_game_thumbnails_games_by_game_id_language_codes_by_language_code_alt_text_request(
    game_id: i64,
    language_code: &str,
    body: &UpdateThumbnailAltTextRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/game-thumbnails/games/{}/language-codes/{}/alt-text",
            crate::rt::path_value(&game_id),
            crate::rt::path_value(&language_code)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/game-thumbnails/games/{gameId}/language-codes/{languageCode}/alt-text`
///
/// Updates the game thumbnail alt text.
pub async fn post_game_thumbnails_games_by_game_id_language_codes_by_language_code_alt_text(
    client: &Client,
    game_id: i64,
    language_code: &str,
    body: &UpdateThumbnailAltTextRequest,
) -> Result<String> {
    client
        .send(
            post_game_thumbnails_games_by_game_id_language_codes_by_language_code_alt_text_request(
                game_id,
                language_code,
                body,
            )?,
        )
        .await?
        .json()
}

/// `POST /v1/game-thumbnails/games/{gameId}/language-codes/{languageCode}/image`
///
/// Uploads the game thumbnail.
pub fn post_game_thumbnails_games_by_game_id_language_codes_by_language_code_image_request(
    game_id: i64,
    language_code: &str,
    params: &PostGameThumbnailsGamesByGameIdLanguageCodesByLanguageCodeImageParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/game-thumbnails/games/{}/language-codes/{}/image",
            crate::rt::path_value(&game_id),
            crate::rt::path_value(&language_code)
        ),
    );
    r.form_file("Files", &params.files);
    Ok(r)
}

/// `POST /v1/game-thumbnails/games/{gameId}/language-codes/{languageCode}/image`
///
/// Uploads the game thumbnail.
pub async fn post_game_thumbnails_games_by_game_id_language_codes_by_language_code_image(
    client: &Client,
    game_id: i64,
    language_code: &str,
    params: &PostGameThumbnailsGamesByGameIdLanguageCodesByLanguageCodeImageParams,
) -> Result<UploadImageForGameThumbnailResponse> {
    client
        .send(
            post_game_thumbnails_games_by_game_id_language_codes_by_language_code_image_request(
                game_id,
                language_code,
                params,
            )?,
        )
        .await?
        .json()
}

/// `POST /v1/game-thumbnails/games/{gameId}/language-codes/{languageCode}/images/order`
///
/// Orders the specified image Ids for the game thumbnails.
pub fn post_game_thumbnails_games_by_game_id_language_codes_by_language_code_images_order_request(
    game_id: i64,
    language_code: &str,
    body: &SortImageIdsRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/game-thumbnails/games/{}/language-codes/{}/images/order",
            crate::rt::path_value(&game_id),
            crate::rt::path_value(&language_code)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/game-thumbnails/games/{gameId}/language-codes/{languageCode}/images/order`
///
/// Orders the specified image Ids for the game thumbnails.
pub async fn post_game_thumbnails_games_by_game_id_language_codes_by_language_code_images_order(
    client: &Client,
    game_id: i64,
    language_code: &str,
    body: &SortImageIdsRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(
            post_game_thumbnails_games_by_game_id_language_codes_by_language_code_images_order_request(
                game_id,
                language_code,
                body,
            )?,
        )
        .await?
        .json()
}

/// `DELETE /v1/game-thumbnails/games/{gameId}/language-codes/{languageCode}/images/{imageId}`
///
/// Deletes the game thumbnail.
pub fn delete_game_thumbnails_games_by_game_id_language_codes_by_language_code_images_by_image_id_request(
    game_id: i64,
    language_code: &str,
    image_id: i64,
) -> Result<Request> {
    let r = Request::new(
        Method::DELETE,
        HOST,
        format!(
            "/v1/game-thumbnails/games/{}/language-codes/{}/images/{}",
            crate::rt::path_value(&game_id),
            crate::rt::path_value(&language_code),
            crate::rt::path_value(&image_id)
        ),
    );
    Ok(r)
}

/// `DELETE /v1/game-thumbnails/games/{gameId}/language-codes/{languageCode}/images/{imageId}`
///
/// Deletes the game thumbnail.
pub async fn delete_game_thumbnails_games_by_game_id_language_codes_by_language_code_images_by_image_id(
    client: &Client,
    game_id: i64,
    language_code: &str,
    image_id: i64,
) -> Result<ApiEmptyResponseModel> {
    client.send(delete_game_thumbnails_games_by_game_id_language_codes_by_language_code_images_by_image_id_request(game_id, language_code, image_id)?).await?.json()
}

/// `PATCH /v1/localizationtable/gametables/{gameId}`
pub fn patch_localizationtable_gametables_by_game_id_request(
    game_id: i64,
    body: &AssociateLocalizationTablesToGameRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!(
            "/v1/localizationtable/gametables/{}",
            crate::rt::path_value(&game_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/localizationtable/gametables/{gameId}`
pub async fn patch_localizationtable_gametables_by_game_id(
    client: &Client,
    game_id: i64,
    body: &AssociateLocalizationTablesToGameRequest,
) -> Result<AssociateLocalizationTablesToGameResponse> {
    client
        .send(patch_localizationtable_gametables_by_game_id_request(
            game_id, body,
        )?)
        .await?
        .json()
}

/// `POST /v1/name-description/games/translation-history`
///
/// Gets the history for name or description in a provided language.
pub fn post_name_description_games_translation_history_request(
    body: &GetNameDescriptionHistoryV2Request,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        "/v1/name-description/games/translation-history".to_string(),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/name-description/games/translation-history`
///
/// Gets the history for name or description in a provided language.
pub async fn post_name_description_games_translation_history(
    client: &Client,
    body: &GetNameDescriptionHistoryV2Request,
) -> Result<GetNameDescriptionHistoryResponse> {
    client
        .send(post_name_description_games_translation_history_request(body)?)
        .await?
        .json()
}

/// `GET /v1/name-description/games/{gameId}`
///
/// Gets a game's name and description in all supported languages
pub fn get_name_description_games_by_game_id_request(game_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/name-description/games/{}", crate::rt::path_value(&game_id)),
    );
    Ok(r)
}

/// `GET /v1/name-description/games/{gameId}`
///
/// Gets a game's name and description in all supported languages
pub async fn get_name_description_games_by_game_id(
    client: &Client,
    game_id: i64,
) -> Result<ApiArrayResponseNameDescription> {
    client
        .send(get_name_description_games_by_game_id_request(game_id)?)
        .await?
        .json()
}

/// `PATCH /v1/name-description/games/{gameId}`
///
/// Updates a game's name and/or description in multiple languages.
pub fn patch_name_description_games_by_game_id_request(
    game_id: i64,
    body: &UpdateNameDescriptionsRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!("/v1/name-description/games/{}", crate::rt::path_value(&game_id)),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/name-description/games/{gameId}`
///
/// Updates a game's name and/or description in multiple languages.
pub async fn patch_name_description_games_by_game_id(
    client: &Client,
    game_id: i64,
    body: &UpdateNameDescriptionsRequest,
) -> Result<UpdateNameDescriptionsResponse> {
    client
        .send(patch_name_description_games_by_game_id_request(game_id, body)?)
        .await?
        .json()
}

/// `GET /v1/name-description/metadata`
///
/// Rollout settings for name/description migration to new page
pub fn get_name_description_metadata_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/name-description/metadata".to_string());
    Ok(r)
}

/// `GET /v1/name-description/metadata`
///
/// Rollout settings for name/description migration to new page
pub async fn get_name_description_metadata(client: &Client) -> Result<GameNameDescriptionMetadataResponse> {
    client
        .send(get_name_description_metadata_request()?)
        .await?
        .json()
}

/// `GET /v1/source-language/games/{gameId}`
///
/// Gets the source language of a game
pub fn get_source_language_games_by_game_id_request(game_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/source-language/games/{}", crate::rt::path_value(&game_id)),
    );
    Ok(r)
}

/// `GET /v1/source-language/games/{gameId}`
///
/// Gets the source language of a game
pub async fn get_source_language_games_by_game_id(client: &Client, game_id: i64) -> Result<Language> {
    client
        .send(get_source_language_games_by_game_id_request(game_id)?)
        .await?
        .json()
}

/// `PATCH /v1/source-language/games/{gameId}`
///
/// Sets the source language of a game
pub fn patch_source_language_games_by_game_id_request(
    game_id: i64,
    params: &PatchSourceLanguageGamesByGameIdParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!("/v1/source-language/games/{}", crate::rt::path_value(&game_id)),
    );
    r.query("languageCode", &params.language_code);
    Ok(r)
}

/// `PATCH /v1/source-language/games/{gameId}`
///
/// Sets the source language of a game
pub async fn patch_source_language_games_by_game_id(
    client: &Client,
    game_id: i64,
    params: &PatchSourceLanguageGamesByGameIdParams,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(patch_source_language_games_by_game_id_request(game_id, params)?)
        .await?
        .json()
}

/// `GET /v1/source-language/games/{gameId}/language-with-locales`
///
/// Gets the source language of a game
pub fn get_source_language_games_by_game_id_language_with_locales_request(game_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/source-language/games/{}/language-with-locales",
            crate::rt::path_value(&game_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/source-language/games/{gameId}/language-with-locales`
///
/// Gets the source language of a game
pub async fn get_source_language_games_by_game_id_language_with_locales(
    client: &Client,
    game_id: i64,
) -> Result<SourceLanguageWithLocales> {
    client
        .send(get_source_language_games_by_game_id_language_with_locales_request(game_id)?)
        .await?
        .json()
}

/// `GET /v1/supported-languages/games/{gameId}`
///
/// Get the supported languages for a game.
pub fn get_supported_languages_games_by_game_id_request(game_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/supported-languages/games/{}",
            crate::rt::path_value(&game_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/supported-languages/games/{gameId}`
///
/// Get the supported languages for a game.
pub async fn get_supported_languages_games_by_game_id(
    client: &Client,
    game_id: i64,
) -> Result<ApiArrayResponseLanguageOrLocale> {
    client
        .send(get_supported_languages_games_by_game_id_request(game_id)?)
        .await?
        .json()
}

/// `PATCH /v1/supported-languages/games/{gameId}`
///
/// Add or remove supported languages for a game.
pub fn patch_supported_languages_games_by_game_id_request(
    game_id: i64,
    body: &Vec<PatchLanguage>,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!(
            "/v1/supported-languages/games/{}",
            crate::rt::path_value(&game_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/supported-languages/games/{gameId}`
///
/// Add or remove supported languages for a game.
pub async fn patch_supported_languages_games_by_game_id(
    client: &Client,
    game_id: i64,
    body: &Vec<PatchLanguage>,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(patch_supported_languages_games_by_game_id_request(game_id, body)?)
        .await?
        .json()
}

/// `GET /v1/supported-languages/games/{gameId}/automatic-translation-status`
///
/// Get the automatic translation status of supported languages for a game.
pub fn get_supported_languages_games_by_game_id_automatic_translation_status_request(
    game_id: i64,
) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/supported-languages/games/{}/automatic-translation-status",
            crate::rt::path_value(&game_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/supported-languages/games/{gameId}/automatic-translation-status`
///
/// Get the automatic translation status of supported languages for a game.
pub async fn get_supported_languages_games_by_game_id_automatic_translation_status(
    client: &Client,
    game_id: i64,
) -> Result<ApiArrayResponseLanguageOrLocaleSettings> {
    client
        .send(get_supported_languages_games_by_game_id_automatic_translation_status_request(game_id)?)
        .await?
        .json()
}

/// `GET /v1/supported-languages/games/{gameId}/in-experience-language-selection`
///
/// Get the user's in-experience language selector languages for a game.
pub fn get_supported_languages_games_by_game_id_in_experience_language_selection_request(
    game_id: i64,
) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/supported-languages/games/{}/in-experience-language-selection",
            crate::rt::path_value(&game_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/supported-languages/games/{gameId}/in-experience-language-selection`
///
/// Get the user's in-experience language selector languages for a game.
pub async fn get_supported_languages_games_by_game_id_in_experience_language_selection(
    client: &Client,
    game_id: i64,
) -> Result<ApiArrayResponseLanguageOrLocale> {
    client
        .send(get_supported_languages_games_by_game_id_in_experience_language_selection_request(game_id)?)
        .await?
        .json()
}

/// `PATCH /v1/supported-languages/games/{gameId}/languages/{languageCode}/automatic-translation-status`
///
/// Enable or disable automatic translation for a game and language.
pub fn patch_supported_languages_games_by_game_id_languages_by_language_code_automatic_translation_status_request(
    game_id: i64,
    language_code: &str,
    body: &bool,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!(
            "/v1/supported-languages/games/{}/languages/{}/automatic-translation-status",
            crate::rt::path_value(&game_id),
            crate::rt::path_value(&language_code)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/supported-languages/games/{gameId}/languages/{languageCode}/automatic-translation-status`
///
/// Enable or disable automatic translation for a game and language.
pub async fn patch_supported_languages_games_by_game_id_languages_by_language_code_automatic_translation_status(
    client: &Client,
    game_id: i64,
    language_code: &str,
    body: &bool,
) -> Result<EditAutomaticTranslationStatusForGameAndLanguageResponse> {
    client.send(patch_supported_languages_games_by_game_id_languages_by_language_code_automatic_translation_status_request(game_id, language_code, body)?).await?.json()
}

/// `PATCH /v1/supported-languages/games/{gameId}/languages/{languageCode}/image-translation-status`
///
/// Enable or disable image translation for a game and language.
pub fn patch_supported_languages_games_by_game_id_languages_by_language_code_image_translation_status_request(
    game_id: i64,
    language_code: &str,
    body: &bool,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!(
            "/v1/supported-languages/games/{}/languages/{}/image-translation-status",
            crate::rt::path_value(&game_id),
            crate::rt::path_value(&language_code)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/supported-languages/games/{gameId}/languages/{languageCode}/image-translation-status`
///
/// Enable or disable image translation for a game and language.
pub async fn patch_supported_languages_games_by_game_id_languages_by_language_code_image_translation_status(
    client: &Client,
    game_id: i64,
    language_code: &str,
    body: &bool,
) -> Result<EditImageTranslationStatusForGameAndLanguageResponse> {
    client.send(patch_supported_languages_games_by_game_id_languages_by_language_code_image_translation_status_request(game_id, language_code, body)?).await?.json()
}

/// `PATCH /v1/supported-languages/games/{gameId}/languages/{languageCode}/universe-display-info-automatic-translation-settings`
///
/// Update the switch which controls if the UniverseDisplayInformation should be automatically translated.
pub fn patch_supported_languages_games_by_game_id_languages_by_language_code_universe_display_info_automatic_translation_settings_request(
    game_id: i64,
    language_code: &str,
    body: &bool,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!(
            "/v1/supported-languages/games/{}/languages/{}/universe-display-info-automatic-translation-settings",
            crate::rt::path_value(&game_id),
            crate::rt::path_value(&language_code)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/supported-languages/games/{gameId}/languages/{languageCode}/universe-display-info-automatic-translation-settings`
///
/// Update the switch which controls if the UniverseDisplayInformation should be automatically translated.
pub async fn patch_supported_languages_games_by_game_id_languages_by_language_code_universe_display_info_automatic_translation_settings(
    client: &Client,
    game_id: i64,
    language_code: &str,
    body: &bool,
) -> Result<UpdateUniverseDisplayInfoAutomaticTranslationSettingsResponse> {
    client.send(patch_supported_languages_games_by_game_id_languages_by_language_code_universe_display_info_automatic_translation_settings_request(game_id, language_code, body)?).await?.json()
}

/// `GET /v1/supported-languages/games/{gameId}/universe-display-info-automatic-translation-settings`
///
/// Get UniverseDisplayInfo automatic translation settings.
pub fn get_supported_languages_games_by_game_id_universe_display_info_automatic_translation_settings_request(
    game_id: i64,
) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/supported-languages/games/{}/universe-display-info-automatic-translation-settings",
            crate::rt::path_value(&game_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/supported-languages/games/{gameId}/universe-display-info-automatic-translation-settings`
///
/// Get UniverseDisplayInfo automatic translation settings.
pub async fn get_supported_languages_games_by_game_id_universe_display_info_automatic_translation_settings(
    client: &Client,
    game_id: i64,
) -> Result<ApiArrayResponseUniverseDisplayInfoAutomaticTranslationSettings> {
    client.send(get_supported_languages_games_by_game_id_universe_display_info_automatic_translation_settings_request(game_id)?).await?.json()
}

/// `GET /v1/supported-languages/metadata`
///
/// Rollout settings for supported languages of a game
pub fn get_supported_languages_metadata_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/supported-languages/metadata".to_string());
    Ok(r)
}

/// `GET /v1/supported-languages/metadata`
///
/// Rollout settings for supported languages of a game
pub async fn get_supported_languages_metadata(client: &Client) -> Result<SupportedLanguagesMetadataResponse> {
    client
        .send(get_supported_languages_metadata_request()?)
        .await?
        .json()
}

/// `GET /v1/translation-analytics/games/{gameId}/download-translation-analytics-report`
///
/// Download translation analytics report after the report is ready
pub fn get_translation_analytics_games_by_game_id_download_translation_analytics_report_request(
    game_id: i64,
    params: &GetTranslationAnalyticsGamesByGameIdDownloadTranslationAnalyticsReportParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/translation-analytics/games/{}/download-translation-analytics-report",
            crate::rt::path_value(&game_id)
        ),
    );
    r.query("startDateTime", &params.start_date_time);
    r.query("endDateTime", &params.end_date_time);
    r.query("reportType", &params.report_type);
    r.query("reportSubjectTargetId", &params.report_subject_target_id);
    Ok(r)
}

/// `GET /v1/translation-analytics/games/{gameId}/download-translation-analytics-report`
///
/// Download translation analytics report after the report is ready
pub async fn get_translation_analytics_games_by_game_id_download_translation_analytics_report(
    client: &Client,
    game_id: i64,
    params: &GetTranslationAnalyticsGamesByGameIdDownloadTranslationAnalyticsReportParams,
) -> Result<serde_json::Value> {
    client
        .send(
            get_translation_analytics_games_by_game_id_download_translation_analytics_report_request(
                game_id, params,
            )?,
        )
        .await?
        .json()
}

/// `POST /v1/translation-analytics/games/{gameId}/request-translation-analytics-report`
///
/// Request translation analytics report to be generated
pub fn post_translation_analytics_games_by_game_id_request_translation_analytics_report_request(
    game_id: i64,
    body: &RequestTranslationAnalyticsReportRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/translation-analytics/games/{}/request-translation-analytics-report",
            crate::rt::path_value(&game_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/translation-analytics/games/{gameId}/request-translation-analytics-report`
///
/// Request translation analytics report to be generated
pub async fn post_translation_analytics_games_by_game_id_request_translation_analytics_report(
    client: &Client,
    game_id: i64,
    body: &RequestTranslationAnalyticsReportRequest,
) -> Result<RequestTranslationAnalyticsReportResponse> {
    client
        .send(
            post_translation_analytics_games_by_game_id_request_translation_analytics_report_request(
                game_id, body,
            )?,
        )
        .await?
        .json()
}

/// `GET /v1/translation-analytics/metadata`
///
/// Get metadata related to UI and rollout settings
pub fn get_translation_analytics_metadata_request() -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        "/v1/translation-analytics/metadata".to_string(),
    );
    Ok(r)
}

/// `GET /v1/translation-analytics/metadata`
///
/// Get metadata related to UI and rollout settings
pub async fn get_translation_analytics_metadata(
    client: &Client,
) -> Result<TranslationAnalyticsMetadataResponse> {
    client
        .send(get_translation_analytics_metadata_request()?)
        .await?
        .json()
}

/// `GET /v1/ui-configurations`
///
/// Get ui configurations for frontend to use.
pub fn get_ui_configurations_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/ui-configurations".to_string());
    Ok(r)
}

/// `GET /v1/ui-configurations`
///
/// Get ui configurations for frontend to use.
pub async fn get_ui_configurations(client: &Client) -> Result<GetUiConfigurationsResponse> {
    client.send(get_ui_configurations_request()?).await?.json()
}

/// `GET /v1/user-localization-settings/player-choice/{universeId}`
///
/// Get user player choice settings for universe.
pub fn get_user_localization_settings_player_choice_by_universe_id_request(
    universe_id: i64,
) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/user-localization-settings/player-choice/{}",
            crate::rt::path_value(&universe_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/user-localization-settings/player-choice/{universeId}`
///
/// Get user player choice settings for universe.
pub async fn get_user_localization_settings_player_choice_by_universe_id(
    client: &Client,
    universe_id: i64,
) -> Result<GetPlayerChoiceUniverseSettingsResponse> {
    client
        .send(get_user_localization_settings_player_choice_by_universe_id_request(universe_id)?)
        .await?
        .json()
}

/// `GET /v1/user-localization-settings/universe/{universeId}`
///
/// Get user localization settings for universe.
pub fn get_user_localization_settings_universe_by_universe_id_request(universe_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/user-localization-settings/universe/{}",
            crate::rt::path_value(&universe_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/user-localization-settings/universe/{universeId}`
///
/// Get user localization settings for universe.
pub async fn get_user_localization_settings_universe_by_universe_id(
    client: &Client,
    universe_id: i64,
) -> Result<GetUserLocalizationSettingsForUniverseResponse> {
    client
        .send(get_user_localization_settings_universe_by_universe_id_request(
            universe_id,
        )?)
        .await?
        .json()
}

/// `POST /v1/user-localization-settings/universe/{universeId}`
///
/// Set user localization settings for universe.
pub fn post_user_localization_settings_universe_by_universe_id_request(
    universe_id: i64,
    body: &SetUserLocalizationSettingsRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/user-localization-settings/universe/{}",
            crate::rt::path_value(&universe_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/user-localization-settings/universe/{universeId}`
///
/// Set user localization settings for universe.
pub async fn post_user_localization_settings_universe_by_universe_id(
    client: &Client,
    universe_id: i64,
    body: &SetUserLocalizationSettingsRequest,
) -> Result<SetUserLocalizationSettingsResponse> {
    client
        .send(post_user_localization_settings_universe_by_universe_id_request(
            universe_id,
            body,
        )?)
        .await?
        .json()
}

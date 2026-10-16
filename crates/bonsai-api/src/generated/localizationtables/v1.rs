//! `localizationtables.roblox.com` v1 — generated from `vendor/roblox-api-tracker/localizationtables/v1.json`.
//!
//! Do not edit. Change the spec or `bonsai-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "localizationtables.roblox.com";

/// `Roblox.InGameContentTables.Client.GameLocation`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GameLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

crate::api_enum! {
    str
    /// \['User' = 0, 'Automation' = 1\]
    TranslatorAgentType {
    User = 0 => "User",
    Automation = 1 => "Automation",
    }
}

/// `Roblox.LocalizationTables.Api.Translator`
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

/// `Roblox.LocalizationTables.Api.Translation`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Translation {
    #[serde(rename = "feedbackCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(rename = "translationText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translator: Option<Translator>,
    #[serde(rename = "updatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
}

/// `Roblox.LocalizationTables.Api.AssetEntry`
///
/// A source asset (image) entry in a localization table, along with its per-locale asset translations. Unlike Roblox.LocalizationTables.Api.Entry, an asset entry is identified by its source asset id and has no key/context/source; each translation's Roblox.LocalizationTables.Api.Translation.TranslationText holds the translated asset id.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AssetEntry {
    #[serde(rename = "assetClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_class: Option<i32>,
    #[serde(rename = "assetProp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_prop: Option<i32>,
    #[serde(rename = "assetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<i32>,
    #[serde(rename = "createdTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<Translator>,
    #[serde(rename = "gameLocations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_locations: Option<Vec<GameLocation>>,
    #[serde(rename = "sourceAssetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_asset_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translations: Option<Vec<Translation>>,
}

/// `Roblox.LocalizationTables.Api.AutoLocalizationMetadataResponse`
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

/// `Roblox.LocalizationTables.Api.MatchedEntry`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MatchedEntry {
    #[serde(rename = "matchedParamIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_param_index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// `Roblox.LocalizationTables.Api.AutoScrapeEntryMetadata`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AutoScrapeEntryMetadata {
    #[serde(rename = "matchedEntry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_entry: Option<MatchedEntry>,
    #[serde(rename = "osPlatform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_platform: Option<String>,
    #[serde(rename = "sessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}

/// `Roblox.LocalizationTables.Api.AutoScrapeEntry`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AutoScrapeEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<AutoScrapeEntryMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screenshot: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

crate::api_enum! {
    str
    CreateTableRequestOwnerType {
    User = 0 => "User",
    Group = 1 => "Group",
    }
}

/// `Roblox.LocalizationTables.Api.CreateTableRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CreateTableRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ownerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<i64>,
    #[serde(rename = "ownerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_type: Option<CreateTableRequestOwnerType>,
}

/// `Roblox.LocalizationTables.Api.CreateTableResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CreateTableResponse {
    #[serde(rename = "assetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

crate::api_enum! {
    str
    EntryIdentifierEntryFormat {
    Invalid = 0 => "Invalid",
    Legacy = 1 => "Legacy",
    Icu = 2 => "Icu",
    }
}

/// `Roblox.LocalizationTables.Api.EntryIdentifier`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EntryIdentifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "entryFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_format: Option<EntryIdentifierEntryFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

crate::api_enum! {
    str
    /// In which order the data is sorted.
    CursorEntryIdentifierSortOrder {
    Asc = 0 => "Asc",
    Desc = 1 => "Desc",
    }
}

/// `Roblox.LocalizationTables.Api.CursorEntryIdentifier`
///
/// A model that contains an entry identifier and an associated cursor for paged lookups.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CursorEntryIdentifier {
    /// The translation history count to get.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// The location to begin our query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<EntryIdentifier>,
    /// In which order the data is sorted.
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<CursorEntryIdentifierSortOrder>,
}

/// `Roblox.LocalizationTables.Api.EntryMetadata`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EntryMetadata {
    #[serde(rename = "entryFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_format: Option<EntryIdentifierEntryFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,
    #[serde(rename = "gameLocations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_locations: Option<Vec<GameLocation>>,
}

/// `Roblox.LocalizationTables.Api.Entry`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Entry {
    #[serde(rename = "createdTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<EntryIdentifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<EntryMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translations: Option<Vec<Translation>>,
}

/// `Roblox.LocalizationTables.Api.EntryIdentifierWithTranslation`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EntryIdentifierWithTranslation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "entryFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_format: Option<EntryIdentifierEntryFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<Translation>,
}

/// `Roblox.LocalizationTables.Api.EntryOperationLimits`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EntryOperationLimits {
    /// Maximum character limit for entry context
    #[serde(rename = "maxContextLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_context_length: Option<i32>,
    /// Maximum character limit for entry example
    #[serde(rename = "maxExampleLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_example_length: Option<i32>,
    /// Maximum character limit for game location path
    #[serde(rename = "maxGameLocationPathLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_game_location_path_length: Option<i32>,
    /// Maximum character limit for entry key
    #[serde(rename = "maxKeyLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_key_length: Option<i32>,
    /// Maximum character limit for entry source text
    #[serde(rename = "maxSourceLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_source_length: Option<i32>,
}

crate::api_enum! {
    str
    /// Enum for translation feedback reason.
    EntryTranslationFeedbackReasons {
    None = 0 => "None",
    Untranslated = 1 => "Untranslated",
    Inaccurate = 2 => "Inaccurate",
    SpellingOrGrammar = 3 => "SpellingOrGrammar",
    Inappropriate = 4 => "Inappropriate",
    }
}

/// `Roblox.LocalizationTables.Api.EntryTranslationFeedback`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EntryTranslationFeedback {
    /// The number of unique reporters that send feedback to the given entry identifier.
    #[serde(rename = "feedbackCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<EntryIdentifierWithTranslation>,
    /// A set of player translation suggestion in text for the given entry identifier.
    #[serde(rename = "playerSuggestionText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_suggestion_text: Option<Vec<String>>,
    /// A set of translation feedback reasons for the given entry identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasons: Option<Vec<EntryTranslationFeedbackReasons>>,
    /// Roblox translation suggestion in text for the given entry identifier.
    #[serde(rename = "robloxSuggestionText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roblox_suggestion_text: Option<String>,
}

/// `Roblox.LocalizationTables.Api.TranslationHistory`
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

/// `Roblox.LocalizationTables.Api.EntryTranslationHistoryPaged`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EntryTranslationHistoryPaged {
    /// A batch of TranslationHistory for the given entry identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<Vec<TranslationHistory>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<EntryIdentifier>,
    /// The cursor to send up on the next request if more history data is required.
    #[serde(rename = "nextCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

/// `Roblox.LocalizationTables.Api.Error`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ErrorModel {
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

/// `Roblox.LocalizationTables.Api.FailedEntry`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FailedEntry {
    #[serde(rename = "createdTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<EntryIdentifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<EntryMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translations: Option<Vec<Translation>>,
}

/// `Roblox.LocalizationTables.Api.FailedEntryTranslationHistoryPaged`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FailedEntryTranslationHistoryPaged {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<EntryIdentifier>,
}

/// `Roblox.LocalizationTables.Api.GameAutolocalizationInformationResponse`
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
    #[serde(rename = "isAutomaticEntriesDeletionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_automatic_entries_deletion_enabled: Option<bool>,
    #[serde(rename = "isAutomaticEntriesSettingEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_automatic_entries_setting_enabled: Option<bool>,
    #[serde(rename = "shouldUseImageLocalizationTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_use_image_localization_table: Option<bool>,
    #[serde(rename = "shouldUseLocalizationTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_use_localization_table: Option<bool>,
    #[serde(rename = "sourceLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_language: Option<String>,
}

/// `Roblox.LocalizationTables.Api.TableOperationLimits`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TableOperationLimits {
    /// Maximum number of entries for a patch request
    #[serde(rename = "maxEntriesPerUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_entries_per_update: Option<i32>,
}

/// `Roblox.LocalizationTables.Api.GetLimitsResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetLimitsResponse {
    #[serde(rename = "entryOperationLimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_operation_limits: Option<EntryOperationLimits>,
    #[serde(rename = "tableOperationLimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_operation_limits: Option<TableOperationLimits>,
}

/// `Roblox.LocalizationTables.Api.GetTableAssetEntriesPagedResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetTableAssetEntriesPagedResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<AssetEntry>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
}

/// `Roblox.LocalizationTables.Api.GetTableEntriesPagedResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetTableEntriesPagedResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<Entry>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
}

/// `Roblox.LocalizationTables.Api.GetTableEntriesTranslationFeedbackRequest`
///
/// A request model for GetTableEntriesTranslationFeedback.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetTableEntriesTranslationFeedbackRequest {
    /// entry identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<EntryIdentifierWithTranslation>>,
    /// locale code of source language, we only accept language code at the moment.
    #[serde(rename = "sourceLocale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_locale: Option<String>,
}

/// `Roblox.LocalizationTables.Api.GetTableEntriesTranslationFeedbackResponse`
///
/// A response model for GetTableEntriesTranslationFeedback.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetTableEntriesTranslationFeedbackResponse {
    /// The entries with their identifier, translation feedback details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<EntryTranslationFeedback>>,
    /// The entries' tableId.
    #[serde(rename = "tableId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_id: Option<String>,
}

crate::api_enum! {
    str
    GetTableEntriesTranslationHistoryRequestSourceType {
    Invalid = 0 => "Invalid",
    Text = 1 => "Text",
    AssetImage = 2 => "AssetImage",
    }
}

/// `Roblox.LocalizationTables.Api.GetTableEntriesTranslationHistoryRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetTableEntriesTranslationHistoryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<CursorEntryIdentifier>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(rename = "sourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<GetTableEntriesTranslationHistoryRequestSourceType>,
}

/// `Roblox.LocalizationTables.Api.GetTableEntriesTranslationHistoryResponse`
///
/// A response model for GetTableEntriesTranslationHistory.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetTableEntriesTranslationHistoryResponse {
    /// The entries with their identifier, translation history, and next cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<EntryTranslationHistoryPaged>>,
    /// The failed entries.
    #[serde(rename = "failedEntries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_entries: Option<Vec<FailedEntryTranslationHistoryPaged>>,
    /// The locale of the translations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// The entries' tableId.
    #[serde(rename = "tableId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_id: Option<String>,
}

/// `Roblox.LocalizationTables.Api.GetTableEntryCountResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetTableEntryCountResponse {
    #[serde(rename = "entryCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// `Roblox.LocalizationTables.Api.GetTableResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetTableResponse {
    #[serde(rename = "assetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ownerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<i64>,
    /// Enum for valid OwnerTypes.
    #[serde(rename = "ownerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_type: Option<CreateTableRequestOwnerType>,
}

/// `Roblox.LocalizationTables.Api.IngestContentMetadataPlaceInformation`
///
/// The place information metadata of an ingest content request.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct IngestContentMetadataPlaceInformation {
    /// The place id of an ingest content request.
    #[serde(rename = "placeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_id: Option<i64>,
    /// The place version number of an ingest content request.
    #[serde(rename = "placeVersionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_version_number: Option<i32>,
}

/// `Roblox.LocalizationTables.Api.IngestContentMetadata`
///
/// The metadata of an ingest content request.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct IngestContentMetadata {
    #[serde(rename = "placeInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_information: Option<IngestContentMetadataPlaceInformation>,
}

/// `Roblox.LocalizationTables.Api.IngestAutoScrapedContentForGameRequest`
///
/// An ingest content request to IngestAutoScrapedContentForGame.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct IngestAutoScrapedContentForGameRequest {
    /// The entries of an ingest content request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<AutoScrapeEntry>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<IngestContentMetadata>,
}

/// `Roblox.LocalizationTables.Api.LocalizationTablesMetadataResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LocalizationTablesMetadataResponse {
    /// Is access to translation metadata feature enabled
    #[serde(rename = "isAccessToTranslationMetaDataEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_access_to_translation_meta_data_enabled: Option<bool>,
    /// Is filter for automatic translations on UI is enabled
    #[serde(rename = "isAutomaticTranslationFilterEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_automatic_translation_filter_enabled: Option<bool>,
    /// Is bulk upload feature enabled
    #[serde(rename = "isBulkUploadFeatureEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_bulk_upload_feature_enabled: Option<bool>,
    /// Is CSV download feature enabled
    #[serde(rename = "isCsvDownloadEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_csv_download_enabled: Option<bool>,
    /// Is access to translation metadata feature enabled
    #[serde(rename = "isTranslationManagementRedirectionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_translation_management_redirection_enabled: Option<bool>,
    /// Is untranslated filter on UI is enabled
    #[serde(rename = "isUntranslatedFilterEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_untranslated_filter_enabled: Option<bool>,
}

/// `Roblox.LocalizationTables.Api.ModifiedEntry`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ModifiedEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<EntryIdentifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translations: Option<Vec<Translation>>,
}

crate::api_enum! {
    str
    /// \['Invalid' = 0, 'User' = 1, 'Automation' = 2, 'Default' = 3\]
    ChangeAgentChangeAgentType {
    Invalid = 0 => "Invalid",
    User = 1 => "User",
    Automation = 2 => "Automation",
    Default = 3 => "Default",
    }
}

crate::api_enum! {
    str
    /// \['None' = 0, 'Id' = 2\]
    ChangeAgentOptionalIdCase {
    None = 0 => "None",
    Id = 1 => "Id",
    }
}

/// `Roblox.Localizationtables.Localizationtables.V1.ChangeAgent`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChangeAgent {
    /// \['Invalid' = 0, 'User' = 1, 'Automation' = 2, 'Default' = 3\]
    #[serde(rename = "ChangeAgentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_agent_type: Option<ChangeAgentChangeAgentType>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// \['None' = 0, 'Id' = 2\]
    #[serde(rename = "OptionalIdCase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_id_case: Option<ChangeAgentOptionalIdCase>,
}

/// `Roblox.LocalizationTables.Api.PatchTranslation`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PatchTranslation {
    #[serde(rename = "changeAgent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_agent: Option<ChangeAgent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(rename = "translationText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation_text: Option<String>,
    #[serde(rename = "updatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
}

/// `Roblox.LocalizationTables.Api.PatchEntry`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PatchEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<EntryIdentifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<EntryMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translations: Option<Vec<PatchTranslation>>,
}

/// `Roblox.LocalizationTables.Api.RaiseEventForAutoScrapedEntriesCleanupRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RaiseEventForAutoScrapedEntriesCleanupRequest {
    /// The time range to remove entries from. Following ISO 8601 Durations format
    #[serde(rename = "maxAgeForFlush")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age_for_flush: Option<String>,
}

/// `Roblox.LocalizationTables.Api.SetAutolocalizationSettingsForGameRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SetAutolocalizationSettingsForGameRequest {
    #[serde(rename = "isAutoLocalizationForImageEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_auto_localization_for_image_enabled: Option<bool>,
    #[serde(rename = "isAutolocalizationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_autolocalization_enabled: Option<bool>,
    #[serde(rename = "isAutomaticEntriesDeletionsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_automatic_entries_deletions_enabled: Option<bool>,
    #[serde(rename = "isAutomaticEntriesSettingEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_automatic_entries_setting_enabled: Option<bool>,
    #[serde(rename = "shouldUseImageLocalizationTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_use_image_localization_table: Option<bool>,
    #[serde(rename = "shouldUseLocalizationTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_use_localization_table: Option<bool>,
}

/// `Roblox.LocalizationTables.Api.SetAutolocalizationTableForGameRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SetAutolocalizationTableForGameRequest {
    #[serde(rename = "tableId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_id: Option<String>,
}

/// `Roblox.LocalizationTables.Api.UpdateTableContentsRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateTableContentsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<PatchEntry>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.LocalizationTables.Api.UpdateTableContentsResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateTableContentsResponse {
    #[serde(rename = "failedEntriesAndTranslations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_entries_and_translations: Option<Vec<FailedEntry>>,
    #[serde(rename = "modifiedEntriesAndTranslations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_entries_and_translations: Option<Vec<ModifiedEntry>>,
}

/// `Roblox.Web.WebAPI.ApiEmptyResponseModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiEmptyResponseModel {}

/// Query, header and form inputs for [`patch_localization_table_tables_by_table_id`].
#[derive(Debug, Clone, Default)]
pub struct PatchLocalizationTableTablesByTableIdParams {
    /// The game id.
    pub game_id: Option<i64>,
}

/// Query, header and form inputs for [`get_localization_table_tables_by_table_id_asset_entries`].
#[derive(Debug, Clone, Default)]
pub struct GetLocalizationTableTablesByTableIdAssetEntriesParams {
    /// The source asset id to start after. If null, starts from the first page. A null cursor in the response means there are no more entries.
    pub cursor: Option<String>,
    /// The universe id that owns the table.
    pub game_id: Option<i64>,
}

/// Query, header and form inputs for [`get_localization_table_tables_by_table_id_entries`].
#[derive(Debug, Clone, Default)]
pub struct GetLocalizationTableTablesByTableIdEntriesParams {
    /// If null, there are no more entries in the table and you've reached the last page.
    pub cursor: Option<String>,
    pub game_id: Option<i64>,
    pub entry_format: Option<EntryIdentifierEntryFormat>,
}

/// Query, header and form inputs for [`post_localization_table_tables_by_table_id_entries_translation_feedback`].
#[derive(Debug, Clone, Default)]
pub struct PostLocalizationTableTablesByTableIdEntriesTranslationFeedbackParams {
    /// The game id.
    pub game_id: Option<i64>,
}

/// Query, header and form inputs for [`post_localization_table_tables_by_table_id_entries_translation_history`].
#[derive(Debug, Clone, Default)]
pub struct PostLocalizationTableTablesByTableIdEntriesTranslationHistoryParams {
    /// The game id.
    pub game_id: Option<i64>,
}

/// Query, header and form inputs for [`get_localization_table_tables_by_table_id_entry_count`].
#[derive(Debug, Clone, Default)]
pub struct GetLocalizationTableTablesByTableIdEntryCountParams {
    /// The game id
    pub game_id: Option<i64>,
    pub entry_format: Option<EntryIdentifierEntryFormat>,
}

/// `POST /v1/auto-localization-table/games/{gameId}/assets-generation-request`
///
/// Generates localization asset of a game.
pub fn post_auto_localization_table_games_by_game_id_assets_generation_request_request(
    game_id: i64,
) -> Result<Request> {
    let r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/auto-localization-table/games/{}/assets-generation-request",
            crate::rt::path_value(&game_id)
        ),
    );
    Ok(r)
}

/// `POST /v1/auto-localization-table/games/{gameId}/assets-generation-request`
///
/// Generates localization asset of a game.
pub async fn post_auto_localization_table_games_by_game_id_assets_generation_request(
    client: &Client,
    game_id: i64,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_auto_localization_table_games_by_game_id_assets_generation_request_request(game_id)?)
        .await?
        .json()
}

/// `POST /v1/auto-localization-table/games/{gameId}/auto-scrape-cleanup-request`
///
/// Enqueues an event to flush the auto scraped entries which doesn't have translations.
pub fn post_auto_localization_table_games_by_game_id_auto_scrape_cleanup_request_request(
    game_id: i64,
    body: &RaiseEventForAutoScrapedEntriesCleanupRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/auto-localization-table/games/{}/auto-scrape-cleanup-request",
            crate::rt::path_value(&game_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/auto-localization-table/games/{gameId}/auto-scrape-cleanup-request`
///
/// Enqueues an event to flush the auto scraped entries which doesn't have translations.
pub async fn post_auto_localization_table_games_by_game_id_auto_scrape_cleanup_request(
    client: &Client,
    game_id: i64,
    body: &RaiseEventForAutoScrapedEntriesCleanupRequest,
) -> Result<serde_json::Value> {
    client
        .send(
            post_auto_localization_table_games_by_game_id_auto_scrape_cleanup_request_request(game_id, body)?,
        )
        .await?
        .json()
}

/// `PATCH /v1/auto-localization-table/games/{gameId}/ingestion`
///
/// Ingests entries for auto localization. Needs to be an authorized user.
pub fn patch_auto_localization_table_games_by_game_id_ingestion_request(
    game_id: i64,
    body: &IngestAutoScrapedContentForGameRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!(
            "/v1/auto-localization-table/games/{}/ingestion",
            crate::rt::path_value(&game_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/auto-localization-table/games/{gameId}/ingestion`
///
/// Ingests entries for auto localization. Needs to be an authorized user.
pub async fn patch_auto_localization_table_games_by_game_id_ingestion(
    client: &Client,
    game_id: i64,
    body: &IngestAutoScrapedContentForGameRequest,
) -> Result<serde_json::Value> {
    client
        .send(patch_auto_localization_table_games_by_game_id_ingestion_request(
            game_id, body,
        )?)
        .await?
        .json()
}

/// `POST /v1/autolocalization/games/{gameId}/autolocalizationtable`
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
pub fn get_autolocalization_metadata_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/autolocalization/metadata".to_string());
    Ok(r)
}

/// `GET /v1/autolocalization/metadata`
///
/// Metadata for AutoLocalization Configuration
pub async fn get_autolocalization_metadata(client: &Client) -> Result<AutoLocalizationMetadataResponse> {
    client
        .send(get_autolocalization_metadata_request()?)
        .await?
        .json()
}

/// `GET /v1/localization-table/limits`
///
/// Get limits for translation table entries operations
pub fn get_localization_table_limits_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/localization-table/limits".to_string());
    Ok(r)
}

/// `GET /v1/localization-table/limits`
///
/// Get limits for translation table entries operations
pub async fn get_localization_table_limits(client: &Client) -> Result<GetLimitsResponse> {
    client
        .send(get_localization_table_limits_request()?)
        .await?
        .json()
}

/// `GET /v1/localization-table/metadata`
///
/// Get metadata for localization UI
pub fn get_localization_table_metadata_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/localization-table/metadata".to_string());
    Ok(r)
}

/// `GET /v1/localization-table/metadata`
///
/// Get metadata for localization UI
pub async fn get_localization_table_metadata(client: &Client) -> Result<LocalizationTablesMetadataResponse> {
    client
        .send(get_localization_table_metadata_request()?)
        .await?
        .json()
}

/// `POST /v1/localization-table/tables`
///
/// Creates a Localization Table with the given data. Note that this endpoint simply creates a table and does not associate it with any universe, so if intending to use this to create tables usable in experience more setup will be needed to grant those experiences access.
pub fn post_localization_table_tables_request(body: &CreateTableRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/localization-table/tables".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/localization-table/tables`
///
/// Creates a Localization Table with the given data. Note that this endpoint simply creates a table and does not associate it with any universe, so if intending to use this to create tables usable in experience more setup will be needed to grant those experiences access.
pub async fn post_localization_table_tables(
    client: &Client,
    body: &CreateTableRequest,
) -> Result<CreateTableResponse> {
    client
        .send(post_localization_table_tables_request(body)?)
        .await?
        .json()
}

/// `GET /v1/localization-table/tables/{assetId}`
///
/// Get table information by the assetId of the table.
pub fn get_localization_table_tables_by_asset_id_request(asset_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/localization-table/tables/{}",
            crate::rt::path_value(&asset_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/localization-table/tables/{assetId}`
///
/// Get table information by the assetId of the table.
pub async fn get_localization_table_tables_by_asset_id(
    client: &Client,
    asset_id: i64,
) -> Result<GetTableResponse> {
    client
        .send(get_localization_table_tables_by_asset_id_request(asset_id)?)
        .await?
        .json()
}

/// `GET /v1/localization-table/tables/{tableId}`
///
/// Get table information by the id of the table.
pub fn get_localization_table_tables_by_table_id_request(table_id: &str) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/localization-table/tables/{}",
            crate::rt::path_value(&table_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/localization-table/tables/{tableId}`
///
/// Get table information by the id of the table.
pub async fn get_localization_table_tables_by_table_id(
    client: &Client,
    table_id: &str,
) -> Result<GetTableResponse> {
    client
        .send(get_localization_table_tables_by_table_id_request(table_id)?)
        .await?
        .json()
}

/// `PATCH /v1/localization-table/tables/{tableId}`
///
/// Updates the tables contents based on what is provided.
pub fn patch_localization_table_tables_by_table_id_request(
    table_id: &str,
    params: &PatchLocalizationTableTablesByTableIdParams,
    body: &UpdateTableContentsRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!(
            "/v1/localization-table/tables/{}",
            crate::rt::path_value(&table_id)
        ),
    );
    r.query("gameId", &params.game_id);
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/localization-table/tables/{tableId}`
///
/// Updates the tables contents based on what is provided.
pub async fn patch_localization_table_tables_by_table_id(
    client: &Client,
    table_id: &str,
    params: &PatchLocalizationTableTablesByTableIdParams,
    body: &UpdateTableContentsRequest,
) -> Result<UpdateTableContentsResponse> {
    client
        .send(patch_localization_table_tables_by_table_id_request(
            table_id, params, body,
        )?)
        .await?
        .json()
}

/// `GET /v1/localization-table/tables/{tableId}/asset-entries`
///
/// Gets a page of asset (image) entries for the specified table, along with the table's per-locale asset translations.
pub fn get_localization_table_tables_by_table_id_asset_entries_request(
    table_id: &str,
    params: &GetLocalizationTableTablesByTableIdAssetEntriesParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/localization-table/tables/{}/asset-entries",
            crate::rt::path_value(&table_id)
        ),
    );
    r.query("cursor", &params.cursor);
    r.query("gameId", &params.game_id);
    Ok(r)
}

/// `GET /v1/localization-table/tables/{tableId}/asset-entries`
///
/// Gets a page of asset (image) entries for the specified table, along with the table's per-locale asset translations.
pub async fn get_localization_table_tables_by_table_id_asset_entries(
    client: &Client,
    table_id: &str,
    params: &GetLocalizationTableTablesByTableIdAssetEntriesParams,
) -> Result<GetTableAssetEntriesPagedResponse> {
    client
        .send(get_localization_table_tables_by_table_id_asset_entries_request(
            table_id, params,
        )?)
        .await?
        .json()
}

/// `GET /v1/localization-table/tables/{tableId}/entries`
///
/// Gets a batch of entries for a table.
pub fn get_localization_table_tables_by_table_id_entries_request(
    table_id: &str,
    params: &GetLocalizationTableTablesByTableIdEntriesParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/localization-table/tables/{}/entries",
            crate::rt::path_value(&table_id)
        ),
    );
    r.query("cursor", &params.cursor);
    r.query("gameId", &params.game_id);
    r.query("entryFormat", &params.entry_format);
    Ok(r)
}

/// `GET /v1/localization-table/tables/{tableId}/entries`
///
/// Gets a batch of entries for a table.
pub async fn get_localization_table_tables_by_table_id_entries(
    client: &Client,
    table_id: &str,
    params: &GetLocalizationTableTablesByTableIdEntriesParams,
) -> Result<GetTableEntriesPagedResponse> {
    client
        .send(get_localization_table_tables_by_table_id_entries_request(
            table_id, params,
        )?)
        .await?
        .json()
}

/// `POST /v1/localization-table/tables/{tableId}/entries/translation-feedback`
///
/// Gets the translation feedback for each entry passed in.
pub fn post_localization_table_tables_by_table_id_entries_translation_feedback_request(
    table_id: &str,
    params: &PostLocalizationTableTablesByTableIdEntriesTranslationFeedbackParams,
    body: &GetTableEntriesTranslationFeedbackRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/localization-table/tables/{}/entries/translation-feedback",
            crate::rt::path_value(&table_id)
        ),
    );
    r.query("gameId", &params.game_id);
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/localization-table/tables/{tableId}/entries/translation-feedback`
///
/// Gets the translation feedback for each entry passed in.
pub async fn post_localization_table_tables_by_table_id_entries_translation_feedback(
    client: &Client,
    table_id: &str,
    params: &PostLocalizationTableTablesByTableIdEntriesTranslationFeedbackParams,
    body: &GetTableEntriesTranslationFeedbackRequest,
) -> Result<GetTableEntriesTranslationFeedbackResponse> {
    client
        .send(
            post_localization_table_tables_by_table_id_entries_translation_feedback_request(
                table_id, params, body,
            )?,
        )
        .await?
        .json()
}

/// `POST /v1/localization-table/tables/{tableId}/entries/translation-history`
///
/// Gets the translation history for each entry passed in.
pub fn post_localization_table_tables_by_table_id_entries_translation_history_request(
    table_id: &str,
    params: &PostLocalizationTableTablesByTableIdEntriesTranslationHistoryParams,
    body: &GetTableEntriesTranslationHistoryRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/localization-table/tables/{}/entries/translation-history",
            crate::rt::path_value(&table_id)
        ),
    );
    r.query("gameId", &params.game_id);
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/localization-table/tables/{tableId}/entries/translation-history`
///
/// Gets the translation history for each entry passed in.
pub async fn post_localization_table_tables_by_table_id_entries_translation_history(
    client: &Client,
    table_id: &str,
    params: &PostLocalizationTableTablesByTableIdEntriesTranslationHistoryParams,
    body: &GetTableEntriesTranslationHistoryRequest,
) -> Result<GetTableEntriesTranslationHistoryResponse> {
    client
        .send(
            post_localization_table_tables_by_table_id_entries_translation_history_request(
                table_id, params, body,
            )?,
        )
        .await?
        .json()
}

/// `GET /v1/localization-table/tables/{tableId}/entry-count`
///
/// Gets the number of entries in the specified table
pub fn get_localization_table_tables_by_table_id_entry_count_request(
    table_id: &str,
    params: &GetLocalizationTableTablesByTableIdEntryCountParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/localization-table/tables/{}/entry-count",
            crate::rt::path_value(&table_id)
        ),
    );
    r.query("gameId", &params.game_id);
    r.query("entryFormat", &params.entry_format);
    Ok(r)
}

/// `GET /v1/localization-table/tables/{tableId}/entry-count`
///
/// Gets the number of entries in the specified table
pub async fn get_localization_table_tables_by_table_id_entry_count(
    client: &Client,
    table_id: &str,
    params: &GetLocalizationTableTablesByTableIdEntryCountParams,
) -> Result<GetTableEntryCountResponse> {
    client
        .send(get_localization_table_tables_by_table_id_entry_count_request(
            table_id, params,
        )?)
        .await?
        .json()
}

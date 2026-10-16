//! `apis.roblox.com` undocumented — generated from `specs/undocumented/apis.roblox.com.yaml`.
//!
//! Do not edit. Change the spec or `rbx-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "apis.roblox.com";

/// `AgeRecommendation`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AgeRecommendation {
    #[serde(rename = "contentMaturity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_maturity: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "minimumAge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_age: Option<i32>,
    #[serde(rename = "minimumAgeDisplay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_age_display: Option<String>,
}

/// `Descriptor`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Descriptor {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `DescriptorUsage`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DescriptorUsage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptor: Option<Descriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `AgeRecommendationSummary`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AgeRecommendationSummary {
    #[serde(rename = "ageRecommendation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_recommendation: Option<AgeRecommendation>,
}

/// `AgeRecommendationDetails`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AgeRecommendationDetails {
    #[serde(rename = "descriptorUsages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptor_usages: Option<Vec<DescriptorUsage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<AgeRecommendationSummary>,
}

/// `AgeRecommendationRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AgeRecommendationRequest {
    #[serde(rename = "universeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_id: Option<String>,
}

/// `AgeRecommendationResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AgeRecommendationResponse {
    #[serde(rename = "ageRecommendationDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_recommendation_details: Option<AgeRecommendationDetails>,
}

/// `BlockedUsers`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BlockedUsers {
    #[serde(rename = "blockedUserIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_user_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// `BlockedUsersResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BlockedUsersResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<BlockedUsers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// `CaptchaMetadata`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CaptchaMetadata {
    #[serde(rename = "funCaptchaPublicKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fun_captcha_public_keys: Option<HashMap<String, String>>,
}

/// `ChallengeContinueRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChallengeContinueRequest {
    #[serde(rename = "challengeID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_id: Option<String>,
    /// JSON text, not an object
    #[serde(rename = "challengeMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_metadata: Option<String>,
    /// twostepverification, captcha, ...
    #[serde(rename = "challengeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_type: Option<String>,
}

/// `ChallengeContinueResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChallengeContinueResponse {
    #[serde(rename = "challengeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_id: Option<String>,
    /// JSON text of the next challenge, if any
    #[serde(rename = "challengeMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_metadata: Option<String>,
    #[serde(rename = "challengeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_type: Option<String>,
}

/// `ChatMessagePiece`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChatMessagePiece {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// `ChatMessage`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChatMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_to_display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pieces: Option<Vec<ChatMessagePiece>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_user_id: Option<i64>,
    /// success, moderated, ...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// user, system, ...
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// hidden, ...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

/// `ChatUser`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChatUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub combined_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `ChatConversation`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChatConversation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<ChatMessage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_user_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// one_to_one, group
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unread_message_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<HashMap<String, ChatUser>>,
}

/// `ChatConversationRef`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChatConversationRef {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
}

/// `ChatConversations`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChatConversations {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversations: Option<Vec<ChatConversation>>,
}

/// `ChatConversationsPage`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChatConversationsPage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversations: Option<Vec<ChatConversation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

/// `ChatNewConversation`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChatNewConversation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_user_ids: Option<Vec<i64>>,
    /// one_to_one, group
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// `ChatCreateRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChatCreateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversations: Option<Vec<ChatNewConversation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_user_data: Option<bool>,
}

/// `ChatMarkRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChatMarkRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_ids: Option<Vec<String>>,
}

/// `ChatMessagesPage`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChatMessagesPage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<ChatMessage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

/// `ChatOutgoing`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChatOutgoing {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

/// `ChatRename`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChatRename {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `ChatSendRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChatSendRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<ChatOutgoing>>,
}

/// `ChatSendResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChatSendResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<ChatMessage>>,
}

/// `ChatSettings`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChatSettings {
    #[serde(rename = "isChatEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_chat_enabled: Option<bool>,
    #[serde(rename = "isChatEnabledByPrivacySetting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_chat_enabled_by_privacy_setting: Option<String>,
    #[serde(rename = "isGroupChatEnabledByPrivacySetting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_group_chat_enabled_by_privacy_setting: Option<String>,
    #[serde(rename = "languageForPrivacySettingUnavailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_for_privacy_setting_unavailable: Option<String>,
    #[serde(rename = "maxConversationTitleLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_conversation_title_length: Option<i32>,
}

/// `ChatStatus`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChatStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// `ChatUnread`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChatUnread {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_unread_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_unread_message_count: Option<i64>,
}

/// `ChatUpdateRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChatUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversations: Option<Vec<ChatRename>>,
}

/// `ChatUsersRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChatUsersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<i64>>,
}

/// `CollectibleItemDetails`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CollectibleItemDetails {
    #[serde(rename = "collectibleItemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collectible_item_id: Option<String>,
    #[serde(rename = "collectibleProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collectible_product_id: Option<String>,
    #[serde(rename = "creatorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<i64>,
    #[serde(rename = "creatorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_name: Option<String>,
    /// User or Group
    #[serde(rename = "creatorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_type: Option<String>,
    #[serde(rename = "itemTargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_target_id: Option<i64>,
    #[serde(rename = "lowestAvailableResaleItemInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lowest_available_resale_item_instance_id: Option<String>,
    #[serde(rename = "lowestAvailableResaleProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lowest_available_resale_product_id: Option<String>,
    #[serde(rename = "lowestResalePrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lowest_resale_price: Option<i64>,
    /// null when the creator doesn't sell it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
    /// ShopOnly, ShopAndAllExperiences, ExperiencesDevApiOnly, ...
    #[serde(rename = "saleLocationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_location_type: Option<String>,
    #[serde(rename = "unitsAvailableForConsumption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units_available_for_consumption: Option<i64>,
}

/// `CollectibleItemIds`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CollectibleItemIds {
    #[serde(rename = "itemIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_ids: Option<Vec<String>>,
}

/// `CollectiblePurchaseRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CollectiblePurchaseRequest {
    #[serde(rename = "collectibleItemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collectible_item_id: Option<String>,
    /// Only for purchase-resale
    #[serde(rename = "collectibleItemInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collectible_item_instance_id: Option<String>,
    #[serde(rename = "collectibleProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collectible_product_id: Option<String>,
    /// 1 is Robux
    #[serde(rename = "expectedCurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_currency: Option<i32>,
    #[serde(rename = "expectedPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_price: Option<i64>,
    #[serde(rename = "expectedPurchaserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_purchaser_id: Option<i64>,
    /// User
    #[serde(rename = "expectedPurchaserType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_purchaser_type: Option<String>,
    #[serde(rename = "expectedSellerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_seller_id: Option<i64>,
    /// User or Group
    #[serde(rename = "expectedSellerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_seller_type: Option<String>,
    /// A new UUID per purchase attempt
    #[serde(rename = "idempotencyKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
}

/// `CollectiblePurchaseResult`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CollectiblePurchaseResult {
    /// InsufficientBalance, ...
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "expirationTimeInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time_in_minutes: Option<i64>,
    /// Set when purchases are restricted for a while
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// Bought; it reaches the inventory a little later
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<bool>,
    /// Flooded, ...
    #[serde(rename = "purchaseResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchased: Option<bool>,
    /// TwoStepVerificationRequired, ...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// `EventHost`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EventHost {
    #[serde(rename = "hasVerifiedBadge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_verified_badge: Option<bool>,
    #[serde(rename = "hostId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_id: Option<i64>,
    #[serde(rename = "hostName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[serde(rename = "hostType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_type: Option<String>,
}

/// `EventThumbnail`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EventThumbnail {
    #[serde(rename = "mediaId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
}

/// `EventTime`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EventTime {
    #[serde(rename = "endUtc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_utc: Option<String>,
    #[serde(rename = "startUtc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_utc: Option<String>,
}

/// `VirtualEvent`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct VirtualEvent {
    #[serde(rename = "displayDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[serde(rename = "displaySubtitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_subtitle: Option<String>,
    #[serde(rename = "displayTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_title: Option<String>,
    #[serde(rename = "eventStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_status: Option<String>,
    #[serde(rename = "eventTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<EventTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<EventHost>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "placeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnails: Option<Vec<EventThumbnail>>,
    #[serde(rename = "universeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_id: Option<i64>,
    /// none, going, maybeGoing, notGoing
    #[serde(rename = "userRsvpStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_rsvp_status: Option<String>,
}

/// `EventsPage`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EventsPage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<VirtualEvent>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
}

/// `ExploreFilterOption`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ExploreFilterOption {
    #[serde(rename = "optionDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_display_name: Option<String>,
    #[serde(rename = "optionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_id: Option<String>,
}

/// `ExploreFilter`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ExploreFilter {
    #[serde(rename = "filterDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_display_name: Option<String>,
    #[serde(rename = "filterOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_options: Option<Vec<ExploreFilterOption>>,
    /// device, country
    #[serde(rename = "filterType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_type: Option<String>,
    #[serde(rename = "selectedOptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_option_id: Option<String>,
}

/// `GameCard`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GameCard {
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "creatorHasVerifiedBadge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_has_verified_badge: Option<bool>,
    #[serde(rename = "creatorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<i64>,
    #[serde(rename = "creatorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_name: Option<String>,
    #[serde(rename = "genreL1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre_l1: Option<String>,
    #[serde(rename = "isSponsored")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_sponsored: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "playerCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_count: Option<i64>,
    #[serde(rename = "rootPlaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_place_id: Option<i64>,
    #[serde(rename = "totalDownVotes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_down_votes: Option<i64>,
    #[serde(rename = "totalUpVotes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_up_votes: Option<i64>,
    #[serde(rename = "universeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_id: Option<i64>,
}

/// `TopicLayoutData`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TopicLayoutData {
    #[serde(rename = "infoText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info_text: Option<String>,
}

/// `ExploreSort`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ExploreSort {
    /// Games, Filters, ...
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ExploreFilter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub games: Option<Vec<GameCard>>,
    #[serde(rename = "sortDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_display_name: Option<String>,
    #[serde(rename = "sortId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(rename = "topicLayoutData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_layout_data: Option<TopicLayoutData>,
    #[serde(rename = "treatmentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_type: Option<String>,
}

/// `ExploreSorts`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ExploreSorts {
    #[serde(rename = "nextSortsPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_sorts_page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sorts: Option<Vec<ExploreSort>>,
}

/// `GamePassCreator`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GamePassCreator {
    #[serde(rename = "creatorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<i64>,
    #[serde(rename = "creatorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `GamePass`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GamePass {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<GamePassCreator>,
    #[serde(rename = "displayDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[serde(rename = "displayIconImageAssetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_icon_image_asset_id: Option<i64>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "isForSale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_for_sale: Option<bool>,
    #[serde(rename = "isOwned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_owned: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

/// `GamePassProductCreator`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GamePassProductCreator {
    #[serde(rename = "CreatorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_type: Option<String>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `GamePassProductInfo`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GamePassProductInfo {
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "Creator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<GamePassProductCreator>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IsForSale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_for_sale: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PriceInRobux")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_in_robux: Option<i64>,
    #[serde(rename = "ProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i64>,
    #[serde(rename = "Sales")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales: Option<i64>,
    #[serde(rename = "TargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<i64>,
    #[serde(rename = "Updated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

/// `GamePassPurchaseRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GamePassPurchaseRequest {
    #[serde(rename = "expectedPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_price: Option<i64>,
    /// A new UUID per purchase attempt
    #[serde(rename = "idempotencyKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
}

/// `GamePassPurchaseResult`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GamePassPurchaseResult {
    #[serde(rename = "expirationTimeInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time_in_minutes: Option<i64>,
    /// Set when purchases are restricted for a while
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchased: Option<bool>,
    /// TwoStepVerificationRequired, ...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// `GamePassesPage`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GamePassesPage {
    #[serde(rename = "gamePasses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_passes: Option<Vec<GamePass>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

/// `GroupRevenueSummary`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupRevenueSummary {
    #[serde(rename = "commissionRobux")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commission_robux: Option<i64>,
    #[serde(rename = "creatorRewardsPayoutRobux")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_rewards_payout_robux: Option<i64>,
    #[serde(rename = "groupPayoutRobux")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_payout_robux: Option<i64>,
    #[serde(rename = "individualToGroupRobux")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual_to_group_robux: Option<i64>,
    #[serde(rename = "itemSaleRobux")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_sale_robux: Option<i64>,
    #[serde(rename = "pendingRobux")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_robux: Option<i64>,
}

/// `OmniGroup`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct OmniGroup {
    /// Game, ...
    #[serde(rename = "contentGroupType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_group_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<Vec<GameCard>>,
}

/// `OmniItem`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct OmniItem {
    #[serde(rename = "contentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_id: Option<i64>,
    /// Game, ...
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

/// `OmniMetadata`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct OmniMetadata {
    #[serde(rename = "Game")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game: Option<HashMap<String, GameCard>>,
}

/// `OmniTopic`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct OmniTopic {
    #[serde(rename = "recommendationList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_list: Option<Vec<OmniItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(rename = "topicId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<i64>,
    #[serde(rename = "treatmentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_type: Option<String>,
}

/// `OmniRecommendation`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct OmniRecommendation {
    #[serde(rename = "contentMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_metadata: Option<OmniMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sorts: Option<Vec<OmniTopic>>,
}

/// `OmniRecommendationRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct OmniRecommendationRequest {
    /// Home
    #[serde(rename = "pageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_type: Option<String>,
    #[serde(rename = "sessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

/// `OmniSearch`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct OmniSearch {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "searchResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_results: Option<Vec<OmniGroup>>,
}

/// `PlaceUniverse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PlaceUniverse {
    #[serde(rename = "universeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_id: Option<i64>,
}

/// `QuickLoginCancel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct QuickLoginCancel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

/// `QuickLoginCode`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct QuickLoginCode {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "expirationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(rename = "privateKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// `QuickLoginDevice`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct QuickLoginDevice {
    #[serde(rename = "deviceInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_info: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

/// `QuickLoginKey`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct QuickLoginKey {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "privateKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
}

/// `QuickLoginStatus`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct QuickLoginStatus {
    #[serde(rename = "accountName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[serde(rename = "expirationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// `ResalePoint`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ResalePoint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

/// `ResaleData`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ResaleData {
    #[serde(rename = "priceDataPoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data_points: Option<Vec<ResalePoint>>,
    #[serde(rename = "recentAveragePrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recent_average_price: Option<f64>,
    #[serde(rename = "volumeDataPoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_data_points: Option<Vec<ResalePoint>>,
}

/// `ResellerSeller`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ResellerSeller {
    #[serde(rename = "hasVerifiedBadge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_verified_badge: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "sellerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_id: Option<i64>,
    #[serde(rename = "sellerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_type: Option<String>,
}

/// `Reseller`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Reseller {
    #[serde(rename = "collectibleItemInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collectible_item_instance_id: Option<String>,
    #[serde(rename = "collectibleProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collectible_product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller: Option<ResellerSeller>,
    #[serde(rename = "serialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<i64>,
}

/// `ResellersPage`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ResellersPage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<Reseller>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
}

/// `RsvpRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RsvpRequest {
    /// going or notGoing
    #[serde(rename = "rsvpStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rsvp_status: Option<String>,
}

/// `RsvpResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RsvpResponse {
    #[serde(rename = "rsvpStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rsvp_status: Option<String>,
}

/// `SessionAgent`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SessionAgent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// App, Browser, ...
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// `SessionLocation`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SessionLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdivision: Option<String>,
}

/// `SessionInfo`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SessionInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<SessionAgent>,
    #[serde(rename = "isCurrentSession")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_current_session: Option<bool>,
    /// Epoch ms as a string
    #[serde(rename = "lastAccessedTimestampEpochMilliseconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_accessed_timestamp_epoch_milliseconds: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<SessionLocation>,
}

/// `SessionList`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SessionList {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sessions: Option<Vec<SessionInfo>>,
}

/// `TransactionAgent`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TransactionAgent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// `TransactionCurrency`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TransactionCurrency {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// `TransactionPlace`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TransactionPlace {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "placeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_id: Option<i64>,
    #[serde(rename = "universeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_id: Option<i64>,
}

/// `TransactionDetails`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TransactionDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place: Option<TransactionPlace>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// `TransactionRecord`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TransactionRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<TransactionAgent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<TransactionCurrency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<TransactionDetails>,
    #[serde(rename = "idHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_hash: Option<String>,
    #[serde(rename = "isPending")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_pending: Option<bool>,
    #[serde(rename = "transactionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<String>,
}

/// `TransactionTotals`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TransactionTotals {
    #[serde(rename = "adSpendTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_spend_total: Option<i64>,
    #[serde(rename = "affiliateSalesTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliate_sales_total: Option<i64>,
    #[serde(rename = "creatorRewardsPayoutsTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_rewards_payouts_total: Option<i64>,
    #[serde(rename = "csAdjustmentTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cs_adjustment_total: Option<i64>,
    #[serde(rename = "currencyPurchasesTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_purchases_total: Option<i64>,
    #[serde(rename = "developerExchangeTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub developer_exchange_total: Option<i64>,
    #[serde(rename = "groupPayoutsTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_payouts_total: Option<i64>,
    #[serde(rename = "groupPremiumPayoutsTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_premium_payouts_total: Option<i64>,
    #[serde(rename = "incomingRobuxTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incoming_robux_total: Option<i64>,
    #[serde(rename = "incomingRobuxTransferTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incoming_robux_transfer_total: Option<i64>,
    #[serde(rename = "individualToGroupTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual_to_group_total: Option<i64>,
    #[serde(rename = "outgoingRobuxTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outgoing_robux_total: Option<i64>,
    #[serde(rename = "outgoingRobuxTransferTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outgoing_robux_transfer_total: Option<i64>,
    #[serde(rename = "pendingRobuxTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_robux_total: Option<i64>,
    #[serde(rename = "premiumPayoutsTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_payouts_total: Option<i64>,
    #[serde(rename = "premiumStipendsTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_stipends_total: Option<i64>,
    #[serde(rename = "purchasesTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchases_total: Option<i64>,
    #[serde(rename = "salesTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_total: Option<i64>,
    #[serde(rename = "tradeSystemCostsTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_system_costs_total: Option<i64>,
    #[serde(rename = "tradeSystemEarningsTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_system_earnings_total: Option<i64>,
}

/// `TransactionTypeFlags`
pub type TransactionTypeFlags = HashMap<String, bool>;

/// `TransactionsPage`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TransactionsPage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<TransactionRecord>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
}

/// `UserGamePass`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UserGamePass {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<GamePassCreator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "gamePassId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_pass_id: Option<i64>,
    #[serde(rename = "iconAssetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_asset_id: Option<i64>,
    #[serde(rename = "isForSale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_for_sale: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
}

/// `UserGamePassesPage`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UserGamePassesPage {
    #[serde(rename = "gamePasses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_passes: Option<Vec<UserGamePass>>,
}

/// `UserSettingOptionValue`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UserSettingOptionValue {
    #[serde(rename = "optionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_type: Option<String>,
    #[serde(rename = "optionValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_value: Option<String>,
}

/// `UserSettingOption`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UserSettingOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option: Option<UserSettingOptionValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirement: Option<String>,
}

/// `UserSettingAndOptions`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UserSettingAndOptions {
    #[serde(rename = "currentValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_value: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<UserSettingOption>>,
}

/// `UserSettingsAndOptions`
pub type UserSettingsAndOptions = HashMap<String, UserSettingAndOptions>;

/// `UserSettingsUpdate`
pub type UserSettingsUpdate = HashMap<String, String>;

/// `VerifiedAge`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct VerifiedAge {
    #[serde(rename = "isSeventeenPlus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_seventeen_plus: Option<bool>,
    #[serde(rename = "isVerified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_verified: Option<bool>,
    #[serde(rename = "verifiedAge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_age: Option<i32>,
}

/// Query, header and form inputs for [`get_explore_sort_content`].
#[derive(Debug, Clone, Default)]
pub struct GetExploreSortContentParams {
    pub session_id: String,
    pub sort_id: String,
    pub device: Option<String>,
    pub country: Option<String>,
}

/// Query, header and form inputs for [`get_explore_sorts`].
#[derive(Debug, Clone, Default)]
pub struct GetExploreSortsParams {
    /// Random UUID per app session.
    pub session_id: String,
    /// computer, phone, tablet, console, vr
    pub device: Option<String>,
    /// Two-letter code or all
    pub country: Option<String>,
    pub sorts_page_token: Option<String>,
}

/// Query, header and form inputs for [`get_universe_game_passes`].
#[derive(Debug, Clone, Default)]
pub struct GetUniverseGamePassesParams {
    /// Full
    pub pass_view: Option<String>,
    pub page_size: Option<i32>,
    pub page_token: Option<String>,
}

/// Query, header and form inputs for [`get_user_game_passes`].
#[derive(Debug, Clone, Default)]
pub struct GetUserGamePassesParams {
    pub count: Option<i32>,
    pub exclusive_start_id: Option<i64>,
}

/// Query, header and form inputs for [`get_item_resellers`].
#[derive(Debug, Clone, Default)]
pub struct GetItemResellersParams {
    pub limit: Option<i32>,
    pub cursor: Option<String>,
}

/// Query, header and form inputs for [`get_chat_messages`].
#[derive(Debug, Clone, Default)]
pub struct GetChatMessagesParams {
    pub conversation_id: String,
    pub cursor: Option<String>,
    pub page_size: Option<i32>,
}

/// Query, header and form inputs for [`get_chat_conversations`].
#[derive(Debug, Clone, Default)]
pub struct GetChatConversationsParams {
    pub include_user_data: Option<bool>,
    pub cursor: Option<String>,
    pub page_size: Option<i32>,
}

/// Query, header and form inputs for [`get_omni_search`].
#[derive(Debug, Clone, Default)]
pub struct GetOmniSearchParams {
    pub search_query: String,
    /// Random UUID per app session.
    pub session_id: String,
    /// all or games
    pub page_type: Option<String>,
    pub page_token: Option<String>,
}

/// Query, header and form inputs for [`get_group_transactions`].
#[derive(Debug, Clone, Default)]
pub struct GetGroupTransactionsParams {
    pub transaction_type: String,
    pub limit: Option<i32>,
    pub cursor: Option<String>,
}

/// Query, header and form inputs for [`get_user_transaction_totals`].
#[derive(Debug, Clone, Default)]
pub struct GetUserTransactionTotalsParams {
    /// Day, Week, Month or Year
    pub time_frame: String,
    /// Always summary
    pub transaction_type: String,
}

/// Query, header and form inputs for [`get_user_transactions`].
#[derive(Debug, Clone, Default)]
pub struct GetUserTransactionsParams {
    /// Purchase, Sale, PremiumStipend, GroupPayout, CurrencyPurchase, TradeRobux, ...
    pub transaction_type: String,
    /// 10, 25, 50 or 100
    pub limit: Option<i32>,
    pub cursor: Option<String>,
    /// PaidAndLimited or All (purchases)
    pub item_pricing_type: Option<String>,
}

/// Query, header and form inputs for [`get_blocked_users`].
#[derive(Debug, Clone, Default)]
pub struct GetBlockedUsersParams {
    pub cursor: Option<String>,
    /// 1 to 49
    pub count: i32,
}

/// Query, header and form inputs for [`get_universe_events`].
#[derive(Debug, Clone, Default)]
pub struct GetUniverseEventsParams {
    /// ISO date-time
    pub from_utc: Option<String>,
    pub limit: Option<i32>,
    pub cursor: Option<String>,
}

/// `GET /age-verification-service/v1/age-verification/verified-age`
///
/// Whether the account passed age verification.
///
/// Undocumented; captured 2026-10-16, www.roblox.com/my/account#!/info.
pub fn get_verified_age_request() -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        "/age-verification-service/v1/age-verification/verified-age".to_string(),
    );
    Ok(r)
}

/// `GET /age-verification-service/v1/age-verification/verified-age`
///
/// Whether the account passed age verification.
///
/// Undocumented; captured 2026-10-16, www.roblox.com/my/account#!/info.
pub async fn get_verified_age(client: &Client) -> Result<VerifiedAge> {
    client.send(get_verified_age_request()?).await?.json()
}

/// `POST /auth-token-service/v1/login/cancel`
///
/// Cancel a quick sign-in code.
///
/// Undocumented; captured 2026-10-14, www.roblox.com/login, Quick Log In.
pub fn post_quick_login_cancel_request(body: &QuickLoginCancel) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        "/auth-token-service/v1/login/cancel".to_string(),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /auth-token-service/v1/login/cancel`
///
/// Cancel a quick sign-in code.
///
/// Undocumented; captured 2026-10-14, www.roblox.com/login, Quick Log In.
pub async fn post_quick_login_cancel(client: &Client, body: &QuickLoginCancel) -> Result<serde_json::Value> {
    client.send(post_quick_login_cancel_request(body)?).await?.json()
}

/// `POST /auth-token-service/v1/login/create`
///
/// Start a quick sign-in. Returns a code to approve on a signed-in device.
///
/// Undocumented; captured 2026-10-14, www.roblox.com/login, Quick Log In.
pub fn post_quick_login_create_request() -> Result<Request> {
    let r = Request::new(
        Method::POST,
        HOST,
        "/auth-token-service/v1/login/create".to_string(),
    );
    Ok(r)
}

/// `POST /auth-token-service/v1/login/create`
///
/// Start a quick sign-in. Returns a code to approve on a signed-in device.
///
/// Undocumented; captured 2026-10-14, www.roblox.com/login, Quick Log In.
pub async fn post_quick_login_create(client: &Client) -> Result<QuickLoginCode> {
    client.send(post_quick_login_create_request()?).await?.json()
}

/// `POST /auth-token-service/v1/login/enterCode`
///
/// Link a quick sign-in code to the signed-in account and see the device asking.
///
/// Undocumented; captured 2026-10-16, www.roblox.com/crossdevicelogin/ConfirmCode.
pub fn post_quick_login_enter_code_request(body: &QuickLoginCancel) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        "/auth-token-service/v1/login/enterCode".to_string(),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /auth-token-service/v1/login/enterCode`
///
/// Link a quick sign-in code to the signed-in account and see the device asking.
///
/// Undocumented; captured 2026-10-16, www.roblox.com/crossdevicelogin/ConfirmCode.
pub async fn post_quick_login_enter_code(
    client: &Client,
    body: &QuickLoginCancel,
) -> Result<QuickLoginDevice> {
    client
        .send(post_quick_login_enter_code_request(body)?)
        .await?
        .json()
}

/// `POST /auth-token-service/v1/login/status`
///
/// Where a quick sign-in code is.
///
/// Undocumented; captured 2026-10-14, www.roblox.com/login, Quick Log In.
pub fn post_quick_login_status_request(body: &QuickLoginKey) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        "/auth-token-service/v1/login/status".to_string(),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /auth-token-service/v1/login/status`
///
/// Where a quick sign-in code is.
///
/// Undocumented; captured 2026-10-14, www.roblox.com/login, Quick Log In.
pub async fn post_quick_login_status(client: &Client, body: &QuickLoginKey) -> Result<QuickLoginStatus> {
    client.send(post_quick_login_status_request(body)?).await?.json()
}

/// `POST /auth-token-service/v1/login/validateCode`
///
/// Approve a quick sign-in code entered with enterCode.
///
/// Undocumented; captured 2026-10-16, www.roblox.com/crossdevicelogin/ConfirmCode.
pub fn post_quick_login_validate_code_request(body: &QuickLoginCancel) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        "/auth-token-service/v1/login/validateCode".to_string(),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /auth-token-service/v1/login/validateCode`
///
/// Approve a quick sign-in code entered with enterCode.
///
/// Undocumented; captured 2026-10-16, www.roblox.com/crossdevicelogin/ConfirmCode.
pub async fn post_quick_login_validate_code(
    client: &Client,
    body: &QuickLoginCancel,
) -> Result<serde_json::Value> {
    client
        .send(post_quick_login_validate_code_request(body)?)
        .await?
        .json()
}

/// `GET /captcha/v1/metadata`
///
/// Arkose public keys per captcha action.
///
/// Undocumented; captured 2026-10-15, www.roblox.com Captcha.js.
pub fn get_captcha_metadata_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/captcha/v1/metadata".to_string());
    Ok(r)
}

/// `GET /captcha/v1/metadata`
///
/// Arkose public keys per captcha action.
///
/// Undocumented; captured 2026-10-15, www.roblox.com Captcha.js.
pub async fn get_captcha_metadata(client: &Client) -> Result<CaptchaMetadata> {
    client.send(get_captcha_metadata_request()?).await?.json()
}

/// `POST /challenge/v1/continue`
///
/// Tell Roblox a challenge was passed.
///
/// Undocumented; captured 2026-10-15, www.roblox.com Challenge.js (generic challenge continue).
pub fn post_challenge_continue_request(body: &ChallengeContinueRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/challenge/v1/continue".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /challenge/v1/continue`
///
/// Tell Roblox a challenge was passed.
///
/// Undocumented; captured 2026-10-15, www.roblox.com Challenge.js (generic challenge continue).
pub async fn post_challenge_continue(
    client: &Client,
    body: &ChallengeContinueRequest,
) -> Result<ChallengeContinueResponse> {
    client.send(post_challenge_continue_request(body)?).await?.json()
}

/// `POST /discovery-api/omni-recommendation`
///
/// Home page topics and their games.
///
/// Undocumented; captured 2026-10-12, www.roblox.com/home.
pub fn post_omni_recommendation_request(body: &OmniRecommendationRequest) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        "/discovery-api/omni-recommendation".to_string(),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /discovery-api/omni-recommendation`
///
/// Home page topics and their games.
///
/// Undocumented; captured 2026-10-12, www.roblox.com/home.
pub async fn post_omni_recommendation(
    client: &Client,
    body: &OmniRecommendationRequest,
) -> Result<OmniRecommendation> {
    client.send(post_omni_recommendation_request(body)?).await?.json()
}

/// `POST /experience-guidelines-api/experience-guidelines/get-age-recommendation`
///
/// Content maturity label and descriptors for an experience.
///
/// Undocumented; captured 2026-10-12, www.roblox.com/games/920587237.
pub fn post_age_recommendation_request(body: &AgeRecommendationRequest) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        "/experience-guidelines-api/experience-guidelines/get-age-recommendation".to_string(),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /experience-guidelines-api/experience-guidelines/get-age-recommendation`
///
/// Content maturity label and descriptors for an experience.
///
/// Undocumented; captured 2026-10-12, www.roblox.com/games/920587237.
pub async fn post_age_recommendation(
    client: &Client,
    body: &AgeRecommendationRequest,
) -> Result<AgeRecommendationResponse> {
    client.send(post_age_recommendation_request(body)?).await?.json()
}

/// `GET /explore-api/v1/get-sort-content`
///
/// All games in one chart sort.
///
/// Undocumented; captured 2026-10-12, www.roblox.com/charts/top-trending.
pub fn get_explore_sort_content_request(params: &GetExploreSortContentParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/explore-api/v1/get-sort-content".to_string());
    r.query("sessionId", &params.session_id);
    r.query("sortId", &params.sort_id);
    r.query("device", &params.device);
    r.query("country", &params.country);
    Ok(r)
}

/// `GET /explore-api/v1/get-sort-content`
///
/// All games in one chart sort.
///
/// Undocumented; captured 2026-10-12, www.roblox.com/charts/top-trending.
pub async fn get_explore_sort_content(
    client: &Client,
    params: &GetExploreSortContentParams,
) -> Result<ExploreSort> {
    client
        .send(get_explore_sort_content_request(params)?)
        .await?
        .json()
}

/// `GET /explore-api/v1/get-sorts`
///
/// Game sorts (Top Trending, Up-and-Coming, ...) with their first games.
///
/// Undocumented; captured 2026-10-12, www.roblox.com/charts.
pub fn get_explore_sorts_request(params: &GetExploreSortsParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/explore-api/v1/get-sorts".to_string());
    r.query("sessionId", &params.session_id);
    r.query("device", &params.device);
    r.query("country", &params.country);
    r.query("sortsPageToken", &params.sorts_page_token);
    Ok(r)
}

/// `GET /explore-api/v1/get-sorts`
///
/// Game sorts (Top Trending, Up-and-Coming, ...) with their first games.
///
/// Undocumented; captured 2026-10-12, www.roblox.com/charts.
pub async fn get_explore_sorts(client: &Client, params: &GetExploreSortsParams) -> Result<ExploreSorts> {
    client.send(get_explore_sorts_request(params)?).await?.json()
}

/// `GET /game-passes/v1/game-passes/{gamePassId}/product-info`
///
/// A pass's name, description, creator, price and dates.
///
/// Undocumented; captured 2026-10-15, www.roblox.com/game-pass/{id}.
pub fn get_game_pass_product_info_request(game_pass_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/game-passes/v1/game-passes/{}/product-info",
            crate::rt::path_value(&game_pass_id)
        ),
    );
    Ok(r)
}

/// `GET /game-passes/v1/game-passes/{gamePassId}/product-info`
///
/// A pass's name, description, creator, price and dates.
///
/// Undocumented; captured 2026-10-15, www.roblox.com/game-pass/{id}.
pub async fn get_game_pass_product_info(client: &Client, game_pass_id: i64) -> Result<GamePassProductInfo> {
    client
        .send(get_game_pass_product_info_request(game_pass_id)?)
        .await?
        .json()
}

/// `POST /game-passes/v1/game-passes/{productId}/purchase`
///
/// Buy a game pass.
///
/// Undocumented; captured 2026-10-14, www.roblox.com/game-pass/{id} Buy (ItemPurchase bundle).
pub fn post_purchase_game_pass_request(product_id: i64, body: &GamePassPurchaseRequest) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/game-passes/v1/game-passes/{}/purchase",
            crate::rt::path_value(&product_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /game-passes/v1/game-passes/{productId}/purchase`
///
/// Buy a game pass.
///
/// Undocumented; captured 2026-10-14, www.roblox.com/game-pass/{id} Buy (ItemPurchase bundle).
pub async fn post_purchase_game_pass(
    client: &Client,
    product_id: i64,
    body: &GamePassPurchaseRequest,
) -> Result<GamePassPurchaseResult> {
    client
        .send(post_purchase_game_pass_request(product_id, body)?)
        .await?
        .json()
}

/// `GET /game-passes/v1/universes/{universeId}/game-passes`
///
/// Passes sold in an experience. passView=Full adds price, isOwned and creator.
///
/// Undocumented; captured 2026-10-12, www.roblox.com/games/920587237 Store tab.
pub fn get_universe_game_passes_request(
    universe_id: i64,
    params: &GetUniverseGamePassesParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/game-passes/v1/universes/{}/game-passes",
            crate::rt::path_value(&universe_id)
        ),
    );
    r.query("passView", &params.pass_view);
    r.query("pageSize", &params.page_size);
    r.query("pageToken", &params.page_token);
    Ok(r)
}

/// `GET /game-passes/v1/universes/{universeId}/game-passes`
///
/// Passes sold in an experience. passView=Full adds price, isOwned and creator.
///
/// Undocumented; captured 2026-10-12, www.roblox.com/games/920587237 Store tab.
pub async fn get_universe_game_passes(
    client: &Client,
    universe_id: i64,
    params: &GetUniverseGamePassesParams,
) -> Result<GamePassesPage> {
    client
        .send(get_universe_game_passes_request(universe_id, params)?)
        .await?
        .json()
}

/// `GET /game-passes/v1/users/{userId}/game-passes`
///
/// Passes a user owns, newest first.
///
/// Undocumented; captured 2026-10-14, www.roblox.com/users/{id}/inventory Passes.
pub fn get_user_game_passes_request(user_id: i64, params: &GetUserGamePassesParams) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/game-passes/v1/users/{}/game-passes",
            crate::rt::path_value(&user_id)
        ),
    );
    r.query("count", &params.count);
    r.query("exclusiveStartId", &params.exclusive_start_id);
    Ok(r)
}

/// `GET /game-passes/v1/users/{userId}/game-passes`
///
/// Passes a user owns, newest first.
///
/// Undocumented; captured 2026-10-14, www.roblox.com/users/{id}/inventory Passes.
pub async fn get_user_game_passes(
    client: &Client,
    user_id: i64,
    params: &GetUserGamePassesParams,
) -> Result<UserGamePassesPage> {
    client
        .send(get_user_game_passes_request(user_id, params)?)
        .await?
        .json()
}

/// `POST /marketplace-items/v1/items/details`
///
/// Sale details of collectible items by collectible item id.
///
/// Undocumented; captured 2026-10-14, www.roblox.com/catalog/1365767.
pub fn post_collectible_item_details_request(body: &CollectibleItemIds) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        "/marketplace-items/v1/items/details".to_string(),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /marketplace-items/v1/items/details`
///
/// Sale details of collectible items by collectible item id.
///
/// Undocumented; captured 2026-10-14, www.roblox.com/catalog/1365767.
pub async fn post_collectible_item_details(
    client: &Client,
    body: &CollectibleItemIds,
) -> Result<Vec<CollectibleItemDetails>> {
    client
        .send(post_collectible_item_details_request(body)?)
        .await?
        .json()
}

/// `POST /marketplace-sales/v1/item/{collectibleItemId}/purchase-item`
///
/// Buy a catalog item or bundle from its creator.
///
/// Undocumented; captured 2026-10-14, www.roblox.com/catalog/{id} Buy (ItemPurchase bundle).
pub fn post_purchase_item_request(
    collectible_item_id: &str,
    body: &CollectiblePurchaseRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/marketplace-sales/v1/item/{}/purchase-item",
            crate::rt::path_value(&collectible_item_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /marketplace-sales/v1/item/{collectibleItemId}/purchase-item`
///
/// Buy a catalog item or bundle from its creator.
///
/// Undocumented; captured 2026-10-14, www.roblox.com/catalog/{id} Buy (ItemPurchase bundle).
pub async fn post_purchase_item(
    client: &Client,
    collectible_item_id: &str,
    body: &CollectiblePurchaseRequest,
) -> Result<CollectiblePurchaseResult> {
    client
        .send(post_purchase_item_request(collectible_item_id, body)?)
        .await?
        .json()
}

/// `POST /marketplace-sales/v1/item/{collectibleItemId}/purchase-resale`
///
/// Buy one reseller's copy of a limited or collectible.
///
/// Undocumented; captured 2026-10-14, www.roblox.com/catalog/{id} Resellers Buy (ItemPurchase bundle).
pub fn post_purchase_resale_request(
    collectible_item_id: &str,
    body: &CollectiblePurchaseRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/marketplace-sales/v1/item/{}/purchase-resale",
            crate::rt::path_value(&collectible_item_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /marketplace-sales/v1/item/{collectibleItemId}/purchase-resale`
///
/// Buy one reseller's copy of a limited or collectible.
///
/// Undocumented; captured 2026-10-14, www.roblox.com/catalog/{id} Resellers Buy (ItemPurchase bundle).
pub async fn post_purchase_resale(
    client: &Client,
    collectible_item_id: &str,
    body: &CollectiblePurchaseRequest,
) -> Result<CollectiblePurchaseResult> {
    client
        .send(post_purchase_resale_request(collectible_item_id, body)?)
        .await?
        .json()
}

/// `GET /marketplace-sales/v1/item/{collectibleItemId}/resale-data`
///
/// Daily average resale price and volume, newest first, plus recent average price.
///
/// Undocumented; captured 2026-10-13, www.roblox.com/catalog/1365767.
pub fn get_item_resale_data_request(collectible_item_id: &str) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/marketplace-sales/v1/item/{}/resale-data",
            crate::rt::path_value(&collectible_item_id)
        ),
    );
    Ok(r)
}

/// `GET /marketplace-sales/v1/item/{collectibleItemId}/resale-data`
///
/// Daily average resale price and volume, newest first, plus recent average price.
///
/// Undocumented; captured 2026-10-13, www.roblox.com/catalog/1365767.
pub async fn get_item_resale_data(client: &Client, collectible_item_id: &str) -> Result<ResaleData> {
    client
        .send(get_item_resale_data_request(collectible_item_id)?)
        .await?
        .json()
}

/// `GET /marketplace-sales/v1/item/{collectibleItemId}/resellers`
///
/// Resale listings of a collectible item, cheapest first.
///
/// Undocumented; captured 2026-10-13, www.roblox.com/catalog/1365767.
pub fn get_item_resellers_request(
    collectible_item_id: &str,
    params: &GetItemResellersParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/marketplace-sales/v1/item/{}/resellers",
            crate::rt::path_value(&collectible_item_id)
        ),
    );
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    Ok(r)
}

/// `GET /marketplace-sales/v1/item/{collectibleItemId}/resellers`
///
/// Resale listings of a collectible item, cheapest first.
///
/// Undocumented; captured 2026-10-13, www.roblox.com/catalog/1365767.
pub async fn get_item_resellers(
    client: &Client,
    collectible_item_id: &str,
    params: &GetItemResellersParams,
) -> Result<ResellersPage> {
    client
        .send(get_item_resellers_request(collectible_item_id, params)?)
        .await?
        .json()
}

/// `POST /platform-chat-api/v1/add-users`
///
/// Add people to a group conversation.
///
/// Undocumented; captured 2026-10-13, chat panel on www.roblox.com (ReactChat bundle).
pub fn post_chat_add_users_request(body: &ChatUsersRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/platform-chat-api/v1/add-users".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /platform-chat-api/v1/add-users`
///
/// Add people to a group conversation.
///
/// Undocumented; captured 2026-10-13, chat panel on www.roblox.com (ReactChat bundle).
pub async fn post_chat_add_users(client: &Client, body: &ChatUsersRequest) -> Result<ChatStatus> {
    client.send(post_chat_add_users_request(body)?).await?.json()
}

/// `POST /platform-chat-api/v1/create-conversations`
///
/// Start a one-to-one or group conversation.
///
/// Undocumented; captured 2026-10-13, chat panel on www.roblox.com (ReactChat bundle).
pub fn post_chat_create_request(body: &ChatCreateRequest) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        "/platform-chat-api/v1/create-conversations".to_string(),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /platform-chat-api/v1/create-conversations`
///
/// Start a one-to-one or group conversation.
///
/// Undocumented; captured 2026-10-13, chat panel on www.roblox.com (ReactChat bundle).
pub async fn post_chat_create(client: &Client, body: &ChatCreateRequest) -> Result<ChatConversations> {
    client.send(post_chat_create_request(body)?).await?.json()
}

/// `GET /platform-chat-api/v1/get-conversation-messages`
///
/// Messages in a conversation, newest first.
///
/// Undocumented; captured 2026-10-13, chat panel on www.roblox.com (ReactChat bundle).
pub fn get_chat_messages_request(params: &GetChatMessagesParams) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        "/platform-chat-api/v1/get-conversation-messages".to_string(),
    );
    r.query("conversation_id", &params.conversation_id);
    r.query("cursor", &params.cursor);
    r.query("pageSize", &params.page_size);
    Ok(r)
}

/// `GET /platform-chat-api/v1/get-conversation-messages`
///
/// Messages in a conversation, newest first.
///
/// Undocumented; captured 2026-10-13, chat panel on www.roblox.com (ReactChat bundle).
pub async fn get_chat_messages(client: &Client, params: &GetChatMessagesParams) -> Result<ChatMessagesPage> {
    client.send(get_chat_messages_request(params)?).await?.json()
}

/// `GET /platform-chat-api/v1/get-conversation-metadata`
///
/// Unread chat counts.
///
/// Undocumented; captured 2026-10-13, chat panel on www.roblox.com (ReactChat bundle).
pub fn get_chat_unread_request() -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        "/platform-chat-api/v1/get-conversation-metadata".to_string(),
    );
    Ok(r)
}

/// `GET /platform-chat-api/v1/get-conversation-metadata`
///
/// Unread chat counts.
///
/// Undocumented; captured 2026-10-13, chat panel on www.roblox.com (ReactChat bundle).
pub async fn get_chat_unread(client: &Client) -> Result<ChatUnread> {
    client.send(get_chat_unread_request()?).await?.json()
}

/// `GET /platform-chat-api/v1/get-user-conversations`
///
/// Your chat conversations, newest first.
///
/// Undocumented; captured 2026-10-13, chat panel on www.roblox.com (ReactChat bundle).
pub fn get_chat_conversations_request(params: &GetChatConversationsParams) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        "/platform-chat-api/v1/get-user-conversations".to_string(),
    );
    r.query("include_user_data", &params.include_user_data);
    r.query("cursor", &params.cursor);
    r.query("pageSize", &params.page_size);
    Ok(r)
}

/// `GET /platform-chat-api/v1/get-user-conversations`
///
/// Your chat conversations, newest first.
///
/// Undocumented; captured 2026-10-13, chat panel on www.roblox.com (ReactChat bundle).
pub async fn get_chat_conversations(
    client: &Client,
    params: &GetChatConversationsParams,
) -> Result<ChatConversationsPage> {
    client.send(get_chat_conversations_request(params)?).await?.json()
}

/// `POST /platform-chat-api/v1/mark-conversations`
///
/// Mark conversations read.
///
/// Undocumented; captured 2026-10-13, chat panel on www.roblox.com (ReactChat bundle).
pub fn post_chat_mark_read_request(body: &ChatMarkRequest) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        "/platform-chat-api/v1/mark-conversations".to_string(),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /platform-chat-api/v1/mark-conversations`
///
/// Mark conversations read.
///
/// Undocumented; captured 2026-10-13, chat panel on www.roblox.com (ReactChat bundle).
pub async fn post_chat_mark_read(client: &Client, body: &ChatMarkRequest) -> Result<ChatStatus> {
    client.send(post_chat_mark_read_request(body)?).await?.json()
}

/// `GET /platform-chat-api/v1/metadata`
///
/// Whether chat is on for you, and its limits.
///
/// Undocumented; captured 2026-10-13, chat panel on www.roblox.com (ReactChat bundle).
pub fn get_chat_settings_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/platform-chat-api/v1/metadata".to_string());
    Ok(r)
}

/// `GET /platform-chat-api/v1/metadata`
///
/// Whether chat is on for you, and its limits.
///
/// Undocumented; captured 2026-10-13, chat panel on www.roblox.com (ReactChat bundle).
pub async fn get_chat_settings(client: &Client) -> Result<ChatSettings> {
    client.send(get_chat_settings_request()?).await?.json()
}

/// `POST /platform-chat-api/v1/remove-users`
///
/// Remove people from a group conversation (yourself to leave).
///
/// Undocumented; captured 2026-10-13, chat panel on www.roblox.com (ReactChat bundle).
pub fn post_chat_remove_users_request(body: &ChatUsersRequest) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        "/platform-chat-api/v1/remove-users".to_string(),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /platform-chat-api/v1/remove-users`
///
/// Remove people from a group conversation (yourself to leave).
///
/// Undocumented; captured 2026-10-13, chat panel on www.roblox.com (ReactChat bundle).
pub async fn post_chat_remove_users(client: &Client, body: &ChatUsersRequest) -> Result<ChatStatus> {
    client.send(post_chat_remove_users_request(body)?).await?.json()
}

/// `POST /platform-chat-api/v1/send-messages`
///
/// Send messages to a conversation.
///
/// Undocumented; captured 2026-10-13, chat panel on www.roblox.com (ReactChat bundle).
pub fn post_chat_send_request(body: &ChatSendRequest) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        "/platform-chat-api/v1/send-messages".to_string(),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /platform-chat-api/v1/send-messages`
///
/// Send messages to a conversation.
///
/// Undocumented; captured 2026-10-13, chat panel on www.roblox.com (ReactChat bundle).
pub async fn post_chat_send(client: &Client, body: &ChatSendRequest) -> Result<ChatSendResponse> {
    client.send(post_chat_send_request(body)?).await?.json()
}

/// `POST /platform-chat-api/v1/update-conversations`
///
/// Rename group conversations.
///
/// Undocumented; captured 2026-10-13, chat panel on www.roblox.com (ReactChat bundle).
pub fn post_chat_update_request(body: &ChatUpdateRequest) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        "/platform-chat-api/v1/update-conversations".to_string(),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /platform-chat-api/v1/update-conversations`
///
/// Rename group conversations.
///
/// Undocumented; captured 2026-10-13, chat panel on www.roblox.com (ReactChat bundle).
pub async fn post_chat_update(client: &Client, body: &ChatUpdateRequest) -> Result<ChatConversations> {
    client.send(post_chat_update_request(body)?).await?.json()
}

/// `POST /platform-chat-api/v1/update-typing-status`
///
/// Tell the others you are typing.
///
/// Undocumented; captured 2026-10-13, chat panel on www.roblox.com (ReactChat bundle).
pub fn post_chat_typing_request(body: &ChatConversationRef) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        "/platform-chat-api/v1/update-typing-status".to_string(),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /platform-chat-api/v1/update-typing-status`
///
/// Tell the others you are typing.
///
/// Undocumented; captured 2026-10-13, chat panel on www.roblox.com (ReactChat bundle).
pub async fn post_chat_typing(client: &Client, body: &ChatConversationRef) -> Result<ChatStatus> {
    client.send(post_chat_typing_request(body)?).await?.json()
}

/// `GET /search-api/omni-search`
///
/// Search games by keyword.
///
/// Undocumented; captured 2026-10-12, www.roblox.com/discover?Keyword=adopt.
pub fn get_omni_search_request(params: &GetOmniSearchParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/search-api/omni-search".to_string());
    r.query("searchQuery", &params.search_query);
    r.query("sessionId", &params.session_id);
    r.query("pageType", &params.page_type);
    r.query("pageToken", &params.page_token);
    Ok(r)
}

/// `GET /search-api/omni-search`
///
/// Search games by keyword.
///
/// Undocumented; captured 2026-10-12, www.roblox.com/discover?Keyword=adopt.
pub async fn get_omni_search(client: &Client, params: &GetOmniSearchParams) -> Result<OmniSearch> {
    client.send(get_omni_search_request(params)?).await?.json()
}

/// `GET /token-metadata-service/v1/sessions`
///
/// Signed-in sessions of the account.
///
/// Undocumented; captured 2026-10-16, www.roblox.com/my/account#!/security.
pub fn get_sessions_request() -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        "/token-metadata-service/v1/sessions".to_string(),
    );
    Ok(r)
}

/// `GET /token-metadata-service/v1/sessions`
///
/// Signed-in sessions of the account.
///
/// Undocumented; captured 2026-10-16, www.roblox.com/my/account#!/security.
pub async fn get_sessions(client: &Client) -> Result<SessionList> {
    client.send(get_sessions_request()?).await?.json()
}

/// `GET /transaction-records/v1/groups/{groupId}/revenue/summary/{timeFrame}`
///
/// A community's Robux in per category over the last day, week, month or year.
///
/// Undocumented; captured 2026-10-15, www.roblox.com/communities/configure?id={id}#!/revenue/summary.
pub fn get_group_revenue_summary_request(group_id: i64, time_frame: &str) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/transaction-records/v1/groups/{}/revenue/summary/{}",
            crate::rt::path_value(&group_id),
            crate::rt::path_value(&time_frame)
        ),
    );
    Ok(r)
}

/// `GET /transaction-records/v1/groups/{groupId}/revenue/summary/{timeFrame}`
///
/// A community's Robux in per category over the last day, week, month or year.
///
/// Undocumented; captured 2026-10-15, www.roblox.com/communities/configure?id={id}#!/revenue/summary.
pub async fn get_group_revenue_summary(
    client: &Client,
    group_id: i64,
    time_frame: &str,
) -> Result<GroupRevenueSummary> {
    client
        .send(get_group_revenue_summary_request(group_id, time_frame)?)
        .await?
        .json()
}

/// `GET /transaction-records/v1/groups/{groupId}/transactions`
///
/// One kind of transaction for a community, newest first.
///
/// Undocumented; captured 2026-10-15, www.roblox.com/communities/configure?id={id}#!/revenue/sales.
pub fn get_group_transactions_request(group_id: i64, params: &GetGroupTransactionsParams) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/transaction-records/v1/groups/{}/transactions",
            crate::rt::path_value(&group_id)
        ),
    );
    r.query("transactionType", &params.transaction_type);
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    Ok(r)
}

/// `GET /transaction-records/v1/groups/{groupId}/transactions`
///
/// One kind of transaction for a community, newest first.
///
/// Undocumented; captured 2026-10-15, www.roblox.com/communities/configure?id={id}#!/revenue/sales.
pub async fn get_group_transactions(
    client: &Client,
    group_id: i64,
    params: &GetGroupTransactionsParams,
) -> Result<TransactionsPage> {
    client
        .send(get_group_transactions_request(group_id, params)?)
        .await?
        .json()
}

/// `GET /transaction-records/v1/users/{userId}/transaction-totals`
///
/// Robux in and out per category over the last day, week, month or year.
///
/// Undocumented; captured 2026-10-15, www.roblox.com/transactions, Summary.
pub fn get_user_transaction_totals_request(
    user_id: i64,
    params: &GetUserTransactionTotalsParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/transaction-records/v1/users/{}/transaction-totals",
            crate::rt::path_value(&user_id)
        ),
    );
    r.query("timeFrame", &params.time_frame);
    r.query("transactionType", &params.transaction_type);
    Ok(r)
}

/// `GET /transaction-records/v1/users/{userId}/transaction-totals`
///
/// Robux in and out per category over the last day, week, month or year.
///
/// Undocumented; captured 2026-10-15, www.roblox.com/transactions, Summary.
pub async fn get_user_transaction_totals(
    client: &Client,
    user_id: i64,
    params: &GetUserTransactionTotalsParams,
) -> Result<TransactionTotals> {
    client
        .send(get_user_transaction_totals_request(user_id, params)?)
        .await?
        .json()
}

/// `GET /transaction-records/v1/users/{userId}/transaction-types`
///
/// Which transaction types the account has, as HasPurchase, HasSale, ... flags.
///
/// Undocumented; captured 2026-10-15, www.roblox.com/transactions.
pub fn get_user_transaction_types_request(user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/transaction-records/v1/users/{}/transaction-types",
            crate::rt::path_value(&user_id)
        ),
    );
    Ok(r)
}

/// `GET /transaction-records/v1/users/{userId}/transaction-types`
///
/// Which transaction types the account has, as HasPurchase, HasSale, ... flags.
///
/// Undocumented; captured 2026-10-15, www.roblox.com/transactions.
pub async fn get_user_transaction_types(client: &Client, user_id: i64) -> Result<TransactionTypeFlags> {
    client
        .send(get_user_transaction_types_request(user_id)?)
        .await?
        .json()
}

/// `GET /transaction-records/v1/users/{userId}/transactions`
///
/// One kind of transaction for a user, newest first.
///
/// Undocumented; captured 2026-10-15, www.roblox.com/transactions, type dropdown.
pub fn get_user_transactions_request(user_id: i64, params: &GetUserTransactionsParams) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/transaction-records/v1/users/{}/transactions",
            crate::rt::path_value(&user_id)
        ),
    );
    r.query("transactionType", &params.transaction_type);
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    r.query("itemPricingType", &params.item_pricing_type);
    Ok(r)
}

/// `GET /transaction-records/v1/users/{userId}/transactions`
///
/// One kind of transaction for a user, newest first.
///
/// Undocumented; captured 2026-10-15, www.roblox.com/transactions, type dropdown.
pub async fn get_user_transactions(
    client: &Client,
    user_id: i64,
    params: &GetUserTransactionsParams,
) -> Result<TransactionsPage> {
    client
        .send(get_user_transactions_request(user_id, params)?)
        .await?
        .json()
}

/// `GET /universes/v1/places/{placeId}/universe`
///
/// The universe (experience) a place belongs to.
///
/// Undocumented; captured 2026-10-12, experience page on www.roblox.com/games/1818.
pub fn get_place_universe_request(place_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/universes/v1/places/{}/universe",
            crate::rt::path_value(&place_id)
        ),
    );
    Ok(r)
}

/// `GET /universes/v1/places/{placeId}/universe`
///
/// The universe (experience) a place belongs to.
///
/// Undocumented; captured 2026-10-12, experience page on www.roblox.com/games/1818.
pub async fn get_place_universe(client: &Client, place_id: i64) -> Result<PlaceUniverse> {
    client.send(get_place_universe_request(place_id)?).await?.json()
}

/// `GET /user-blocking-api/v1/users/get-blocked-users`
///
/// Users the signed-in account has blocked.
///
/// Undocumented; captured 2026-10-12, www.roblox.com/my/account#!/privacy.
pub fn get_blocked_users_request(params: &GetBlockedUsersParams) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        "/user-blocking-api/v1/users/get-blocked-users".to_string(),
    );
    r.query("cursor", &params.cursor);
    r.query("count", &params.count);
    Ok(r)
}

/// `GET /user-blocking-api/v1/users/get-blocked-users`
///
/// Users the signed-in account has blocked.
///
/// Undocumented; captured 2026-10-12, www.roblox.com/my/account#!/privacy.
pub async fn get_blocked_users(
    client: &Client,
    params: &GetBlockedUsersParams,
) -> Result<BlockedUsersResponse> {
    client.send(get_blocked_users_request(params)?).await?.json()
}

/// `POST /user-blocking-api/v1/users/{userId}/block-user`
///
/// Block a user. The body is a bare string like "Successfully blocked user ...".
///
/// Undocumented; captured 2026-10-12, www.roblox.com/users/{id}/profile.
pub fn post_block_user_request(user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/user-blocking-api/v1/users/{}/block-user",
            crate::rt::path_value(&user_id)
        ),
    );
    Ok(r)
}

/// `POST /user-blocking-api/v1/users/{userId}/block-user`
///
/// Block a user. The body is a bare string like "Successfully blocked user ...".
///
/// Undocumented; captured 2026-10-12, www.roblox.com/users/{id}/profile.
pub async fn post_block_user(client: &Client, user_id: i64) -> Result<String> {
    client.send(post_block_user_request(user_id)?).await?.json()
}

/// `GET /user-blocking-api/v1/users/{userId}/is-blocked`
///
/// Whether the signed-in account blocked this user. The body is a bare boolean.
///
/// Undocumented; captured 2026-10-12, www.roblox.com/users/{id}/profile.
pub fn get_is_blocked_request(user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/user-blocking-api/v1/users/{}/is-blocked",
            crate::rt::path_value(&user_id)
        ),
    );
    Ok(r)
}

/// `GET /user-blocking-api/v1/users/{userId}/is-blocked`
///
/// Whether the signed-in account blocked this user. The body is a bare boolean.
///
/// Undocumented; captured 2026-10-12, www.roblox.com/users/{id}/profile.
pub async fn get_is_blocked(client: &Client, user_id: i64) -> Result<bool> {
    client.send(get_is_blocked_request(user_id)?).await?.json()
}

/// `POST /user-blocking-api/v1/users/{userId}/unblock-user`
///
/// Unblock a user. The body is a bare string like "Successfully unblocked user ...".
///
/// Undocumented; captured 2026-10-12, www.roblox.com/my/account#!/privacy.
pub fn post_unblock_user_request(user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/user-blocking-api/v1/users/{}/unblock-user",
            crate::rt::path_value(&user_id)
        ),
    );
    Ok(r)
}

/// `POST /user-blocking-api/v1/users/{userId}/unblock-user`
///
/// Unblock a user. The body is a bare string like "Successfully unblocked user ...".
///
/// Undocumented; captured 2026-10-12, www.roblox.com/my/account#!/privacy.
pub async fn post_unblock_user(client: &Client, user_id: i64) -> Result<String> {
    client.send(post_unblock_user_request(user_id)?).await?.json()
}

/// `POST /user-settings-api/v1/user-settings`
///
/// Change one privacy setting (user action only).
///
/// Undocumented; captured 2026-10-14, www.roblox.com/my/account#!/privacy.
pub fn post_user_settings_request(body: &UserSettingsUpdate) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        "/user-settings-api/v1/user-settings".to_string(),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /user-settings-api/v1/user-settings`
///
/// Change one privacy setting (user action only).
///
/// Undocumented; captured 2026-10-14, www.roblox.com/my/account#!/privacy.
pub async fn post_user_settings(client: &Client, body: &UserSettingsUpdate) -> Result<serde_json::Value> {
    client.send(post_user_settings_request(body)?).await?.json()
}

/// `GET /user-settings-api/v1/user-settings/settings-and-options`
///
/// Privacy and safety settings, each with its value and allowed options.
///
/// Undocumented; captured 2026-10-14, www.roblox.com/my/account#!/privacy.
pub fn get_user_settings_and_options_request() -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        "/user-settings-api/v1/user-settings/settings-and-options".to_string(),
    );
    Ok(r)
}

/// `GET /user-settings-api/v1/user-settings/settings-and-options`
///
/// Privacy and safety settings, each with its value and allowed options.
///
/// Undocumented; captured 2026-10-14, www.roblox.com/my/account#!/privacy.
pub async fn get_user_settings_and_options(client: &Client) -> Result<UserSettingsAndOptions> {
    client
        .send(get_user_settings_and_options_request()?)
        .await?
        .json()
}

/// `GET /virtual-events/v1/universes/{universeId}/virtual-events`
///
/// Events in an experience.
///
/// Undocumented; captured 2026-10-12, www.roblox.com/games/920587237 Events.
pub fn get_universe_events_request(universe_id: i64, params: &GetUniverseEventsParams) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/virtual-events/v1/universes/{}/virtual-events",
            crate::rt::path_value(&universe_id)
        ),
    );
    r.query("fromUtc", &params.from_utc);
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    Ok(r)
}

/// `GET /virtual-events/v1/universes/{universeId}/virtual-events`
///
/// Events in an experience.
///
/// Undocumented; captured 2026-10-12, www.roblox.com/games/920587237 Events.
pub async fn get_universe_events(
    client: &Client,
    universe_id: i64,
    params: &GetUniverseEventsParams,
) -> Result<EventsPage> {
    client
        .send(get_universe_events_request(universe_id, params)?)
        .await?
        .json()
}

/// `POST /virtual-events/v1/virtual-events/{eventId}/rsvps`
///
/// RSVP to an event.
///
/// Undocumented; captured 2026-10-12, www.roblox.com/events/{id}.
pub fn post_event_rsvp_request(event_id: &str, body: &RsvpRequest) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/virtual-events/v1/virtual-events/{}/rsvps",
            crate::rt::path_value(&event_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /virtual-events/v1/virtual-events/{eventId}/rsvps`
///
/// RSVP to an event.
///
/// Undocumented; captured 2026-10-12, www.roblox.com/events/{id}.
pub async fn post_event_rsvp(client: &Client, event_id: &str, body: &RsvpRequest) -> Result<RsvpResponse> {
    client
        .send(post_event_rsvp_request(event_id, body)?)
        .await?
        .json()
}

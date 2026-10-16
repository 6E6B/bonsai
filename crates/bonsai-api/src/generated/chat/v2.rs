//! `chat.roblox.com` v2 — generated from `vendor/roblox-api-tracker/chat/v2.json`.
//!
//! Do not edit. Change the spec or `bonsai-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "chat.roblox.com";

crate::api_enum! {
    str
    /// Type of a chat participant \['User' = 0, 'System' = 1\]
    RejectedChatParticipantType {
    User = 0 => "User",
    System = 1 => "System",
    }
}

/// `Roblox.Chat.Api.Models.RejectedChatParticipant`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RejectedChatParticipant {
    /// display name of the chat participant
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Verified badge status of the participant.
    #[serde(rename = "hasVerifiedBadge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_verified_badge: Option<bool>,
    /// Name of the chat participant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Reason as to why the chat participant was rejected to be a part of the conversation
    #[serde(rename = "rejectedReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_reason: Option<String>,
    /// Id of the chat participant
    #[serde(rename = "targetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<i64>,
    /// Type of a chat participant \['User' = 0, 'System' = 1\]
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<RejectedChatParticipantType>,
}

crate::api_enum! {
    str
    /// Status of the response \['Success' = 0\]
    AddUserToConversationResponseResultType {
    Success = 0 => "Success",
    }
}

/// `Roblox.Chat.Api.Models.AddUserToConversationResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AddUserToConversationResponse {
    /// Conversation Id to add a new user to
    #[serde(rename = "conversationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<i64>,
    /// List of rejected chat participants
    #[serde(rename = "rejectedParticipants")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_participants: Option<Vec<RejectedChatParticipant>>,
    /// Status of the response \['Success' = 0\]
    #[serde(rename = "resultType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_type: Option<AddUserToConversationResponseResultType>,
    /// Status message to be sent to the requestor
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

/// `Roblox.Chat.Api.Models.AddUsersToConversationRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AddUsersToConversationRequest {
    /// The conversation id
    #[serde(rename = "conversationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<i64>,
    /// Array of user ids to be added to the conversation.
    #[serde(rename = "participantUserIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_user_ids: Option<Vec<i64>>,
}

/// `Roblox.Chat.Api.Models.SetConversationUniverseEventBased`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SetConversationUniverseEventBased {
    /// The UserId for the user who setConversationUniverse
    #[serde(rename = "actorUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_user_id: Option<i64>,
    /// The universeId for SetConversationUniverse EventBased ChatMessage
    #[serde(rename = "universeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_id: Option<i64>,
}

crate::api_enum! {
    str
    /// The chat message event type
    EventBasedType {
    SetConversationUniverse = 0 => "SetConversationUniverse",
    }
}

/// `Roblox.Chat.Api.Models.EventBased`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EventBased {
    #[serde(rename = "setConversationUniverse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_conversation_universe: Option<SetConversationUniverseEventBased>,
    /// The chat message event type
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<EventBasedType>,
}

/// `Roblox.Chat.Api.Models.GameLink`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GameLink {
    /// The universeId for game link chat message
    #[serde(rename = "universeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_id: Option<i64>,
}

crate::api_enum! {
    str
    /// The chat message link type
    LinkType {
    Game = 0 => "Game",
    }
}

/// `Roblox.Chat.Api.Models.Link`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Link {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game: Option<GameLink>,
    /// The chat message link type
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<LinkType>,
}

crate::api_enum! {
    str
    /// Type of the chat message \['PlainText' = 0, 'Link' = 1, 'EventBased' = 2\]
    ChatMessageMessageType {
    PlainText = 0 => "PlainText",
    Link = 1 => "Link",
    EventBased = 2 => "EventBased",
    }
}

/// `Roblox.Chat.Api.Models.ChatMessage`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChatMessage {
    /// The chat mesage itself
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Decorators for the chat message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decorators: Option<Vec<String>>,
    #[serde(rename = "eventBased")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_based: Option<EventBased>,
    /// Id of the chat message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
    /// Type of the chat message \['PlainText' = 0, 'Link' = 1, 'EventBased' = 2\]
    #[serde(rename = "messageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<ChatMessageMessageType>,
    /// Is the chat message read
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
    /// Id of the entity that has sent the chat message
    #[serde(rename = "senderTargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_target_id: Option<i64>,
    /// Type of the chat message sender \['User' = 0, 'System' = 1\]
    #[serde(rename = "senderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_type: Option<RejectedChatParticipantType>,
    /// The dateTime when the chat message was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent: Option<String>,
}

crate::api_enum! {
    int
    /// \['Disabled' = 0, 'Enabled' = 1, 'Unavailable' = 2\]
    ChatMetadataResponseIsChatEnabledByPrivacySetting {
    Disabled = 0 => "Disabled",
    Enabled = 1 => "Enabled",
    Unavailable = 2 => "Unavailable",
    }
}

/// `Roblox.Chat.Api.Models.ChatMetadataResponse`
///
/// Chat Metadata Response Model
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChatMetadataResponse {
    #[serde(rename = "cachedDataFromLocalStorageExpirationMS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_data_from_local_storage_expiration_ms: Option<i32>,
    #[serde(rename = "chatDataFromLocalStorageExpirationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_data_from_local_storage_expiration_seconds: Option<i32>,
    #[serde(rename = "isAliasChatForClientSideEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_alias_chat_for_client_side_enabled: Option<bool>,
    #[serde(rename = "isChatDataFromLocalStorageEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_chat_data_from_local_storage_enabled: Option<bool>,
    /// \['Disabled' = 0, 'Enabled' = 1, 'Unavailable' = 2\]
    #[serde(rename = "isChatEnabledByPrivacySetting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_chat_enabled_by_privacy_setting: Option<ChatMetadataResponseIsChatEnabledByPrivacySetting>,
    #[serde(rename = "isInvalidMessageTypeFallbackEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_invalid_message_type_fallback_enabled: Option<bool>,
    #[serde(rename = "isPlatformChatApiEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_platform_chat_api_enabled: Option<bool>,
    #[serde(rename = "isPlayTogetherForGameCardsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_play_together_for_game_cards_enabled: Option<bool>,
    #[serde(rename = "isRespectingMessageTypeEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_respecting_message_type_enabled: Option<bool>,
    #[serde(rename = "isRoactChatEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_roact_chat_enabled: Option<bool>,
    #[serde(rename = "isUsingCacheToLoadFriendsInfoEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_using_cache_to_load_friends_info_enabled: Option<bool>,
    #[serde(rename = "languageForPrivacySettingUnavailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_for_privacy_setting_unavailable: Option<String>,
    #[serde(rename = "maxConversationTitleLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_conversation_title_length: Option<i32>,
    #[serde(rename = "numberOfMembersForPartyChrome")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_members_for_party_chrome: Option<i32>,
    #[serde(rename = "partyChromeDisplayTimeStampInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_chrome_display_time_stamp_interval: Option<i32>,
    #[serde(rename = "relativeValueToRecordUiPerformance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_value_to_record_ui_performance: Option<f64>,
    #[serde(rename = "senderTypesForUnknownMessageTypeError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_types_for_unknown_message_type_error: Option<Vec<String>>,
    #[serde(rename = "shouldRespectConversationHasUnreadMessageToMarkAsRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_respect_conversation_has_unread_message_to_mark_as_read: Option<bool>,
    #[serde(rename = "signalRDisconnectionResponseInMilliseconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_r_disconnection_response_in_milliseconds: Option<i32>,
    #[serde(rename = "typingInChatForReceiverExpirationMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typing_in_chat_for_receiver_expiration_ms: Option<i32>,
    #[serde(rename = "typingInChatFromSenderThrottleMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typing_in_chat_from_sender_throttle_ms: Option<i32>,
    #[serde(rename = "validMessageTypesWhiteList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_message_types_white_list: Option<Vec<String>>,
    #[serde(rename = "webChatEventSampleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_chat_event_sample_rate: Option<i32>,
}

/// `Roblox.Chat.Api.Models.ChatParticipant`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChatParticipant {
    /// display name of the chat participant
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Verified badge status of the participant.
    #[serde(rename = "hasVerifiedBadge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_verified_badge: Option<bool>,
    /// Name of the chat participant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Id of the chat participant
    #[serde(rename = "targetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<i64>,
    /// Type of a chat participant \['User' = 0, 'System' = 1\]
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<RejectedChatParticipantType>,
}

/// `Roblox.Chat.Api.Models.ChatSettingsResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChatSettingsResponse {
    /// Is chat enabled for the user
    #[serde(rename = "chatEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_enabled: Option<bool>,
    /// Was the Last ChatMessage Sent within the last x days or the account was created in the last x days? Note: user is active by default unless he does not chat for more than x days after account creation
    #[serde(rename = "isActiveChatUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active_chat_user: Option<bool>,
    /// Is Connect Tab Enabled for this user
    #[serde(rename = "isConnectTabEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_connect_tab_enabled: Option<bool>,
}

/// `Roblox.Chat.Api.Models.ConversationTitle`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ConversationTitle {
    /// Specifies if the title displayed for the user is generated as a default title in the backend or was custom assigned by the user.
    #[serde(rename = "isDefaultTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default_title: Option<bool>,
    /// Specifies the title for the conversation specific to the viewer
    #[serde(rename = "titleForViewer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_for_viewer: Option<String>,
}

crate::api_enum! {
    str
    /// Type of the conversation \['OneToOneConversation' = 0, 'MultiUserConversation' = 1, 'CloudEditConversation' = 2\]
    ConversationConversationType {
    OneToOneConversation = 0 => "OneToOneConversation",
    MultiUserConversation = 1 => "MultiUserConversation",
    CloudEditConversation = 2 => "CloudEditConversation",
    }
}

/// `Roblox.Chat.Api.Models.ConversationUniverse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ConversationUniverse {
    /// Id Corresponding to the root place of the universe
    #[serde(rename = "rootPlaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_place_id: Option<i64>,
    /// Id corresponding to the universe
    #[serde(rename = "universeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_id: Option<i64>,
}

/// `Roblox.Chat.Api.Models.Conversation`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Conversation {
    #[serde(rename = "conversationTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_title: Option<ConversationTitle>,
    /// Type of the conversation \['OneToOneConversation' = 0, 'MultiUserConversation' = 1, 'CloudEditConversation' = 2\]
    #[serde(rename = "conversationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_type: Option<ConversationConversationType>,
    #[serde(rename = "conversationUniverse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_universe: Option<ConversationUniverse>,
    /// Does the conversation have any unread messages
    #[serde(rename = "hasUnreadMessages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_unread_messages: Option<bool>,
    /// Chat conversation Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator: Option<ChatParticipant>,
    /// Specifies the datetime when the conversation was last updated
    #[serde(rename = "lastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    /// Participants involved in the conversation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participants: Option<Vec<ChatParticipant>>,
    /// Chat conversation title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// `Roblox.Chat.Api.Models.CreateCloudEditConversationRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CreateCloudEditConversationRequest {
    /// Place id to create a Cloud edit conversation.
    #[serde(rename = "placeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_id: Option<i64>,
}

/// `Roblox.Chat.Api.Models.CreateGroupConversationRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CreateGroupConversationRequest {
    /// Array of user id to create a group conversation with.
    #[serde(rename = "participantUserIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_user_ids: Option<Vec<i64>>,
    /// Title of the new group conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// `Roblox.Chat.Api.Models.CreateOneToOneConversationRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CreateOneToOneConversationRequest {
    /// Participant user id to create 1-1 conversation.
    #[serde(rename = "participantUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_user_id: Option<i64>,
}

/// `Roblox.Chat.Api.Models.MarkAsReadRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MarkAsReadRequest {
    /// the conversation id to be marked as read.
    #[serde(rename = "conversationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<i64>,
    /// The end message id string.
    #[serde(rename = "endMessageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_message_id: Option<String>,
}

/// `Roblox.Chat.Api.Models.MarkAsReadResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MarkAsReadResponse {
    /// Status of the response \['Success' = 0\]
    #[serde(rename = "resultType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_type: Option<AddUserToConversationResponseResultType>,
}

/// `Roblox.Chat.Api.Models.MarkAsSeenRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MarkAsSeenRequest {
    /// The conversation ids to be marked as seen.
    #[serde(rename = "conversationsToMarkSeen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversations_to_mark_seen: Option<Vec<i64>>,
}

/// `Roblox.Chat.Api.Models.MarkAsSeenResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MarkAsSeenResponse {
    /// Status of the response \['Success' = 0\]
    #[serde(rename = "resultType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_type: Option<AddUserToConversationResponseResultType>,
}

/// `Roblox.Chat.Api.Models.MultigetConversationMessagesResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MultigetConversationMessagesResponse {
    /// Chat messages associated with the conversation
    #[serde(rename = "chatMessages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_messages: Option<Vec<ChatMessage>>,
    /// Conversation Id to get multiple conversations
    #[serde(rename = "conversationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<i64>,
}

/// `Roblox.Chat.Api.Models.RemoveUserFromConversationRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RemoveUserFromConversationRequest {
    /// The conversation id.
    #[serde(rename = "conversationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<i64>,
    /// Participant user id to be removed from the conversation.
    #[serde(rename = "participantUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_user_id: Option<i64>,
}

/// `Roblox.Chat.Api.Models.RemoveUserFromConversationResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RemoveUserFromConversationResponse {
    /// Id of the conversation to remove a user from
    #[serde(rename = "conversationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<i64>,
    /// Status of the response \['Success' = 0\]
    #[serde(rename = "resultType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_type: Option<AddUserToConversationResponseResultType>,
    /// Status message to be sent to the requestor
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

crate::api_enum! {
    str
    /// Status of the response \['Success' = 0, 'Moderated' = 1, 'TextTooLong' = 2\]
    RenameConversationResponseResultType {
    Success = 0 => "Success",
    Moderated = 1 => "Moderated",
    TextTooLong = 2 => "TextTooLong",
    }
}

/// `Roblox.Chat.Api.Models.RenameConversationResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RenameConversationResponse {
    /// New conversation title after rename
    #[serde(rename = "conversationTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_title: Option<String>,
    /// Status of the response \['Success' = 0, 'Moderated' = 1, 'TextTooLong' = 2\]
    #[serde(rename = "resultType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_type: Option<RenameConversationResponseResultType>,
    /// Status message to be sent to the requestor
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<ConversationTitle>,
}

/// `Roblox.Chat.Api.Models.RenameGroupConversationRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RenameGroupConversationRequest {
    /// The conversation id to rename the conversation.
    #[serde(rename = "conversationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<i64>,
    /// New title for the conversation.
    #[serde(rename = "newTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_title: Option<String>,
}

crate::api_enum! {
    str
    /// Feature name strings for rollout settings \['LuaChat' = 0, 'ConversationUniverse' = 1, 'PlayTogether' = 2, 'Party' = 3, 'GameLink' = 4, 'OldPlayTogether' = 5\]
    RolloutSettingModelFeatureName {
    LuaChat = 0 => "LuaChat",
    ConversationUniverse = 1 => "ConversationUniverse",
    PlayTogether = 2 => "PlayTogether",
    Party = 3 => "Party",
    GameLink = 4 => "GameLink",
    OldPlayTogether = 5 => "OldPlayTogether",
    }
}

/// `Roblox.Chat.Api.Models.RolloutSettingModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RolloutSettingModel {
    /// Feature name strings for rollout settings \['LuaChat' = 0, 'ConversationUniverse' = 1, 'PlayTogether' = 2, 'Party' = 3, 'GameLink' = 4, 'OldPlayTogether' = 5\]
    #[serde(rename = "featureName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_name: Option<RolloutSettingModelFeatureName>,
    /// Is the feature enabled to be rolledout
    #[serde(rename = "isRolloutEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rollout_enabled: Option<bool>,
}

/// `Roblox.Chat.Api.Models.RolloutSettingsResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RolloutSettingsResponse {
    /// List of rollout features
    #[serde(rename = "rolloutFeatures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollout_features: Option<Vec<RolloutSettingModel>>,
}

/// `Roblox.Chat.Api.Models.SendGameLinkChatMessageRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SendGameLinkChatMessageRequest {
    /// The conversation Id to send mesage to.
    #[serde(rename = "conversationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<i64>,
    /// The decorators to attach to the new message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decorators: Option<Vec<String>>,
    /// Temporary field to denote requests from Experience Invite components
    #[serde(rename = "isExperienceInvite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_experience_invite: Option<bool>,
    /// Temporary field to store placeId for experience invite experiment
    #[serde(rename = "placeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_id: Option<i64>,
    /// The message to be sent.
    #[serde(rename = "universeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_id: Option<i64>,
    /// Temporary field to store userId for experience invite experiment
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}

crate::api_enum! {
    str
    /// Status of the response \['Success' = 0, 'Moderated' = 1, 'TextTooLong' = 2, 'NoRealtimeConnection' = 3\]
    SendLinkChatResponseResultType {
    Success = 0 => "Success",
    Moderated = 1 => "Moderated",
    TextTooLong = 2 => "TextTooLong",
    NoRealtimeConnection = 3 => "NoRealtimeConnection",
    }
}

/// `Roblox.Chat.Api.Models.SendLinkChatResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SendLinkChatResponse {
    /// \['Game' = 0\]
    #[serde(rename = "chatMessageLinkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_message_link_type: Option<LinkType>,
    /// Id of the chat message being sent
    #[serde(rename = "messageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// Type of the chat message being sent \['PlainText' = 0, 'Link' = 1, 'EventBased' = 2\]
    #[serde(rename = "messageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<ChatMessageMessageType>,
    /// Status of the response \['Success' = 0, 'Moderated' = 1, 'TextTooLong' = 2, 'NoRealtimeConnection' = 3\]
    #[serde(rename = "resultType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_type: Option<SendLinkChatResponseResultType>,
    /// DateTime of when the message was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent: Option<String>,
    /// Status message to be sent to the requestor
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

/// `Roblox.Chat.Api.Models.SendPlainTextChatMessageRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SendPlainTextChatMessageRequest {
    /// The conversation Id to send mesage to.
    #[serde(rename = "conversationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<i64>,
    /// The decorators to attach to the new message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decorators: Option<Vec<String>>,
    /// Temporary field to denote requests from Experience Invite components
    #[serde(rename = "isExperienceInvite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_experience_invite: Option<bool>,
    /// The message to be sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Temporary field to store userId for experience invite experiment
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}

/// `Roblox.Chat.Api.Models.SendPlainTextChatMessageResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SendPlainTextChatMessageResponse {
    /// The actual chat message that was being sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Was the chat message filtered based on the receivers of the message
    #[serde(rename = "filteredForReceivers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filtered_for_receivers: Option<bool>,
    /// Id of the chat message being sent
    #[serde(rename = "messageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// Type of the chat message being sent \['PlainText' = 0, 'Link' = 1, 'EventBased' = 2\]
    #[serde(rename = "messageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<ChatMessageMessageType>,
    /// Status of the response \['Success' = 0, 'Moderated' = 1, 'TextTooLong' = 2, 'NoRealtimeConnection' = 3\]
    #[serde(rename = "resultType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_type: Option<SendLinkChatResponseResultType>,
    /// DateTime of when the message was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent: Option<String>,
    /// Status message to be sent to the requestor
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

/// `Roblox.Chat.Api.Models.StartNewConversationResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct StartNewConversationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation: Option<Conversation>,
    /// List of chat participants who were rejected to be part of the associated conversation
    #[serde(rename = "rejectedParticipants")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_participants: Option<Vec<RejectedChatParticipant>>,
    /// Status of the response \['Success' = 0\]
    #[serde(rename = "resultType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_type: Option<AddUserToConversationResponseResultType>,
    /// Status message to be sent to the requestor
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

/// `Roblox.Chat.Api.Models.UnreadConversationCountResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UnreadConversationCountResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

/// `Roblox.Chat.Api.Models.UpdateUserTypingStatusRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateUserTypingStatusRequest {
    /// The Conversation id in which user is typing.
    #[serde(rename = "conversationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<i64>,
    /// isTyping flag to indicate if user has started typing or stopped.
    #[serde(rename = "isTyping")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_typing: Option<bool>,
}

/// `Roblox.Chat.Api.Models.UserVisibleStatusResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UserVisibleStatusResponse {
    /// Status message to be sent to the requestor
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

/// Query, header and form inputs for [`get_get_conversations`].
#[derive(Debug, Clone, Default)]
pub struct GetGetConversationsParams {
    /// Ids of the conversations to be returned
    pub conversation_ids: Vec<i64>,
}

/// Query, header and form inputs for [`get_get_messages`].
#[derive(Debug, Clone, Default)]
pub struct GetGetMessagesParams {
    pub conversation_id: i64,
    pub page_size: i32,
    pub exclusive_start_message_id: Option<String>,
}

/// Query, header and form inputs for [`get_get_rollout_settings`].
#[derive(Debug, Clone, Default)]
pub struct GetGetRolloutSettingsParams {
    /// string array to request feature rollout settings
    pub feature_names: Vec<String>,
}

/// Query, header and form inputs for [`get_get_unread_conversations`].
#[derive(Debug, Clone, Default)]
pub struct GetGetUnreadConversationsParams {
    pub page_number: i32,
    pub page_size: i32,
}

/// Query, header and form inputs for [`get_get_unread_messages`].
#[derive(Debug, Clone, Default)]
pub struct GetGetUnreadMessagesParams {
    pub conversation_ids: Vec<i64>,
    pub page_size: i32,
}

/// Query, header and form inputs for [`get_get_user_conversations`].
#[derive(Debug, Clone, Default)]
pub struct GetGetUserConversationsParams {
    pub page_number: i32,
    pub page_size: i32,
}

/// Query, header and form inputs for [`get_multi_get_latest_messages`].
#[derive(Debug, Clone, Default)]
pub struct GetMultiGetLatestMessagesParams {
    pub conversation_ids: Vec<i64>,
    pub page_size: i32,
}

/// `POST /v2/add-to-conversation`
///
/// Adds users to an existing conversation
pub fn post_add_to_conversation_request(body: &AddUsersToConversationRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v2/add-to-conversation".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/add-to-conversation`
///
/// Adds users to an existing conversation
pub async fn post_add_to_conversation(
    client: &Client,
    body: &AddUsersToConversationRequest,
) -> Result<AddUserToConversationResponse> {
    client.send(post_add_to_conversation_request(body)?).await?.json()
}

/// `GET /v2/chat-settings`
///
/// For every authenticated user, the clients hit this endpoint to get the chat related settings. Example : Chat is enabled or not
pub fn get_chat_settings_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v2/chat-settings".to_string());
    Ok(r)
}

/// `GET /v2/chat-settings`
///
/// For every authenticated user, the clients hit this endpoint to get the chat related settings. Example : Chat is enabled or not
pub async fn get_chat_settings(client: &Client) -> Result<ChatSettingsResponse> {
    client.send(get_chat_settings_request()?).await?.json()
}

/// `GET /v2/get-conversations`
///
/// Gets conversations for the conversationIds specified in the parameters
pub fn get_get_conversations_request(params: &GetGetConversationsParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v2/get-conversations".to_string());
    r.query("conversationIds", &params.conversation_ids);
    Ok(r)
}

/// `GET /v2/get-conversations`
///
/// Gets conversations for the conversationIds specified in the parameters
pub async fn get_get_conversations(
    client: &Client,
    params: &GetGetConversationsParams,
) -> Result<Vec<Conversation>> {
    client.send(get_get_conversations_request(params)?).await?.json()
}

/// `GET /v2/get-messages`
///
/// Gets messages corresponding to the given conversationId
pub fn get_get_messages_request(params: &GetGetMessagesParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v2/get-messages".to_string());
    r.query("conversationId", &params.conversation_id);
    r.query("pageSize", &params.page_size);
    r.query("exclusiveStartMessageId", &params.exclusive_start_message_id);
    Ok(r)
}

/// `GET /v2/get-messages`
///
/// Gets messages corresponding to the given conversationId
pub async fn get_get_messages(client: &Client, params: &GetGetMessagesParams) -> Result<Vec<ChatMessage>> {
    client.send(get_get_messages_request(params)?).await?.json()
}

/// `GET /v2/get-rollout-settings`
///
/// Gets the rollout settings for requested feature
pub fn get_get_rollout_settings_request(params: &GetGetRolloutSettingsParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v2/get-rollout-settings".to_string());
    r.query("featureNames", &params.feature_names);
    Ok(r)
}

/// `GET /v2/get-rollout-settings`
///
/// Gets the rollout settings for requested feature
pub async fn get_get_rollout_settings(
    client: &Client,
    params: &GetGetRolloutSettingsParams,
) -> Result<RolloutSettingsResponse> {
    client
        .send(get_get_rollout_settings_request(params)?)
        .await?
        .json()
}

/// `GET /v2/get-unread-conversation-count`
///
/// Gets the count of unread conversations
pub fn get_get_unread_conversation_count_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v2/get-unread-conversation-count".to_string());
    Ok(r)
}

/// `GET /v2/get-unread-conversation-count`
///
/// Gets the count of unread conversations
pub async fn get_get_unread_conversation_count(client: &Client) -> Result<UnreadConversationCountResponse> {
    client
        .send(get_get_unread_conversation_count_request()?)
        .await?
        .json()
}

/// `GET /v2/get-unread-conversations`
///
/// Gets a list of unread conversations
#[deprecated(note = "marked deprecated by Roblox")]
pub fn get_get_unread_conversations_request(params: &GetGetUnreadConversationsParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v2/get-unread-conversations".to_string());
    r.query("pageNumber", &params.page_number);
    r.query("pageSize", &params.page_size);
    Ok(r)
}

/// `GET /v2/get-unread-conversations`
///
/// Gets a list of unread conversations
#[deprecated(note = "marked deprecated by Roblox")]
pub async fn get_get_unread_conversations(
    client: &Client,
    params: &GetGetUnreadConversationsParams,
) -> Result<Vec<Conversation>> {
    client
        .send(get_get_unread_conversations_request(params)?)
        .await?
        .json()
}

/// `GET /v2/get-unread-messages`
///
/// Gets the unread messages for the conversationIds specified in the parameters
pub fn get_get_unread_messages_request(params: &GetGetUnreadMessagesParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v2/get-unread-messages".to_string());
    r.query("conversationIds", &params.conversation_ids);
    r.query("pageSize", &params.page_size);
    Ok(r)
}

/// `GET /v2/get-unread-messages`
///
/// Gets the unread messages for the conversationIds specified in the parameters
pub async fn get_get_unread_messages(
    client: &Client,
    params: &GetGetUnreadMessagesParams,
) -> Result<Vec<MultigetConversationMessagesResponse>> {
    client
        .send(get_get_unread_messages_request(params)?)
        .await?
        .json()
}

/// `GET /v2/get-user-conversations`
///
/// Gets all conversations associated with the logged-in User
pub fn get_get_user_conversations_request(params: &GetGetUserConversationsParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v2/get-user-conversations".to_string());
    r.query("pageNumber", &params.page_number);
    r.query("pageSize", &params.page_size);
    Ok(r)
}

/// `GET /v2/get-user-conversations`
///
/// Gets all conversations associated with the logged-in User
pub async fn get_get_user_conversations(
    client: &Client,
    params: &GetGetUserConversationsParams,
) -> Result<Vec<Conversation>> {
    client
        .send(get_get_user_conversations_request(params)?)
        .await?
        .json()
}

/// `POST /v2/mark-as-read`
///
/// Mark the specified conversation messages as read
pub fn post_mark_as_read_request(body: &MarkAsReadRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v2/mark-as-read".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/mark-as-read`
///
/// Mark the specified conversation messages as read
pub async fn post_mark_as_read(client: &Client, body: &MarkAsReadRequest) -> Result<MarkAsReadResponse> {
    client.send(post_mark_as_read_request(body)?).await?.json()
}

/// `POST /v2/mark-as-seen`
///
/// Mark the specified conversation messages as seen
pub fn post_mark_as_seen_request(body: &MarkAsSeenRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v2/mark-as-seen".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/mark-as-seen`
///
/// Mark the specified conversation messages as seen
pub async fn post_mark_as_seen(client: &Client, body: &MarkAsSeenRequest) -> Result<MarkAsSeenResponse> {
    client.send(post_mark_as_seen_request(body)?).await?.json()
}

/// `GET /v2/metadata`
///
/// Get Chat metadata.
pub fn get_metadata_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v2/metadata".to_string());
    Ok(r)
}

/// `GET /v2/metadata`
///
/// Get Chat metadata.
pub async fn get_metadata(client: &Client) -> Result<ChatMetadataResponse> {
    client.send(get_metadata_request()?).await?.json()
}

/// `GET /v2/multi-get-latest-messages`
///
/// Gets latest messages corresponding to the given list of conversation Ids
pub fn get_multi_get_latest_messages_request(params: &GetMultiGetLatestMessagesParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v2/multi-get-latest-messages".to_string());
    r.query("conversationIds", &params.conversation_ids);
    r.query("pageSize", &params.page_size);
    Ok(r)
}

/// `GET /v2/multi-get-latest-messages`
///
/// Gets latest messages corresponding to the given list of conversation Ids
pub async fn get_multi_get_latest_messages(
    client: &Client,
    params: &GetMultiGetLatestMessagesParams,
) -> Result<Vec<MultigetConversationMessagesResponse>> {
    client
        .send(get_multi_get_latest_messages_request(params)?)
        .await?
        .json()
}

/// `POST /v2/remove-from-conversation`
///
/// Removes a user from an existing conversation
pub fn post_remove_from_conversation_request(body: &RemoveUserFromConversationRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v2/remove-from-conversation".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/remove-from-conversation`
///
/// Removes a user from an existing conversation
pub async fn post_remove_from_conversation(
    client: &Client,
    body: &RemoveUserFromConversationRequest,
) -> Result<RemoveUserFromConversationResponse> {
    client
        .send(post_remove_from_conversation_request(body)?)
        .await?
        .json()
}

/// `POST /v2/rename-group-conversation`
///
/// Renames the title of an existing conversation
pub fn post_rename_group_conversation_request(body: &RenameGroupConversationRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v2/rename-group-conversation".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/rename-group-conversation`
///
/// Renames the title of an existing conversation
pub async fn post_rename_group_conversation(
    client: &Client,
    body: &RenameGroupConversationRequest,
) -> Result<RenameConversationResponse> {
    client
        .send(post_rename_group_conversation_request(body)?)
        .await?
        .json()
}

/// `POST /v2/send-game-link-message`
pub fn post_send_game_link_message_request(body: &SendGameLinkChatMessageRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v2/send-game-link-message".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/send-game-link-message`
pub async fn post_send_game_link_message(
    client: &Client,
    body: &SendGameLinkChatMessageRequest,
) -> Result<SendLinkChatResponse> {
    client
        .send(post_send_game_link_message_request(body)?)
        .await?
        .json()
}

/// `POST /v2/send-message`
///
/// Sends a chat message
pub fn post_send_message_request(body: &SendPlainTextChatMessageRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v2/send-message".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/send-message`
///
/// Sends a chat message
pub async fn post_send_message(
    client: &Client,
    body: &SendPlainTextChatMessageRequest,
) -> Result<SendPlainTextChatMessageResponse> {
    client.send(post_send_message_request(body)?).await?.json()
}

/// `POST /v2/start-cloud-edit-conversation`
///
/// Creates a new cloud edit conversation
pub fn post_start_cloud_edit_conversation_request(
    body: &CreateCloudEditConversationRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        "/v2/start-cloud-edit-conversation".to_string(),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/start-cloud-edit-conversation`
///
/// Creates a new cloud edit conversation
pub async fn post_start_cloud_edit_conversation(
    client: &Client,
    body: &CreateCloudEditConversationRequest,
) -> Result<StartNewConversationResponse> {
    client
        .send(post_start_cloud_edit_conversation_request(body)?)
        .await?
        .json()
}

/// `POST /v2/start-group-conversation`
///
/// Creates a new group conversation
pub fn post_start_group_conversation_request(body: &CreateGroupConversationRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v2/start-group-conversation".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/start-group-conversation`
///
/// Creates a new group conversation
pub async fn post_start_group_conversation(
    client: &Client,
    body: &CreateGroupConversationRequest,
) -> Result<StartNewConversationResponse> {
    client
        .send(post_start_group_conversation_request(body)?)
        .await?
        .json()
}

/// `POST /v2/start-one-to-one-conversation`
///
/// Creates a new one-on-one conversation
pub fn post_start_one_to_one_conversation_request(
    body: &CreateOneToOneConversationRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        "/v2/start-one-to-one-conversation".to_string(),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/start-one-to-one-conversation`
///
/// Creates a new one-on-one conversation
pub async fn post_start_one_to_one_conversation(
    client: &Client,
    body: &CreateOneToOneConversationRequest,
) -> Result<StartNewConversationResponse> {
    client
        .send(post_start_one_to_one_conversation_request(body)?)
        .await?
        .json()
}

/// `POST /v2/update-user-typing-status`
///
/// Updates the status for whether a user is typing currently
pub fn post_update_user_typing_status_request(body: &UpdateUserTypingStatusRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v2/update-user-typing-status".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/update-user-typing-status`
///
/// Updates the status for whether a user is typing currently
pub async fn post_update_user_typing_status(
    client: &Client,
    body: &UpdateUserTypingStatusRequest,
) -> Result<UserVisibleStatusResponse> {
    client
        .send(post_update_user_typing_status_request(body)?)
        .await?
        .json()
}

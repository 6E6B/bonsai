//! `gamejoin.roblox.com` v1 — generated from `vendor/roblox-api-tracker/gamejoin/v1.json`.
//!
//! Do not edit. Change the spec or `bonsai-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "gamejoin.roblox.com";

/// `Roblox.GameJoin.Api.GameJoinRequest`
///
/// Request model for game join.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GameJoinRequest {
    /// The integer representing browserTrackerId
    #[serde(rename = "browserTrackerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_tracker_id: Option<i64>,
    /// String representing channel name in RCC Channel 2.0. Note: Only internal users (email address ending in roblox.com and verified) can use this feature. For other users, this field should be null, otherwise the request will be rejected.
    #[serde(rename = "channelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    /// See "Points of Interest" project. A GUID that the developer has mapped to a special piece in their experience
    #[serde(rename = "eventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// The GameJoin Attempt Id for this game join request
    #[serde(rename = "gameJoinAttemptId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_join_attempt_id: Option<String>,
    /// The gamerTag, if any, for the game join request
    #[serde(rename = "gamerTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gamer_tag: Option<String>,
    /// Is this an Immersive Ads Teleport request.
    #[serde(rename = "isImmersiveAdsTeleport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_immersive_ads_teleport: Option<bool>,
    /// Boolean representing whether this is a PlayTogether game
    #[serde(rename = "isPlayTogetherGame")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_play_together_game: Option<bool>,
    /// Whether or not the game full join queue is used depends on the API and whether the join is a teleport. This optional override allows the Client to specify if it wants to queue when joining full games
    #[serde(rename = "isQueueAllowedOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_queue_allowed_override: Option<bool>,
    /// Is this a teleport request
    #[serde(rename = "isTeleport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_teleport: Option<bool>,
    /// Internal usage only field for some request context.
    #[serde(rename = "isoContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso_context: Option<String>,
    /// The JoinOrigin of a gamejoin request, as propagated from the game client.
    #[serde(rename = "joinOrigin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_origin: Option<String>,
    /// The ID of the player's current party, if any
    #[serde(rename = "partyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_id: Option<String>,
    /// The Place Id for the game join request
    #[serde(rename = "placeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_id: Option<i64>,
}

/// `Roblox.Web.GameLaunch.ConnectionFlow.ServerConnection`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ServerConnection {
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

/// `Roblox.Web.GameLaunch.ConnectionFlow.UdmuxEndpoint`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UdmuxEndpoint {
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

/// `Roblox.Web.GameLaunch.ConnectionFlow.JoinInformation`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct JoinInformation {
    #[serde(rename = "AccountAge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_age: Option<i32>,
    #[serde(rename = "AlternateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_name: Option<String>,
    #[serde(rename = "AnalyticsSessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_session_id: Option<String>,
    #[serde(rename = "BaseUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    #[serde(rename = "ChannelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    #[serde(rename = "CharacterAppearance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_appearance: Option<String>,
    #[serde(rename = "ChatStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_style: Option<String>,
    #[serde(rename = "ClientPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_port: Option<i32>,
    #[serde(rename = "ClientPublicKeyData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_public_key_data: Option<String>,
    #[serde(rename = "ClientTicket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ticket: Option<String>,
    #[serde(rename = "CookieStoreEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_store_enabled: Option<bool>,
    #[serde(rename = "CookieStoreFirstTimePlayKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_store_first_time_play_key: Option<String>,
    #[serde(rename = "CookieStoreFiveMinutePlayKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_store_five_minute_play_key: Option<String>,
    #[serde(rename = "CountryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(rename = "CreatorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<i64>,
    #[serde(rename = "CreatorTypeEnum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_type_enum: Option<String>,
    #[serde(rename = "DataCenterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_center_id: Option<i32>,
    #[serde(rename = "DirectServerReturn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_server_return: Option<bool>,
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "EphemeralEarlyPubKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_early_pub_key: Option<String>,
    #[serde(rename = "EventID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(rename = "FlexibleChatEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flexible_chat_enabled: Option<bool>,
    #[serde(rename = "FollowUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow_user_id: Option<i64>,
    #[serde(rename = "GameChatType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_chat_type: Option<String>,
    #[serde(rename = "GameId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_id: Option<String>,
    #[serde(rename = "GameLocale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_locale: Option<String>,
    #[serde(rename = "HasVerifiedBadge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_verified_badge: Option<bool>,
    #[serde(rename = "IsUnknownOrUnder13")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_unknown_or_under13: Option<bool>,
    #[serde(rename = "MachineAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine_address: Option<String>,
    #[serde(rename = "MembershipType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub membership_type: Option<String>,
    #[serde(rename = "PartyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_id: Option<String>,
    #[serde(rename = "PepperId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pepper_id: Option<i32>,
    #[serde(rename = "PingInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ping_interval: Option<i32>,
    #[serde(rename = "PingUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ping_url: Option<String>,
    #[serde(rename = "PlaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_id: Option<i64>,
    #[serde(rename = "PrivateServerID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_server_id: Option<String>,
    #[serde(rename = "PrivateServerOwnerID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_server_owner_id: Option<i64>,
    #[serde(rename = "RandomSeed1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_seed1: Option<String>,
    #[serde(rename = "RccVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rcc_version: Option<String>,
    #[serde(rename = "RobloxLocale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roblox_locale: Option<String>,
    #[serde(rename = "SeleniumTestMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selenium_test_mode: Option<bool>,
    #[serde(rename = "ServerConnections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_connections: Option<Vec<ServerConnection>>,
    #[serde(rename = "ServerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_port: Option<i32>,
    #[serde(rename = "SessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "SuperSafeChat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub super_safe_chat: Option<bool>,
    #[serde(rename = "TokenGenAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_gen_algorithm: Option<i32>,
    #[serde(rename = "TokenValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_value: Option<String>,
    #[serde(rename = "UdmuxEndpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udmux_endpoints: Option<Vec<UdmuxEndpoint>>,
    #[serde(rename = "UniverseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_id: Option<i64>,
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(rename = "VerifiedAMP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_amp: Option<i32>,
    #[serde(rename = "WhoCanWhisperChatWithMeInExperiences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub who_can_whisper_chat_with_me_in_experiences: Option<String>,
    #[serde(rename = "characterAppearanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_appearance_id: Option<i64>,
}

/// `Roblox.Web.GameJoin.StatusData.CreatorExperienceBanData`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CreatorExperienceBanData {
    #[serde(rename = "displayReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_reason: Option<String>,
    #[serde(rename = "displayReasonTextFilterStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_reason_text_filter_status: Option<i32>,
    #[serde(rename = "durationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i64>,
    #[serde(rename = "isInherited")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_inherited: Option<bool>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

/// `Roblox.Web.GameJoin.StatusData`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct StatusData {
    #[serde(rename = "creatorExperienceBan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_experience_ban: Option<CreatorExperienceBanData>,
}

/// `Roblox.GameJoin.Api.GameJoinResponse`
///
/// Response model for a game join request
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GameJoinResponse {
    /// The authentication ticket
    #[serde(rename = "authenticationTicket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_ticket: Option<String>,
    /// The authentication URL
    #[serde(rename = "authenticationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_url: Option<String>,
    /// The job id string
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "joinScript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_script: Option<JoinInformation>,
    /// The URL to get the join 'script' from set when status == 2 (Joining)
    #[serde(rename = "joinScriptUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_script_url: Option<String>,
    /// The string message to provide additional details on status code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The player's current position in the join queue. Only used when trying to join full servers. 0 if not queued
    #[serde(rename = "queuePosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_position: Option<i32>,
    /// The integer status code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "statusData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_data: Option<StatusData>,
}

/// `Roblox.GameJoin.Api.JoinGameInstanceRequest`
///
/// Request model for join game instance.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct JoinGameInstanceRequest {
    /// The integer representing browserTrackerId
    #[serde(rename = "browserTrackerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_tracker_id: Option<i64>,
    #[serde(rename = "cId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_id: Option<String>,
    /// String representing channel name in RCC Channel 2.0. Note: Only internal users (email address ending in roblox.com and verified) can use this feature. For other users, this field should be null, otherwise the request will be rejected.
    #[serde(rename = "channelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    /// The Guid representing the game instance
    #[serde(rename = "gameId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_id: Option<String>,
    /// The GameJoin Attempt Id for this game join request
    #[serde(rename = "gameJoinAttemptId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_join_attempt_id: Option<String>,
    /// The gamerTag, if any, for the game join request
    #[serde(rename = "gamerTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gamer_tag: Option<String>,
    /// Is this an Immersive Ads Teleport request.
    #[serde(rename = "isImmersiveAdsTeleport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_immersive_ads_teleport: Option<bool>,
    /// Boolean representing whether this is a PlayTogether game
    #[serde(rename = "isPlayTogetherGame")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_play_together_game: Option<bool>,
    /// Whether or not the game full join queue is used depends on the API and whether the join is a teleport. This optional override allows the Client to specify if it wants to queue when joining full games
    #[serde(rename = "isQueueAllowedOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_queue_allowed_override: Option<bool>,
    /// Is this a teleport request
    #[serde(rename = "isTeleport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_teleport: Option<bool>,
    /// The JoinOrigin of a gamejoin request, as propagated from the game client.
    #[serde(rename = "joinOrigin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_origin: Option<String>,
    /// The ID of the player's current party, if any
    #[serde(rename = "partyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_id: Option<String>,
    /// The Place Id for the game join request
    #[serde(rename = "placeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_id: Option<i64>,
}

/// `Roblox.GameJoin.Api.JoinPlayTogetherGameRequest`
///
/// Request model for join game instance.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct JoinPlayTogetherGameRequest {
    /// The integer representing browserTrackerId
    #[serde(rename = "browserTrackerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_tracker_id: Option<i64>,
    /// String representing channel name in RCC Channel 2.0. Note: Only internal users (email address ending in roblox.com and verified) can use this feature. For other users, this field should be null, otherwise the request will be rejected.
    #[serde(rename = "channelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    /// conversation id for requesting play together game
    #[serde(rename = "conversationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<i64>,
    /// The GameJoin Attempt Id for this game join request
    #[serde(rename = "gameJoinAttemptId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_join_attempt_id: Option<String>,
    /// The gamerTag, if any, for the game join request
    #[serde(rename = "gamerTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gamer_tag: Option<String>,
    /// Is this an Immersive Ads Teleport request.
    #[serde(rename = "isImmersiveAdsTeleport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_immersive_ads_teleport: Option<bool>,
    /// Boolean representing whether this is a PlayTogether game
    #[serde(rename = "isPlayTogetherGame")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_play_together_game: Option<bool>,
    /// Whether or not the game full join queue is used depends on the API and whether the join is a teleport. This optional override allows the Client to specify if it wants to queue when joining full games
    #[serde(rename = "isQueueAllowedOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_queue_allowed_override: Option<bool>,
    /// Is this a teleport request
    #[serde(rename = "isTeleport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_teleport: Option<bool>,
    /// The JoinOrigin of a gamejoin request, as propagated from the game client.
    #[serde(rename = "joinOrigin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_origin: Option<String>,
    /// The ID of the player's current party, if any
    #[serde(rename = "partyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_id: Option<String>,
    /// The Place Id for the game join request
    #[serde(rename = "placeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_id: Option<i64>,
}

/// `Roblox.GameJoin.Api.JoinPrivateGameRequest`
///
/// Request model for join private game .
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct JoinPrivateGameRequest {
    /// access code for join private game request
    #[serde(rename = "accessCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_code: Option<String>,
    /// The integer representing browserTrackerId
    #[serde(rename = "browserTrackerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_tracker_id: Option<i64>,
    /// String representing channel name in RCC Channel 2.0. Note: Only internal users (email address ending in roblox.com and verified) can use this feature. For other users, this field should be null, otherwise the request will be rejected.
    #[serde(rename = "channelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    /// The GameJoin Attempt Id for this game join request
    #[serde(rename = "gameJoinAttemptId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_join_attempt_id: Option<String>,
    /// The gamerTag, if any, for the game join request
    #[serde(rename = "gamerTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gamer_tag: Option<String>,
    /// Is this an Immersive Ads Teleport request.
    #[serde(rename = "isImmersiveAdsTeleport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_immersive_ads_teleport: Option<bool>,
    /// Boolean representing whether this is a PlayTogether game
    #[serde(rename = "isPlayTogetherGame")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_play_together_game: Option<bool>,
    /// Whether or not the game full join queue is used depends on the API and whether the join is a teleport. This optional override allows the Client to specify if it wants to queue when joining full games
    #[serde(rename = "isQueueAllowedOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_queue_allowed_override: Option<bool>,
    /// Is this a teleport request
    #[serde(rename = "isTeleport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_teleport: Option<bool>,
    /// The JoinOrigin of a gamejoin request, as propagated from the game client.
    #[serde(rename = "joinOrigin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_origin: Option<String>,
    /// link code for join private game request
    #[serde(rename = "linkCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_code: Option<String>,
    /// The ID of the player's current party, if any
    #[serde(rename = "partyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_id: Option<String>,
    /// The Place Id for the game join request
    #[serde(rename = "placeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_id: Option<i64>,
}

/// `Roblox.GameJoin.Api.JoinReservedGameRequest`
///
/// Request model for join private game .
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct JoinReservedGameRequest {
    /// access code for join private game request
    #[serde(rename = "accessCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_code: Option<String>,
    /// The integer representing browserTrackerId
    #[serde(rename = "browserTrackerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_tracker_id: Option<i64>,
    #[serde(rename = "cId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_id: Option<String>,
    /// String representing channel name in RCC Channel 2.0. Note: Only internal users (email address ending in roblox.com and verified) can use this feature. For other users, this field should be null, otherwise the request will be rejected.
    #[serde(rename = "channelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    /// The GameJoin Attempt Id for this game join request
    #[serde(rename = "gameJoinAttemptId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_join_attempt_id: Option<String>,
    /// The gamerTag, if any, for the game join request
    #[serde(rename = "gamerTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gamer_tag: Option<String>,
    /// Is this an Immersive Ads Teleport request.
    #[serde(rename = "isImmersiveAdsTeleport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_immersive_ads_teleport: Option<bool>,
    /// Boolean representing whether this is a PlayTogether game
    #[serde(rename = "isPlayTogetherGame")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_play_together_game: Option<bool>,
    /// Whether or not the game full join queue is used depends on the API and whether the join is a teleport. This optional override allows the Client to specify if it wants to queue when joining full games
    #[serde(rename = "isQueueAllowedOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_queue_allowed_override: Option<bool>,
    /// Is this a teleport request
    #[serde(rename = "isTeleport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_teleport: Option<bool>,
    /// The JoinOrigin of a gamejoin request, as propagated from the game client.
    #[serde(rename = "joinOrigin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_origin: Option<String>,
    /// The ID of the player's current party, if any
    #[serde(rename = "partyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_id: Option<String>,
    /// The Place Id for the game join request
    #[serde(rename = "placeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_id: Option<i64>,
}

/// `Roblox.GameJoin.Api.PlayWithUserRequest`
///
/// Request model for join private game .
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PlayWithUserRequest {
    /// The integer representing browserTrackerId
    #[serde(rename = "browserTrackerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_tracker_id: Option<i64>,
    /// String representing channel name in RCC Channel 2.0. Note: Only internal users (email address ending in roblox.com and verified) can use this feature. For other users, this field should be null, otherwise the request will be rejected.
    #[serde(rename = "channelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    /// The GameJoin Attempt Id for this game join request
    #[serde(rename = "gameJoinAttemptId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_join_attempt_id: Option<String>,
    /// The gamerTag, if any, for the game join request
    #[serde(rename = "gamerTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gamer_tag: Option<String>,
    /// Is this an Immersive Ads Teleport request.
    #[serde(rename = "isImmersiveAdsTeleport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_immersive_ads_teleport: Option<bool>,
    /// Boolean representing whether this is a PlayTogether game
    #[serde(rename = "isPlayTogetherGame")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_play_together_game: Option<bool>,
    /// Whether or not the game full join queue is used depends on the API and whether the join is a teleport. This optional override allows the Client to specify if it wants to queue when joining full games
    #[serde(rename = "isQueueAllowedOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_queue_allowed_override: Option<bool>,
    /// Is this a teleport request
    #[serde(rename = "isTeleport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_teleport: Option<bool>,
    /// The JoinOrigin of a gamejoin request, as propagated from the game client.
    #[serde(rename = "joinOrigin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_origin: Option<String>,
    /// The ID of the player's current party, if any
    #[serde(rename = "partyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_id: Option<String>,
    /// The Place Id for the game join request
    #[serde(rename = "placeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_id: Option<i64>,
    /// User Id for join private game request
    #[serde(rename = "userIdToFollow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_to_follow: Option<i64>,
}

/// `Roblox.GameJoin.Api.TeamCreateRequest`
///
/// Request model for create team .
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TeamCreateRequest {
    /// The integer representing browserTrackerId
    #[serde(rename = "browserTrackerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_tracker_id: Option<i64>,
    /// String representing channel name in RCC Channel 2.0. Note: Only internal users (email address ending in roblox.com and verified) can use this feature. For other users, this field should be null, otherwise the request will be rejected.
    #[serde(rename = "channelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    /// The GameJoin Attempt Id for this game join request
    #[serde(rename = "gameJoinAttemptId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_join_attempt_id: Option<String>,
    /// The gamerTag, if any, for the game join request
    #[serde(rename = "gamerTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gamer_tag: Option<String>,
    /// Is this an Immersive Ads Teleport request.
    #[serde(rename = "isImmersiveAdsTeleport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_immersive_ads_teleport: Option<bool>,
    /// Boolean representing whether this is a PlayTogether game
    #[serde(rename = "isPlayTogetherGame")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_play_together_game: Option<bool>,
    /// Whether or not the game full join queue is used depends on the API and whether the join is a teleport. This optional override allows the Client to specify if it wants to queue when joining full games
    #[serde(rename = "isQueueAllowedOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_queue_allowed_override: Option<bool>,
    /// Is this a teleport request
    #[serde(rename = "isTeleport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_teleport: Option<bool>,
    /// The JoinOrigin of a gamejoin request, as propagated from the game client.
    #[serde(rename = "joinOrigin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_origin: Option<String>,
    /// The ID of the player's current party, if any
    #[serde(rename = "partyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_id: Option<String>,
    /// The Place Id for the game join request
    #[serde(rename = "placeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_id: Option<i64>,
}

/// `Roblox.GameJoin.Api.TeamCreateResponse`
///
/// Response model for a create team request
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TeamCreateResponse {
    /// The string message to provide additional details on status code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
    /// The integer status code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

/// `POST /v1/join-game`
///
/// Endpoint to join a game
pub fn post_join_game_request(body: &GameJoinRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/join-game".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/join-game`
///
/// Endpoint to join a game
pub async fn post_join_game(client: &Client, body: &GameJoinRequest) -> Result<GameJoinResponse> {
    client.send(post_join_game_request(body)?).await?.json()
}

/// `POST /v1/join-game-instance`
///
/// Endpoint to join a particular game instance
pub fn post_join_game_instance_request(body: &JoinGameInstanceRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/join-game-instance".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/join-game-instance`
///
/// Endpoint to join a particular game instance
pub async fn post_join_game_instance(
    client: &Client,
    body: &JoinGameInstanceRequest,
) -> Result<GameJoinResponse> {
    client.send(post_join_game_instance_request(body)?).await?.json()
}

/// `POST /v1/join-play-together-game`
///
/// Endpoint to join play together game
pub fn post_join_play_together_game_request(body: &JoinPlayTogetherGameRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/join-play-together-game".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/join-play-together-game`
///
/// Endpoint to join play together game
pub async fn post_join_play_together_game(
    client: &Client,
    body: &JoinPlayTogetherGameRequest,
) -> Result<GameJoinResponse> {
    client
        .send(post_join_play_together_game_request(body)?)
        .await?
        .json()
}

/// `POST /v1/join-private-game`
///
/// Endpoint to join a private game
pub fn post_join_private_game_request(body: &JoinPrivateGameRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/join-private-game".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/join-private-game`
///
/// Endpoint to join a private game
pub async fn post_join_private_game(
    client: &Client,
    body: &JoinPrivateGameRequest,
) -> Result<GameJoinResponse> {
    client.send(post_join_private_game_request(body)?).await?.json()
}

/// `POST /v1/join-reserved-game`
///
/// Endpoint to join a reserved game
pub fn post_join_reserved_game_request(body: &JoinReservedGameRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/join-reserved-game".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/join-reserved-game`
///
/// Endpoint to join a reserved game
pub async fn post_join_reserved_game(
    client: &Client,
    body: &JoinReservedGameRequest,
) -> Result<GameJoinResponse> {
    client.send(post_join_reserved_game_request(body)?).await?.json()
}

/// `POST /v1/play-with-user`
///
/// Endpoint to play with user
pub fn post_play_with_user_request(body: &PlayWithUserRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/play-with-user".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/play-with-user`
///
/// Endpoint to play with user
pub async fn post_play_with_user(client: &Client, body: &PlayWithUserRequest) -> Result<GameJoinResponse> {
    client.send(post_play_with_user_request(body)?).await?.json()
}

/// `POST /v1/team-create`
///
/// Endpoint to join team create session
///
/// Xsrf protection disabled because it will only be used on Roblox clients. Adding an extra round-trip would impact client performance.
pub fn post_team_create_request(body: &TeamCreateRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/team-create".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/team-create`
///
/// Endpoint to join team create session
///
/// Xsrf protection disabled because it will only be used on Roblox clients. Adding an extra round-trip would impact client performance.
pub async fn post_team_create(client: &Client, body: &TeamCreateRequest) -> Result<TeamCreateResponse> {
    client.send(post_team_create_request(body)?).await?.json()
}

/// `POST /v1/team-create-preemptive`
///
/// Endpoint to preemptively start team create session.
///
/// Since the request origin might not be Roblox software, this endpoint implements a different set of security rules. More specifically, it requires Xsrf protection and disables user-agent check so that it can be called from browsers.
pub fn post_team_create_preemptive_request(body: &TeamCreateRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/team-create-preemptive".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/team-create-preemptive`
///
/// Endpoint to preemptively start team create session.
///
/// Since the request origin might not be Roblox software, this endpoint implements a different set of security rules. More specifically, it requires Xsrf protection and disables user-agent check so that it can be called from browsers.
pub async fn post_team_create_preemptive(
    client: &Client,
    body: &TeamCreateRequest,
) -> Result<TeamCreateResponse> {
    client
        .send(post_team_create_preemptive_request(body)?)
        .await?
        .json()
}

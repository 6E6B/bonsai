//! `friends.roblox.com` v1 — generated from `vendor/roblox-api-tracker/friends/v1.json`.
//!
//! Do not edit. Change the spec or `bonsai-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "friends.roblox.com";

/// `Roblox.Friends.Api.CaptchaStatusResponseModel`
///
/// This is response model to notify when action succeeded, failed, or captcha is required
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CaptchaStatusResponseModel {
    /// Captcha is set to true if captcha is required from user to perform action
    #[serde(rename = "isCaptchaRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_captcha_required: Option<bool>,
    #[serde(rename = "notificationMuted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_muted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

/// `Roblox.Friends.Api.FollowCountResponse`
///
/// Response model for following or followers Count
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FollowCountResponse {
    /// Count of following or followers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

/// `Roblox.Friends.Api.FollowingExistsRequestModel`
///
/// Request model for FollowingExists endpoint
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FollowingExistsRequestModel {
    /// The userIds which the user may or may not be following.
    #[serde(rename = "targetUserIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_user_ids: Option<Vec<i64>>,
}

crate::api_enum! {
    int
    /// The origin source type associated with the friend request. \['Unknown' = 0, 'PlayerSearch' = 1, 'QrCode' = 2, 'InGame' = 3, 'UserProfile' = 4, 'QqContactImporter' = 5, 'WeChatContactImporter' = 6, 'ProfileShare' = 7, 'PhoneContactImporter' = 8, 'FriendRecommendations' = 9, 'UserCommunities' = 10, 'TrustedFriend' = 11, 'SchoolMemberList' = 12, 'SocialTabsPage' = 13\]
    FriendRequestOriginSourceType {
    UnknownValue = 0 => "Unknown",
    PlayerSearch = 1 => "PlayerSearch",
    QrCode = 2 => "QrCode",
    InGame = 3 => "InGame",
    UserProfile = 4 => "UserProfile",
    QqContactImporter = 5 => "QqContactImporter",
    WeChatContactImporter = 6 => "WeChatContactImporter",
    ProfileShare = 7 => "ProfileShare",
    PhoneContactImporter = 8 => "PhoneContactImporter",
    FriendRecommendations = 9 => "FriendRecommendations",
    UserCommunities = 10 => "UserCommunities",
    TrustedFriend = 11 => "TrustedFriend",
    SchoolMemberList = 12 => "SchoolMemberList",
    SocialTabsPage = 13 => "SocialTabsPage",
    }
}

/// `Roblox.Friends.Api.FriendRequest`
///
/// A response model representing a friend request.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FriendRequest {
    /// The contact name associated with the friend request.
    #[serde(rename = "contactName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_name: Option<String>,
    /// The origin source type associated with the friend request. \['Unknown' = 0, 'PlayerSearch' = 1, 'QrCode' = 2, 'InGame' = 3, 'UserProfile' = 4, 'QqContactImporter' = 5, 'WeChatContactImporter' = 6, 'ProfileShare' = 7, 'PhoneContactImporter' = 8, 'FriendRecommendations' = 9, 'UserCommunities' = 10, 'TrustedFriend' = 11, 'SchoolMemberList' = 12, 'SocialTabsPage' = 13\]
    #[serde(rename = "originSourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_source_type: Option<FriendRequestOriginSourceType>,
    /// The sender user Id.
    #[serde(rename = "senderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<i64>,
    /// The nickname associated with the friend request.
    #[serde(rename = "senderNickname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_nickname: Option<String>,
    /// When the friend request was sent.
    #[serde(rename = "sentAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_at: Option<String>,
    /// The source universe Id which the request was sent in.
    #[serde(rename = "sourceUniverseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_universe_id: Option<i64>,
}

/// `Roblox.Friends.Api.FriendRequestResponse`
///
/// A response model representing a friend request.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FriendRequestResponse {
    /// When the user signed up.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// The user description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The user DisplayName.
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Used when user is logged in from third party app (e.g. QQ) ExternalAppDisplayName is the name used in that app (e.g. QQ nickname)
    #[serde(rename = "externalAppDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_app_display_name: Option<String>,
    #[serde(rename = "friendRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friend_request: Option<FriendRequest>,
    /// The user's verified badge status.
    #[serde(rename = "hasVerifiedBadge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_verified_badge: Option<bool>,
    /// The user Id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Whether or not the user is banned
    #[serde(rename = "isBanned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_banned: Option<bool>,
    /// The user's verified badge status.
    #[serde(rename = "isUnseen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_unseen: Option<bool>,
    /// mutualFriendsList
    #[serde(rename = "mutualFriendsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutual_friends_list: Option<Vec<String>>,
    /// The user name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

crate::api_enum! {
    int
    /// The friendship status. \['NotFriends' = 0, 'Friends' = 1, 'RequestSent' = 2, 'RequestReceived' = 3\]
    FriendStatusResponseStatus {
    NotFriends = 0 => "NotFriends",
    Friends = 1 => "Friends",
    RequestSent = 2 => "RequestSent",
    RequestReceived = 3 => "RequestReceived",
    }
}

/// `Roblox.Friends.Api.FriendStatusResponse`
///
/// The friendship status response model.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FriendStatusResponse {
    /// The user Id of the friend.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The friendship status. \['NotFriends' = 0, 'Friends' = 1, 'RequestSent' = 2, 'RequestReceived' = 3\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<FriendStatusResponseStatus>,
}

/// `Roblox.Friends.Api.FriendsCountResponse`
///
/// Response model for Friends Count
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FriendsCountResponse {
    /// Count of friends
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}

/// `Roblox.Friends.Api.Models.Response.FollowingExistsResponse`
///
/// Response contained in list for FollowingExists endpoint. Corresponds to a single userId.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FollowingExistsResponse {
    /// Whether or not a user is followed by userId in FriendsController.FollowingExists
    #[serde(rename = "isFollowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_followed: Option<bool>,
    /// Whether or not a user is following userId in FriendsController.FollowingExists
    #[serde(rename = "isFollowing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_following: Option<bool>,
    /// The userId being potentially followed
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}

/// `Roblox.Friends.Api.Models.Response.FollowingExistsResponseModel`
///
/// Response model for FollowingExists endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FollowingExistsResponseModel {
    /// A list of userIds and whether or not the given user is following them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followings: Option<Vec<FollowingExistsResponse>>,
}

/// `Roblox.Friends.Api.Models.Response.FriendResponse`
///
/// A response model representing friend information
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FriendResponse {
    /// The friend's verified badge status.
    #[serde(rename = "hasVerifiedBadge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_verified_badge: Option<bool>,
    /// The friend's userId
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
}

/// `Roblox.Friends.Api.Models.Response.FriendsPageMetadataResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FriendsPageMetadataResponse {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "frequentFriendSortRollout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequent_friend_sort_rollout: Option<i32>,
    #[serde(rename = "isFriendsFilterBarEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_friends_filter_bar_enabled: Option<bool>,
    #[serde(rename = "isFriendsPageSortExperimentEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_friends_page_sort_experiment_enabled: Option<bool>,
    #[serde(rename = "isFriendsUserDataStoreCacheEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_friends_user_data_store_cache_enabled: Option<bool>,
    #[serde(rename = "userName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

/// `Roblox.Friends.Api.Models.Response.NewFriendRequestsCountResponse`
///
/// The friendship status response model.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct NewFriendRequestsCountResponse {
    /// Count of new friend requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

/// `Roblox.Friends.Api.Models.Response.RefreshQrSessionResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RefreshQrSessionResponse {
    #[serde(rename = "Success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

/// `Roblox.Friends.Api.Models.Response.TrustedFriendRequestResponse`
///
/// A response model representing a friend request.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TrustedFriendRequestResponse {
    /// The sender user Id.
    #[serde(rename = "senderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<i64>,
    /// When the TF request was sent.
    #[serde(rename = "sentAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_at: Option<String>,
}

/// `Roblox.Friends.Api.Models.Response.UserPresenceResponseModel`
///
/// Response model for !:IUserPresence objects
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UserPresenceResponseModel {
    /// Location Type
    #[serde(rename = "UserLocationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_location_type: Option<String>,
    /// User Presence Type
    #[serde(rename = "UserPresenceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_presence_type: Option<String>,
    /// Game Instance Id
    #[serde(rename = "gameInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_instance_id: Option<String>,
    /// Last Location
    #[serde(rename = "lastLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_location: Option<String>,
    /// Most recent time online
    #[serde(rename = "lastOnline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_online: Option<String>,
    /// Place Id
    #[serde(rename = "placeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_id: Option<i64>,
    /// Root Place Id
    #[serde(rename = "rootPlaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_place_id: Option<i64>,
    /// Universe Id
    #[serde(rename = "universeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_id: Option<i64>,
}

/// `Roblox.Friends.Api.Models.Response.UserPresenceResponse`
///
/// A response model representing user presence information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UserPresenceResponse {
    /// The user DisplayName.
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The user Id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The user name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Carousel sort score
    #[serde(rename = "sortScore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_score: Option<f64>,
    #[serde(rename = "userPresence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_presence: Option<UserPresenceResponseModel>,
}

/// `Roblox.Friends.Api.Models.Response.UserResponse`
///
/// A response model representing user information that also contains select presence information
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UserResponse {
    /// When the user signed up.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// The user description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The user DisplayName.
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Used when user is logged in from third party app (e.g. QQ) ExternalAppDisplayName is the name used in that app (e.g. QQ nickname)
    #[serde(rename = "externalAppDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_app_display_name: Option<String>,
    /// Frequents rank for the user.
    #[serde(rename = "friendFrequentRank")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friend_frequent_rank: Option<i64>,
    /// Frequents value for the user.
    #[serde(rename = "friendFrequentScore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friend_frequent_score: Option<i64>,
    /// The user's verified badge status.
    #[serde(rename = "hasVerifiedBadge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_verified_badge: Option<bool>,
    /// The user Id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Whether or not the user is banned
    #[serde(rename = "isBanned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_banned: Option<bool>,
    /// Whether the user is deleted.
    #[serde(rename = "isDeleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    /// Whether the user is online.
    #[serde(rename = "isOnline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_online: Option<bool>,
    /// The user name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Where the user is online.
    #[serde(rename = "presenceType")]
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_type: Option<i32>,
}

/// `Roblox.Friends.Api.MultigetAreFriendsRequestModel`
///
/// Request model for MultigetAreFriends endpoint
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MultigetAreFriendsRequestModel {
    /// The target userIds to check against with the requesting user regarding friendship.
    #[serde(rename = "targetUserIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_user_ids: Option<Vec<i64>>,
}

/// `Roblox.Friends.Api.MultigetAreFriendsResponse`
///
/// Response model for MultigetAreFriendsResponse
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MultigetAreFriendsResponse {
    /// friends id list
    #[serde(rename = "friendsId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friends_id: Option<Vec<i64>>,
}

/// `Roblox.Friends.Api.PendingFriendRequestCountModel`
///
/// Response model for Pending Friend Request Count
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PendingFriendRequestCountModel {
    /// Count of pending friend requests
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}

/// `Roblox.Paging.CursoredPagedResult\[Roblox.Friends.Api.Models.Response.FriendResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CursoredPagedResultFriendResponse {
    #[serde(rename = "HasMore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(rename = "NextCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(rename = "PageItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_items: Option<Vec<FriendResponse>>,
    #[serde(rename = "PreviousCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_cursor: Option<String>,
}

/// `Roblox.Web.Captcha.Models.Request.CaptchaTokenRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CaptchaTokenRequest {
    #[serde(rename = "captchaId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_id: Option<String>,
    #[serde(rename = "captchaProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_provider: Option<String>,
    #[serde(rename = "captchaToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_token: Option<String>,
    #[serde(rename = "challengeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_id: Option<String>,
}

/// `Roblox.Web.WebAPI.ApiEmptyResponseModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiEmptyResponseModel {}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.Friends.Api.FriendStatusResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseFriendStatusResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<FriendStatusResponse>>,
}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.Friends.Api.Models.Response.UserPresenceResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseUserPresenceResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<UserPresenceResponse>>,
}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.Friends.Api.Models.Response.UserResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseUserResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<UserResponse>>,
}

/// `Roblox.Web.WebAPI.Models.ApiPageResponse\[Roblox.Friends.Api.FriendRequestResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiPageResponseFriendRequestResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<FriendRequestResponse>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
}

/// `Roblox.Web.WebAPI.Models.ApiPageResponse\[Roblox.Friends.Api.Models.Response.TrustedFriendRequestResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiPageResponseTrustedFriendRequestResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<TrustedFriendRequestResponse>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
}

/// `Roblox.Web.WebAPI.Models.ApiPageResponse\[Roblox.Friends.Api.Models.Response.UserResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiPageResponseUserResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<UserResponse>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
}

/// Query, header and form inputs for [`get_metadata`].
#[derive(Debug, Clone, Default)]
pub struct GetMetadataParams {
    pub target_user_id: Option<i64>,
}

/// Query, header and form inputs for [`get_my_friends_requests`].
#[derive(Debug, Clone, Default)]
pub struct GetMyFriendsRequestsParams {
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// Optional session identifier.
    pub session_id: Option<String>,
    pub friend_request_sort: Option<i32>,
    /// Whether to uprank unseen friend requests and return isUnseen per friend request.
    pub uprank_unseen: Option<bool>,
}

/// Query, header and form inputs for [`get_my_trusted_friends_requests`].
#[derive(Debug, Clone, Default)]
pub struct GetMyTrustedFriendsRequestsParams {
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// Specifies how to sort the returned trusted friend requests.
    pub sort_order: Option<i32>,
}

crate::api_enum! {
    str
    /// The order the results are sorted in.
    GetUsersByTargetUserIdFollowersSortOrder {
    Asc = 0 => "Asc",
    Desc = 1 => "Desc",
    }
}

/// Query, header and form inputs for [`get_users_by_target_user_id_followers`].
#[derive(Debug, Clone, Default)]
pub struct GetUsersByTargetUserIdFollowersParams {
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// The order the results are sorted in.
    pub sort_order: Option<GetUsersByTargetUserIdFollowersSortOrder>,
}

/// Query, header and form inputs for [`get_users_by_target_user_id_followings`].
#[derive(Debug, Clone, Default)]
pub struct GetUsersByTargetUserIdFollowingsParams {
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// The order the results are sorted in.
    pub sort_order: Option<GetUsersByTargetUserIdFollowersSortOrder>,
}

/// Query, header and form inputs for [`get_users_by_user_id_friends`].
#[derive(Debug, Clone, Default)]
pub struct GetUsersByUserIdFriendsParams {
    /// Specifies how to sort the returned friends.
    pub user_sort: Option<i32>,
}

crate::api_enum! {
    int
    /// \['FindFriends' = 0, 'FindTrustedFriends' = 1\]
    GetUsersByUserIdFriendsFindFindFriendsType {
    FindFriends = 0 => "FindFriends",
    FindTrustedFriends = 1 => "FindTrustedFriends",
    }
}

/// Query, header and form inputs for [`get_users_by_user_id_friends_find`].
#[derive(Debug, Clone, Default)]
pub struct GetUsersByUserIdFriendsFindParams {
    /// Specifies how to sort the returned friends.
    pub user_sort: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// The number of results per request.
    pub limit: Option<i32>,
    /// \['FindFriends' = 0, 'FindTrustedFriends' = 1\]
    pub find_friends_type: Option<GetUsersByUserIdFriendsFindFindFriendsType>,
}

/// Query, header and form inputs for [`get_users_by_user_id_friends_online`].
#[derive(Debug, Clone, Default)]
pub struct GetUsersByUserIdFriendsOnlineParams {
    /// The sort order to return the friends.
    pub user_sort: Option<i32>,
}

/// Query, header and form inputs for [`get_users_by_user_id_friends_search`].
#[derive(Debug, Clone, Default)]
pub struct GetUsersByUserIdFriendsSearchParams {
    /// The string to search names of friends for.
    pub query: Option<String>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// The number of results per request.
    pub limit: Option<i32>,
}

/// Query, header and form inputs for [`get_users_by_user_id_friends_statuses`].
#[derive(Debug, Clone, Default)]
pub struct GetUsersByUserIdFriendsStatusesParams {
    pub user_ids: Vec<i64>,
}

/// `GET /v1/metadata`
pub fn get_metadata_request(params: &GetMetadataParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/metadata".to_string());
    r.query("targetUserId", &params.target_user_id);
    Ok(r)
}

/// `GET /v1/metadata`
pub async fn get_metadata(
    client: &Client,
    params: &GetMetadataParams,
) -> Result<FriendsPageMetadataResponse> {
    client.send(get_metadata_request(params)?).await?.json()
}

/// `GET /v1/my/friends/count`
///
/// Get the number of friends a user has
pub fn get_my_friends_count_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/my/friends/count".to_string());
    Ok(r)
}

/// `GET /v1/my/friends/count`
///
/// Get the number of friends a user has
pub async fn get_my_friends_count(client: &Client) -> Result<FriendsCountResponse> {
    client.send(get_my_friends_count_request()?).await?.json()
}

/// `POST /v1/my/friends/refresh-qr-session`
pub fn post_my_friends_refresh_qr_session_request() -> Result<Request> {
    let r = Request::new(
        Method::POST,
        HOST,
        "/v1/my/friends/refresh-qr-session".to_string(),
    );
    Ok(r)
}

/// `POST /v1/my/friends/refresh-qr-session`
pub async fn post_my_friends_refresh_qr_session(client: &Client) -> Result<RefreshQrSessionResponse> {
    client
        .send(post_my_friends_refresh_qr_session_request()?)
        .await?
        .json()
}

/// `GET /v1/my/friends/requests`
///
/// Get all users that friend requests with targetUserId using exclusive start paging
pub fn get_my_friends_requests_request(params: &GetMyFriendsRequestsParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/my/friends/requests".to_string());
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    r.query("sessionId", &params.session_id);
    r.query("friendRequestSort", &params.friend_request_sort);
    r.query("uprankUnseen", &params.uprank_unseen);
    Ok(r)
}

/// `GET /v1/my/friends/requests`
///
/// Get all users that friend requests with targetUserId using exclusive start paging
pub async fn get_my_friends_requests(
    client: &Client,
    params: &GetMyFriendsRequestsParams,
) -> Result<ApiPageResponseFriendRequestResponse> {
    client
        .send(get_my_friends_requests_request(params)?)
        .await?
        .json()
}

/// `GET /v1/my/friends/{userId}/check-qr-session`
pub fn get_my_friends_by_user_id_check_qr_session_request(user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/my/friends/{}/check-qr-session",
            crate::rt::path_value(&user_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/my/friends/{userId}/check-qr-session`
pub async fn get_my_friends_by_user_id_check_qr_session(client: &Client, user_id: i64) -> Result<bool> {
    client
        .send(get_my_friends_by_user_id_check_qr_session_request(user_id)?)
        .await?
        .json()
}

/// `GET /v1/my/new-friend-requests/count`
pub fn get_my_new_friend_requests_count_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/my/new-friend-requests/count".to_string());
    Ok(r)
}

/// `GET /v1/my/new-friend-requests/count`
pub async fn get_my_new_friend_requests_count(client: &Client) -> Result<NewFriendRequestsCountResponse> {
    client
        .send(get_my_new_friend_requests_count_request()?)
        .await?
        .json()
}

/// `GET /v1/my/trusted-friends/count`
///
/// Get the number of trusted friends a user has
pub fn get_my_trusted_friends_count_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/my/trusted-friends/count".to_string());
    Ok(r)
}

/// `GET /v1/my/trusted-friends/count`
///
/// Get the number of trusted friends a user has
pub async fn get_my_trusted_friends_count(client: &Client) -> Result<FriendsCountResponse> {
    client.send(get_my_trusted_friends_count_request()?).await?.json()
}

/// `GET /v1/my/trusted-friends/requests`
///
/// Get all incoming trusted friend requests using exclusive start paging.
pub fn get_my_trusted_friends_requests_request(
    params: &GetMyTrustedFriendsRequestsParams,
) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/my/trusted-friends/requests".to_string());
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    r.query("sortOrder", &params.sort_order);
    Ok(r)
}

/// `GET /v1/my/trusted-friends/requests`
///
/// Get all incoming trusted friend requests using exclusive start paging.
pub async fn get_my_trusted_friends_requests(
    client: &Client,
    params: &GetMyTrustedFriendsRequestsParams,
) -> Result<ApiPageResponseTrustedFriendRequestResponse> {
    client
        .send(get_my_trusted_friends_requests_request(params)?)
        .await?
        .json()
}

/// `POST /v1/user/following-exists`
///
/// Returns whether or not the current user is following each userId in a list of userIds
pub fn post_user_following_exists_request(body: &FollowingExistsRequestModel) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/user/following-exists".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/user/following-exists`
///
/// Returns whether or not the current user is following each userId in a list of userIds
pub async fn post_user_following_exists(
    client: &Client,
    body: &FollowingExistsRequestModel,
) -> Result<FollowingExistsResponseModel> {
    client
        .send(post_user_following_exists_request(body)?)
        .await?
        .json()
}

/// `GET /v1/user/friend-requests/count`
///
/// Return the number of pending friend requests.
pub fn get_user_friend_requests_count_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/user/friend-requests/count".to_string());
    Ok(r)
}

/// `GET /v1/user/friend-requests/count`
///
/// Return the number of pending friend requests.
pub async fn get_user_friend_requests_count(client: &Client) -> Result<PendingFriendRequestCountModel> {
    client
        .send(get_user_friend_requests_count_request()?)
        .await?
        .json()
}

/// `GET /v1/user/trusted-friend-requests/count`
///
/// Return the number of pending trusted friend requests.
pub fn get_user_trusted_friend_requests_count_request() -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        "/v1/user/trusted-friend-requests/count".to_string(),
    );
    Ok(r)
}

/// `GET /v1/user/trusted-friend-requests/count`
///
/// Return the number of pending trusted friend requests.
pub async fn get_user_trusted_friend_requests_count(
    client: &Client,
) -> Result<PendingFriendRequestCountModel> {
    client
        .send(get_user_trusted_friend_requests_count_request()?)
        .await?
        .json()
}

/// `POST /v1/user/{userId}/multiget-are-friends`
///
/// Check if the requesting user is friends with the specified users.
pub fn post_user_by_user_id_multiget_are_friends_request(
    user_id: i64,
    body: &MultigetAreFriendsRequestModel,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/user/{}/multiget-are-friends",
            crate::rt::path_value(&user_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/user/{userId}/multiget-are-friends`
///
/// Check if the requesting user is friends with the specified users.
pub async fn post_user_by_user_id_multiget_are_friends(
    client: &Client,
    user_id: i64,
    body: &MultigetAreFriendsRequestModel,
) -> Result<MultigetAreFriendsResponse> {
    client
        .send(post_user_by_user_id_multiget_are_friends_request(user_id, body)?)
        .await?
        .json()
}

/// `POST /v1/users/{targetUserId}/follow`
///
/// Creates the following between a user and user with targetUserId
pub fn post_users_by_target_user_id_follow_request(
    target_user_id: i64,
    body: &CaptchaTokenRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!("/v1/users/{}/follow", crate::rt::path_value(&target_user_id)),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/users/{targetUserId}/follow`
///
/// Creates the following between a user and user with targetUserId
pub async fn post_users_by_target_user_id_follow(
    client: &Client,
    target_user_id: i64,
    body: &CaptchaTokenRequest,
) -> Result<CaptchaStatusResponseModel> {
    client
        .send(post_users_by_target_user_id_follow_request(target_user_id, body)?)
        .await?
        .json()
}

/// `GET /v1/users/{targetUserId}/followers`
///
/// Get all users that follow user with targetUserId in page response format
pub fn get_users_by_target_user_id_followers_request(
    target_user_id: i64,
    params: &GetUsersByTargetUserIdFollowersParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/users/{}/followers", crate::rt::path_value(&target_user_id)),
    );
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    r.query("sortOrder", &params.sort_order);
    Ok(r)
}

/// `GET /v1/users/{targetUserId}/followers`
///
/// Get all users that follow user with targetUserId in page response format
pub async fn get_users_by_target_user_id_followers(
    client: &Client,
    target_user_id: i64,
    params: &GetUsersByTargetUserIdFollowersParams,
) -> Result<ApiPageResponseUserResponse> {
    client
        .send(get_users_by_target_user_id_followers_request(
            target_user_id,
            params,
        )?)
        .await?
        .json()
}

/// `GET /v1/users/{targetUserId}/followers/count`
///
/// Get the number of following a user has
pub fn get_users_by_target_user_id_followers_count_request(target_user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/users/{}/followers/count",
            crate::rt::path_value(&target_user_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/users/{targetUserId}/followers/count`
///
/// Get the number of following a user has
pub async fn get_users_by_target_user_id_followers_count(
    client: &Client,
    target_user_id: i64,
) -> Result<FollowCountResponse> {
    client
        .send(get_users_by_target_user_id_followers_count_request(
            target_user_id,
        )?)
        .await?
        .json()
}

/// `GET /v1/users/{targetUserId}/followings`
///
/// Get all users that user with targetUserId is following in page response format
pub fn get_users_by_target_user_id_followings_request(
    target_user_id: i64,
    params: &GetUsersByTargetUserIdFollowingsParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/users/{}/followings", crate::rt::path_value(&target_user_id)),
    );
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    r.query("sortOrder", &params.sort_order);
    Ok(r)
}

/// `GET /v1/users/{targetUserId}/followings`
///
/// Get all users that user with targetUserId is following in page response format
pub async fn get_users_by_target_user_id_followings(
    client: &Client,
    target_user_id: i64,
    params: &GetUsersByTargetUserIdFollowingsParams,
) -> Result<ApiPageResponseUserResponse> {
    client
        .send(get_users_by_target_user_id_followings_request(
            target_user_id,
            params,
        )?)
        .await?
        .json()
}

/// `GET /v1/users/{targetUserId}/followings/count`
///
/// Get the number of following a user has
pub fn get_users_by_target_user_id_followings_count_request(target_user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/users/{}/followings/count",
            crate::rt::path_value(&target_user_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/users/{targetUserId}/followings/count`
///
/// Get the number of following a user has
pub async fn get_users_by_target_user_id_followings_count(
    client: &Client,
    target_user_id: i64,
) -> Result<FollowCountResponse> {
    client
        .send(get_users_by_target_user_id_followings_count_request(
            target_user_id,
        )?)
        .await?
        .json()
}

/// `POST /v1/users/{targetUserId}/unfollow`
///
/// Deletes the following between a user and user with targetUserId
pub fn post_users_by_target_user_id_unfollow_request(target_user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::POST,
        HOST,
        format!("/v1/users/{}/unfollow", crate::rt::path_value(&target_user_id)),
    );
    Ok(r)
}

/// `POST /v1/users/{targetUserId}/unfollow`
///
/// Deletes the following between a user and user with targetUserId
pub async fn post_users_by_target_user_id_unfollow(
    client: &Client,
    target_user_id: i64,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_users_by_target_user_id_unfollow_request(target_user_id)?)
        .await?
        .json()
}

/// `GET /v1/users/{userId}/friends`
///
/// Get list of all friends for the specified user.
pub fn get_users_by_user_id_friends_request(
    user_id: i64,
    params: &GetUsersByUserIdFriendsParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/users/{}/friends", crate::rt::path_value(&user_id)),
    );
    r.query("userSort", &params.user_sort);
    Ok(r)
}

/// `GET /v1/users/{userId}/friends`
///
/// Get list of all friends for the specified user.
pub async fn get_users_by_user_id_friends(
    client: &Client,
    user_id: i64,
    params: &GetUsersByUserIdFriendsParams,
) -> Result<ApiArrayResponseUserResponse> {
    client
        .send(get_users_by_user_id_friends_request(user_id, params)?)
        .await?
        .json()
}

/// `GET /v1/users/{userId}/friends/count`
///
/// Get the number of friends a user has
pub fn get_users_by_user_id_friends_count_request(user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/users/{}/friends/count", crate::rt::path_value(&user_id)),
    );
    Ok(r)
}

/// `GET /v1/users/{userId}/friends/count`
///
/// Get the number of friends a user has
pub async fn get_users_by_user_id_friends_count(
    client: &Client,
    user_id: i64,
) -> Result<FriendsCountResponse> {
    client
        .send(get_users_by_user_id_friends_count_request(user_id)?)
        .await?
        .json()
}

/// `GET /v1/users/{userId}/friends/find`
///
/// Get a paginated list of all friends for the specified user.
pub fn get_users_by_user_id_friends_find_request(
    user_id: i64,
    params: &GetUsersByUserIdFriendsFindParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/users/{}/friends/find", crate::rt::path_value(&user_id)),
    );
    r.query("userSort", &params.user_sort);
    r.query("cursor", &params.cursor);
    r.query("limit", &params.limit);
    r.query("findFriendsType", &params.find_friends_type);
    Ok(r)
}

/// `GET /v1/users/{userId}/friends/find`
///
/// Get a paginated list of all friends for the specified user.
pub async fn get_users_by_user_id_friends_find(
    client: &Client,
    user_id: i64,
    params: &GetUsersByUserIdFriendsFindParams,
) -> Result<CursoredPagedResultFriendResponse> {
    client
        .send(get_users_by_user_id_friends_find_request(user_id, params)?)
        .await?
        .json()
}

/// `GET /v1/users/{userId}/friends/inactive`
///
/// Get list of inactive friends for the specified user.
pub fn get_users_by_user_id_friends_inactive_request(user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/users/{}/friends/inactive", crate::rt::path_value(&user_id)),
    );
    Ok(r)
}

/// `GET /v1/users/{userId}/friends/inactive`
///
/// Get list of inactive friends for the specified user.
pub async fn get_users_by_user_id_friends_inactive(
    client: &Client,
    user_id: i64,
) -> Result<ApiArrayResponseUserResponse> {
    client
        .send(get_users_by_user_id_friends_inactive_request(user_id)?)
        .await?
        .json()
}

/// `GET /v1/users/{userId}/friends/online`
///
/// Get list of all online friends for the specified user.
pub fn get_users_by_user_id_friends_online_request(
    user_id: i64,
    params: &GetUsersByUserIdFriendsOnlineParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/users/{}/friends/online", crate::rt::path_value(&user_id)),
    );
    r.query("userSort", &params.user_sort);
    Ok(r)
}

/// `GET /v1/users/{userId}/friends/online`
///
/// Get list of all online friends for the specified user.
pub async fn get_users_by_user_id_friends_online(
    client: &Client,
    user_id: i64,
    params: &GetUsersByUserIdFriendsOnlineParams,
) -> Result<ApiArrayResponseUserPresenceResponse> {
    client
        .send(get_users_by_user_id_friends_online_request(user_id, params)?)
        .await?
        .json()
}

/// `GET /v1/users/{userId}/friends/search`
///
/// Search for friends by name using a text query.
pub fn get_users_by_user_id_friends_search_request(
    user_id: i64,
    params: &GetUsersByUserIdFriendsSearchParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/users/{}/friends/search", crate::rt::path_value(&user_id)),
    );
    r.query("query", &params.query);
    r.query("cursor", &params.cursor);
    r.query("limit", &params.limit);
    Ok(r)
}

/// `GET /v1/users/{userId}/friends/search`
///
/// Search for friends by name using a text query.
pub async fn get_users_by_user_id_friends_search(
    client: &Client,
    user_id: i64,
    params: &GetUsersByUserIdFriendsSearchParams,
) -> Result<CursoredPagedResultFriendResponse> {
    client
        .send(get_users_by_user_id_friends_search_request(user_id, params)?)
        .await?
        .json()
}

/// `GET /v1/users/{userId}/friends/statuses`
///
/// Gets a list of friend statuses of specified users against the specified user.
pub fn get_users_by_user_id_friends_statuses_request(
    user_id: i64,
    params: &GetUsersByUserIdFriendsStatusesParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/users/{}/friends/statuses", crate::rt::path_value(&user_id)),
    );
    r.query("userIds", &params.user_ids);
    Ok(r)
}

/// `GET /v1/users/{userId}/friends/statuses`
///
/// Gets a list of friend statuses of specified users against the specified user.
pub async fn get_users_by_user_id_friends_statuses(
    client: &Client,
    user_id: i64,
    params: &GetUsersByUserIdFriendsStatusesParams,
) -> Result<ApiArrayResponseFriendStatusResponse> {
    client
        .send(get_users_by_user_id_friends_statuses_request(user_id, params)?)
        .await?
        .json()
}

/// `GET /v1/users/{userId}/trusted-friends/count`
///
/// Get the number of trusted friends a user has
pub fn get_users_by_user_id_trusted_friends_count_request(user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/users/{}/trusted-friends/count",
            crate::rt::path_value(&user_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/users/{userId}/trusted-friends/count`
///
/// Get the number of trusted friends a user has
pub async fn get_users_by_user_id_trusted_friends_count(
    client: &Client,
    user_id: i64,
) -> Result<FriendsCountResponse> {
    client
        .send(get_users_by_user_id_trusted_friends_count_request(user_id)?)
        .await?
        .json()
}

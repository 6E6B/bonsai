//! `groups.roblox.com` v1 — generated from `vendor/roblox-api-tracker/groups/v1.json`.
//!
//! Do not edit. Change the spec or `rbx-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "groups.roblox.com";

/// `Roblox.Groups.Client.BlockedKeywordModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BlockedKeywordModel {
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isPrivate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// `Roblox.Groups.Api.BlockedKeywordPageResponse\[Roblox.Groups.Client.BlockedKeywordModel\]`
///
/// ApiPageResponse for blocked keywords
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BlockedKeywordPageResponseBlockedKeywordModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<BlockedKeywordModel>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
    /// Total number of active keywords in the group
    #[serde(rename = "totalActiveKeywordsCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_active_keywords_count: Option<i32>,
}

/// `Roblox.Groups.Api.ChangeOwnerRequest`
///
/// A request model for changing the group owner.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChangeOwnerRequest {
    /// The user id.
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}

/// `Roblox.Groups.Api.CommunityFeatureFreezeStatus`
///
/// Response model representing the freeze status of a community feature.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CommunityFeatureFreezeStatus {
    /// The community feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature: Option<String>,
    /// Whether the feature is currently disabled.
    #[serde(rename = "isDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
}

/// `Roblox.Groups.Api.CreateBlockedKeywordsRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CreateBlockedKeywordsRequest {
    #[serde(rename = "isPrivate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
}

/// `Roblox.Groups.Api.GetCommunityFeatureFreezesResponse`
///
/// Response model for the community feature freezes endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetCommunityFeatureFreezesResponse {
    /// The freeze status of each community feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<CommunityFeatureFreezeStatus>>,
}

crate::api_enum! {
    str
    /// The feature type. \['Payouts' = 0, 'ContentUpload' = 1, 'GroupOwnershipTransfer' = 2, 'GameOwnershipTransfer' = 3, 'ForumRead' = 4, 'ForumWrite' = 5\]
    GroupFeatureResponseFeature {
    Payouts = 0 => "Payouts",
    ContentUpload = 1 => "ContentUpload",
    GroupOwnershipTransfer = 2 => "GroupOwnershipTransfer",
    GameOwnershipTransfer = 3 => "GameOwnershipTransfer",
    ForumRead = 4 => "ForumRead",
    ForumWrite = 5 => "ForumWrite",
    }
}

/// `Roblox.Groups.Api.GroupFeatureResponse`
///
/// Response model representing the freeze status of a group feature.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupFeatureResponse {
    /// The UTC time at which the moderator-force expires, allowing the freeze to be deactivated upon request. Only present when the feature is forced frozen.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    /// The feature type. \['Payouts' = 0, 'ContentUpload' = 1, 'GroupOwnershipTransfer' = 2, 'GameOwnershipTransfer' = 3, 'ForumRead' = 4, 'ForumWrite' = 5\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature: Option<GroupFeatureResponseFeature>,
    /// Whether the feature is currently frozen.
    #[serde(rename = "isFeatureBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_feature_blocked: Option<bool>,
}

/// `Roblox.Groups.Api.GetGroupFeaturesResponse`
///
/// Response model for the group features endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetGroupFeaturesResponse {
    /// The freeze status of each feature for the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<GroupFeatureResponse>>,
    /// Whether the group is currently locked.
    #[serde(rename = "isLocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
}

crate::api_enum! {
    int
    /// What type of social media (including Roblox Group) this points to \['Facebook' = 0, 'Twitter' = 1, 'YouTube' = 2, 'Twitch' = 3, 'GooglePlus' = 4, 'Discord' = 5, 'RobloxGroup' = 6, 'Amazon' = 7, 'Guilded' = 8\]
    SocialLinkResponseType {
    Facebook = 0 => "Facebook",
    Twitter = 1 => "Twitter",
    YouTube = 2 => "YouTube",
    Twitch = 3 => "Twitch",
    GooglePlus = 4 => "GooglePlus",
    Discord = 5 => "Discord",
    RobloxGroup = 6 => "RobloxGroup",
    Amazon = 7 => "Amazon",
    Guilded = 8 => "Guilded",
    }
}

/// `Roblox.Groups.Api.SocialLinkResponse`
///
/// A social link response from a create request
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SocialLinkResponse {
    /// The id of the social link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The title of the link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// What type of social media (including Roblox Group) this points to \['Facebook' = 0, 'Twitter' = 1, 'YouTube' = 2, 'Twitch' = 3, 'GooglePlus' = 4, 'Discord' = 5, 'RobloxGroup' = 6, 'Amazon' = 7, 'Guilded' = 8\]
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<SocialLinkResponseType>,
    /// The url of the link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// `Roblox.Groups.Api.GetSocialLinkResponse`
///
/// A social link response from a create request
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetSocialLinkResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<SocialLinkResponse>>,
    #[serde(rename = "socialLinksVerificationStatus")]
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_links_verification_status: Option<i32>,
}

crate::api_enum! {
    int
    /// The role color. \['Invalid' = 0, 'Blue' = 1, 'Green' = 2, 'Purple' = 3, 'Yellow' = 4, 'Orange' = 5, 'Red' = 6, 'Magenta' = 7, 'Teal' = 8, 'Turquoise' = 9, 'Rust' = 10, 'Pistachio' = 11, 'Midnight' = 12, 'Lavender' = 13, 'Pink' = 14, 'Crimson' = 15, 'Plum' = 16\]
    GroupRoleResponseColor {
    Invalid = 0 => "Invalid",
    Blue = 1 => "Blue",
    Green = 2 => "Green",
    Purple = 3 => "Purple",
    Yellow = 4 => "Yellow",
    Orange = 5 => "Orange",
    Red = 6 => "Red",
    Magenta = 7 => "Magenta",
    Teal = 8 => "Teal",
    Turquoise = 9 => "Turquoise",
    Rust = 10 => "Rust",
    Pistachio = 11 => "Pistachio",
    Midnight = 12 => "Midnight",
    Lavender = 13 => "Lavender",
    Pink = 14 => "Pink",
    Crimson = 15 => "Crimson",
    Plum = 16 => "Plum",
    }
}

/// `Roblox.Groups.Api.GroupRoleResponse`
///
/// A group role response model
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupRoleResponse {
    /// The role color. \['Invalid' = 0, 'Blue' = 1, 'Green' = 2, 'Purple' = 3, 'Yellow' = 4, 'Orange' = 5, 'Red' = 6, 'Magenta' = 7, 'Teal' = 8, 'Turquoise' = 9, 'Rust' = 10, 'Pistachio' = 11, 'Midnight' = 12, 'Lavender' = 13, 'Pink' = 14, 'Crimson' = 15, 'Plum' = 16\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<GroupRoleResponseColor>,
    /// The role description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The role id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Whether or not the role is the base role for the group
    #[serde(rename = "isBase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_base: Option<bool>,
    /// Whether the role is private (only visible to members with permission). Omitted from the response when false.
    #[serde(rename = "isPrivate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    /// The number of members in the role.
    #[serde(rename = "memberCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i64>,
    /// The role name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The role rank
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
}

/// `Roblox.Groups.Api.GroupAllRolesResponse`
///
/// A group roles response model
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupAllRolesResponse {
    /// The group id
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i64>,
    /// The roles in the group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<GroupRoleResponse>>,
}

/// `Roblox.Groups.Api.Models.Response.UserModel`
///
/// A model representing data about an Roblox.Platform.Membership.IUser
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UserModel {
    /// The user's builders club membership type
    #[serde(rename = "buildersClubMembershipType")]
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builders_club_membership_type: Option<i32>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The user's verified badge status.
    #[serde(rename = "hasVerifiedBadge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_verified_badge: Option<bool>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// `Roblox.Groups.Api.UserGroupRoleResponse`
///
/// A user group role response model
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UserGroupRoleResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<GroupRoleResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserModel>,
}

/// `Roblox.Groups.Api.Models.Response.GroupAuditLogResponseItem`
///
/// A group audit log response model
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupAuditLogResponseItem {
    /// The action type
    #[serde(rename = "actionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<UserGroupRoleResponse>,
    /// Date the group action was performed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// Information on the action performed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<serde_json::Value>,
}

/// `Roblox.Groups.Api.GroupAuditLogPageResponse\[Roblox.Groups.Api.Models.Response.GroupAuditLogResponseItem\]`
///
/// ApiPageResponse for group audit log
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupAuditLogPageResponseGroupAuditLogResponseItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GroupAuditLogResponseItem>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
}

/// `Roblox.Groups.Api.GroupBanMemberResponse`
///
/// A ban member from group response
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupBanMemberResponse {
    #[serde(rename = "actingUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acting_user: Option<UserGroupRoleResponse>,
    /// The group ban's created time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserModel>,
}

/// `Roblox.Groups.Api.GroupForumsPermissionsModel`
///
/// A model representing group posts permissions
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupForumsPermissionsModel {
    /// Create bug reports (support ticket) permission. Only emitted when the permission is exposed (Roblox.Groups.Api.Properties.IGroupsApiSettings.ExposeCreateBugReportsPermission).
    #[serde(rename = "createBugReports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_bug_reports: Option<bool>,
    /// Create comments permission
    #[serde(rename = "createComments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_comments: Option<bool>,
    /// Create posts permission
    #[serde(rename = "createPosts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_posts: Option<bool>,
    /// Lock posts permission
    #[serde(rename = "lockPosts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_posts: Option<bool>,
    /// Manage categories permission
    #[serde(rename = "manageCategories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage_categories: Option<bool>,
    /// Pin posts permission
    #[serde(rename = "pinPosts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin_posts: Option<bool>,
    /// Remove comments permission
    #[serde(rename = "removeComments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_comments: Option<bool>,
    /// Remove posts permission
    #[serde(rename = "removePosts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_posts: Option<bool>,
    /// View forums permission
    #[serde(rename = "viewForums")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_forums: Option<bool>,
}

/// `Roblox.Groups.Api.GroupChannelPermissionsModel`
///
/// A model for communication channel permissions.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupChannelPermissionsModel {
    /// Group Channel Id
    #[serde(rename = "channelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(rename = "groupForumsPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_forums_permissions: Option<GroupForumsPermissionsModel>,
}

/// `Roblox.Groups.Api.GroupConfigurationDetailsResponse`
///
/// A detailed group response model
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupConfigurationDetailsResponse {
    /// The group cover photo id
    #[serde(rename = "coverPhotoId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_photo_id: Option<i64>,
    /// The group emblem id
    #[serde(rename = "emblemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emblem_id: Option<i64>,
    /// The group id
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i64>,
}

/// `Roblox.Groups.Api.GroupConfigurationResponse`
///
/// A response model for group configuration
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupConfigurationResponse {
    /// The cost of purchasing a group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<i64>,
    /// The maximum file size of the group cover photo in megabytes
    #[serde(rename = "coverPhotoMaxFileSizeMb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_photo_max_file_size_mb: Option<i32>,
    /// The maximum length of a group description
    #[serde(rename = "descriptionMaxLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_max_length: Option<i32>,
    /// The maximum file size of the group icon in megabytes
    #[serde(rename = "iconMaxFileSizeMb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_max_file_size_mb: Option<i32>,
    /// Should the frontend use the 2sv webview component (as opposed to the built-in 2sv pop up)
    #[serde(rename = "isUsingTwoStepWebviewComponent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_using_two_step_webview_component: Option<bool>,
    /// The maximum length of a group name
    #[serde(rename = "nameMaxLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_max_length: Option<i32>,
    /// The valid dimensions of the group cover photo in CSV format
    #[serde(rename = "validCoverPhotoDimensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_cover_photo_dimensions: Option<String>,
}

/// `Roblox.Groups.Api.GroupNameChangeConfigurationResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupNameChangeConfigurationResponse {
    /// The cooldown for group name changes in days
    #[serde(rename = "cooldownInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cooldown_in_days: Option<i32>,
    /// The cost of renaming a group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<i64>,
    /// The ownership cooldown for group name changes in days
    #[serde(rename = "ownershipCooldownInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_cooldown_in_days: Option<i32>,
}

/// `Roblox.Groups.Api.RecurringPayoutsConfigurationResponse`
///
/// A response model for recurring payout configuration
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RecurringPayoutsConfigurationResponse {
    /// The maximum number of recurring payout partners
    #[serde(rename = "maxPayoutPartners")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_payout_partners: Option<i32>,
}

/// `Roblox.Groups.Api.RoleConfigurationResponse`
///
/// A response model for role configuration
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RoleConfigurationResponse {
    /// The cost of purchasing a role
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<i64>,
    /// The maximum length of a role description
    #[serde(rename = "descriptionMaxLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_max_length: Option<i32>,
    /// The maximum number of roles in a group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// The maximum rank a role can have
    #[serde(rename = "maxRank")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_rank: Option<i32>,
    /// The minimum rank a role can have
    #[serde(rename = "minRank")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_rank: Option<i32>,
    /// The maximum length of a role name
    #[serde(rename = "nameMaxLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_max_length: Option<i32>,
}

/// `Roblox.Groups.Api.GroupConfigurationDisplayOptionsResponse`
///
/// A response model for group configuration
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupConfigurationDisplayOptionsResponse {
    #[serde(rename = "groupConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_configuration: Option<GroupConfigurationResponse>,
    #[serde(rename = "groupNameChangeConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name_change_configuration: Option<GroupNameChangeConfigurationResponse>,
    /// If set to true, default group emblem policies handled by GUAC will be enabled If set to false, default group emblem policies will not be enabled
    #[serde(rename = "isDefaultEmblemPolicyEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default_emblem_policy_enabled: Option<bool>,
    /// The configuration of premium payouts shows in Group Revenue Summary page
    #[serde(rename = "isPremiumPayoutsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_premium_payouts_enabled: Option<bool>,
    #[serde(rename = "recurringPayoutsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_payouts_configuration: Option<RecurringPayoutsConfigurationResponse>,
    #[serde(rename = "roleConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_configuration: Option<RoleConfigurationResponse>,
}

/// `Roblox.Groups.Api.GroupContentModerationPermissionsModel`
///
/// A model representing group content moderation permissions
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupContentModerationPermissionsModel {
    /// Manage keyword block list permission
    #[serde(rename = "manageKeywordBlockList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage_keyword_block_list: Option<bool>,
    /// View keyword block list permission
    #[serde(rename = "viewKeywordBlockList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_keyword_block_list: Option<bool>,
}

/// `Roblox.Groups.Api.GroupDescriptionResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupDescriptionResponse {
    /// The new description returned
    #[serde(rename = "newDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_description: Option<String>,
}

/// `Roblox.Groups.Client.TierCapabilities`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TierCapabilities {
    #[serde(rename = "isEligibleForUnrestrictedMessages")]
    pub is_eligible_for_unrestricted_messages: bool,
}

crate::api_enum! {
    int
    /// \['OwnerModerationStatusOk' = 1, 'OwnerAgeEstimationVerified' = 2, 'OwnerIdVerified' = 3, 'OwnerTwoStepVerified' = 4, 'CommunityMeetsPlayerRequirement' = 5\]
    TierRequirementKey {
    OwnerModerationStatusOk = 1 => "OwnerModerationStatusOk",
    OwnerAgeEstimationVerified = 2 => "OwnerAgeEstimationVerified",
    OwnerIdVerified = 3 => "OwnerIdVerified",
    OwnerTwoStepVerified = 4 => "OwnerTwoStepVerified",
    CommunityMeetsPlayerRequirement = 5 => "CommunityMeetsPlayerRequirement",
    }
}

/// `Roblox.Groups.Client.TierRequirement`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TierRequirement {
    /// \['OwnerModerationStatusOk' = 1, 'OwnerAgeEstimationVerified' = 2, 'OwnerIdVerified' = 3, 'OwnerTwoStepVerified' = 4, 'CommunityMeetsPlayerRequirement' = 5\]
    pub key: TierRequirementKey,
    pub satisfied: bool,
}

/// `Roblox.Groups.Client.CommunityTierInfoResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CommunityTierInfoResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<TierCapabilities>,
    #[serde(rename = "currentTier")]
    pub current_tier: i32,
    #[serde(rename = "groupId")]
    pub group_id: i64,
    #[serde(rename = "lastEvaluatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_evaluated_time: Option<String>,
    #[serde(rename = "previousTier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_tier: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<Vec<TierRequirement>>,
    #[serde(rename = "tierUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier_updated_time: Option<String>,
}

/// `Roblox.Groups.Api.ShoutResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ShoutResponse {
    /// The shout's message
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_: Option<String>,
    /// The shout's created time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poster: Option<UserModel>,
    /// The shout's last updated time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

/// `Roblox.Groups.Api.GroupDetailResponse`
///
/// A detailed group response model
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupDetailResponse {
    #[serde(rename = "communityTier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub community_tier: Option<CommunityTierInfoResponse>,
    /// The group description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether the group has social modules enabled (e.g. Forums) (determines if "Followers" vs "Members" should be shown).
    #[serde(rename = "hasSocialModules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_social_modules: Option<bool>,
    /// Whether the group has a verified badge.
    #[serde(rename = "hasVerifiedBadge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_verified_badge: Option<bool>,
    /// The group id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Whether the group is Builders Club only
    #[serde(rename = "isBuildersClubOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_builders_club_only: Option<bool>,
    /// Whether the group is deactivated
    #[serde(rename = "isDeactivated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_deactivated: Option<bool>,
    /// Whether the group is locked
    #[serde(rename = "isLocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
    /// The number of members in the group
    #[serde(rename = "memberCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i32>,
    /// The group name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<UserModel>,
    /// Whether the group is public (no approval required)
    #[serde(rename = "publicEntryAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_entry_allowed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shout: Option<ShoutResponse>,
}

/// `Roblox.Groups.Api.GroupEconomyPermissionsModel`
///
/// A model representing data about an Roblox.Platform.Membership.IUser
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupEconomyPermissionsModel {
    /// Add group places permission
    #[serde(rename = "addGroupPlaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_group_places: Option<bool>,
    /// Advertise group permission
    #[serde(rename = "advertiseGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advertise_group: Option<bool>,
    /// Create items permission
    #[serde(rename = "createItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_items: Option<bool>,
    /// Manage group games permission
    #[serde(rename = "manageGroupGames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage_group_games: Option<bool>,
    /// Manage items permission
    #[serde(rename = "manageItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage_items: Option<bool>,
    /// Spend group funds permission
    #[serde(rename = "spendGroupFunds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spend_group_funds: Option<bool>,
    /// Permission to view universe analytics from the creator dashboard.
    #[serde(rename = "viewAnalytics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_analytics: Option<bool>,
    /// Manage group games permission
    #[serde(rename = "viewGroupPayouts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_group_payouts: Option<bool>,
}

/// `Roblox.Groups.Api.GroupJoinRequestResponse`
///
/// Response model for a group join request
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupJoinRequestResponse {
    /// The DateTime the request was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester: Option<UserModel>,
}

/// `Roblox.Groups.Api.GroupManagementPermissionsModel`
///
/// A model representing data about an Roblox.Platform.Membership.IUser
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupManagementPermissionsModel {
    /// Bypass slowmode permission.
    #[serde(rename = "bypassSlowmode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_slowmode: Option<bool>,
    /// Manage clan permission
    #[serde(rename = "manageClan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage_clan: Option<bool>,
    /// Manage group relationships permission
    #[serde(rename = "manageRelationships")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage_relationships: Option<bool>,
    /// View audit logs permission
    #[serde(rename = "viewAuditLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_audit_logs: Option<bool>,
    /// View community analytics permission.
    #[serde(rename = "viewCommunityAnalytics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_community_analytics: Option<bool>,
}

crate::api_enum! {
    int
    /// \['AnnouncementCreatedNotification' = 0, 'ForumPostCreatedNotification' = 1, 'ForumCommentCreatedNotification' = 2, 'ForumCommentReplyCreatedNotification' = 3, 'ForumSubscriberNotification' = 4\]
    GroupNotificationPreferenceDataType {
    AnnouncementCreatedNotification = 0 => "AnnouncementCreatedNotification",
    ForumPostCreatedNotification = 1 => "ForumPostCreatedNotification",
    ForumCommentCreatedNotification = 2 => "ForumCommentCreatedNotification",
    ForumCommentReplyCreatedNotification = 3 => "ForumCommentReplyCreatedNotification",
    ForumSubscriberNotification = 4 => "ForumSubscriberNotification",
    }
}

/// `Roblox.Groups.Api.GroupNotificationPreferenceData`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupNotificationPreferenceData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// \['AnnouncementCreatedNotification' = 0, 'ForumPostCreatedNotification' = 1, 'ForumCommentCreatedNotification' = 2, 'ForumCommentReplyCreatedNotification' = 3, 'ForumSubscriberNotification' = 4\]
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<GroupNotificationPreferenceDataType>,
}

/// `Roblox.Groups.Api.GroupMembershipDetailResponse`
///
/// A group membership response model
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupMembershipDetailResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<GroupDetailResponse>,
    /// Whether the group notification preferences are enabled for the user
    #[serde(rename = "isNotificationsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_notifications_enabled: Option<bool>,
    /// Whether the group is the user's Primary Group
    #[serde(rename = "isPrimaryGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_primary_group: Option<bool>,
    #[serde(rename = "notificationPreferences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_preferences: Option<Vec<GroupNotificationPreferenceData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<GroupRoleResponse>,
}

/// `Roblox.Groups.Api.GroupMembershipPermissionsModel`
///
/// A model representing data about an Roblox.Platform.Membership.IUser
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupMembershipPermissionsModel {
    /// Ban from group permission
    #[serde(rename = "banMembers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ban_members: Option<bool>,
    /// Update a member's role
    #[serde(rename = "changeRank")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_rank: Option<bool>,
    /// Invite members to the group
    #[serde(rename = "inviteMembers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_members: Option<bool>,
    /// Remove from group permission
    #[serde(rename = "removeMembers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_members: Option<bool>,
}

/// `Roblox.Groups.Api.GroupOpenCloudPermissionsModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupOpenCloudPermissionsModel {
    /// Permission to administer all of the group's API keys.
    #[serde(rename = "administerCloudAuthentication")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administer_cloud_authentication: Option<bool>,
    /// Permission to create and use API keys on the group's resources.
    #[serde(rename = "useCloudAuthentication")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_cloud_authentication: Option<bool>,
}

/// `Roblox.Groups.Api.GroupPostsPermissionsModel`
///
/// A model representing group posts permissions
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupPostsPermissionsModel {
    /// Post to status permission
    #[serde(rename = "postToStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_to_status: Option<bool>,
    /// View status permission
    #[serde(rename = "viewStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_status: Option<bool>,
}

/// `Roblox.Groups.Api.GroupPermissionsModel`
///
/// A model for group permissions.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupPermissionsModel {
    #[serde(rename = "groupContentModerationPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_content_moderation_permissions: Option<GroupContentModerationPermissionsModel>,
    #[serde(rename = "groupEconomyPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_economy_permissions: Option<GroupEconomyPermissionsModel>,
    #[serde(rename = "groupForumsPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_forums_permissions: Option<GroupForumsPermissionsModel>,
    #[serde(rename = "groupManagementPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_management_permissions: Option<GroupManagementPermissionsModel>,
    #[serde(rename = "groupMembershipPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_membership_permissions: Option<GroupMembershipPermissionsModel>,
    #[serde(rename = "groupOpenCloudPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_open_cloud_permissions: Option<GroupOpenCloudPermissionsModel>,
    #[serde(rename = "groupPostsPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_posts_permissions: Option<GroupPostsPermissionsModel>,
}

/// `Roblox.Groups.Api.GroupMembershipMetadataResponse`
///
/// A user's group membership metadata response model
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupMembershipMetadataResponse {
    /// Whether enemies are allowed
    #[serde(rename = "areEnemiesAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub are_enemies_allowed: Option<bool>,
    /// Whether group funds are visible
    #[serde(rename = "areGroupFundsVisible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub are_group_funds_visible: Option<bool>,
    /// Whether group games are visible
    #[serde(rename = "areGroupGamesVisible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub are_group_games_visible: Option<bool>,
    /// If the user can configure the group
    #[serde(rename = "canConfigure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_configure: Option<bool>,
    /// Whether the user can view the group member list
    #[serde(rename = "canViewMemberList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_view_member_list: Option<bool>,
    /// The users's permissions for each communication channel in the group
    #[serde(rename = "channelPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_permissions: Option<Vec<GroupChannelPermissionsModel>>,
    /// The group id
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i64>,
    /// Whether the user is banned from the group
    #[serde(rename = "isBannedFromGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_banned_from_group: Option<bool>,
    /// Whether the group's notification preferences set to enabled for the user
    #[serde(rename = "isNotificationsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_notifications_enabled: Option<bool>,
    /// Whether the authenticated user is the owner of the group
    #[serde(rename = "isOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_owner: Option<bool>,
    /// Whether there has been a request to join this group
    #[serde(rename = "isPendingJoin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_pending_join: Option<bool>,
    /// Whether the group is primary
    #[serde(rename = "isPrimary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
    #[serde(rename = "notificationPreferences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_preferences: Option<Vec<GroupNotificationPreferenceData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<GroupPermissionsModel>,
    #[serde(rename = "userRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_role: Option<UserGroupRoleResponse>,
}

/// `Roblox.Groups.Api.GroupPayoutResponse`
///
/// A group payout response
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupPayoutResponse {
    /// The group payout percentage for the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserModel>,
}

/// `Roblox.Groups.Api.GroupPayoutRestrictionResponse`
///
/// Response model for Group Payout Restriction
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupPayoutRestrictionResponse {
    /// Whether the group can use one-time payout feature.
    #[serde(rename = "canUseOneTimePayout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_use_one_time_payout: Option<bool>,
    /// Whether the group can use recurring payout feature.
    #[serde(rename = "canUseRecurringPayout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_use_recurring_payout: Option<bool>,
}

/// `Roblox.Groups.Api.GroupPermissionsResponse`
///
/// A group role's permissions response model
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupPermissionsResponse {
    /// The group id
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<GroupPermissionsModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<GroupRoleResponse>,
}

/// `Roblox.Groups.Api.GroupPolicyResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupPolicyResponse {
    #[serde(rename = "canViewGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_view_group: Option<bool>,
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i64>,
}

/// `Roblox.Groups.Api.GroupPoliciesResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupPoliciesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<GroupPolicyResponse>>,
}

/// `Roblox.Groups.Api.GroupPolicyRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupPolicyRequest {
    #[serde(rename = "groupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<i64>>,
}

crate::api_enum! {
    int
    /// The group relationship type \['Allies' = 1, 'Enemies' = 2\]
    GroupRelationshipsResponseRelationshipType {
    Allies = 1 => "Allies",
    Enemies = 2 => "Enemies",
    }
}

/// `Roblox.Groups.Api.GroupRelationshipsResponse`
///
/// A group relationships response model
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupRelationshipsResponse {
    /// The group id
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i64>,
    /// The index for the next page of related groups
    #[serde(rename = "nextRowIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_row_index: Option<i64>,
    /// The related or requested groups
    #[serde(rename = "relatedGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_groups: Option<Vec<GroupDetailResponse>>,
    /// The group relationship type \['Allies' = 1, 'Enemies' = 2\]
    #[serde(rename = "relationshipType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_type: Option<GroupRelationshipsResponseRelationshipType>,
    /// The total number of groups for this relationship type
    #[serde(rename = "totalGroupCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_group_count: Option<i64>,
}

/// `Roblox.Groups.Api.GroupRoleDetailResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupRoleDetailResponse {
    /// The role color. \['Invalid' = 0, 'Blue' = 1, 'Green' = 2, 'Purple' = 3, 'Yellow' = 4, 'Orange' = 5, 'Red' = 6, 'Magenta' = 7, 'Teal' = 8, 'Turquoise' = 9, 'Rust' = 10, 'Pistachio' = 11, 'Midnight' = 12, 'Lavender' = 13, 'Pink' = 14, 'Crimson' = 15, 'Plum' = 16\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<GroupRoleResponseColor>,
    /// The role description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The id of the group the role belongs to
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i64>,
    /// The role id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Whether or not the role is the base role for the group
    #[serde(rename = "isBase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_base: Option<bool>,
    /// Whether the role is private (only visible to members with permission). Omitted from the response when false.
    #[serde(rename = "isPrivate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    /// The number of members in the role.
    #[serde(rename = "memberCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i64>,
    /// The role name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The role rank
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
}

/// `Roblox.Groups.Api.GroupSearchMetadataResponse`
///
/// Response Model For Group Search Metadata Endpoint
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupSearchMetadataResponse {
    /// Whether or not the Friends' Groups sort should show for the authenticated user
    #[serde(rename = "ShowFriendsGroupsSort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_friends_groups_sort: Option<bool>,
    /// Suggested Group Category translation keys
    #[serde(rename = "SuggestedGroupKeywords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_group_keywords: Option<Vec<String>>,
}

/// `Roblox.Groups.Api.GroupSearchResponseItem`
///
/// A group search response
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupSearchResponseItem {
    /// When the group was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// The group description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The group's verified badge status
    #[serde(rename = "hasVerifiedBadge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_verified_badge: Option<bool>,
    /// The group id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The number of members in the group
    #[serde(rename = "memberCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i32>,
    /// The group name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The previous name of the group
    #[serde(rename = "previousName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_name: Option<String>,
    /// When true anyone can join the group. When false manual approval is required to join the group
    #[serde(rename = "publicEntryAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_entry_allowed: Option<bool>,
    /// When the group was last updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

/// `Roblox.Groups.Api.GroupSearchPageResponse`
///
/// ApiPageResponse for catalog search
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupSearchPageResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GroupSearchResponseItem>>,
    /// Keyword used for search query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
    #[serde(rename = "totalResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_results: Option<i64>,
}

/// `Roblox.Groups.Api.GroupSettingsResponse`
///
/// Response model for Group Settings
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupSettingsResponse {
    /// The account tenure requirement for the group. Null if the account tenure requirement could not be determined due to an error.
    #[serde(rename = "accountTenureRequirement")]
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tenure_requirement: Option<i32>,
    /// Whether enemy club declarations are allowed.
    #[serde(rename = "areEnemiesAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub are_enemies_allowed: Option<bool>,
    /// Whether funds are publicly visible.
    #[serde(rename = "areGroupFundsVisible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub are_group_funds_visible: Option<bool>,
    /// Whether games are publicly visible.
    #[serde(rename = "areGroupGamesVisible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub are_group_games_visible: Option<bool>,
    #[serde(rename = "communityTier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub community_tier: Option<CommunityTierInfoResponse>,
    /// Whether this group still auto-assigns its lowest non-guest role to new members.
    #[serde(rename = "hasLegacyAutoAssignRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_legacy_auto_assign_role: Option<bool>,
    /// Whether public entry is allowed.
    #[serde(rename = "isApprovalRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_approval_required: Option<bool>,
    /// Whether automatic assignment of the lowest non-guest role is disabled for this group. For non-legacy groups (created after the multi-role cutoff), this is always true. For legacy groups, reflects the persisted GroupFeatureSettings value. Null when the value could not be determined.
    #[serde(rename = "isAutoAssignRoleDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_auto_assign_role_disabled: Option<bool>,
    /// Whether Builder's Club is required.
    #[serde(rename = "isBuildersClubRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_builders_club_required: Option<bool>,
    /// If the group name change feature is enabled for this group.
    #[serde(rename = "isGroupNameChangeEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_group_name_change_enabled: Option<bool>,
    /// Whether the group member list is visible to public.
    #[serde(rename = "isMemberListVisibleToPublic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_member_list_visible_to_public: Option<bool>,
    /// The slowmode level for the group (0-4). 0 = No slowmode, 1-4 = Slowmode levels with increasing restrictions.
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slowmode: Option<i32>,
    /// The verification level for the group. Null if the verification level could not be determined due to an error.
    #[serde(rename = "verificationLevel")]
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_level: Option<i32>,
}

/// `Roblox.Groups.Api.GroupsDisplayOptionsResponse`
///
/// A group roles response model
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupsDisplayOptionsResponse {
    /// Whether or not group bans are enabled
    #[serde(rename = "areGroupBansEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub are_group_bans_enabled: Option<bool>,
    /// If set to true, groups showcase will not show on users profiles. If set to false, group showcase will display on users profiles.
    #[serde(rename = "areProfileGroupsHidden")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub are_profile_groups_hidden: Option<bool>,
    /// Whether or not group notifications can be enabled
    #[serde(rename = "canEnableGroupNotifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_enable_group_notifications: Option<bool>,
    /// The user's current group membership count
    #[serde(rename = "currentGroupCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_group_count: Option<i32>,
    /// The user's builders club membership group limit
    #[serde(rename = "groupLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_limit: Option<i32>,
    /// The maximum length of a group status
    #[serde(rename = "groupStatusMaxLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_status_max_length: Option<i32>,
    /// If set to true, group details will respect GUAC policies for group details If set to false, group details will not respect GUAC policies
    #[serde(rename = "isGroupDetailsPolicyEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_group_details_policy_enabled: Option<bool>,
    /// Whether or not we should show previous names of this group
    #[serde(rename = "showPreviousGroupNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_previous_group_names: Option<bool>,
}

/// `Roblox.Groups.Api.HasGroupFeaturesBlockedResponse`
///
/// Response model for the features/status endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HasGroupFeaturesBlockedResponse {
    /// Whether the group has any active feature freezes.
    #[serde(rename = "hasFeaturesBlocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_features_blocked: Option<bool>,
}

/// `Roblox.Groups.Api.JoinGroupRequest`
///
/// A request model for joining group.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct JoinGroupRequest {
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
    #[serde(rename = "redemptionToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redemption_token: Option<String>,
    #[serde(rename = "sessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

/// `Roblox.Groups.Api.MembersRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MembersRequest {
    /// The user ids being either accepted or declined
    #[serde(rename = "UserIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<i64>>,
}

/// `Roblox.Groups.Api.Models.Request.CreateRoleSetRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CreateRoleSetRequest {
    /// The description of the roleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional. When true the role is only visible to members with permission to see private roles. Defaults to false (public). Only accessible to group owners.
    #[serde(rename = "isPrivate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    /// The name of the roleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The rank/positioning of the roleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
    /// Setting to use group funds or not.
    #[serde(rename = "usingGroupFunds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub using_group_funds: Option<bool>,
}

/// `Roblox.Groups.Api.Models.Request.UpdateRoleSetRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateRoleSetRequest {
    /// Optional. The color of the roleset (GroupRoleSetColorType value, e.g. 0 = Invalid, 1 = Blue). When omitted, only name and description are updated and the existing color is left unchanged. When set, name, description, and color are updated via UpdateGroupRoleSetProperties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<i32>,
    /// The description of the roleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional. When set, updates the visibility of the roleset. Only accessible to group owners. The base (guest) role cannot be made private.
    #[serde(rename = "isPrivate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    /// The name of the roleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The rank/positioning of the roleset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
}

/// `Roblox.Groups.Api.Models.Response.GroupNameHistoryResponseItem`
///
/// A group name history response model
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupNameHistoryResponseItem {
    /// Date the name change was applied
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// The group name before the change
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

crate::api_enum! {
    int
    /// One-time payout status. \['NotHeld' = 0, 'Held' = 1\]
    OneTimePayoutResponseStatus {
    NotHeld = 0 => "NotHeld",
    Held = 1 => "Held",
    }
}

/// `Roblox.Groups.Api.OneTimePayoutResponse`
///
/// Response model for one-time payout requests.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct OneTimePayoutResponse {
    /// One-time payout status. \['NotHeld' = 0, 'Held' = 1\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OneTimePayoutResponseStatus>,
}

crate::api_enum! {
    int
    /// The recipient type. \['User' = 0, 'Group' = 1\]
    PayoutRecipientRequestRecipientType {
    User = 0 => "User",
    Group = 1 => "Group",
    }
}

/// `Roblox.Groups.Api.PayoutRecipientRequest`
///
/// A request model for paying out Robux.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PayoutRecipientRequest {
    /// The amount to payout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// The recipient id.
    #[serde(rename = "recipientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_id: Option<i64>,
    /// The recipient type. \['User' = 0, 'Group' = 1\]
    #[serde(rename = "recipientType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_type: Option<PayoutRecipientRequestRecipientType>,
}

crate::api_enum! {
    int
    /// The Roblox.Groups.Api.PayoutType. \['FixedAmount' = 1, 'Percentage' = 2\]
    PayoutRequestPayoutType {
    FixedAmount = 1 => "FixedAmount",
    Percentage = 2 => "Percentage",
    }
}

crate::api_enum! {
    str
    /// \['Standard' = 1, 'O18Boosted' = 2\]
    WatermarkContributionRequestBalanceKey {
    Standard = 0 => "Standard",
    O18Boosted = 1 => "O18Boosted",
    }
}

/// `Roblox.Groups.Api.WatermarkContributionRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct WatermarkContributionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// \['Standard' = 1, 'O18Boosted' = 2\]
    #[serde(rename = "balanceKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_key: Option<WatermarkContributionRequestBalanceKey>,
}

/// `Roblox.Groups.Api.PayoutRequest`
///
/// Multi-payout request information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PayoutRequest {
    /// The idempotency key of the payout request.
    #[serde(rename = "IdempotencyKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    /// The Roblox.Groups.Api.PayoutType. \['FixedAmount' = 1, 'Percentage' = 2\]
    #[serde(rename = "PayoutType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_type: Option<PayoutRequestPayoutType>,
    /// The recipients of the payouts.
    #[serde(rename = "Recipients")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<PayoutRecipientRequest>>,
    /// Optional watermark contributions for DevEx attribution tracking.
    #[serde(rename = "WatermarkContributions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watermark_contributions: Option<Vec<WatermarkContributionRequest>>,
}

/// `Roblox.Groups.Api.PostGroupStatusRequest`
///
/// A request model for setting the authenticated user's primary group.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PostGroupStatusRequest {
    /// The message to set the group status to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// `Roblox.Groups.Api.PrimaryGroupRequest`
///
/// A request model for setting the authenticated user's primary group.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PrimaryGroupRequest {
    /// The group id.
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i64>,
}

/// `Roblox.Groups.Api.RelationshipsRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RelationshipsRequest {
    /// The group ids being either accepted or declined
    #[serde(rename = "GroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<i64>>,
}

crate::api_enum! {
    str
    /// The desired status of a group feature. \['On' = 0, 'Blocked' = 1\]
    SetFeaturesRequestModelFeaturesContentUpload {
    On = 0 => "On",
    Blocked = 1 => "Blocked",
    }
}

/// Dictionary of features and their desired status.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SetFeaturesRequestModelFeatures {
    /// The desired status of a group feature. \['On' = 0, 'Blocked' = 1\]
    #[serde(rename = "ContentUpload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_upload: Option<SetFeaturesRequestModelFeaturesContentUpload>,
    /// The desired status of a group feature. \['On' = 0, 'Blocked' = 1\]
    #[serde(rename = "ForumRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_read: Option<SetFeaturesRequestModelFeaturesContentUpload>,
    /// The desired status of a group feature. \['On' = 0, 'Blocked' = 1\]
    #[serde(rename = "ForumWrite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_write: Option<SetFeaturesRequestModelFeaturesContentUpload>,
    /// The desired status of a group feature. \['On' = 0, 'Blocked' = 1\]
    #[serde(rename = "GameOwnershipTransfer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_ownership_transfer: Option<SetFeaturesRequestModelFeaturesContentUpload>,
    /// The desired status of a group feature. \['On' = 0, 'Blocked' = 1\]
    #[serde(rename = "GroupOwnershipTransfer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ownership_transfer: Option<SetFeaturesRequestModelFeaturesContentUpload>,
    /// The desired status of a group feature. \['On' = 0, 'Blocked' = 1\]
    #[serde(rename = "Payouts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts: Option<SetFeaturesRequestModelFeaturesContentUpload>,
}

/// `Roblox.Groups.Api.SetFeaturesRequestModel`
///
/// Request model for setting the desired status of group features.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SetFeaturesRequestModel {
    /// Dictionary of features and their desired status.
    #[serde(rename = "Features")]
    pub features: SetFeaturesRequestModelFeatures,
}

/// `Roblox.Groups.Api.SetFeaturesResponseModel`
///
/// Response model for setting the desired status of group features.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SetFeaturesResponseModel {
    /// Whether any features were updated.
    #[serde(rename = "Updated")]
    pub updated: bool,
}

/// `Roblox.Groups.Api.SocialLinkRequest`
///
/// An update request for a social link
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SocialLinkRequest {
    /// The title of the link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// What type of social media this points to
    #[serde(rename = "type")]
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<i32>,
    /// The url of the link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// `Roblox.Groups.Api.UpdateBlockedKeywordRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateBlockedKeywordRequest {
    #[serde(rename = "isPrivate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
}

/// `Roblox.Groups.Api.UpdateGroupDescriptionRequest`
///
/// A request model for setting a description for the group
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateGroupDescriptionRequest {
    /// The group description being set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// `Roblox.Groups.Api.UpdateGroupNameRequest`
///
/// A request model for setting a name for the group
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateGroupNameRequest {
    /// The group name being set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.Groups.Api.UpdateGroupNameResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateGroupNameResponse {
    /// The new description returned
    #[serde(rename = "newName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_name: Option<String>,
}

/// `Roblox.Groups.Api.UpdateGroupNotificationPreferenceRequest`
///
/// A request model for updating a group's notification preference.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateGroupNotificationPreferenceRequest {
    /// Whether the user wants to receive notifications from the group.
    #[serde(rename = "notificationsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications_enabled: Option<bool>,
    /// \['AnnouncementCreatedNotification' = 0, 'ForumPostCreatedNotification' = 1, 'ForumCommentCreatedNotification' = 2, 'ForumCommentReplyCreatedNotification' = 3, 'ForumSubscriberNotification' = 4\]
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<GroupNotificationPreferenceDataType>,
}

/// `Roblox.Groups.Api.UpdateGroupSettingsRequest`
///
/// A request model for updating a group's settings.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateGroupSettingsRequest {
    /// The account tenure requirement for the group.
    #[serde(rename = "accountTenureRequirement")]
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tenure_requirement: Option<i32>,
    /// Whether enemy club declarations are allowed.
    #[serde(rename = "areEnemiesAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub are_enemies_allowed: Option<bool>,
    /// Whether funds are publicly visible.
    #[serde(rename = "areGroupFundsVisible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub are_group_funds_visible: Option<bool>,
    /// Whether games are publicly visible.
    #[serde(rename = "areGroupGamesVisible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub are_group_games_visible: Option<bool>,
    /// Whether public entry is allowed.
    #[serde(rename = "isApprovalRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_approval_required: Option<bool>,
    /// Whether the group member list is visible to public.
    #[serde(rename = "isMemberListVisibleToPublic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_member_list_visible_to_public: Option<bool>,
    /// The slowmode level for the group (0-4). 0 = No slowmode, 1-4 = Slowmode levels with increasing restrictions. Null = No change to current slowmode setting.
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slowmode: Option<i32>,
    /// The verification level for the group.
    #[serde(rename = "verificationLevel")]
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_level: Option<i32>,
}

/// The permission-value pairs to be updated.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdatePermissionsRequestPermissions {
    #[serde(rename = "AddGroupPlaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_group_places: Option<bool>,
    #[serde(rename = "AdministerCloudAuthentication")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administer_cloud_authentication: Option<bool>,
    #[serde(rename = "AdvertiseGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advertise_group: Option<bool>,
    #[serde(rename = "BanMembers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ban_members: Option<bool>,
    #[serde(rename = "BypassSlowmode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_slowmode: Option<bool>,
    #[serde(rename = "ChangeRank")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_rank: Option<bool>,
    #[serde(rename = "CreateBugReports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_bug_reports: Option<bool>,
    #[serde(rename = "CreateComments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_comments: Option<bool>,
    #[serde(rename = "CreateItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_items: Option<bool>,
    #[serde(rename = "CreatePosts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_posts: Option<bool>,
    #[serde(rename = "InviteMembers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_members: Option<bool>,
    #[serde(rename = "LockPosts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_posts: Option<bool>,
    #[serde(rename = "ManageCategories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage_categories: Option<bool>,
    #[serde(rename = "ManageClan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage_clan: Option<bool>,
    #[serde(rename = "ManageGroupGames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage_group_games: Option<bool>,
    #[serde(rename = "ManageItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage_items: Option<bool>,
    #[serde(rename = "ManageKeywordBlockList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage_keyword_block_list: Option<bool>,
    #[serde(rename = "ManageRelationships")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage_relationships: Option<bool>,
    #[serde(rename = "PinPosts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin_posts: Option<bool>,
    #[serde(rename = "PostToStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_to_status: Option<bool>,
    #[serde(rename = "RemoveComments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_comments: Option<bool>,
    #[serde(rename = "RemoveMembers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_members: Option<bool>,
    #[serde(rename = "RemovePosts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_posts: Option<bool>,
    #[serde(rename = "SpendGroupFunds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spend_group_funds: Option<bool>,
    #[serde(rename = "UseCloudAuthentication")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_cloud_authentication: Option<bool>,
    #[serde(rename = "ViewAnalytics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_analytics: Option<bool>,
    #[serde(rename = "ViewAuditLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_audit_logs: Option<bool>,
    #[serde(rename = "ViewCommunityAnalytics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_community_analytics: Option<bool>,
    #[serde(rename = "ViewForums")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_forums: Option<bool>,
    #[serde(rename = "ViewKeywordBlockList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_keyword_block_list: Option<bool>,
    #[serde(rename = "ViewStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_status: Option<bool>,
}

/// `Roblox.Groups.Api.UpdatePermissionsRequest`
///
/// A request model for updating a group's roleset's permissions.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdatePermissionsRequest {
    /// The permission-value pairs to be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<UpdatePermissionsRequestPermissions>,
}

/// `Roblox.Groups.Api.UpdateUserRoleRequest`
///
/// A request model for setting a users role in a group.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateUserRoleRequest {
    /// The role in the group the user should be put into.
    #[serde(rename = "roleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<i64>,
}

/// `Roblox.Groups.Api.UserGroupMembershipResponse`
///
/// A users group membership response model
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UserGroupMembershipResponse {
    /// The list of group memberships
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<GroupMembershipDetailResponse>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserModel>,
}

/// `Roblox.Groups.Client.CreateBlockedKeywordsResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CreateBlockedKeywordsResponse {
    #[serde(rename = "createdKeywords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_keywords: Option<Vec<BlockedKeywordModel>>,
    #[serde(rename = "hadDuplicateKeywords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub had_duplicate_keywords: Option<bool>,
    #[serde(rename = "hadModeratedKeywords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub had_moderated_keywords: Option<bool>,
}

/// `Roblox.Groups.Client.EmoteModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EmoteModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.Groups.Client.EmoteSetModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EmoteSetModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emotes: Option<Vec<EmoteModel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.Groups.Client.GetGroupEmoteSetsResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetGroupEmoteSetsResponse {
    #[serde(rename = "emoteSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emote_sets: Option<Vec<EmoteSetModel>>,
}

/// `Roblox.Groups.Client.GroupFeaturedContentResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupFeaturedContentResponse {
    #[serde(rename = "contentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_id: Option<String>,
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i64>,
}

/// `Roblox.Groups.Client.TierEvaluationResultResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TierEvaluationResultResponse {
    #[serde(rename = "passedSignals")]
    pub passed_signals: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<Vec<TierRequirement>>,
    #[serde(rename = "tierInfo")]
    pub tier_info: CommunityTierInfoResponse,
}

/// `Roblox.Web.Responses.Groups.GroupBasicResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupBasicResponse {
    #[serde(rename = "hasVerifiedBadge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_verified_badge: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "memberCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

crate::api_enum! {
    int
    /// \['User' = 0\]
    RelatedEntityTypeResponseGroupOwnerTypeType {
    User = 0 => "User",
    }
}

/// `Roblox.Web.Responses.RelatedEntityTypeResponse\[Roblox.Web.Responses.Groups.GroupOwnerType\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RelatedEntityTypeResponseGroupOwnerType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// \['User' = 0\]
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<RelatedEntityTypeResponseGroupOwnerTypeType>,
}

/// `Roblox.Web.Responses.Groups.GroupResponseV2`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupResponseV2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "hasVerifiedBadge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_verified_badge: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "memberCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<RelatedEntityTypeResponseGroupOwnerType>,
}

/// `Roblox.Web.WebAPI.ApiEmptyResponseModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiEmptyResponseModel {}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.Groups.Api.GroupDetailResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseGroupDetailResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GroupDetailResponse>>,
}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.Groups.Api.GroupMembershipDetailResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseGroupMembershipDetailResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GroupMembershipDetailResponse>>,
}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.Groups.Api.GroupPayoutResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseGroupPayoutResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GroupPayoutResponse>>,
}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.Groups.Api.GroupPermissionsResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseGroupPermissionsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GroupPermissionsResponse>>,
}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.Groups.Api.GroupRoleDetailResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseGroupRoleDetailResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GroupRoleDetailResponse>>,
}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.Groups.Api.UserGroupMembershipResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseUserGroupMembershipResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<UserGroupMembershipResponse>>,
}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.Web.Responses.Groups.GroupBasicResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseGroupBasicResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GroupBasicResponse>>,
}

/// `Roblox.Web.WebAPI.Models.ApiPageResponse\[Roblox.Groups.Api.GroupBanMemberResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiPageResponseGroupBanMemberResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GroupBanMemberResponse>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
}

/// `Roblox.Web.WebAPI.Models.ApiPageResponse\[Roblox.Groups.Api.GroupJoinRequestResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiPageResponseGroupJoinRequestResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GroupJoinRequestResponse>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
}

/// `Roblox.Web.WebAPI.Models.ApiPageResponse\[Roblox.Groups.Api.Models.Response.GroupNameHistoryResponseItem\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiPageResponseGroupNameHistoryResponseItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GroupNameHistoryResponseItem>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
}

/// `Roblox.Web.WebAPI.Models.ApiPageResponse\[Roblox.Groups.Api.Models.Response.UserModel\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiPageResponseUserModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<UserModel>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
}

/// `Roblox.Web.WebAPI.Models.ApiPageResponse\[Roblox.Groups.Api.UserGroupRoleResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiPageResponseUserGroupRoleResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<UserGroupRoleResponse>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
}

/// Query, header and form inputs for [`get_featured_content_event`].
#[derive(Debug, Clone, Default)]
pub struct GetFeaturedContentEventParams {
    /// The group Id.
    pub group_id: i64,
}

/// Query, header and form inputs for [`post_featured_content_event`].
#[derive(Debug, Clone, Default)]
pub struct PostFeaturedContentEventParams {
    /// The group Id.
    pub group_id: i64,
    /// The event Id.
    pub event_id: i64,
}

/// Query, header and form inputs for [`delete_featured_content_event`].
#[derive(Debug, Clone, Default)]
pub struct DeleteFeaturedContentEventParams {
    /// The group Id.
    pub group_id: i64,
    /// The event Id.
    pub event_id: i64,
}

/// Query, header and form inputs for [`post_groups_create`].
#[derive(Debug, Clone, Default)]
pub struct PostGroupsCreateParams {
    /// The name of the group.
    pub name: Option<String>,
    /// The group description.
    pub description: Option<String>,
    /// Whether or not the group is open for anyone to join.
    pub public_group: Option<bool>,
    /// Whether or not the group is only open to join for builders club members.
    pub builders_club_members_only: Option<bool>,
    pub files: Option<FilePart>,
}

/// Query, header and form inputs for [`patch_groups_icon`].
#[derive(Debug, Clone, Default)]
pub struct PatchGroupsIconParams {
    /// The group Id.
    pub group_id: i64,
    pub files: Option<FilePart>,
}

/// Query, header and form inputs for [`get_groups_search`].
#[derive(Debug, Clone, Default)]
pub struct GetGroupsSearchParams {
    /// The keyword or phrase to use as the search parameter.
    pub keyword: String,
    /// Whether or not to prioritize the exact match for the keyword (optional, defaults to false).
    pub prioritize_exact_match: Option<bool>,
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
}

/// Query, header and form inputs for [`get_groups_search_lookup`].
#[derive(Debug, Clone, Default)]
pub struct GetGroupsSearchLookupParams {
    /// The group name.
    pub group_name: String,
}

crate::api_enum! {
    str
    /// Filter for specific type of action performed
    GetGroupsByGroupIdAuditLogActionType {
    DeletePost = 0 => "DeletePost",
    RemoveMember = 1 => "RemoveMember",
    AcceptJoinRequest = 2 => "AcceptJoinRequest",
    DeclineJoinRequest = 3 => "DeclineJoinRequest",
    PostStatus = 4 => "PostStatus",
    ChangeRank = 5 => "ChangeRank",
    BuyAd = 6 => "BuyAd",
    SendAllyRequest = 7 => "SendAllyRequest",
    CreateEnemy = 8 => "CreateEnemy",
    AcceptAllyRequest = 9 => "AcceptAllyRequest",
    DeclineAllyRequest = 10 => "DeclineAllyRequest",
    DeleteAlly = 11 => "DeleteAlly",
    DeleteEnemy = 12 => "DeleteEnemy",
    AddGroupPlace = 13 => "AddGroupPlace",
    RemoveGroupPlace = 14 => "RemoveGroupPlace",
    CreateItems = 15 => "CreateItems",
    ConfigureItems = 16 => "ConfigureItems",
    SpendGroupFunds = 17 => "SpendGroupFunds",
    ChangeOwner = 18 => "ChangeOwner",
    Delete = 19 => "Delete",
    AdjustCurrencyAmounts = 20 => "AdjustCurrencyAmounts",
    Abandon = 21 => "Abandon",
    Claim = 22 => "Claim",
    Rename = 23 => "Rename",
    ChangeDescription = 24 => "ChangeDescription",
    InviteToClan = 25 => "InviteToClan",
    KickFromClan = 26 => "KickFromClan",
    CancelClanInvite = 27 => "CancelClanInvite",
    BuyClan = 28 => "BuyClan",
    CreateGroupAsset = 29 => "CreateGroupAsset",
    UpdateGroupAsset = 30 => "UpdateGroupAsset",
    ConfigureGroupAsset = 31 => "ConfigureGroupAsset",
    RevertGroupAsset = 32 => "RevertGroupAsset",
    CreateGroupDeveloperProduct = 33 => "CreateGroupDeveloperProduct",
    ConfigureGroupGame = 34 => "ConfigureGroupGame",
    CreateGroupDeveloperSubscriptionProduct = 35 => "CreateGroupDeveloperSubscriptionProduct",
    Lock = 36 => "Lock",
    Unlock = 37 => "Unlock",
    CreateGamePass = 38 => "CreateGamePass",
    CreateBadge = 39 => "CreateBadge",
    ConfigureBadge = 40 => "ConfigureBadge",
    SavePlace = 41 => "SavePlace",
    PublishPlace = 42 => "PublishPlace",
    UpdateRolesetRank = 43 => "UpdateRolesetRank",
    UpdateRolesetData = 44 => "UpdateRolesetData",
    UpdateRoleSetPosition = 45 => "UpdateRoleSetPosition",
    BanMember = 46 => "BanMember",
    UnbanMember = 47 => "UnbanMember",
    CreateForumCategory = 48 => "CreateForumCategory",
    UpdateForumCategory = 49 => "UpdateForumCategory",
    ArchiveForumCategory = 50 => "ArchiveForumCategory",
    DeleteForumCategory = 51 => "DeleteForumCategory",
    DeleteForumPost = 52 => "DeleteForumPost",
    DeleteForumComment = 53 => "DeleteForumComment",
    PinForumPost = 54 => "PinForumPost",
    UnpinForumPost = 55 => "UnpinForumPost",
    LockForumPost = 56 => "LockForumPost",
    UnlockForumPost = 57 => "UnlockForumPost",
    CreateRoleset = 58 => "CreateRoleset",
    DeleteRoleset = 59 => "DeleteRoleset",
    CreateCommerceProduct = 60 => "CreateCommerceProduct",
    SetCommerceProductActive = 61 => "SetCommerceProductActive",
    ArchiveCommerceProduct = 62 => "ArchiveCommerceProduct",
    AcceptCommerceProductBundlingFee = 63 => "AcceptCommerceProductBundlingFee",
    SetCommerceProductInactive = 64 => "SetCommerceProductInactive",
    ConnectMerchant = 65 => "ConnectMerchant",
    DisconnectMerchant = 66 => "DisconnectMerchant",
    JoinGroup = 67 => "JoinGroup",
    LeaveGroup = 68 => "LeaveGroup",
    UpdateGroupIcon = 69 => "UpdateGroupIcon",
    UpdateGroupCoverPhoto = 70 => "UpdateGroupCoverPhoto",
    AssignRole = 71 => "AssignRole",
    UnassignRole = 72 => "UnassignRole",
    PublishAnnouncement = 73 => "PublishAnnouncement",
    DeleteAnnouncement = 74 => "DeleteAnnouncement",
    UpdateRoleSetPermissions = 75 => "UpdateRoleSetPermissions",
    UpdateGroupSecuritySettings = 76 => "UpdateGroupSecuritySettings",
    GrantEnterpriseTier = 77 => "GrantEnterpriseTier",
    RevokeEnterpriseTier = 78 => "RevokeEnterpriseTier",
    DeactivateGroup = 79 => "DeactivateGroup",
    }
}

crate::api_enum! {
    str
    /// The order the results are sorted in.
    GetGroupsByGroupIdAuditLogSortOrder {
    Asc = 0 => "Asc",
    Desc = 1 => "Desc",
    }
}

/// Query, header and form inputs for [`get_groups_by_group_id_audit_log`].
#[derive(Debug, Clone, Default)]
pub struct GetGroupsByGroupIdAuditLogParams {
    /// Filter for specific type of action performed
    pub action_type: Option<GetGroupsByGroupIdAuditLogActionType>,
    /// Filter for specific user id
    pub user_id: Option<i64>,
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// The order the results are sorted in.
    pub sort_order: Option<GetGroupsByGroupIdAuditLogSortOrder>,
}

/// Query, header and form inputs for [`get_groups_by_group_id_bans`].
#[derive(Debug, Clone, Default)]
pub struct GetGroupsByGroupIdBansParams {
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// The order the results are sorted in.
    pub sort_order: Option<GetGroupsByGroupIdAuditLogSortOrder>,
}

/// Query, header and form inputs for [`get_groups_by_group_id_blocked_keywords`].
#[derive(Debug, Clone, Default)]
pub struct GetGroupsByGroupIdBlockedKeywordsParams {
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// The order the results are sorted in.
    pub sort_order: Option<GetGroupsByGroupIdAuditLogSortOrder>,
}

/// Query, header and form inputs for [`get_groups_by_group_id_join_requests`].
#[derive(Debug, Clone, Default)]
pub struct GetGroupsByGroupIdJoinRequestsParams {
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// The order the results are sorted in.
    pub sort_order: Option<GetGroupsByGroupIdAuditLogSortOrder>,
}

/// Query, header and form inputs for [`get_groups_by_group_id_membership`].
#[derive(Debug, Clone, Default)]
pub struct GetGroupsByGroupIdMembershipParams {
    pub include_notification_preferences: bool,
}

/// Query, header and form inputs for [`get_groups_by_group_id_name_history`].
#[derive(Debug, Clone, Default)]
pub struct GetGroupsByGroupIdNameHistoryParams {
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// The order the results are sorted in.
    pub sort_order: Option<GetGroupsByGroupIdAuditLogSortOrder>,
}

crate::api_enum! {
    int
    /// \['Unchanged' = 0, 'Created' = 1, 'Deleted' = 2, 'Updated' = 3\]
    PatchGroupsByGroupIdNotificationPreferenceResponse {
    Unchanged = 0 => "Unchanged",
    Created = 1 => "Created",
    Deleted = 2 => "Deleted",
    Updated = 3 => "Updated",
    }
}

/// Query, header and form inputs for [`get_groups_by_group_id_relationships_by_group_relationship_type`].
#[derive(Debug, Clone, Default)]
pub struct GetGroupsByGroupIdRelationshipsByGroupRelationshipTypeParams {
    /// The start index of the page request
    pub start_row_index: i32,
    /// The maximum number of rows for the page request, should be at least 1.
    pub max_rows: i32,
}

/// Query, header and form inputs for [`get_groups_by_group_id_relationships_by_group_relationship_type_requests`].
#[derive(Debug, Clone, Default)]
pub struct GetGroupsByGroupIdRelationshipsByGroupRelationshipTypeRequestsParams {
    /// The start index of the page request
    pub start_row_index: i32,
    /// The maximum number of rows for the page request, should be at least 1.
    pub max_rows: i32,
}

/// Query, header and form inputs for [`get_groups_by_group_id_roles`].
#[derive(Debug, Clone, Default)]
pub struct GetGroupsByGroupIdRolesParams {
    pub include_private: bool,
}

/// Query, header and form inputs for [`get_groups_by_group_id_roles_by_role_set_id_users`].
#[derive(Debug, Clone, Default)]
pub struct GetGroupsByGroupIdRolesByRoleSetIdUsersParams {
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// The order the results are sorted in.
    pub sort_order: Option<GetGroupsByGroupIdAuditLogSortOrder>,
}

/// Query, header and form inputs for [`get_groups_by_group_id_users`].
#[derive(Debug, Clone, Default)]
pub struct GetGroupsByGroupIdUsersParams {
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// The order the results are sorted in.
    pub sort_order: Option<GetGroupsByGroupIdAuditLogSortOrder>,
}

/// Query, header and form inputs for [`post_groups_by_group_id_users`].
#[derive(Debug, Clone, Default)]
pub struct PostGroupsByGroupIdUsersParams {
    /// The place ID of the experience the player is in.
    pub roblox_place_id: Option<i64>,
    /// The player's current game Id.
    pub roblox_game_id: Option<String>,
    /// The player's current session Id.
    pub roblox_session_id: Option<String>,
}

/// Query, header and form inputs for [`get_roles`].
#[derive(Debug, Clone, Default)]
pub struct GetRolesParams {
    /// A list of role ids
    pub ids: Vec<i64>,
    pub include_private: bool,
}

/// Query, header and form inputs for [`get_users_by_user_id_groups_roles`].
#[derive(Debug, Clone, Default)]
pub struct GetUsersByUserIdGroupsRolesParams {
    pub include_locked: Option<bool>,
    pub include_notification_preferences: Option<bool>,
    pub discovery_type: Option<i32>,
}

/// `GET /v1/featured-content/event`
///
/// Gets the featured event for a group
pub fn get_featured_content_event_request(params: &GetFeaturedContentEventParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/featured-content/event".to_string());
    r.query("groupId", &params.group_id);
    Ok(r)
}

/// `GET /v1/featured-content/event`
///
/// Gets the featured event for a group
pub async fn get_featured_content_event(
    client: &Client,
    params: &GetFeaturedContentEventParams,
) -> Result<GroupFeaturedContentResponse> {
    client
        .send(get_featured_content_event_request(params)?)
        .await?
        .json()
}

/// `POST /v1/featured-content/event`
///
/// Sets the featured event for a group
pub fn post_featured_content_event_request(params: &PostFeaturedContentEventParams) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/featured-content/event".to_string());
    r.query("groupId", &params.group_id);
    r.query("eventId", &params.event_id);
    Ok(r)
}

/// `POST /v1/featured-content/event`
///
/// Sets the featured event for a group
pub async fn post_featured_content_event(
    client: &Client,
    params: &PostFeaturedContentEventParams,
) -> Result<GroupFeaturedContentResponse> {
    client
        .send(post_featured_content_event_request(params)?)
        .await?
        .json()
}

/// `DELETE /v1/featured-content/event`
///
/// Deletes the featured event for a group
pub fn delete_featured_content_event_request(params: &DeleteFeaturedContentEventParams) -> Result<Request> {
    let mut r = Request::new(Method::DELETE, HOST, "/v1/featured-content/event".to_string());
    r.query("groupId", &params.group_id);
    r.query("eventId", &params.event_id);
    Ok(r)
}

/// `DELETE /v1/featured-content/event`
///
/// Deletes the featured event for a group
pub async fn delete_featured_content_event(
    client: &Client,
    params: &DeleteFeaturedContentEventParams,
) -> Result<()> {
    client
        .send(delete_featured_content_event_request(params)?)
        .await?
        .unit()
}

/// `GET /v1/groups/configuration/metadata`
///
/// Gets Group configuration contextual information
pub fn get_groups_configuration_metadata_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/groups/configuration/metadata".to_string());
    Ok(r)
}

/// `GET /v1/groups/configuration/metadata`
///
/// Gets Group configuration contextual information
pub async fn get_groups_configuration_metadata(
    client: &Client,
) -> Result<GroupConfigurationDisplayOptionsResponse> {
    client
        .send(get_groups_configuration_metadata_request()?)
        .await?
        .json()
}

/// `POST /v1/groups/create`
///
/// Creates a new group.
///
/// This endpoint will charge Robux for the group purchase. Accepts "icon" and "coverPhoto" in Files object. Defaults to first file if "icon" is not present. Http status code 413 is thrown when the group icon file size is too large.
pub fn post_groups_create_request(params: &PostGroupsCreateParams) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/groups/create".to_string());
    r.form_text("name", &params.name);
    r.form_text("description", &params.description);
    r.form_text("publicGroup", &params.public_group);
    r.form_text("buildersClubMembersOnly", &params.builders_club_members_only);
    r.form_file("Files", &params.files);
    Ok(r)
}

/// `POST /v1/groups/create`
///
/// Creates a new group.
///
/// This endpoint will charge Robux for the group purchase. Accepts "icon" and "coverPhoto" in Files object. Defaults to first file if "icon" is not present. Http status code 413 is thrown when the group icon file size is too large.
pub async fn post_groups_create(client: &Client, params: &PostGroupsCreateParams) -> Result<GroupResponseV2> {
    client.send(post_groups_create_request(params)?).await?.json()
}

/// `PATCH /v1/groups/icon`
///
/// Updates the group icon.
pub fn patch_groups_icon_request(params: &PatchGroupsIconParams) -> Result<Request> {
    let mut r = Request::new(Method::PATCH, HOST, "/v1/groups/icon".to_string());
    r.query("groupId", &params.group_id);
    r.form_file("Files", &params.files);
    Ok(r)
}

/// `PATCH /v1/groups/icon`
///
/// Updates the group icon.
pub async fn patch_groups_icon(
    client: &Client,
    params: &PatchGroupsIconParams,
) -> Result<ApiEmptyResponseModel> {
    client.send(patch_groups_icon_request(params)?).await?.json()
}

/// `GET /v1/groups/metadata`
///
/// Gets Groups contextual information: Max number of groups a user can be part of. Current number of groups a user is a member of. Whether to show/hide certain features based on device type.
pub fn get_groups_metadata_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/groups/metadata".to_string());
    Ok(r)
}

/// `GET /v1/groups/metadata`
///
/// Gets Groups contextual information: Max number of groups a user can be part of. Current number of groups a user is a member of. Whether to show/hide certain features based on device type.
pub async fn get_groups_metadata(client: &Client) -> Result<GroupsDisplayOptionsResponse> {
    client.send(get_groups_metadata_request()?).await?.json()
}

/// `POST /v1/groups/policies`
///
/// Gets group policy info used for compliance.
pub fn post_groups_policies_request(body: &GroupPolicyRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/groups/policies".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/groups/policies`
///
/// Gets group policy info used for compliance.
pub async fn post_groups_policies(
    client: &Client,
    body: &GroupPolicyRequest,
) -> Result<GroupPoliciesResponse> {
    client.send(post_groups_policies_request(body)?).await?.json()
}

/// `GET /v1/groups/search`
///
/// Search for groups by keyword.
pub fn get_groups_search_request(params: &GetGroupsSearchParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/groups/search".to_string());
    r.query("keyword", &params.keyword);
    r.query("prioritizeExactMatch", &params.prioritize_exact_match);
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    Ok(r)
}

/// `GET /v1/groups/search`
///
/// Search for groups by keyword.
pub async fn get_groups_search(
    client: &Client,
    params: &GetGroupsSearchParams,
) -> Result<GroupSearchPageResponse> {
    client.send(get_groups_search_request(params)?).await?.json()
}

/// `GET /v1/groups/search/lookup`
///
/// Looks up groups by a name. Prioritizes an exact match as the first result.
///
/// Should only be used for direct lookups where a user is inputting a group name, shouldn't be used for search pages.
pub fn get_groups_search_lookup_request(params: &GetGroupsSearchLookupParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/groups/search/lookup".to_string());
    r.query("groupName", &params.group_name);
    Ok(r)
}

/// `GET /v1/groups/search/lookup`
///
/// Looks up groups by a name. Prioritizes an exact match as the first result.
///
/// Should only be used for direct lookups where a user is inputting a group name, shouldn't be used for search pages.
pub async fn get_groups_search_lookup(
    client: &Client,
    params: &GetGroupsSearchLookupParams,
) -> Result<ApiArrayResponseGroupBasicResponse> {
    client
        .send(get_groups_search_lookup_request(params)?)
        .await?
        .json()
}

/// `GET /v1/groups/search/metadata`
///
/// Get suggested groups and other miscellaneous information needed for the group/join page like flags
///
/// Although there is no reason for this to require an authenticated user right now, in the future, we will use coco to return different suggested groups based upon that user's request context
pub fn get_groups_search_metadata_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/groups/search/metadata".to_string());
    Ok(r)
}

/// `GET /v1/groups/search/metadata`
///
/// Get suggested groups and other miscellaneous information needed for the group/join page like flags
///
/// Although there is no reason for this to require an authenticated user right now, in the future, we will use coco to return different suggested groups based upon that user's request context
pub async fn get_groups_search_metadata(client: &Client) -> Result<GroupSearchMetadataResponse> {
    client.send(get_groups_search_metadata_request()?).await?.json()
}

/// `GET /v1/groups/{groupId}`
///
/// Gets group information
pub fn get_groups_by_group_id_request(group_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/groups/{}", crate::rt::path_value(&group_id)),
    );
    Ok(r)
}

/// `GET /v1/groups/{groupId}`
///
/// Gets group information
pub async fn get_groups_by_group_id(client: &Client, group_id: i64) -> Result<GroupDetailResponse> {
    client
        .send(get_groups_by_group_id_request(group_id)?)
        .await?
        .json()
}

/// `GET /v1/groups/{groupId}/audit-log`
///
/// Gets the Group's audit log
pub fn get_groups_by_group_id_audit_log_request(
    group_id: i64,
    params: &GetGroupsByGroupIdAuditLogParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/groups/{}/audit-log", crate::rt::path_value(&group_id)),
    );
    r.query("actionType", &params.action_type);
    r.query("userId", &params.user_id);
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    r.query("sortOrder", &params.sort_order);
    Ok(r)
}

/// `GET /v1/groups/{groupId}/audit-log`
///
/// Gets the Group's audit log
pub async fn get_groups_by_group_id_audit_log(
    client: &Client,
    group_id: i64,
    params: &GetGroupsByGroupIdAuditLogParams,
) -> Result<GroupAuditLogPageResponseGroupAuditLogResponseItem> {
    client
        .send(get_groups_by_group_id_audit_log_request(group_id, params)?)
        .await?
        .json()
}

/// `GET /v1/groups/{groupId}/bans`
///
/// Gets the bans for the group
pub fn get_groups_by_group_id_bans_request(
    group_id: i64,
    params: &GetGroupsByGroupIdBansParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/groups/{}/bans", crate::rt::path_value(&group_id)),
    );
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    r.query("sortOrder", &params.sort_order);
    Ok(r)
}

/// `GET /v1/groups/{groupId}/bans`
///
/// Gets the bans for the group
pub async fn get_groups_by_group_id_bans(
    client: &Client,
    group_id: i64,
    params: &GetGroupsByGroupIdBansParams,
) -> Result<ApiPageResponseGroupBanMemberResponse> {
    client
        .send(get_groups_by_group_id_bans_request(group_id, params)?)
        .await?
        .json()
}

/// `GET /v1/groups/{groupId}/bans/{userId}`
///
/// Fetch the group ban of a user from a group
pub fn get_groups_by_group_id_bans_by_user_id_request(group_id: i64, user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/groups/{}/bans/{}",
            crate::rt::path_value(&group_id),
            crate::rt::path_value(&user_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/groups/{groupId}/bans/{userId}`
///
/// Fetch the group ban of a user from a group
pub async fn get_groups_by_group_id_bans_by_user_id(
    client: &Client,
    group_id: i64,
    user_id: i64,
) -> Result<GroupBanMemberResponse> {
    client
        .send(get_groups_by_group_id_bans_by_user_id_request(group_id, user_id)?)
        .await?
        .json()
}

/// `POST /v1/groups/{groupId}/bans/{userId}`
///
/// Bans a user from a group
pub fn post_groups_by_group_id_bans_by_user_id_request(group_id: i64, user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/groups/{}/bans/{}",
            crate::rt::path_value(&group_id),
            crate::rt::path_value(&user_id)
        ),
    );
    Ok(r)
}

/// `POST /v1/groups/{groupId}/bans/{userId}`
///
/// Bans a user from a group
pub async fn post_groups_by_group_id_bans_by_user_id(
    client: &Client,
    group_id: i64,
    user_id: i64,
) -> Result<GroupBanMemberResponse> {
    client
        .send(post_groups_by_group_id_bans_by_user_id_request(
            group_id, user_id,
        )?)
        .await?
        .json()
}

/// `DELETE /v1/groups/{groupId}/bans/{userId}`
///
/// Unbans a user from a group
pub fn delete_groups_by_group_id_bans_by_user_id_request(group_id: i64, user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::DELETE,
        HOST,
        format!(
            "/v1/groups/{}/bans/{}",
            crate::rt::path_value(&group_id),
            crate::rt::path_value(&user_id)
        ),
    );
    Ok(r)
}

/// `DELETE /v1/groups/{groupId}/bans/{userId}`
///
/// Unbans a user from a group
pub async fn delete_groups_by_group_id_bans_by_user_id(
    client: &Client,
    group_id: i64,
    user_id: i64,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(delete_groups_by_group_id_bans_by_user_id_request(
            group_id, user_id,
        )?)
        .await?
        .json()
}

/// `GET /v1/groups/{groupId}/blocked-keywords`
///
/// Retrieves a paginated list of blocked keywords for a specific Group.
pub fn get_groups_by_group_id_blocked_keywords_request(
    group_id: i64,
    params: &GetGroupsByGroupIdBlockedKeywordsParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/groups/{}/blocked-keywords", crate::rt::path_value(&group_id)),
    );
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    r.query("sortOrder", &params.sort_order);
    Ok(r)
}

/// `GET /v1/groups/{groupId}/blocked-keywords`
///
/// Retrieves a paginated list of blocked keywords for a specific Group.
pub async fn get_groups_by_group_id_blocked_keywords(
    client: &Client,
    group_id: i64,
    params: &GetGroupsByGroupIdBlockedKeywordsParams,
) -> Result<BlockedKeywordPageResponseBlockedKeywordModel> {
    client
        .send(get_groups_by_group_id_blocked_keywords_request(group_id, params)?)
        .await?
        .json()
}

/// `POST /v1/groups/{groupId}/blocked-keywords`
///
/// Adds new blocked keyword(s) to the specified Group.
pub fn post_groups_by_group_id_blocked_keywords_request(
    group_id: i64,
    body: &CreateBlockedKeywordsRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!("/v1/groups/{}/blocked-keywords", crate::rt::path_value(&group_id)),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/groups/{groupId}/blocked-keywords`
///
/// Adds new blocked keyword(s) to the specified Group.
pub async fn post_groups_by_group_id_blocked_keywords(
    client: &Client,
    group_id: i64,
    body: &CreateBlockedKeywordsRequest,
) -> Result<CreateBlockedKeywordsResponse> {
    client
        .send(post_groups_by_group_id_blocked_keywords_request(group_id, body)?)
        .await?
        .json()
}

/// `PATCH /v1/groups/{groupId}/blocked-keywords/{keywordId}`
///
/// Updates an existing blocked keyword for the specified Group.
pub fn patch_groups_by_group_id_blocked_keywords_by_keyword_id_request(
    group_id: i64,
    keyword_id: &str,
    body: &UpdateBlockedKeywordRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!(
            "/v1/groups/{}/blocked-keywords/{}",
            crate::rt::path_value(&group_id),
            crate::rt::path_value(&keyword_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/groups/{groupId}/blocked-keywords/{keywordId}`
///
/// Updates an existing blocked keyword for the specified Group.
pub async fn patch_groups_by_group_id_blocked_keywords_by_keyword_id(
    client: &Client,
    group_id: i64,
    keyword_id: &str,
    body: &UpdateBlockedKeywordRequest,
) -> Result<BlockedKeywordModel> {
    client
        .send(patch_groups_by_group_id_blocked_keywords_by_keyword_id_request(
            group_id, keyword_id, body,
        )?)
        .await?
        .json()
}

/// `DELETE /v1/groups/{groupId}/blocked-keywords/{keywordId}`
///
/// Deletes a specific blocked keyword from the specified Group.
pub fn delete_groups_by_group_id_blocked_keywords_by_keyword_id_request(
    group_id: i64,
    keyword_id: &str,
) -> Result<Request> {
    let r = Request::new(
        Method::DELETE,
        HOST,
        format!(
            "/v1/groups/{}/blocked-keywords/{}",
            crate::rt::path_value(&group_id),
            crate::rt::path_value(&keyword_id)
        ),
    );
    Ok(r)
}

/// `DELETE /v1/groups/{groupId}/blocked-keywords/{keywordId}`
///
/// Deletes a specific blocked keyword from the specified Group.
pub async fn delete_groups_by_group_id_blocked_keywords_by_keyword_id(
    client: &Client,
    group_id: i64,
    keyword_id: &str,
) -> Result<()> {
    client
        .send(delete_groups_by_group_id_blocked_keywords_by_keyword_id_request(
            group_id, keyword_id,
        )?)
        .await?
        .unit()
}

/// `POST /v1/groups/{groupId}/change-owner`
///
/// Changes the group owner to another user.
pub fn post_groups_by_group_id_change_owner_request(
    group_id: i64,
    body: &ChangeOwnerRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!("/v1/groups/{}/change-owner", crate::rt::path_value(&group_id)),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/groups/{groupId}/change-owner`
///
/// Changes the group owner to another user.
pub async fn post_groups_by_group_id_change_owner(
    client: &Client,
    group_id: i64,
    body: &ChangeOwnerRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_groups_by_group_id_change_owner_request(group_id, body)?)
        .await?
        .json()
}

/// `POST /v1/groups/{groupId}/claim-ownership`
///
/// Claims ownership of the group as the authenticated user
pub fn post_groups_by_group_id_claim_ownership_request(group_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::POST,
        HOST,
        format!("/v1/groups/{}/claim-ownership", crate::rt::path_value(&group_id)),
    );
    Ok(r)
}

/// `POST /v1/groups/{groupId}/claim-ownership`
///
/// Claims ownership of the group as the authenticated user
pub async fn post_groups_by_group_id_claim_ownership(
    client: &Client,
    group_id: i64,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_groups_by_group_id_claim_ownership_request(group_id)?)
        .await?
        .json()
}

/// `GET /v1/groups/{groupId}/community-feature-freezes`
///
/// Gets the freeze status of the community features for a group.
pub fn get_groups_by_group_id_community_feature_freezes_request(group_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/groups/{}/community-feature-freezes",
            crate::rt::path_value(&group_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/groups/{groupId}/community-feature-freezes`
///
/// Gets the freeze status of the community features for a group.
pub async fn get_groups_by_group_id_community_feature_freezes(
    client: &Client,
    group_id: i64,
) -> Result<GetCommunityFeatureFreezesResponse> {
    client
        .send(get_groups_by_group_id_community_feature_freezes_request(
            group_id,
        )?)
        .await?
        .json()
}

/// `POST /v1/groups/{groupId}/community-tier/evaluate`
pub fn post_groups_by_group_id_community_tier_evaluate_request(group_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/groups/{}/community-tier/evaluate",
            crate::rt::path_value(&group_id)
        ),
    );
    Ok(r)
}

/// `POST /v1/groups/{groupId}/community-tier/evaluate`
pub async fn post_groups_by_group_id_community_tier_evaluate(
    client: &Client,
    group_id: i64,
) -> Result<TierEvaluationResultResponse> {
    client
        .send(post_groups_by_group_id_community_tier_evaluate_request(group_id)?)
        .await?
        .json()
}

/// `GET /v1/groups/{groupId}/configuration`
///
/// Gets group configuration information
pub fn get_groups_by_group_id_configuration_request(group_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/groups/{}/configuration", crate::rt::path_value(&group_id)),
    );
    Ok(r)
}

/// `GET /v1/groups/{groupId}/configuration`
///
/// Gets group configuration information
pub async fn get_groups_by_group_id_configuration(
    client: &Client,
    group_id: i64,
) -> Result<GroupConfigurationDetailsResponse> {
    client
        .send(get_groups_by_group_id_configuration_request(group_id)?)
        .await?
        .json()
}

/// `POST /v1/groups/{groupId}/deactivate`
///
/// Deactivates the group as its owner.
pub fn post_groups_by_group_id_deactivate_request(group_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::POST,
        HOST,
        format!("/v1/groups/{}/deactivate", crate::rt::path_value(&group_id)),
    );
    Ok(r)
}

/// `POST /v1/groups/{groupId}/deactivate`
///
/// Deactivates the group as its owner.
pub async fn post_groups_by_group_id_deactivate(
    client: &Client,
    group_id: i64,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_groups_by_group_id_deactivate_request(group_id)?)
        .await?
        .json()
}

/// `PATCH /v1/groups/{groupId}/description`
///
/// Updates the groups description
pub fn patch_groups_by_group_id_description_request(
    group_id: i64,
    body: &UpdateGroupDescriptionRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!("/v1/groups/{}/description", crate::rt::path_value(&group_id)),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/groups/{groupId}/description`
///
/// Updates the groups description
pub async fn patch_groups_by_group_id_description(
    client: &Client,
    group_id: i64,
    body: &UpdateGroupDescriptionRequest,
) -> Result<GroupDescriptionResponse> {
    client
        .send(patch_groups_by_group_id_description_request(group_id, body)?)
        .await?
        .json()
}

/// `GET /v1/groups/{groupId}/emotes`
///
/// Gets a groups emote sets.
pub fn get_groups_by_group_id_emotes_request(group_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/groups/{}/emotes", crate::rt::path_value(&group_id)),
    );
    Ok(r)
}

/// `GET /v1/groups/{groupId}/emotes`
///
/// Gets a groups emote sets.
pub async fn get_groups_by_group_id_emotes(
    client: &Client,
    group_id: i64,
) -> Result<GetGroupEmoteSetsResponse> {
    client
        .send(get_groups_by_group_id_emotes_request(group_id)?)
        .await?
        .json()
}

/// `GET /v1/groups/{groupId}/features`
///
/// Gets the freeze status of all features and the lock status for a group.
pub fn get_groups_by_group_id_features_request(group_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/groups/{}/features", crate::rt::path_value(&group_id)),
    );
    Ok(r)
}

/// `GET /v1/groups/{groupId}/features`
///
/// Gets the freeze status of all features and the lock status for a group.
pub async fn get_groups_by_group_id_features(
    client: &Client,
    group_id: i64,
) -> Result<GetGroupFeaturesResponse> {
    client
        .send(get_groups_by_group_id_features_request(group_id)?)
        .await?
        .json()
}

/// `PATCH /v1/groups/{groupId}/features`
///
/// Sets the desired status of group features. Currently only removes active freezes for features set to Roblox.Groups.Api.FeatureStatus.On.
pub fn patch_groups_by_group_id_features_request(
    group_id: i64,
    body: &SetFeaturesRequestModel,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!("/v1/groups/{}/features", crate::rt::path_value(&group_id)),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/groups/{groupId}/features`
///
/// Sets the desired status of group features. Currently only removes active freezes for features set to Roblox.Groups.Api.FeatureStatus.On.
pub async fn patch_groups_by_group_id_features(
    client: &Client,
    group_id: i64,
    body: &SetFeaturesRequestModel,
) -> Result<SetFeaturesResponseModel> {
    client
        .send(patch_groups_by_group_id_features_request(group_id, body)?)
        .await?
        .json()
}

/// `GET /v1/groups/{groupId}/features/status`
///
/// Checks whether a group has ANY feature disabled (includes feature freezes and group lock). Used to display a banner on Creator Hub/Studio to inform group members that some features may not be available.
pub fn get_groups_by_group_id_features_status_request(group_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/groups/{}/features/status", crate::rt::path_value(&group_id)),
    );
    Ok(r)
}

/// `GET /v1/groups/{groupId}/features/status`
///
/// Checks whether a group has ANY feature disabled (includes feature freezes and group lock). Used to display a banner on Creator Hub/Studio to inform group members that some features may not be available.
pub async fn get_groups_by_group_id_features_status(
    client: &Client,
    group_id: i64,
) -> Result<HasGroupFeaturesBlockedResponse> {
    client
        .send(get_groups_by_group_id_features_status_request(group_id)?)
        .await?
        .json()
}

/// `GET /v1/groups/{groupId}/join-requests`
///
/// Gets a page of Group Join Requests for a group.
pub fn get_groups_by_group_id_join_requests_request(
    group_id: i64,
    params: &GetGroupsByGroupIdJoinRequestsParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/groups/{}/join-requests", crate::rt::path_value(&group_id)),
    );
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    r.query("sortOrder", &params.sort_order);
    Ok(r)
}

/// `GET /v1/groups/{groupId}/join-requests`
///
/// Gets a page of Group Join Requests for a group.
pub async fn get_groups_by_group_id_join_requests(
    client: &Client,
    group_id: i64,
    params: &GetGroupsByGroupIdJoinRequestsParams,
) -> Result<ApiPageResponseGroupJoinRequestResponse> {
    client
        .send(get_groups_by_group_id_join_requests_request(group_id, params)?)
        .await?
        .json()
}

/// `POST /v1/groups/{groupId}/join-requests`
///
/// Batch accepts group join requests
pub fn post_groups_by_group_id_join_requests_request(
    group_id: i64,
    body: &MembersRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!("/v1/groups/{}/join-requests", crate::rt::path_value(&group_id)),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/groups/{groupId}/join-requests`
///
/// Batch accepts group join requests
pub async fn post_groups_by_group_id_join_requests(
    client: &Client,
    group_id: i64,
    body: &MembersRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_groups_by_group_id_join_requests_request(group_id, body)?)
        .await?
        .json()
}

/// `DELETE /v1/groups/{groupId}/join-requests`
///
/// Batch declines group join requests
pub fn delete_groups_by_group_id_join_requests_request(
    group_id: i64,
    body: &MembersRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::DELETE,
        HOST,
        format!("/v1/groups/{}/join-requests", crate::rt::path_value(&group_id)),
    );
    r.json(body)?;
    Ok(r)
}

/// `DELETE /v1/groups/{groupId}/join-requests`
///
/// Batch declines group join requests
pub async fn delete_groups_by_group_id_join_requests(
    client: &Client,
    group_id: i64,
    body: &MembersRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(delete_groups_by_group_id_join_requests_request(group_id, body)?)
        .await?
        .json()
}

/// `GET /v1/groups/{groupId}/join-requests/users/{userId}`
///
/// Gets a group join request by userId.
pub fn get_groups_by_group_id_join_requests_users_by_user_id_request(
    group_id: i64,
    user_id: i64,
) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/groups/{}/join-requests/users/{}",
            crate::rt::path_value(&group_id),
            crate::rt::path_value(&user_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/groups/{groupId}/join-requests/users/{userId}`
///
/// Gets a group join request by userId.
pub async fn get_groups_by_group_id_join_requests_users_by_user_id(
    client: &Client,
    group_id: i64,
    user_id: i64,
) -> Result<GroupJoinRequestResponse> {
    client
        .send(get_groups_by_group_id_join_requests_users_by_user_id_request(
            group_id, user_id,
        )?)
        .await?
        .json()
}

/// `POST /v1/groups/{groupId}/join-requests/users/{userId}`
///
/// Accepts a group join request.
pub fn post_groups_by_group_id_join_requests_users_by_user_id_request(
    group_id: i64,
    user_id: i64,
) -> Result<Request> {
    let r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/groups/{}/join-requests/users/{}",
            crate::rt::path_value(&group_id),
            crate::rt::path_value(&user_id)
        ),
    );
    Ok(r)
}

/// `POST /v1/groups/{groupId}/join-requests/users/{userId}`
///
/// Accepts a group join request.
pub async fn post_groups_by_group_id_join_requests_users_by_user_id(
    client: &Client,
    group_id: i64,
    user_id: i64,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_groups_by_group_id_join_requests_users_by_user_id_request(
            group_id, user_id,
        )?)
        .await?
        .json()
}

/// `DELETE /v1/groups/{groupId}/join-requests/users/{userId}`
///
/// Declines/cancels a group join request.
pub fn delete_groups_by_group_id_join_requests_users_by_user_id_request(
    group_id: i64,
    user_id: i64,
) -> Result<Request> {
    let r = Request::new(
        Method::DELETE,
        HOST,
        format!(
            "/v1/groups/{}/join-requests/users/{}",
            crate::rt::path_value(&group_id),
            crate::rt::path_value(&user_id)
        ),
    );
    Ok(r)
}

/// `DELETE /v1/groups/{groupId}/join-requests/users/{userId}`
///
/// Declines/cancels a group join request.
pub async fn delete_groups_by_group_id_join_requests_users_by_user_id(
    client: &Client,
    group_id: i64,
    user_id: i64,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(delete_groups_by_group_id_join_requests_users_by_user_id_request(
            group_id, user_id,
        )?)
        .await?
        .json()
}

/// `GET /v1/groups/{groupId}/membership`
///
/// Gets group membership information in the context of the authenticated user
pub fn get_groups_by_group_id_membership_request(
    group_id: i64,
    params: &GetGroupsByGroupIdMembershipParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/groups/{}/membership", crate::rt::path_value(&group_id)),
    );
    r.query(
        "includeNotificationPreferences",
        &params.include_notification_preferences,
    );
    Ok(r)
}

/// `GET /v1/groups/{groupId}/membership`
///
/// Gets group membership information in the context of the authenticated user
pub async fn get_groups_by_group_id_membership(
    client: &Client,
    group_id: i64,
    params: &GetGroupsByGroupIdMembershipParams,
) -> Result<GroupMembershipMetadataResponse> {
    client
        .send(get_groups_by_group_id_membership_request(group_id, params)?)
        .await?
        .json()
}

/// `PATCH /v1/groups/{groupId}/name`
///
/// Updates the group's name.
///
/// This endpoint will charge Robux for the group rename.
pub fn patch_groups_by_group_id_name_request(
    group_id: i64,
    body: &UpdateGroupNameRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!("/v1/groups/{}/name", crate::rt::path_value(&group_id)),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/groups/{groupId}/name`
///
/// Updates the group's name.
///
/// This endpoint will charge Robux for the group rename.
pub async fn patch_groups_by_group_id_name(
    client: &Client,
    group_id: i64,
    body: &UpdateGroupNameRequest,
) -> Result<UpdateGroupNameResponse> {
    client
        .send(patch_groups_by_group_id_name_request(group_id, body)?)
        .await?
        .json()
}

/// `GET /v1/groups/{groupId}/name-history`
///
/// Gets the Group's name change history.
pub fn get_groups_by_group_id_name_history_request(
    group_id: i64,
    params: &GetGroupsByGroupIdNameHistoryParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/groups/{}/name-history", crate::rt::path_value(&group_id)),
    );
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    r.query("sortOrder", &params.sort_order);
    Ok(r)
}

/// `GET /v1/groups/{groupId}/name-history`
///
/// Gets the Group's name change history.
pub async fn get_groups_by_group_id_name_history(
    client: &Client,
    group_id: i64,
    params: &GetGroupsByGroupIdNameHistoryParams,
) -> Result<ApiPageResponseGroupNameHistoryResponseItem> {
    client
        .send(get_groups_by_group_id_name_history_request(group_id, params)?)
        .await?
        .json()
}

/// `PATCH /v1/groups/{groupId}/notification-preference`
///
/// Updates the group's settings
pub fn patch_groups_by_group_id_notification_preference_request(
    group_id: i64,
    body: &UpdateGroupNotificationPreferenceRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!(
            "/v1/groups/{}/notification-preference",
            crate::rt::path_value(&group_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/groups/{groupId}/notification-preference`
///
/// Updates the group's settings
pub async fn patch_groups_by_group_id_notification_preference(
    client: &Client,
    group_id: i64,
    body: &UpdateGroupNotificationPreferenceRequest,
) -> Result<PatchGroupsByGroupIdNotificationPreferenceResponse> {
    client
        .send(patch_groups_by_group_id_notification_preference_request(
            group_id, body,
        )?)
        .await?
        .json()
}

/// `GET /v1/groups/{groupId}/payout-restriction`
///
/// Gets a value indicating whether the group can use payout feature
pub fn get_groups_by_group_id_payout_restriction_request(group_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/groups/{}/payout-restriction",
            crate::rt::path_value(&group_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/groups/{groupId}/payout-restriction`
///
/// Gets a value indicating whether the group can use payout feature
pub async fn get_groups_by_group_id_payout_restriction(
    client: &Client,
    group_id: i64,
) -> Result<GroupPayoutRestrictionResponse> {
    client
        .send(get_groups_by_group_id_payout_restriction_request(group_id)?)
        .await?
        .json()
}

/// `GET /v1/groups/{groupId}/payouts`
///
/// Gets a list of the group payout percentages
pub fn get_groups_by_group_id_payouts_request(group_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/groups/{}/payouts", crate::rt::path_value(&group_id)),
    );
    Ok(r)
}

/// `GET /v1/groups/{groupId}/payouts`
///
/// Gets a list of the group payout percentages
pub async fn get_groups_by_group_id_payouts(
    client: &Client,
    group_id: i64,
) -> Result<ApiArrayResponseGroupPayoutResponse> {
    client
        .send(get_groups_by_group_id_payouts_request(group_id)?)
        .await?
        .json()
}

/// `POST /v1/groups/{groupId}/payouts`
///
/// Pays out a user in Robux.
pub fn post_groups_by_group_id_payouts_request(group_id: i64, body: &PayoutRequest) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!("/v1/groups/{}/payouts", crate::rt::path_value(&group_id)),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/groups/{groupId}/payouts`
///
/// Pays out a user in Robux.
pub async fn post_groups_by_group_id_payouts(
    client: &Client,
    group_id: i64,
    body: &PayoutRequest,
) -> Result<OneTimePayoutResponse> {
    client
        .send(post_groups_by_group_id_payouts_request(group_id, body)?)
        .await?
        .json()
}

/// `POST /v1/groups/{groupId}/payouts/recurring`
///
/// Updates recurring payouts.
///
/// This endpoint will remove any recipients not sent in the request. If a recipient in the request is not a valid member in the group they will not be added to the recurring payouts.
pub fn post_groups_by_group_id_payouts_recurring_request(
    group_id: i64,
    body: &PayoutRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/groups/{}/payouts/recurring",
            crate::rt::path_value(&group_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/groups/{groupId}/payouts/recurring`
///
/// Updates recurring payouts.
///
/// This endpoint will remove any recipients not sent in the request. If a recipient in the request is not a valid member in the group they will not be added to the recurring payouts.
pub async fn post_groups_by_group_id_payouts_recurring(
    client: &Client,
    group_id: i64,
    body: &PayoutRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_groups_by_group_id_payouts_recurring_request(group_id, body)?)
        .await?
        .json()
}

/// `GET /v1/groups/{groupId}/relationships/{groupRelationshipType}`
///
/// Gets a group's relationships
pub fn get_groups_by_group_id_relationships_by_group_relationship_type_request(
    group_id: i64,
    group_relationship_type: &str,
    params: &GetGroupsByGroupIdRelationshipsByGroupRelationshipTypeParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/groups/{}/relationships/{}",
            crate::rt::path_value(&group_id),
            crate::rt::path_value(&group_relationship_type)
        ),
    );
    r.query("StartRowIndex", &params.start_row_index);
    r.query("MaxRows", &params.max_rows);
    Ok(r)
}

/// `GET /v1/groups/{groupId}/relationships/{groupRelationshipType}`
///
/// Gets a group's relationships
pub async fn get_groups_by_group_id_relationships_by_group_relationship_type(
    client: &Client,
    group_id: i64,
    group_relationship_type: &str,
    params: &GetGroupsByGroupIdRelationshipsByGroupRelationshipTypeParams,
) -> Result<GroupRelationshipsResponse> {
    client
        .send(
            get_groups_by_group_id_relationships_by_group_relationship_type_request(
                group_id,
                group_relationship_type,
                params,
            )?,
        )
        .await?
        .json()
}

/// `GET /v1/groups/{groupId}/relationships/{groupRelationshipType}/requests`
///
/// Gets a group's relationship requests
pub fn get_groups_by_group_id_relationships_by_group_relationship_type_requests_request(
    group_id: i64,
    group_relationship_type: &str,
    params: &GetGroupsByGroupIdRelationshipsByGroupRelationshipTypeRequestsParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/groups/{}/relationships/{}/requests",
            crate::rt::path_value(&group_id),
            crate::rt::path_value(&group_relationship_type)
        ),
    );
    r.query("StartRowIndex", &params.start_row_index);
    r.query("MaxRows", &params.max_rows);
    Ok(r)
}

/// `GET /v1/groups/{groupId}/relationships/{groupRelationshipType}/requests`
///
/// Gets a group's relationship requests
pub async fn get_groups_by_group_id_relationships_by_group_relationship_type_requests(
    client: &Client,
    group_id: i64,
    group_relationship_type: &str,
    params: &GetGroupsByGroupIdRelationshipsByGroupRelationshipTypeRequestsParams,
) -> Result<GroupRelationshipsResponse> {
    client
        .send(
            get_groups_by_group_id_relationships_by_group_relationship_type_requests_request(
                group_id,
                group_relationship_type,
                params,
            )?,
        )
        .await?
        .json()
}

/// `POST /v1/groups/{groupId}/relationships/{groupRelationshipType}/requests`
///
/// Batch accepts group affiliate requests
pub fn post_groups_by_group_id_relationships_by_group_relationship_type_requests_request(
    group_id: i64,
    group_relationship_type: &str,
    body: &RelationshipsRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/groups/{}/relationships/{}/requests",
            crate::rt::path_value(&group_id),
            crate::rt::path_value(&group_relationship_type)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/groups/{groupId}/relationships/{groupRelationshipType}/requests`
///
/// Batch accepts group affiliate requests
pub async fn post_groups_by_group_id_relationships_by_group_relationship_type_requests(
    client: &Client,
    group_id: i64,
    group_relationship_type: &str,
    body: &RelationshipsRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(
            post_groups_by_group_id_relationships_by_group_relationship_type_requests_request(
                group_id,
                group_relationship_type,
                body,
            )?,
        )
        .await?
        .json()
}

/// `DELETE /v1/groups/{groupId}/relationships/{groupRelationshipType}/requests`
///
/// Batch declines group affiliate requests
pub fn delete_groups_by_group_id_relationships_by_group_relationship_type_requests_request(
    group_id: i64,
    group_relationship_type: &str,
    body: &RelationshipsRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::DELETE,
        HOST,
        format!(
            "/v1/groups/{}/relationships/{}/requests",
            crate::rt::path_value(&group_id),
            crate::rt::path_value(&group_relationship_type)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `DELETE /v1/groups/{groupId}/relationships/{groupRelationshipType}/requests`
///
/// Batch declines group affiliate requests
pub async fn delete_groups_by_group_id_relationships_by_group_relationship_type_requests(
    client: &Client,
    group_id: i64,
    group_relationship_type: &str,
    body: &RelationshipsRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(
            delete_groups_by_group_id_relationships_by_group_relationship_type_requests_request(
                group_id,
                group_relationship_type,
                body,
            )?,
        )
        .await?
        .json()
}

/// `POST /v1/groups/{groupId}/relationships/{groupRelationshipType}/requests/{relatedGroupId}`
///
/// Accepts a group relationship request.
pub fn post_groups_by_group_id_relationships_by_group_relationship_type_requests_by_related_group_id_request(
    group_id: i64,
    group_relationship_type: &str,
    related_group_id: i64,
) -> Result<Request> {
    let r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/groups/{}/relationships/{}/requests/{}",
            crate::rt::path_value(&group_id),
            crate::rt::path_value(&group_relationship_type),
            crate::rt::path_value(&related_group_id)
        ),
    );
    Ok(r)
}

/// `POST /v1/groups/{groupId}/relationships/{groupRelationshipType}/requests/{relatedGroupId}`
///
/// Accepts a group relationship request.
pub async fn post_groups_by_group_id_relationships_by_group_relationship_type_requests_by_related_group_id(
    client: &Client,
    group_id: i64,
    group_relationship_type: &str,
    related_group_id: i64,
) -> Result<ApiEmptyResponseModel> {
    client.send(post_groups_by_group_id_relationships_by_group_relationship_type_requests_by_related_group_id_request(group_id, group_relationship_type, related_group_id)?).await?.json()
}

/// `DELETE /v1/groups/{groupId}/relationships/{groupRelationshipType}/requests/{relatedGroupId}`
///
/// Declines a group relationship request.
pub fn delete_groups_by_group_id_relationships_by_group_relationship_type_requests_by_related_group_id_request(
    group_id: i64,
    group_relationship_type: &str,
    related_group_id: i64,
) -> Result<Request> {
    let r = Request::new(
        Method::DELETE,
        HOST,
        format!(
            "/v1/groups/{}/relationships/{}/requests/{}",
            crate::rt::path_value(&group_id),
            crate::rt::path_value(&group_relationship_type),
            crate::rt::path_value(&related_group_id)
        ),
    );
    Ok(r)
}

/// `DELETE /v1/groups/{groupId}/relationships/{groupRelationshipType}/requests/{relatedGroupId}`
///
/// Declines a group relationship request.
pub async fn delete_groups_by_group_id_relationships_by_group_relationship_type_requests_by_related_group_id(
    client: &Client,
    group_id: i64,
    group_relationship_type: &str,
    related_group_id: i64,
) -> Result<ApiEmptyResponseModel> {
    client.send(delete_groups_by_group_id_relationships_by_group_relationship_type_requests_by_related_group_id_request(group_id, group_relationship_type, related_group_id)?).await?.json()
}

/// `POST /v1/groups/{groupId}/relationships/{groupRelationshipType}/{relatedGroupId}`
///
/// Create a group relationship.
pub fn post_groups_by_group_id_relationships_by_group_relationship_type_by_related_group_id_request(
    group_id: i64,
    group_relationship_type: &str,
    related_group_id: i64,
) -> Result<Request> {
    let r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/groups/{}/relationships/{}/{}",
            crate::rt::path_value(&group_id),
            crate::rt::path_value(&group_relationship_type),
            crate::rt::path_value(&related_group_id)
        ),
    );
    Ok(r)
}

/// `POST /v1/groups/{groupId}/relationships/{groupRelationshipType}/{relatedGroupId}`
///
/// Create a group relationship.
pub async fn post_groups_by_group_id_relationships_by_group_relationship_type_by_related_group_id(
    client: &Client,
    group_id: i64,
    group_relationship_type: &str,
    related_group_id: i64,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(
            post_groups_by_group_id_relationships_by_group_relationship_type_by_related_group_id_request(
                group_id,
                group_relationship_type,
                related_group_id,
            )?,
        )
        .await?
        .json()
}

/// `DELETE /v1/groups/{groupId}/relationships/{groupRelationshipType}/{relatedGroupId}`
///
/// Deletes a group relationship.
pub fn delete_groups_by_group_id_relationships_by_group_relationship_type_by_related_group_id_request(
    group_id: i64,
    group_relationship_type: &str,
    related_group_id: i64,
) -> Result<Request> {
    let r = Request::new(
        Method::DELETE,
        HOST,
        format!(
            "/v1/groups/{}/relationships/{}/{}",
            crate::rt::path_value(&group_id),
            crate::rt::path_value(&group_relationship_type),
            crate::rt::path_value(&related_group_id)
        ),
    );
    Ok(r)
}

/// `DELETE /v1/groups/{groupId}/relationships/{groupRelationshipType}/{relatedGroupId}`
///
/// Deletes a group relationship.
pub async fn delete_groups_by_group_id_relationships_by_group_relationship_type_by_related_group_id(
    client: &Client,
    group_id: i64,
    group_relationship_type: &str,
    related_group_id: i64,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(
            delete_groups_by_group_id_relationships_by_group_relationship_type_by_related_group_id_request(
                group_id,
                group_relationship_type,
                related_group_id,
            )?,
        )
        .await?
        .json()
}

/// `GET /v1/groups/{groupId}/roles`
///
/// Gets a list of the rolesets in a group.
pub fn get_groups_by_group_id_roles_request(
    group_id: i64,
    params: &GetGroupsByGroupIdRolesParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/groups/{}/roles", crate::rt::path_value(&group_id)),
    );
    r.query("includePrivate", &params.include_private);
    Ok(r)
}

/// `GET /v1/groups/{groupId}/roles`
///
/// Gets a list of the rolesets in a group.
pub async fn get_groups_by_group_id_roles(
    client: &Client,
    group_id: i64,
    params: &GetGroupsByGroupIdRolesParams,
) -> Result<GroupAllRolesResponse> {
    client
        .send(get_groups_by_group_id_roles_request(group_id, params)?)
        .await?
        .json()
}

/// `GET /v1/groups/{groupId}/roles/guest/permissions`
///
/// Gets the permissions for a group's guest roleset. These can be viewed by all (members and guests) users.
pub fn get_groups_by_group_id_roles_guest_permissions_request(group_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/groups/{}/roles/guest/permissions",
            crate::rt::path_value(&group_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/groups/{groupId}/roles/guest/permissions`
///
/// Gets the permissions for a group's guest roleset. These can be viewed by all (members and guests) users.
pub async fn get_groups_by_group_id_roles_guest_permissions(
    client: &Client,
    group_id: i64,
) -> Result<GroupPermissionsResponse> {
    client
        .send(get_groups_by_group_id_roles_guest_permissions_request(group_id)?)
        .await?
        .json()
}

/// `GET /v1/groups/{groupId}/roles/permissions`
///
/// Gets all permissions for each role
pub fn get_groups_by_group_id_roles_permissions_request(group_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/groups/{}/roles/permissions",
            crate::rt::path_value(&group_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/groups/{groupId}/roles/permissions`
///
/// Gets all permissions for each role
pub async fn get_groups_by_group_id_roles_permissions(
    client: &Client,
    group_id: i64,
) -> Result<ApiArrayResponseGroupPermissionsResponse> {
    client
        .send(get_groups_by_group_id_roles_permissions_request(group_id)?)
        .await?
        .json()
}

/// `GET /v1/groups/{groupId}/roles/{roleSetId}/permissions`
///
/// Gets the permissions for a group's roleset. The authorized user must either be the group owner or the roleset being requested, except for guest roles, which can be viewed by all (members and guests).
pub fn get_groups_by_group_id_roles_by_role_set_id_permissions_request(
    group_id: i64,
    role_set_id: i64,
) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/groups/{}/roles/{}/permissions",
            crate::rt::path_value(&group_id),
            crate::rt::path_value(&role_set_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/groups/{groupId}/roles/{roleSetId}/permissions`
///
/// Gets the permissions for a group's roleset. The authorized user must either be the group owner or the roleset being requested, except for guest roles, which can be viewed by all (members and guests).
pub async fn get_groups_by_group_id_roles_by_role_set_id_permissions(
    client: &Client,
    group_id: i64,
    role_set_id: i64,
) -> Result<GroupPermissionsResponse> {
    client
        .send(get_groups_by_group_id_roles_by_role_set_id_permissions_request(
            group_id,
            role_set_id,
        )?)
        .await?
        .json()
}

/// `PATCH /v1/groups/{groupId}/roles/{roleSetId}/permissions`
///
/// Updates the permissions for a group's roleset. The authorized user must be the group owner.
pub fn patch_groups_by_group_id_roles_by_role_set_id_permissions_request(
    group_id: i64,
    role_set_id: i64,
    body: &UpdatePermissionsRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!(
            "/v1/groups/{}/roles/{}/permissions",
            crate::rt::path_value(&group_id),
            crate::rt::path_value(&role_set_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/groups/{groupId}/roles/{roleSetId}/permissions`
///
/// Updates the permissions for a group's roleset. The authorized user must be the group owner.
pub async fn patch_groups_by_group_id_roles_by_role_set_id_permissions(
    client: &Client,
    group_id: i64,
    role_set_id: i64,
    body: &UpdatePermissionsRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(patch_groups_by_group_id_roles_by_role_set_id_permissions_request(
            group_id,
            role_set_id,
            body,
        )?)
        .await?
        .json()
}

/// `GET /v1/groups/{groupId}/roles/{roleSetId}/users`
///
/// Gets a list of users in a group for a specific roleset.
pub fn get_groups_by_group_id_roles_by_role_set_id_users_request(
    group_id: i64,
    role_set_id: i64,
    params: &GetGroupsByGroupIdRolesByRoleSetIdUsersParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/groups/{}/roles/{}/users",
            crate::rt::path_value(&group_id),
            crate::rt::path_value(&role_set_id)
        ),
    );
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    r.query("sortOrder", &params.sort_order);
    Ok(r)
}

/// `GET /v1/groups/{groupId}/roles/{roleSetId}/users`
///
/// Gets a list of users in a group for a specific roleset.
pub async fn get_groups_by_group_id_roles_by_role_set_id_users(
    client: &Client,
    group_id: i64,
    role_set_id: i64,
    params: &GetGroupsByGroupIdRolesByRoleSetIdUsersParams,
) -> Result<ApiPageResponseUserModel> {
    client
        .send(get_groups_by_group_id_roles_by_role_set_id_users_request(
            group_id,
            role_set_id,
            params,
        )?)
        .await?
        .json()
}

/// `POST /v1/groups/{groupId}/rolesets/create`
///
/// Creates new group roleset.
pub fn post_groups_by_group_id_rolesets_create_request(
    group_id: i64,
    body: &CreateRoleSetRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!("/v1/groups/{}/rolesets/create", crate::rt::path_value(&group_id)),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/groups/{groupId}/rolesets/create`
///
/// Creates new group roleset.
pub async fn post_groups_by_group_id_rolesets_create(
    client: &Client,
    group_id: i64,
    body: &CreateRoleSetRequest,
) -> Result<GroupRoleResponse> {
    client
        .send(post_groups_by_group_id_rolesets_create_request(group_id, body)?)
        .await?
        .json()
}

/// `PATCH /v1/groups/{groupId}/rolesets/{rolesetId}`
///
/// Updates existing group roleset.
pub fn patch_groups_by_group_id_rolesets_by_roleset_id_request(
    group_id: i64,
    roleset_id: i64,
    body: &UpdateRoleSetRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!(
            "/v1/groups/{}/rolesets/{}",
            crate::rt::path_value(&group_id),
            crate::rt::path_value(&roleset_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/groups/{groupId}/rolesets/{rolesetId}`
///
/// Updates existing group roleset.
pub async fn patch_groups_by_group_id_rolesets_by_roleset_id(
    client: &Client,
    group_id: i64,
    roleset_id: i64,
    body: &UpdateRoleSetRequest,
) -> Result<GroupRoleResponse> {
    client
        .send(patch_groups_by_group_id_rolesets_by_roleset_id_request(
            group_id, roleset_id, body,
        )?)
        .await?
        .json()
}

/// `DELETE /v1/groups/{groupId}/rolesets/{rolesetId}`
///
/// Deletes existing group roleset.
pub fn delete_groups_by_group_id_rolesets_by_roleset_id_request(
    group_id: i64,
    roleset_id: i64,
) -> Result<Request> {
    let r = Request::new(
        Method::DELETE,
        HOST,
        format!(
            "/v1/groups/{}/rolesets/{}",
            crate::rt::path_value(&group_id),
            crate::rt::path_value(&roleset_id)
        ),
    );
    Ok(r)
}

/// `DELETE /v1/groups/{groupId}/rolesets/{rolesetId}`
///
/// Deletes existing group roleset.
pub async fn delete_groups_by_group_id_rolesets_by_roleset_id(
    client: &Client,
    group_id: i64,
    roleset_id: i64,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(delete_groups_by_group_id_rolesets_by_roleset_id_request(
            group_id, roleset_id,
        )?)
        .await?
        .json()
}

/// `GET /v1/groups/{groupId}/settings`
///
/// Gets the Group's settings
pub fn get_groups_by_group_id_settings_request(group_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/groups/{}/settings", crate::rt::path_value(&group_id)),
    );
    Ok(r)
}

/// `GET /v1/groups/{groupId}/settings`
///
/// Gets the Group's settings
pub async fn get_groups_by_group_id_settings(
    client: &Client,
    group_id: i64,
) -> Result<GroupSettingsResponse> {
    client
        .send(get_groups_by_group_id_settings_request(group_id)?)
        .await?
        .json()
}

/// `PATCH /v1/groups/{groupId}/settings`
///
/// Updates the group's settings
pub fn patch_groups_by_group_id_settings_request(
    group_id: i64,
    body: &UpdateGroupSettingsRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!("/v1/groups/{}/settings", crate::rt::path_value(&group_id)),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/groups/{groupId}/settings`
///
/// Updates the group's settings
pub async fn patch_groups_by_group_id_settings(
    client: &Client,
    group_id: i64,
    body: &UpdateGroupSettingsRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(patch_groups_by_group_id_settings_request(group_id, body)?)
        .await?
        .json()
}

/// `GET /v1/groups/{groupId}/social-links`
///
/// Get social link data associated with a group
pub fn get_groups_by_group_id_social_links_request(group_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/groups/{}/social-links", crate::rt::path_value(&group_id)),
    );
    Ok(r)
}

/// `GET /v1/groups/{groupId}/social-links`
///
/// Get social link data associated with a group
pub async fn get_groups_by_group_id_social_links(
    client: &Client,
    group_id: i64,
) -> Result<GetSocialLinkResponse> {
    client
        .send(get_groups_by_group_id_social_links_request(group_id)?)
        .await?
        .json()
}

/// `POST /v1/groups/{groupId}/social-links`
///
/// Posts a social links
pub fn post_groups_by_group_id_social_links_request(
    group_id: i64,
    body: &SocialLinkRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!("/v1/groups/{}/social-links", crate::rt::path_value(&group_id)),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/groups/{groupId}/social-links`
///
/// Posts a social links
pub async fn post_groups_by_group_id_social_links(
    client: &Client,
    group_id: i64,
    body: &SocialLinkRequest,
) -> Result<SocialLinkResponse> {
    client
        .send(post_groups_by_group_id_social_links_request(group_id, body)?)
        .await?
        .json()
}

/// `PATCH /v1/groups/{groupId}/social-links/{socialLinkId}`
///
/// Updates a social link
pub fn patch_groups_by_group_id_social_links_by_social_link_id_request(
    group_id: i64,
    social_link_id: i64,
    body: &SocialLinkRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!(
            "/v1/groups/{}/social-links/{}",
            crate::rt::path_value(&group_id),
            crate::rt::path_value(&social_link_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/groups/{groupId}/social-links/{socialLinkId}`
///
/// Updates a social link
pub async fn patch_groups_by_group_id_social_links_by_social_link_id(
    client: &Client,
    group_id: i64,
    social_link_id: i64,
    body: &SocialLinkRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(patch_groups_by_group_id_social_links_by_social_link_id_request(
            group_id,
            social_link_id,
            body,
        )?)
        .await?
        .json()
}

/// `DELETE /v1/groups/{groupId}/social-links/{socialLinkId}`
///
/// Deletes a social link
pub fn delete_groups_by_group_id_social_links_by_social_link_id_request(
    group_id: i64,
    social_link_id: i64,
) -> Result<Request> {
    let r = Request::new(
        Method::DELETE,
        HOST,
        format!(
            "/v1/groups/{}/social-links/{}",
            crate::rt::path_value(&group_id),
            crate::rt::path_value(&social_link_id)
        ),
    );
    Ok(r)
}

/// `DELETE /v1/groups/{groupId}/social-links/{socialLinkId}`
///
/// Deletes a social link
pub async fn delete_groups_by_group_id_social_links_by_social_link_id(
    client: &Client,
    group_id: i64,
    social_link_id: i64,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(delete_groups_by_group_id_social_links_by_social_link_id_request(
            group_id,
            social_link_id,
        )?)
        .await?
        .json()
}

/// `PATCH /v1/groups/{groupId}/status`
///
/// Sets group status
pub fn patch_groups_by_group_id_status_request(
    group_id: i64,
    body: &PostGroupStatusRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!("/v1/groups/{}/status", crate::rt::path_value(&group_id)),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/groups/{groupId}/status`
///
/// Sets group status
pub async fn patch_groups_by_group_id_status(
    client: &Client,
    group_id: i64,
    body: &PostGroupStatusRequest,
) -> Result<ShoutResponse> {
    client
        .send(patch_groups_by_group_id_status_request(group_id, body)?)
        .await?
        .json()
}

/// `GET /v1/groups/{groupId}/users`
///
/// Gets a list of users in a group.
pub fn get_groups_by_group_id_users_request(
    group_id: i64,
    params: &GetGroupsByGroupIdUsersParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/groups/{}/users", crate::rt::path_value(&group_id)),
    );
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    r.query("sortOrder", &params.sort_order);
    Ok(r)
}

/// `GET /v1/groups/{groupId}/users`
///
/// Gets a list of users in a group.
pub async fn get_groups_by_group_id_users(
    client: &Client,
    group_id: i64,
    params: &GetGroupsByGroupIdUsersParams,
) -> Result<ApiPageResponseUserGroupRoleResponse> {
    client
        .send(get_groups_by_group_id_users_request(group_id, params)?)
        .await?
        .json()
}

/// `POST /v1/groups/{groupId}/users`
///
/// Joins a group
pub fn post_groups_by_group_id_users_request(
    group_id: i64,
    params: &PostGroupsByGroupIdUsersParams,
    body: &JoinGroupRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!("/v1/groups/{}/users", crate::rt::path_value(&group_id)),
    );
    r.header("Roblox-Place-Id", &params.roblox_place_id);
    r.header("Roblox-Game-Id", &params.roblox_game_id);
    r.header("Roblox-Session-Id", &params.roblox_session_id);
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/groups/{groupId}/users`
///
/// Joins a group
pub async fn post_groups_by_group_id_users(
    client: &Client,
    group_id: i64,
    params: &PostGroupsByGroupIdUsersParams,
    body: &JoinGroupRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_groups_by_group_id_users_request(group_id, params, body)?)
        .await?
        .json()
}

/// `PATCH /v1/groups/{groupId}/users/{userId}`
///
/// Updates a users role in a group.
pub fn patch_groups_by_group_id_users_by_user_id_request(
    group_id: i64,
    user_id: i64,
    body: &UpdateUserRoleRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!(
            "/v1/groups/{}/users/{}",
            crate::rt::path_value(&group_id),
            crate::rt::path_value(&user_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/groups/{groupId}/users/{userId}`
///
/// Updates a users role in a group.
pub async fn patch_groups_by_group_id_users_by_user_id(
    client: &Client,
    group_id: i64,
    user_id: i64,
    body: &UpdateUserRoleRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(patch_groups_by_group_id_users_by_user_id_request(
            group_id, user_id, body,
        )?)
        .await?
        .json()
}

/// `DELETE /v1/groups/{groupId}/users/{userId}`
///
/// Removes a user from a group
pub fn delete_groups_by_group_id_users_by_user_id_request(group_id: i64, user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::DELETE,
        HOST,
        format!(
            "/v1/groups/{}/users/{}",
            crate::rt::path_value(&group_id),
            crate::rt::path_value(&user_id)
        ),
    );
    Ok(r)
}

/// `DELETE /v1/groups/{groupId}/users/{userId}`
///
/// Removes a user from a group
pub async fn delete_groups_by_group_id_users_by_user_id(
    client: &Client,
    group_id: i64,
    user_id: i64,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(delete_groups_by_group_id_users_by_user_id_request(
            group_id, user_id,
        )?)
        .await?
        .json()
}

/// `GET /v1/groups/{groupId}/users/{userId}/permissions`
///
/// Gets the permissions a user has in a group. Only available to group owner and RCC
pub fn get_groups_by_group_id_users_by_user_id_permissions_request(
    group_id: i64,
    user_id: i64,
) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/groups/{}/users/{}/permissions",
            crate::rt::path_value(&group_id),
            crate::rt::path_value(&user_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/groups/{groupId}/users/{userId}/permissions`
///
/// Gets the permissions a user has in a group. Only available to group owner and RCC
pub async fn get_groups_by_group_id_users_by_user_id_permissions(
    client: &Client,
    group_id: i64,
    user_id: i64,
) -> Result<GroupPermissionsResponse> {
    client
        .send(get_groups_by_group_id_users_by_user_id_permissions_request(
            group_id, user_id,
        )?)
        .await?
        .json()
}

/// `GET /v1/roles`
///
/// Gets the Roles by their ids.
pub fn get_roles_request(params: &GetRolesParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/roles".to_string());
    r.query("ids", &params.ids);
    r.query("includePrivate", &params.include_private);
    Ok(r)
}

/// `GET /v1/roles`
///
/// Gets the Roles by their ids.
pub async fn get_roles(
    client: &Client,
    params: &GetRolesParams,
) -> Result<ApiArrayResponseGroupRoleDetailResponse> {
    client.send(get_roles_request(params)?).await?.json()
}

/// `GET /v1/user/groups/pending`
///
/// Gets groups that the authenticated user has requested to join
pub fn get_user_groups_pending_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/user/groups/pending".to_string());
    Ok(r)
}

/// `GET /v1/user/groups/pending`
///
/// Gets groups that the authenticated user has requested to join
pub async fn get_user_groups_pending(client: &Client) -> Result<ApiArrayResponseGroupDetailResponse> {
    client.send(get_user_groups_pending_request()?).await?.json()
}

/// `POST /v1/user/groups/primary`
///
/// Sets the authenticated user's primary group
pub fn post_user_groups_primary_request(body: &PrimaryGroupRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/user/groups/primary".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/user/groups/primary`
///
/// Sets the authenticated user's primary group
pub async fn post_user_groups_primary(
    client: &Client,
    body: &PrimaryGroupRequest,
) -> Result<ApiEmptyResponseModel> {
    client.send(post_user_groups_primary_request(body)?).await?.json()
}

/// `DELETE /v1/user/groups/primary`
///
/// Removes the authenticated user's primary group
pub fn delete_user_groups_primary_request() -> Result<Request> {
    let r = Request::new(Method::DELETE, HOST, "/v1/user/groups/primary".to_string());
    Ok(r)
}

/// `DELETE /v1/user/groups/primary`
///
/// Removes the authenticated user's primary group
pub async fn delete_user_groups_primary(client: &Client) -> Result<ApiEmptyResponseModel> {
    client.send(delete_user_groups_primary_request()?).await?.json()
}

/// `GET /v1/users/{userId}/friends/groups/roles`
///
/// Gets a list of all groups the specified users' friends are in.
pub fn get_users_by_user_id_friends_groups_roles_request(user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/users/{}/friends/groups/roles",
            crate::rt::path_value(&user_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/users/{userId}/friends/groups/roles`
///
/// Gets a list of all groups the specified users' friends are in.
pub async fn get_users_by_user_id_friends_groups_roles(
    client: &Client,
    user_id: i64,
) -> Result<ApiArrayResponseUserGroupMembershipResponse> {
    client
        .send(get_users_by_user_id_friends_groups_roles_request(user_id)?)
        .await?
        .json()
}

/// `GET /v1/users/{userId}/groups/primary/role`
///
/// Gets a user's primary group.
pub fn get_users_by_user_id_groups_primary_role_request(user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/users/{}/groups/primary/role",
            crate::rt::path_value(&user_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/users/{userId}/groups/primary/role`
///
/// Gets a user's primary group.
pub async fn get_users_by_user_id_groups_primary_role(
    client: &Client,
    user_id: i64,
) -> Result<GroupMembershipDetailResponse> {
    client
        .send(get_users_by_user_id_groups_primary_role_request(user_id)?)
        .await?
        .json()
}

/// `GET /v1/users/{userId}/groups/roles`
///
/// Gets a list of all group roles for groups the specified user is in.
pub fn get_users_by_user_id_groups_roles_request(
    user_id: i64,
    params: &GetUsersByUserIdGroupsRolesParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/users/{}/groups/roles", crate::rt::path_value(&user_id)),
    );
    r.query("includeLocked", &params.include_locked);
    r.query(
        "includeNotificationPreferences",
        &params.include_notification_preferences,
    );
    r.query("discoveryType", &params.discovery_type);
    Ok(r)
}

/// `GET /v1/users/{userId}/groups/roles`
///
/// Gets a list of all group roles for groups the specified user is in.
pub async fn get_users_by_user_id_groups_roles(
    client: &Client,
    user_id: i64,
    params: &GetUsersByUserIdGroupsRolesParams,
) -> Result<ApiArrayResponseGroupMembershipDetailResponse> {
    client
        .send(get_users_by_user_id_groups_roles_request(user_id, params)?)
        .await?
        .json()
}

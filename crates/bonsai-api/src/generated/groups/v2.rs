//! `groups.roblox.com` v2 — generated from `vendor/roblox-api-tracker/groups/v2.json`.
//!
//! Do not edit. Change the spec or `bonsai-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "groups.roblox.com";

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

/// `Roblox.Web.Responses.Groups.GroupRoleBasicResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupRoleBasicResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
}

/// `Roblox.Groups.Api.GroupMembershipResponse`
///
/// A basic group membership response model
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupMembershipResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<GroupBasicResponse>,
    /// Whether the group notification preferences are enabled for the user TODO: Deprecate this field - it only returns if announcement notifications are enabled and that information is already returned in the NotificationPreferences list below
    #[serde(rename = "isNotificationsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_notifications_enabled: Option<bool>,
    #[serde(rename = "notificationPreferences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_preferences: Option<Vec<GroupNotificationPreferenceData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<GroupRoleBasicResponse>,
}

crate::api_enum! {
    int
    /// \['User' = 0, 'Group' = 1, 'Experience' = 2\]
    RelatedEntityTypeResponseCreatorTypeType {
    User = 0 => "User",
    Group = 1 => "Group",
    Experience = 2 => "Experience",
    }
}

/// `Roblox.Web.Responses.RelatedEntityTypeResponse\[Roblox.Platform.Core.CreatorType\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RelatedEntityTypeResponseCreatorType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// \['User' = 0, 'Group' = 1, 'Experience' = 2\]
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<RelatedEntityTypeResponseCreatorTypeType>,
}

crate::api_enum! {
    int
    /// \['Image' = 1, 'TShirt' = 2, 'Audio' = 3, 'Mesh' = 4, 'Lua' = 5, 'HTML' = 6, 'Text' = 7, 'Hat' = 8, 'Place' = 9, 'Model' = 10, 'Shirt' = 11, 'Pants' = 12, 'Decal' = 13, 'Avatar' = 16, 'Head' = 17, 'Face' = 18, 'Gear' = 19, 'Badge' = 21, 'GroupEmblem' = 22, 'Animation' = 24, 'Arms' = 25, 'Legs' = 26, 'Torso' = 27, 'RightArm' = 28, 'LeftArm' = 29, 'LeftLeg' = 30, 'RightLeg' = 31, 'Package' = 32, 'YouTubeVideo' = 33, 'GamePass' = 34, 'App' = 35, 'Code' = 37, 'Plugin' = 38, 'SolidModel' = 39, 'MeshPart' = 40, 'HairAccessory' = 41, 'FaceAccessory' = 42, 'NeckAccessory' = 43, 'ShoulderAccessory' = 44, 'FrontAccessory' = 45, 'BackAccessory' = 46, 'WaistAccessory' = 47, 'ClimbAnimation' = 48, 'DeathAnimation' = 49, 'FallAnimation' = 50, 'IdleAnimation' = 51, 'JumpAnimation' = 52, 'RunAnimation' = 53, 'SwimAnimation' = 54, 'WalkAnimation' = 55, 'PoseAnimation' = 56, 'LocalizationTableManifest' = 59, 'LocalizationTableTranslation' = 60, 'EmoteAnimation' = 61, 'Video' = 62, 'TexturePack' = 63, 'TShirtAccessory' = 64, 'ShirtAccessory' = 65, 'PantsAccessory' = 66, 'JacketAccessory' = 67, 'SweaterAccessory' = 68, 'ShortsAccessory' = 69, 'LeftShoeAccessory' = 70, 'RightShoeAccessory' = 71, 'DressSkirtAccessory' = 72, 'FontFamily' = 73, 'FontFace' = 74, 'MeshHiddenSurfaceRemoval' = 75, 'EyebrowAccessory' = 76, 'EyelashAccessory' = 77, 'MoodAnimation' = 78, 'DynamicHead' = 79, 'CodeSnippet' = 80, 'AdsVideo' = 81, 'OtaUpdate' = 82, 'Screenshot' = 83, 'RuntimePropertySet' = 84, 'StorePreviewVideo' = 85, 'GamePreviewVideo' = 86, 'CreatorExperienceConfig' = 87, 'FaceMakeup' = 88, 'LipMakeup' = 89, 'EyeMakeup' = 90, 'VoxelFragment' = 91, 'AvatarBackground' = 92, 'TextDocument' = 93, 'Post' = 94, 'AnimatedImage' = 95, 'ComputeFunction' = 96\]
    RelatedEntityTypeResponseAssetTypeType {
    Image = 1 => "Image",
    TShirt = 2 => "TShirt",
    Audio = 3 => "Audio",
    Mesh = 4 => "Mesh",
    Lua = 5 => "Lua",
    Html = 6 => "HTML",
    Text = 7 => "Text",
    Hat = 8 => "Hat",
    Place = 9 => "Place",
    Model = 10 => "Model",
    Shirt = 11 => "Shirt",
    Pants = 12 => "Pants",
    Decal = 13 => "Decal",
    Avatar = 16 => "Avatar",
    Head = 17 => "Head",
    Face = 18 => "Face",
    Gear = 19 => "Gear",
    Badge = 21 => "Badge",
    GroupEmblem = 22 => "GroupEmblem",
    Animation = 24 => "Animation",
    Arms = 25 => "Arms",
    Legs = 26 => "Legs",
    Torso = 27 => "Torso",
    RightArm = 28 => "RightArm",
    LeftArm = 29 => "LeftArm",
    LeftLeg = 30 => "LeftLeg",
    RightLeg = 31 => "RightLeg",
    Package = 32 => "Package",
    YouTubeVideo = 33 => "YouTubeVideo",
    GamePass = 34 => "GamePass",
    App = 35 => "App",
    Code = 37 => "Code",
    Plugin = 38 => "Plugin",
    SolidModel = 39 => "SolidModel",
    MeshPart = 40 => "MeshPart",
    HairAccessory = 41 => "HairAccessory",
    FaceAccessory = 42 => "FaceAccessory",
    NeckAccessory = 43 => "NeckAccessory",
    ShoulderAccessory = 44 => "ShoulderAccessory",
    FrontAccessory = 45 => "FrontAccessory",
    BackAccessory = 46 => "BackAccessory",
    WaistAccessory = 47 => "WaistAccessory",
    ClimbAnimation = 48 => "ClimbAnimation",
    DeathAnimation = 49 => "DeathAnimation",
    FallAnimation = 50 => "FallAnimation",
    IdleAnimation = 51 => "IdleAnimation",
    JumpAnimation = 52 => "JumpAnimation",
    RunAnimation = 53 => "RunAnimation",
    SwimAnimation = 54 => "SwimAnimation",
    WalkAnimation = 55 => "WalkAnimation",
    PoseAnimation = 56 => "PoseAnimation",
    LocalizationTableManifest = 59 => "LocalizationTableManifest",
    LocalizationTableTranslation = 60 => "LocalizationTableTranslation",
    EmoteAnimation = 61 => "EmoteAnimation",
    Video = 62 => "Video",
    TexturePack = 63 => "TexturePack",
    TShirtAccessory = 64 => "TShirtAccessory",
    ShirtAccessory = 65 => "ShirtAccessory",
    PantsAccessory = 66 => "PantsAccessory",
    JacketAccessory = 67 => "JacketAccessory",
    SweaterAccessory = 68 => "SweaterAccessory",
    ShortsAccessory = 69 => "ShortsAccessory",
    LeftShoeAccessory = 70 => "LeftShoeAccessory",
    RightShoeAccessory = 71 => "RightShoeAccessory",
    DressSkirtAccessory = 72 => "DressSkirtAccessory",
    FontFamily = 73 => "FontFamily",
    FontFace = 74 => "FontFace",
    MeshHiddenSurfaceRemoval = 75 => "MeshHiddenSurfaceRemoval",
    EyebrowAccessory = 76 => "EyebrowAccessory",
    EyelashAccessory = 77 => "EyelashAccessory",
    MoodAnimation = 78 => "MoodAnimation",
    DynamicHead = 79 => "DynamicHead",
    CodeSnippet = 80 => "CodeSnippet",
    AdsVideo = 81 => "AdsVideo",
    OtaUpdate = 82 => "OtaUpdate",
    Screenshot = 83 => "Screenshot",
    RuntimePropertySet = 84 => "RuntimePropertySet",
    StorePreviewVideo = 85 => "StorePreviewVideo",
    GamePreviewVideo = 86 => "GamePreviewVideo",
    CreatorExperienceConfig = 87 => "CreatorExperienceConfig",
    FaceMakeup = 88 => "FaceMakeup",
    LipMakeup = 89 => "LipMakeup",
    EyeMakeup = 90 => "EyeMakeup",
    VoxelFragment = 91 => "VoxelFragment",
    AvatarBackground = 92 => "AvatarBackground",
    TextDocument = 93 => "TextDocument",
    Post = 94 => "Post",
    AnimatedImage = 95 => "AnimatedImage",
    ComputeFunction = 96 => "ComputeFunction",
    }
}

/// `Roblox.Web.Responses.RelatedEntityTypeResponse\[Roblox.Platform.Assets.AssetType\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RelatedEntityTypeResponseAssetType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// \['Image' = 1, 'TShirt' = 2, 'Audio' = 3, 'Mesh' = 4, 'Lua' = 5, 'HTML' = 6, 'Text' = 7, 'Hat' = 8, 'Place' = 9, 'Model' = 10, 'Shirt' = 11, 'Pants' = 12, 'Decal' = 13, 'Avatar' = 16, 'Head' = 17, 'Face' = 18, 'Gear' = 19, 'Badge' = 21, 'GroupEmblem' = 22, 'Animation' = 24, 'Arms' = 25, 'Legs' = 26, 'Torso' = 27, 'RightArm' = 28, 'LeftArm' = 29, 'LeftLeg' = 30, 'RightLeg' = 31, 'Package' = 32, 'YouTubeVideo' = 33, 'GamePass' = 34, 'App' = 35, 'Code' = 37, 'Plugin' = 38, 'SolidModel' = 39, 'MeshPart' = 40, 'HairAccessory' = 41, 'FaceAccessory' = 42, 'NeckAccessory' = 43, 'ShoulderAccessory' = 44, 'FrontAccessory' = 45, 'BackAccessory' = 46, 'WaistAccessory' = 47, 'ClimbAnimation' = 48, 'DeathAnimation' = 49, 'FallAnimation' = 50, 'IdleAnimation' = 51, 'JumpAnimation' = 52, 'RunAnimation' = 53, 'SwimAnimation' = 54, 'WalkAnimation' = 55, 'PoseAnimation' = 56, 'LocalizationTableManifest' = 59, 'LocalizationTableTranslation' = 60, 'EmoteAnimation' = 61, 'Video' = 62, 'TexturePack' = 63, 'TShirtAccessory' = 64, 'ShirtAccessory' = 65, 'PantsAccessory' = 66, 'JacketAccessory' = 67, 'SweaterAccessory' = 68, 'ShortsAccessory' = 69, 'LeftShoeAccessory' = 70, 'RightShoeAccessory' = 71, 'DressSkirtAccessory' = 72, 'FontFamily' = 73, 'FontFace' = 74, 'MeshHiddenSurfaceRemoval' = 75, 'EyebrowAccessory' = 76, 'EyelashAccessory' = 77, 'MoodAnimation' = 78, 'DynamicHead' = 79, 'CodeSnippet' = 80, 'AdsVideo' = 81, 'OtaUpdate' = 82, 'Screenshot' = 83, 'RuntimePropertySet' = 84, 'StorePreviewVideo' = 85, 'GamePreviewVideo' = 86, 'CreatorExperienceConfig' = 87, 'FaceMakeup' = 88, 'LipMakeup' = 89, 'EyeMakeup' = 90, 'VoxelFragment' = 91, 'AvatarBackground' = 92, 'TextDocument' = 93, 'Post' = 94, 'AnimatedImage' = 95, 'ComputeFunction' = 96\]
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<RelatedEntityTypeResponseAssetTypeType>,
}

/// `Roblox.Groups.Api.Models.Response.GroupExperienceResponse`
///
/// A response model representing an experience/game.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupExperienceResponse {
    /// When the game was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<RelatedEntityTypeResponseCreatorType>,
    /// The game description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The game (universe) Id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The game name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The number of place visits for this game.
    #[serde(rename = "placeVisits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_visits: Option<i64>,
    #[serde(rename = "rootPlace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_place: Option<RelatedEntityTypeResponseAssetType>,
    /// When the game was last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

/// `Roblox.Web.WebAPI.Models.ApiPageResponse\[Roblox.Groups.Api.GroupDetailResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiPageResponseGroupDetailResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GroupDetailResponse>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
}

crate::api_enum! {
    int
    /// The group relationship type. \['Allies' = 1, 'Enemies' = 2\]
    GroupRelationshipsV2ResponseRelationshipType {
    Allies = 1 => "Allies",
    Enemies = 2 => "Enemies",
    }
}

/// `Roblox.Groups.Api.Models.Response.GroupRelationshipsV2Response`
///
/// A group relationships response model for V2 endpoints with cursor-based pagination.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupRelationshipsV2Response {
    /// The group id.
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i64>,
    #[serde(rename = "groupResponses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_responses: Option<ApiPageResponseGroupDetailResponse>,
    /// The group relationship type. \['Allies' = 1, 'Enemies' = 2\]
    #[serde(rename = "relationshipType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_type: Option<GroupRelationshipsV2ResponseRelationshipType>,
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

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.Groups.Api.GroupMembershipResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseGroupMembershipResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GroupMembershipResponse>>,
}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.Web.Responses.Groups.GroupResponseV2\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseGroupResponseV2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GroupResponseV2>>,
}

/// `Roblox.Web.WebAPI.Models.ApiPageResponse\[Roblox.Groups.Api.Models.Response.GroupExperienceResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiPageResponseGroupExperienceResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GroupExperienceResponse>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
}

/// Query, header and form inputs for [`get_groups`].
#[derive(Debug, Clone, Default)]
pub struct GetGroupsParams {
    /// The group Ids.
    pub group_ids: Vec<i64>,
}

crate::api_enum! {
    str
    /// The order the results are sorted in.
    GetGroupsByGroupIdExperiencesSortOrder {
    Asc = 0 => "Asc",
    Desc = 1 => "Desc",
    }
}

/// Query, header and form inputs for [`get_groups_by_group_id_experiences`].
#[derive(Debug, Clone, Default)]
pub struct GetGroupsByGroupIdExperiencesParams {
    /// The access type of the experiences.
    pub access_filter: Option<i32>,
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// The order the results are sorted in.
    pub sort_order: Option<GetGroupsByGroupIdExperiencesSortOrder>,
}

/// Query, header and form inputs for [`get_groups_by_group_id_relationships_by_group_relationship_type`].
#[derive(Debug, Clone, Default)]
pub struct GetGroupsByGroupIdRelationshipsByGroupRelationshipTypeParams {
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// The order the results are sorted in.
    pub sort_order: Option<GetGroupsByGroupIdExperiencesSortOrder>,
}

/// Query, header and form inputs for [`get_groups_by_group_id_relationships_by_group_relationship_type_requests`].
#[derive(Debug, Clone, Default)]
pub struct GetGroupsByGroupIdRelationshipsByGroupRelationshipTypeRequestsParams {
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// The order the results are sorted in.
    pub sort_order: Option<GetGroupsByGroupIdExperiencesSortOrder>,
}

/// Query, header and form inputs for [`get_users_by_user_id_groups_roles`].
#[derive(Debug, Clone, Default)]
pub struct GetUsersByUserIdGroupsRolesParams {
    pub include_locked: bool,
    pub include_notification_preferences: bool,
    pub discovery_type: i32,
}

/// `GET /v2/groups`
///
/// Multi-get groups information by Ids.
///
/// If a group comes back as null, it will not be returned in the response.
pub fn get_groups_request(params: &GetGroupsParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v2/groups".to_string());
    r.query_csv("groupIds", &params.group_ids);
    Ok(r)
}

/// `GET /v2/groups`
///
/// Multi-get groups information by Ids.
///
/// If a group comes back as null, it will not be returned in the response.
pub async fn get_groups(
    client: &Client,
    params: &GetGroupsParams,
) -> Result<ApiArrayResponseGroupResponseV2> {
    client.send(get_groups_request(params)?).await?.json()
}

/// `GET /v2/groups/{groupId}/experiences`
///
/// Gets experiences created by the specified group id.
pub fn get_groups_by_group_id_experiences_request(
    group_id: i64,
    params: &GetGroupsByGroupIdExperiencesParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v2/groups/{}/experiences", crate::rt::path_value(&group_id)),
    );
    r.query("accessFilter", &params.access_filter);
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    r.query("sortOrder", &params.sort_order);
    Ok(r)
}

/// `GET /v2/groups/{groupId}/experiences`
///
/// Gets experiences created by the specified group id.
pub async fn get_groups_by_group_id_experiences(
    client: &Client,
    group_id: i64,
    params: &GetGroupsByGroupIdExperiencesParams,
) -> Result<ApiPageResponseGroupExperienceResponse> {
    client
        .send(get_groups_by_group_id_experiences_request(group_id, params)?)
        .await?
        .json()
}

/// `GET /v2/groups/{groupId}/relationships/{groupRelationshipType}`
///
/// Gets a group's relationships with cursor-based pagination.
pub fn get_groups_by_group_id_relationships_by_group_relationship_type_request(
    group_id: i64,
    group_relationship_type: &str,
    params: &GetGroupsByGroupIdRelationshipsByGroupRelationshipTypeParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v2/groups/{}/relationships/{}",
            crate::rt::path_value(&group_id),
            crate::rt::path_value(&group_relationship_type)
        ),
    );
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    r.query("sortOrder", &params.sort_order);
    Ok(r)
}

/// `GET /v2/groups/{groupId}/relationships/{groupRelationshipType}`
///
/// Gets a group's relationships with cursor-based pagination.
pub async fn get_groups_by_group_id_relationships_by_group_relationship_type(
    client: &Client,
    group_id: i64,
    group_relationship_type: &str,
    params: &GetGroupsByGroupIdRelationshipsByGroupRelationshipTypeParams,
) -> Result<GroupRelationshipsV2Response> {
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

/// `GET /v2/groups/{groupId}/relationships/{groupRelationshipType}/requests`
///
/// Gets a group's relationship requests with cursor-based pagination.
pub fn get_groups_by_group_id_relationships_by_group_relationship_type_requests_request(
    group_id: i64,
    group_relationship_type: &str,
    params: &GetGroupsByGroupIdRelationshipsByGroupRelationshipTypeRequestsParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v2/groups/{}/relationships/{}/requests",
            crate::rt::path_value(&group_id),
            crate::rt::path_value(&group_relationship_type)
        ),
    );
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    r.query("sortOrder", &params.sort_order);
    Ok(r)
}

/// `GET /v2/groups/{groupId}/relationships/{groupRelationshipType}/requests`
///
/// Gets a group's relationship requests with cursor-based pagination.
pub async fn get_groups_by_group_id_relationships_by_group_relationship_type_requests(
    client: &Client,
    group_id: i64,
    group_relationship_type: &str,
    params: &GetGroupsByGroupIdRelationshipsByGroupRelationshipTypeRequestsParams,
) -> Result<GroupRelationshipsV2Response> {
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

/// `GET /v2/users/{userId}/groups/roles`
///
/// Gets a list of all group roles for groups the specified user is in.
pub fn get_users_by_user_id_groups_roles_request(
    user_id: i64,
    params: &GetUsersByUserIdGroupsRolesParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v2/users/{}/groups/roles", crate::rt::path_value(&user_id)),
    );
    r.query("includeLocked", &params.include_locked);
    r.query(
        "includeNotificationPreferences",
        &params.include_notification_preferences,
    );
    r.query("discoveryType", &params.discovery_type);
    Ok(r)
}

/// `GET /v2/users/{userId}/groups/roles`
///
/// Gets a list of all group roles for groups the specified user is in.
pub async fn get_users_by_user_id_groups_roles(
    client: &Client,
    user_id: i64,
    params: &GetUsersByUserIdGroupsRolesParams,
) -> Result<ApiArrayResponseGroupMembershipResponse> {
    client
        .send(get_users_by_user_id_groups_roles_request(user_id, params)?)
        .await?
        .json()
}

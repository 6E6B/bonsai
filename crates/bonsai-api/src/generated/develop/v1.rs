//! `develop.roblox.com` v1 — generated from `vendor/roblox-api-tracker/develop/v1.json`.
//!
//! Do not edit. Change the spec or `bonsai-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "develop.roblox.com";

crate::api_enum! {
    int
    /// \['Invalid' = 0, 'Editors' = 1, 'PlayTesters' = 2, 'Friends' = 3, 'Public' = 4\]
    ActivationEligibilityResponseAllowedAudiences {
    Invalid = 0 => "Invalid",
    Editors = 1 => "Editors",
    PlayTesters = 2 => "PlayTesters",
    Friends = 3 => "Friends",
    Public = 4 => "Public",
    }
}

crate::api_enum! {
    int
    /// The creator tier of the universe owner. \['Invalid' = 0, 'Blocked' = 1, 'Private' = 2, 'Trusted' = 3, 'Everyone' = 4\]
    ActivationEligibilityResponseCreatorTier {
    Invalid = 0 => "Invalid",
    Blocked = 1 => "Blocked",
    Private = 2 => "Private",
    Trusted = 3 => "Trusted",
    Everyone = 4 => "Everyone",
    }
}

/// `Roblox.Api.Develop.Models.BuildGeneratedRatingPreviewResponse.AgeRecommendationModel`
///
/// Age recommendation.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AgeRecommendationModel {
    /// Explicit content maturity value.
    #[serde(rename = "contentMaturity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_maturity: Option<String>,
    /// Localized display name.
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Localized short display name including the maturity header.
    #[serde(rename = "displayNameWithHeaderShort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name_with_header_short: Option<String>,
    /// IGRS rating enum name, when present.
    #[serde(rename = "igrsRating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub igrs_rating: Option<String>,
    /// Localized IGRS rating display message, when present.
    #[serde(rename = "igrsRatingDisplayMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub igrs_rating_display_message: Option<String>,
    /// Minimum recommended age.
    #[serde(rename = "minimumAge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_age: Option<i32>,
    /// Localized minimum-age display value.
    #[serde(rename = "minimumAgeDisplay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_age_display: Option<String>,
}

/// `Roblox.Api.Develop.Models.BuildGeneratedRatingPreviewResponse.AgeRecommendationSummaryModel`
///
/// Age recommendation summary.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AgeRecommendationSummaryModel {
    #[serde(rename = "ageRecommendation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_recommendation: Option<AgeRecommendationModel>,
}

/// `Roblox.Api.Develop.Models.BuildGeneratedRatingPreviewResponse.AgeRangeModel`
///
/// Age range for a descriptor usage.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AgeRangeModel {
    /// Inclusive maximum age.
    #[serde(rename = "maxAgeInclusive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age_inclusive: Option<i32>,
    /// Inclusive minimum age.
    #[serde(rename = "minAgeInclusive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_age_inclusive: Option<i32>,
}

/// `Roblox.Api.Develop.Models.BuildGeneratedRatingPreviewResponse.ExperienceDescriptorModel`
///
/// Experience descriptor definition.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ExperienceDescriptorModel {
    /// Whether the descriptor is supported by the Compliance API.
    #[serde(rename = "complianceApiSupported")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_api_supported: Option<bool>,
    /// Localized descriptor display name.
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Descriptor icon URL.
    #[serde(rename = "iconUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// Descriptor name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.Api.Develop.Models.BuildGeneratedRatingPreviewResponse.ExperienceDescriptorDimensionUsageModel`
///
/// Descriptor dimension usage.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ExperienceDescriptorDimensionUsageModel {
    /// Dimension name.
    #[serde(rename = "dimensionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_name: Option<String>,
    /// Dimension value.
    #[serde(rename = "dimensionValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_value: Option<String>,
}

/// `Roblox.Api.Develop.Models.BuildGeneratedRatingPreviewResponse.ExperienceDescriptorUsageModel`
///
/// A descriptor usage that contributed to a recommendation.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ExperienceDescriptorUsageModel {
    #[serde(rename = "ageRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_range: Option<AgeRangeModel>,
    /// Localized age-range display name.
    #[serde(rename = "ageRangeDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_range_display_name: Option<String>,
    /// Whether the experience contains this descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains: Option<bool>,
    /// Localized descriptor display name.
    #[serde(rename = "descriptorDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptor_display_name: Option<String>,
    #[serde(rename = "experienceDescriptor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experience_descriptor: Option<ExperienceDescriptorModel>,
    /// Dimensions associated with this usage.
    #[serde(rename = "experienceDescriptorDimensionUsages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experience_descriptor_dimension_usages: Option<Vec<ExperienceDescriptorDimensionUsageModel>>,
    /// Whether this usage follows the Compliance API, when specified.
    #[serde(rename = "followsComplianceApi")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follows_compliance_api: Option<bool>,
    /// Descriptor usage name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.Api.Develop.Models.BuildGeneratedRatingPreviewResponse.AgeRecommendationDetailsModel`
///
/// Age recommendation details.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AgeRecommendationDetailsModel {
    #[serde(rename = "ageRecommendationSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_recommendation_summary: Option<AgeRecommendationSummaryModel>,
    /// Descriptor usages that contributed to the recommendation.
    #[serde(rename = "experienceDescriptorUsages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experience_descriptor_usages: Option<Vec<ExperienceDescriptorUsageModel>>,
}

/// `Roblox.Api.Develop.Models.BuildGeneratedRatingPreviewResponse.RestrictedCountryModel`
///
/// Country-specific restrictions.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RestrictedCountryModel {
    /// ISO country code.
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// Localized country display name.
    #[serde(rename = "countryDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_display_name: Option<String>,
    /// Descriptor usages responsible for the restriction.
    #[serde(rename = "experienceDescriptorUsages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experience_descriptor_usages: Option<Vec<ExperienceDescriptorUsageModel>>,
}

/// `Roblox.Api.Develop.Models.BuildGeneratedRatingPreviewResponse`
///
/// The complete build-generated rating preview returned by Experience Guidelines.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BuildGeneratedRatingPreviewResponse {
    #[serde(rename = "ageRecommendationDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_recommendation_details: Option<AgeRecommendationDetailsModel>,
    /// Country-specific restrictions produced by the preview.
    #[serde(rename = "restrictedCountries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_countries: Option<Vec<RestrictedCountryModel>>,
}

/// `Roblox.Api.Develop.Models.ActivationEligibilityResponse`
///
/// The result of various checks for a user's eligibility to activate a given universe from private to public.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ActivationEligibilityResponse {
    /// Audiences the universe is allowed to be published to.
    #[serde(rename = "allowedAudiences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_audiences: Option<Vec<ActivationEligibilityResponseAllowedAudiences>>,
    /// The creator tier of the universe owner. \['Invalid' = 0, 'Blocked' = 1, 'Private' = 2, 'Trusted' = 3, 'Everyone' = 4\]
    #[serde(rename = "creatorTier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_tier: Option<ActivationEligibilityResponseCreatorTier>,
    /// The overall result of activation eligibility. This requires the user publish gate and either a maturity rating or an intact Build seal.
    #[serde(rename = "isEligible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_eligible: Option<bool>,
    /// Whether the universe is publicly published or not.
    #[serde(rename = "isPublicPublish")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public_publish: Option<bool>,
    /// Whether the universeId is within the tracked list of distinct allowed published universes.
    #[serde(rename = "isPublishToExistingUniverse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_publish_to_existing_universe: Option<bool>,
    /// Whether the universe is in the "Select" program.
    #[serde(rename = "isUniverseSelect")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_universe_select: Option<bool>,
    /// isUserEligibleForPublicPublish
    #[serde(rename = "isUserEligibleForPublicPublish")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_user_eligible_for_public_publish: Option<bool>,
    /// Whether the place has an active content maturity rating or not.
    #[serde(rename = "maturityRated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maturity_rated: Option<bool>,
    #[serde(rename = "provisionalRating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisional_rating: Option<BuildGeneratedRatingPreviewResponse>,
    /// Whether the universe has a provisional rating via a Build seal in Content Catalog. When true, the universe is eligible for activation even without a full maturity questionnaire.
    #[serde(rename = "provisionalRatingAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisional_rating_allowed: Option<bool>,
    /// How many public publishes are remaining for the given user / group.
    #[serde(rename = "remainingPublicPublishCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_public_publish_count: Option<i32>,
}

/// `Roblox.Api.Develop.Models.UniverseModel`
///
/// Represents a universe in API endpoint results.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UniverseModel {
    /// The audiences this universe is visible to (e.g. Editors, PlayTesters, Friends, Public). Always non-null; may be empty when audience visibility has not been configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audiences: Option<Vec<ActivationEligibilityResponseAllowedAudiences>>,
    /// The created System.DateTime
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// The name of the creator of the universe.
    #[serde(rename = "creatorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_name: Option<String>,
    /// The id of the user or group that created this universe.
    #[serde(rename = "creatorTargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_target_id: Option<i64>,
    /// The creator type, either "User" or "Group"
    #[serde(rename = "creatorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_type: Option<String>,
    /// The description of the universe
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The universe Id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Is this universe active
    #[serde(rename = "isActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// Is this universe archived
    #[serde(rename = "isArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// The name of the universe
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The universe's privacy type.
    #[serde(rename = "privacyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_type: Option<String>,
    /// The universe's root place id
    #[serde(rename = "rootPlaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_place_id: Option<i64>,
    /// The updated System.DateTime
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

/// `Roblox.Api.Develop.Models.GameTemplateModel`
///
/// Represents a game template in API endpoint responses.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GameTemplateModel {
    /// The type of this game template.
    #[serde(rename = "gameTemplateType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_template_type: Option<String>,
    /// Whether this game template has tutorials.
    #[serde(rename = "hasTutorials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_tutorials: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe: Option<UniverseModel>,
}

/// `Roblox.Api.Develop.Models.GroupModel`
///
/// A model containing information about a group
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupModel {
    /// The Roblox.Platform.Groups.IGroup's Id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The Roblox.Platform.Groups.IGroup's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.Api.Develop.Models.IPlaceModel`
///
/// A model containing information about a place
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct IPlaceModel {}

/// `Roblox.Api.Develop.Models.PlaceConfigurationModel`
///
/// A model containing information about a place to be configured
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PlaceConfigurationModel {
    /// The new description for the place.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name for the place.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.Api.Develop.Models.PlaceModel`
///
/// A model containing information about a place
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PlaceModel {
    /// Returns the place description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Returns the place id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Returns the place name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Returns the id of the place's universe, or null - if the place is not part of a universe.
    #[serde(rename = "universeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_id: Option<i64>,
}

/// `Roblox.Api.Develop.Models.PlaytesterEligibility`
///
/// Whether one candidate user can currently be added as a private playtester.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PlaytesterEligibility {
    /// Whether the candidate is eligible.
    #[serde(rename = "isEligible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_eligible: Option<bool>,
    /// The candidate user id.
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}

/// `Roblox.Api.Develop.Models.PlaytesterEligibilityResponse`
///
/// Per-user add eligibility for private playtester candidates.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PlaytesterEligibilityResponse {
    /// The per-user eligibility results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playtesters: Option<Vec<PlaytesterEligibility>>,
}

/// `Roblox.Api.Develop.Models.PlaytestersRequest`
///
/// Request model for adding or removing playtesters.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PlaytestersRequest {
    /// The user ids to add or remove as playtesters for this universe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playtesters: Option<Vec<i64>>,
}

/// `Roblox.Api.Develop.Models.PlaytestersResponse`
///
/// Response model for playtester list endpoints.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PlaytestersResponse {
    /// The user ids that are playtesters for this universe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playtesters: Option<Vec<i64>>,
}

/// `Roblox.Api.Develop.Models.PrivateServerDetailsResponse`
///
/// Model for private server details responses from the UniverseSettings controller.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PrivateServerDetailsResponse {
    /// The number of active VIP servers for this universe. A negative value indicates at least this many exist (i.e. -100 means 100+ active private servers).
    #[serde(rename = "activeServersCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_servers_count: Option<i64>,
    /// The number of active VIP server subscriptions. A negative value indicates at least this many exist (i.e. -100 means 100+ active subscriptions).
    #[serde(rename = "activeSubscriptionsCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_subscriptions_count: Option<i64>,
    /// Whether or not VIP servers are enabled on this universe.
    #[serde(rename = "isEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    /// The price of the VIP server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
}

/// `Roblox.Api.Develop.Models.Response.AssetVotingModel`
///
/// Asset voting information
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AssetVotingModel {
    /// The !:IAsset's id.
    #[serde(rename = "assetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<i64>,
    /// Whether the user can vote on this !:IAsset.
    #[serde(rename = "canUserVote")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_user_vote: Option<bool>,
    /// The number of down votes.
    #[serde(rename = "downVotes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub down_votes: Option<i64>,
    /// Whether the user has voted on this !:IAsset.
    #[serde(rename = "hasUserVoted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_user_voted: Option<bool>,
    /// The reason this !:IAsset cannot be voted on.
    #[serde(rename = "reasonForNotAbleToVote")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_for_not_able_to_vote: Option<String>,
    /// Whether votes should be shown.
    #[serde(rename = "shouldShowVotes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_show_votes: Option<bool>,
    /// The number of up votes.
    #[serde(rename = "upVotes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub up_votes: Option<i64>,
}

/// `Roblox.Api.Develop.Models.Response.TeamCreateSettingsResponse`
///
/// Team create settings
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TeamCreateSettingsResponse {
    /// Whether or not the universe should be enabled for team create
    #[serde(rename = "isEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
}

/// `Roblox.Api.Develop.Models.TeamCreateMembershipRequest`
///
/// Request model for a TeamCreate membership
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TeamCreateMembershipRequest {
    /// The id of the user.
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}

/// `Roblox.Api.Develop.Models.UniverseIdPermissionsModel`
///
/// A model containing information about a universe permissions
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UniverseIdPermissionsModel {
    /// Determines whether or not consumer can cloud the target universe This is only nullable/optional in the context of https://develop.roblox.com/docs#!/Universes/get_v1_universes_universeId_context_permissions endpoint which is consumed only internally. It should be computed and set for all other usages.
    #[serde(rename = "canCloudEdit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_cloud_edit: Option<bool>,
    /// Determines whether or not consumer can manage the target universe
    #[serde(rename = "canManage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage: Option<bool>,
    /// The universe Id these permissions reference
    #[serde(rename = "universeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_id: Option<i64>,
}

/// `Roblox.Api.Develop.Models.UniversePermissionsModel`
///
/// A model containing information about a universe permissions
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UniversePermissionsModel {
    /// Determines whether or not consumer can cloud the target universe This is only nullable/optional in the context of https://develop.roblox.com/docs#!/Universes/get_v1_universes_universeId_context_permissions endpoint which is consumed only internally. It should be computed and set for all other usages.
    #[serde(rename = "canCloudEdit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_cloud_edit: Option<bool>,
    /// Determines whether or not consumer can manage the target universe
    #[serde(rename = "canManage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage: Option<bool>,
}

crate::api_enum! {
    int
    /// \['Computer' = 1, 'Phone' = 2, 'Tablet' = 3, 'Console' = 4, 'VR' = 5, 'TV' = 6\]
    UniverseSettingsRequestPlayableDevices {
    Computer = 1 => "Computer",
    Phone = 2 => "Phone",
    Tablet = 3 => "Tablet",
    Console = 4 => "Console",
    Vr = 5 => "VR",
    Tv = 6 => "TV",
    }
}

/// `Roblox.Api.Develop.Models.UniverseSettingsRequest`
///
/// Model for UniverseSettings patch requests
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UniverseSettingsRequest {
    /// Whether demo mode is enabled for this paid access game.
    #[serde(rename = "demoModeEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demo_mode_enabled: Option<bool>,
    /// Sets the base price of the experience for Fiat purchases.
    #[serde(rename = "fiatBasePriceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fiat_base_price_id: Option<String>,
    /// Determines the change type of the Fiat Product Change request. Can either Activate the product, Update the price, or Deactivate the product.
    #[serde(rename = "fiatProductChangeType")]
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fiat_product_change_type: Option<i32>,
    /// Game genre.
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<i32>,
    /// Archive status of the universe.
    #[serde(rename = "isArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// Whether the game is offered for sale.
    #[serde(rename = "isForSale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_for_sale: Option<bool>,
    /// Whether game access is limited to friends for user-owned games or group members for group-owned games.
    #[serde(rename = "isFriendsOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_friends_only: Option<bool>,
    /// Sets whether access to APIs for mesh and texture is enabled for this universe.
    #[serde(rename = "isMeshTextureApiAccessAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_mesh_texture_api_access_allowed: Option<bool>,
    /// Set is rewarded on-demand ads are allowed for this universe.
    #[serde(rename = "isRewardedOnDemandAdsAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rewarded_on_demand_ads_allowed: Option<bool>,
    /// The name of the universe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// List of device types this game can be played on.
    #[serde(rename = "playableDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playable_devices: Option<Vec<UniverseSettingsRequestPlayableDevices>>,
    /// Price of the game, in Robux.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
    /// Whether custom animations are allowed in the universe.
    #[serde(rename = "universeAnimationType")]
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_animation_type: Option<i32>,
    /// Which avatar types are allowed in the universe.
    #[serde(rename = "universeAvatarType")]
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_avatar_type: Option<i32>,
    /// What avatar body types are allowed by the universe.
    #[serde(rename = "universeBodyType")]
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_body_type: Option<i32>,
    /// What type of collisions are used by the universe.
    #[serde(rename = "universeCollisionType")]
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_collision_type: Option<i32>,
    /// What avatar joint positioning type is allowed by the universe.
    #[serde(rename = "universeJointPositioningType")]
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_joint_positioning_type: Option<i32>,
    /// Whether custom scales allowed in the universe.
    #[serde(rename = "universeScaleType")]
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_scale_type: Option<i32>,
}

crate::api_enum! {
    int
    /// The current moderation status of the game if it is for sale in fiat. \['Invalid' = 0, 'NotModerated' = 1, 'Pending' = 2, 'Approved' = 3, 'Rejected' = 4\]
    UniverseSettingsResponseFiatModerationStatus {
    Invalid = 0 => "Invalid",
    NotModerated = 1 => "NotModerated",
    Pending = 2 => "Pending",
    Approved = 3 => "Approved",
    Rejected = 4 => "Rejected",
    }
}

crate::api_enum! {
    int
    /// Game genre. \['All' = 0, 'Tutorial' = 1, 'Scary' = 2, 'TownAndCity' = 3, 'War' = 4, 'Funny' = 5, 'Fantasy' = 6, 'Adventure' = 7, 'SciFi' = 8, 'Pirate' = 9, 'FPS' = 10, 'RPG' = 11, 'Sports' = 12, 'Ninja' = 13, 'WildWest' = 14\]
    UniverseSettingsResponseGenre {
    All = 0 => "All",
    Tutorial = 1 => "Tutorial",
    Scary = 2 => "Scary",
    TownAndCity = 3 => "TownAndCity",
    War = 4 => "War",
    Funny = 5 => "Funny",
    Fantasy = 6 => "Fantasy",
    Adventure = 7 => "Adventure",
    SciFi = 8 => "SciFi",
    Pirate = 9 => "Pirate",
    Fps = 10 => "FPS",
    Rpg = 11 => "RPG",
    Sports = 12 => "Sports",
    Ninja = 13 => "Ninja",
    WildWest = 14 => "WildWest",
    }
}

crate::api_enum! {
    int
    /// Whether custom animations are allowed in the universe. \['Standard' = 1, 'PlayerChoice' = 2\]
    UniverseSettingsResponseUniverseAnimationType {
    Standard = 1 => "Standard",
    PlayerChoice = 2 => "PlayerChoice",
    }
}

crate::api_enum! {
    int
    /// Which avatar types are allowed in the universe. \['MorphToR6' = 1, 'PlayerChoice' = 2, 'MorphToR15' = 3\]
    UniverseSettingsResponseUniverseAvatarType {
    MorphToR6 = 1 => "MorphToR6",
    PlayerChoice = 2 => "PlayerChoice",
    MorphToR15 = 3 => "MorphToR15",
    }
}

crate::api_enum! {
    int
    /// What type of collisions are used by the universe. \['InnerBox' = 1, 'OuterBox' = 2\]
    UniverseSettingsResponseUniverseCollisionType {
    InnerBox = 1 => "InnerBox",
    OuterBox = 2 => "OuterBox",
    }
}

crate::api_enum! {
    int
    /// What avatar joint positioning is allowed by the universe. \['Standard' = 1, 'ArtistIntent' = 2\]
    UniverseSettingsResponseUniverseJointPositioningType {
    Standard = 1 => "Standard",
    ArtistIntent = 2 => "ArtistIntent",
    }
}

crate::api_enum! {
    int
    /// Whether custom scales allowed in the universe. \['NoScales' = 1, 'AllScales' = 2\]
    UniverseSettingsResponseUniverseScaleType {
    NoScales = 1 => "NoScales",
    AllScales = 2 => "AllScales",
    }
}

/// `Roblox.Api.Develop.Models.UniverseSettingsResponse`
///
/// Model for UniverseSettings controller responses
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UniverseSettingsResponse {
    /// If the universe allows the use of private servers.
    #[serde(rename = "allowPrivateServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_private_servers: Option<bool>,
    /// The audiences this universe is visible to (e.g. Editors, PlayTesters, Friends, Public). Always non-null; may be empty when audience visibility has not been configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audiences: Option<Vec<ActivationEligibilityResponseAllowedAudiences>>,
    /// The earliest time at which demo mode can be toggled again (ISO 8601). Null when no cooldown is active (i.e. demo mode can be changed immediately).
    #[serde(rename = "demoModeChangeableAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demo_mode_changeable_after: Option<String>,
    /// Whether demo mode is enabled.
    #[serde(rename = "demoModeEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demo_mode_enabled: Option<bool>,
    /// The basePriceId for the Fiat product.
    #[serde(rename = "fiatBasePriceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fiat_base_price_id: Option<String>,
    /// The current moderation status of the game if it is for sale in fiat. \['Invalid' = 0, 'NotModerated' = 1, 'Pending' = 2, 'Approved' = 3, 'Rejected' = 4\]
    #[serde(rename = "fiatModerationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fiat_moderation_status: Option<UniverseSettingsResponseFiatModerationStatus>,
    /// Game genre. \['All' = 0, 'Tutorial' = 1, 'Scary' = 2, 'TownAndCity' = 3, 'War' = 4, 'Funny' = 5, 'Fantasy' = 6, 'Adventure' = 7, 'SciFi' = 8, 'Pirate' = 9, 'FPS' = 10, 'RPG' = 11, 'Sports' = 12, 'Ninja' = 13, 'WildWest' = 14\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<UniverseSettingsResponseGenre>,
    /// The universe Id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Archive status of the universe
    #[serde(rename = "isArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// Whether the game is offered for sale.
    #[serde(rename = "isForSale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_for_sale: Option<bool>,
    /// Whether the game is offered for sale in fiat.
    #[serde(rename = "isForSaleInFiat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_for_sale_in_fiat: Option<bool>,
    /// Whether game access is limited to friends for user-owned games or group members for group-owned games.
    #[serde(rename = "isFriendsOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_friends_only: Option<bool>,
    /// Whether access to APIs for mesh and texture is enabled for this universe.
    #[serde(rename = "isMeshTextureApiAccessAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_mesh_texture_api_access_allowed: Option<bool>,
    /// Whether rewarded on-demand ads are allowed for this universe.
    #[serde(rename = "isRewardedOnDemandAdsAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rewarded_on_demand_ads_allowed: Option<bool>,
    /// Whether studio access to APIs is allowed or not.
    #[serde(rename = "isStudioAccessToApisAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_studio_access_to_apis_allowed: Option<bool>,
    /// The universe name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// List of device types this game can be played on.
    #[serde(rename = "playableDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playable_devices: Option<Vec<UniverseSettingsRequestPlayableDevices>>,
    /// Price of the game, in Robux.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
    /// Privacy type of the universe.
    #[serde(rename = "privacyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_type: Option<String>,
    /// The price to purchase a private server in robux.
    #[serde(rename = "privateServerPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_server_price: Option<i64>,
    /// Promotional Text for the universe.
    #[serde(rename = "promotionalText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotional_text: Option<String>,
    /// Whether custom animations are allowed in the universe. \['Standard' = 1, 'PlayerChoice' = 2\]
    #[serde(rename = "universeAnimationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_animation_type: Option<UniverseSettingsResponseUniverseAnimationType>,
    /// Which avatar types are allowed in the universe. \['MorphToR6' = 1, 'PlayerChoice' = 2, 'MorphToR15' = 3\]
    #[serde(rename = "universeAvatarType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_avatar_type: Option<UniverseSettingsResponseUniverseAvatarType>,
    /// What avatar body types are allowed by the universe. \['Standard' = 1, 'PlayerChoice' = 2\]
    #[serde(rename = "universeBodyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_body_type: Option<UniverseSettingsResponseUniverseAnimationType>,
    /// What type of collisions are used by the universe. \['InnerBox' = 1, 'OuterBox' = 2\]
    #[serde(rename = "universeCollisionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_collision_type: Option<UniverseSettingsResponseUniverseCollisionType>,
    /// What avatar joint positioning is allowed by the universe. \['Standard' = 1, 'ArtistIntent' = 2\]
    #[serde(rename = "universeJointPositioningType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_joint_positioning_type: Option<UniverseSettingsResponseUniverseJointPositioningType>,
    /// Whether custom scales allowed in the universe. \['NoScales' = 1, 'AllScales' = 2\]
    #[serde(rename = "universeScaleType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_scale_type: Option<UniverseSettingsResponseUniverseScaleType>,
}

/// `Roblox.Api.Develop.Models.UniverseTeamCreateSettingsModel`
///
/// Team create settings of a universe
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UniverseTeamCreateSettingsModel {
    /// Id of the universe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Whether or not the universe is enabled for team create
    #[serde(rename = "isEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
}

/// `Roblox.Api.Develop.Models.UpdateTeamCreateSettingsRequest`
///
/// Team create settings request
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateTeamCreateSettingsRequest {
    /// Whether or not the universe should be enabled for team create
    #[serde(rename = "isEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
}

crate::api_enum! {
    int
    /// Whether user is eligible for the developer exchange program. \['Incomplete' = 0, 'NotRequired' = 1, 'Completed' = 2\]
    UserPublicPublishEligibilityResponseHasDevex {
    Incomplete = 0 => "Incomplete",
    NotRequired = 1 => "NotRequired",
    Completed = 2 => "Completed",
    }
}

/// `Roblox.Api.Develop.Models.UserPublicPublishEligibilityResponse`
///
/// The result of various checks for a user's eligibility to publish a public universe.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UserPublicPublishEligibilityResponse {
    /// Whether user is eligible for the developer exchange program. \['Incomplete' = 0, 'NotRequired' = 1, 'Completed' = 2\]
    #[serde(rename = "hasDevex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_devex: Option<UserPublicPublishEligibilityResponseHasDevex>,
    /// Whether user has spent any Robux since 1/1/25, including gift cards. \['Incomplete' = 0, 'NotRequired' = 1, 'Completed' = 2\]
    #[serde(rename = "hasTransactions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_transactions: Option<UserPublicPublishEligibilityResponseHasDevex>,
    /// Whether user has completed ID Verification or not. \['Incomplete' = 0, 'NotRequired' = 1, 'Completed' = 2\]
    #[serde(rename = "idVerified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_verified: Option<UserPublicPublishEligibilityResponseHasDevex>,
    /// The overall result of eligibility.
    #[serde(rename = "isEligible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_eligible: Option<bool>,
}

/// `Roblox.Develop.Api.UpdatePluginRequest`
///
/// A request model for updating a plugin.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdatePluginRequest {
    /// Whether or not comments should be enabled.
    #[serde(rename = "commentsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments_enabled: Option<bool>,
    /// The new plugin description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The new plugin name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.Web.Responses.Plugins.PluginResponse`
///
/// Represents information about a plugin.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PluginResponse {
    /// Whether or not the plugin allows comments to be posted.
    #[serde(rename = "commentsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments_enabled: Option<bool>,
    /// The time the plugin was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// The plugin description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The plugin Id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The plugin name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The last time the plugin was updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    /// plugin version id
    #[serde(rename = "versionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<i64>,
}

/// `Roblox.Web.Responses.Users.SkinnyUserResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SkinnyUserResponse {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.Web.WebAPI.ApiEmptyResponseModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiEmptyResponseModel {}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.Api.Develop.Models.GameTemplateModel\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseGameTemplateModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GameTemplateModel>>,
}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.Api.Develop.Models.GroupModel\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseGroupModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GroupModel>>,
}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.Api.Develop.Models.Response.AssetVotingModel\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseAssetVotingModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<AssetVotingModel>>,
}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.Api.Develop.Models.UniverseIdPermissionsModel\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseUniverseIdPermissionsModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<UniverseIdPermissionsModel>>,
}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.Api.Develop.Models.UniverseModel\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseUniverseModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<UniverseModel>>,
}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.Api.Develop.Models.UniverseTeamCreateSettingsModel\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseUniverseTeamCreateSettingsModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<UniverseTeamCreateSettingsModel>>,
}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.Web.Responses.Plugins.PluginResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponsePluginResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<PluginResponse>>,
}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.Web.Responses.Users.SkinnyUserResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseSkinnyUserResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<SkinnyUserResponse>>,
}

/// `Roblox.Web.WebAPI.Models.ApiPageResponse\[Roblox.Api.Develop.Models.IPlaceModel\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiPageResponseIPlaceModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<IPlaceModel>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
}

/// `Roblox.Web.WebAPI.Models.ApiPageResponse\[Roblox.Api.Develop.Models.UniverseModel\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiPageResponseUniverseModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<UniverseModel>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
}

/// Query, header and form inputs for [`get_assets_voting`].
#[derive(Debug, Clone, Default)]
pub struct GetAssetsVotingParams {
    /// The ids of the Roblox.Platform.Assets.IAsset.
    pub asset_ids: Vec<i64>,
}

crate::api_enum! {
    str
    /// Sorted by universeId
    GetGroupsByGroupIdUniversesSortOrder {
    Asc = 0 => "Asc",
    Desc = 1 => "Desc",
    }
}

/// Query, header and form inputs for [`get_groups_by_group_id_universes`].
#[derive(Debug, Clone, Default)]
pub struct GetGroupsByGroupIdUniversesParams {
    /// Whether or not to return archived games.
    pub is_archived: Option<bool>,
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// Sorted by universeId
    pub sort_order: Option<GetGroupsByGroupIdUniversesSortOrder>,
}

/// Query, header and form inputs for [`get_places_by_place_id_teamcreate_active_session_members`].
#[derive(Debug, Clone, Default)]
pub struct GetPlacesByPlaceIdTeamcreateActiveSessionMembersParams {
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
}

/// Query, header and form inputs for [`get_plugins`].
#[derive(Debug, Clone, Default)]
pub struct GetPluginsParams {
    /// The plugin ids.
    pub plugin_ids: Vec<i64>,
}

/// Query, header and form inputs for [`get_universes_multiget`].
#[derive(Debug, Clone, Default)]
pub struct GetUniversesMultigetParams {
    /// The universe IDs to get. Limit 100.
    pub ids: Vec<i64>,
}

/// Query, header and form inputs for [`get_universes_multiget_permissions`].
#[derive(Debug, Clone, Default)]
pub struct GetUniversesMultigetPermissionsParams {
    /// The universe ids.
    pub ids: Vec<i64>,
}

/// Query, header and form inputs for [`get_universes_multiget_teamcreate`].
#[derive(Debug, Clone, Default)]
pub struct GetUniversesMultigetTeamcreateParams {
    /// The universe Ids.
    pub ids: Vec<i64>,
}

/// Query, header and form inputs for [`get_universes_by_universe_id_places`].
#[derive(Debug, Clone, Default)]
pub struct GetUniversesByUniverseIdPlacesParams {
    pub is_universe_creation: Option<bool>,
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// Sorted by placeId
    pub sort_order: Option<GetGroupsByGroupIdUniversesSortOrder>,
}

/// Query, header and form inputs for [`get_user_universes`].
#[derive(Debug, Clone, Default)]
pub struct GetUserUniversesParams {
    /// Whether or not to return archived games.
    pub is_archived: Option<bool>,
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// Sorted by universeId
    pub sort_order: Option<GetGroupsByGroupIdUniversesSortOrder>,
}

/// `GET /v1/assets/voting`
///
/// Gets the voting information of the given assets
#[deprecated(note = "marked deprecated by Roblox")]
pub fn get_assets_voting_request(params: &GetAssetsVotingParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/assets/voting".to_string());
    r.query_csv("assetIds", &params.asset_ids);
    Ok(r)
}

/// `GET /v1/assets/voting`
///
/// Gets the voting information of the given assets
#[deprecated(note = "marked deprecated by Roblox")]
pub async fn get_assets_voting(
    client: &Client,
    params: &GetAssetsVotingParams,
) -> Result<ApiArrayResponseAssetVotingModel> {
    client.send(get_assets_voting_request(params)?).await?.json()
}

/// `GET /v1/gametemplates`
///
/// Gets a page of templates that can be used to start off making games.
///
/// Templates subject to change without notice. Sort order of templates specified by Roblox.
pub fn get_gametemplates_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/gametemplates".to_string());
    Ok(r)
}

/// `GET /v1/gametemplates`
///
/// Gets a page of templates that can be used to start off making games.
///
/// Templates subject to change without notice. Sort order of templates specified by Roblox.
pub async fn get_gametemplates(client: &Client) -> Result<ApiArrayResponseGameTemplateModel> {
    client.send(get_gametemplates_request()?).await?.json()
}

/// `GET /v1/groups/{groupId}/universes`
///
/// Gets a list of universes for the given group.
pub fn get_groups_by_group_id_universes_request(
    group_id: i64,
    params: &GetGroupsByGroupIdUniversesParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/groups/{}/universes", crate::rt::path_value(&group_id)),
    );
    r.query("isArchived", &params.is_archived);
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    r.query("sortOrder", &params.sort_order);
    Ok(r)
}

/// `GET /v1/groups/{groupId}/universes`
///
/// Gets a list of universes for the given group.
pub async fn get_groups_by_group_id_universes(
    client: &Client,
    group_id: i64,
    params: &GetGroupsByGroupIdUniversesParams,
) -> Result<ApiPageResponseUniverseModel> {
    client
        .send(get_groups_by_group_id_universes_request(group_id, params)?)
        .await?
        .json()
}

/// `POST /v1/places/{placeId}`
///
/// Updates the place configuration for the place with the id placeId
///
/// Currently the only supported functionality for updating the configuration is around Name, and Description.
pub fn post_places_by_place_id_request(place_id: i64, body: &PlaceConfigurationModel) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!("/v1/places/{}", crate::rt::path_value(&place_id)),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/places/{placeId}`
///
/// Updates the place configuration for the place with the id placeId
///
/// Currently the only supported functionality for updating the configuration is around Name, and Description.
pub async fn post_places_by_place_id(
    client: &Client,
    place_id: i64,
    body: &PlaceConfigurationModel,
) -> Result<PlaceModel> {
    client
        .send(post_places_by_place_id_request(place_id, body)?)
        .await?
        .json()
}

/// `PATCH /v1/places/{placeId}`
///
/// Updates the place configuration for the place with the id placeId
///
/// Currently the only supported functionality for updating the configuration is around Name, and Description.
pub fn patch_places_by_place_id_request(place_id: i64, body: &PlaceConfigurationModel) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!("/v1/places/{}", crate::rt::path_value(&place_id)),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/places/{placeId}`
///
/// Updates the place configuration for the place with the id placeId
///
/// Currently the only supported functionality for updating the configuration is around Name, and Description.
pub async fn patch_places_by_place_id(
    client: &Client,
    place_id: i64,
    body: &PlaceConfigurationModel,
) -> Result<PlaceModel> {
    client
        .send(patch_places_by_place_id_request(place_id, body)?)
        .await?
        .json()
}

/// `GET /v1/places/{placeId}/teamcreate/active_session/members`
///
/// List of users in the active Team Create session
pub fn get_places_by_place_id_teamcreate_active_session_members_request(
    place_id: i64,
    params: &GetPlacesByPlaceIdTeamcreateActiveSessionMembersParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/places/{}/teamcreate/active_session/members",
            crate::rt::path_value(&place_id)
        ),
    );
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    Ok(r)
}

/// `GET /v1/places/{placeId}/teamcreate/active_session/members`
///
/// List of users in the active Team Create session
pub async fn get_places_by_place_id_teamcreate_active_session_members(
    client: &Client,
    place_id: i64,
    params: &GetPlacesByPlaceIdTeamcreateActiveSessionMembersParams,
) -> Result<ApiArrayResponseSkinnyUserResponse> {
    client
        .send(get_places_by_place_id_teamcreate_active_session_members_request(
            place_id, params,
        )?)
        .await?
        .json()
}

/// `GET /v1/plugins`
///
/// Gets plugin details by ids.
pub fn get_plugins_request(params: &GetPluginsParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/plugins".to_string());
    r.query_csv("pluginIds", &params.plugin_ids);
    Ok(r)
}

/// `GET /v1/plugins`
///
/// Gets plugin details by ids.
pub async fn get_plugins(
    client: &Client,
    params: &GetPluginsParams,
) -> Result<ApiArrayResponsePluginResponse> {
    client.send(get_plugins_request(params)?).await?.json()
}

/// `PATCH /v1/plugins/{pluginId}`
///
/// Updates a plugin.
pub fn patch_plugins_by_plugin_id_request(plugin_id: i64, body: &UpdatePluginRequest) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!("/v1/plugins/{}", crate::rt::path_value(&plugin_id)),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/plugins/{pluginId}`
///
/// Updates a plugin.
pub async fn patch_plugins_by_plugin_id(
    client: &Client,
    plugin_id: i64,
    body: &UpdatePluginRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(patch_plugins_by_plugin_id_request(plugin_id, body)?)
        .await?
        .json()
}

/// `GET /v1/universes/multiget`
///
/// Gets a list of universes.
///
/// If a universe can not be found for a given ID (such as -1) it will be skipped.
pub fn get_universes_multiget_request(params: &GetUniversesMultigetParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/universes/multiget".to_string());
    r.query("ids", &params.ids);
    Ok(r)
}

/// `GET /v1/universes/multiget`
///
/// Gets a list of universes.
///
/// If a universe can not be found for a given ID (such as -1) it will be skipped.
pub async fn get_universes_multiget(
    client: &Client,
    params: &GetUniversesMultigetParams,
) -> Result<ApiArrayResponseUniverseModel> {
    client.send(get_universes_multiget_request(params)?).await?.json()
}

/// `GET /v1/universes/multiget/permissions`
///
/// Returns an array of granted and declined permissions related to the universes with the ids in ids for the authenticated user.
///
/// If a universe can not be found for a given ID (such as -1) it will be skipped.
pub fn get_universes_multiget_permissions_request(
    params: &GetUniversesMultigetPermissionsParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        "/v1/universes/multiget/permissions".to_string(),
    );
    r.query("ids", &params.ids);
    Ok(r)
}

/// `GET /v1/universes/multiget/permissions`
///
/// Returns an array of granted and declined permissions related to the universes with the ids in ids for the authenticated user.
///
/// If a universe can not be found for a given ID (such as -1) it will be skipped.
pub async fn get_universes_multiget_permissions(
    client: &Client,
    params: &GetUniversesMultigetPermissionsParams,
) -> Result<ApiArrayResponseUniverseIdPermissionsModel> {
    client
        .send(get_universes_multiget_permissions_request(params)?)
        .await?
        .json()
}

/// `GET /v1/universes/multiget/teamcreate`
///
/// Gets TeamCreate settings for multiple universes specified by Ids
pub fn get_universes_multiget_teamcreate_request(
    params: &GetUniversesMultigetTeamcreateParams,
) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/universes/multiget/teamcreate".to_string());
    r.query("ids", &params.ids);
    Ok(r)
}

/// `GET /v1/universes/multiget/teamcreate`
///
/// Gets TeamCreate settings for multiple universes specified by Ids
pub async fn get_universes_multiget_teamcreate(
    client: &Client,
    params: &GetUniversesMultigetTeamcreateParams,
) -> Result<ApiArrayResponseUniverseTeamCreateSettingsModel> {
    client
        .send(get_universes_multiget_teamcreate_request(params)?)
        .await?
        .json()
}

/// `GET /v1/universes/user-public-publish-eligibility`
///
/// Returns the result of various checks for a user's eligibility to publish a public universe
pub fn get_universes_user_public_publish_eligibility_request() -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        "/v1/universes/user-public-publish-eligibility".to_string(),
    );
    Ok(r)
}

/// `GET /v1/universes/user-public-publish-eligibility`
///
/// Returns the result of various checks for a user's eligibility to publish a public universe
pub async fn get_universes_user_public_publish_eligibility(
    client: &Client,
) -> Result<UserPublicPublishEligibilityResponse> {
    client
        .send(get_universes_user_public_publish_eligibility_request()?)
        .await?
        .json()
}

/// `GET /v1/universes/{universeId}`
///
/// Gets a Roblox.Api.Develop.Models.UniverseModel.
pub fn get_universes_by_universe_id_request(universe_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/universes/{}", crate::rt::path_value(&universe_id)),
    );
    Ok(r)
}

/// `GET /v1/universes/{universeId}`
///
/// Gets a Roblox.Api.Develop.Models.UniverseModel.
pub async fn get_universes_by_universe_id(client: &Client, universe_id: i64) -> Result<UniverseModel> {
    client
        .send(get_universes_by_universe_id_request(universe_id)?)
        .await?
        .json()
}

/// `POST /v1/universes/{universeId}/activate`
///
/// Activates a universes.
pub fn post_universes_by_universe_id_activate_request(universe_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::POST,
        HOST,
        format!("/v1/universes/{}/activate", crate::rt::path_value(&universe_id)),
    );
    Ok(r)
}

/// `POST /v1/universes/{universeId}/activate`
///
/// Activates a universes.
pub async fn post_universes_by_universe_id_activate(
    client: &Client,
    universe_id: i64,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_universes_by_universe_id_activate_request(universe_id)?)
        .await?
        .json()
}

/// `GET /v1/universes/{universeId}/activation-eligibility`
///
/// Returns the result of various checks for a user's eligibility to activate a given universe from private to public universeId for authenticated user
pub fn get_universes_by_universe_id_activation_eligibility_request(universe_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/universes/{}/activation-eligibility",
            crate::rt::path_value(&universe_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/universes/{universeId}/activation-eligibility`
///
/// Returns the result of various checks for a user's eligibility to activate a given universe from private to public universeId for authenticated user
pub async fn get_universes_by_universe_id_activation_eligibility(
    client: &Client,
    universe_id: i64,
) -> Result<ActivationEligibilityResponse> {
    client
        .send(get_universes_by_universe_id_activation_eligibility_request(
            universe_id,
        )?)
        .await?
        .json()
}

/// `GET /v1/universes/{universeId}/configuration`
///
/// Get settings for an owned universe.
pub fn get_universes_by_universe_id_configuration_request(universe_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/universes/{}/configuration",
            crate::rt::path_value(&universe_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/universes/{universeId}/configuration`
///
/// Get settings for an owned universe.
pub async fn get_universes_by_universe_id_configuration(
    client: &Client,
    universe_id: i64,
) -> Result<UniverseSettingsResponse> {
    client
        .send(get_universes_by_universe_id_configuration_request(universe_id)?)
        .await?
        .json()
}

/// `PATCH /v1/universes/{universeId}/configuration`
///
/// Update universe settings for an owned universe.
pub fn patch_universes_by_universe_id_configuration_request(
    universe_id: i64,
    body: &UniverseSettingsRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!(
            "/v1/universes/{}/configuration",
            crate::rt::path_value(&universe_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/universes/{universeId}/configuration`
///
/// Update universe settings for an owned universe.
pub async fn patch_universes_by_universe_id_configuration(
    client: &Client,
    universe_id: i64,
    body: &UniverseSettingsRequest,
) -> Result<UniverseSettingsResponse> {
    client
        .send(patch_universes_by_universe_id_configuration_request(
            universe_id,
            body,
        )?)
        .await?
        .json()
}

/// `GET /v1/universes/{universeId}/configuration/playtesters`
///
/// Get the list of playtesters for an owned universe.
pub fn get_universes_by_universe_id_configuration_playtesters_request(universe_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/universes/{}/configuration/playtesters",
            crate::rt::path_value(&universe_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/universes/{universeId}/configuration/playtesters`
///
/// Get the list of playtesters for an owned universe.
pub async fn get_universes_by_universe_id_configuration_playtesters(
    client: &Client,
    universe_id: i64,
) -> Result<PlaytestersResponse> {
    client
        .send(get_universes_by_universe_id_configuration_playtesters_request(
            universe_id,
        )?)
        .await?
        .json()
}

/// `POST /v1/universes/{universeId}/configuration/playtesters`
///
/// Add playtesters to an owned universe. Existing ids are ignored (additive union).
pub fn post_universes_by_universe_id_configuration_playtesters_request(
    universe_id: i64,
    body: &PlaytestersRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/universes/{}/configuration/playtesters",
            crate::rt::path_value(&universe_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/universes/{universeId}/configuration/playtesters`
///
/// Add playtesters to an owned universe. Existing ids are ignored (additive union).
pub async fn post_universes_by_universe_id_configuration_playtesters(
    client: &Client,
    universe_id: i64,
    body: &PlaytestersRequest,
) -> Result<PlaytestersResponse> {
    client
        .send(post_universes_by_universe_id_configuration_playtesters_request(
            universe_id,
            body,
        )?)
        .await?
        .json()
}

/// `DELETE /v1/universes/{universeId}/configuration/playtesters`
///
/// Remove playtesters from an owned universe.
pub fn delete_universes_by_universe_id_configuration_playtesters_request(
    universe_id: i64,
    body: &PlaytestersRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::DELETE,
        HOST,
        format!(
            "/v1/universes/{}/configuration/playtesters",
            crate::rt::path_value(&universe_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `DELETE /v1/universes/{universeId}/configuration/playtesters`
///
/// Remove playtesters from an owned universe.
pub async fn delete_universes_by_universe_id_configuration_playtesters(
    client: &Client,
    universe_id: i64,
    body: &PlaytestersRequest,
) -> Result<PlaytestersResponse> {
    client
        .send(delete_universes_by_universe_id_configuration_playtesters_request(
            universe_id,
            body,
        )?)
        .await?
        .json()
}

/// `POST /v1/universes/{universeId}/configuration/playtesters/eligibility`
///
/// Check whether each candidate user can currently be added as a private playtester.
pub fn post_universes_by_universe_id_configuration_playtesters_eligibility_request(
    universe_id: i64,
    body: &PlaytestersRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/universes/{}/configuration/playtesters/eligibility",
            crate::rt::path_value(&universe_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/universes/{universeId}/configuration/playtesters/eligibility`
///
/// Check whether each candidate user can currently be added as a private playtester.
pub async fn post_universes_by_universe_id_configuration_playtesters_eligibility(
    client: &Client,
    universe_id: i64,
    body: &PlaytestersRequest,
) -> Result<PlaytesterEligibilityResponse> {
    client
        .send(post_universes_by_universe_id_configuration_playtesters_eligibility_request(universe_id, body)?)
        .await?
        .json()
}

/// `GET /v1/universes/{universeId}/configuration/vip-servers`
///
/// Get settings for an owned universe's VIP servers.
pub fn get_universes_by_universe_id_configuration_vip_servers_request(universe_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/universes/{}/configuration/vip-servers",
            crate::rt::path_value(&universe_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/universes/{universeId}/configuration/vip-servers`
///
/// Get settings for an owned universe's VIP servers.
pub async fn get_universes_by_universe_id_configuration_vip_servers(
    client: &Client,
    universe_id: i64,
) -> Result<PrivateServerDetailsResponse> {
    client
        .send(get_universes_by_universe_id_configuration_vip_servers_request(
            universe_id,
        )?)
        .await?
        .json()
}

/// `POST /v1/universes/{universeId}/deactivate`
///
/// Deactivates a universe.
pub fn post_universes_by_universe_id_deactivate_request(universe_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::POST,
        HOST,
        format!("/v1/universes/{}/deactivate", crate::rt::path_value(&universe_id)),
    );
    Ok(r)
}

/// `POST /v1/universes/{universeId}/deactivate`
///
/// Deactivates a universe.
pub async fn post_universes_by_universe_id_deactivate(
    client: &Client,
    universe_id: i64,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_universes_by_universe_id_deactivate_request(universe_id)?)
        .await?
        .json()
}

/// `GET /v1/universes/{universeId}/permissions`
///
/// Returns list of granted and declined permissions related to the universe with the id universeId for authenticated user
pub fn get_universes_by_universe_id_permissions_request(universe_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/universes/{}/permissions",
            crate::rt::path_value(&universe_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/universes/{universeId}/permissions`
///
/// Returns list of granted and declined permissions related to the universe with the id universeId for authenticated user
pub async fn get_universes_by_universe_id_permissions(
    client: &Client,
    universe_id: i64,
) -> Result<UniversePermissionsModel> {
    client
        .send(get_universes_by_universe_id_permissions_request(universe_id)?)
        .await?
        .json()
}

/// `GET /v1/universes/{universeId}/places`
///
/// Gets a list of places for a universe.
pub fn get_universes_by_universe_id_places_request(
    universe_id: i64,
    params: &GetUniversesByUniverseIdPlacesParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/universes/{}/places", crate::rt::path_value(&universe_id)),
    );
    r.query("isUniverseCreation", &params.is_universe_creation);
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    r.query("sortOrder", &params.sort_order);
    Ok(r)
}

/// `GET /v1/universes/{universeId}/places`
///
/// Gets a list of places for a universe.
pub async fn get_universes_by_universe_id_places(
    client: &Client,
    universe_id: i64,
    params: &GetUniversesByUniverseIdPlacesParams,
) -> Result<ApiPageResponseIPlaceModel> {
    client
        .send(get_universes_by_universe_id_places_request(universe_id, params)?)
        .await?
        .json()
}

/// `GET /v1/universes/{universeId}/teamcreate`
///
/// Gets TeamCreate settings for an Roblox.Platform.Universes.IUniverse.
pub fn get_universes_by_universe_id_teamcreate_request(universe_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/universes/{}/teamcreate", crate::rt::path_value(&universe_id)),
    );
    Ok(r)
}

/// `GET /v1/universes/{universeId}/teamcreate`
///
/// Gets TeamCreate settings for an Roblox.Platform.Universes.IUniverse.
pub async fn get_universes_by_universe_id_teamcreate(
    client: &Client,
    universe_id: i64,
) -> Result<TeamCreateSettingsResponse> {
    client
        .send(get_universes_by_universe_id_teamcreate_request(universe_id)?)
        .await?
        .json()
}

/// `PATCH /v1/universes/{universeId}/teamcreate`
///
/// Edit team create settings for a universe.
///
/// Enables, or disables team create for a universe.
pub fn patch_universes_by_universe_id_teamcreate_request(
    universe_id: i64,
    body: &UpdateTeamCreateSettingsRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!("/v1/universes/{}/teamcreate", crate::rt::path_value(&universe_id)),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/universes/{universeId}/teamcreate`
///
/// Edit team create settings for a universe.
///
/// Enables, or disables team create for a universe.
pub async fn patch_universes_by_universe_id_teamcreate(
    client: &Client,
    universe_id: i64,
    body: &UpdateTeamCreateSettingsRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(patch_universes_by_universe_id_teamcreate_request(
            universe_id,
            body,
        )?)
        .await?
        .json()
}

/// `DELETE /v1/universes/{universeId}/teamcreate/memberships`
///
/// Removes a user from a TeamCreate permissions list.
pub fn delete_universes_by_universe_id_teamcreate_memberships_request(
    universe_id: i64,
    body: &TeamCreateMembershipRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::DELETE,
        HOST,
        format!(
            "/v1/universes/{}/teamcreate/memberships",
            crate::rt::path_value(&universe_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `DELETE /v1/universes/{universeId}/teamcreate/memberships`
///
/// Removes a user from a TeamCreate permissions list.
pub async fn delete_universes_by_universe_id_teamcreate_memberships(
    client: &Client,
    universe_id: i64,
    body: &TeamCreateMembershipRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(delete_universes_by_universe_id_teamcreate_memberships_request(
            universe_id,
            body,
        )?)
        .await?
        .json()
}

/// `GET /v1/user/groups/canmanage`
///
/// Gets a list of Groups that a user can manage.
pub fn get_user_groups_canmanage_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/user/groups/canmanage".to_string());
    Ok(r)
}

/// `GET /v1/user/groups/canmanage`
///
/// Gets a list of Groups that a user can manage.
pub async fn get_user_groups_canmanage(client: &Client) -> Result<ApiArrayResponseGroupModel> {
    client.send(get_user_groups_canmanage_request()?).await?.json()
}

/// `GET /v1/user/groups/canmanagegamesoritems`
///
/// Gets a list of groups a user can manage games or items for.
pub fn get_user_groups_canmanagegamesoritems_request() -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        "/v1/user/groups/canmanagegamesoritems".to_string(),
    );
    Ok(r)
}

/// `GET /v1/user/groups/canmanagegamesoritems`
///
/// Gets a list of groups a user can manage games or items for.
pub async fn get_user_groups_canmanagegamesoritems(client: &Client) -> Result<ApiArrayResponseGroupModel> {
    client
        .send(get_user_groups_canmanagegamesoritems_request()?)
        .await?
        .json()
}

/// `GET /v1/user/universes`
///
/// Gets a list of universes for the authenticated user.
pub fn get_user_universes_request(params: &GetUserUniversesParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/user/universes".to_string());
    r.query("isArchived", &params.is_archived);
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    r.query("sortOrder", &params.sort_order);
    Ok(r)
}

/// `GET /v1/user/universes`
///
/// Gets a list of universes for the authenticated user.
pub async fn get_user_universes(
    client: &Client,
    params: &GetUserUniversesParams,
) -> Result<ApiPageResponseUniverseModel> {
    client.send(get_user_universes_request(params)?).await?.json()
}

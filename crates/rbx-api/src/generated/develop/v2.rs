//! `develop.roblox.com` v2 — generated from `vendor/roblox-api-tracker/develop/v2.json`.
//!
//! Do not edit. Change the spec or `rbx-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "develop.roblox.com";

/// `Roblox.Api.Develop.AssetVersion`
///
/// Model of an asset version.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AssetVersion {
    /// The VersionID of the asset version.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The ID of the asset.
    #[serde(rename = "assetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<i64>,
    /// The version number.
    #[serde(rename = "assetVersionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_version_number: Option<i32>,
    /// The created date of this asset version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// ID of the universe this asset version was created in.
    #[serde(rename = "creatingUniverseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creating_universe_id: Option<i64>,
    /// ID of the asset version creator.
    #[serde(rename = "creatorTargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_target_id: Option<i64>,
    /// Type of the asset version creator.
    #[serde(rename = "creatorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_type: Option<String>,
    /// Indicates if this version is same to current published version. This property is available on /v1/{assetId}/published-versions and /v1/{assetId}/version/{versionNumber}.
    #[serde(rename = "isEqualToCurrentPublishedVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_equal_to_current_published_version: Option<bool>,
    /// Indicates if this version is / was published. This property is available on /v1/{assetId}/saved-versions. This should be true for all assets coming from GetAssetPublishedVersionsByAssetId
    #[serde(rename = "isPublished")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_published: Option<bool>,
}

/// `Roblox.Api.Develop.Models.PlaceConfigurationModelV2`
///
/// A model containing information about a place to be configured
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PlaceConfigurationModelV2 {
    /// Determines if copying of the place is allowed.
    #[serde(rename = "allowCopying")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_copying: Option<bool>,
    /// List of allowed gear types
    #[serde(rename = "allowedGearTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_gear_types: Option<Vec<String>>,
    /// The number of social slots for the place when the slot type is custom.
    #[serde(rename = "customSocialSlotsCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_social_slots_count: Option<i32>,
    /// The new description for the place.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// If all genres are allowed, or only the experience type
    #[serde(rename = "isAllGenresAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_all_genres_allowed: Option<bool>,
    /// The max number of players for the place.
    #[serde(rename = "maxPlayerCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_player_count: Option<i32>,
    /// The name for the place.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The social slot type for the place. Determines how users are placed into servers. Examples: Automatic, Empty, Custom
    #[serde(rename = "socialSlotType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_slot_type: Option<String>,
}

crate::api_enum! {
    int
    /// The types of Gear that can be worn inside a place. Valid values are derived from amp::AssetCategory \['Invalid' = 0, 'Melee' = 8, 'Ranged' = 9, 'Explosive' = 10, 'PowerUps' = 11, 'Navigation' = 12, 'Musical' = 13, 'Social' = 14, 'Building' = 21, 'PersonalTransport' = 22\]
    PlaceModelV2AllowedGearTypes {
    Invalid = 0 => "Invalid",
    Melee = 8 => "Melee",
    Ranged = 9 => "Ranged",
    Explosive = 10 => "Explosive",
    PowerUps = 11 => "PowerUps",
    Navigation = 12 => "Navigation",
    Musical = 13 => "Musical",
    Social = 14 => "Social",
    Building = 21 => "Building",
    PersonalTransport = 22 => "PersonalTransport",
    }
}

/// `Roblox.Api.Develop.Models.PlaceModelV2`
///
/// A model containing information about a place
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PlaceModelV2 {
    /// Determines if copying of the place is allowed.
    #[serde(rename = "allowCopying")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_copying: Option<bool>,
    /// Types of Roblox gear that are allowed to exist in the place. Valid values are from amp::AssetCategory
    #[serde(rename = "allowedGearTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_gear_types: Option<Vec<PlaceModelV2AllowedGearTypes>>,
    /// The time place was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// The current saved version number of the place.
    #[serde(rename = "currentSavedVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_saved_version: Option<i32>,
    /// The number of social slots for the place when the slot type is custom.
    #[serde(rename = "customSocialSlotsCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_social_slots_count: Option<i32>,
    /// Returns the place description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Returns the place id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Whether all genres allowed in the place.
    #[serde(rename = "isAllGenresAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_all_genres_allowed: Option<bool>,
    /// Returns whether this place is the root place.
    #[serde(rename = "isRootPlace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_root_place: Option<bool>,
    /// The max number of players for the place.
    #[serde(rename = "maxPlayerCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_player_count: Option<i32>,
    /// The maximum allowed number of players for the place that the user can set, based on user roleset.
    #[serde(rename = "maxPlayersAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_players_allowed: Option<i32>,
    /// Returns the place name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The social slot type for the place. Determines how users are placed into servers. Examples: Automatic, Empty, Custom
    #[serde(rename = "socialSlotType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_slot_type: Option<String>,
    /// Returns the id of the place's universe, or null - if the place is not part of a universe.
    #[serde(rename = "universeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_id: Option<i64>,
    /// The time place was updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

crate::api_enum! {
    int
    /// The region policy label \['Unknown' = 0, 'China' = 1\]
    UniverseModerationPolicyStatusRegion {
    UnknownValue = 0 => "Unknown",
    China = 1 => "China",
    }
}

/// `Roblox.Api.Develop.Models.UniverseModerationPolicyStatus`
///
/// Represents a universe moderation policy status
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UniverseModerationPolicyStatus {
    /// The region policy label \['Unknown' = 0, 'China' = 1\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<UniverseModerationPolicyStatusRegion>,
    /// The status of the universe
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

crate::api_enum! {
    int
    /// \['Invalid' = 0, 'Editors' = 1, 'PlayTesters' = 2, 'Friends' = 3, 'Public' = 4\]
    UniverseSettingsRequestV2Audiences {
    Invalid = 0 => "Invalid",
    Editors = 1 => "Editors",
    PlayTesters = 2 => "PlayTesters",
    Friends = 3 => "Friends",
    Public = 4 => "Public",
    }
}

/// `Roblox.UniversePluginPermissionAuthority.Models.UniversePluginPermissions`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UniversePluginPermissions {
    #[serde(rename = "IsClientTeleportAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_client_teleport_allowed: Option<bool>,
    #[serde(rename = "IsThirdPartyAssetAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_third_party_asset_allowed: Option<bool>,
    #[serde(rename = "IsThirdPartyPurchaseAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_third_party_purchase_allowed: Option<bool>,
    #[serde(rename = "IsThirdPartyTeleportAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_third_party_teleport_allowed: Option<bool>,
}

crate::api_enum! {
    int
    /// \['Computer' = 1, 'Phone' = 2, 'Tablet' = 3, 'Console' = 4, 'VR' = 5, 'TV' = 6\]
    UniverseSettingsRequestV2PlayableDevices {
    Computer = 1 => "Computer",
    Phone = 2 => "Phone",
    Tablet = 3 => "Tablet",
    Console = 4 => "Console",
    Vr = 5 => "VR",
    Tv = 6 => "TV",
    }
}

/// `Roblox.Platform.UniverseSettings.UniverseAvatarAssetOverrideResponseModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UniverseAvatarAssetOverrideResponseModel {
    #[serde(rename = "assetID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<i64>,
    #[serde(rename = "assetTypeID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type_id: Option<i32>,
    #[serde(rename = "isPlayerChoice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_player_choice: Option<bool>,
}

/// `Roblox.Web.Responses.Avatar.ScaleModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ScaleModel {
    #[serde(rename = "bodyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_type: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proportion: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
}

/// `Roblox.Api.Develop.Models.UniverseSettingsRequestV2`
///
/// Model for UniverseSettings patch requests
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UniverseSettingsRequestV2 {
    /// If the universe allows the use of private servers.
    #[serde(rename = "allowPrivateServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_private_servers: Option<bool>,
    /// The audiences this universe should be visible to (e.g. Editors, PlayTesters, Friends, Public). When provided, replaces the universe's existing audience set with the supplied values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audiences: Option<Vec<UniverseSettingsRequestV2Audiences>>,
    /// Whether demo mode is enabled for this paid access game.
    #[serde(rename = "demoModeEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demo_mode_enabled: Option<bool>,
    /// The description of the universe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional JSON string used to store avatar settings used by the game engine. Will not be updated if null or empty. To clear settings, set to empty JSON object "{}". Note: This is an experimental field which may be changed or removed in future.
    #[serde(rename = "engineAvatarSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_avatar_settings: Option<String>,
    /// Sets the base price of the experience for Fiat purchases.
    #[serde(rename = "fiatBasePriceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fiat_base_price_id: Option<String>,
    /// Determines the change type of the Fiat Product Change request. Can either Activate, Update Base Price, or Deactivate.
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
    /// Whether rewarded on-demand ads are allowed for this universe.
    #[serde(rename = "isRewardedOnDemandAdsAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rewarded_on_demand_ads_allowed: Option<bool>,
    /// The name of the universe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A list of opt in region.
    #[serde(rename = "optInRegions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_in_regions: Option<Vec<UniverseModerationPolicyStatusRegion>>,
    /// A list of opt out region.
    #[serde(rename = "optOutRegions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_out_regions: Option<Vec<UniverseModerationPolicyStatusRegion>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<UniversePluginPermissions>,
    /// List of device types this game can be played on.
    #[serde(rename = "playableDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playable_devices: Option<Vec<UniverseSettingsRequestV2PlayableDevices>>,
    /// Price of the game, in Robux.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
    /// The price to purchase a private server in robux.
    #[serde(rename = "privateServerPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_server_price: Option<i64>,
    /// Promotional text for the universe. Empty stores an empty value.
    #[serde(rename = "promotionalText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotional_text: Option<String>,
    /// Whether Studio can access data stores of this universe.
    #[serde(rename = "studioAccessToApisAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub studio_access_to_apis_allowed: Option<bool>,
    /// Whether custom animations are allowed in the universe.
    #[serde(rename = "universeAnimationType")]
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_animation_type: Option<i32>,
    /// A list of avatar asset overrides.
    #[serde(rename = "universeAvatarAssetOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_avatar_asset_overrides: Option<Vec<UniverseAvatarAssetOverrideResponseModel>>,
    #[serde(rename = "universeAvatarMaxScales")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_avatar_max_scales: Option<ScaleModel>,
    #[serde(rename = "universeAvatarMinScales")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_avatar_min_scales: Option<ScaleModel>,
    /// Which avatar types are allowed in the universe.
    #[serde(rename = "universeAvatarType")]
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_avatar_type: Option<i32>,
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
}

crate::api_enum! {
    int
    /// The current moderation status of the game if it is for sale in fiat. \['Invalid' = 0, 'NotModerated' = 1, 'Pending' = 2, 'Approved' = 3, 'Rejected' = 4\]
    UniverseSettingsResponseV2FiatModerationStatus {
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
    UniverseSettingsResponseV2Genre {
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
    UniverseSettingsResponseV2UniverseAnimationType {
    Standard = 1 => "Standard",
    PlayerChoice = 2 => "PlayerChoice",
    }
}

crate::api_enum! {
    int
    /// Which avatar types are allowed in the universe. \['MorphToR6' = 1, 'PlayerChoice' = 2, 'MorphToR15' = 3\]
    UniverseSettingsResponseV2UniverseAvatarType {
    MorphToR6 = 1 => "MorphToR6",
    PlayerChoice = 2 => "PlayerChoice",
    MorphToR15 = 3 => "MorphToR15",
    }
}

crate::api_enum! {
    int
    /// What type of collisions are used by the universe. \['InnerBox' = 1, 'OuterBox' = 2\]
    UniverseSettingsResponseV2UniverseCollisionType {
    InnerBox = 1 => "InnerBox",
    OuterBox = 2 => "OuterBox",
    }
}

crate::api_enum! {
    int
    /// What avatar joint positioning is allowed by the universe. \['Standard' = 1, 'ArtistIntent' = 2\]
    UniverseSettingsResponseV2UniverseJointPositioningType {
    Standard = 1 => "Standard",
    ArtistIntent = 2 => "ArtistIntent",
    }
}

/// `Roblox.Api.Develop.Models.UniverseSettingsResponseV2`
///
/// Model for UniverseSettings controller responses
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UniverseSettingsResponseV2 {
    /// If the universe allows the use of private servers.
    #[serde(rename = "allowPrivateServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_private_servers: Option<bool>,
    /// The audiences this universe is visible to (e.g. Editors, PlayTesters, Friends, Public). Always non-null; may be empty when audience visibility has not been configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audiences: Option<Vec<UniverseSettingsRequestV2Audiences>>,
    /// The earliest time at which demo mode can be toggled again (ISO 8601). Null when no cooldown is active (i.e. demo mode can be changed immediately).
    #[serde(rename = "demoModeChangeableAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demo_mode_changeable_after: Option<String>,
    /// Whether demo mode is enabled.
    #[serde(rename = "demoModeEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demo_mode_enabled: Option<bool>,
    /// The universe description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional JSON string used to store avatar settings used by the game engine. Note: This is an experimental field which may be changed or removed in future.
    #[serde(rename = "engineAvatarSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_avatar_settings: Option<String>,
    /// The basePriceId for the Fiat product.
    #[serde(rename = "fiatBasePriceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fiat_base_price_id: Option<String>,
    /// The current moderation status of the game if it is for sale in fiat. \['Invalid' = 0, 'NotModerated' = 1, 'Pending' = 2, 'Approved' = 3, 'Rejected' = 4\]
    #[serde(rename = "fiatModerationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fiat_moderation_status: Option<UniverseSettingsResponseV2FiatModerationStatus>,
    /// Game genre. \['All' = 0, 'Tutorial' = 1, 'Scary' = 2, 'TownAndCity' = 3, 'War' = 4, 'Funny' = 5, 'Fantasy' = 6, 'Adventure' = 7, 'SciFi' = 8, 'Pirate' = 9, 'FPS' = 10, 'RPG' = 11, 'Sports' = 12, 'Ninja' = 13, 'WildWest' = 14\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<UniverseSettingsResponseV2Genre>,
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
    /// The universe name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The regions the universe has opted in for
    #[serde(rename = "optInRegions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_in_regions: Option<Vec<UniverseModerationPolicyStatus>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<UniversePluginPermissions>,
    /// List of device types this game can be played on.
    #[serde(rename = "playableDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playable_devices: Option<Vec<UniverseSettingsRequestV2PlayableDevices>>,
    /// Price of the game, in Robux.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
    /// The price to purchase a private server in robux.
    #[serde(rename = "privateServerPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_server_price: Option<i64>,
    /// Promotional Text for the universe.
    #[serde(rename = "promotionalText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotional_text: Option<String>,
    /// Whether Studio can access data stores of this universe.
    #[serde(rename = "studioAccessToApisAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub studio_access_to_apis_allowed: Option<bool>,
    /// Whether custom animations are allowed in the universe. \['Standard' = 1, 'PlayerChoice' = 2\]
    #[serde(rename = "universeAnimationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_animation_type: Option<UniverseSettingsResponseV2UniverseAnimationType>,
    /// A collection of avatar asset settings allowed by the universe.
    #[serde(rename = "universeAvatarAssetOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_avatar_asset_overrides: Option<Vec<UniverseAvatarAssetOverrideResponseModel>>,
    #[serde(rename = "universeAvatarMaxScales")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_avatar_max_scales: Option<ScaleModel>,
    #[serde(rename = "universeAvatarMinScales")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_avatar_min_scales: Option<ScaleModel>,
    /// Which avatar types are allowed in the universe. \['MorphToR6' = 1, 'PlayerChoice' = 2, 'MorphToR15' = 3\]
    #[serde(rename = "universeAvatarType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_avatar_type: Option<UniverseSettingsResponseV2UniverseAvatarType>,
    /// What type of collisions are used by the universe. \['InnerBox' = 1, 'OuterBox' = 2\]
    #[serde(rename = "universeCollisionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_collision_type: Option<UniverseSettingsResponseV2UniverseCollisionType>,
    /// What avatar joint positioning is allowed by the universe. \['Standard' = 1, 'ArtistIntent' = 2\]
    #[serde(rename = "universeJointPositioningType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_joint_positioning_type: Option<UniverseSettingsResponseV2UniverseJointPositioningType>,
}

/// `Roblox.Web.WebAPI.ApiEmptyResponseModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiEmptyResponseModel {}

/// `Roblox.Web.WebAPI.Models.ApiPageResponse\[Roblox.Api.Develop.AssetVersion\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiPageResponseAssetVersion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<AssetVersion>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
}

crate::api_enum! {
    str
    /// Sort by version number, default is desc.
    GetAssetsByIdVersionsSortOrder {
    Asc = 0 => "Asc",
    Desc = 1 => "Desc",
    }
}

/// Query, header and form inputs for [`get_assets_by_id_versions`].
#[derive(Debug, Clone, Default)]
pub struct GetAssetsByIdVersionsParams {
    /// The ID of the place.Roblox.Platform.Assets.IPlace
    pub roblox_place_id: i64,
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// Sort by version number, default is desc.
    pub sort_order: Option<GetAssetsByIdVersionsSortOrder>,
}

/// Query, header and form inputs for [`delete_teamtest_by_place_id`].
#[derive(Debug, Clone, Default)]
pub struct DeleteTeamtestByPlaceIdParams {
    /// the Guid of the game instance System.Guid
    pub game_id: String,
}

/// `GET /v2/assets/{id}/versions`
///
/// Retrieves asset information for the specified asset ID. The authenticated user must be able to manage the asset or granted by package permission.
#[deprecated(note = "marked deprecated by Roblox")]
pub fn get_assets_by_id_versions_request(id: i64, params: &GetAssetsByIdVersionsParams) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v2/assets/{}/versions", crate::rt::path_value(&id)),
    );
    r.header("Roblox-Place-Id", &params.roblox_place_id);
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    r.query("sortOrder", &params.sort_order);
    Ok(r)
}

/// `GET /v2/assets/{id}/versions`
///
/// Retrieves asset information for the specified asset ID. The authenticated user must be able to manage the asset or granted by package permission.
#[deprecated(note = "marked deprecated by Roblox")]
pub async fn get_assets_by_id_versions(
    client: &Client,
    id: i64,
    params: &GetAssetsByIdVersionsParams,
) -> Result<ApiPageResponseAssetVersion> {
    client
        .send(get_assets_by_id_versions_request(id, params)?)
        .await?
        .json()
}

/// `GET /v2/places/{placeId}`
///
/// Gets the place configuration for the place with the id placeId
pub fn get_places_by_place_id_request(place_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v2/places/{}", crate::rt::path_value(&place_id)),
    );
    Ok(r)
}

/// `GET /v2/places/{placeId}`
///
/// Gets the place configuration for the place with the id placeId
pub async fn get_places_by_place_id(client: &Client, place_id: i64) -> Result<PlaceModelV2> {
    client
        .send(get_places_by_place_id_request(place_id)?)
        .await?
        .json()
}

/// `PATCH /v2/places/{placeId}`
///
/// Updates the place configuration for the place with the id placeId
pub fn patch_places_by_place_id_request(place_id: i64, body: &PlaceConfigurationModelV2) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!("/v2/places/{}", crate::rt::path_value(&place_id)),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v2/places/{placeId}`
///
/// Updates the place configuration for the place with the id placeId
pub async fn patch_places_by_place_id(
    client: &Client,
    place_id: i64,
    body: &PlaceConfigurationModelV2,
) -> Result<PlaceModelV2> {
    client
        .send(patch_places_by_place_id_request(place_id, body)?)
        .await?
        .json()
}

/// `DELETE /v2/teamtest/{placeId}`
///
/// Close a game instance that is being used for team testing
pub fn delete_teamtest_by_place_id_request(
    place_id: i64,
    params: &DeleteTeamtestByPlaceIdParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::DELETE,
        HOST,
        format!("/v2/teamtest/{}", crate::rt::path_value(&place_id)),
    );
    r.query("gameId", &params.game_id);
    Ok(r)
}

/// `DELETE /v2/teamtest/{placeId}`
///
/// Close a game instance that is being used for team testing
pub async fn delete_teamtest_by_place_id(
    client: &Client,
    place_id: i64,
    params: &DeleteTeamtestByPlaceIdParams,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(delete_teamtest_by_place_id_request(place_id, params)?)
        .await?
        .json()
}

/// `PATCH /v2/universes/{universeId}/configuration`
///
/// Update universe settings for an owned universe. V2 Contains data for avatar scale and asset override.
pub fn patch_universes_by_universe_id_configuration_request(
    universe_id: i64,
    body: &UniverseSettingsRequestV2,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!(
            "/v2/universes/{}/configuration",
            crate::rt::path_value(&universe_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v2/universes/{universeId}/configuration`
///
/// Update universe settings for an owned universe. V2 Contains data for avatar scale and asset override.
pub async fn patch_universes_by_universe_id_configuration(
    client: &Client,
    universe_id: i64,
    body: &UniverseSettingsRequestV2,
) -> Result<UniverseSettingsResponseV2> {
    client
        .send(patch_universes_by_universe_id_configuration_request(
            universe_id,
            body,
        )?)
        .await?
        .json()
}

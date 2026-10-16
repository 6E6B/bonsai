//! `games.roblox.com` v1 — generated from `vendor/roblox-api-tracker/games/v1.json`.
//!
//! Do not edit. Change the spec or `bonsai-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "games.roblox.com";

/// `Roblox.Games.Api.CreateVipServersRequest`
///
/// An create request for a vip server
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CreateVipServersRequest {
    /// the expected price of the vip server
    #[serde(rename = "expectedPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_price: Option<i64>,
    /// Optional bool to confirm intention to purchase multiple private servers in quick succession.
    #[serde(rename = "isPurchaseConfirmed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_purchase_confirmed: Option<bool>,
    /// The name new vip server will be
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.Games.Api.PlaceResponse`
///
/// Response model for a VIP server place
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PlaceResponse {
    /// Place id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Place name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.Games.Api.GameResponse`
///
/// Response model for a VIP server game.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GameResponse {
    /// Game id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Game name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "rootPlace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_place: Option<PlaceResponse>,
}

/// `Roblox.Games.Api.GameServerPlayerResponse`
///
/// A response model representing a game server player.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GameServerPlayerResponse {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The thumbnail token for the player.
    #[serde(rename = "playerToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_token: Option<String>,
}

/// `Roblox.Games.Api.Models.Request.GameFavoritesRequest`
///
/// Game favorite request model.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GameFavoritesRequest {
    /// true for favor the game, false for unfavor the game.
    #[serde(rename = "isFavorited")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_favorited: Option<bool>,
}

/// `Roblox.Games.Api.Models.Request.SetUserGameVoteRequest`
///
/// A request for set user's vote for a game
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SetUserGameVoteRequest {
    /// The vote to set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vote: Option<bool>,
}

/// `Roblox.Games.Api.Models.Response.GameCreator`
///
/// Response model for getting the game creator
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GameCreator {
    /// Builder verified badge status.
    #[serde(rename = "hasVerifiedBadge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_verified_badge: Option<bool>,
    /// The game creator id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The game creator account is Luobu Real Name Verified
    #[serde(rename = "isRNVAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rnv_account: Option<bool>,
    /// The game creator name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The game creator type
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

crate::api_enum! {
    int
    /// Avatar type. Possible values are MorphToR6, MorphToR15, and PlayerChoice \['MorphToR6' = 1, 'PlayerChoice' = 2, 'MorphToR15' = 3\]
    GameDetailResponseUniverseAvatarType {
    MorphToR6 = 1 => "MorphToR6",
    PlayerChoice = 2 => "PlayerChoice",
    MorphToR15 = 3 => "MorphToR15",
    }
}

/// `Roblox.Games.Api.Models.Response.GameDetailResponse`
///
/// Response model for getting the game detail
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GameDetailResponse {
    /// The game allowed gear categoris
    #[serde(rename = "allowedGearCategories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_gear_categories: Option<Vec<String>>,
    /// The game allowed gear genres
    #[serde(rename = "allowedGearGenres")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_gear_genres: Option<Vec<String>>,
    /// The game allows place to be copied
    #[serde(rename = "copyingAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copying_allowed: Option<bool>,
    /// Gets or sets the flag to indicate whether the create vip servers button should be allowed in game details page
    #[serde(rename = "createVipServersAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_vip_servers_allowed: Option<bool>,
    /// The game created time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<GameCreator>,
    /// The game description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Game number of favorites.
    #[serde(rename = "favoritedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favorited_count: Option<i64>,
    /// The game genre display name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,
    /// The game genre from experience-genres-service
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre_l1: Option<String>,
    /// The game subgenre from experience-genres-service
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre_l2: Option<String>,
    /// The game universe id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Is this game all genre.
    #[serde(rename = "isAllGenre")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_all_genre: Option<bool>,
    /// Is this game favorited by user.
    #[serde(rename = "isFavoritedByUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_favorited_by_user: Option<bool>,
    /// The game allows place to be copied
    #[serde(rename = "isGenreEnforced")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_genre_enforced: Option<bool>,
    #[serde(rename = "licenseDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_description: Option<String>,
    /// The game max players
    #[serde(rename = "maxPlayers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_players: Option<i32>,
    /// The game name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Current player count of the game
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playing: Option<i64>,
    /// The game paid access price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
    /// The game root place id
    #[serde(rename = "rootPlaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_place_id: Option<i64>,
    /// The game description in the source language, if different from the returned description.
    #[serde(rename = "sourceDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_description: Option<String>,
    /// The game name in the source language, if different from the returned name.
    #[serde(rename = "sourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    /// The setting of IsStudioAccessToApisAllowed of the universe
    #[serde(rename = "studioAccessToApisAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub studio_access_to_apis_allowed: Option<bool>,
    /// Avatar type. Possible values are MorphToR6, MorphToR15, and PlayerChoice \['MorphToR6' = 1, 'PlayerChoice' = 2, 'MorphToR15' = 3\]
    #[serde(rename = "universeAvatarType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_avatar_type: Option<GameDetailResponseUniverseAvatarType>,
    /// The game updated time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    /// The total visits to the game
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visits: Option<i64>,
}

/// `Roblox.Games.Api.Models.Response.GameFavoriteResponse`
///
/// Game favorite response model.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GameFavoriteResponse {
    /// Is it a favorite game.
    #[serde(rename = "isFavorited")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_favorited: Option<bool>,
}

/// `Roblox.Games.Api.Models.Response.GameFavoritesCountResponse`
///
/// Response model for favorited game count.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GameFavoritesCountResponse {
    /// Favorites count.
    #[serde(rename = "favoritesCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favorites_count: Option<i64>,
}

/// `Roblox.Games.Api.Models.Response.GameMediaItem`
///
/// Response model for getting the game media item
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GameMediaItem {
    /// The media item's alt text
    #[serde(rename = "altText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_text: Option<String>,
    /// The media item is approved or not
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved: Option<bool>,
    /// The media item type, Image or YouTubeVideo
    #[serde(rename = "assetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<String>,
    /// The media item type id
    #[serde(rename = "assetTypeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type_id: Option<i64>,
    /// The media item id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The media item image id
    #[serde(rename = "imageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<i64>,
    /// The media item video hash
    #[serde(rename = "videoHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_hash: Option<String>,
    /// The video title for video items.
    #[serde(rename = "videoTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_title: Option<String>,
}

/// `Roblox.Games.Api.Models.Response.GamePassResponse`
///
/// Response model for getting game passes
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GamePassResponse {
    /// The game pass display name
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The game pass id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Is an owned game pass
    #[serde(rename = "isOwned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_owned: Option<bool>,
    /// The game pass name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The game pass price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
    /// The game pass product id
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i64>,
    /// The seller's ID
    #[serde(rename = "sellerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_id: Option<i64>,
    /// The seller's display name
    #[serde(rename = "sellerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_name: Option<String>,
}

/// `Roblox.Games.Api.Models.Response.GameProductResponse`
///
/// Response model for getting the game product information
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GameProductResponse {
    /// The game purchasability
    #[serde(rename = "isForSale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_for_sale: Option<bool>,
    /// The game price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
    /// The game product id
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i64>,
    /// The game seller id User ID for users, Agent ID for groups
    #[serde(rename = "sellerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_id: Option<i64>,
    /// The game universe id
    #[serde(rename = "universeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_id: Option<i64>,
}

/// `Roblox.Games.Api.Models.Response.GameResponseModel`
///
/// Response model for games.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GameResponseModel {
    /// Age Recommendation display name.
    #[serde(rename = "ageRecommendationDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_recommendation_display_name: Option<String>,
    /// Provide all necessary information which helps analytics for improvement, for example, the algorithm, dataset version, position, etc..
    #[serde(rename = "analyticsIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_identifier: Option<String>,
    /// Creator verified badge status
    #[serde(rename = "creatorHasVerifiedBadge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_has_verified_badge: Option<bool>,
    /// Creator Id
    #[serde(rename = "creatorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<i64>,
    /// Creator name
    #[serde(rename = "creatorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_name: Option<String>,
    /// Creator type
    #[serde(rename = "creatorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_type: Option<String>,
    /// Provide all necessary information which helps analytics for improvement, for example, the algorithm, dataset version, position, etc..
    #[serde(rename = "gameDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_description: Option<String>,
    /// The game genre display name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,
    /// Place image token
    #[serde(rename = "imageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_token: Option<String>,
    /// Show the sponsored label
    #[serde(rename = "isShowSponsoredLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_show_sponsored_label: Option<bool>,
    /// Is sponsored
    #[serde(rename = "isSponsored")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_sponsored: Option<bool>,
    /// Age Recommendation minimum age.
    #[serde(rename = "minimumAge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_age: Option<i32>,
    /// Game name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Native ad data
    #[serde(rename = "nativeAdData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_ad_data: Option<String>,
    /// Place Id
    #[serde(rename = "placeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_id: Option<i64>,
    /// Player count
    #[serde(rename = "playerCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_count: Option<i32>,
    /// The game paid access price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
    /// Total down votes
    #[serde(rename = "totalDownVotes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_down_votes: Option<i64>,
    /// Total up votes
    #[serde(rename = "totalUpVotes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_up_votes: Option<i64>,
    /// Universe id
    #[serde(rename = "universeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_id: Option<i64>,
}

/// `Roblox.Games.Api.Models.Response.GameRecommendationsResponse`
///
/// Game recommendations response model.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GameRecommendationsResponse {
    /// The recommended games.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub games: Option<Vec<GameResponseModel>>,
    /// The pagination key for next page query. It will be null if there is no more data.
    #[serde(rename = "nextPaginationKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_pagination_key: Option<String>,
}

/// `Roblox.Games.Api.Models.Response.SpotlightTypeData`
///
/// Parent class to derive custom data for each spotlight type.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SpotlightTypeData {}

/// `Roblox.Games.Api.Models.Response.GameSpotlightResponse`
///
/// Response model for a spotlight game.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GameSpotlightResponse {
    #[serde(rename = "gameInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_info: Option<GameResponseModel>,
    /// Localized text to display on the client for this spotlight game (call to action).
    #[serde(rename = "spotlightActionText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spotlight_action_text: Option<String>,
    /// The spotlight game type; reason the game has been spotlighted.
    #[serde(rename = "spotlightType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spotlight_type: Option<String>,
    #[serde(rename = "spotlightTypeData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spotlight_type_data: Option<SpotlightTypeData>,
}

/// `Roblox.Games.Api.Models.Response.GameVoteResponse`
///
/// Response model for getting the game vote
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GameVoteResponse {
    /// The game down votes
    #[serde(rename = "downVotes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub down_votes: Option<i64>,
    /// The game universe id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The game up votes
    #[serde(rename = "upVotes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub up_votes: Option<i64>,
}

/// `Roblox.Games.Api.Models.Response.MyPrivateServersData`
///
/// Data model for private server
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MyPrivateServersData {
    /// The name of the private server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The id of the owner of the private server
    #[serde(rename = "ownerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<i64>,
    /// The name of the owner of the private server
    #[serde(rename = "ownerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_name: Option<String>,
    /// The place id of the private server.
    #[serde(rename = "placeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_id: Option<i64>,
    /// The price of the private server product in robux
    #[serde(rename = "priceInRobux")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_in_robux: Option<i64>,
    /// The universe id of the private server.
    #[serde(rename = "universeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_id: Option<i64>,
}

/// `Roblox.Games.Api.Models.Response.MyPrivateServersResponse`
///
/// Response model for getting private servers
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MyPrivateServersResponse {
    /// The data of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<MyPrivateServersData>>,
    /// The cursor to use for the next page of data.
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    /// The cursor to use for the previous page of data.
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
}

/// `Roblox.Games.Api.Models.Response.PlaceDetails`
///
/// Response model for a place.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PlaceDetails {
    /// Creator name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builder: Option<String>,
    /// Creator Id
    #[serde(rename = "builderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builder_id: Option<i64>,
    /// Place description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Builder verified badge status.
    #[serde(rename = "hasVerifiedBadge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_verified_badge: Option<bool>,
    /// Place image token
    #[serde(rename = "imageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_token: Option<String>,
    /// Is playable
    #[serde(rename = "isPlayable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_playable: Option<bool>,
    /// Place name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Place Id
    #[serde(rename = "placeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_id: Option<i64>,
    /// Price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
    /// Reason prohibited
    #[serde(rename = "reasonProhibited")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_prohibited: Option<String>,
    /// Place description in source language
    #[serde(rename = "sourceDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_description: Option<String>,
    /// Place name in source language
    #[serde(rename = "sourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    /// Universe id
    #[serde(rename = "universeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_id: Option<i64>,
    /// UniverseRootPlaceId
    #[serde(rename = "universeRootPlaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_root_place_id: Option<i64>,
    /// Url
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

crate::api_enum! {
    int
    /// The actual playability status of the universe including the reason if unplayable \['UnplayableOtherReason' = 0, 'Playable' = 1, 'GuestProhibited' = 2, 'GameUnapproved' = 3, 'IncorrectConfiguration' = 4, 'UniverseRootPlaceIsPrivate' = 5, 'InsufficientPermissionFriendsOnly' = 6, 'InsufficientPermissionGroupOnly' = 7, 'DeviceRestricted' = 8, 'UnderReview' = 9, 'PurchaseRequired' = 10, 'AccountRestricted' = 11, 'TemporarilyUnavailable' = 12, 'PlaceHasNoPublishedVersion' = 13, 'ComplianceBlocked' = 14, 'ContextualPlayabilityRegionalAvailability' = 15, 'ContextualPlayabilityRegionalCompliance' = 16, 'ContextualPlayabilityAgeRecommendationParentalControls' = 17, 'ContextualPlayabilityAgeGated' = 18, 'ContextualPlayabilityUnverifiedSeventeenPlusUser' = 19\]
    PlayabilityStatusResponsePlayabilityStatus {
    UnplayableOtherReason = 0 => "UnplayableOtherReason",
    Playable = 1 => "Playable",
    GuestProhibited = 2 => "GuestProhibited",
    GameUnapproved = 3 => "GameUnapproved",
    IncorrectConfiguration = 4 => "IncorrectConfiguration",
    UniverseRootPlaceIsPrivate = 5 => "UniverseRootPlaceIsPrivate",
    InsufficientPermissionFriendsOnly = 6 => "InsufficientPermissionFriendsOnly",
    InsufficientPermissionGroupOnly = 7 => "InsufficientPermissionGroupOnly",
    DeviceRestricted = 8 => "DeviceRestricted",
    UnderReview = 9 => "UnderReview",
    PurchaseRequired = 10 => "PurchaseRequired",
    AccountRestricted = 11 => "AccountRestricted",
    TemporarilyUnavailable = 12 => "TemporarilyUnavailable",
    PlaceHasNoPublishedVersion = 13 => "PlaceHasNoPublishedVersion",
    ComplianceBlocked = 14 => "ComplianceBlocked",
    ContextualPlayabilityRegionalAvailability = 15 => "ContextualPlayabilityRegionalAvailability",
    ContextualPlayabilityRegionalCompliance = 16 => "ContextualPlayabilityRegionalCompliance",
    ContextualPlayabilityAgeRecommendationParentalControls = 17 => "ContextualPlayabilityAgeRecommendationParentalControls",
    ContextualPlayabilityAgeGated = 18 => "ContextualPlayabilityAgeGated",
    ContextualPlayabilityUnverifiedSeventeenPlusUser = 19 => "ContextualPlayabilityUnverifiedSeventeenPlusUser",
    }
}

/// `Roblox.Games.Api.Models.Response.PlayabilityStatusResponse`
///
/// Response model for getting a universe's playability status for a user
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PlayabilityStatusResponse {
    /// Whether or not the universe is playable for the user
    #[serde(rename = "isPlayable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_playable: Option<bool>,
    /// The actual playability status of the universe including the reason if unplayable \['UnplayableOtherReason' = 0, 'Playable' = 1, 'GuestProhibited' = 2, 'GameUnapproved' = 3, 'IncorrectConfiguration' = 4, 'UniverseRootPlaceIsPrivate' = 5, 'InsufficientPermissionFriendsOnly' = 6, 'InsufficientPermissionGroupOnly' = 7, 'DeviceRestricted' = 8, 'UnderReview' = 9, 'PurchaseRequired' = 10, 'AccountRestricted' = 11, 'TemporarilyUnavailable' = 12, 'PlaceHasNoPublishedVersion' = 13, 'ComplianceBlocked' = 14, 'ContextualPlayabilityRegionalAvailability' = 15, 'ContextualPlayabilityRegionalCompliance' = 16, 'ContextualPlayabilityAgeRecommendationParentalControls' = 17, 'ContextualPlayabilityAgeGated' = 18, 'ContextualPlayabilityUnverifiedSeventeenPlusUser' = 19\]
    #[serde(rename = "playabilityStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playability_status: Option<PlayabilityStatusResponsePlayabilityStatus>,
    /// The universeId of the requested universe to help with batching requests
    #[serde(rename = "universeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_id: Option<i64>,
}

/// `Roblox.Games.Api.Models.Response.PrivateServersEnabledInUniverseResponse`
///
/// Response model for getting whether a universe has private servers enabled.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PrivateServersEnabledInUniverseResponse {
    /// Whether private servers are enabled in the universe.
    #[serde(rename = "privateServersEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_servers_enabled: Option<bool>,
}

/// `Roblox.Games.Api.Models.Response.Thumbnail`
///
/// Thumbnail response model.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Thumbnail {
    /// CdnUrl
    #[serde(rename = "cdnUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdn_url: Option<String>,
    /// Final
    #[serde(rename = "final")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_: Option<bool>,
    /// PlaceId
    #[serde(rename = "placeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_id: Option<i64>,
    /// RetryToken
    #[serde(rename = "retryToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_token: Option<String>,
    /// UniverseId
    #[serde(rename = "universeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_id: Option<i64>,
    /// Url
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// `Roblox.Games.Api.Models.Response.UserGameVoteResponse`
///
/// Response model for getting the user vote status for a game
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UserGameVoteResponse {
    /// Whether the user can vote the game
    #[serde(rename = "canVote")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_vote: Option<bool>,
    /// The reason why the user can't vote
    #[serde(rename = "reasonForNotVoteable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_for_not_voteable: Option<String>,
    /// The user's vote status for the game
    #[serde(rename = "userVote")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_vote: Option<bool>,
}

/// `Roblox.Games.Api.Models.Response.VerifiedBadgeUserResponse`
///
/// A response model specific to multi-get user.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct VerifiedBadgeUserResponse {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The user's verified badge status.
    #[serde(rename = "hasVerifiedBadge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_verified_badge: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.Web.Responses.Games.GameServerResponse`
///
/// Game server list response model.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GameServerResponse {
    /// The private server access code.
    #[serde(rename = "accessCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_code: Option<String>,
    /// The current frames per second the server is running at.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fps: Option<f64>,
    /// The game server ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The max number of players allowed to enter the server at the same time.
    #[serde(rename = "maxPlayers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_players: Option<i32>,
    /// The name of the private server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<VerifiedBadgeUserResponse>,
    /// The game server ping time (measured in milliseconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ping: Option<i32>,
    /// The thumbnail tokens for all the players in the server.
    #[serde(rename = "playerTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_tokens: Option<Vec<String>>,
    /// The players in the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub players: Option<Vec<GameServerPlayerResponse>>,
    /// The number of players actively in the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playing: Option<i32>,
    /// The private server ID.
    #[serde(rename = "vipServerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vip_server_id: Option<i64>,
}

/// `Roblox.Games.Api.PrivateServersResponse`
///
/// User Presences Response model
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PrivateServersResponse {
    /// A collection of game server responses
    #[serde(rename = "privateServerResponses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_server_responses: Option<Vec<GameServerResponse>>,
}

crate::api_enum! {
    int
    /// Invite response type \['Invalid' = 0, 'CanInvite' = 1, 'UnableToAddAnyUser' = 2, 'UnableToAddNonFriends' = 3, 'UnableToAddSpecificUser' = 4\]
    VipServerCanInviteResponseInviteResponseType {
    Invalid = 0 => "Invalid",
    CanInvite = 1 => "CanInvite",
    UnableToAddAnyUser = 2 => "UnableToAddAnyUser",
    UnableToAddNonFriends = 3 => "UnableToAddNonFriends",
    UnableToAddSpecificUser = 4 => "UnableToAddSpecificUser",
    }
}

/// `Roblox.Games.Api.VipServerCanInviteResponse`
///
/// Response model for VIP server can-invite user.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct VipServerCanInviteResponse {
    /// Sufficient permissions to invite user
    #[serde(rename = "canInvite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite: Option<bool>,
    /// Does owner privacy restrict joins
    #[serde(rename = "doesOwnerPrivacyRestrictJoins")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub does_owner_privacy_restrict_joins: Option<bool>,
    /// Invite response type \['Invalid' = 0, 'CanInvite' = 1, 'UnableToAddAnyUser' = 2, 'UnableToAddNonFriends' = 3, 'UnableToAddSpecificUser' = 4\]
    #[serde(rename = "inviteResponseType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_response_type: Option<VipServerCanInviteResponseInviteResponseType>,
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

/// `Roblox.Games.Api.VipServerPermissionsResponse`
///
/// Response model for VIP server permissions.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct VipServerPermissionsResponse {
    /// Clan access
    #[serde(rename = "clanAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clan_allowed: Option<bool>,
    /// EnemyClanId
    #[serde(rename = "enemyClanId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enemy_clan_id: Option<i64>,
    /// Friend access
    #[serde(rename = "friendsAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friends_allowed: Option<bool>,
    /// Users
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<SkinnyUserResponse>>,
}

/// `Roblox.Games.Api.VipServerSubscriptionResponse`
///
/// Response model for VIP server subscription.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct VipServerSubscriptionResponse {
    /// Active
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Can renew a subscription
    #[serde(rename = "canRenew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_renew: Option<bool>,
    /// Expiration date
    #[serde(rename = "expirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    /// Expired
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired: Option<bool>,
    /// Subscription cancelled due to insufficient funds
    #[serde(rename = "hasInsufficientFunds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_insufficient_funds: Option<bool>,
    /// Subscriptions price has changed
    #[serde(rename = "hasPriceChanged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_price_changed: Option<bool>,
    /// Subscription has a recurring profile
    #[serde(rename = "hasRecurringProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_recurring_profile: Option<bool>,
    /// Price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
}

/// `Roblox.Games.Api.VipServerVoiceSettingsResponse`
///
/// Contains voice settings of a vip server
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct VipServerVoiceSettingsResponse {
    /// Whether the voice chat is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// `Roblox.Games.Api.VipServerResponse`
///
/// Response model for a VIP server.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct VipServerResponse {
    /// Server status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game: Option<GameResponse>,
    /// Server Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Server join code
    #[serde(rename = "joinCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_code: Option<String>,
    /// Private server link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    /// Server name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<VipServerPermissionsResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<VipServerSubscriptionResponse>,
    #[serde(rename = "voiceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_settings: Option<VipServerVoiceSettingsResponse>,
}

/// `Roblox.Games.Api.VipServerUpdatePermissionsRequest`
///
/// Request model for updating permissions for a VIP server.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct VipServerUpdatePermissionsRequest {
    /// Clan access
    #[serde(rename = "clanAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clan_allowed: Option<bool>,
    /// EnemyClanId
    #[serde(rename = "enemyClanId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enemy_clan_id: Option<i32>,
    /// Friend access
    #[serde(rename = "friendsAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friends_allowed: Option<bool>,
    /// Users to add
    #[serde(rename = "usersToAdd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users_to_add: Option<Vec<i64>>,
    /// Users to remove
    #[serde(rename = "usersToRemove")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users_to_remove: Option<Vec<i64>>,
}

/// `Roblox.Games.Api.VipServerUpdateRequest`
///
/// Request model to update a VIP server.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct VipServerUpdateRequest {
    /// Server status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Server name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Server join code
    #[serde(rename = "newJoinCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_join_code: Option<bool>,
}

/// `Roblox.Games.Api.VipServerUpdateSubscriptionRequest`
///
/// Request model to update subscriptions for a VIP server.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct VipServerUpdateSubscriptionRequest {
    /// Active
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
}

/// `Roblox.Games.Api.VipServerUpdateVoiceSettingsRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct VipServerUpdateVoiceSettingsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

crate::api_enum! {
    int
    /// \['Forward' = 1, 'Backward' = 2\]
    ExclusiveStartKeyCursorStringPagingDirection {
    Forward = 1 => "Forward",
    Backward = 2 => "Backward",
    }
}

crate::api_enum! {
    int
    /// \['Asc' = 1, 'Desc' = 2\]
    ExclusiveStartKeyCursorStringSortOrder {
    Asc = 1 => "Asc",
    Desc = 2 => "Desc",
    }
}

/// `Roblox.Paging.ExclusiveStartKeyCursor\[System.String\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ExclusiveStartKeyCursorString {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discriminator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "pageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// \['Forward' = 1, 'Backward' = 2\]
    #[serde(rename = "pagingDirection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paging_direction: Option<ExclusiveStartKeyCursorStringPagingDirection>,
    /// \['Asc' = 1, 'Desc' = 2\]
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<ExclusiveStartKeyCursorStringSortOrder>,
}

/// `Roblox.Paging.StartIndexCursor`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct StartIndexCursor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discriminator: Option<String>,
    #[serde(rename = "startIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i64>,
}

/// `Roblox.Web.WebAPI.ApiEmptyResponseModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiEmptyResponseModel {}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.Games.Api.Models.Response.GameDetailResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseGameDetailResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GameDetailResponse>>,
}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.Games.Api.Models.Response.GameMediaItem\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseGameMediaItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GameMediaItem>>,
}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.Games.Api.Models.Response.GameProductResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseGameProductResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GameProductResponse>>,
}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.Games.Api.Models.Response.GameSpotlightResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseGameSpotlightResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GameSpotlightResponse>>,
}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.Games.Api.Models.Response.GameVoteResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseGameVoteResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GameVoteResponse>>,
}

/// `Roblox.Web.WebAPI.Models.ApiPageResponse\[Roblox.Games.Api.Models.Response.GamePassResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiPageResponseGamePassResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GamePassResponse>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
}

/// `Roblox.Web.WebAPI.Models.ApiPageResponse\[Roblox.Web.Responses.Games.GameServerResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiPageResponseGameServerResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GameServerResponse>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
}

/// Query, header and form inputs for [`get_games`].
#[derive(Debug, Clone, Default)]
pub struct GetGamesParams {
    /// A list of universe Ids. Cannot exceed a maximum of 100 IDs.
    pub universe_ids: Vec<i64>,
}

/// Query, header and form inputs for [`get_games_game_thumbnail`].
#[derive(Debug, Clone, Default)]
pub struct GetGamesGameThumbnailParams {
    pub image_token: String,
    pub height: Option<i32>,
    pub width: Option<i32>,
}

/// Query, header and form inputs for [`get_games_game_thumbnails`].
#[derive(Debug, Clone, Default)]
pub struct GetGamesGameThumbnailsParams {
    pub image_tokens: Vec<String>,
    pub height: Option<i32>,
    pub width: Option<i32>,
}

/// Query, header and form inputs for [`get_games_games_product_info`].
#[derive(Debug, Clone, Default)]
pub struct GetGamesGamesProductInfoParams {
    /// A list of universe Ids. Cannot exceed a maximum of 100 IDs.
    pub universe_ids: Vec<i64>,
}

/// Query, header and form inputs for [`get_games_multiget_place_details`].
#[derive(Debug, Clone, Default)]
pub struct GetGamesMultigetPlaceDetailsParams {
    /// List of placeId to uniquely Identify a place
    pub place_ids: Vec<i64>,
}

/// Query, header and form inputs for [`get_games_multiget_playability_status`].
#[derive(Debug, Clone, Default)]
pub struct GetGamesMultigetPlayabilityStatusParams {
    /// A list of universe Ids. Cannot exceed a maximum of 100 IDs.
    pub universe_ids: Vec<i64>,
}

/// Query, header and form inputs for [`get_games_recommendations_algorithm_by_algorithm_name`].
#[derive(Debug, Clone, Default)]
pub struct GetGamesRecommendationsAlgorithmByAlgorithmNameParams {
    /// The key of a page, which includes the start row index and all other necessary information to query the data. This parameter is usually not needed for the first page.
    pub pagination_key: String,
    /// The requested number of rows.
    pub max_rows: i32,
    /// Truncated Results
    pub is_truncated_results_enabled: bool,
}

/// Query, header and form inputs for [`get_games_recommendations_game_by_universe_id`].
#[derive(Debug, Clone, Default)]
pub struct GetGamesRecommendationsGameByUniverseIdParams {
    /// The key of a page, which includes the start row index and all other necessary information to query the data. This parameter is usually not needed for the first page.
    pub pagination_key: String,
    /// The requested number of rows.
    pub max_rows: i32,
    /// Truncated Results
    pub is_truncated_results_enabled: bool,
}

/// Query, header and form inputs for [`get_games_votes`].
#[derive(Debug, Clone, Default)]
pub struct GetGamesVotesParams {
    /// A list of universe Ids. Cannot exceed a maximum of 100 IDs.
    pub universe_ids: Vec<i64>,
}

crate::api_enum! {
    str
    /// The order the results are sorted in.
    GetGamesByPlaceIdPrivateServersSortOrder {
    Asc = 0 => "Asc",
    Desc = 1 => "Desc",
    }
}

/// Query, header and form inputs for [`get_games_by_place_id_private_servers`].
#[derive(Debug, Clone, Default)]
pub struct GetGamesByPlaceIdPrivateServersParams {
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// The order the results are sorted in.
    pub sort_order: Option<GetGamesByPlaceIdPrivateServersSortOrder>,
}

/// Query, header and form inputs for [`get_games_by_place_id_servers_by_server_type`].
#[derive(Debug, Clone, Default)]
pub struct GetGamesByPlaceIdServersByServerTypeParams {
    /// The sort order of the servers.
    pub sort_order: Option<i32>,
    /// Exclude full servers.
    pub exclude_full_games: Option<bool>,
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
}

/// Query, header and form inputs for [`get_games_by_universe_id_game_passes`].
#[derive(Debug, Clone, Default)]
pub struct GetGamesByUniverseIdGamePassesParams {
    pub limit: i32,
    pub sort_order: Option<i32>,
    /// The cursor to figure out where to start fetching
    pub cursor: Option<String>,
}

/// Query, header and form inputs for [`get_private_servers`].
#[derive(Debug, Clone, Default)]
pub struct GetPrivateServersParams {
    pub private_server_ids: Vec<i64>,
}

/// Query, header and form inputs for [`get_private_servers_my_private_servers`].
#[derive(Debug, Clone, Default)]
pub struct GetPrivateServersMyPrivateServersParams {
    /// Type of private server to fetch
    pub private_servers_tab: Option<i32>,
    /// Number of items per page
    pub items_per_page: Option<i32>,
    /// Current cursor
    pub cursor: Option<String>,
}

/// `GET /v1/games`
///
/// Gets a list of games' detail
pub fn get_games_request(params: &GetGamesParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/games".to_string());
    r.query("universeIds", &params.universe_ids);
    Ok(r)
}

/// `GET /v1/games`
///
/// Gets a list of games' detail
pub async fn get_games(
    client: &Client,
    params: &GetGamesParams,
) -> Result<ApiArrayResponseGameDetailResponse> {
    client.send(get_games_request(params)?).await?.json()
}

/// `GET /v1/games/game-thumbnail`
///
/// Get a single game thumbnail
#[deprecated(note = "marked deprecated by Roblox")]
pub fn get_games_game_thumbnail_request(params: &GetGamesGameThumbnailParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/games/game-thumbnail".to_string());
    r.query("imageToken", &params.image_token);
    r.query("height", &params.height);
    r.query("width", &params.width);
    Ok(r)
}

/// `GET /v1/games/game-thumbnail`
///
/// Get a single game thumbnail
#[deprecated(note = "marked deprecated by Roblox")]
pub async fn get_games_game_thumbnail(client: &Client, params: &GetGamesGameThumbnailParams) -> Result<()> {
    client
        .send(get_games_game_thumbnail_request(params)?)
        .await?
        .unit()
}

/// `GET /v1/games/game-thumbnails`
///
/// Gets a list of game thumbnails
#[deprecated(note = "marked deprecated by Roblox")]
pub fn get_games_game_thumbnails_request(params: &GetGamesGameThumbnailsParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/games/game-thumbnails".to_string());
    r.query("imageTokens", &params.image_tokens);
    r.query("height", &params.height);
    r.query("width", &params.width);
    Ok(r)
}

/// `GET /v1/games/game-thumbnails`
///
/// Gets a list of game thumbnails
#[deprecated(note = "marked deprecated by Roblox")]
pub async fn get_games_game_thumbnails(
    client: &Client,
    params: &GetGamesGameThumbnailsParams,
) -> Result<Vec<Thumbnail>> {
    client
        .send(get_games_game_thumbnails_request(params)?)
        .await?
        .json()
}

/// `GET /v1/games/games-product-info`
///
/// Gets a list of games' product info, used to purchase a game
pub fn get_games_games_product_info_request(params: &GetGamesGamesProductInfoParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/games/games-product-info".to_string());
    r.query("universeIds", &params.universe_ids);
    Ok(r)
}

/// `GET /v1/games/games-product-info`
///
/// Gets a list of games' product info, used to purchase a game
pub async fn get_games_games_product_info(
    client: &Client,
    params: &GetGamesGamesProductInfoParams,
) -> Result<ApiArrayResponseGameProductResponse> {
    client
        .send(get_games_games_product_info_request(params)?)
        .await?
        .json()
}

/// `GET /v1/games/list-spotlight`
///
/// Gets games that the client should spotlight.
pub fn get_games_list_spotlight_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/games/list-spotlight".to_string());
    Ok(r)
}

/// `GET /v1/games/list-spotlight`
///
/// Gets games that the client should spotlight.
pub async fn get_games_list_spotlight(client: &Client) -> Result<ApiArrayResponseGameSpotlightResponse> {
    client.send(get_games_list_spotlight_request()?).await?.json()
}

/// `GET /v1/games/multiget-place-details`
///
/// Get place details
pub fn get_games_multiget_place_details_request(
    params: &GetGamesMultigetPlaceDetailsParams,
) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/games/multiget-place-details".to_string());
    r.query("placeIds", &params.place_ids);
    Ok(r)
}

/// `GET /v1/games/multiget-place-details`
///
/// Get place details
pub async fn get_games_multiget_place_details(
    client: &Client,
    params: &GetGamesMultigetPlaceDetailsParams,
) -> Result<Vec<PlaceDetails>> {
    client
        .send(get_games_multiget_place_details_request(params)?)
        .await?
        .json()
}

/// `GET /v1/games/multiget-playability-status`
///
/// Gets a list of universe playability statuses for the authenticated user
pub fn get_games_multiget_playability_status_request(
    params: &GetGamesMultigetPlayabilityStatusParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        "/v1/games/multiget-playability-status".to_string(),
    );
    r.query("universeIds", &params.universe_ids);
    Ok(r)
}

/// `GET /v1/games/multiget-playability-status`
///
/// Gets a list of universe playability statuses for the authenticated user
pub async fn get_games_multiget_playability_status(
    client: &Client,
    params: &GetGamesMultigetPlayabilityStatusParams,
) -> Result<Vec<PlayabilityStatusResponse>> {
    client
        .send(get_games_multiget_playability_status_request(params)?)
        .await?
        .json()
}

/// `GET /v1/games/recommendations/algorithm/{algorithmName}`
///
/// Get games recommendations
#[deprecated(note = "marked deprecated by Roblox")]
pub fn get_games_recommendations_algorithm_by_algorithm_name_request(
    algorithm_name: &str,
    params: &GetGamesRecommendationsAlgorithmByAlgorithmNameParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/games/recommendations/algorithm/{}",
            crate::rt::path_value(&algorithm_name)
        ),
    );
    r.query("PaginationKey", &params.pagination_key);
    r.query("MaxRows", &params.max_rows);
    r.query("IsTruncatedResultsEnabled", &params.is_truncated_results_enabled);
    Ok(r)
}

/// `GET /v1/games/recommendations/algorithm/{algorithmName}`
///
/// Get games recommendations
#[deprecated(note = "marked deprecated by Roblox")]
pub async fn get_games_recommendations_algorithm_by_algorithm_name(
    client: &Client,
    algorithm_name: &str,
    params: &GetGamesRecommendationsAlgorithmByAlgorithmNameParams,
) -> Result<GameRecommendationsResponse> {
    client
        .send(get_games_recommendations_algorithm_by_algorithm_name_request(
            algorithm_name,
            params,
        )?)
        .await?
        .json()
}

/// `GET /v1/games/recommendations/game/{universeId}`
///
/// Get games recommendations based on a given universe
pub fn get_games_recommendations_game_by_universe_id_request(
    universe_id: i64,
    params: &GetGamesRecommendationsGameByUniverseIdParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/games/recommendations/game/{}",
            crate::rt::path_value(&universe_id)
        ),
    );
    r.query("PaginationKey", &params.pagination_key);
    r.query("MaxRows", &params.max_rows);
    r.query("IsTruncatedResultsEnabled", &params.is_truncated_results_enabled);
    Ok(r)
}

/// `GET /v1/games/recommendations/game/{universeId}`
///
/// Get games recommendations based on a given universe
pub async fn get_games_recommendations_game_by_universe_id(
    client: &Client,
    universe_id: i64,
    params: &GetGamesRecommendationsGameByUniverseIdParams,
) -> Result<GameRecommendationsResponse> {
    client
        .send(get_games_recommendations_game_by_universe_id_request(
            universe_id,
            params,
        )?)
        .await?
        .json()
}

/// `POST /v1/games/vip-servers/{universeId}`
///
/// Create VIP server for a game
pub fn post_games_vip_servers_by_universe_id_request(
    universe_id: i64,
    body: &CreateVipServersRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!("/v1/games/vip-servers/{}", crate::rt::path_value(&universe_id)),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/games/vip-servers/{universeId}`
///
/// Create VIP server for a game
pub async fn post_games_vip_servers_by_universe_id(
    client: &Client,
    universe_id: i64,
    body: &CreateVipServersRequest,
) -> Result<GameServerResponse> {
    client
        .send(post_games_vip_servers_by_universe_id_request(universe_id, body)?)
        .await?
        .json()
}

/// `GET /v1/games/votes`
///
/// Gets a list of universe vote status
pub fn get_games_votes_request(params: &GetGamesVotesParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/games/votes".to_string());
    r.query("universeIds", &params.universe_ids);
    Ok(r)
}

/// `GET /v1/games/votes`
///
/// Gets a list of universe vote status
pub async fn get_games_votes(
    client: &Client,
    params: &GetGamesVotesParams,
) -> Result<ApiArrayResponseGameVoteResponse> {
    client.send(get_games_votes_request(params)?).await?.json()
}

/// `GET /v1/games/{placeId}/private-servers`
///
/// Get list of private servers user can access for given game id.
pub fn get_games_by_place_id_private_servers_request(
    place_id: i64,
    params: &GetGamesByPlaceIdPrivateServersParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/games/{}/private-servers", crate::rt::path_value(&place_id)),
    );
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    r.query("sortOrder", &params.sort_order);
    Ok(r)
}

/// `GET /v1/games/{placeId}/private-servers`
///
/// Get list of private servers user can access for given game id.
pub async fn get_games_by_place_id_private_servers(
    client: &Client,
    place_id: i64,
    params: &GetGamesByPlaceIdPrivateServersParams,
) -> Result<ApiPageResponseGameServerResponse> {
    client
        .send(get_games_by_place_id_private_servers_request(place_id, params)?)
        .await?
        .json()
}

/// `GET /v1/games/{placeId}/servers/{serverType}`
///
/// Get the game server list
pub fn get_games_by_place_id_servers_by_server_type_request(
    place_id: i64,
    server_type: i32,
    params: &GetGamesByPlaceIdServersByServerTypeParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/games/{}/servers/{}",
            crate::rt::path_value(&place_id),
            crate::rt::path_value(&server_type)
        ),
    );
    r.query("sortOrder", &params.sort_order);
    r.query("excludeFullGames", &params.exclude_full_games);
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    Ok(r)
}

/// `GET /v1/games/{placeId}/servers/{serverType}`
///
/// Get the game server list
pub async fn get_games_by_place_id_servers_by_server_type(
    client: &Client,
    place_id: i64,
    server_type: i32,
    params: &GetGamesByPlaceIdServersByServerTypeParams,
) -> Result<ApiPageResponseGameServerResponse> {
    client
        .send(get_games_by_place_id_servers_by_server_type_request(
            place_id,
            server_type,
            params,
        )?)
        .await?
        .json()
}

/// `GET /v1/games/{universeId}/favorites`
///
/// Returns if a game was marked as favorite for the authenticated user
pub fn get_games_by_universe_id_favorites_request(universe_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/games/{}/favorites", crate::rt::path_value(&universe_id)),
    );
    Ok(r)
}

/// `GET /v1/games/{universeId}/favorites`
///
/// Returns if a game was marked as favorite for the authenticated user
pub async fn get_games_by_universe_id_favorites(
    client: &Client,
    universe_id: i64,
) -> Result<GameFavoriteResponse> {
    client
        .send(get_games_by_universe_id_favorites_request(universe_id)?)
        .await?
        .json()
}

/// `POST /v1/games/{universeId}/favorites`
///
/// Favors (or unfavors) a game for the authenticated user
pub fn post_games_by_universe_id_favorites_request(
    universe_id: i64,
    body: &GameFavoritesRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!("/v1/games/{}/favorites", crate::rt::path_value(&universe_id)),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/games/{universeId}/favorites`
///
/// Favors (or unfavors) a game for the authenticated user
pub async fn post_games_by_universe_id_favorites(
    client: &Client,
    universe_id: i64,
    body: &GameFavoritesRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_games_by_universe_id_favorites_request(universe_id, body)?)
        .await?
        .json()
}

/// `GET /v1/games/{universeId}/favorites/count`
///
/// Get the favorites count of the a specific game
pub fn get_games_by_universe_id_favorites_count_request(universe_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/games/{}/favorites/count",
            crate::rt::path_value(&universe_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/games/{universeId}/favorites/count`
///
/// Get the favorites count of the a specific game
pub async fn get_games_by_universe_id_favorites_count(
    client: &Client,
    universe_id: i64,
) -> Result<GameFavoritesCountResponse> {
    client
        .send(get_games_by_universe_id_favorites_count_request(universe_id)?)
        .await?
        .json()
}

/// `GET /v1/games/{universeId}/game-passes`
///
/// Get the game's game passes
pub fn get_games_by_universe_id_game_passes_request(
    universe_id: i64,
    params: &GetGamesByUniverseIdGamePassesParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/games/{}/game-passes", crate::rt::path_value(&universe_id)),
    );
    r.query("limit", &params.limit);
    r.query("sortOrder", &params.sort_order);
    r.query("cursor", &params.cursor);
    Ok(r)
}

/// `GET /v1/games/{universeId}/game-passes`
///
/// Get the game's game passes
pub async fn get_games_by_universe_id_game_passes(
    client: &Client,
    universe_id: i64,
    params: &GetGamesByUniverseIdGamePassesParams,
) -> Result<ApiPageResponseGamePassResponse> {
    client
        .send(get_games_by_universe_id_game_passes_request(universe_id, params)?)
        .await?
        .json()
}

/// `GET /v1/games/{universeId}/media`
///
/// Get the game media data
#[deprecated(note = "marked deprecated by Roblox")]
pub fn get_games_by_universe_id_media_request(universe_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/games/{}/media", crate::rt::path_value(&universe_id)),
    );
    Ok(r)
}

/// `GET /v1/games/{universeId}/media`
///
/// Get the game media data
#[deprecated(note = "marked deprecated by Roblox")]
pub async fn get_games_by_universe_id_media(
    client: &Client,
    universe_id: i64,
) -> Result<ApiArrayResponseGameMediaItem> {
    client
        .send(get_games_by_universe_id_media_request(universe_id)?)
        .await?
        .json()
}

/// `PATCH /v1/games/{universeId}/user-votes`
///
/// Set the user's vote for a game
pub fn patch_games_by_universe_id_user_votes_request(
    universe_id: i64,
    body: &SetUserGameVoteRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!("/v1/games/{}/user-votes", crate::rt::path_value(&universe_id)),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/games/{universeId}/user-votes`
///
/// Set the user's vote for a game
pub async fn patch_games_by_universe_id_user_votes(
    client: &Client,
    universe_id: i64,
    body: &SetUserGameVoteRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(patch_games_by_universe_id_user_votes_request(universe_id, body)?)
        .await?
        .json()
}

/// `GET /v1/games/{universeId}/votes`
///
/// Get the game vote status
#[deprecated(note = "marked deprecated by Roblox")]
pub fn get_games_by_universe_id_votes_request(universe_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/games/{}/votes", crate::rt::path_value(&universe_id)),
    );
    Ok(r)
}

/// `GET /v1/games/{universeId}/votes`
///
/// Get the game vote status
#[deprecated(note = "marked deprecated by Roblox")]
pub async fn get_games_by_universe_id_votes(client: &Client, universe_id: i64) -> Result<GameVoteResponse> {
    client
        .send(get_games_by_universe_id_votes_request(universe_id)?)
        .await?
        .json()
}

/// `GET /v1/games/{universeId}/votes/user`
///
/// Get the user's vote status for a game
pub fn get_games_by_universe_id_votes_user_request(universe_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/games/{}/votes/user", crate::rt::path_value(&universe_id)),
    );
    Ok(r)
}

/// `GET /v1/games/{universeId}/votes/user`
///
/// Get the user's vote status for a game
pub async fn get_games_by_universe_id_votes_user(
    client: &Client,
    universe_id: i64,
) -> Result<UserGameVoteResponse> {
    client
        .send(get_games_by_universe_id_votes_user_request(universe_id)?)
        .await?
        .json()
}

/// `GET /v1/private-servers`
///
/// Get private servers from private server ids
pub fn get_private_servers_request(params: &GetPrivateServersParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/private-servers".to_string());
    r.query("privateServerIds", &params.private_server_ids);
    Ok(r)
}

/// `GET /v1/private-servers`
///
/// Get private servers from private server ids
pub async fn get_private_servers(
    client: &Client,
    params: &GetPrivateServersParams,
) -> Result<PrivateServersResponse> {
    client.send(get_private_servers_request(params)?).await?.json()
}

/// `GET /v1/private-servers/enabled-in-universe/{universeId}`
///
/// Checks if the private servers are enabled in the specified universe.
pub fn get_private_servers_enabled_in_universe_by_universe_id_request(universe_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/private-servers/enabled-in-universe/{}",
            crate::rt::path_value(&universe_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/private-servers/enabled-in-universe/{universeId}`
///
/// Checks if the private servers are enabled in the specified universe.
pub async fn get_private_servers_enabled_in_universe_by_universe_id(
    client: &Client,
    universe_id: i64,
) -> Result<PrivateServersEnabledInUniverseResponse> {
    client
        .send(get_private_servers_enabled_in_universe_by_universe_id_request(
            universe_id,
        )?)
        .await?
        .json()
}

/// `GET /v1/private-servers/my-private-servers`
///
/// Get private servers for the authenticated user
pub fn get_private_servers_my_private_servers_request(
    params: &GetPrivateServersMyPrivateServersParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        "/v1/private-servers/my-private-servers".to_string(),
    );
    r.query("privateServersTab", &params.private_servers_tab);
    r.query("itemsPerPage", &params.items_per_page);
    r.query("cursor", &params.cursor);
    Ok(r)
}

/// `GET /v1/private-servers/my-private-servers`
///
/// Get private servers for the authenticated user
pub async fn get_private_servers_my_private_servers(
    client: &Client,
    params: &GetPrivateServersMyPrivateServersParams,
) -> Result<MyPrivateServersResponse> {
    client
        .send(get_private_servers_my_private_servers_request(params)?)
        .await?
        .json()
}

/// `GET /v1/vip-server/can-invite/{userId}`
///
/// Determines if a user by id is allowed to receive a VIP Server invite from the authenticated user.
pub fn get_vip_server_can_invite_by_user_id_request(user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/vip-server/can-invite/{}", crate::rt::path_value(&user_id)),
    );
    Ok(r)
}

/// `GET /v1/vip-server/can-invite/{userId}`
///
/// Determines if a user by id is allowed to receive a VIP Server invite from the authenticated user.
pub async fn get_vip_server_can_invite_by_user_id(
    client: &Client,
    user_id: i64,
) -> Result<VipServerCanInviteResponse> {
    client
        .send(get_vip_server_can_invite_by_user_id_request(user_id)?)
        .await?
        .json()
}

/// `GET /v1/vip-servers/{id}`
///
/// Get necessary data to generate webpage
pub fn get_vip_servers_by_id_request(id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/vip-servers/{}", crate::rt::path_value(&id)),
    );
    Ok(r)
}

/// `GET /v1/vip-servers/{id}`
///
/// Get necessary data to generate webpage
pub async fn get_vip_servers_by_id(client: &Client, id: i64) -> Result<VipServerResponse> {
    client.send(get_vip_servers_by_id_request(id)?).await?.json()
}

/// `PATCH /v1/vip-servers/{id}`
///
/// Updates vip server
pub fn patch_vip_servers_by_id_request(id: i64, body: &VipServerUpdateRequest) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!("/v1/vip-servers/{}", crate::rt::path_value(&id)),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/vip-servers/{id}`
///
/// Updates vip server
pub async fn patch_vip_servers_by_id(
    client: &Client,
    id: i64,
    body: &VipServerUpdateRequest,
) -> Result<VipServerResponse> {
    client
        .send(patch_vip_servers_by_id_request(id, body)?)
        .await?
        .json()
}

/// `PATCH /v1/vip-servers/{id}/permissions`
///
/// Update friend/clan access and allowed friends/clan list
pub fn patch_vip_servers_by_id_permissions_request(
    id: i64,
    body: &VipServerUpdatePermissionsRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!("/v1/vip-servers/{}/permissions", crate::rt::path_value(&id)),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/vip-servers/{id}/permissions`
///
/// Update friend/clan access and allowed friends/clan list
pub async fn patch_vip_servers_by_id_permissions(
    client: &Client,
    id: i64,
    body: &VipServerUpdatePermissionsRequest,
) -> Result<VipServerPermissionsResponse> {
    client
        .send(patch_vip_servers_by_id_permissions_request(id, body)?)
        .await?
        .json()
}

/// `PATCH /v1/vip-servers/{id}/subscription`
///
/// Updates subscription status of a vip server
pub fn patch_vip_servers_by_id_subscription_request(
    id: i64,
    body: &VipServerUpdateSubscriptionRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!("/v1/vip-servers/{}/subscription", crate::rt::path_value(&id)),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/vip-servers/{id}/subscription`
///
/// Updates subscription status of a vip server
pub async fn patch_vip_servers_by_id_subscription(
    client: &Client,
    id: i64,
    body: &VipServerUpdateSubscriptionRequest,
) -> Result<VipServerSubscriptionResponse> {
    client
        .send(patch_vip_servers_by_id_subscription_request(id, body)?)
        .await?
        .json()
}

/// `PATCH /v1/vip-servers/{id}/voicesettings`
///
/// Update voice settings for a private server.
pub fn patch_vip_servers_by_id_voicesettings_request(
    id: i64,
    body: &VipServerUpdateVoiceSettingsRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!("/v1/vip-servers/{}/voicesettings", crate::rt::path_value(&id)),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/vip-servers/{id}/voicesettings`
///
/// Update voice settings for a private server.
pub async fn patch_vip_servers_by_id_voicesettings(
    client: &Client,
    id: i64,
    body: &VipServerUpdateVoiceSettingsRequest,
) -> Result<VipServerVoiceSettingsResponse> {
    client
        .send(patch_vip_servers_by_id_voicesettings_request(id, body)?)
        .await?
        .json()
}

//! `games.roblox.com` v2 — generated from `vendor/roblox-api-tracker/games/v2.json`.
//!
//! Do not edit. Change the spec or `rbx-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "games.roblox.com";

crate::api_enum! {
    int
    /// \['Forward' = 1, 'Backward' = 2\]
    IExclusiveStartKeyInfoInt32PagingDirection {
    Forward = 1 => "Forward",
    Backward = 2 => "Backward",
    }
}

crate::api_enum! {
    int
    /// \['Asc' = 1, 'Desc' = 2\]
    IExclusiveStartKeyInfoInt32SortOrder {
    Asc = 1 => "Asc",
    Desc = 2 => "Desc",
    }
}

/// `Roblox.Platform.Core.ExclusiveStartPaging.IExclusiveStartKeyInfo\[System.Int32\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct IExclusiveStartKeyInfoInt32 {
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// \['Forward' = 1, 'Backward' = 2\]
    #[serde(rename = "PagingDirection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paging_direction: Option<IExclusiveStartKeyInfoInt32PagingDirection>,
    /// \['Asc' = 1, 'Desc' = 2\]
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<IExclusiveStartKeyInfoInt32SortOrder>,
}

/// `Roblox.Platform.Core.ExclusiveStartPaging.IExclusiveStartKeyInfo\[System.Int64\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct IExclusiveStartKeyInfoInt64 {
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// \['Forward' = 1, 'Backward' = 2\]
    #[serde(rename = "PagingDirection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paging_direction: Option<IExclusiveStartKeyInfoInt32PagingDirection>,
    /// \['Asc' = 1, 'Desc' = 2\]
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<IExclusiveStartKeyInfoInt32SortOrder>,
}

/// `Roblox.Web.Responses.Games.GameMediaItemResponseV2`
///
/// Response model for getting the game media item
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GameMediaItemResponseV2 {
    /// The media item alt text
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

crate::api_enum! {
    int
    /// \['User' = 0, 'Group' = 1\]
    RelatedEntityTypeResponseCreatorTypeType {
    User = 0 => "User",
    Group = 1 => "Group",
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
    /// \['User' = 0, 'Group' = 1\]
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<RelatedEntityTypeResponseCreatorTypeType>,
}

crate::api_enum! {
    int
    /// \['Image' = 1, 'TShirt' = 2, 'Audio' = 3, 'Mesh' = 4, 'Lua' = 5, 'HTML' = 6, 'Text' = 7, 'Hat' = 8, 'Place' = 9, 'Model' = 10, 'Shirt' = 11, 'Pants' = 12, 'Decal' = 13, 'Avatar' = 16, 'Head' = 17, 'Face' = 18, 'Gear' = 19, 'Badge' = 21, 'GroupEmblem' = 22, 'Animation' = 24, 'Arms' = 25, 'Legs' = 26, 'Torso' = 27, 'RightArm' = 28, 'LeftArm' = 29, 'LeftLeg' = 30, 'RightLeg' = 31, 'Package' = 32, 'YouTubeVideo' = 33, 'GamePass' = 34, 'App' = 35, 'Code' = 37, 'Plugin' = 38, 'SolidModel' = 39, 'MeshPart' = 40, 'HairAccessory' = 41, 'FaceAccessory' = 42, 'NeckAccessory' = 43, 'ShoulderAccessory' = 44, 'FrontAccessory' = 45, 'BackAccessory' = 46, 'WaistAccessory' = 47, 'ClimbAnimation' = 48, 'DeathAnimation' = 49, 'FallAnimation' = 50, 'IdleAnimation' = 51, 'JumpAnimation' = 52, 'RunAnimation' = 53, 'SwimAnimation' = 54, 'WalkAnimation' = 55, 'PoseAnimation' = 56, 'LocalizationTableManifest' = 59, 'LocalizationTableTranslation' = 60, 'EmoteAnimation' = 61, 'Video' = 62, 'TexturePack' = 63, 'TShirtAccessory' = 64, 'ShirtAccessory' = 65, 'PantsAccessory' = 66, 'JacketAccessory' = 67, 'SweaterAccessory' = 68, 'ShortsAccessory' = 69, 'LeftShoeAccessory' = 70, 'RightShoeAccessory' = 71, 'DressSkirtAccessory' = 72, 'FontFamily' = 73, 'FontFace' = 74, 'MeshHiddenSurfaceRemoval' = 75, 'EyebrowAccessory' = 76, 'EyelashAccessory' = 77, 'MoodAnimation' = 78, 'DynamicHead' = 79, 'CodeSnippet' = 80, 'AdsVideo' = 81, 'OtaUpdate' = 82\]
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
    /// \['Image' = 1, 'TShirt' = 2, 'Audio' = 3, 'Mesh' = 4, 'Lua' = 5, 'HTML' = 6, 'Text' = 7, 'Hat' = 8, 'Place' = 9, 'Model' = 10, 'Shirt' = 11, 'Pants' = 12, 'Decal' = 13, 'Avatar' = 16, 'Head' = 17, 'Face' = 18, 'Gear' = 19, 'Badge' = 21, 'GroupEmblem' = 22, 'Animation' = 24, 'Arms' = 25, 'Legs' = 26, 'Torso' = 27, 'RightArm' = 28, 'LeftArm' = 29, 'LeftLeg' = 30, 'RightLeg' = 31, 'Package' = 32, 'YouTubeVideo' = 33, 'GamePass' = 34, 'App' = 35, 'Code' = 37, 'Plugin' = 38, 'SolidModel' = 39, 'MeshPart' = 40, 'HairAccessory' = 41, 'FaceAccessory' = 42, 'NeckAccessory' = 43, 'ShoulderAccessory' = 44, 'FrontAccessory' = 45, 'BackAccessory' = 46, 'WaistAccessory' = 47, 'ClimbAnimation' = 48, 'DeathAnimation' = 49, 'FallAnimation' = 50, 'IdleAnimation' = 51, 'JumpAnimation' = 52, 'RunAnimation' = 53, 'SwimAnimation' = 54, 'WalkAnimation' = 55, 'PoseAnimation' = 56, 'LocalizationTableManifest' = 59, 'LocalizationTableTranslation' = 60, 'EmoteAnimation' = 61, 'Video' = 62, 'TexturePack' = 63, 'TShirtAccessory' = 64, 'ShirtAccessory' = 65, 'PantsAccessory' = 66, 'JacketAccessory' = 67, 'SweaterAccessory' = 68, 'ShortsAccessory' = 69, 'LeftShoeAccessory' = 70, 'RightShoeAccessory' = 71, 'DressSkirtAccessory' = 72, 'FontFamily' = 73, 'FontFace' = 74, 'MeshHiddenSurfaceRemoval' = 75, 'EyebrowAccessory' = 76, 'EyelashAccessory' = 77, 'MoodAnimation' = 78, 'DynamicHead' = 79, 'CodeSnippet' = 80, 'AdsVideo' = 81, 'OtaUpdate' = 82\]
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<RelatedEntityTypeResponseAssetTypeType>,
}

/// `Roblox.Web.Responses.Games.GameResponseV2`
///
/// A response model representing a game.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GameResponseV2 {
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

/// `Roblox.Web.WebAPI.ExclusiveStartRequest\[System.Int32\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ExclusiveStartRequestInt32 {
    #[serde(rename = "CursorRecipe")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor_recipe: Option<String>,
    #[serde(rename = "ExclusiveStartKeyInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_key_info: Option<IExclusiveStartKeyInfoInt32>,
}

/// `Roblox.Web.WebAPI.ExclusiveStartRequest\[System.Int64\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ExclusiveStartRequestInt64 {
    #[serde(rename = "CursorRecipe")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor_recipe: Option<String>,
    #[serde(rename = "ExclusiveStartKeyInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_key_info: Option<IExclusiveStartKeyInfoInt64>,
}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.Web.Responses.Games.GameMediaItemResponseV2\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseGameMediaItemResponseV2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GameMediaItemResponseV2>>,
}

/// `Roblox.Web.WebAPI.Models.ApiPageResponse\[Roblox.Web.Responses.Games.GameResponseV2\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiPageResponseGameResponseV2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GameResponseV2>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
}

crate::api_enum! {
    str
    /// The order the results are sorted in.
    GetGroupsByGroupIdGamesSortOrder {
    Asc = 0 => "Asc",
    Desc = 1 => "Desc",
    }
}

/// Query, header and form inputs for [`get_groups_by_group_id_games`].
#[derive(Debug, Clone, Default)]
pub struct GetGroupsByGroupIdGamesParams {
    /// Filtering option via access level.
    pub access_filter: Option<i32>,
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// The order the results are sorted in.
    pub sort_order: Option<GetGroupsByGroupIdGamesSortOrder>,
}

/// Query, header and form inputs for [`get_groups_by_group_id_games_v2`].
#[derive(Debug, Clone, Default)]
pub struct GetGroupsByGroupIdGamesV2Params {
    /// Filtering option via access level.
    pub access_filter: Option<i32>,
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// The order the results are sorted in.
    pub sort_order: Option<GetGroupsByGroupIdGamesSortOrder>,
}

/// Query, header and form inputs for [`get_users_by_user_id_favorite_games`].
#[derive(Debug, Clone, Default)]
pub struct GetUsersByUserIdFavoriteGamesParams {
    /// Filtering option via access level.
    pub access_filter: Option<i32>,
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// The order the results are sorted in.
    pub sort_order: Option<GetGroupsByGroupIdGamesSortOrder>,
}

/// Query, header and form inputs for [`get_users_by_user_id_games`].
#[derive(Debug, Clone, Default)]
pub struct GetUsersByUserIdGamesParams {
    /// Filtering option via access level.
    pub access_filter: Option<i32>,
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// The order the results are sorted in.
    pub sort_order: Option<GetGroupsByGroupIdGamesSortOrder>,
}

/// `GET /v2/games/{universeId}/media`
///
/// Get the game media data
pub fn get_games_by_universe_id_media_request(universe_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v2/games/{}/media", crate::rt::path_value(&universe_id)),
    );
    Ok(r)
}

/// `GET /v2/games/{universeId}/media`
///
/// Get the game media data
pub async fn get_games_by_universe_id_media(
    client: &Client,
    universe_id: i64,
) -> Result<ApiArrayResponseGameMediaItemResponseV2> {
    client
        .send(get_games_by_universe_id_media_request(universe_id)?)
        .await?
        .json()
}

/// `GET /v2/groups/{groupId}/games`
///
/// Gets games created by the specified group.
pub fn get_groups_by_group_id_games_request(
    group_id: i64,
    params: &GetGroupsByGroupIdGamesParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v2/groups/{}/games", crate::rt::path_value(&group_id)),
    );
    r.query("accessFilter", &params.access_filter);
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    r.query("sortOrder", &params.sort_order);
    Ok(r)
}

/// `GET /v2/groups/{groupId}/games`
///
/// Gets games created by the specified group.
pub async fn get_groups_by_group_id_games(
    client: &Client,
    group_id: i64,
    params: &GetGroupsByGroupIdGamesParams,
) -> Result<ApiPageResponseGameResponseV2> {
    client
        .send(get_groups_by_group_id_games_request(group_id, params)?)
        .await?
        .json()
}

/// `GET /v2/groups/{groupId}/gamesV2`
///
/// Gets games created by the specified group. This will replace V1 after validation. Note that this can only used for public/all universes.
pub fn get_groups_by_group_id_games_v2_request(
    group_id: i64,
    params: &GetGroupsByGroupIdGamesV2Params,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v2/groups/{}/gamesV2", crate::rt::path_value(&group_id)),
    );
    r.query("accessFilter", &params.access_filter);
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    r.query("sortOrder", &params.sort_order);
    Ok(r)
}

/// `GET /v2/groups/{groupId}/gamesV2`
///
/// Gets games created by the specified group. This will replace V1 after validation. Note that this can only used for public/all universes.
pub async fn get_groups_by_group_id_games_v2(
    client: &Client,
    group_id: i64,
    params: &GetGroupsByGroupIdGamesV2Params,
) -> Result<ApiPageResponseGameResponseV2> {
    client
        .send(get_groups_by_group_id_games_v2_request(group_id, params)?)
        .await?
        .json()
}

/// `GET /v2/users/{userId}/favorite/games`
///
/// Gets users favorite games.
#[deprecated(note = "marked deprecated by Roblox")]
pub fn get_users_by_user_id_favorite_games_request(
    user_id: i64,
    params: &GetUsersByUserIdFavoriteGamesParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v2/users/{}/favorite/games", crate::rt::path_value(&user_id)),
    );
    r.query("accessFilter", &params.access_filter);
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    r.query("sortOrder", &params.sort_order);
    Ok(r)
}

/// `GET /v2/users/{userId}/favorite/games`
///
/// Gets users favorite games.
#[deprecated(note = "marked deprecated by Roblox")]
pub async fn get_users_by_user_id_favorite_games(
    client: &Client,
    user_id: i64,
    params: &GetUsersByUserIdFavoriteGamesParams,
) -> Result<ApiPageResponseGameResponseV2> {
    client
        .send(get_users_by_user_id_favorite_games_request(user_id, params)?)
        .await?
        .json()
}

/// `GET /v2/users/{userId}/games`
///
/// Gets games created by the specified user.
pub fn get_users_by_user_id_games_request(
    user_id: i64,
    params: &GetUsersByUserIdGamesParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v2/users/{}/games", crate::rt::path_value(&user_id)),
    );
    r.query("accessFilter", &params.access_filter);
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    r.query("sortOrder", &params.sort_order);
    Ok(r)
}

/// `GET /v2/users/{userId}/games`
///
/// Gets games created by the specified user.
pub async fn get_users_by_user_id_games(
    client: &Client,
    user_id: i64,
    params: &GetUsersByUserIdGamesParams,
) -> Result<ApiPageResponseGameResponseV2> {
    client
        .send(get_users_by_user_id_games_request(user_id, params)?)
        .await?
        .json()
}

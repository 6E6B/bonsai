//! `badges.roblox.com` v1 — generated from `vendor/roblox-api-tracker/badges/v1.json`.
//!
//! Do not edit. Change the spec or `rbx-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "badges.roblox.com";

/// `Roblox.Badges.Api.BadgeAwardResponse`
///
/// The result of being awarded a badge.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BadgeAwardResponse {
    /// When the badge was awarded.
    #[serde(rename = "awardedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub awarded_date: Option<String>,
    /// The badge Id.
    #[serde(rename = "badgeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge_id: Option<i64>,
}

/// `Roblox.Badges.Api.BadgeCreatorResponse`
///
/// Represents information about the badge creator. (Creator of the place that awarded the badge)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BadgeCreatorResponse {
    /// The creator ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The name of the creator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The type of the creator
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// `Roblox.Badges.Api.BadgeMetadataResponse`
///
/// Metadata about badges.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BadgeMetadataResponse {
    /// The cost in Robux for creating a new badge.
    #[serde(rename = "badgeCreationPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge_creation_price: Option<i64>,
    /// The asset ID of the curated default badge icon used when a badge is created without an uploaded icon file, or '0' when the default badge icon feature is disabled.
    #[serde(rename = "defaultBadgeIconImageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_badge_icon_image_id: Option<i64>,
    /// The max length for a badge description.
    #[serde(rename = "maxBadgeDescriptionLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_badge_description_length: Option<i32>,
    /// The max length for a badge name.
    #[serde(rename = "maxBadgeNameLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_badge_name_length: Option<i32>,
}

/// `Roblox.Badges.Api.UniverseResponse`
///
/// A response containing universe information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UniverseResponse {
    /// The universe Id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The universe name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The description of the universe.
    #[serde(rename = "rootPlaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_place_id: Option<i64>,
}

/// `Roblox.Web.Responses.Badges.BadgeAwardStatisticsResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BadgeAwardStatisticsResponse {
    #[serde(rename = "awardedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub awarded_count: Option<i64>,
    #[serde(rename = "pastDayAwardedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub past_day_awarded_count: Option<i64>,
    #[serde(rename = "winRatePercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub win_rate_percentage: Option<f64>,
}

/// `Roblox.Badges.Api.BadgeResponse`
///
/// A response containing badge information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BadgeResponse {
    #[serde(rename = "awardingUniverse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub awarding_universe: Option<UniverseResponse>,
    /// When the badge was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// The badge description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The localized badge description.
    #[serde(rename = "displayDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    /// The localized badge icon asset Id.
    #[serde(rename = "displayIconImageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_icon_image_id: Option<i64>,
    /// The localized name of the badge.
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Whether or not the badge is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The badge icon asset Id.
    #[serde(rename = "iconImageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_image_id: Option<i64>,
    /// The badge Id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The name of the badge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<BadgeAwardStatisticsResponse>,
    /// When the badge was updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

crate::api_enum! {
    int
    /// \['Place' = 1\]
    RelatedEntityTypeResponseBadgeAwarderTypeType {
    Place = 1 => "Place",
    }
}

/// `Roblox.Web.Responses.RelatedEntityTypeResponse\[Roblox.Platform.Badges.BadgeAwarderType\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RelatedEntityTypeResponseBadgeAwarderType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// \['Place' = 1\]
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<RelatedEntityTypeResponseBadgeAwarderTypeType>,
}

/// `Roblox.Badges.Api.GetBadgesByUserResponse`
///
/// Response for the GetBadgesByUser endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetBadgesByUserResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub awarder: Option<RelatedEntityTypeResponseBadgeAwarderType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<BadgeCreatorResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[serde(rename = "displayIconImageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_icon_image_id: Option<i64>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "iconImageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_image_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<BadgeAwardStatisticsResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

/// `Roblox.Badges.Api.IconUploadResponse`
///
/// Badge icon upload response.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct IconUploadResponse {
    /// The asset id of the uploaded icon.
    #[serde(rename = "targetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<i64>,
}

/// `Roblox.Badges.Api.UpdateBadgeRequest`
///
/// A request model used for updating badge information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateBadgeRequest {
    /// The new badge description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The new enabled state of the badge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The new badge name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.Web.Responses.Badges.BadgeResponseV2`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BadgeResponseV2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub awarder: Option<RelatedEntityTypeResponseBadgeAwarderType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[serde(rename = "displayIconImageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_icon_image_id: Option<i64>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "iconImageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_image_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<BadgeAwardStatisticsResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

/// `Roblox.Web.WebAPI.ApiEmptyResponseModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiEmptyResponseModel {}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.Badges.Api.BadgeAwardResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseBadgeAwardResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<BadgeAwardResponse>>,
}

/// `Roblox.Web.WebAPI.Models.ApiPageResponse\[Roblox.Badges.Api.BadgeResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiPageResponseBadgeResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<BadgeResponse>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
}

/// `Roblox.Web.WebAPI.Models.ApiPageResponse\[Roblox.Badges.Api.GetBadgesByUserResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiPageResponseGetBadgesByUserResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GetBadgesByUserResponse>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
}

/// Query, header and form inputs for [`post_badges_by_badge_id_icon`].
#[derive(Debug, Clone, Default)]
pub struct PostBadgesByBadgeIdIconParams {
    pub files: Option<FilePart>,
}

crate::api_enum! {
    str
    /// The key to sort badges by.
    GetUniversesByUniverseIdBadgesSortBy {
    Rank = 0 => "Rank",
    DateCreated = 1 => "DateCreated",
    }
}

crate::api_enum! {
    str
    /// The order the results are sorted in.
    GetUniversesByUniverseIdBadgesSortOrder {
    Asc = 0 => "Asc",
    Desc = 1 => "Desc",
    }
}

/// Query, header and form inputs for [`get_universes_by_universe_id_badges`].
#[derive(Debug, Clone, Default)]
pub struct GetUniversesByUniverseIdBadgesParams {
    /// The key to sort badges by.
    pub sort_by: Option<GetUniversesByUniverseIdBadgesSortBy>,
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// The order the results are sorted in.
    pub sort_order: Option<GetUniversesByUniverseIdBadgesSortOrder>,
}

/// Query, header and form inputs for [`post_universes_by_universe_id_badges`].
#[derive(Debug, Clone, Default)]
pub struct PostUniversesByUniverseIdBadgesParams {
    /// The badge name.
    pub name: Option<String>,
    /// The badge description.
    pub description: Option<String>,
    /// Whether or not to pay for the badge with user funds, or group funds.
    pub payment_source_type: Option<i32>,
    /// The badge icon. Optional: when omitted and a curated default badge icon is configured on the service, the badge is created with that default icon. When provided, exactly one file must be supplied.
    pub files: Option<FilePart>,
    /// User expected cost of a badge.
    pub expected_cost: Option<i64>,
    /// Whether or not the badge should be created in the active state.
    pub is_active: Option<bool>,
}

/// Query, header and form inputs for [`get_users_by_user_id_badges`].
#[derive(Debug, Clone, Default)]
pub struct GetUsersByUserIdBadgesParams {
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// The order the results are sorted in.
    pub sort_order: Option<GetUniversesByUniverseIdBadgesSortOrder>,
}

/// Query, header and form inputs for [`get_users_by_user_id_badges_awarded_dates`].
#[derive(Debug, Clone, Default)]
pub struct GetUsersByUserIdBadgesAwardedDatesParams {
    /// The CSV of badge Ids.
    pub badge_ids: Vec<i64>,
}

/// `GET /v1/badges/metadata`
///
/// Gets metadata about the badges system.
///
/// Returns creation price and field length limits. Callers use this before creating or editing badges.
pub fn get_badges_metadata_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/badges/metadata".to_string());
    Ok(r)
}

/// `GET /v1/badges/metadata`
///
/// Gets metadata about the badges system.
///
/// Returns creation price and field length limits. Callers use this before creating or editing badges.
pub async fn get_badges_metadata(client: &Client) -> Result<BadgeMetadataResponse> {
    client.send(get_badges_metadata_request()?).await?.json()
}

/// `GET /v1/badges/{badgeId}`
///
/// Gets badge information by the badge Id.
///
/// Returns display fields, award statistics, and owning universe for a single badge.
pub fn get_badges_by_badge_id_request(badge_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/badges/{}", crate::rt::path_value(&badge_id)),
    );
    Ok(r)
}

/// `GET /v1/badges/{badgeId}`
///
/// Gets badge information by the badge Id.
///
/// Returns display fields, award statistics, and owning universe for a single badge.
pub async fn get_badges_by_badge_id(client: &Client, badge_id: i64) -> Result<BadgeResponse> {
    client
        .send(get_badges_by_badge_id_request(badge_id)?)
        .await?
        .json()
}

/// `PATCH /v1/badges/{badgeId}`
///
/// Updates badge configuration.
///
/// Managers may change enabled state, name, and description when authorized for the badge.
pub fn patch_badges_by_badge_id_request(badge_id: i64, body: &UpdateBadgeRequest) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!("/v1/badges/{}", crate::rt::path_value(&badge_id)),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/badges/{badgeId}`
///
/// Updates badge configuration.
///
/// Managers may change enabled state, name, and description when authorized for the badge.
pub async fn patch_badges_by_badge_id(
    client: &Client,
    badge_id: i64,
    body: &UpdateBadgeRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(patch_badges_by_badge_id_request(badge_id, body)?)
        .await?
        .json()
}

/// `POST /v1/badges/{badgeId}/icon`
///
/// Overwrites a badge icon with a new one.
///
/// Accepts a moderated image upload and updates the badge's icon asset.
pub fn post_badges_by_badge_id_icon_request(
    badge_id: i64,
    params: &PostBadgesByBadgeIdIconParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!("/v1/badges/{}/icon", crate::rt::path_value(&badge_id)),
    );
    r.form_file("Files", &params.files);
    Ok(r)
}

/// `POST /v1/badges/{badgeId}/icon`
///
/// Overwrites a badge icon with a new one.
///
/// Accepts a moderated image upload and updates the badge's icon asset.
pub async fn post_badges_by_badge_id_icon(
    client: &Client,
    badge_id: i64,
    params: &PostBadgesByBadgeIdIconParams,
) -> Result<IconUploadResponse> {
    client
        .send(post_badges_by_badge_id_icon_request(badge_id, params)?)
        .await?
        .json()
}

/// `GET /v1/universes/{universeId}/badges`
///
/// Gets badges by their awarding game.
///
/// Lists badges created under the universe's root place, with paging and optional sort.
pub fn get_universes_by_universe_id_badges_request(
    universe_id: i64,
    params: &GetUniversesByUniverseIdBadgesParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/universes/{}/badges", crate::rt::path_value(&universe_id)),
    );
    r.query("sortBy", &params.sort_by);
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    r.query("sortOrder", &params.sort_order);
    Ok(r)
}

/// `GET /v1/universes/{universeId}/badges`
///
/// Gets badges by their awarding game.
///
/// Lists badges created under the universe's root place, with paging and optional sort.
pub async fn get_universes_by_universe_id_badges(
    client: &Client,
    universe_id: i64,
    params: &GetUniversesByUniverseIdBadgesParams,
) -> Result<ApiPageResponseBadgeResponse> {
    client
        .send(get_universes_by_universe_id_badges_request(universe_id, params)?)
        .await?
        .json()
}

/// `POST /v1/universes/{universeId}/badges`
///
/// Creates a new badge.
///
/// Charges the selected payment source and creates the badge under the universe's root place.
pub fn post_universes_by_universe_id_badges_request(
    universe_id: i64,
    params: &PostUniversesByUniverseIdBadgesParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!("/v1/universes/{}/badges", crate::rt::path_value(&universe_id)),
    );
    r.form_text("name", &params.name);
    r.form_text("description", &params.description);
    r.form_text("paymentSourceType", &params.payment_source_type);
    r.form_file("files", &params.files);
    r.form_text("expectedCost", &params.expected_cost);
    r.form_text("isActive", &params.is_active);
    Ok(r)
}

/// `POST /v1/universes/{universeId}/badges`
///
/// Creates a new badge.
///
/// Charges the selected payment source and creates the badge under the universe's root place.
pub async fn post_universes_by_universe_id_badges(
    client: &Client,
    universe_id: i64,
    params: &PostUniversesByUniverseIdBadgesParams,
) -> Result<BadgeResponseV2> {
    client
        .send(post_universes_by_universe_id_badges_request(universe_id, params)?)
        .await?
        .json()
}

/// `GET /v1/universes/{universeId}/free-badges-quota`
///
/// Gets the number of free badges left for the current UTC day by their awarding game.
///
/// Reflects daily free-creation allowance for the universe when the feature is enabled.
pub fn get_universes_by_universe_id_free_badges_quota_request(universe_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/universes/{}/free-badges-quota",
            crate::rt::path_value(&universe_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/universes/{universeId}/free-badges-quota`
///
/// Gets the number of free badges left for the current UTC day by their awarding game.
///
/// Reflects daily free-creation allowance for the universe when the feature is enabled.
pub async fn get_universes_by_universe_id_free_badges_quota(
    client: &Client,
    universe_id: i64,
) -> Result<i32> {
    client
        .send(get_universes_by_universe_id_free_badges_quota_request(
            universe_id,
        )?)
        .await?
        .json()
}

/// `DELETE /v1/user/badges/{badgeId}`
///
/// Removes a badge from the authenticated user.
///
/// Revokes the caller's own awarded badge instance when the badge exists on their account.
pub fn delete_user_badges_by_badge_id_request(badge_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::DELETE,
        HOST,
        format!("/v1/user/badges/{}", crate::rt::path_value(&badge_id)),
    );
    Ok(r)
}

/// `DELETE /v1/user/badges/{badgeId}`
///
/// Removes a badge from the authenticated user.
///
/// Revokes the caller's own awarded badge instance when the badge exists on their account.
pub async fn delete_user_badges_by_badge_id(client: &Client, badge_id: i64) -> Result<ApiEmptyResponseModel> {
    client
        .send(delete_user_badges_by_badge_id_request(badge_id)?)
        .await?
        .json()
}

/// `GET /v1/users/{userId}/badges`
///
/// Gets a list of badges a user has been awarded.
///
/// Respects inventory privacy settings when evaluating which awarded badges are visible.
pub fn get_users_by_user_id_badges_request(
    user_id: i64,
    params: &GetUsersByUserIdBadgesParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/users/{}/badges", crate::rt::path_value(&user_id)),
    );
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    r.query("sortOrder", &params.sort_order);
    Ok(r)
}

/// `GET /v1/users/{userId}/badges`
///
/// Gets a list of badges a user has been awarded.
///
/// Respects inventory privacy settings when evaluating which awarded badges are visible.
pub async fn get_users_by_user_id_badges(
    client: &Client,
    user_id: i64,
    params: &GetUsersByUserIdBadgesParams,
) -> Result<ApiPageResponseGetBadgesByUserResponse> {
    client
        .send(get_users_by_user_id_badges_request(user_id, params)?)
        .await?
        .json()
}

/// `GET /v1/users/{userId}/badges/awarded-dates`
///
/// Gets timestamps for when badges were awarded to a user.
///
/// Batch form of awarded-date lookup with a maximum number of badge ids per request.
pub fn get_users_by_user_id_badges_awarded_dates_request(
    user_id: i64,
    params: &GetUsersByUserIdBadgesAwardedDatesParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/users/{}/badges/awarded-dates",
            crate::rt::path_value(&user_id)
        ),
    );
    r.query("badgeIds", &params.badge_ids);
    Ok(r)
}

/// `GET /v1/users/{userId}/badges/awarded-dates`
///
/// Gets timestamps for when badges were awarded to a user.
///
/// Batch form of awarded-date lookup with a maximum number of badge ids per request.
pub async fn get_users_by_user_id_badges_awarded_dates(
    client: &Client,
    user_id: i64,
    params: &GetUsersByUserIdBadgesAwardedDatesParams,
) -> Result<ApiArrayResponseBadgeAwardResponse> {
    client
        .send(get_users_by_user_id_badges_awarded_dates_request(
            user_id, params,
        )?)
        .await?
        .json()
}

/// `GET /v1/users/{userId}/badges/{badgeId}/awarded-date`
///
/// Gets timestamp for when a single badge was awarded to a user.
///
/// Returns 204 when the user has not been awarded the badge or the badge is filtered from view.
pub fn get_users_by_user_id_badges_by_badge_id_awarded_date_request(
    user_id: i64,
    badge_id: i64,
) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/users/{}/badges/{}/awarded-date",
            crate::rt::path_value(&user_id),
            crate::rt::path_value(&badge_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/users/{userId}/badges/{badgeId}/awarded-date`
///
/// Gets timestamp for when a single badge was awarded to a user.
///
/// Returns 204 when the user has not been awarded the badge or the badge is filtered from view.
pub async fn get_users_by_user_id_badges_by_badge_id_awarded_date(
    client: &Client,
    user_id: i64,
    badge_id: i64,
) -> Result<()> {
    client
        .send(get_users_by_user_id_badges_by_badge_id_awarded_date_request(
            user_id, badge_id,
        )?)
        .await?
        .unit()
}

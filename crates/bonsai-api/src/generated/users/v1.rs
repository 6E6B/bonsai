//! `users.roblox.com` v1 — generated from `vendor/roblox-api-tracker/users/v1.json`.
//!
//! Do not edit. Change the spec or `bonsai-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "users.roblox.com";

/// `Roblox.Users.Api.AuthenticatedGetUserResponse`
///
/// A response model representing absolute minimal authenticating user information. No new attributes should be added to this response since it is in the critical path of app launch and we want to minimize dependencies.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AuthenticatedGetUserResponse {
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
}

/// `Roblox.Users.Api.BirthdateRequest`
///
/// The birthdate request
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BirthdateRequest {
    /// The birth day
    #[serde(rename = "birthDay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_day: Option<i32>,
    /// The birth month
    #[serde(rename = "birthMonth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_month: Option<i32>,
    /// The birth year
    #[serde(rename = "birthYear")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_year: Option<i32>,
    /// Password
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

/// `Roblox.Users.Api.BirthdateResponse`
///
/// The birthdate response
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BirthdateResponse {
    /// The birth day
    #[serde(rename = "birthDay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_day: Option<i32>,
    /// The birth month
    #[serde(rename = "birthMonth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_month: Option<i32>,
    /// The birth year
    #[serde(rename = "birthYear")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_year: Option<i32>,
}

/// `Roblox.Users.Api.DescriptionRequest`
///
/// The description request
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DescriptionRequest {
    /// The description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// `Roblox.Users.Api.DescriptionResponse`
///
/// The description response
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DescriptionResponse {
    /// The description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

crate::api_enum! {
    int
    /// The gender \['Unknown' = 1, 'Male' = 2, 'Female' = 3\]
    GenderRequestGender {
    UnknownValue = 1 => "Unknown",
    Male = 2 => "Male",
    Female = 3 => "Female",
    }
}

/// `Roblox.Users.Api.GenderRequest`
///
/// The gender request
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GenderRequest {
    /// The gender \['Unknown' = 1, 'Male' = 2, 'Female' = 3\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<GenderRequestGender>,
}

/// `Roblox.Users.Api.GenderResponse`
///
/// The gender response
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GenderResponse {
    /// The gender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<i32>,
}

/// `Roblox.Users.Api.GetUserResponse`
///
/// A response model representing user information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetUserResponse {
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
    /// Unused, legacy attribute. For now always null to not disturb existing client code that might rely on its existence.
    #[serde(rename = "externalAppDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_app_display_name: Option<String>,
    /// The user's verified badge status.
    #[serde(rename = "hasVerifiedBadge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_verified_badge: Option<bool>,
    /// The user Id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Whether the user is banned
    #[serde(rename = "isBanned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_banned: Option<bool>,
    /// The user name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.Users.Api.MultiGetByUserIdRequest`
///
/// Request model for getting users by ids.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MultiGetByUserIdRequest {
    /// Whether the response should exclude banned users
    #[serde(rename = "excludeBannedUsers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_banned_users: Option<bool>,
    /// The user ids.
    #[serde(rename = "userIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<i64>>,
}

/// `Roblox.Users.Api.MultiGetByUsernameRequest`
///
/// Request model for getting users by usernames.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MultiGetByUsernameRequest {
    /// Whether the response should exclude banned users
    #[serde(rename = "excludeBannedUsers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_banned_users: Option<bool>,
    /// The usernames.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usernames: Option<Vec<String>>,
}

/// `Roblox.Users.Api.MultiGetUserByNameResponse`
///
/// A response model specific to multi-get user by name.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MultiGetUserByNameResponse {
    /// The user DisplayName.
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The user's verified badge status.
    #[serde(rename = "hasVerifiedBadge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_verified_badge: Option<bool>,
    /// The user Id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The user name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The username the user was requested with.
    #[serde(rename = "requestedUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_username: Option<String>,
}

/// `Roblox.Users.Api.MultiGetUserResponse`
///
/// \<p\>A response model specific to multi-get user.\</p\> \<p\> Note: Currently the MultiGet responses don't have all the data that the single gets have, to reduce payload size! We might want to revisit this decision in the future. \</p\>
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MultiGetUserResponse {
    /// The user DisplayName.
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The user's verified badge status.
    #[serde(rename = "hasVerifiedBadge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_verified_badge: Option<bool>,
    /// The user Id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The user name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.Users.Api.SearchGetUserResponse`
///
/// A user response model specific to getting a user from user search.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SearchGetUserResponse {
    /// The user DisplayName.
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The user's verified badge status.
    #[serde(rename = "hasVerifiedBadge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_verified_badge: Option<bool>,
    /// The user Id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The user name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Previous usernames for a user.
    #[serde(rename = "previousUsernames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_usernames: Option<Vec<String>>,
}

/// `Roblox.Users.Api.SetDisplayNameRequest`
///
/// Request model for changing a display name.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SetDisplayNameRequest {
    /// The users new display name.
    #[serde(rename = "newDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_display_name: Option<String>,
}

/// `Roblox.Users.Api.UserAgeBracketResponse`
///
/// A user age bracket response model.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UserAgeBracketResponse {
    /// The age bracket of the user.
    #[serde(rename = "ageBracket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_bracket: Option<i32>,
}

/// `Roblox.Users.Api.UserCountryCodeResponse`
///
/// A user country code response model.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UserCountryCodeResponse {
    /// The country code of the user.
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
}

/// `Roblox.Users.Api.UserRolesResponse`
///
/// A user roles response model.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UserRolesResponse {
    /// The roles of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
}

/// `Roblox.Users.Api.UsernameHistoryResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UsernameHistoryResponse {
    /// A past username belonging to a particular userId
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.Web.WebAPI.ApiEmptyResponseModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiEmptyResponseModel {}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.Users.Api.MultiGetUserByNameResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseMultiGetUserByNameResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<MultiGetUserByNameResponse>>,
}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.Users.Api.MultiGetUserResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseMultiGetUserResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<MultiGetUserResponse>>,
}

/// `Roblox.Web.WebAPI.Models.ApiPageResponse\[Roblox.Users.Api.SearchGetUserResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiPageResponseSearchGetUserResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<SearchGetUserResponse>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
}

/// `Roblox.Web.WebAPI.Models.ApiPageResponse\[Roblox.Users.Api.UsernameHistoryResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiPageResponseUsernameHistoryResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<UsernameHistoryResponse>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
}

/// Query, header and form inputs for [`get_display_names_validate`].
#[derive(Debug, Clone, Default)]
pub struct GetDisplayNamesValidateParams {
    /// The display name.
    pub display_name: String,
    /// The new user's birthdate
    pub birthdate: String,
}

/// Query, header and form inputs for [`get_users_search`].
#[derive(Debug, Clone, Default)]
pub struct GetUsersSearchParams {
    /// The search keyword.
    pub keyword: String,
    pub session_id: Option<String>,
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
}

/// Query, header and form inputs for [`get_users_by_user_id_display_names_validate`].
#[derive(Debug, Clone, Default)]
pub struct GetUsersByUserIdDisplayNamesValidateParams {
    /// The display name.
    pub display_name: String,
}

crate::api_enum! {
    str
    /// The order the results are sorted in.
    GetUsersByUserIdUsernameHistorySortOrder {
    Asc = 0 => "Asc",
    Desc = 1 => "Desc",
    }
}

/// Query, header and form inputs for [`get_users_by_user_id_username_history`].
#[derive(Debug, Clone, Default)]
pub struct GetUsersByUserIdUsernameHistoryParams {
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// The order the results are sorted in.
    pub sort_order: Option<GetUsersByUserIdUsernameHistorySortOrder>,
}

/// `GET /v1/birthdate`
///
/// Get the user's birthdate
pub fn get_birthdate_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/birthdate".to_string());
    Ok(r)
}

/// `GET /v1/birthdate`
///
/// Get the user's birthdate
pub async fn get_birthdate(client: &Client) -> Result<BirthdateResponse> {
    client.send(get_birthdate_request()?).await?.json()
}

/// `POST /v1/birthdate`
///
/// Update the user's birthdate
pub fn post_birthdate_request(body: &BirthdateRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/birthdate".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/birthdate`
///
/// Update the user's birthdate
pub async fn post_birthdate(client: &Client, body: &BirthdateRequest) -> Result<ApiEmptyResponseModel> {
    client.send(post_birthdate_request(body)?).await?.json()
}

/// `GET /v1/description`
///
/// Get the user's description
pub fn get_description_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/description".to_string());
    Ok(r)
}

/// `GET /v1/description`
///
/// Get the user's description
pub async fn get_description(client: &Client) -> Result<DescriptionResponse> {
    client.send(get_description_request()?).await?.json()
}

/// `POST /v1/description`
///
/// Update the user's description
pub fn post_description_request(body: &DescriptionRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/description".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/description`
///
/// Update the user's description
pub async fn post_description(client: &Client, body: &DescriptionRequest) -> Result<DescriptionResponse> {
    client.send(post_description_request(body)?).await?.json()
}

/// `GET /v1/display-names/validate`
///
/// Validate a display name for a new user.
pub fn get_display_names_validate_request(params: &GetDisplayNamesValidateParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/display-names/validate".to_string());
    r.query("displayName", &params.display_name);
    r.query("birthdate", &params.birthdate);
    Ok(r)
}

/// `GET /v1/display-names/validate`
///
/// Validate a display name for a new user.
pub async fn get_display_names_validate(
    client: &Client,
    params: &GetDisplayNamesValidateParams,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(get_display_names_validate_request(params)?)
        .await?
        .json()
}

/// `GET /v1/gender`
///
/// Get the user's gender
pub fn get_gender_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/gender".to_string());
    Ok(r)
}

/// `GET /v1/gender`
///
/// Get the user's gender
pub async fn get_gender(client: &Client) -> Result<GenderResponse> {
    client.send(get_gender_request()?).await?.json()
}

/// `POST /v1/gender`
///
/// Update the user's gender
pub fn post_gender_request(body: &GenderRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/gender".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/gender`
///
/// Update the user's gender
pub async fn post_gender(client: &Client, body: &GenderRequest) -> Result<ApiEmptyResponseModel> {
    client.send(post_gender_request(body)?).await?.json()
}

/// `POST /v1/usernames/users`
///
/// Get users by usernames.
///
/// This endpoint will also check previous usernames. Does not require X-CSRF-Token protection because this is essentially a get request but as a POST to avoid URI limits.
pub fn post_usernames_users_request(body: &MultiGetByUsernameRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/usernames/users".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/usernames/users`
///
/// Get users by usernames.
///
/// This endpoint will also check previous usernames. Does not require X-CSRF-Token protection because this is essentially a get request but as a POST to avoid URI limits.
pub async fn post_usernames_users(
    client: &Client,
    body: &MultiGetByUsernameRequest,
) -> Result<ApiArrayResponseMultiGetUserByNameResponse> {
    client.send(post_usernames_users_request(body)?).await?.json()
}

/// `POST /v1/users`
///
/// Get users by ids.
///
/// Does not require X-CSRF-Token protection because this is essentially a get request but as a POST to avoid URI limits.
pub fn post_users_request(body: &MultiGetByUserIdRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/users".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/users`
///
/// Get users by ids.
///
/// Does not require X-CSRF-Token protection because this is essentially a get request but as a POST to avoid URI limits.
pub async fn post_users(
    client: &Client,
    body: &MultiGetByUserIdRequest,
) -> Result<ApiArrayResponseMultiGetUserResponse> {
    client.send(post_users_request(body)?).await?.json()
}

/// `GET /v1/users/authenticated`
///
/// Gets the minimal authenticated user.
pub fn get_users_authenticated_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/users/authenticated".to_string());
    Ok(r)
}

/// `GET /v1/users/authenticated`
///
/// Gets the minimal authenticated user.
pub async fn get_users_authenticated(client: &Client) -> Result<AuthenticatedGetUserResponse> {
    client.send(get_users_authenticated_request()?).await?.json()
}

/// `GET /v1/users/authenticated/age-bracket`
///
/// Gets the age bracket of the authenticated user.
pub fn get_users_authenticated_age_bracket_request() -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        "/v1/users/authenticated/age-bracket".to_string(),
    );
    Ok(r)
}

/// `GET /v1/users/authenticated/age-bracket`
///
/// Gets the age bracket of the authenticated user.
pub async fn get_users_authenticated_age_bracket(client: &Client) -> Result<UserAgeBracketResponse> {
    client
        .send(get_users_authenticated_age_bracket_request()?)
        .await?
        .json()
}

/// `GET /v1/users/authenticated/country-code`
///
/// Gets the country code of the authenticated user.
pub fn get_users_authenticated_country_code_request() -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        "/v1/users/authenticated/country-code".to_string(),
    );
    Ok(r)
}

/// `GET /v1/users/authenticated/country-code`
///
/// Gets the country code of the authenticated user.
pub async fn get_users_authenticated_country_code(client: &Client) -> Result<UserCountryCodeResponse> {
    client
        .send(get_users_authenticated_country_code_request()?)
        .await?
        .json()
}

/// `GET /v1/users/authenticated/roles`
///
/// Gets the (public) roles of the authenticated user, such as '"Soothsayer"' and '"BetaTester"'.
pub fn get_users_authenticated_roles_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/users/authenticated/roles".to_string());
    Ok(r)
}

/// `GET /v1/users/authenticated/roles`
///
/// Gets the (public) roles of the authenticated user, such as '"Soothsayer"' and '"BetaTester"'.
pub async fn get_users_authenticated_roles(client: &Client) -> Result<UserRolesResponse> {
    client
        .send(get_users_authenticated_roles_request()?)
        .await?
        .json()
}

/// `GET /v1/users/search`
///
/// Searches for users by keyword.
pub fn get_users_search_request(params: &GetUsersSearchParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/users/search".to_string());
    r.query("keyword", &params.keyword);
    r.query("sessionId", &params.session_id);
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    Ok(r)
}

/// `GET /v1/users/search`
///
/// Searches for users by keyword.
pub async fn get_users_search(
    client: &Client,
    params: &GetUsersSearchParams,
) -> Result<ApiPageResponseSearchGetUserResponse> {
    client.send(get_users_search_request(params)?).await?.json()
}

/// `GET /v1/users/{userId}`
///
/// Gets detailed user information by id.
pub fn get_users_by_user_id_request(user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/users/{}", crate::rt::path_value(&user_id)),
    );
    Ok(r)
}

/// `GET /v1/users/{userId}`
///
/// Gets detailed user information by id.
pub async fn get_users_by_user_id(client: &Client, user_id: i64) -> Result<GetUserResponse> {
    client.send(get_users_by_user_id_request(user_id)?).await?.json()
}

/// `PATCH /v1/users/{userId}/display-names`
///
/// Set the display name for the authorized user.
pub fn patch_users_by_user_id_display_names_request(
    user_id: i64,
    body: &SetDisplayNameRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!("/v1/users/{}/display-names", crate::rt::path_value(&user_id)),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/users/{userId}/display-names`
///
/// Set the display name for the authorized user.
pub async fn patch_users_by_user_id_display_names(
    client: &Client,
    user_id: i64,
    body: &SetDisplayNameRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(patch_users_by_user_id_display_names_request(user_id, body)?)
        .await?
        .json()
}

/// `GET /v1/users/{userId}/display-names/validate`
///
/// Validate a display name for an existing user.
pub fn get_users_by_user_id_display_names_validate_request(
    user_id: i64,
    params: &GetUsersByUserIdDisplayNamesValidateParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/users/{}/display-names/validate",
            crate::rt::path_value(&user_id)
        ),
    );
    r.query("displayName", &params.display_name);
    Ok(r)
}

/// `GET /v1/users/{userId}/display-names/validate`
///
/// Validate a display name for an existing user.
pub async fn get_users_by_user_id_display_names_validate(
    client: &Client,
    user_id: i64,
    params: &GetUsersByUserIdDisplayNamesValidateParams,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(get_users_by_user_id_display_names_validate_request(
            user_id, params,
        )?)
        .await?
        .json()
}

/// `GET /v1/users/{userId}/username-history`
///
/// Retrieves the username history for a particular user.
pub fn get_users_by_user_id_username_history_request(
    user_id: i64,
    params: &GetUsersByUserIdUsernameHistoryParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/users/{}/username-history", crate::rt::path_value(&user_id)),
    );
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    r.query("sortOrder", &params.sort_order);
    Ok(r)
}

/// `GET /v1/users/{userId}/username-history`
///
/// Retrieves the username history for a particular user.
pub async fn get_users_by_user_id_username_history(
    client: &Client,
    user_id: i64,
    params: &GetUsersByUserIdUsernameHistoryParams,
) -> Result<ApiPageResponseUsernameHistoryResponse> {
    client
        .send(get_users_by_user_id_username_history_request(user_id, params)?)
        .await?
        .json()
}

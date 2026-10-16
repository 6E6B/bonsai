//! `translationroles.roblox.com` v1 — generated from `vendor/roblox-api-tracker/translationroles/v1.json`.
//!
//! Do not edit. Change the spec or `bonsai-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "translationroles.roblox.com";

crate::api_enum! {
    str
    /// \['User' = 0, 'Group' = 1, 'GroupRole' = 2\]
    GameLocalizationRolesAssigneeAssigneeType {
    User = 0 => "user",
    Group = 1 => "group",
    GroupRole = 2 => "groupRole",
    }
}

/// `Roblox.GameLocalization.Client.GameLocalizationRoles.Assignee`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GameLocalizationRolesAssignee {
    /// \['User' = 0, 'Group' = 1, 'GroupRole' = 2\]
    #[serde(rename = "assigneeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_type: Option<GameLocalizationRolesAssigneeAssigneeType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
}

/// `Roblox.GameLocalization.Client.GameLocalizationRoles.GameLocalizationRoleAssignment`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GameLocalizationRoleAssignment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<GameLocalizationRolesAssignee>,
    #[serde(rename = "gameId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_id: Option<i64>,
}

/// `Roblox.TranslationRoles.Api.Assignee`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TranslationRolesAssignee {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// \['User' = 0, 'Group' = 1, 'GroupRole' = 2\]
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<GameLocalizationRolesAssigneeAssigneeType>,
}

/// `Roblox.TranslationRoles.Api.GetGameLocalizationRoleAssignmentsForUserResponse`
///
/// Response model containing a list of games and associated role assignment info for the requested user and role.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetGameLocalizationRoleAssignmentsForUserResponse {
    /// List of games with associated role assignment info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub games: Option<Vec<GameLocalizationRoleAssignment>>,
    /// Cursor which can be used for navigating to next page.
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    /// Cursor which can be used for navigating to previous page.
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
}

crate::api_enum! {
    str
    /// The role to be assigned or revoked \['Translator' = 0\]
    UpdateRoleRequestRole {
    Translator = 0 => "translator",
    }
}

/// `Roblox.TranslationRoles.Api.UpdateRoleRequest`
///
/// The request body for update role endpoints
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateRoleRequest {
    /// The id of the assignee
    #[serde(rename = "assigneeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_id: Option<i64>,
    /// The type of the assignee \['User' = 0, 'Group' = 1, 'GroupRole' = 2\]
    #[serde(rename = "assigneeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_type: Option<GameLocalizationRolesAssigneeAssigneeType>,
    /// To assign or to revoke
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke: Option<bool>,
    /// The role to be assigned or revoked \['Translator' = 0\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<UpdateRoleRequestRole>,
}

/// `Roblox.Web.WebAPI.ApiEmptyResponseModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiEmptyResponseModel {}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.TranslationRoles.Api.Assignee\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseAssignee {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<TranslationRolesAssignee>>,
}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[System.String\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseString {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<String>>,
}

/// Query, header and form inputs for [`get_game_localization_roles_roles_by_role_current_user`].
#[derive(Debug, Clone, Default)]
pub struct GetGameLocalizationRolesRolesByRoleCurrentUserParams {
    /// Part of pagination. Last primary key of returned items in previous operation.
    pub exclusive_start_key: Option<String>,
    /// Part of pagination. Maximum number of items that might be returned in the page.
    pub page_size: Option<i32>,
    /// Optional seleted groupId of resources requested for the user and role.
    pub group_id: Option<i64>,
}

/// `PATCH /v1/game-localization-roles/games/{gameId}`
///
/// Assigns or revokes a role
pub fn patch_game_localization_roles_games_by_game_id_request(
    game_id: i64,
    body: &UpdateRoleRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::PATCH,
        HOST,
        format!(
            "/v1/game-localization-roles/games/{}",
            crate::rt::path_value(&game_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/game-localization-roles/games/{gameId}`
///
/// Assigns or revokes a role
pub async fn patch_game_localization_roles_games_by_game_id(
    client: &Client,
    game_id: i64,
    body: &UpdateRoleRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(patch_game_localization_roles_games_by_game_id_request(
            game_id, body,
        )?)
        .await?
        .json()
}

/// `GET /v1/game-localization-roles/games/{gameId}/current-user/roles`
///
/// Retrieves the list of roles granted to current logged-in user
pub fn get_game_localization_roles_games_by_game_id_current_user_roles_request(
    game_id: i64,
) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/game-localization-roles/games/{}/current-user/roles",
            crate::rt::path_value(&game_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/game-localization-roles/games/{gameId}/current-user/roles`
///
/// Retrieves the list of roles granted to current logged-in user
pub async fn get_game_localization_roles_games_by_game_id_current_user_roles(
    client: &Client,
    game_id: i64,
) -> Result<ApiArrayResponseString> {
    client
        .send(get_game_localization_roles_games_by_game_id_current_user_roles_request(game_id)?)
        .await?
        .json()
}

/// `GET /v1/game-localization-roles/games/{gameId}/roles/{role}/assignees`
///
/// Gets list of users assigned a specific role in a game.
pub fn get_game_localization_roles_games_by_game_id_roles_by_role_assignees_request(
    game_id: i64,
    role: UpdateRoleRequestRole,
) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/game-localization-roles/games/{}/roles/{}/assignees",
            crate::rt::path_value(&game_id),
            crate::rt::path_value(&role)
        ),
    );
    Ok(r)
}

/// `GET /v1/game-localization-roles/games/{gameId}/roles/{role}/assignees`
///
/// Gets list of users assigned a specific role in a game.
pub async fn get_game_localization_roles_games_by_game_id_roles_by_role_assignees(
    client: &Client,
    game_id: i64,
    role: UpdateRoleRequestRole,
) -> Result<ApiArrayResponseAssignee> {
    client
        .send(get_game_localization_roles_games_by_game_id_roles_by_role_assignees_request(game_id, role)?)
        .await?
        .json()
}

/// `GET /v1/game-localization-roles/roles/{role}/current-user`
///
/// Gets the list of games and associated role assignment info for the requested user and role.
pub fn get_game_localization_roles_roles_by_role_current_user_request(
    role: UpdateRoleRequestRole,
    params: &GetGameLocalizationRolesRolesByRoleCurrentUserParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/game-localization-roles/roles/{}/current-user",
            crate::rt::path_value(&role)
        ),
    );
    r.query("exclusiveStartKey", &params.exclusive_start_key);
    r.query("pageSize", &params.page_size);
    r.query("groupId", &params.group_id);
    Ok(r)
}

/// `GET /v1/game-localization-roles/roles/{role}/current-user`
///
/// Gets the list of games and associated role assignment info for the requested user and role.
pub async fn get_game_localization_roles_roles_by_role_current_user(
    client: &Client,
    role: UpdateRoleRequestRole,
    params: &GetGameLocalizationRolesRolesByRoleCurrentUserParams,
) -> Result<GetGameLocalizationRoleAssignmentsForUserResponse> {
    client
        .send(get_game_localization_roles_roles_by_role_current_user_request(
            role, params,
        )?)
        .await?
        .json()
}

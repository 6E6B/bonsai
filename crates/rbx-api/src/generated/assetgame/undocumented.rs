//! `assetgame.roblox.com` undocumented — generated from `specs/undocumented/assetgame.roblox.com.yaml`.
//!
//! Do not edit. Change the spec or `rbx-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "assetgame.roblox.com";

crate::api_enum! {
    str
    PlaceLauncherRequest {
    RequestGame = 0 => "RequestGame",
    RequestGameJob = 1 => "RequestGameJob",
    RequestFollowUser = 2 => "RequestFollowUser",
    RequestPrivateGame = 3 => "RequestPrivateGame",
    }
}

/// Query, header and form inputs for [`place_launcher`].
#[derive(Debug, Clone, Default)]
pub struct PlaceLauncherParams {
    pub request: PlaceLauncherRequest,
    pub browser_tracker_id: Option<i64>,
    pub place_id: Option<i64>,
    /// Server job id, for RequestGameJob
    pub game_id: Option<String>,
    /// User to follow, for RequestFollowUser
    pub user_id: Option<i64>,
    /// Private server access code, for RequestPrivateGame
    pub access_code: Option<String>,
    /// Private server link code, for RequestPrivateGame
    pub link_code: Option<String>,
    pub is_play_together_game: Option<bool>,
    pub join_attempt_id: Option<String>,
    pub join_attempt_origin: Option<String>,
}

/// `GET /game/PlaceLauncher.ashx`
///
/// Join script request used by the player. request is RequestGame, RequestGameJob, RequestFollowUser or RequestPrivateGame.
///
/// Undocumented; captured 2026-10-12, launch URI from the Play button on www.roblox.com/games/{placeId}.
pub fn place_launcher_request(params: &PlaceLauncherParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/game/PlaceLauncher.ashx".to_string());
    r.query("request", &params.request);
    r.query("browserTrackerId", &params.browser_tracker_id);
    r.query("placeId", &params.place_id);
    r.query("gameId", &params.game_id);
    r.query("userId", &params.user_id);
    r.query("accessCode", &params.access_code);
    r.query("linkCode", &params.link_code);
    r.query("isPlayTogetherGame", &params.is_play_together_game);
    r.query("joinAttemptId", &params.join_attempt_id);
    r.query("joinAttemptOrigin", &params.join_attempt_origin);
    Ok(r)
}

/// `GET /game/PlaceLauncher.ashx`
///
/// Join script request used by the player. request is RequestGame, RequestGameJob, RequestFollowUser or RequestPrivateGame.
///
/// Undocumented; captured 2026-10-12, launch URI from the Play button on www.roblox.com/games/{placeId}.
pub async fn place_launcher(client: &Client, params: &PlaceLauncherParams) -> Result<()> {
    client.send(place_launcher_request(params)?).await?.unit()
}

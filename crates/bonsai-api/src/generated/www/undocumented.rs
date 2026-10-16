//! `www.roblox.com` undocumented — generated from `specs/undocumented/www.roblox.com.yaml`.
//!
//! Do not edit. Change the spec or `bonsai-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "www.roblox.com";

/// Query, header and form inputs for [`page_games_start`].
#[derive(Debug, Clone, Default)]
pub struct PageGamesStartParams {
    pub place_id: i64,
    pub game_instance_id: Option<String>,
}

/// Query, header and form inputs for [`page_report_abuse`].
#[derive(Debug, Clone, Default)]
pub struct PageReportAbuseParams {
    pub target_id: i64,
    pub submitter_id: Option<i64>,
    /// user, place, asset, group
    pub abuse_vector: String,
}

/// Query, header and form inputs for [`page_share_link`].
#[derive(Debug, Clone, Default)]
pub struct PageShareLinkParams {
    pub code: String,
    pub type_: String,
}

/// `GET /badges/{badgeId}`
///
/// Badge page
///
/// Undocumented; captured 2026-10-12, badge page.
pub fn page_badge_request(badge_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/badges/{}", crate::rt::path_value(&badge_id)),
    );
    Ok(r)
}

/// `GET /badges/{badgeId}`
///
/// Badge page
///
/// Undocumented; captured 2026-10-12, badge page.
pub async fn page_badge(client: &Client, badge_id: i64) -> Result<()> {
    client.send(page_badge_request(badge_id)?).await?.unit()
}

/// `GET /bundles/{bundleId}`
///
/// Bundle page
///
/// Undocumented; captured 2026-10-12, marketplace bundle page.
pub fn page_bundle_request(bundle_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/bundles/{}", crate::rt::path_value(&bundle_id)),
    );
    Ok(r)
}

/// `GET /bundles/{bundleId}`
///
/// Bundle page
///
/// Undocumented; captured 2026-10-12, marketplace bundle page.
pub async fn page_bundle(client: &Client, bundle_id: i64) -> Result<()> {
    client.send(page_bundle_request(bundle_id)?).await?.unit()
}

/// `GET /catalog`
///
/// Marketplace
///
/// Undocumented; captured 2026-10-12.
pub fn page_marketplace_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/catalog".to_string());
    Ok(r)
}

/// `GET /catalog`
///
/// Marketplace
///
/// Undocumented; captured 2026-10-12.
pub async fn page_marketplace(client: &Client) -> Result<()> {
    client.send(page_marketplace_request()?).await?.unit()
}

/// `GET /catalog/{assetId}`
///
/// Marketplace item page
///
/// Undocumented; captured 2026-10-12, marketplace item page.
pub fn page_item_request(asset_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/catalog/{}", crate::rt::path_value(&asset_id)),
    );
    Ok(r)
}

/// `GET /catalog/{assetId}`
///
/// Marketplace item page
///
/// Undocumented; captured 2026-10-12, marketplace item page.
pub async fn page_item(client: &Client, asset_id: i64) -> Result<()> {
    client.send(page_item_request(asset_id)?).await?.unit()
}

/// `GET /charts`
///
/// Charts and discover
///
/// Undocumented; captured 2026-10-12.
pub fn page_charts_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/charts".to_string());
    Ok(r)
}

/// `GET /charts`
///
/// Charts and discover
///
/// Undocumented; captured 2026-10-12.
pub async fn page_charts(client: &Client) -> Result<()> {
    client.send(page_charts_request()?).await?.unit()
}

/// `GET /communities`
///
/// Communities
///
/// Undocumented; captured 2026-10-12.
pub fn page_communities_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/communities".to_string());
    Ok(r)
}

/// `GET /communities`
///
/// Communities
///
/// Undocumented; captured 2026-10-12.
pub async fn page_communities(client: &Client) -> Result<()> {
    client.send(page_communities_request()?).await?.unit()
}

/// `GET /communities/{groupId}`
///
/// Community page
///
/// Undocumented; captured 2026-10-12, community page.
pub fn page_community_request(group_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/communities/{}", crate::rt::path_value(&group_id)),
    );
    Ok(r)
}

/// `GET /communities/{groupId}`
///
/// Community page
///
/// Undocumented; captured 2026-10-12, community page.
pub async fn page_community(client: &Client, group_id: i64) -> Result<()> {
    client.send(page_community_request(group_id)?).await?.unit()
}

/// `GET /game-pass/{passId}`
///
/// Game pass page
///
/// Undocumented; captured 2026-10-12, game pass page.
pub fn page_game_pass_request(pass_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/game-pass/{}", crate::rt::path_value(&pass_id)),
    );
    Ok(r)
}

/// `GET /game-pass/{passId}`
///
/// Game pass page
///
/// Undocumented; captured 2026-10-12, game pass page.
pub async fn page_game_pass(client: &Client, pass_id: i64) -> Result<()> {
    client.send(page_game_pass_request(pass_id)?).await?.unit()
}

/// `GET /games/start`
///
/// Website launch landing for a place, optionally a specific server
///
/// Undocumented; captured 2026-10-12, deep link landing used by invite links.
pub fn page_games_start_request(params: &PageGamesStartParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/games/start".to_string());
    r.query("placeId", &params.place_id);
    r.query("gameInstanceId", &params.game_instance_id);
    Ok(r)
}

/// `GET /games/start`
///
/// Website launch landing for a place, optionally a specific server
///
/// Undocumented; captured 2026-10-12, deep link landing used by invite links.
pub async fn page_games_start(client: &Client, params: &PageGamesStartParams) -> Result<()> {
    client.send(page_games_start_request(params)?).await?.unit()
}

/// `GET /games/{placeId}`
///
/// Experience page
///
/// Undocumented; captured 2026-10-12, experience page.
pub fn page_game_request(place_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/games/{}", crate::rt::path_value(&place_id)),
    );
    Ok(r)
}

/// `GET /games/{placeId}`
///
/// Experience page
///
/// Undocumented; captured 2026-10-12, experience page.
pub async fn page_game(client: &Client, place_id: i64) -> Result<()> {
    client.send(page_game_request(place_id)?).await?.unit()
}

/// `GET /home`
///
/// Home
///
/// Undocumented; captured 2026-10-12.
pub fn page_home_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/home".to_string());
    Ok(r)
}

/// `GET /home`
///
/// Home
///
/// Undocumented; captured 2026-10-12.
pub async fn page_home(client: &Client) -> Result<()> {
    client.send(page_home_request()?).await?.unit()
}

/// `GET /login`
///
/// Sign-in page
///
/// Undocumented; captured 2026-10-14, sign-in page; loaded in the app's sign-in window.
pub fn page_login_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/login".to_string());
    Ok(r)
}

/// `GET /login`
///
/// Sign-in page
///
/// Undocumented; captured 2026-10-14, sign-in page; loaded in the app's sign-in window.
pub async fn page_login(client: &Client) -> Result<()> {
    client.send(page_login_request()?).await?.unit()
}

/// `GET /my/account`
///
/// Account settings
///
/// Undocumented; captured 2026-10-12.
pub fn page_settings_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/my/account".to_string());
    Ok(r)
}

/// `GET /my/account`
///
/// Account settings
///
/// Undocumented; captured 2026-10-12.
pub async fn page_settings(client: &Client) -> Result<()> {
    client.send(page_settings_request()?).await?.unit()
}

/// `GET /my/avatar`
///
/// Avatar editor
///
/// Undocumented; captured 2026-10-12.
pub fn page_avatar_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/my/avatar".to_string());
    Ok(r)
}

/// `GET /my/avatar`
///
/// Avatar editor
///
/// Undocumented; captured 2026-10-12.
pub async fn page_avatar(client: &Client) -> Result<()> {
    client.send(page_avatar_request()?).await?.unit()
}

/// `GET /my/messages`
///
/// Private messages
///
/// Undocumented; captured 2026-10-12.
pub fn page_messages_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/my/messages".to_string());
    Ok(r)
}

/// `GET /my/messages`
///
/// Private messages
///
/// Undocumented; captured 2026-10-12.
pub async fn page_messages(client: &Client) -> Result<()> {
    client.send(page_messages_request()?).await?.unit()
}

/// `GET /report-abuse/`
///
/// Report form; the app only opens it, it never submits reports
///
/// Undocumented; captured 2026-10-12, profile menu Report.
pub fn page_report_abuse_request(params: &PageReportAbuseParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/report-abuse/".to_string());
    r.query("targetId", &params.target_id);
    r.query("submitterId", &params.submitter_id);
    r.query("abuseVector", &params.abuse_vector);
    Ok(r)
}

/// `GET /report-abuse/`
///
/// Report form; the app only opens it, it never submits reports
///
/// Undocumented; captured 2026-10-12, profile menu Report.
pub async fn page_report_abuse(client: &Client, params: &PageReportAbuseParams) -> Result<()> {
    client.send(page_report_abuse_request(params)?).await?.unit()
}

/// `GET /share`
///
/// Share link (private servers use type=Server)
///
/// Undocumented; captured 2026-10-12, private server and experience share links.
pub fn page_share_link_request(params: &PageShareLinkParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/share".to_string());
    r.query("code", &params.code);
    r.query("type", &params.type_);
    Ok(r)
}

/// `GET /share`
///
/// Share link (private servers use type=Server)
///
/// Undocumented; captured 2026-10-12, private server and experience share links.
pub async fn page_share_link(client: &Client, params: &PageShareLinkParams) -> Result<()> {
    client.send(page_share_link_request(params)?).await?.unit()
}

/// `GET /trades`
///
/// Trades
///
/// Undocumented; captured 2026-10-12.
pub fn page_trades_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/trades".to_string());
    Ok(r)
}

/// `GET /trades`
///
/// Trades
///
/// Undocumented; captured 2026-10-12.
pub async fn page_trades(client: &Client) -> Result<()> {
    client.send(page_trades_request()?).await?.unit()
}

/// `GET /transactions`
///
/// Transactions
///
/// Undocumented; captured 2026-10-12.
pub fn page_transactions_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/transactions".to_string());
    Ok(r)
}

/// `GET /transactions`
///
/// Transactions
///
/// Undocumented; captured 2026-10-12.
pub async fn page_transactions(client: &Client) -> Result<()> {
    client.send(page_transactions_request()?).await?.unit()
}

/// `GET /upgrades/robux`
///
/// Official Robux purchase page (the app never buys Robux)
///
/// Undocumented; captured 2026-10-12.
pub fn page_buy_robux_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/upgrades/robux".to_string());
    Ok(r)
}

/// `GET /upgrades/robux`
///
/// Official Robux purchase page (the app never buys Robux)
///
/// Undocumented; captured 2026-10-12.
pub async fn page_buy_robux(client: &Client) -> Result<()> {
    client.send(page_buy_robux_request()?).await?.unit()
}

/// `GET /users/friends`
///
/// Friends
///
/// Undocumented; captured 2026-10-12.
pub fn page_friends_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/users/friends".to_string());
    Ok(r)
}

/// `GET /users/friends`
///
/// Friends
///
/// Undocumented; captured 2026-10-12.
pub async fn page_friends(client: &Client) -> Result<()> {
    client.send(page_friends_request()?).await?.unit()
}

/// `GET /users/{userId}/inventory`
///
/// Inventory page
///
/// Undocumented; captured 2026-10-12.
pub fn page_inventory_request(user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/users/{}/inventory", crate::rt::path_value(&user_id)),
    );
    Ok(r)
}

/// `GET /users/{userId}/inventory`
///
/// Inventory page
///
/// Undocumented; captured 2026-10-12.
pub async fn page_inventory(client: &Client, user_id: i64) -> Result<()> {
    client.send(page_inventory_request(user_id)?).await?.unit()
}

/// `GET /users/{userId}/profile`
///
/// Profile page
///
/// Undocumented; captured 2026-10-12, profile page.
pub fn page_profile_request(user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/users/{}/profile", crate::rt::path_value(&user_id)),
    );
    Ok(r)
}

/// `GET /users/{userId}/profile`
///
/// Profile page
///
/// Undocumented; captured 2026-10-12, profile page.
pub async fn page_profile(client: &Client, user_id: i64) -> Result<()> {
    client.send(page_profile_request(user_id)?).await?.unit()
}

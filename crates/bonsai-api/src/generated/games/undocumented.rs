//! `games.roblox.com` undocumented — generated from `specs/undocumented/games.roblox.com.yaml`.
//!
//! Do not edit. Change the spec or `bonsai-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "games.roblox.com";

/// `MyPrivateServer`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MyPrivateServer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "expirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ownerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<i64>,
    #[serde(rename = "ownerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_name: Option<String>,
    #[serde(rename = "placeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_id: Option<i64>,
    #[serde(rename = "priceInRobux")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_in_robux: Option<i64>,
    #[serde(rename = "privateServerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_server_id: Option<i64>,
    #[serde(rename = "universeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_id: Option<i64>,
    #[serde(rename = "universeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_name: Option<String>,
    #[serde(rename = "willRenew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub will_renew: Option<bool>,
}

/// `MyPrivateServers`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MyPrivateServers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<MyPrivateServer>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
}

/// Query, header and form inputs for [`get_my_private_servers`].
#[derive(Debug, Clone, Default)]
pub struct GetMyPrivateServersParams {
    pub private_servers_tab: Option<i32>,
    pub items_per_page: Option<i32>,
    pub cursor: Option<String>,
}

/// `GET /v1/private-servers/my-private-servers`
///
/// Private servers the signed-in account owns (tab 0) or can join (tab 1).
///
/// Undocumented; captured 2026-10-14, www.roblox.com/my/account#!/subscriptions.
pub fn get_my_private_servers_request(params: &GetMyPrivateServersParams) -> Result<Request> {
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
/// Private servers the signed-in account owns (tab 0) or can join (tab 1).
///
/// Undocumented; captured 2026-10-14, www.roblox.com/my/account#!/subscriptions.
pub async fn get_my_private_servers(
    client: &Client,
    params: &GetMyPrivateServersParams,
) -> Result<MyPrivateServers> {
    client.send(get_my_private_servers_request(params)?).await?.json()
}

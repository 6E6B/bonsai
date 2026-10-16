//! `economy.roblox.com` v1 — generated from `vendor/roblox-api-tracker/economy/v1.json`.
//!
//! Do not edit. Change the spec or `bonsai-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "economy.roblox.com";

/// `Roblox.Web.Responses.Economy.CurrencyResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CurrencyResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robux: Option<i64>,
}

/// `GET /v1/user/currency`
///
/// Gets currency for the authenticated user.
///
/// Currency can only be retrieved for the authenticated user.
pub fn get_user_currency_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/user/currency".to_string());
    Ok(r)
}

/// `GET /v1/user/currency`
///
/// Gets currency for the authenticated user.
///
/// Currency can only be retrieved for the authenticated user.
pub async fn get_user_currency(client: &Client) -> Result<CurrencyResponse> {
    client.send(get_user_currency_request()?).await?.json()
}

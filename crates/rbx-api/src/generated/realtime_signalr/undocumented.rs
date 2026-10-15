//! `realtime-signalr.roblox.com` undocumented — generated from `specs/undocumented/realtime-signalr.roblox.com.yaml`.
//!
//! Do not edit. Change the spec or `rbx-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "realtime-signalr.roblox.com";

/// `GET /userhub`
///
/// WebSocket upgrade for the realtime hub. Use rbx_api::realtime, not this request directly.
///
/// Undocumented; captured 2026-10-13, every signed-in page on www.roblox.com.
pub fn get_userhub_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/userhub".to_string());
    Ok(r)
}

/// `GET /userhub`
///
/// WebSocket upgrade for the realtime hub. Use rbx_api::realtime, not this request directly.
///
/// Undocumented; captured 2026-10-13, every signed-in page on www.roblox.com.
pub async fn get_userhub(client: &Client) -> Result<()> {
    client.send(get_userhub_request()?).await?.unit()
}

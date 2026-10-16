//! `auth.roblox.com` undocumented — generated from `specs/undocumented/auth.roblox.com.yaml`.
//!
//! Do not edit. Change the spec or `bonsai-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "auth.roblox.com";

/// Query, header and form inputs for [`post_authentication_ticket`].
#[derive(Debug, Clone, Default)]
pub struct PostAuthenticationTicketParams {
    /// Must be https://www.roblox.com/
    pub referer: String,
    /// The site sends 1
    pub rbx_authentication_negotiation: Option<String>,
}

/// `POST /v1/authentication-ticket`
///
/// Mint a one-shot authentication ticket for the game client. The ticket is returned in the 'rbx-authentication-ticket' response header.
///
/// Undocumented; captured 2026-10-12, Play button on www.roblox.com/games/{placeId}.
pub fn post_authentication_ticket_request(params: &PostAuthenticationTicketParams) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/authentication-ticket".to_string());
    r.header("Referer", &params.referer);
    r.header(
        "RBXAuthenticationNegotiation",
        &params.rbx_authentication_negotiation,
    );
    Ok(r)
}

/// `POST /v1/authentication-ticket`
///
/// Mint a one-shot authentication ticket for the game client. The ticket is returned in the 'rbx-authentication-ticket' response header.
///
/// Undocumented; captured 2026-10-12, Play button on www.roblox.com/games/{placeId}.
pub async fn post_authentication_ticket(
    client: &Client,
    params: &PostAuthenticationTicketParams,
) -> Result<()> {
    client
        .send(post_authentication_ticket_request(params)?)
        .await?
        .unit()
}

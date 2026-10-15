//! `premiumfeatures.roblox.com` undocumented — generated from `specs/undocumented/premiumfeatures.roblox.com.yaml`.
//!
//! Do not edit. Change the spec or `rbx-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "premiumfeatures.roblox.com";

/// `GET /v1/users/{userId}/validate-membership`
///
/// Whether the user has Premium. The body is a bare boolean.
///
/// Undocumented; captured 2026-10-14, www.roblox.com/my/account#!/billing.
pub fn get_has_premium_request(user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/users/{}/validate-membership",
            crate::rt::path_value(&user_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/users/{userId}/validate-membership`
///
/// Whether the user has Premium. The body is a bare boolean.
///
/// Undocumented; captured 2026-10-14, www.roblox.com/my/account#!/billing.
pub async fn get_has_premium(client: &Client, user_id: i64) -> Result<bool> {
    client.send(get_has_premium_request(user_id)?).await?.json()
}

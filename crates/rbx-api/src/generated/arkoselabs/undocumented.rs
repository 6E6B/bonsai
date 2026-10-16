//! `arkoselabs.roblox.com` undocumented — generated from `specs/undocumented/arkoselabs.roblox.com.yaml`.
//!
//! Do not edit. Change the spec or `rbx-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "arkoselabs.roblox.com";

/// `GET /v2/{publicKey}/api.js`
///
/// Arkose captcha script for one public key
///
/// Undocumented; captured 2026-10-15, www.roblox.com Captcha.js.
pub fn captcha_script_request(public_key: &str) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v2/{}/api.js", crate::rt::path_value(&public_key)),
    );
    Ok(r)
}

/// `GET /v2/{publicKey}/api.js`
///
/// Arkose captcha script for one public key
///
/// Undocumented; captured 2026-10-15, www.roblox.com Captcha.js.
pub async fn captcha_script(client: &Client, public_key: &str) -> Result<()> {
    client.send(captcha_script_request(public_key)?).await?.unit()
}

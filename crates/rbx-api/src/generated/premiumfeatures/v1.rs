//! `premiumfeatures.roblox.com` v1 — generated from `vendor/roblox-api-tracker/premiumfeatures/v1.json`.
//!
//! Do not edit. Change the spec or `rbx-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "premiumfeatures.roblox.com";

/// Query, header and form inputs for [`get_users_by_user_id_premium_upsell_precheck`].
#[derive(Debug, Clone, Default)]
pub struct GetUsersByUserIdPremiumUpsellPrecheckParams {
    pub universe_id: i64,
    pub place_id: i64,
}

/// `GET /v1/users/{userId}/premium-upsell-precheck`
///
/// Premium upsell precheck
pub fn get_users_by_user_id_premium_upsell_precheck_request(
    user_id: i64,
    params: &GetUsersByUserIdPremiumUpsellPrecheckParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/users/{}/premium-upsell-precheck",
            crate::rt::path_value(&user_id)
        ),
    );
    r.query("universeId", &params.universe_id);
    r.query("placeId", &params.place_id);
    Ok(r)
}

/// `GET /v1/users/{userId}/premium-upsell-precheck`
///
/// Premium upsell precheck
pub async fn get_users_by_user_id_premium_upsell_precheck(
    client: &Client,
    user_id: i64,
    params: &GetUsersByUserIdPremiumUpsellPrecheckParams,
) -> Result<()> {
    client
        .send(get_users_by_user_id_premium_upsell_precheck_request(
            user_id, params,
        )?)
        .await?
        .unit()
}

/// `GET /v1/users/{userId}/validate-membership`
///
/// Get if a user has a Premium membership
pub fn get_users_by_user_id_validate_membership_request(user_id: i64) -> Result<Request> {
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
/// Get if a user has a Premium membership
pub async fn get_users_by_user_id_validate_membership(client: &Client, user_id: i64) -> Result<()> {
    client
        .send(get_users_by_user_id_validate_membership_request(user_id)?)
        .await?
        .unit()
}

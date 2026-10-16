//! `premiumfeatures.roblox.com` undocumented — generated from `specs/undocumented/premiumfeatures.roblox.com.yaml`.
//!
//! Do not edit. Change the spec or `bonsai-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "premiumfeatures.roblox.com";

/// `SubscriptionProduct`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SubscriptionProduct {
    /// ISO date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    /// ISO date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal: Option<String>,
    #[serde(rename = "robuxStipendAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robux_stipend_amount: Option<i64>,
    #[serde(rename = "subscriptionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_name: Option<String>,
}

/// `SubscriptionDetails`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SubscriptionDetails {
    #[serde(rename = "subscriptionProductModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_product_model: Option<SubscriptionProduct>,
}

/// `GET /v1/users/{userId}/subscriptions/details`
///
/// Premium subscription tier and renewal. 404 when there is no subscription.
///
/// Undocumented; captured 2026-10-16, www.roblox.com/my/account#!/billing.
pub fn get_subscription_details_request(user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/users/{}/subscriptions/details",
            crate::rt::path_value(&user_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/users/{userId}/subscriptions/details`
///
/// Premium subscription tier and renewal. 404 when there is no subscription.
///
/// Undocumented; captured 2026-10-16, www.roblox.com/my/account#!/billing.
pub async fn get_subscription_details(client: &Client, user_id: i64) -> Result<SubscriptionDetails> {
    client
        .send(get_subscription_details_request(user_id)?)
        .await?
        .json()
}

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

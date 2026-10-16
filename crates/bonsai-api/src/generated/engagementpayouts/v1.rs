//! `engagementpayouts.roblox.com` v1 — generated from `vendor/roblox-api-tracker/engagementpayouts/v1.json`.
//!
//! Do not edit. Change the spec or `bonsai-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "engagementpayouts.roblox.com";

/// `Roblox.EngagementPayouts.Api.PayoutResponseModel`
///
/// A model for payout responses
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PayoutResponseModel {
    /// Gets the eligibility type
    #[serde(rename = "eligibilityType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eligibility_type: Option<String>,
    /// Gets the engagement score
    #[serde(rename = "engagementScore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engagement_score: Option<f64>,
    /// Gets the payout in Robux
    #[serde(rename = "payoutInRobux")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_in_robux: Option<i64>,
    /// Gets the payout type
    #[serde(rename = "payoutType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_type: Option<String>,
}

/// Query, header and form inputs for [`get_universe_payout_history`].
#[derive(Debug, Clone, Default)]
pub struct GetUniversePayoutHistoryParams {
    /// The ID of the universe in question.
    pub universe_id: i64,
    /// The first date in the range, specified as yyyy-MM-dd.
    pub start_date: String,
    /// The last date in the range, specified as yyyy-MM-dd.
    pub end_date: String,
}

/// `GET /v1/universe-payout-history`
///
/// Gets the engagement payout history for a specific universe and a given date range, specified by start and end dates.
pub fn get_universe_payout_history_request(params: &GetUniversePayoutHistoryParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/universe-payout-history".to_string());
    r.query("universeId", &params.universe_id);
    r.query("startDate", &params.start_date);
    r.query("endDate", &params.end_date);
    Ok(r)
}

/// `GET /v1/universe-payout-history`
///
/// Gets the engagement payout history for a specific universe and a given date range, specified by start and end dates.
pub async fn get_universe_payout_history(
    client: &Client,
    params: &GetUniversePayoutHistoryParams,
) -> Result<HashMap<String, PayoutResponseModel>> {
    client
        .send(get_universe_payout_history_request(params)?)
        .await?
        .json()
}

//! `economycreatorstats.roblox.com` v1 — generated from `vendor/roblox-api-tracker/economycreatorstats/v1.json`.
//!
//! Do not edit. Change the spec or `rbx-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "economycreatorstats.roblox.com";

crate::api_enum! {
    int
    /// The Roblox.EconomyCreatorStats.Api.StatisticsDataGranularity. \['Hourly' = 0, 'Daily' = 1, 'Monthly' = 2\]
    StatisticsResponseDataGranularity {
    Hourly = 0 => "Hourly",
    Daily = 1 => "Daily",
    Monthly = 2 => "Monthly",
    }
}

/// `Roblox.EconomyCreatorStats.Api.Models.StatisticsResponse`
///
/// The response model
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct StatisticsResponse {
    /// The actual data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, Vec<Vec<f64>>>>,
    /// The Roblox.EconomyCreatorStats.Api.StatisticsDataGranularity. \['Hourly' = 0, 'Daily' = 1, 'Monthly' = 2\]
    #[serde(rename = "dataGranularity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_granularity: Option<StatisticsResponseDataGranularity>,
}

crate::api_enum! {
    str
    /// The type of a statistics dataset. \['PremiumUpsells' = 0, 'PremiumVisits' = 1\]
    GetUniversesByUniverseIdStatsType {
    PremiumUpsells = 0 => "PremiumUpsells",
    PremiumVisits = 1 => "PremiumVisits",
    }
}

/// Query, header and form inputs for [`get_universes_by_universe_id_stats`].
#[derive(Debug, Clone, Default)]
pub struct GetUniversesByUniverseIdStatsParams {
    /// The type of a statistics dataset. \['PremiumUpsells' = 0, 'PremiumVisits' = 1\]
    pub type_: GetUniversesByUniverseIdStatsType,
    pub start_time: String,
    pub end_time: String,
}

/// `GET /v1/universes/{universeId}/stats`
///
/// Get statistics data for a universe.
pub fn get_universes_by_universe_id_stats_request(
    universe_id: i64,
    params: &GetUniversesByUniverseIdStatsParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/universes/{}/stats", crate::rt::path_value(&universe_id)),
    );
    r.query("Type", &params.type_);
    r.query("StartTime", &params.start_time);
    r.query("EndTime", &params.end_time);
    Ok(r)
}

/// `GET /v1/universes/{universeId}/stats`
///
/// Get statistics data for a universe.
pub async fn get_universes_by_universe_id_stats(
    client: &Client,
    universe_id: i64,
    params: &GetUniversesByUniverseIdStatsParams,
) -> Result<StatisticsResponse> {
    client
        .send(get_universes_by_universe_id_stats_request(universe_id, params)?)
        .await?
        .json()
}

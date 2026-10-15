//! `trades.roblox.com` v1 — generated from `vendor/roblox-api-tracker/trades/v1.json`.
//!
//! Do not edit. Change the spec or `rbx-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "trades.roblox.com";

crate::api_enum! {
    int
    /// If you can't trade with a user, status explains why you can't trade with them. \['Unknown' = 0, 'CanTrade' = 1, 'CannotTradeWithSelf' = 2, 'SenderCannotTrade' = 3, 'ReceiverCannotTrade' = 4, 'SenderPrivacyTooStrict' = 5, 'UsersCannotTrade' = 6, 'TradeAccepterNeedsFriction' = 7, 'SenderAgeCheckRequired' = 8\]
    CanTradeResponseStatus {
    UnknownValue = 0 => "Unknown",
    CanTrade = 1 => "CanTrade",
    CannotTradeWithSelf = 2 => "CannotTradeWithSelf",
    SenderCannotTrade = 3 => "SenderCannotTrade",
    ReceiverCannotTrade = 4 => "ReceiverCannotTrade",
    SenderPrivacyTooStrict = 5 => "SenderPrivacyTooStrict",
    UsersCannotTrade = 6 => "UsersCannotTrade",
    TradeAccepterNeedsFriction = 7 => "TradeAccepterNeedsFriction",
    SenderAgeCheckRequired = 8 => "SenderAgeCheckRequired",
    }
}

/// `Roblox.Trades.Api.CanTradeResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CanTradeResponse {
    /// Returns true if you can trade with the given user.
    #[serde(rename = "canTrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_trade: Option<bool>,
    /// If you can't trade with a user, status explains why you can't trade with them. \['Unknown' = 0, 'CanTrade' = 1, 'CannotTradeWithSelf' = 2, 'SenderCannotTrade' = 3, 'ReceiverCannotTrade' = 4, 'SenderPrivacyTooStrict' = 5, 'UsersCannotTrade' = 6, 'TradeAccepterNeedsFriction' = 7, 'SenderAgeCheckRequired' = 8\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CanTradeResponseStatus>,
}

/// `Roblox.Trades.Api.NewTradeResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct NewTradeResponse {
    /// The trade id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
}

/// `Roblox.Trades.Api.TradeCountResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TradeCountResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

/// `Roblox.Web.Responses.Users.SkinnyUserResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SkinnyUserResponse {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

crate::api_enum! {
    int
    /// The minimum MembershipType required to own the userAsset. \['None' = 0, 'BC' = 1, 'TBC' = 2, 'OBC' = 3, 'RobloxPremium' = 4\]
    UserAssetResponseMembershipType {
    None = 0 => "None",
    Bc = 1 => "BC",
    Tbc = 2 => "TBC",
    Obc = 3 => "OBC",
    RobloxPremium = 4 => "RobloxPremium",
    }
}

/// `Roblox.Trades.Api.UserAssetResponse`
///
/// A model containing information about a UserAsset.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UserAssetResponse {
    /// The asset id of the user asset
    #[serde(rename = "assetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<i64>,
    /// The asset stock.
    #[serde(rename = "assetStock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_stock: Option<i64>,
    /// The user asset id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The minimum MembershipType required to own the userAsset. \['None' = 0, 'BC' = 1, 'TBC' = 2, 'OBC' = 3, 'RobloxPremium' = 4\]
    #[serde(rename = "membershipType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub membership_type: Option<UserAssetResponseMembershipType>,
    /// The asset name of the asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The original price of the asset
    #[serde(rename = "originalPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_price: Option<i64>,
    /// The recent average price of the asset
    #[serde(rename = "recentAveragePrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recent_average_price: Option<i64>,
    /// The serial number of the user asset
    #[serde(rename = "serialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<i64>,
}

/// `Roblox.Trades.Api.TradeOfferResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TradeOfferResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robux: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<SkinnyUserResponse>,
    #[serde(rename = "userAssets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_assets: Option<Vec<UserAssetResponse>>,
}

crate::api_enum! {
    str
    /// \['Unknown' = 1, 'Open' = 2, 'Pending' = 3, 'Completed' = 4, 'Expired' = 5, 'Declined' = 6, 'RejectedDueToError' = 7, 'Countered' = 8, 'Processing' = 9, 'InterventionRequired' = 10, 'TwoStepVerificationRequired' = 11\]
    TradeDetailResponseStatus {
    UnknownValue = 0 => "Unknown",
    Open = 1 => "Open",
    Pending = 2 => "Pending",
    Completed = 3 => "Completed",
    Expired = 4 => "Expired",
    Declined = 5 => "Declined",
    RejectedDueToError = 6 => "RejectedDueToError",
    Countered = 7 => "Countered",
    Processing = 8 => "Processing",
    InterventionRequired = 9 => "InterventionRequired",
    TwoStepVerificationRequired = 10 => "TwoStepVerificationRequired",
    }
}

/// `Roblox.Trades.Api.TradeDetailResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TradeDetailResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "isActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offers: Option<Vec<TradeOfferResponse>>,
    /// \['Unknown' = 1, 'Open' = 2, 'Pending' = 3, 'Completed' = 4, 'Expired' = 5, 'Declined' = 6, 'RejectedDueToError' = 7, 'Countered' = 8, 'Processing' = 9, 'InterventionRequired' = 10, 'TwoStepVerificationRequired' = 11\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TradeDetailResponseStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<SkinnyUserResponse>,
}

/// `Roblox.Trades.Api.TradeMetadata`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TradeMetadata {
    #[serde(rename = "maxItemsPerSide")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items_per_side: Option<i32>,
    #[serde(rename = "minValueRatio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value_ratio: Option<f64>,
    #[serde(rename = "tradeSystemMaxRobuxPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_system_max_robux_percent: Option<f64>,
    #[serde(rename = "tradeSystemRobuxFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_system_robux_fee: Option<f64>,
}

/// `Roblox.Trades.Api.TradeOfferRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TradeOfferRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robux: Option<i64>,
    #[serde(rename = "userAssetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_asset_ids: Option<Vec<i64>>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}

/// `Roblox.Trades.Api.TradeRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TradeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offers: Option<Vec<TradeOfferRequest>>,
}

/// `Roblox.Trades.Api.TradeResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TradeResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "isActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// \['Unknown' = 1, 'Open' = 2, 'Pending' = 3, 'Completed' = 4, 'Expired' = 5, 'Declined' = 6, 'RejectedDueToError' = 7, 'Countered' = 8, 'Processing' = 9, 'InterventionRequired' = 10, 'TwoStepVerificationRequired' = 11\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TradeDetailResponseStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<SkinnyUserResponse>,
}

/// `Roblox.Web.WebAPI.ApiEmptyResponseModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiEmptyResponseModel {}

/// `Roblox.Web.WebAPI.Models.ApiPageResponse\[Roblox.Trades.Api.TradeResponse\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiPageResponseTradeResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<TradeResponse>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
}

crate::api_enum! {
    str
    /// Sorted by trade creation date
    GetTradesByTradeStatusTypeSortOrder {
    Asc = 0 => "Asc",
    Desc = 1 => "Desc",
    }
}

/// Query, header and form inputs for [`get_trades_by_trade_status_type`].
#[derive(Debug, Clone, Default)]
pub struct GetTradesByTradeStatusTypeParams {
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// Sorted by trade creation date
    pub sort_order: Option<GetTradesByTradeStatusTypeSortOrder>,
}

/// `POST /v1/trades/expire-outdated`
///
/// Deprecated. TradeSession are automatically set to expire while the inbound/outbound trades are fetched. Expires Outdated Inbound Trades for User
pub fn post_trades_expire_outdated_request() -> Result<Request> {
    let r = Request::new(Method::POST, HOST, "/v1/trades/expire-outdated".to_string());
    Ok(r)
}

/// `POST /v1/trades/expire-outdated`
///
/// Deprecated. TradeSession are automatically set to expire while the inbound/outbound trades are fetched. Expires Outdated Inbound Trades for User
pub async fn post_trades_expire_outdated(client: &Client) -> Result<ApiEmptyResponseModel> {
    client.send(post_trades_expire_outdated_request()?).await?.json()
}

/// `GET /v1/trades/metadata`
///
/// Gets metadata about the trade system.
pub fn get_trades_metadata_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/trades/metadata".to_string());
    Ok(r)
}

/// `GET /v1/trades/metadata`
///
/// Gets metadata about the trade system.
pub async fn get_trades_metadata(client: &Client) -> Result<TradeMetadata> {
    client.send(get_trades_metadata_request()?).await?.json()
}

/// `POST /v1/trades/send`
///
/// Sends a trade.
pub fn post_trades_send_request(body: &TradeRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/trades/send".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/trades/send`
///
/// Sends a trade.
pub async fn post_trades_send(client: &Client, body: &TradeRequest) -> Result<NewTradeResponse> {
    client.send(post_trades_send_request(body)?).await?.json()
}

/// `GET /v1/trades/{tradeId}`
///
/// Gets detailed information about a trade.
pub fn get_trades_by_trade_id_request(trade_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/trades/{}", crate::rt::path_value(&trade_id)),
    );
    Ok(r)
}

/// `GET /v1/trades/{tradeId}`
///
/// Gets detailed information about a trade.
pub async fn get_trades_by_trade_id(client: &Client, trade_id: i64) -> Result<TradeDetailResponse> {
    client
        .send(get_trades_by_trade_id_request(trade_id)?)
        .await?
        .json()
}

/// `POST /v1/trades/{tradeId}/accept`
///
/// Accepts a trade.
pub fn post_trades_by_trade_id_accept_request(trade_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::POST,
        HOST,
        format!("/v1/trades/{}/accept", crate::rt::path_value(&trade_id)),
    );
    Ok(r)
}

/// `POST /v1/trades/{tradeId}/accept`
///
/// Accepts a trade.
pub async fn post_trades_by_trade_id_accept(client: &Client, trade_id: i64) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_trades_by_trade_id_accept_request(trade_id)?)
        .await?
        .json()
}

/// `POST /v1/trades/{tradeId}/counter`
///
/// Counters a trade.
pub fn post_trades_by_trade_id_counter_request(trade_id: i64, body: &TradeRequest) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!("/v1/trades/{}/counter", crate::rt::path_value(&trade_id)),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/trades/{tradeId}/counter`
///
/// Counters a trade.
pub async fn post_trades_by_trade_id_counter(
    client: &Client,
    trade_id: i64,
    body: &TradeRequest,
) -> Result<NewTradeResponse> {
    client
        .send(post_trades_by_trade_id_counter_request(trade_id, body)?)
        .await?
        .json()
}

/// `POST /v1/trades/{tradeId}/decline`
///
/// Declines a trade.
pub fn post_trades_by_trade_id_decline_request(trade_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::POST,
        HOST,
        format!("/v1/trades/{}/decline", crate::rt::path_value(&trade_id)),
    );
    Ok(r)
}

/// `POST /v1/trades/{tradeId}/decline`
///
/// Declines a trade.
pub async fn post_trades_by_trade_id_decline(
    client: &Client,
    trade_id: i64,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_trades_by_trade_id_decline_request(trade_id)?)
        .await?
        .json()
}

/// `GET /v1/trades/{tradeStatusType}`
///
/// Fetches a list of the authenticated user's trades.
pub fn get_trades_by_trade_status_type_request(
    trade_status_type: &str,
    params: &GetTradesByTradeStatusTypeParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/trades/{}", crate::rt::path_value(&trade_status_type)),
    );
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    r.query("sortOrder", &params.sort_order);
    Ok(r)
}

/// `GET /v1/trades/{tradeStatusType}`
///
/// Fetches a list of the authenticated user's trades.
pub async fn get_trades_by_trade_status_type(
    client: &Client,
    trade_status_type: &str,
    params: &GetTradesByTradeStatusTypeParams,
) -> Result<ApiPageResponseTradeResponse> {
    client
        .send(get_trades_by_trade_status_type_request(
            trade_status_type,
            params,
        )?)
        .await?
        .json()
}

/// `GET /v1/trades/{tradeStatusType}/count`
///
/// Gets the total number of pending trades for the authenticated user. Inbound is the only accepted tradeStatusType.
pub fn get_trades_by_trade_status_type_count_request(trade_status_type: i32) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/trades/{}/count", crate::rt::path_value(&trade_status_type)),
    );
    Ok(r)
}

/// `GET /v1/trades/{tradeStatusType}/count`
///
/// Gets the total number of pending trades for the authenticated user. Inbound is the only accepted tradeStatusType.
pub async fn get_trades_by_trade_status_type_count(
    client: &Client,
    trade_status_type: i32,
) -> Result<TradeCountResponse> {
    client
        .send(get_trades_by_trade_status_type_count_request(trade_status_type)?)
        .await?
        .json()
}

/// `GET /v1/users/{userId}/can-trade-with`
///
/// Returns whether you can trade with another user.
pub fn get_users_by_user_id_can_trade_with_request(user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/users/{}/can-trade-with", crate::rt::path_value(&user_id)),
    );
    Ok(r)
}

/// `GET /v1/users/{userId}/can-trade-with`
///
/// Returns whether you can trade with another user.
pub async fn get_users_by_user_id_can_trade_with(client: &Client, user_id: i64) -> Result<CanTradeResponse> {
    client
        .send(get_users_by_user_id_can_trade_with_request(user_id)?)
        .await?
        .json()
}

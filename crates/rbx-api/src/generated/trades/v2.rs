//! `trades.roblox.com` v2 — generated from `vendor/roblox-api-tracker/trades/v2.json`.
//!
//! Do not edit. Change the spec or `rbx-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "trades.roblox.com";

/// `Roblox.Trades.Api.Models.V2.CurrencyTransferEligibilityResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CurrencyTransferEligibilityResponse {
    #[serde(rename = "canRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_request: Option<bool>,
    #[serde(rename = "canSend")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send: Option<bool>,
}

crate::api_enum! {
    str
    /// \['Day' = 1, 'Week' = 2, 'Month' = 3, 'Year' = 4, 'Lifetime' = 5\]
    FreeTradesAllowanceResponseWindow {
    Day = 0 => "day",
    Week = 1 => "week",
    Month = 2 => "month",
    Year = 3 => "year",
    Lifetime = 4 => "lifetime",
    }
}

/// `Roblox.Trades.Api.Models.V2.FreeTradesAllowanceResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FreeTradesAllowanceResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining: Option<i32>,
    /// \['Day' = 1, 'Week' = 2, 'Month' = 3, 'Year' = 4, 'Lifetime' = 5\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<FreeTradesAllowanceResponseWindow>,
}

crate::api_enum! {
    str
    /// The trade eligibility status of the user. \['Unknown' = 0, 'Eligible' = 1, 'IneligibleTradeSystemDisabled' = 2, 'IneligibleCannotTradeWithRoblox' = 3, 'IneligibleUserNotFound' = 4, 'IneligibleMissingPremiumMembership' = 5, 'IneligibleLegalOrRegulatoryRestrictions' = 6, 'IneligibleFreeTradesLimitReached' = 7, 'IneligibleAgeCheckRequired' = 8\]
    CanTradeResponseTradeEligibility {
    UnknownValue = 0 => "Unknown",
    Eligible = 1 => "Eligible",
    IneligibleTradeSystemDisabled = 2 => "IneligibleTradeSystemDisabled",
    IneligibleCannotTradeWithRoblox = 3 => "IneligibleCannotTradeWithRoblox",
    IneligibleUserNotFound = 4 => "IneligibleUserNotFound",
    IneligibleMissingPremiumMembership = 5 => "IneligibleMissingPremiumMembership",
    IneligibleLegalOrRegulatoryRestrictions = 6 => "IneligibleLegalOrRegulatoryRestrictions",
    IneligibleFreeTradesLimitReached = 7 => "IneligibleFreeTradesLimitReached",
    IneligibleAgeCheckRequired = 8 => "IneligibleAgeCheckRequired",
    }
}

/// `Roblox.Trades.Api.Models.V2.CanTradeResponse`
///
/// The response for the CanTrade endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CanTradeResponse {
    /// Whether the user can trade or not.
    #[serde(rename = "canTrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_trade: Option<bool>,
    #[serde(rename = "currencyTransferEligibility")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_transfer_eligibility: Option<CurrencyTransferEligibilityResponse>,
    #[serde(rename = "freeTradesAllowance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_trades_allowance: Option<FreeTradesAllowanceResponse>,
    /// The trade eligibility status of the user. \['Unknown' = 0, 'Eligible' = 1, 'IneligibleTradeSystemDisabled' = 2, 'IneligibleCannotTradeWithRoblox' = 3, 'IneligibleUserNotFound' = 4, 'IneligibleMissingPremiumMembership' = 5, 'IneligibleLegalOrRegulatoryRestrictions' = 6, 'IneligibleFreeTradesLimitReached' = 7, 'IneligibleAgeCheckRequired' = 8\]
    #[serde(rename = "tradeEligibility")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_eligibility: Option<CanTradeResponseTradeEligibility>,
    /// The ID of the user.
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}

crate::api_enum! {
    str
    /// The mutual trade eligibility status between the two users. \['Unknown' = 0, 'Eligible' = 1, 'CallingUserIneligible' = 2, 'TargetUserIneligible' = 3, 'CannotTradeWithSelf' = 4, 'CallingUserPrivacySettingsRestricted' = 5, 'CallingUserAgeCheckRequired' = 6\]
    CanTradeWithResponseMutualTradeEligibility {
    UnknownValue = 0 => "Unknown",
    Eligible = 1 => "Eligible",
    CallingUserIneligible = 2 => "CallingUserIneligible",
    TargetUserIneligible = 3 => "TargetUserIneligible",
    CannotTradeWithSelf = 4 => "CannotTradeWithSelf",
    CallingUserPrivacySettingsRestricted = 5 => "CallingUserPrivacySettingsRestricted",
    CallingUserAgeCheckRequired = 6 => "CallingUserAgeCheckRequired",
    }
}

/// `Roblox.Trades.Api.Models.V2.CanTradeWithResponse`
///
/// The response for the CanTradeWith endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CanTradeWithResponse {
    /// Whether the user can trade with the target user or not.
    #[serde(rename = "canTrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_trade: Option<bool>,
    #[serde(rename = "currencyTransferEligibility")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_transfer_eligibility: Option<CurrencyTransferEligibilityResponse>,
    /// The mutual trade eligibility status between the two users. \['Unknown' = 0, 'Eligible' = 1, 'CallingUserIneligible' = 2, 'TargetUserIneligible' = 3, 'CannotTradeWithSelf' = 4, 'CallingUserPrivacySettingsRestricted' = 5, 'CallingUserAgeCheckRequired' = 6\]
    #[serde(rename = "mutualTradeEligibility")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutual_trade_eligibility: Option<CanTradeWithResponseMutualTradeEligibility>,
    /// The ID of the target user.
    #[serde(rename = "targetUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_user_id: Option<i64>,
    /// The ID of the user.
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}

crate::api_enum! {
    str
    /// The type of the underlying. \['Unknown' = 0, 'Asset' = 1, 'Bundle' = 2\]
    ItemTargetItemType {
    UnknownValue = 0 => "Unknown",
    Asset = 1 => "Asset",
    Bundle = 2 => "Bundle",
    }
}

/// `Roblox.Trades.Api.Models.V2.ItemTarget`
///
/// The underlying of a tradable item.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ItemTarget {
    /// The type of the underlying. \['Unknown' = 0, 'Asset' = 1, 'Bundle' = 2\]
    #[serde(rename = "itemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_type: Option<ItemTargetItemType>,
    /// The id of the underlying.
    #[serde(rename = "targetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
}

/// `Roblox.Trades.Api.Models.V2.TradableItemInstance`
///
/// A tradable item instance.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TradableItemInstance {
    /// Total quantity of the item.
    #[serde(rename = "assetStock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_stock: Option<i64>,
    /// The collectible item instance id.
    #[serde(rename = "collectibleItemInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collectible_item_instance_id: Option<String>,
    /// Whether the item is on hold.
    #[serde(rename = "isOnHold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_on_hold: Option<bool>,
    /// The name of the item.
    #[serde(rename = "itemName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_name: Option<String>,
    #[serde(rename = "itemTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_target: Option<ItemTarget>,
    /// The original price of the item.
    #[serde(rename = "originalPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_price: Option<i64>,
    /// The recent average price of the item.
    #[serde(rename = "recentAveragePrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recent_average_price: Option<i64>,
    /// The serial number of the item if it is LimitedUnique.
    #[serde(rename = "serialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<i64>,
}

/// `Roblox.Trades.Api.Models.V2.TradableItem`
///
/// A tradable item.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TradableItem {
    /// Total quantity of the item.
    #[serde(rename = "assetStock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_stock: Option<i64>,
    /// The collectible item id.
    #[serde(rename = "collectibleItemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collectible_item_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<TradableItemInstance>>,
    /// The name of the item.
    #[serde(rename = "itemName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_name: Option<String>,
    #[serde(rename = "itemTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_target: Option<ItemTarget>,
    /// The original price of the item.
    #[serde(rename = "originalPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_price: Option<i64>,
    /// The recent average price of the item.
    #[serde(rename = "recentAveragePrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recent_average_price: Option<i64>,
}

/// `Roblox.Trades.Api.Models.V2.GetUserTradableItemsResponse`
///
/// The response for the GetUserTradableItems endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetUserTradableItemsResponse {
    /// The items that the user can trade.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<TradableItem>>,
    /// The cursor for the next page of items.
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    /// The ID of the user.
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}

/// `Roblox.Trades.Api.Models.V2.NewTradeResponse`
///
/// Represents a newly created trade.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct NewTradeResponse {
    /// The ID of the trade.
    #[serde(rename = "tradeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<i64>,
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

/// `Roblox.Trades.Api.Models.V2.TradeOffer`
///
/// Represents a trade offer.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TradeOffer {
    /// The items in the trade offer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<TradableItemInstance>>,
    /// The amount of Robux in the trade offer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robux: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<SkinnyUserResponse>,
}

crate::api_enum! {
    str
    /// The status of the trade. \['Unknown' = 1, 'Open' = 2, 'Pending' = 3, 'Completed' = 4, 'Expired' = 5, 'Declined' = 6, 'RejectedDueToError' = 7, 'Countered' = 8, 'Processing' = 9, 'InterventionRequired' = 10, 'TwoStepVerificationRequired' = 11\]
    TradeDetailsResponseStatus {
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

/// `Roblox.Trades.Api.Models.V2.TradeDetailsResponse`
///
/// The response for the TradeDetailsV2 endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TradeDetailsResponse {
    #[serde(rename = "participantAOffer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_a_offer: Option<TradeOffer>,
    #[serde(rename = "participantBOffer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_b_offer: Option<TradeOffer>,
    /// The status of the trade. \['Unknown' = 1, 'Open' = 2, 'Pending' = 3, 'Completed' = 4, 'Expired' = 5, 'Declined' = 6, 'RejectedDueToError' = 7, 'Countered' = 8, 'Processing' = 9, 'InterventionRequired' = 10, 'TwoStepVerificationRequired' = 11\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TradeDetailsResponseStatus>,
    /// The ID of the trade.
    #[serde(rename = "tradeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<i64>,
}

/// `Roblox.Trades.Api.Models.V2.TradeOfferRequest`
///
/// Represents a trade offer.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TradeOfferRequest {
    /// List of items in the trade offer.
    #[serde(rename = "collectibleItemInstanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collectible_item_instance_ids: Option<Vec<String>>,
    /// The amount of Robux in the trade offer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robux: Option<i64>,
    /// The user ID of the offer.
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}

/// `Roblox.Trades.Api.Models.V2.TradeRequest`
///
/// Represents a trade request. The calling user must be either participant A or B in the trade.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TradeRequest {
    #[serde(rename = "recipientOffer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_offer: Option<TradeOfferRequest>,
    #[serde(rename = "senderOffer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_offer: Option<TradeOfferRequest>,
}

crate::api_enum! {
    str
    /// The type of the underlying of a tradable item. \['Unknown' = 0, 'HatAccessory' = 1, 'HairAccessory' = 2, 'FaceAccessory' = 3, 'NeckAccessory' = 4, 'ShoulderAccessory' = 5, 'FrontAccessory' = 6, 'BackAccessory' = 7, 'WaistAccessory' = 8, 'Gear' = 9, 'Face' = 10, 'JacketAccessory' = 11, 'SweaterAccessory' = 12, 'DressSkirtAccessory' = 13, 'Character' = 14, 'DynamicHead' = 15, 'Shoes' = 16, 'Animation' = 17\]
    GetUsersByUserIdTradableItemsItemTargetTypes {
    UnknownValue = 0 => "Unknown",
    HatAccessory = 1 => "HatAccessory",
    HairAccessory = 2 => "HairAccessory",
    FaceAccessory = 3 => "FaceAccessory",
    NeckAccessory = 4 => "NeckAccessory",
    ShoulderAccessory = 5 => "ShoulderAccessory",
    FrontAccessory = 6 => "FrontAccessory",
    BackAccessory = 7 => "BackAccessory",
    WaistAccessory = 8 => "WaistAccessory",
    Gear = 9 => "Gear",
    Face = 10 => "Face",
    JacketAccessory = 11 => "JacketAccessory",
    SweaterAccessory = 12 => "SweaterAccessory",
    DressSkirtAccessory = 13 => "DressSkirtAccessory",
    Character = 14 => "Character",
    DynamicHead = 15 => "DynamicHead",
    Shoes = 16 => "Shoes",
    Animation = 17 => "Animation",
    }
}

crate::api_enum! {
    str
    /// The key to sort tradable items by.
    GetUsersByUserIdTradableItemsSortBy {
    UnknownValue = 0 => "Unknown",
    CreationTime = 1 => "CreationTime",
    AcquisitionTime = 2 => "AcquisitionTime",
    }
}

/// Query, header and form inputs for [`get_users_by_user_id_tradable_items`].
#[derive(Debug, Clone, Default)]
pub struct GetUsersByUserIdTradableItemsParams {
    /// Optional search query to filter items by.
    pub search: Option<String>,
    /// Optional list of item target types to filter by.
    pub item_target_types: Option<Vec<GetUsersByUserIdTradableItemsItemTargetTypes>>,
    /// The key to sort tradable items by.
    pub sort_by: Option<GetUsersByUserIdTradableItemsSortBy>,
    /// The sort order for the tradable items.
    pub sort_order: Option<i32>,
    /// The maximum number of items to return.
    pub limit: Option<i32>,
    /// The pagination cursor.
    pub cursor: Option<String>,
}

/// `POST /v2/trades/send`
///
/// Sends a new trade.
pub fn post_trades_send_request(body: &TradeRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v2/trades/send".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/trades/send`
///
/// Sends a new trade.
pub async fn post_trades_send(client: &Client, body: &TradeRequest) -> Result<NewTradeResponse> {
    client.send(post_trades_send_request(body)?).await?.json()
}

/// `GET /v2/trades/{tradeId}`
///
/// Gets the details of a trade.
pub fn get_trades_by_trade_id_request(trade_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v2/trades/{}", crate::rt::path_value(&trade_id)),
    );
    Ok(r)
}

/// `GET /v2/trades/{tradeId}`
///
/// Gets the details of a trade.
pub async fn get_trades_by_trade_id(client: &Client, trade_id: i64) -> Result<TradeDetailsResponse> {
    client
        .send(get_trades_by_trade_id_request(trade_id)?)
        .await?
        .json()
}

/// `POST /v2/trades/{tradeId}/counter`
///
/// Counters an existing trade.
pub fn post_trades_by_trade_id_counter_request(trade_id: i64, body: &TradeRequest) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!("/v2/trades/{}/counter", crate::rt::path_value(&trade_id)),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/trades/{tradeId}/counter`
///
/// Counters an existing trade.
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

/// `GET /v2/users/me/can-trade`
///
/// Checks if the calling user can trade with others.
pub fn get_users_me_can_trade_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v2/users/me/can-trade".to_string());
    Ok(r)
}

/// `GET /v2/users/me/can-trade`
///
/// Checks if the calling user can trade with others.
pub async fn get_users_me_can_trade(client: &Client) -> Result<CanTradeResponse> {
    client.send(get_users_me_can_trade_request()?).await?.json()
}

/// `GET /v2/users/{userId}/can-trade-with`
///
/// Checks if the user can trade with a specific user.
pub fn get_users_by_user_id_can_trade_with_request(user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v2/users/{}/can-trade-with", crate::rt::path_value(&user_id)),
    );
    Ok(r)
}

/// `GET /v2/users/{userId}/can-trade-with`
///
/// Checks if the user can trade with a specific user.
pub async fn get_users_by_user_id_can_trade_with(
    client: &Client,
    user_id: i64,
) -> Result<CanTradeWithResponse> {
    client
        .send(get_users_by_user_id_can_trade_with_request(user_id)?)
        .await?
        .json()
}

/// `GET /v2/users/{userId}/tradableItems`
///
/// Gets tradable items for a user.
pub fn get_users_by_user_id_tradable_items_request(
    user_id: i64,
    params: &GetUsersByUserIdTradableItemsParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v2/users/{}/tradableItems", crate::rt::path_value(&user_id)),
    );
    r.query("search", &params.search);
    r.query("itemTargetTypes", &params.item_target_types);
    r.query("sortBy", &params.sort_by);
    r.query("sortOrder", &params.sort_order);
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    Ok(r)
}

/// `GET /v2/users/{userId}/tradableItems`
///
/// Gets tradable items for a user.
pub async fn get_users_by_user_id_tradable_items(
    client: &Client,
    user_id: i64,
    params: &GetUsersByUserIdTradableItemsParams,
) -> Result<GetUserTradableItemsResponse> {
    client
        .send(get_users_by_user_id_tradable_items_request(user_id, params)?)
        .await?
        .json()
}

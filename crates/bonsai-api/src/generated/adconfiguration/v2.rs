//! `adconfiguration.roblox.com` v2 — generated from `vendor/roblox-api-tracker/adconfiguration/v2.json`.
//!
//! Do not edit. Change the spec or `bonsai-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "adconfiguration.roblox.com";

crate::api_enum! {
    int
    /// Targeting age bracket(s) of the ad set \['Undefined' = 1, 'AgeUnder13' = 2, 'Age13OrOver' = 4, 'Age13To16' = 8, 'Age17OrOver' = 16\]
    CreateSponsoredGameV2RequestTargetAgeBracket {
    Undefined = 1 => "Undefined",
    AgeUnder13 = 2 => "AgeUnder13",
    Age13OrOver = 4 => "Age13OrOver",
    Age13To16 = 8 => "Age13To16",
    Age17OrOver = 16 => "Age17OrOver",
    }
}

crate::api_enum! {
    int
    /// Targeting device type(s) of the ad set \['Undefined' = 1, 'Computer' = 2, 'Phone' = 4, 'Tablet' = 8, 'Console' = 16, 'VR' = 32\]
    CreateSponsoredGameV2RequestTargetDeviceType {
    Undefined = 1 => "Undefined",
    Computer = 2 => "Computer",
    Phone = 4 => "Phone",
    Tablet = 8 => "Tablet",
    Console = 16 => "Console",
    Vr = 32 => "VR",
    }
}

crate::api_enum! {
    int
    /// Targeting gender(s) of the ad set \['Undefined' = 1, 'Male' = 2, 'Female' = 4\]
    CreateSponsoredGameV2RequestTargetGender {
    Undefined = 1 => "Undefined",
    Male = 2 => "Male",
    Female = 4 => "Female",
    }
}

/// `Roblox.AdConfiguration.Api.CreateSponsoredGameV2Request`
///
/// A request model for creating a sponsored game
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CreateSponsoredGameV2Request {
    /// The name of the Ad
    #[serde(rename = "adName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_name: Option<String>,
    /// The bid amount of the Ad in Robux
    #[serde(rename = "bidAmountInRobux")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_amount_in_robux: Option<i64>,
    /// The budget in Robux
    #[serde(rename = "budgetInRobux")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_in_robux: Option<i64>,
    /// The end date of the ad set
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// The start date of the ad set
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Targeting age bracket(s) of the ad set \['Undefined' = 1, 'AgeUnder13' = 2, 'Age13OrOver' = 4, 'Age13To16' = 8, 'Age17OrOver' = 16\]
    #[serde(rename = "targetAgeBracket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_age_bracket: Option<CreateSponsoredGameV2RequestTargetAgeBracket>,
    /// Targeting device type(s) of the ad set \['Undefined' = 1, 'Computer' = 2, 'Phone' = 4, 'Tablet' = 8, 'Console' = 16, 'VR' = 32\]
    #[serde(rename = "targetDeviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_device_type: Option<CreateSponsoredGameV2RequestTargetDeviceType>,
    /// Targeting gender(s) of the ad set \['Undefined' = 1, 'Male' = 2, 'Female' = 4\]
    #[serde(rename = "targetGender")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_gender: Option<CreateSponsoredGameV2RequestTargetGender>,
    /// The target universe id
    #[serde(rename = "universeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_id: Option<i64>,
}

crate::api_enum! {
    int
    /// The type of the ad creative. Typically, CreativeType.Image. \['Undefined' = 0, 'Image' = 1, 'Video' = 2\]
    CreativeModelCreativeType {
    Undefined = 0 => "Undefined",
    Image = 1 => "Image",
    Video = 2 => "Video",
    }
}

/// `Roblox.AdConfiguration.Api.CreativeModel`
///
/// A model representing an Ad Creative (for example, an ad thumbnail).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CreativeModel {
    /// The ID of the creative. Typically, a thumbnail's imageId.
    #[serde(rename = "creativeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creative_id: Option<i64>,
    /// The type of the ad creative. Typically, CreativeType.Image. \['Undefined' = 0, 'Image' = 1, 'Video' = 2\]
    #[serde(rename = "creativeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creative_type: Option<CreativeModelCreativeType>,
}

crate::api_enum! {
    int
    /// The status of the ad set \['Undefined' = 0, 'Draft' = 1, 'Scheduled' = 2, 'Running' = 3, 'Paused' = 4, 'Stopped' = 5, 'Completed' = 6\]
    SponsoredCampaignModelAdSetStatus {
    Undefined = 0 => "Undefined",
    Draft = 1 => "Draft",
    Scheduled = 2 => "Scheduled",
    Running = 3 => "Running",
    Paused = 4 => "Paused",
    Stopped = 5 => "Stopped",
    Completed = 6 => "Completed",
    }
}

crate::api_enum! {
    int
    /// The status of the ad \['Undefined' = 0, 'Draft' = 1, 'PendingApproval' = 2, 'Approved' = 3, 'Moderated' = 4\]
    SponsoredCampaignModelAdStatus {
    Undefined = 0 => "Undefined",
    Draft = 1 => "Draft",
    PendingApproval = 2 => "PendingApproval",
    Approved = 3 => "Approved",
    Moderated = 4 => "Moderated",
    }
}

crate::api_enum! {
    int
    /// The target type of the campaign \['Undefined' = 0, 'Universe' = 1, 'Asset' = 2, 'ImmersiveAd' = 3\]
    SponsoredCampaignModelCampaignTargetType {
    Undefined = 0 => "Undefined",
    Universe = 1 => "Universe",
    Asset = 2 => "Asset",
    ImmersiveAd = 3 => "ImmersiveAd",
    }
}

/// `Roblox.AdConfiguration.Api.SponsoredCampaignModel`
///
/// A response model represents an Ad with complete information, including infos from its associated campaign and ad set
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SponsoredCampaignModel {
    /// The id of the ad
    #[serde(rename = "adId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_id: Option<i64>,
    /// The name of the Ad
    #[serde(rename = "adName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_name: Option<String>,
    /// The id of the ad set
    #[serde(rename = "adSetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_set_id: Option<i64>,
    /// The status of the ad set \['Undefined' = 0, 'Draft' = 1, 'Scheduled' = 2, 'Running' = 3, 'Paused' = 4, 'Stopped' = 5, 'Completed' = 6\]
    #[serde(rename = "adSetStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_set_status: Option<SponsoredCampaignModelAdSetStatus>,
    /// The status of the ad \['Undefined' = 0, 'Draft' = 1, 'PendingApproval' = 2, 'Approved' = 3, 'Moderated' = 4\]
    #[serde(rename = "adStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_status: Option<SponsoredCampaignModelAdStatus>,
    /// The bid amount of the Ad in Robux
    #[serde(rename = "bidAmountInRobux")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_amount_in_robux: Option<i64>,
    /// The budget of the ad set in Robux
    #[serde(rename = "budgetInRobux")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_in_robux: Option<i64>,
    /// The target id of the campaign
    #[serde(rename = "campaignTargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_target_id: Option<i64>,
    /// The target type of the campaign \['Undefined' = 0, 'Universe' = 1, 'Asset' = 2, 'ImmersiveAd' = 3\]
    #[serde(rename = "campaignTargetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_target_type: Option<SponsoredCampaignModelCampaignTargetType>,
    /// The number of conversions by ad click generated by the ad
    #[serde(rename = "clickConversions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_conversions: Option<i64>,
    /// The target id of the creative associated with the Ad
    #[serde(rename = "creativeTargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creative_target_id: Option<i64>,
    /// The type of the creative associated with the Ad \['Undefined' = 0, 'Image' = 1, 'Video' = 2\]
    #[serde(rename = "creativeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creative_type: Option<CreativeModelCreativeType>,
    /// The end date of the ad set
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// The number of conversions by ad impression generated by the ad
    #[serde(rename = "impressionConversions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impression_conversions: Option<i64>,
    /// The start date of the ad set
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// The target age bracket of the ad set \['Undefined' = 1, 'AgeUnder13' = 2, 'Age13OrOver' = 4, 'Age13To16' = 8, 'Age17OrOver' = 16\]
    #[serde(rename = "targetAgeBracket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_age_bracket: Option<CreateSponsoredGameV2RequestTargetAgeBracket>,
    /// The target device type of the ad set \['Undefined' = 1, 'Computer' = 2, 'Phone' = 4, 'Tablet' = 8, 'Console' = 16, 'VR' = 32\]
    #[serde(rename = "targetDeviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_device_type: Option<CreateSponsoredGameV2RequestTargetDeviceType>,
    /// The target gender of the ad set \['Undefined' = 1, 'Male' = 2, 'Female' = 4\]
    #[serde(rename = "targetGender")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_gender: Option<CreateSponsoredGameV2RequestTargetGender>,
    /// The total number of clicks generated by the ad
    #[serde(rename = "totalClicks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_clicks: Option<i64>,
    /// The total number of conversions generated by the ad
    #[serde(rename = "totalConversions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_conversions: Option<i64>,
    /// The total number of impressions generated by the ad
    #[serde(rename = "totalImpressions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_impressions: Option<i64>,
    /// The total spend of the ad set in Robux
    #[serde(rename = "totalSpendInRobux")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_spend_in_robux: Option<i64>,
}

/// `Roblox.AdConfiguration.Api.GetSponsoredCampaignsResponse`
///
/// A response model for retrieving a page of Roblox.AdConfiguration.Api.SponsoredCampaignModel.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetSponsoredCampaignsResponse {
    /// The cursor for retrieving the next page, if present.
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    /// The cursor for retrieving the previous page, if present.
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
    /// A collection of Roblox.AdConfiguration.Api.SponsoredCampaignModel.
    #[serde(rename = "sponsoredCampaigns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sponsored_campaigns: Option<Vec<SponsoredCampaignModel>>,
}

/// `Roblox.AdConfiguration.Api.SponsoredGameV2Model`
///
/// A response model represents an Ad with complete information, including infos from its associated campaign and ad set
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SponsoredGameV2Model {
    /// The id of the ad
    #[serde(rename = "adId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_id: Option<i64>,
    /// The name of the Ad
    #[serde(rename = "adName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_name: Option<String>,
    /// The id of the ad set
    #[serde(rename = "adSetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_set_id: Option<i64>,
    /// The status of the ad set \['Undefined' = 0, 'Draft' = 1, 'Scheduled' = 2, 'Running' = 3, 'Paused' = 4, 'Stopped' = 5, 'Completed' = 6\]
    #[serde(rename = "adSetStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_set_status: Option<SponsoredCampaignModelAdSetStatus>,
    /// The status of the ad \['Undefined' = 0, 'Draft' = 1, 'PendingApproval' = 2, 'Approved' = 3, 'Moderated' = 4\]
    #[serde(rename = "adStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_status: Option<SponsoredCampaignModelAdStatus>,
    /// The bid amount of the Ad in Robux
    #[serde(rename = "bidAmountInRobux")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_amount_in_robux: Option<i64>,
    /// The budget of the ad set in Robux
    #[serde(rename = "budgetInRobux")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_in_robux: Option<i64>,
    /// The target id of the campaign
    #[serde(rename = "campaignTargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_target_id: Option<i64>,
    /// The target type of the campaign \['Undefined' = 0, 'Universe' = 1, 'Asset' = 2, 'ImmersiveAd' = 3\]
    #[serde(rename = "campaignTargetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_target_type: Option<SponsoredCampaignModelCampaignTargetType>,
    /// The number of conversions by ad click generated by the ad
    #[serde(rename = "clickConversions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_conversions: Option<i64>,
    /// The target id of the creative associated with the Ad
    #[serde(rename = "creativeTargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creative_target_id: Option<i64>,
    /// The type of the creative associated with the Ad \['Undefined' = 0, 'Image' = 1, 'Video' = 2\]
    #[serde(rename = "creativeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creative_type: Option<CreativeModelCreativeType>,
    /// The Url of the creative
    #[serde(rename = "creativeUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creative_url: Option<String>,
    /// The end date of the ad set
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// The number of conversions by ad impression generated by the ad
    #[serde(rename = "impressionConversions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impression_conversions: Option<i64>,
    /// The start date of the ad set
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// The target age bracket of the ad set \['Undefined' = 1, 'AgeUnder13' = 2, 'Age13OrOver' = 4, 'Age13To16' = 8, 'Age17OrOver' = 16\]
    #[serde(rename = "targetAgeBracket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_age_bracket: Option<CreateSponsoredGameV2RequestTargetAgeBracket>,
    /// The target device type of the ad set \['Undefined' = 1, 'Computer' = 2, 'Phone' = 4, 'Tablet' = 8, 'Console' = 16, 'VR' = 32\]
    #[serde(rename = "targetDeviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_device_type: Option<CreateSponsoredGameV2RequestTargetDeviceType>,
    /// The target gender of the ad set \['Undefined' = 1, 'Male' = 2, 'Female' = 4\]
    #[serde(rename = "targetGender")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_gender: Option<CreateSponsoredGameV2RequestTargetGender>,
    /// The total number of clicks generated by the ad
    #[serde(rename = "totalClicks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_clicks: Option<i64>,
    /// The total number of conversions generated by the ad
    #[serde(rename = "totalConversions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_conversions: Option<i64>,
    /// The total number of impressions generated by the ad
    #[serde(rename = "totalImpressions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_impressions: Option<i64>,
    /// The total spend of the ad set in Robux
    #[serde(rename = "totalSpendInRobux")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_spend_in_robux: Option<i64>,
}

/// `Roblox.AdConfiguration.Api.GetSponsoredGamesResponse`
///
/// A response model for retrieving a page of Roblox.AdConfiguration.Api.SponsoredGameV2Model.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetSponsoredGamesResponse {
    /// The cursor for retrieving the next page, if present.
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    /// The cursor for retrieving the previous page, if present.
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
    /// A collection of Roblox.AdConfiguration.Api.SponsoredGameV2Model.
    #[serde(rename = "sponsoredGames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sponsored_games: Option<Vec<SponsoredGameV2Model>>,
}

/// `Roblox.AdConfiguration.Api.Models.CampaignTargetModel`
///
/// Represents a sponsored ad campaign target
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CampaignTargetModel {
    /// The ID of the campaign target
    #[serde(rename = "campaignTargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_target_id: Option<i64>,
    /// The campaign target type \['Undefined' = 0, 'Universe' = 1, 'Asset' = 2, 'ImmersiveAd' = 3\]
    #[serde(rename = "campaignTargetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_target_type: Option<SponsoredCampaignModelCampaignTargetType>,
    /// The name of the campaign target (i.e. the asset name, universe name, group name, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

crate::api_enum! {
    int
    /// The location to place the campaign \['Undefined' = 1, 'GameSort' = 2, 'AvatarShop' = 4, 'ItemDetails' = 8, 'HomePage' = 16, 'Billboard300x250' = 32, 'Billboard600x300' = 64, 'Billboard300x600' = 128\]
    CreateSponsoredCampaignRequestPlacementLocation {
    Undefined = 1 => "Undefined",
    GameSort = 2 => "GameSort",
    AvatarShop = 4 => "AvatarShop",
    ItemDetails = 8 => "ItemDetails",
    HomePage = 16 => "HomePage",
    Billboard300x250 = 32 => "Billboard300x250",
    Billboard600x300 = 64 => "Billboard600x300",
    Billboard300x600 = 128 => "Billboard300x600",
    }
}

/// `Roblox.AdConfiguration.Api.Models.CreateSponsoredCampaignRequest`
///
/// A request model for creating a sponsored game
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CreateSponsoredCampaignRequest {
    /// The name of the Campaign / Ad
    #[serde(rename = "campaignName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_name: Option<String>,
    /// The ID of the campaign target
    #[serde(rename = "campaignTargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_target_id: Option<i64>,
    /// The type of the campaign target \['Undefined' = 0, 'Universe' = 1, 'Asset' = 2, 'ImmersiveAd' = 3\]
    #[serde(rename = "campaignTargetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_target_type: Option<SponsoredCampaignModelCampaignTargetType>,
    #[serde(rename = "creativeModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creative_model: Option<CreativeModel>,
    /// The daily bid amount for the campaign / ad, in Robux
    #[serde(rename = "dailyBidAmountInRobux")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_bid_amount_in_robux: Option<i64>,
    /// The end date of the ad set
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// The location to place the campaign \['Undefined' = 1, 'GameSort' = 2, 'AvatarShop' = 4, 'ItemDetails' = 8, 'HomePage' = 16, 'Billboard300x250' = 32, 'Billboard600x300' = 64, 'Billboard300x600' = 128\]
    #[serde(rename = "placementLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_location: Option<CreateSponsoredCampaignRequestPlacementLocation>,
    /// The start date of the ad set
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Targeting age bracket(s) of the ad set \['Undefined' = 1, 'AgeUnder13' = 2, 'Age13OrOver' = 4, 'Age13To16' = 8, 'Age17OrOver' = 16\]
    #[serde(rename = "targetAgeBracket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_age_bracket: Option<CreateSponsoredGameV2RequestTargetAgeBracket>,
    /// Targeting device type(s) of the ad set \['Undefined' = 1, 'Computer' = 2, 'Phone' = 4, 'Tablet' = 8, 'Console' = 16, 'VR' = 32\]
    #[serde(rename = "targetDeviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_device_type: Option<CreateSponsoredGameV2RequestTargetDeviceType>,
    /// Targeting gender(s) of the ad set \['Undefined' = 1, 'Male' = 2, 'Female' = 4\]
    #[serde(rename = "targetGender")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_gender: Option<CreateSponsoredGameV2RequestTargetGender>,
}

/// `Roblox.AdConfiguration.Api.Models.GetCampaignTargetsResponse`
///
/// The response model which returns a collection of Roblox.AdConfiguration.Api.Models.CampaignTargetModel
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetCampaignTargetsResponse {
    /// Gets or sets a collection of Roblox.AdConfiguration.Api.Models.CampaignTargetModel
    #[serde(rename = "campaignTargetModels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_target_models: Option<Vec<CampaignTargetModel>>,
}

/// `Roblox.AdConfiguration.Api.Models.GetEligibleCampaignTargetsRequest`
///
/// A model represents a request to stop a sponsored campaign / ad.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetEligibleCampaignTargetsRequest {
    /// The list of campaign types we want to include in the results
    #[serde(rename = "campaignTargetTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_target_types: Option<Vec<SponsoredCampaignModelCampaignTargetType>>,
    /// The group id, if applicable.
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i64>,
}

/// `Roblox.AdConfiguration.Api.Models.UniverseModel`
///
/// Represents a universe in API endpoint results.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UniverseModel {
    /// The universe Id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The name of the universe
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.AdConfiguration.Api.Models.GetRecentAdsRankedUniversesResponse`
///
/// The response model for getting a list of recent-ads-ranked universes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetRecentAdsRankedUniversesResponse {
    /// Gets or sets a list of Roblox.AdConfiguration.Api.Models.UniverseModel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universes: Option<Vec<UniverseModel>>,
}

/// `Roblox.AdConfiguration.Api.Models.StopSponsoredCampaignRequest`
///
/// A model represents a request to stop a sponsored campaign / ad.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct StopSponsoredCampaignRequest {
    /// The ID of the ad set to stop.
    #[serde(rename = "adSetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_set_id: Option<i64>,
}

/// `Roblox.AdConfiguration.Api.Models.StopSponsoredGameV2Request`
///
/// A model represents a request to stop a sponsored game ad.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct StopSponsoredGameV2Request {
    /// Gets the ID of the ad set to stop.
    #[serde(rename = "adSetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_set_id: Option<i64>,
}

/// `Roblox.Web.WebAPI.ApiEmptyResponseModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiEmptyResponseModel {}

/// Query, header and form inputs for [`get_sponsored_campaigns`].
#[derive(Debug, Clone, Default)]
pub struct GetSponsoredCampaignsParams {
    /// The campaign target type enum value
    pub campaign_target_type: i32,
    /// The id of the campaign target
    pub campaign_target_id: i64,
    /// Indicates whether to include reporting stats in the response
    pub include_reporting_stats: Option<bool>,
    /// Indicates whether to retrieve archived ads
    pub is_archived: Option<bool>,
    /// The cursor of the page to retrieve. If empty, fetches the first page
    pub page_cursor: Option<String>,
}

/// Query, header and form inputs for [`get_sponsored_campaigns_multi_get_can_user_sponsor`].
#[derive(Debug, Clone, Default)]
pub struct GetSponsoredCampaignsMultiGetCanUserSponsorParams {
    /// Ads.Management.Service.CampaignTargetType.
    pub campaign_target_type: i32,
    /// The IDs of the campaign targets.
    pub campaign_target_ids: Vec<i64>,
}

/// Query, header and form inputs for [`get_sponsored_games`].
#[derive(Debug, Clone, Default)]
pub struct GetSponsoredGamesParams {
    /// The universe id of the ad campaign.
    pub universe_id: i64,
    /// Indicates whether to include reporting stats in the response.
    pub include_reporting_stats: Option<bool>,
    /// Indicates whether to retrieve archived ads.
    pub is_archived: Option<bool>,
    /// The cursor of the page to retrieve.
    pub page_cursor: Option<String>,
}

/// Query, header and form inputs for [`get_sponsored_games_universes`].
#[derive(Debug, Clone, Default)]
pub struct GetSponsoredGamesUniversesParams {
    /// The group id, if applicable.
    pub group_id: Option<i64>,
}

/// `GET /v2/sponsored-campaigns`
///
/// Gets a page of Roblox.AdConfiguration.Api.SponsoredCampaignModel with specified input parameters.
pub fn get_sponsored_campaigns_request(params: &GetSponsoredCampaignsParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v2/sponsored-campaigns".to_string());
    r.query("campaignTargetType", &params.campaign_target_type);
    r.query("campaignTargetId", &params.campaign_target_id);
    r.query("includeReportingStats", &params.include_reporting_stats);
    r.query("isArchived", &params.is_archived);
    r.query("pageCursor", &params.page_cursor);
    Ok(r)
}

/// `GET /v2/sponsored-campaigns`
///
/// Gets a page of Roblox.AdConfiguration.Api.SponsoredCampaignModel with specified input parameters.
pub async fn get_sponsored_campaigns(
    client: &Client,
    params: &GetSponsoredCampaignsParams,
) -> Result<GetSponsoredCampaignsResponse> {
    client
        .send(get_sponsored_campaigns_request(params)?)
        .await?
        .json()
}

/// `POST /v2/sponsored-campaigns/create`
///
/// Creates a complete ad. Including ad campaign, ad set, escrow, and the ad. Currently intended for creation of sponsorships only.
pub fn post_sponsored_campaigns_create_request(body: &CreateSponsoredCampaignRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v2/sponsored-campaigns/create".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/sponsored-campaigns/create`
///
/// Creates a complete ad. Including ad campaign, ad set, escrow, and the ad. Currently intended for creation of sponsorships only.
pub async fn post_sponsored_campaigns_create(
    client: &Client,
    body: &CreateSponsoredCampaignRequest,
) -> Result<i64> {
    client
        .send(post_sponsored_campaigns_create_request(body)?)
        .await?
        .json()
}

/// `GET /v2/sponsored-campaigns/eligible-asset-type-ids`
///
/// Get all asset type IDs that are eligible to be sponsored.
pub fn get_sponsored_campaigns_eligible_asset_type_ids_request() -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        "/v2/sponsored-campaigns/eligible-asset-type-ids".to_string(),
    );
    Ok(r)
}

/// `GET /v2/sponsored-campaigns/eligible-asset-type-ids`
///
/// Get all asset type IDs that are eligible to be sponsored.
pub async fn get_sponsored_campaigns_eligible_asset_type_ids(client: &Client) -> Result<Vec<i32>> {
    client
        .send(get_sponsored_campaigns_eligible_asset_type_ids_request()?)
        .await?
        .json()
}

/// `POST /v2/sponsored-campaigns/eligible-campaign-targets`
///
/// Returns a collection of Roblox.AdConfiguration.Api.Models.CampaignTargetModel that the user is authorized to sponsor, ordered by most recently advertised
pub fn post_sponsored_campaigns_eligible_campaign_targets_request(
    body: &GetEligibleCampaignTargetsRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        "/v2/sponsored-campaigns/eligible-campaign-targets".to_string(),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/sponsored-campaigns/eligible-campaign-targets`
///
/// Returns a collection of Roblox.AdConfiguration.Api.Models.CampaignTargetModel that the user is authorized to sponsor, ordered by most recently advertised
pub async fn post_sponsored_campaigns_eligible_campaign_targets(
    client: &Client,
    body: &GetEligibleCampaignTargetsRequest,
) -> Result<GetCampaignTargetsResponse> {
    client
        .send(post_sponsored_campaigns_eligible_campaign_targets_request(body)?)
        .await?
        .json()
}

/// `GET /v2/sponsored-campaigns/multi-get-can-user-sponsor`
///
/// Checks whether the targets are eligible for sponsorship, and if the user is authorized to sponsor the targets.
pub fn get_sponsored_campaigns_multi_get_can_user_sponsor_request(
    params: &GetSponsoredCampaignsMultiGetCanUserSponsorParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        "/v2/sponsored-campaigns/multi-get-can-user-sponsor".to_string(),
    );
    r.query("campaignTargetType", &params.campaign_target_type);
    r.query("campaignTargetIds", &params.campaign_target_ids);
    Ok(r)
}

/// `GET /v2/sponsored-campaigns/multi-get-can-user-sponsor`
///
/// Checks whether the targets are eligible for sponsorship, and if the user is authorized to sponsor the targets.
pub async fn get_sponsored_campaigns_multi_get_can_user_sponsor(
    client: &Client,
    params: &GetSponsoredCampaignsMultiGetCanUserSponsorParams,
) -> Result<HashMap<String, bool>> {
    client
        .send(get_sponsored_campaigns_multi_get_can_user_sponsor_request(
            params,
        )?)
        .await?
        .json()
}

/// `POST /v2/sponsored-campaigns/stop`
///
/// Stops a sponsored campaign / ad (ad set) from running. Initiated by a user.
pub fn post_sponsored_campaigns_stop_request(body: &StopSponsoredCampaignRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v2/sponsored-campaigns/stop".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/sponsored-campaigns/stop`
///
/// Stops a sponsored campaign / ad (ad set) from running. Initiated by a user.
pub async fn post_sponsored_campaigns_stop(
    client: &Client,
    body: &StopSponsoredCampaignRequest,
) -> Result<i64> {
    client
        .send(post_sponsored_campaigns_stop_request(body)?)
        .await?
        .json()
}

/// `GET /v2/sponsored-games`
///
/// Gets a page of Roblox.AdConfiguration.Api.SponsoredGameV2Model with specified input parameters.
pub fn get_sponsored_games_request(params: &GetSponsoredGamesParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v2/sponsored-games".to_string());
    r.query("universeId", &params.universe_id);
    r.query("includeReportingStats", &params.include_reporting_stats);
    r.query("isArchived", &params.is_archived);
    r.query("pageCursor", &params.page_cursor);
    Ok(r)
}

/// `GET /v2/sponsored-games`
///
/// Gets a page of Roblox.AdConfiguration.Api.SponsoredGameV2Model with specified input parameters.
pub async fn get_sponsored_games(
    client: &Client,
    params: &GetSponsoredGamesParams,
) -> Result<GetSponsoredGamesResponse> {
    client.send(get_sponsored_games_request(params)?).await?.json()
}

/// `POST /v2/sponsored-games/create`
///
/// Creates a new sponsored game ad with specified input parameters.
pub fn post_sponsored_games_create_request(body: &CreateSponsoredGameV2Request) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v2/sponsored-games/create".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/sponsored-games/create`
///
/// Creates a new sponsored game ad with specified input parameters.
pub async fn post_sponsored_games_create(
    client: &Client,
    body: &CreateSponsoredGameV2Request,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_sponsored_games_create_request(body)?)
        .await?
        .json()
}

/// `POST /v2/sponsored-games/stop`
///
/// To stop a sponsored-game ad (ad set) from running, initiated by a user.
pub fn post_sponsored_games_stop_request(body: &StopSponsoredGameV2Request) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v2/sponsored-games/stop".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/sponsored-games/stop`
///
/// To stop a sponsored-game ad (ad set) from running, initiated by a user.
pub async fn post_sponsored_games_stop(
    client: &Client,
    body: &StopSponsoredGameV2Request,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_sponsored_games_stop_request(body)?)
        .await?
        .json()
}

/// `GET /v2/sponsored-games/universes`
///
/// Gets a list of universes for the authenticated user, or the given group, ordered by most recently created sponsored game ads.
pub fn get_sponsored_games_universes_request(params: &GetSponsoredGamesUniversesParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v2/sponsored-games/universes".to_string());
    r.query("groupId", &params.group_id);
    Ok(r)
}

/// `GET /v2/sponsored-games/universes`
///
/// Gets a list of universes for the authenticated user, or the given group, ordered by most recently created sponsored game ads.
pub async fn get_sponsored_games_universes(
    client: &Client,
    params: &GetSponsoredGamesUniversesParams,
) -> Result<GetRecentAdsRankedUniversesResponse> {
    client
        .send(get_sponsored_games_universes_request(params)?)
        .await?
        .json()
}

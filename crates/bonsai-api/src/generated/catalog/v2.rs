//! `catalog.roblox.com` v2 — generated from `vendor/roblox-api-tracker/catalog/v2.json`.
//!
//! Do not edit. Change the spec or `bonsai-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "catalog.roblox.com";

/// `Roblox.Catalog.Api.BundleItemDetailModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BundleItemDetailModel {
    /// The asset type of the bundle item, if it's an asset.
    #[serde(rename = "assetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owned: Option<bool>,
    /// Whether the bundle item supports head shapes.
    #[serde(rename = "supportsHeadShapes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_head_shapes: Option<bool>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// `Roblox.Catalog.Api.BundleItemDetailModelV2`
///
/// The is the beta (non game-engine) version of BundleItemDetailModel for internal consumption on Roblox web and universal-app.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BundleItemDetailModelV2 {
    /// The asset type of the bundle item, if it's an asset.
    #[serde(rename = "assetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owned: Option<bool>,
    /// Whether the bundle item supports head shapes.
    #[serde(rename = "supportsHeadShapes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_head_shapes: Option<bool>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

crate::api_enum! {
    int
    /// Catalog item restriction enum. \['ThirteenPlus' = 1, 'LimitedUnique' = 2, 'Limited' = 3, 'BuildersClub' = 4, 'TurboBuildersClub' = 5, 'OutrageousBuildersClub' = 6, 'Rthro' = 7, 'Live' = 8, 'Collectible' = 9\]
    CatalogSearchDetailedResponseItemItemRestrictions {
    ThirteenPlus = 1 => "ThirteenPlus",
    LimitedUnique = 2 => "LimitedUnique",
    Limited = 3 => "Limited",
    BuildersClub = 4 => "BuildersClub",
    TurboBuildersClub = 5 => "TurboBuildersClub",
    OutrageousBuildersClub = 6 => "OutrageousBuildersClub",
    Rthro = 7 => "Rthro",
    Live = 8 => "Live",
    Collectible = 9 => "Collectible",
    }
}

crate::api_enum! {
    int
    /// Catalog item status enum. \['New' = 1, 'Sale' = 2, 'SaleTimer' = 7, 'IsFAE' = 8\]
    CatalogSearchDetailedResponseItemItemStatus {
    New = 1 => "New",
    Sale = 2 => "Sale",
    SaleTimer = 7 => "SaleTimer",
    IsFae = 8 => "IsFAE",
    }
}

crate::api_enum! {
    int
    /// The Roblox.Catalog.Api.CatalogSearchDetailedResponseItem.ItemType item type. \['Asset' = 1, 'Bundle' = 2\]
    CatalogSearchDetailedResponseItemItemType {
    Asset = 1 => "Asset",
    Bundle = 2 => "Bundle",
    }
}

crate::api_enum! {
    int
    /// The sale location type of the item. \['NotApplicable' = 0, 'ShopOnly' = 1, 'MyExperiencesOnly' = 2, 'ShopAndMyExperiences' = 3, 'ExperiencesById' = 4, 'ShopAndAllExperiences' = 5, 'ExperiencesDevApiOnly' = 6, 'ShopAndExperiencesById' = 7\]
    CatalogSearchDetailedResponseItemSaleLocationType {
    NotApplicable = 0 => "NotApplicable",
    ShopOnly = 1 => "ShopOnly",
    MyExperiencesOnly = 2 => "MyExperiencesOnly",
    ShopAndMyExperiences = 3 => "ShopAndMyExperiences",
    ExperiencesById = 4 => "ExperiencesById",
    ShopAndAllExperiences = 5 => "ShopAndAllExperiences",
    ExperiencesDevApiOnly = 6 => "ExperiencesDevApiOnly",
    ShopAndExperiencesById = 7 => "ShopAndExperiencesById",
    }
}

/// `Roblox.Catalog.Api.Discount`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Discount {
    #[serde(rename = "discountCampaign")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_campaign: Option<String>,
    #[serde(rename = "localizedDiscountAttribution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localized_discount_attribution: Option<String>,
    #[serde(rename = "robuxDiscountAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robux_discount_amount: Option<i64>,
    #[serde(rename = "robuxDiscountPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robux_discount_percentage: Option<f64>,
}

/// `Roblox.Catalog.Api.DiscountInformation`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DiscountInformation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<Discount>>,
    #[serde(rename = "originalPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_price: Option<i64>,
    #[serde(rename = "totalDiscountAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_discount_amount: Option<i64>,
    #[serde(rename = "totalDiscountPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_discount_percentage: Option<f64>,
}

/// `Roblox.Catalog.Api.TimedOption`
///
/// Defines a timed option for a catalog item.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TimedOption {
    /// The days for the timed option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i32>,
    #[serde(rename = "discountInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_information: Option<DiscountInformation>,
    /// The price for the timed option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
    /// To indicate if this option is selected by the client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected: Option<bool>,
}

/// `Roblox.Catalog.Api.CatalogSearchDetailedResponseItem`
///
/// Game-engine version of fully hydrated asset or bundle in catalog marketplace. Documented here: https://create.roblox.com/docs/reference/engine/classes/AvatarEditorService#SearchCatalog.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CatalogSearchDetailedResponseItem {
    /// The Roblox.Platform.Assets.AssetType serialized if item is an asset.
    #[serde(rename = "assetType")]
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<i32>,
    /// The Roblox.Platform.Bundles.Core.BundleType serialized if item is a bundle.
    #[serde(rename = "bundleType")]
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_type: Option<i32>,
    /// The System.Collections.Generic.IEnumerable'1 contained in the bundle, serialized if item is a bundle.
    #[serde(rename = "bundledItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundled_items: Option<Vec<BundleItemDetailModel>>,
    /// The item's collectible item id. It is an UUID if a item is collectible type. Otherwise, it is null.
    #[serde(rename = "collectibleItemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collectible_item_id: Option<String>,
    /// The verified status of a creator.
    #[serde(rename = "creatorHasVerifiedBadge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_has_verified_badge: Option<bool>,
    /// The creator name of the item's creator.
    #[serde(rename = "creatorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_name: Option<String>,
    /// The creator id of the item's creator.
    #[serde(rename = "creatorTargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_target_id: Option<i64>,
    /// The Roblox.Catalog.Api.CatalogSearchDetailedResponseItem.CreatorType of the item's creator.
    #[serde(rename = "creatorType")]
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_type: Option<i32>,
    /// The item description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The number of times the item has been favorited.
    #[serde(rename = "favoriteCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favorite_count: Option<i64>,
    /// An indicator if the item has resellers or not (null if not resellable).
    #[serde(rename = "hasResellers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_resellers: Option<bool>,
    /// The Item Id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// An indicator if the item is off sale or not.
    #[serde(rename = "isOffSale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_off_sale: Option<bool>,
    /// Gets or sets the property whether a bundle is recolorable or not. Not serialized for asset.
    #[serde(rename = "isRecolorable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_recolorable: Option<bool>,
    /// The System.Collections.Generic.IEnumerable'1 if item has Roblox.Catalog.Api.CatalogItemRestriction.
    #[serde(rename = "itemRestrictions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_restrictions: Option<Vec<CatalogSearchDetailedResponseItemItemRestrictions>>,
    /// The System.Collections.Generic.IEnumerable'1 if item has Roblox.Catalog.Api.CatalogItemStatus.
    #[serde(rename = "itemStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_status: Option<Vec<CatalogSearchDetailedResponseItemItemStatus>>,
    /// The Roblox.Catalog.Api.CatalogSearchDetailedResponseItem.ItemType item type. \['Asset' = 1, 'Bundle' = 2\]
    #[serde(rename = "itemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_type: Option<CatalogSearchDetailedResponseItemItemType>,
    /// The item's lowest price, only if the item is resellable and there are resellers.
    #[serde(rename = "lowestPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lowest_price: Option<i64>,
    /// The item's lowest resale price, only if the item is resellable and there are resellers, including current user.
    #[serde(rename = "lowestResalePrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lowest_resale_price: Option<i64>,
    /// The item name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// When the item will go off sale, if the item has an off deadline.
    #[serde(rename = "offSaleDeadline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_sale_deadline: Option<String>,
    /// The item's price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
    /// The localized string item status if the item's price should not be displayed.
    #[serde(rename = "priceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_status: Option<String>,
    /// The product id of corresponding item.
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i64>,
    /// Quantity limit for how many instances a user can buy.
    #[serde(rename = "quantityLimitPerUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_limit_per_user: Option<i32>,
    /// The sale location type of the item. \['NotApplicable' = 0, 'ShopOnly' = 1, 'MyExperiencesOnly' = 2, 'ShopAndMyExperiences' = 3, 'ExperiencesById' = 4, 'ShopAndAllExperiences' = 5, 'ExperiencesDevApiOnly' = 6, 'ShopAndExperiencesById' = 7\]
    #[serde(rename = "saleLocationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_location_type: Option<CatalogSearchDetailedResponseItemSaleLocationType>,
    /// Whether the item supports head shapes.
    #[serde(rename = "supportsHeadShapes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_head_shapes: Option<bool>,
    /// The timed options for the item.
    #[serde(rename = "timedOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_options: Option<Vec<TimedOption>>,
    /// The collectible or limited-unique item's total quantity of unique instances.
    #[serde(rename = "totalQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_quantity: Option<i64>,
    /// The number of items in stock, only if the item is resellable and is limitedEdition.
    #[serde(rename = "unitsAvailableForConsumption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units_available_for_consumption: Option<i64>,
}

crate::api_enum! {
    int
    /// The type of license attached to a collectible. \['Invalid' = 0, 'ThirdParty' = 1, 'FirstParty' = 2\]
    CollectibleLicenseLicenseType {
    Invalid = 0 => "Invalid",
    ThirdParty = 1 => "ThirdParty",
    FirstParty = 2 => "FirstParty",
    }
}

/// `Roblox.Catalog.Api.CollectibleLicense`
///
/// A model representing the license attached to a collectible, linked via the collectibleItemId.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CollectibleLicense {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "licenseDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_display_name: Option<String>,
    #[serde(rename = "licenseHolder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_holder: Option<String>,
    /// The type of license attached to a collectible. \['Invalid' = 0, 'ThirdParty' = 1, 'FirstParty' = 2\]
    #[serde(rename = "licenseType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_type: Option<CollectibleLicenseLicenseType>,
}

/// `Roblox.Catalog.Api.TaxonomyModel`
///
/// public api model coaslescing taxonomy information for a single item.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TaxonomyModel {
    /// The id value to pass into taxonomy field in SearchV2 catalog-api.
    #[serde(rename = "taxonomyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxonomy_id: Option<String>,
    /// The localized human readable name of the category.
    #[serde(rename = "taxonomyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxonomy_name: Option<String>,
}

/// `Roblox.Catalog.Api.CatalogSearchDetailedResponseItemV2`
///
/// This is the beta (non game-engine) version of our hydration model representing asset or bundle in marketplace.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CatalogSearchDetailedResponseItemV2 {
    /// The Roblox.Platform.Assets.AssetType serialized if item is an asset.
    #[serde(rename = "assetType")]
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<i32>,
    /// The Roblox.Platform.Bundles.Core.BundleType serialized if item is a bundle.
    #[serde(rename = "bundleType")]
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_type: Option<i32>,
    /// The System.Collections.Generic.IEnumerable'1 contained in the bundle, serialized if item is a bundle.
    #[serde(rename = "bundledItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundled_items: Option<Vec<BundleItemDetailModelV2>>,
    /// The item's collectible item id. It is an UUID if a item is collectible type. Otherwise, it is null.
    #[serde(rename = "collectibleItemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collectible_item_id: Option<String>,
    /// The verified status of a creator.
    #[serde(rename = "creatorHasVerifiedBadge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_has_verified_badge: Option<bool>,
    /// The creator name of the item's creator.
    #[serde(rename = "creatorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_name: Option<String>,
    /// The creator id of the item's creator.
    #[serde(rename = "creatorTargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_target_id: Option<i64>,
    /// The Roblox.Catalog.Api.CatalogSearchDetailedResponseItem.CreatorType of the item's creator.
    #[serde(rename = "creatorType")]
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_type: Option<i32>,
    /// The item description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "discountInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_information: Option<DiscountInformation>,
    /// The number of times the item has been favorited.
    #[serde(rename = "favoriteCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favorite_count: Option<i64>,
    /// An indicator if the item has resellers or not (null if not resellable).
    #[serde(rename = "hasResellers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_resellers: Option<bool>,
    /// The Item Id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// An indicator if the item is off sale or not.
    #[serde(rename = "isOffSale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_off_sale: Option<bool>,
    /// Gets or sets the property whether a bundle is recolorable or not. Not serialized for asset.
    #[serde(rename = "isRecolorable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_recolorable: Option<bool>,
    /// The UTC creation date-time of the asset or bundle.
    #[serde(rename = "itemCreatedUtc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_created_utc: Option<String>,
    /// The System.Collections.Generic.IEnumerable'1 if item has Roblox.Catalog.Api.CatalogItemRestriction.
    #[serde(rename = "itemRestrictions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_restrictions: Option<Vec<CatalogSearchDetailedResponseItemItemRestrictions>>,
    /// The System.Collections.Generic.IEnumerable'1 if item has Roblox.Catalog.Api.CatalogItemStatus.
    #[serde(rename = "itemStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_status: Option<Vec<CatalogSearchDetailedResponseItemItemStatus>>,
    /// The Roblox.Catalog.Api.CatalogSearchDetailedResponseItem.ItemType item type. \['Asset' = 1, 'Bundle' = 2\]
    #[serde(rename = "itemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_type: Option<CatalogSearchDetailedResponseItemItemType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<CollectibleLicense>,
    /// The item's lowest price, only if the item is resellable and there are resellers.
    #[serde(rename = "lowestPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lowest_price: Option<i64>,
    /// The item's lowest resale price, only if the item is resellable and there are resellers, including current user.
    #[serde(rename = "lowestResalePrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lowest_resale_price: Option<i64>,
    /// The item name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// When the item will go off sale, if the item has an off deadline.
    #[serde(rename = "offSaleDeadline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_sale_deadline: Option<String>,
    /// The item's price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
    /// The localized string item status if the item's price should not be displayed.
    #[serde(rename = "priceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_status: Option<String>,
    /// The product id of corresponding item.
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i64>,
    /// Quantity limit for how many instances a user can buy.
    #[serde(rename = "quantityLimitPerUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_limit_per_user: Option<i32>,
    /// The sale location type of the item. \['NotApplicable' = 0, 'ShopOnly' = 1, 'MyExperiencesOnly' = 2, 'ShopAndMyExperiences' = 3, 'ExperiencesById' = 4, 'ShopAndAllExperiences' = 5, 'ExperiencesDevApiOnly' = 6, 'ShopAndExperiencesById' = 7\]
    #[serde(rename = "saleLocationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_location_type: Option<CatalogSearchDetailedResponseItemSaleLocationType>,
    /// Whether the item supports head shapes.
    #[serde(rename = "supportsHeadShapes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_head_shapes: Option<bool>,
    /// The taxonomy ids and names for this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxonomy: Option<Vec<TaxonomyModel>>,
    /// The timed options for the item.
    #[serde(rename = "timedOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_options: Option<Vec<TimedOption>>,
    /// The collectible or limited-unique item's total quantity of unique instances.
    #[serde(rename = "totalQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_quantity: Option<i64>,
    /// The number of items in stock, only if the item is resellable and is limitedEdition.
    #[serde(rename = "unitsAvailableForConsumption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units_available_for_consumption: Option<i64>,
}

/// `Roblox.Catalog.Api.ElasticsearchDebugInfo`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ElasticsearchDebugInfo {
    /// Gets or sets the nest query that resulted from the operation.
    #[serde(rename = "elasticsearchQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_query: Option<String>,
    /// Gets or sets the index name.
    #[serde(rename = "indexName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    /// Gets or sets if Force Termination was Enabled in the Request.
    #[serde(rename = "isForceTerminationEnabledByRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_force_termination_enabled_by_request: Option<bool>,
    /// Gets or sets if the results were returned from cache.
    #[serde(rename = "isFromCache")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_from_cache: Option<bool>,
    /// Gets or sets if the query was Terminated Early.
    #[serde(rename = "isTerminatedEarly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_terminated_early: Option<bool>,
    /// Gets or sets the search result data source.
    #[serde(rename = "searchResultDataSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_result_data_source: Option<String>,
    /// Gets or sets the search result engagement score.
    #[serde(rename = "searchResultEngagementScore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_result_engagement_score: Option<String>,
    /// Gets or sets the search result relevance score.
    #[serde(rename = "searchResultRelevanceScore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_result_relevance_score: Option<String>,
}

/// `Roblox.Catalog.Api.CatalogSearchPageResponse\[Roblox.Catalog.Api.CatalogSearchDetailedResponseItemV2\]`
///
/// ApiPageResponse for catalog search.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CatalogSearchPageResponseCatalogSearchDetailedResponseItemV2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<CatalogSearchDetailedResponseItemV2>>,
    #[serde(rename = "elasticsearchDebugInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_debug_info: Option<ElasticsearchDebugInfo>,
    /// Keyword used for search query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
}

/// `Roblox.Web.WebAPI.Models.ApiPageResponse\[Roblox.Catalog.Api.CatalogSearchDetailedResponseItem\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiPageResponseCatalogSearchDetailedResponseItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<CatalogSearchDetailedResponseItem>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(rename = "previousPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
}

crate::api_enum! {
    str
    /// The order the results are sorted in.
    GetAssetsByAssetIdBundlesSortOrder {
    Asc = 0 => "Asc",
    Desc = 1 => "Desc",
    }
}

/// Query, header and form inputs for [`get_assets_by_asset_id_bundles`].
#[derive(Debug, Clone, Default)]
pub struct GetAssetsByAssetIdBundlesParams {
    /// Roblox-Place-Id header.
    pub roblox_place_id: i64,
    /// Roblox-Game-Id header.
    pub roblox_game_id: String,
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// The order the results are sorted in.
    pub sort_order: Option<GetAssetsByAssetIdBundlesSortOrder>,
}

crate::api_enum! {
    int
    /// The category filter types, representing generic collections of catalog items requested by client. \['None' = 0, 'Featured' = 1, 'Collectibles' = 2, 'CommunityCreations' = 3, 'Landing' = 4, 'Premium' = 5, 'Recommended' = 6, 'HighDefinition' = 7\]
    GetSearchItemsDetailsCategoryFilter {
    None = 0 => "None",
    Featured = 1 => "Featured",
    Collectibles = 2 => "Collectibles",
    CommunityCreations = 3 => "CommunityCreations",
    Landing = 4 => "Landing",
    Premium = 5 => "Premium",
    Recommended = 6 => "Recommended",
    HighDefinition = 7 => "HighDefinition",
    }
}

crate::api_enum! {
    int
    /// \['Past12Hours' = 0, 'PastDay' = 1, 'Past3Days' = 2, 'PastWeek' = 3, 'PastMonth' = 4, 'AllTime' = 5\]
    GetSearchItemsDetailsSortAggregation {
    Past12Hours = 0 => "Past12Hours",
    PastDay = 1 => "PastDay",
    Past3Days = 2 => "Past3Days",
    PastWeek = 3 => "PastWeek",
    PastMonth = 4 => "PastMonth",
    AllTime = 5 => "AllTime",
    }
}

crate::api_enum! {
    int
    /// Price values to be consolidated after roll out to api site. \['Relevance' = 0, 'Favorited' = 1, 'Sales' = 2, 'Created' = 3, 'Updated' = 3, 'PriceAsc' = 4, 'PriceDesc' = 5\]
    GetSearchItemsDetailsSortType {
    Relevance = 0 => "Relevance",
    Favorited = 1 => "Favorited",
    Sales = 2 => "Sales",
    Created = 3 => "Created",
    Updated = 3 => "Updated",
    PriceAsc = 4 => "PriceAsc",
    PriceDesc = 5 => "PriceDesc",
    }
}

crate::api_enum! {
    int
    /// \['All' = 0, 'User' = 1, 'Group' = 2\]
    GetSearchItemsDetailsCreatorType {
    All = 0 => "All",
    User = 1 => "User",
    Group = 2 => "Group",
    }
}

crate::api_enum! {
    int
    /// The Sales Type filter options for search. \['Unknown' = 0, 'All' = 1, 'Collectibles' = 2, 'Premium' = 3, 'TimedOptions' = 4\]
    GetSearchItemsDetailsSalesTypeFilter {
    UnknownValue = 0 => "Unknown",
    All = 1 => "All",
    Collectibles = 2 => "Collectibles",
    Premium = 3 => "Premium",
    TimedOptions = 4 => "TimedOptions",
    }
}

crate::api_enum! {
    str
    /// The order the results are sorted in.
    GetSearchItemsDetailsSortOrder {
    Desc = 0 => "Desc",
    }
}

/// Query, header and form inputs for [`get_search_items_details`].
#[derive(Debug, Clone, Default)]
pub struct GetSearchItemsDetailsParams {
    pub taxonomy: Option<String>,
    pub asset_type_ids: Option<Vec<i32>>,
    pub bundle_type_ids: Option<Vec<i32>>,
    /// The category filter types, representing generic collections of catalog items requested by client. \['None' = 0, 'Featured' = 1, 'Collectibles' = 2, 'CommunityCreations' = 3, 'Landing' = 4, 'Premium' = 5, 'Recommended' = 6, 'HighDefinition' = 7\]
    pub category_filter: Option<GetSearchItemsDetailsCategoryFilter>,
    /// \['Past12Hours' = 0, 'PastDay' = 1, 'Past3Days' = 2, 'PastWeek' = 3, 'PastMonth' = 4, 'AllTime' = 5\]
    pub sort_aggregation: Option<GetSearchItemsDetailsSortAggregation>,
    /// Price values to be consolidated after roll out to api site. \['Relevance' = 0, 'Favorited' = 1, 'Sales' = 2, 'Created' = 3, 'Updated' = 3, 'PriceAsc' = 4, 'PriceDesc' = 5\]
    pub sort_type: Option<GetSearchItemsDetailsSortType>,
    /// \['All' = 0, 'User' = 1, 'Group' = 2\]
    pub creator_type: Option<GetSearchItemsDetailsCreatorType>,
    pub creator_target_id: Option<i64>,
    pub creator_name: Option<String>,
    pub max_price: Option<i32>,
    pub min_price: Option<i32>,
    pub keyword: Option<String>,
    pub include_not_for_sale: Option<bool>,
    pub triggered_by_topic_discovery: Option<bool>,
    /// The Sales Type filter options for search. \['Unknown' = 0, 'All' = 1, 'Collectibles' = 2, 'Premium' = 3, 'TimedOptions' = 4\]
    pub sales_type_filter: Option<GetSearchItemsDetailsSalesTypeFilter>,
    /// The input topics format is split by ",". E.g "topics=cat,hat,red".
    pub topics: Option<String>,
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// The order the results are sorted in.
    pub sort_order: Option<GetSearchItemsDetailsSortOrder>,
}

/// `GET /v2/assets/{assetId}/bundles`
///
/// Lists bundles that contain the given asset (hydrated search-detail shape for marketplace).
pub fn get_assets_by_asset_id_bundles_request(
    asset_id: i64,
    params: &GetAssetsByAssetIdBundlesParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v2/assets/{}/bundles", crate::rt::path_value(&asset_id)),
    );
    r.header("Roblox-Place-Id", &params.roblox_place_id);
    r.header("Roblox-Game-Id", &params.roblox_game_id);
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    r.query("sortOrder", &params.sort_order);
    Ok(r)
}

/// `GET /v2/assets/{assetId}/bundles`
///
/// Lists bundles that contain the given asset (hydrated search-detail shape for marketplace).
pub async fn get_assets_by_asset_id_bundles(
    client: &Client,
    asset_id: i64,
    params: &GetAssetsByAssetIdBundlesParams,
) -> Result<ApiPageResponseCatalogSearchDetailedResponseItem> {
    client
        .send(get_assets_by_asset_id_bundles_request(asset_id, params)?)
        .await?
        .json()
}

/// `GET /v2/search/items/details`
///
/// Search for catalog items.
///
/// This endpoint is for search by item type ids.
pub fn get_search_items_details_request(params: &GetSearchItemsDetailsParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v2/search/items/details".to_string());
    r.query("Taxonomy", &params.taxonomy);
    r.query("AssetTypeIds", &params.asset_type_ids);
    r.query("BundleTypeIds", &params.bundle_type_ids);
    r.query("CategoryFilter", &params.category_filter);
    r.query("SortAggregation", &params.sort_aggregation);
    r.query("SortType", &params.sort_type);
    r.query("CreatorType", &params.creator_type);
    r.query("CreatorTargetId", &params.creator_target_id);
    r.query("CreatorName", &params.creator_name);
    r.query("MaxPrice", &params.max_price);
    r.query("MinPrice", &params.min_price);
    r.query("Keyword", &params.keyword);
    r.query("IncludeNotForSale", &params.include_not_for_sale);
    r.query("TriggeredByTopicDiscovery", &params.triggered_by_topic_discovery);
    r.query("SalesTypeFilter", &params.sales_type_filter);
    r.query("Topics", &params.topics);
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    r.query("sortOrder", &params.sort_order);
    Ok(r)
}

/// `GET /v2/search/items/details`
///
/// Search for catalog items.
///
/// This endpoint is for search by item type ids.
pub async fn get_search_items_details(
    client: &Client,
    params: &GetSearchItemsDetailsParams,
) -> Result<CatalogSearchPageResponseCatalogSearchDetailedResponseItemV2> {
    client
        .send(get_search_items_details_request(params)?)
        .await?
        .json()
}

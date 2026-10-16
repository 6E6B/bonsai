//! `catalog.roblox.com` v1 — generated from `vendor/roblox-api-tracker/catalog/v1.json`.
//!
//! Do not edit. Change the spec or `bonsai-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "catalog.roblox.com";

/// `Roblox.Catalog.Api.AssetFavoriteModel`
///
/// A model to represent asset favorites.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AssetFavoriteModel {
    /// The Id of the asset being favorited.
    #[serde(rename = "assetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<i64>,
    /// The time at which the user favorited the asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// The Id of the user favoriting the asset.
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}

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

crate::api_enum! {
    int
    /// Catalog item restriction enum. \['ThirteenPlus' = 1, 'LimitedUnique' = 2, 'Limited' = 3, 'BuildersClub' = 4, 'TurboBuildersClub' = 5, 'OutrageousBuildersClub' = 6, 'Rthro' = 7, 'Live' = 8, 'Collectible' = 9\]
    CatalogSearchDetailedCompleteResponseItemItemRestrictions {
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
    CatalogSearchDetailedCompleteResponseItemItemStatus {
    New = 1 => "New",
    Sale = 2 => "Sale",
    SaleTimer = 7 => "SaleTimer",
    IsFae = 8 => "IsFAE",
    }
}

crate::api_enum! {
    int
    /// The Roblox.Catalog.Api.CatalogSearchDetailedResponseItem.ItemType item type. \['Asset' = 1, 'Bundle' = 2\]
    CatalogSearchDetailedCompleteResponseItemItemType {
    Asset = 1 => "Asset",
    Bundle = 2 => "Bundle",
    }
}

crate::api_enum! {
    int
    /// The sale location type of the item. \['NotApplicable' = 0, 'ShopOnly' = 1, 'MyExperiencesOnly' = 2, 'ShopAndMyExperiences' = 3, 'ExperiencesById' = 4, 'ShopAndAllExperiences' = 5, 'ExperiencesDevApiOnly' = 6, 'ShopAndExperiencesById' = 7\]
    CatalogSearchDetailedCompleteResponseItemSaleLocationType {
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

/// `Roblox.Catalog.Api.CatalogSearchDetailedCompleteResponseItem`
///
/// The detailed model for catalog items, complete with economy and ownership info. This response model is bound in the game-engine here: https://create.roblox.com/docs/reference/engine/classes/AvatarEditorService#GetItemDetails.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CatalogSearchDetailedCompleteResponseItem {
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
    /// The universeId that this asset was created in, null if not an in-experience creation.
    #[serde(rename = "creatingUniverseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creating_universe_id: Option<i64>,
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
    /// Id to pass into economy purchase api userID of lowest private seller if resellable Creator AgentID (deprecated) / CreatorTargetId otherwise.
    #[serde(rename = "expectedSellerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_seller_id: Option<i64>,
    /// If a user purchased timed option item, its expiration time.
    #[serde(rename = "expirationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
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
    /// Whether item can be purchased.
    #[serde(rename = "isPurchasable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_purchasable: Option<bool>,
    /// Gets or sets the property whether a bundle is recolorable or not. Not serialized for asset.
    #[serde(rename = "isRecolorable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_recolorable: Option<bool>,
    /// The System.Collections.Generic.IEnumerable'1 if item has Roblox.Catalog.Api.CatalogItemRestriction.
    #[serde(rename = "itemRestrictions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_restrictions: Option<Vec<CatalogSearchDetailedCompleteResponseItemItemRestrictions>>,
    /// The System.Collections.Generic.IEnumerable'1 if item has Roblox.Catalog.Api.CatalogItemStatus.
    #[serde(rename = "itemStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_status: Option<Vec<CatalogSearchDetailedCompleteResponseItemItemStatus>>,
    /// The Roblox.Catalog.Api.CatalogSearchDetailedResponseItem.ItemType item type. \['Asset' = 1, 'Bundle' = 2\]
    #[serde(rename = "itemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_type: Option<CatalogSearchDetailedCompleteResponseItemItemType>,
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
    /// Whether item is owned by user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owned: Option<bool>,
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
    pub sale_location_type: Option<CatalogSearchDetailedCompleteResponseItemSaleLocationType>,
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

/// `Roblox.Catalog.Api.CatalogItemOrError`
///
/// A union type representing either a successfully hydrated item or an error. Exactly one of Roblox.Catalog.Api.CatalogItemOrError.Item or Roblox.Catalog.Api.CatalogItemOrError.HttpStatusCode will be non-null.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CatalogItemOrError {
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<CatalogSearchDetailedCompleteResponseItem>,
    /// \['Asset' = 1, 'Bundle' = 2\]
    #[serde(rename = "itemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_type: Option<CatalogSearchDetailedCompleteResponseItemItemType>,
}

/// `Roblox.Catalog.Api.BatchItemDetailsCompleteResponse`
///
/// Response for the batch GetItemDetailsComplete endpoint. Each entry in Roblox.Catalog.Api.BatchItemDetailsCompleteResponse.Data is either a successfully hydrated item or a critical error.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BatchItemDetailsCompleteResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<CatalogItemOrError>>,
}

/// `Roblox.Catalog.Api.BundleCreatorModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BundleCreatorModel {
    #[serde(rename = "hasVerifiedBadge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_verified_badge: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

crate::api_enum! {
    int
    /// The type of collectible item, limited or non-limited for now. \['Invalid' = 0, 'Limited' = 1, 'NonLimited' = 2\]
    CollectibleItemDetailCollectibleItemType {
    Invalid = 0 => "Invalid",
    Limited = 1 => "Limited",
    NonLimited = 2 => "NonLimited",
    }
}

crate::api_enum! {
    int
    /// \['Invalid' = 0, 'None' = 1, 'Disabled' = 2\]
    CollectibleItemDetailResaleRestriction {
    Invalid = 0 => "Invalid",
    None = 1 => "None",
    Disabled = 2 => "Disabled",
    }
}

/// `Roblox.Catalog.Api.SaleLocation`
///
/// SaleLocation information for a collectible item (asset or bundle).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SaleLocation {
    #[serde(rename = "enabledUniverseIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_universe_ids: Option<Vec<i64>>,
    /// Sale location type related for an item detail. \['NotApplicable' = 0, 'ShopOnly' = 1, 'MyExperiencesOnly' = 2, 'ShopAndMyExperiences' = 3, 'ExperiencesById' = 4, 'ShopAndAllExperiences' = 5, 'ExperiencesDevApiOnly' = 6, 'ShopAndExperiencesById' = 7\]
    #[serde(rename = "saleLocationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_location_type: Option<CatalogSearchDetailedCompleteResponseItemSaleLocationType>,
    #[serde(rename = "saleLocationTypeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_location_type_id: Option<i32>,
    #[serde(rename = "universeIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_ids: Option<Vec<i64>>,
}

crate::api_enum! {
    int
    /// \['Invalid' = 0, 'Draft' = 1, 'OffSale' = 2, 'OnSale' = 3, 'PendingSale' = 4\]
    CollectibleItemDetailSaleStatus {
    Invalid = 0 => "Invalid",
    Draft = 1 => "Draft",
    OffSale = 2 => "OffSale",
    OnSale = 3 => "OnSale",
    PendingSale = 4 => "PendingSale",
    }
}

/// `Roblox.Catalog.Api.CollectibleItemDetail`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CollectibleItemDetail {
    #[serde(rename = "collectibleItemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collectible_item_id: Option<String>,
    /// The type of collectible item, limited or non-limited for now. \['Invalid' = 0, 'Limited' = 1, 'NonLimited' = 2\]
    #[serde(rename = "collectibleItemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collectible_item_type: Option<CollectibleItemDetailCollectibleItemType>,
    #[serde(rename = "collectibleProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collectible_product_id: Option<String>,
    #[serde(rename = "hasResellers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_resellers: Option<bool>,
    #[serde(rename = "lowestAvailableResaleItemInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lowest_available_resale_item_instance_id: Option<String>,
    #[serde(rename = "lowestAvailableResaleProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lowest_available_resale_product_id: Option<String>,
    #[serde(rename = "lowestPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lowest_price: Option<i64>,
    #[serde(rename = "lowestResalePrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lowest_resale_price: Option<i64>,
    #[serde(rename = "offSaleDeadline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_sale_deadline: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
    #[serde(rename = "quantityLimitPerUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_limit_per_user: Option<i32>,
    /// \['Invalid' = 0, 'None' = 1, 'Disabled' = 2\]
    #[serde(rename = "resaleRestriction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resale_restriction: Option<CollectibleItemDetailResaleRestriction>,
    #[serde(rename = "saleLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_location: Option<SaleLocation>,
    /// \['Invalid' = 0, 'Draft' = 1, 'OffSale' = 2, 'OnSale' = 3, 'PendingSale' = 4\]
    #[serde(rename = "saleStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_status: Option<CollectibleItemDetailSaleStatus>,
    #[serde(rename = "totalQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_quantity: Option<i64>,
    #[serde(rename = "unitsAvailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units_available: Option<i64>,
}

/// `Roblox.Catalog.Api.PremiumPricingModel`
///
/// Defines the Premium pricing for a catalog item.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PremiumPricingModel {
    /// The Premium discount percentage for a catalog item.
    #[serde(rename = "premiumDiscountPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_discount_percentage: Option<i32>,
    /// The Premium price for a catalog item.
    #[serde(rename = "premiumPriceInRobux")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_price_in_robux: Option<i64>,
}

/// `Roblox.Catalog.Api.BundleProductModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BundleProductModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "isForSale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_for_sale: Option<bool>,
    #[serde(rename = "isFree")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_free: Option<bool>,
    #[serde(rename = "isPublicDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public_domain: Option<bool>,
    #[serde(rename = "noPriceText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_price_text: Option<String>,
    #[serde(rename = "premiumPricing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_pricing: Option<PremiumPricingModel>,
    #[serde(rename = "priceInRobux")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_in_robux: Option<i64>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// `Roblox.Catalog.Api.BundleDetailsModel`
///
/// The hydration model representing a bundle on marketplace. Returned in all bundles controller endpoints. Bound in the game-engine MarketplaceService.GetProductInfo method. https://create.roblox.com/docs/reference/engine/classes/MarketplaceService#GetProductInfo.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BundleDetailsModel {
    #[serde(rename = "bundleType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_type: Option<String>,
    #[serde(rename = "collectibleItemDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collectible_item_detail: Option<CollectibleItemDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<BundleCreatorModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "discountInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_information: Option<DiscountInformation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "isRecolorable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_recolorable: Option<bool>,
    #[serde(rename = "itemRestrictions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_restrictions: Option<Vec<CatalogSearchDetailedCompleteResponseItemItemRestrictions>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<BundleItemDetailModel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<BundleProductModel>,
}

/// `Roblox.Catalog.Api.BundleFavoriteModel`
///
/// A model to represent bundle favorites.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BundleFavoriteModel {
    /// The Id of the bundle being favorited.
    #[serde(rename = "bundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<i64>,
    /// The time at which the user favorited the bundle.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// The Id of the user favoriting the bundle.
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
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
    pub item_restrictions: Option<Vec<CatalogSearchDetailedCompleteResponseItemItemRestrictions>>,
    /// The System.Collections.Generic.IEnumerable'1 if item has Roblox.Catalog.Api.CatalogItemStatus.
    #[serde(rename = "itemStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_status: Option<Vec<CatalogSearchDetailedCompleteResponseItemItemStatus>>,
    /// The Roblox.Catalog.Api.CatalogSearchDetailedResponseItem.ItemType item type. \['Asset' = 1, 'Bundle' = 2\]
    #[serde(rename = "itemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_type: Option<CatalogSearchDetailedCompleteResponseItemItemType>,
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
    pub sale_location_type: Option<CatalogSearchDetailedCompleteResponseItemSaleLocationType>,
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
    pub item_restrictions: Option<Vec<CatalogSearchDetailedCompleteResponseItemItemRestrictions>>,
    /// The System.Collections.Generic.IEnumerable'1 if item has Roblox.Catalog.Api.CatalogItemStatus.
    #[serde(rename = "itemStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_status: Option<Vec<CatalogSearchDetailedCompleteResponseItemItemStatus>>,
    /// The Roblox.Catalog.Api.CatalogSearchDetailedResponseItem.ItemType item type. \['Asset' = 1, 'Bundle' = 2\]
    #[serde(rename = "itemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_type: Option<CatalogSearchDetailedCompleteResponseItemItemType>,
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
    pub sale_location_type: Option<CatalogSearchDetailedCompleteResponseItemSaleLocationType>,
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

/// `Roblox.Catalog.Api.SubcategoryModel`
///
/// Response model for subcategory.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SubcategoryModel {
    /// List of AssetTypeIds corresponding to AssetType enum that this category returns.
    #[serde(rename = "assetTypeIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type_ids: Option<Vec<i32>>,
    /// List of bundleTypeIds corresponding to BundleType enum that this category returns.
    #[serde(rename = "bundleTypeIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_type_ids: Option<Vec<i32>>,
    /// Subcategory name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Subcategory short name.
    #[serde(rename = "shortName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    /// Subcategory type.
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subcategory: Option<i32>,
    /// Subcategory id.
    #[serde(rename = "subcategoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subcategory_id: Option<i32>,
    /// The taxonomy UUID associated with this node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxonomy: Option<String>,
}

/// `Roblox.Catalog.Api.CategoryModel`
///
/// Response model for category.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CategoryModel {
    /// List of AssetTypeIds corresponding to AssetType enum that this category returns.
    #[serde(rename = "assetTypeIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type_ids: Option<Vec<i32>>,
    /// List of bundleTypeIds corresponding to BundleType enum that this category returns.
    #[serde(rename = "bundleTypeIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_type_ids: Option<Vec<i32>>,
    /// Category type.
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<i32>,
    /// Category id.
    #[serde(rename = "categoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<i32>,
    /// Gets or sets whether the category is searchable in search bar.
    #[serde(rename = "isSearchable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_searchable: Option<bool>,
    /// Category name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Category order index.
    #[serde(rename = "orderIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_index: Option<i32>,
    /// Subcategories under this category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subcategories: Option<Vec<SubcategoryModel>>,
    /// The associated public facing web_stable_id corresponding to internal taxonomy uuid for this category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxonomy: Option<String>,
}

/// `Roblox.Catalog.Api.FavoriteBundlesResponse`
///
/// A response containing favorited bundles and whether there are more.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FavoriteBundlesResponse {
    /// Collection of favorited bundles and associated details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favorites: Option<Vec<BundleDetailsModel>>,
    /// True if there exists a next page of favorited bundles.
    #[serde(rename = "moreFavorites")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub more_favorites: Option<bool>,
    /// Pagination cursor for the next page.
    #[serde(rename = "nextCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// Pagination cursor for the previous page.
    #[serde(rename = "previousCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_cursor: Option<String>,
}

/// `Roblox.Catalog.Api.MultigetItemDetailsRequestItem`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MultigetItemDetailsRequestItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// \['Asset' = 1, 'Bundle' = 2\]
    #[serde(rename = "itemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_type: Option<CatalogSearchDetailedCompleteResponseItemItemType>,
}

/// `Roblox.Catalog.Api.MultigetItemDetailsRequestModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MultigetItemDetailsRequestModel {
    /// The items to retrieve details for. Each endpoint has an item count limit per request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<MultigetItemDetailsRequestItem>>,
}

/// `Roblox.Catalog.Api.OwnedBundleModel`
///
/// A model to represent owned bundles.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct OwnedBundleModel {
    #[serde(rename = "bundleType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<BundleCreatorModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.Catalog.Api.TopicModel`
///
/// Response model for avatar topics.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TopicModel {
    /// The display topic name.
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The original topic name stored in the table.
    #[serde(rename = "originalTopicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_topic_name: Option<String>,
}

crate::api_enum! {
    int
    /// \['Unknown' = 1, 'Male' = 2, 'Female' = 3\]
    TopicRequestModelGenderType {
    UnknownValue = 1 => "Unknown",
    Male = 2 => "Male",
    Female = 3 => "Female",
    }
}

crate::api_enum! {
    int
    /// \['Invalid' = 0, 'Asset' = 1, 'Bundle' = 2\]
    AvatarItemItemType {
    Invalid = 0 => "Invalid",
    Asset = 1 => "Asset",
    Bundle = 2 => "Bundle",
    }
}

/// `Roblox.MarketplaceTopicDiscovery.TopicDiscoveryService.V1Beta1.AvatarItem`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AvatarItem {
    /// \['Invalid' = 0, 'Asset' = 1, 'Bundle' = 2\]
    #[serde(rename = "ItemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_type: Option<AvatarItemItemType>,
    #[serde(rename = "TargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<i64>,
}

/// `Roblox.Catalog.Api.TopicRequestModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TopicRequestModel {
    /// \['Unknown' = 1, 'Male' = 2, 'Female' = 3\]
    #[serde(rename = "genderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender_type: Option<TopicRequestModelGenderType>,
    #[serde(rename = "inputQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<AvatarItem>>,
    /// Maximum number of topic results returned from the server.
    #[serde(rename = "maxResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_result: Option<i32>,
    #[serde(rename = "selectTopics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_topics: Option<Vec<String>>,
}

/// `Roblox.MarketplaceTopicDiscovery.TopicDiscoveryService.V1Beta1.Error`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ErrorModel {
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// `Roblox.Catalog.Api.TopicResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TopicResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queries: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<TopicModel>>,
}

/// `Roblox.Web.WebAPI.ApiEmptyResponseModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiEmptyResponseModel {}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.Catalog.Api.BundleDetailsModel\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseBundleDetailsModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<BundleDetailsModel>>,
}

/// `Roblox.Web.WebAPI.Models.ApiArrayResponse\[Roblox.Catalog.Api.CatalogSearchDetailedResponseItemV2\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiArrayResponseCatalogSearchDetailedResponseItemV2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<CatalogSearchDetailedResponseItemV2>>,
}

/// `Roblox.Web.WebAPI.Models.ApiPageResponse\[Roblox.Catalog.Api.BundleDetailsModel\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiPageResponseBundleDetailsModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<BundleDetailsModel>>,
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

/// `Roblox.Web.WebAPI.Models.ApiPageResponse\[Roblox.Catalog.Api.OwnedBundleModel\]`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiPageResponseOwnedBundleModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<OwnedBundleModel>>,
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
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// The order the results are sorted in.
    pub sort_order: Option<GetAssetsByAssetIdBundlesSortOrder>,
}

/// Query, header and form inputs for [`get_bundles_details`].
#[derive(Debug, Clone, Default)]
pub struct GetBundlesDetailsParams {
    pub bundle_ids: Vec<i64>,
}

/// Query, header and form inputs for [`get_bundles_by_bundle_id_recommendations`].
#[derive(Debug, Clone, Default)]
pub struct GetBundlesByBundleIdRecommendationsParams {
    /// The number of recommended items to return.
    pub num_items: Option<i32>,
}

/// Query, header and form inputs for [`post_catalog_items_details_complete`].
#[derive(Debug, Clone, Default)]
pub struct PostCatalogItemsDetailsCompleteParams {
    pub roblox_place_id: i64,
    pub roblox_game_id: String,
}

/// Query, header and form inputs for [`get_favorites_users_by_user_id_favorites_by_asset_type_id_assets`].
#[derive(Debug, Clone, Default)]
pub struct GetFavoritesUsersByUserIdFavoritesByAssetTypeIdAssetsParams {
    /// The number of results per request.
    pub limit: Option<i32>,
    /// The paging cursor for the previous or next page.
    pub cursor: Option<String>,
    /// The order the results are sorted in.
    pub sort_order: Option<GetAssetsByAssetIdBundlesSortOrder>,
}

/// Query, header and form inputs for [`get_favorites_users_by_user_id_favorites_by_subtype_id_bundles`].
#[derive(Debug, Clone, Default)]
pub struct GetFavoritesUsersByUserIdFavoritesBySubtypeIdBundlesParams {
    pub items_per_page: Option<i32>,
    pub cursor: Option<String>,
    pub is_previous: Option<bool>,
}

/// Query, header and form inputs for [`get_users_by_user_id_bundles`].
#[derive(Debug, Clone, Default)]
pub struct GetUsersByUserIdBundlesParams {
    pub cursor: Option<String>,
    pub limit: Option<i32>,
    pub sort_order: Option<i32>,
}

crate::api_enum! {
    int
    /// \['BodyParts' = 1, 'AvatarAnimations' = 2, 'Shoes' = 3, 'DynamicHead' = 4\]
    GetUsersByUserIdBundlesByBundleTypeBundleType {
    BodyParts = 1 => "BodyParts",
    AvatarAnimations = 2 => "AvatarAnimations",
    Shoes = 3 => "Shoes",
    DynamicHead = 4 => "DynamicHead",
    }
}

crate::api_enum! {
    int
    /// \['Asc' = 1, 'Desc' = 2\]
    GetUsersByUserIdBundlesByBundleTypeSortOrder {
    Asc = 1 => "Asc",
    Desc = 2 => "Desc",
    }
}

/// Query, header and form inputs for [`get_users_by_user_id_bundles_by_bundle_type`].
#[derive(Debug, Clone, Default)]
pub struct GetUsersByUserIdBundlesByBundleTypeParams {
    pub cursor: String,
    pub limit: Option<i32>,
    /// \['Asc' = 1, 'Desc' = 2\]
    pub sort_order: Option<GetUsersByUserIdBundlesByBundleTypeSortOrder>,
}

/// `GET /v1/asset-to-category`
///
/// Lists a mapping for assets to category IDs to convert from inventory ID to catalog ID. Creates a mapping to link 'Get More' button in inventory page to the relevant catalog page.
pub fn get_asset_to_category_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/asset-to-category".to_string());
    Ok(r)
}

/// `GET /v1/asset-to-category`
///
/// Lists a mapping for assets to category IDs to convert from inventory ID to catalog ID. Creates a mapping to link 'Get More' button in inventory page to the relevant catalog page.
pub async fn get_asset_to_category(client: &Client) -> Result<HashMap<String, i64>> {
    client.send(get_asset_to_category_request()?).await?.json()
}

/// `GET /v1/asset-to-subcategory`
///
/// Lists a mapping for assets to subcategory IDs to convert from inventory ID to catalog ID. Creates a mapping to link 'Get More' button in inventory page to the relevant catalog page.
pub fn get_asset_to_subcategory_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/asset-to-subcategory".to_string());
    Ok(r)
}

/// `GET /v1/asset-to-subcategory`
///
/// Lists a mapping for assets to subcategory IDs to convert from inventory ID to catalog ID. Creates a mapping to link 'Get More' button in inventory page to the relevant catalog page.
pub async fn get_asset_to_subcategory(client: &Client) -> Result<HashMap<String, i64>> {
    client.send(get_asset_to_subcategory_request()?).await?.json()
}

/// `GET /v1/assets/{assetId}/bundles`
///
/// Lists the bundles a particular asset belongs to. Use the Id of the last bundle in the response to get the next page.
pub fn get_assets_by_asset_id_bundles_request(
    asset_id: i64,
    params: &GetAssetsByAssetIdBundlesParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/assets/{}/bundles", crate::rt::path_value(&asset_id)),
    );
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    r.query("sortOrder", &params.sort_order);
    Ok(r)
}

/// `GET /v1/assets/{assetId}/bundles`
///
/// Lists the bundles a particular asset belongs to. Use the Id of the last bundle in the response to get the next page.
pub async fn get_assets_by_asset_id_bundles(
    client: &Client,
    asset_id: i64,
    params: &GetAssetsByAssetIdBundlesParams,
) -> Result<ApiPageResponseBundleDetailsModel> {
    client
        .send(get_assets_by_asset_id_bundles_request(asset_id, params)?)
        .await?
        .json()
}

/// `GET /v1/bundles/details`
///
/// Returns details about the given bundleIds.
pub fn get_bundles_details_request(params: &GetBundlesDetailsParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/bundles/details".to_string());
    r.query("bundleIds", &params.bundle_ids);
    Ok(r)
}

/// `GET /v1/bundles/details`
///
/// Returns details about the given bundleIds.
pub async fn get_bundles_details(
    client: &Client,
    params: &GetBundlesDetailsParams,
) -> Result<Vec<BundleDetailsModel>> {
    client.send(get_bundles_details_request(params)?).await?.json()
}

/// `GET /v1/bundles/{bundleId}/details`
///
/// Returns details about the given bundleId.
pub fn get_bundles_by_bundle_id_details_request(bundle_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/bundles/{}/details", crate::rt::path_value(&bundle_id)),
    );
    Ok(r)
}

/// `GET /v1/bundles/{bundleId}/details`
///
/// Returns details about the given bundleId.
pub async fn get_bundles_by_bundle_id_details(client: &Client, bundle_id: i64) -> Result<BundleDetailsModel> {
    client
        .send(get_bundles_by_bundle_id_details_request(bundle_id)?)
        .await?
        .json()
}

/// `GET /v1/bundles/{bundleId}/recommendations`
///
/// Gets recommendations for a given bundle, bundleId of 0 returns randomized bundles - Accepts both public and authenticated users.
pub fn get_bundles_by_bundle_id_recommendations_request(
    bundle_id: i64,
    params: &GetBundlesByBundleIdRecommendationsParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/bundles/{}/recommendations",
            crate::rt::path_value(&bundle_id)
        ),
    );
    r.query("numItems", &params.num_items);
    Ok(r)
}

/// `GET /v1/bundles/{bundleId}/recommendations`
///
/// Gets recommendations for a given bundle, bundleId of 0 returns randomized bundles - Accepts both public and authenticated users.
pub async fn get_bundles_by_bundle_id_recommendations(
    client: &Client,
    bundle_id: i64,
    params: &GetBundlesByBundleIdRecommendationsParams,
) -> Result<ApiArrayResponseBundleDetailsModel> {
    client
        .send(get_bundles_by_bundle_id_recommendations_request(
            bundle_id, params,
        )?)
        .await?
        .json()
}

/// `POST /v1/catalog/items/details`
///
/// Returns details for one or more catalog items.
///
/// There is an item count limit per request. Exceeding this returns 400 Bad Request.
pub fn post_catalog_items_details_request(body: &MultigetItemDetailsRequestModel) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/catalog/items/details".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/catalog/items/details`
///
/// Returns details for one or more catalog items.
///
/// There is an item count limit per request. Exceeding this returns 400 Bad Request.
pub async fn post_catalog_items_details(
    client: &Client,
    body: &MultigetItemDetailsRequestModel,
) -> Result<ApiArrayResponseCatalogSearchDetailedResponseItemV2> {
    client
        .send(post_catalog_items_details_request(body)?)
        .await?
        .json()
}

/// `POST /v1/catalog/items/details/complete`
///
/// Returns list of item details complete with user ownership and economy information. Batch version of GetItemDetailsComplete. Items with critical errors are returned in ItemErrors with their corresponding HTTP status code (404 for not-found, 500 for other errors).
pub fn post_catalog_items_details_complete_request(
    params: &PostCatalogItemsDetailsCompleteParams,
    body: &MultigetItemDetailsRequestModel,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        "/v1/catalog/items/details/complete".to_string(),
    );
    r.header("Roblox-Place-Id", &params.roblox_place_id);
    r.header("Roblox-Game-Id", &params.roblox_game_id);
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/catalog/items/details/complete`
///
/// Returns list of item details complete with user ownership and economy information. Batch version of GetItemDetailsComplete. Items with critical errors are returned in ItemErrors with their corresponding HTTP status code (404 for not-found, 500 for other errors).
pub async fn post_catalog_items_details_complete(
    client: &Client,
    params: &PostCatalogItemsDetailsCompleteParams,
    body: &MultigetItemDetailsRequestModel,
) -> Result<BatchItemDetailsCompleteResponse> {
    client
        .send(post_catalog_items_details_complete_request(params, body)?)
        .await?
        .json()
}

/// `GET /v1/categories`
///
/// Lists Category Names and their Ids.
pub fn get_categories_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/categories".to_string());
    Ok(r)
}

/// `GET /v1/categories`
///
/// Lists Category Names and their Ids.
pub async fn get_categories(client: &Client) -> Result<Vec<CategoryModel>> {
    client.send(get_categories_request()?).await?.json()
}

/// `GET /v1/favorites/assets/{assetId}/count`
///
/// Gets the favorite count for the given asset Id.
pub fn get_favorites_assets_by_asset_id_count_request(asset_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/favorites/assets/{}/count", crate::rt::path_value(&asset_id)),
    );
    Ok(r)
}

/// `GET /v1/favorites/assets/{assetId}/count`
///
/// Gets the favorite count for the given asset Id.
pub async fn get_favorites_assets_by_asset_id_count(client: &Client, asset_id: i64) -> Result<i64> {
    client
        .send(get_favorites_assets_by_asset_id_count_request(asset_id)?)
        .await?
        .json()
}

/// `GET /v1/favorites/bundles/{bundleId}/count`
///
/// Gets the favorite count for the given bundle Id.
pub fn get_favorites_bundles_by_bundle_id_count_request(bundle_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/favorites/bundles/{}/count",
            crate::rt::path_value(&bundle_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/favorites/bundles/{bundleId}/count`
///
/// Gets the favorite count for the given bundle Id.
pub async fn get_favorites_bundles_by_bundle_id_count(client: &Client, bundle_id: i64) -> Result<i64> {
    client
        .send(get_favorites_bundles_by_bundle_id_count_request(bundle_id)?)
        .await?
        .json()
}

/// `GET /v1/favorites/users/{userId}/assets/{assetId}/favorite`
///
/// Gets the favorite model for the asset and user.
pub fn get_favorites_users_by_user_id_assets_by_asset_id_favorite_request(
    user_id: i64,
    asset_id: i64,
) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/favorites/users/{}/assets/{}/favorite",
            crate::rt::path_value(&user_id),
            crate::rt::path_value(&asset_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/favorites/users/{userId}/assets/{assetId}/favorite`
///
/// Gets the favorite model for the asset and user.
pub async fn get_favorites_users_by_user_id_assets_by_asset_id_favorite(
    client: &Client,
    user_id: i64,
    asset_id: i64,
) -> Result<AssetFavoriteModel> {
    client
        .send(get_favorites_users_by_user_id_assets_by_asset_id_favorite_request(user_id, asset_id)?)
        .await?
        .json()
}

/// `POST /v1/favorites/users/{userId}/assets/{assetId}/favorite`
///
/// Create a favorite for an asset by the authenticated user.
pub fn post_favorites_users_by_user_id_assets_by_asset_id_favorite_request(
    user_id: i64,
    asset_id: i64,
) -> Result<Request> {
    let r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/favorites/users/{}/assets/{}/favorite",
            crate::rt::path_value(&user_id),
            crate::rt::path_value(&asset_id)
        ),
    );
    Ok(r)
}

/// `POST /v1/favorites/users/{userId}/assets/{assetId}/favorite`
///
/// Create a favorite for an asset by the authenticated user.
pub async fn post_favorites_users_by_user_id_assets_by_asset_id_favorite(
    client: &Client,
    user_id: i64,
    asset_id: i64,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_favorites_users_by_user_id_assets_by_asset_id_favorite_request(user_id, asset_id)?)
        .await?
        .json()
}

/// `DELETE /v1/favorites/users/{userId}/assets/{assetId}/favorite`
///
/// Delete a favorite for an asset by the authenticated user.
pub fn delete_favorites_users_by_user_id_assets_by_asset_id_favorite_request(
    user_id: i64,
    asset_id: i64,
) -> Result<Request> {
    let r = Request::new(
        Method::DELETE,
        HOST,
        format!(
            "/v1/favorites/users/{}/assets/{}/favorite",
            crate::rt::path_value(&user_id),
            crate::rt::path_value(&asset_id)
        ),
    );
    Ok(r)
}

/// `DELETE /v1/favorites/users/{userId}/assets/{assetId}/favorite`
///
/// Delete a favorite for an asset by the authenticated user.
pub async fn delete_favorites_users_by_user_id_assets_by_asset_id_favorite(
    client: &Client,
    user_id: i64,
    asset_id: i64,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(delete_favorites_users_by_user_id_assets_by_asset_id_favorite_request(user_id, asset_id)?)
        .await?
        .json()
}

/// `GET /v1/favorites/users/{userId}/bundles/{bundleId}/favorite`
///
/// Gets the favorite model for the bundle and user.
pub fn get_favorites_users_by_user_id_bundles_by_bundle_id_favorite_request(
    user_id: i64,
    bundle_id: i64,
) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/favorites/users/{}/bundles/{}/favorite",
            crate::rt::path_value(&user_id),
            crate::rt::path_value(&bundle_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/favorites/users/{userId}/bundles/{bundleId}/favorite`
///
/// Gets the favorite model for the bundle and user.
pub async fn get_favorites_users_by_user_id_bundles_by_bundle_id_favorite(
    client: &Client,
    user_id: i64,
    bundle_id: i64,
) -> Result<BundleFavoriteModel> {
    client
        .send(get_favorites_users_by_user_id_bundles_by_bundle_id_favorite_request(user_id, bundle_id)?)
        .await?
        .json()
}

/// `POST /v1/favorites/users/{userId}/bundles/{bundleId}/favorite`
///
/// Create a favorite for the bundle by the authenticated user.
pub fn post_favorites_users_by_user_id_bundles_by_bundle_id_favorite_request(
    user_id: i64,
    bundle_id: i64,
) -> Result<Request> {
    let r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/favorites/users/{}/bundles/{}/favorite",
            crate::rt::path_value(&user_id),
            crate::rt::path_value(&bundle_id)
        ),
    );
    Ok(r)
}

/// `POST /v1/favorites/users/{userId}/bundles/{bundleId}/favorite`
///
/// Create a favorite for the bundle by the authenticated user.
pub async fn post_favorites_users_by_user_id_bundles_by_bundle_id_favorite(
    client: &Client,
    user_id: i64,
    bundle_id: i64,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_favorites_users_by_user_id_bundles_by_bundle_id_favorite_request(user_id, bundle_id)?)
        .await?
        .json()
}

/// `DELETE /v1/favorites/users/{userId}/bundles/{bundleId}/favorite`
///
/// Delete favorite for the bundle by the authenticated user.
pub fn delete_favorites_users_by_user_id_bundles_by_bundle_id_favorite_request(
    user_id: i64,
    bundle_id: i64,
) -> Result<Request> {
    let r = Request::new(
        Method::DELETE,
        HOST,
        format!(
            "/v1/favorites/users/{}/bundles/{}/favorite",
            crate::rt::path_value(&user_id),
            crate::rt::path_value(&bundle_id)
        ),
    );
    Ok(r)
}

/// `DELETE /v1/favorites/users/{userId}/bundles/{bundleId}/favorite`
///
/// Delete favorite for the bundle by the authenticated user.
pub async fn delete_favorites_users_by_user_id_bundles_by_bundle_id_favorite(
    client: &Client,
    user_id: i64,
    bundle_id: i64,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(delete_favorites_users_by_user_id_bundles_by_bundle_id_favorite_request(user_id, bundle_id)?)
        .await?
        .json()
}

/// `GET /v1/favorites/users/{userId}/favorites/{assetTypeId}/assets`
///
/// Lists the marketplace assets favorited by a given user with the given assetTypeId.
pub fn get_favorites_users_by_user_id_favorites_by_asset_type_id_assets_request(
    user_id: i64,
    asset_type_id: i32,
    params: &GetFavoritesUsersByUserIdFavoritesByAssetTypeIdAssetsParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/favorites/users/{}/favorites/{}/assets",
            crate::rt::path_value(&user_id),
            crate::rt::path_value(&asset_type_id)
        ),
    );
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    r.query("sortOrder", &params.sort_order);
    Ok(r)
}

/// `GET /v1/favorites/users/{userId}/favorites/{assetTypeId}/assets`
///
/// Lists the marketplace assets favorited by a given user with the given assetTypeId.
pub async fn get_favorites_users_by_user_id_favorites_by_asset_type_id_assets(
    client: &Client,
    user_id: i64,
    asset_type_id: i32,
    params: &GetFavoritesUsersByUserIdFavoritesByAssetTypeIdAssetsParams,
) -> Result<ApiPageResponseCatalogSearchDetailedResponseItem> {
    client
        .send(
            get_favorites_users_by_user_id_favorites_by_asset_type_id_assets_request(
                user_id,
                asset_type_id,
                params,
            )?,
        )
        .await?
        .json()
}

/// `GET /v1/favorites/users/{userId}/favorites/{subtypeId}/bundles`
///
/// Lists the bundles favorited by a given user with the given bundle subtypeId.Switched to EAAS style pagination cursors since July 2024.
pub fn get_favorites_users_by_user_id_favorites_by_subtype_id_bundles_request(
    user_id: i64,
    subtype_id: i32,
    params: &GetFavoritesUsersByUserIdFavoritesBySubtypeIdBundlesParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/favorites/users/{}/favorites/{}/bundles",
            crate::rt::path_value(&user_id),
            crate::rt::path_value(&subtype_id)
        ),
    );
    r.query("itemsPerPage", &params.items_per_page);
    r.query("cursor", &params.cursor);
    r.query("isPrevious", &params.is_previous);
    Ok(r)
}

/// `GET /v1/favorites/users/{userId}/favorites/{subtypeId}/bundles`
///
/// Lists the bundles favorited by a given user with the given bundle subtypeId.Switched to EAAS style pagination cursors since July 2024.
pub async fn get_favorites_users_by_user_id_favorites_by_subtype_id_bundles(
    client: &Client,
    user_id: i64,
    subtype_id: i32,
    params: &GetFavoritesUsersByUserIdFavoritesBySubtypeIdBundlesParams,
) -> Result<FavoriteBundlesResponse> {
    client
        .send(
            get_favorites_users_by_user_id_favorites_by_subtype_id_bundles_request(
                user_id, subtype_id, params,
            )?,
        )
        .await?
        .json()
}

/// `GET /v1/subcategories`
///
/// Lists Subcategory Names and their Ids.
pub fn get_subcategories_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/subcategories".to_string());
    Ok(r)
}

/// `GET /v1/subcategories`
///
/// Lists Subcategory Names and their Ids.
pub async fn get_subcategories(client: &Client) -> Result<HashMap<String, i64>> {
    client.send(get_subcategories_request()?).await?.json()
}

/// `POST /v1/topic/get-topics`
///
/// Gets topics for a populated request, or trending queries when the request has no topic inputs. Topics and queries are mutually exclusive in the response.
pub fn post_topic_get_topics_request(body: &TopicRequestModel) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/topic/get-topics".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/topic/get-topics`
///
/// Gets topics for a populated request, or trending queries when the request has no topic inputs. Topics and queries are mutually exclusive in the response.
pub async fn post_topic_get_topics(client: &Client, body: &TopicRequestModel) -> Result<TopicResponse> {
    client.send(post_topic_get_topics_request(body)?).await?.json()
}

/// `GET /v1/users/{userId}/bundles`
///
/// Lists the bundles owned by a given user.
pub fn get_users_by_user_id_bundles_request(
    user_id: i64,
    params: &GetUsersByUserIdBundlesParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/users/{}/bundles", crate::rt::path_value(&user_id)),
    );
    r.query("cursor", &params.cursor);
    r.query("limit", &params.limit);
    r.query("sortOrder", &params.sort_order);
    Ok(r)
}

/// `GET /v1/users/{userId}/bundles`
///
/// Lists the bundles owned by a given user.
pub async fn get_users_by_user_id_bundles(
    client: &Client,
    user_id: i64,
    params: &GetUsersByUserIdBundlesParams,
) -> Result<ApiPageResponseOwnedBundleModel> {
    client
        .send(get_users_by_user_id_bundles_request(user_id, params)?)
        .await?
        .json()
}

/// `GET /v1/users/{userId}/bundles/{bundleType}`
pub fn get_users_by_user_id_bundles_by_bundle_type_request(
    user_id: i64,
    bundle_type: GetUsersByUserIdBundlesByBundleTypeBundleType,
    params: &GetUsersByUserIdBundlesByBundleTypeParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/users/{}/bundles/{}",
            crate::rt::path_value(&user_id),
            crate::rt::path_value(&bundle_type)
        ),
    );
    r.query("cursor", &params.cursor);
    r.query("limit", &params.limit);
    r.query("sortOrder", &params.sort_order);
    Ok(r)
}

/// `GET /v1/users/{userId}/bundles/{bundleType}`
pub async fn get_users_by_user_id_bundles_by_bundle_type(
    client: &Client,
    user_id: i64,
    bundle_type: GetUsersByUserIdBundlesByBundleTypeBundleType,
    params: &GetUsersByUserIdBundlesByBundleTypeParams,
) -> Result<ApiPageResponseOwnedBundleModel> {
    client
        .send(get_users_by_user_id_bundles_by_bundle_type_request(
            user_id,
            bundle_type,
            params,
        )?)
        .await?
        .json()
}

//! Marketplace: categories, search, item pages (details, resellers, price history, owners,
//! recommendations, bundle contents), favorites, which of your accounts own an item, and a
//! local wishlist.
//!
//! Buying is in `purchase.rs`.

use rbx_api::generated::{apis, catalog, economy, inventory};
use rbx_api::{AccountId, Error};
use serde::{Deserialize, Serialize};

use crate::Core;
use crate::cache::{Kind, now};
use crate::fmt;
use crate::res::{Key, Page, Res};

const CATEGORIES_AGE: i64 = 86_400;
const SEARCH_AGE: i64 = 300;
const ITEM_AGE: i64 = 120;
const LIST_AGE: i64 = 300;
const HISTORY_AGE: i64 = 3_600;
const OWN_AGE: i64 = 600;

// ---------------------------------------------------------------- items

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ItemKind {
    #[default]
    Asset,
    Bundle,
}

impl ItemKind {
    pub fn api_name(self) -> &'static str {
        match self {
            ItemKind::Asset => "Asset",
            ItemKind::Bundle => "Bundle",
        }
    }
    /// The number inventory's is-owned endpoint takes (0 Asset, 3 Bundle; checked live).
    fn owned_type(self) -> i32 {
        match self {
            ItemKind::Asset => 0,
            ItemKind::Bundle => 3,
        }
    }
}

/// An asset or bundle id.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ItemRef {
    pub kind: ItemKind,
    pub id: u64,
}

impl ItemRef {
    pub fn asset(id: u64) -> ItemRef {
        ItemRef {
            kind: ItemKind::Asset,
            id,
        }
    }
    pub fn bundle(id: u64) -> ItemRef {
        ItemRef {
            kind: ItemKind::Bundle,
            id,
        }
    }
}

/// What kind of limited an item is, if any.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Limited {
    #[default]
    No,
    /// Classic limited (shared stock).
    Limited,
    /// Classic limited unique (numbered copies).
    LimitedU,
    /// A resellable collectible (UGC limited).
    Collectible,
}

impl Limited {
    pub fn label(self) -> Option<&'static str> {
        match self {
            Limited::No => None,
            Limited::Limited => Some("Limited"),
            Limited::LimitedU => Some("Limited U"),
            Limited::Collectible => Some("Collectible"),
        }
    }
    pub fn resellable(self) -> bool {
        self != Limited::No
    }
}

/// The account that publishes Roblox's own items.
pub const ROBLOX_USER_ID: u64 = 1;

/// Whether a name reads as "Roblox" once decoration (symbols, spaces, accents) is dropped and
/// lookalike letters and digits are folded: "Roblox°°°", "R0BL0X", "Rоblox" with a Cyrillic о.
/// A creator whose name passes for Roblox but isn't Roblox's account.
pub(crate) fn fake_roblox(name: &str, id: u64, is_group: bool) -> bool {
    let official = id == ROBLOX_USER_ID && !is_group;
    !official && reads_as_roblox(name)
}

fn reads_as_roblox(name: &str) -> bool {
    let folded: String = name
        .chars()
        .filter_map(|c| match c {
            '0' | 'о' | 'О' | 'ο' | 'Ο' | 'σ' => Some('o'),
            '1' | 'i' | 'I' | '|' | 'ӏ' | 'І' | 'і' | 'Ι' | 'ι' => Some('l'),
            'х' | 'Х' | 'χ' | 'Χ' => Some('x'),
            'В' | 'Β' | 'в' | 'β' => Some('b'),
            'г' | 'Г' => Some('r'),
            c if c.is_ascii_alphabetic() => Some(c.to_ascii_lowercase()),
            _ => None,
        })
        .collect();
    folded == "roblox"
}

/// The fields a catalog tile needs. Search results, recommendations and the wishlist all give these.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CatalogItem {
    pub item: ItemRef,
    pub name: String,
    pub creator: String,
    pub creator_id: u64,
    pub creator_is_group: bool,
    pub creator_verified: bool,
    /// List price; `None` when it has none (off sale, not for sale).
    pub price: Option<i64>,
    /// Cheapest resale listing, for limiteds and collectibles.
    pub lowest_resale: Option<i64>,
    /// Text the site shows instead of a price ("Free", "Off Sale").
    pub price_status: String,
    pub off_sale: bool,
    pub limited: Limited,
    pub favorites: i64,
    pub asset_type: i32,
    pub bundle_type: i32,
    pub collectible_id: String,
    /// Copies in total and copies still for sale, for limited stock items.
    pub total_quantity: i64,
    pub units_left: i64,
    pub premium_only: bool,
}

impl CatalogItem {
    /// The price someone pays right now: resale for limiteds that are off sale, else the list price.
    pub fn buy_price(&self) -> Option<i64> {
        self.buy_offer().map(|(_, p)| p)
    }

    /// Who "Buy" buys from and for how much, like the site: the creator while they sell it and
    /// have stock, else the cheapest reseller; whichever is cheaper when both.
    pub fn buy_offer(&self) -> Option<(crate::purchase::Seller, i64)> {
        use crate::purchase::Seller;
        let sold_out = self.total_quantity > 0 && self.units_left == 0;
        let creator = self.price.filter(|_| !self.off_sale && !sold_out);
        let resale = self.lowest_resale.filter(|p| *p > 0 && self.limited.resellable());
        match (creator, resale) {
            (Some(p), Some(r)) if r < p => Some((Seller::Cheapest, r)),
            (Some(p), _) => Some((Seller::Creator, p)),
            (None, Some(r)) => Some((Seller::Cheapest, r)),
            (None, None) => None,
        }
    }

    /// Whether an account that owns it can buy another copy. Only limiteds and collectibles
    /// come in copies; Roblox refuses anything else with `QuantityLimitExceeded`.
    pub fn buy_again(&self) -> bool {
        self.limited.resellable()
    }

    /// "R$ 1,234", "Free" or "Off sale".
    pub fn price_label(&self) -> String {
        match self.buy_price() {
            Some(0) => "Free".into(),
            Some(p) => fmt::robux(p),
            None if !self.price_status.is_empty() && self.price_status != "Off Sale" => {
                self.price_status.clone()
            }
            None => "Off sale".into(),
        }
    }

    /// Named to pass as Roblox's own account (user 1), like "Roblox°°°" or "R0BL0X".
    pub fn fake_roblox_creator(&self) -> bool {
        fake_roblox(&self.creator, self.creator_id, self.creator_is_group)
    }

    /// "Hat", "Hair", "Body bundle"...
    pub fn type_name(&self) -> &'static str {
        match self.item.kind {
            ItemKind::Asset => asset_type_name(self.asset_type),
            ItemKind::Bundle => bundle_type_name(self.bundle_type),
        }
    }

    pub(crate) fn from_search(x: catalog::v2::CatalogSearchDetailedResponseItemV2) -> Option<CatalogItem> {
        use catalog::v2::CatalogSearchDetailedResponseItemItemRestrictions as R;
        use catalog::v2::CatalogSearchDetailedResponseItemItemType as T;
        let kind = match x.item_type? {
            T::Asset => ItemKind::Asset,
            T::Bundle => ItemKind::Bundle,
            _ => return None,
        };
        let r = x.item_restrictions.unwrap_or_default();
        let limited = if r.contains(&R::LimitedUnique) {
            Limited::LimitedU
        } else if r.contains(&R::Limited) {
            Limited::Limited
        } else if r.contains(&R::Collectible) {
            Limited::Collectible
        } else {
            Limited::No
        };
        let status = x.price_status.unwrap_or_default();
        Some(CatalogItem {
            item: ItemRef {
                kind,
                id: x.id? as u64,
            },
            name: x.name.unwrap_or_default(),
            creator: x.creator_name.unwrap_or_default(),
            creator_id: x.creator_target_id.unwrap_or(0).max(0) as u64,
            // creatorType: 1 User, 2 Group.
            creator_is_group: x.creator_type == Some(2),
            creator_verified: x.creator_has_verified_badge.unwrap_or(false),
            price: x.price.filter(|_| status != "Off Sale"),
            lowest_resale: x.lowest_resale_price.or(x.lowest_price).filter(|p| *p > 0),
            off_sale: x.is_off_sale.unwrap_or(status == "Off Sale"),
            price_status: status,
            limited,
            favorites: x.favorite_count.unwrap_or(0),
            asset_type: x.asset_type.unwrap_or(0),
            bundle_type: x.bundle_type.unwrap_or(0),
            collectible_id: x.collectible_item_id.unwrap_or_default(),
            total_quantity: x.total_quantity.unwrap_or(0),
            units_left: x.units_available_for_consumption.unwrap_or(0),
            premium_only: r.iter().any(|x| {
                matches!(
                    x,
                    R::BuildersClub | R::TurboBuildersClub | R::OutrageousBuildersClub
                )
            }),
        })
    }
}

/// Asset type names for display. From Roblox's AssetType enum.
pub fn asset_type_name(t: i32) -> &'static str {
    match t {
        1 => "Image",
        2 => "Classic T-shirt",
        3 => "Audio",
        4 => "Mesh",
        5 => "Script",
        8 => "Hat",
        9 => "Place",
        10 => "Model",
        11 => "Classic shirt",
        12 => "Classic pants",
        13 => "Decal",
        17 => "Head",
        18 => "Face",
        19 => "Gear",
        21 => "Badge",
        24 => "Animation",
        27..=31 => "Body part",
        32 => "Package",
        34 => "Pass",
        38 => "Plugin",
        40 => "Mesh part",
        41 => "Hair",
        42 => "Face accessory",
        43 => "Neck accessory",
        44 => "Shoulder accessory",
        45 => "Front accessory",
        46 => "Back accessory",
        47 => "Waist accessory",
        48..=56 => "Avatar animation",
        61 => "Emote",
        62 => "Video",
        64 => "T-shirt",
        65 => "Shirt",
        66 => "Pants",
        67 => "Jacket",
        68 => "Sweater",
        69 => "Shorts",
        70 => "Left shoe",
        71 => "Right shoe",
        72 => "Dress or skirt",
        73 => "Font family",
        76 => "Eyebrows",
        77 => "Eyelashes",
        78 => "Mood",
        79 => "Head",
        88 => "Face makeup",
        89 => "Lip makeup",
        90 => "Eye makeup",
        92 => "Background",
        _ => "Item",
    }
}

pub fn bundle_type_name(t: i32) -> &'static str {
    match t {
        1 => "Body",
        2 => "Animation pack",
        3 => "Shoes",
        4 | 5 => "Head",
        _ => "Bundle",
    }
}

// ---------------------------------------------------------------- categories and search

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Category {
    pub name: String,
    /// What search takes to narrow to this category.
    pub taxonomy: String,
    pub subcategories: Vec<Subcategory>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Subcategory {
    pub name: String,
    pub taxonomy: String,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CatalogSort {
    #[default]
    Relevance,
    Favorited,
    Bestselling,
    Newest,
    PriceLow,
    PriceHigh,
}

impl CatalogSort {
    pub const ALL: [CatalogSort; 6] = [
        CatalogSort::Relevance,
        CatalogSort::Favorited,
        CatalogSort::Bestselling,
        CatalogSort::Newest,
        CatalogSort::PriceLow,
        CatalogSort::PriceHigh,
    ];
    pub fn label(self) -> &'static str {
        match self {
            CatalogSort::Relevance => "Relevance",
            CatalogSort::Favorited => "Most favorited",
            CatalogSort::Bestselling => "Bestselling this week",
            CatalogSort::Newest => "Recently created",
            CatalogSort::PriceLow => "Price: low to high",
            CatalogSort::PriceHigh => "Price: high to low",
        }
    }
    pub fn key(self) -> &'static str {
        match self {
            CatalogSort::Relevance => "relevance",
            CatalogSort::Favorited => "favorited",
            CatalogSort::Bestselling => "bestselling",
            CatalogSort::Newest => "newest",
            CatalogSort::PriceLow => "price_low",
            CatalogSort::PriceHigh => "price_high",
        }
    }
    pub fn from_key(k: &str) -> CatalogSort {
        CatalogSort::ALL
            .into_iter()
            .find(|s| s.key() == k)
            .unwrap_or_default()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SalesType {
    #[default]
    All,
    /// Limiteds and resellable collectibles.
    Limited,
    Premium,
}

impl SalesType {
    pub const ALL: [SalesType; 3] = [SalesType::All, SalesType::Limited, SalesType::Premium];
    pub fn label(self) -> &'static str {
        match self {
            SalesType::All => "All items",
            SalesType::Limited => "Limited and collectible",
            SalesType::Premium => "Premium",
        }
    }
    pub fn key(self) -> &'static str {
        match self {
            SalesType::All => "all",
            SalesType::Limited => "limited",
            SalesType::Premium => "premium",
        }
    }
    pub fn from_key(k: &str) -> SalesType {
        SalesType::ALL
            .into_iter()
            .find(|s| s.key() == k)
            .unwrap_or_default()
    }
}

/// One marketplace search.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CatalogQuery {
    pub taxonomy: String,
    pub keyword: String,
    pub sort: CatalogSort,
    pub sales: SalesType,
    pub min_price: Option<u32>,
    pub max_price: Option<u32>,
    pub creator: String,
    pub include_off_sale: bool,
}

impl CatalogQuery {
    /// Stable cache id.
    pub fn id(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}",
            self.taxonomy,
            self.keyword.trim().to_lowercase(),
            self.sort.key(),
            self.sales.key(),
            self.min_price.map(|p| p.to_string()).unwrap_or_default(),
            self.max_price.map(|p| p.to_string()).unwrap_or_default(),
            self.creator.trim().to_lowercase(),
            self.include_off_sale,
        )
    }

    /// The query an [`id`](Self::id) came from. The keyword comes back lowercased, which
    /// searches the same.
    pub fn from_id(id: &str) -> Option<CatalogQuery> {
        let (taxonomy, rest) = id.split_once('|')?;
        // The keyword may hold `|`; the six fields after it don't.
        let mut tail: Vec<&str> = rest.rsplitn(7, '|').collect();
        if tail.len() != 7 {
            return None;
        }
        let keyword = tail.pop()?;
        let [off_sale, creator, max, min, sales, sort] = tail[..] else {
            return None;
        };
        let price = |s: &str| {
            if s.is_empty() {
                Ok(None)
            } else {
                s.parse().map(Some)
            }
        };
        Some(CatalogQuery {
            taxonomy: taxonomy.to_string(),
            keyword: keyword.to_string(),
            sort: CatalogSort::from_key(sort),
            sales: SalesType::from_key(sales),
            min_price: price(min).ok()?,
            max_price: price(max).ok()?,
            creator: creator.to_string(),
            include_off_sale: off_sale.parse().ok()?,
        })
    }

    /// How many filters (beyond category, keyword and sort) are set.
    pub fn filter_count(&self) -> usize {
        usize::from(self.sales != SalesType::All)
            + usize::from(self.min_price.is_some() || self.max_price.is_some())
            + usize::from(!self.creator.trim().is_empty())
            + usize::from(self.include_off_sale)
    }

    fn params(&self, cursor: Option<String>) -> catalog::v2::GetSearchItemsDetailsParams {
        use catalog::v2::{
            GetSearchItemsDetailsSalesTypeFilter as Sales, GetSearchItemsDetailsSortAggregation as Agg,
            GetSearchItemsDetailsSortType as Sort,
        };
        let (sort, agg) = match self.sort {
            CatalogSort::Relevance => (Sort::Relevance, None),
            CatalogSort::Favorited => (Sort::Favorited, Some(Agg::AllTime)),
            CatalogSort::Bestselling => (Sort::Sales, Some(Agg::PastWeek)),
            CatalogSort::Newest => (Sort::Created, None),
            CatalogSort::PriceLow => (Sort::PriceAsc, None),
            CatalogSort::PriceHigh => (Sort::PriceDesc, None),
        };
        let some = |s: &str| Some(s.trim().to_string()).filter(|s| !s.is_empty());
        catalog::v2::GetSearchItemsDetailsParams {
            taxonomy: some(&self.taxonomy),
            keyword: some(&self.keyword),
            sort_type: Some(sort),
            sort_aggregation: agg,
            sales_type_filter: Some(match self.sales {
                SalesType::All => Sales::All,
                SalesType::Limited => Sales::Collectibles,
                SalesType::Premium => Sales::Premium,
            }),
            min_price: self.min_price.map(|p| p as i32),
            max_price: self.max_price.map(|p| p as i32),
            creator_name: some(&self.creator),
            include_not_for_sale: self.include_off_sale.then_some(true),
            limit: Some(SEARCH_PAGE),
            cursor,
            ..Default::default()
        }
    }
}

/// One of the sizes catalog search accepts (10, 28, 30, 50, 60, 100, 120).
const SEARCH_PAGE: i32 = 60;

// ---------------------------------------------------------------- item page

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct BundlePart {
    pub id: u64,
    pub name: String,
    /// An asset of this type; 0 for the bundle's outfit.
    pub asset_type: i32,
}

/// Everything the item page header shows.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ItemDetails {
    pub card: CatalogItem,
    pub description: String,
    pub created: String,
    #[serde(default)]
    pub updated: String,
    /// The marketplace category ("Head Accessories").
    pub category: String,
    #[serde(default)]
    pub genres: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub off_sale_deadline: String,
    pub has_resellers: bool,
    pub contents: Vec<BundlePart>,
}

impl ItemDetails {
    fn from_api(x: catalog::undocumented::ItemDetails) -> Option<ItemDetails> {
        let kind = match x.item_type.as_deref() {
            Some("Bundle") => ItemKind::Bundle,
            _ => ItemKind::Asset,
        };
        let r = x.item_restrictions.unwrap_or_default();
        let has = |s: &str| r.iter().any(|x| x == s);
        let limited = if has("LimitedUnique") {
            Limited::LimitedU
        } else if has("Limited") {
            Limited::Limited
        } else if has("Collectible") {
            Limited::Collectible
        } else {
            Limited::No
        };
        let status = x.price_status.unwrap_or_default();
        let off_sale = x.is_off_sale.unwrap_or(status == "Off Sale");
        let card = CatalogItem {
            item: ItemRef {
                kind,
                id: x.id? as u64,
            },
            name: x.name.unwrap_or_default(),
            creator: x.creator_name.unwrap_or_default(),
            creator_id: x.creator_target_id.unwrap_or(0).max(0) as u64,
            creator_is_group: x.creator_type.as_deref() == Some("Group"),
            creator_verified: x.creator_has_verified_badge.unwrap_or(false),
            price: x.price.filter(|_| !off_sale),
            lowest_resale: x.lowest_resale_price.or(x.lowest_price).filter(|p| *p > 0),
            price_status: status,
            off_sale,
            limited,
            favorites: x.favorite_count.unwrap_or(0),
            asset_type: x.asset_type.unwrap_or(0),
            bundle_type: x.bundle_type.unwrap_or(0),
            collectible_id: x.collectible_item_id.unwrap_or_default(),
            total_quantity: x.total_quantity.unwrap_or(0),
            units_left: x.units_available_for_consumption.unwrap_or(0),
            premium_only: has("BuildersClub"),
        };
        let taxonomy_names: Vec<String> = x
            .taxonomy
            .unwrap_or_default()
            .into_iter()
            .filter_map(|t| t.taxonomy_name)
            .filter(|n| !n.trim().is_empty())
            .collect();
        let category = taxonomy_names.first().cloned().unwrap_or_default();
        let genres: Vec<String> = x
            .genres
            .unwrap_or_default()
            .into_iter()
            .filter(|g| !g.trim().is_empty() && g != "All")
            .collect();
        let bundled = x.bundled_items.unwrap_or_default();
        let head_shapes = x.supports_head_shapes == Some(true)
            || bundled.iter().any(|b| b.supports_head_shapes == Some(true));
        let mut tags = Vec::new();
        let mut push_tag = |t: &str| {
            let t = t.trim();
            if !t.is_empty() && !tags.iter().any(|e: &String| e.eq_ignore_ascii_case(t)) {
                tags.push(t.to_string());
            }
        };
        for name in &taxonomy_names {
            push_tag(name);
        }
        for prop in x.render_properties.unwrap_or_default() {
            match prop.as_str() {
                "AdvancedMaterials" => push_tag("Advanced Materials"),
                "Emissive" => push_tag("Emissive"),
                other => push_tag(other),
            }
        }
        if head_shapes {
            push_tag("Adjustable head");
        }
        if has("Live") {
            push_tag("Animated");
        }
        if has("ThirteenPlus") {
            push_tag("13+");
        }
        if limited.resellable() {
            push_tag("Tradable");
        }
        let created = x.item_created_utc.unwrap_or_default();
        let updated = x.item_updated_utc.unwrap_or_default();
        Some(ItemDetails {
            card,
            description: x.description.unwrap_or_default(),
            created,
            updated,
            category,
            genres,
            tags,
            off_sale_deadline: x.off_sale_deadline.unwrap_or_default(),
            has_resellers: x.has_resellers.unwrap_or(false),
            contents: bundled
                .into_iter()
                .filter(|b| b.type_.as_deref() == Some("Asset"))
                .filter_map(|b| {
                    Some(BundlePart {
                        id: b.id? as u64,
                        name: b.name.unwrap_or_default(),
                        asset_type: b.asset_type.unwrap_or(0),
                    })
                })
                .collect(),
        })
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Reseller {
    /// The listed copy (`collectibleItemInstanceId`), to buy exactly this one.
    #[serde(default)]
    pub instance_id: String,
    pub seller_id: u64,
    #[serde(default)]
    pub seller_is_group: bool,
    pub seller_name: String,
    pub seller_verified: bool,
    pub price: i64,
    pub serial: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Owner {
    /// `None` when the owner hides their inventory.
    pub user_id: Option<u64>,
    pub name: String,
    pub serial: Option<i64>,
    /// When this copy last changed hands.
    pub since: String,
}

/// Daily resale averages, oldest first.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PriceHistory {
    pub rap: Option<i64>,
    /// (unix seconds, average price)
    pub prices: Vec<(i64, f64)>,
    /// (unix seconds, copies sold)
    pub volume: Vec<(i64, f64)>,
}

/// An item someone wants, kept locally per account.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WishItem {
    pub item: ItemRef,
    pub name: String,
    /// What it cost when added, to show drops.
    pub price_then: Option<i64>,
    pub added: i64,
}

impl Core {
    /// Marketplace categories, in the site's order.
    pub fn catalog_categories(&self) -> Res<Vec<Category>> {
        self.inner.res(
            Key::shared("catalog_categories", ""),
            CATEGORIES_AGE,
            |f| async move {
                let r = catalog::undocumented::get_navigation_menu_items(&f.client).await?;
                let mut cats = r.categories.unwrap_or_default();
                cats.sort_by_key(|c| c.order_index.unwrap_or(i32::MAX));
                Ok(cats
                    .into_iter()
                    .filter(|c| c.is_searchable != Some(false))
                    .map(|c| Category {
                        name: c.name.unwrap_or_default(),
                        taxonomy: c.taxonomy.unwrap_or_default(),
                        subcategories: c
                            .subcategories
                            .unwrap_or_default()
                            .into_iter()
                            .map(|s| Subcategory {
                                name: s
                                    .short_name
                                    .filter(|n| !n.is_empty())
                                    .or(s.name)
                                    .unwrap_or_default(),
                                taxonomy: s.taxonomy.unwrap_or_default(),
                            })
                            .collect(),
                    })
                    .collect())
            },
        )
    }

    /// One page-accumulating marketplace search.
    pub fn catalog_search(&self, q: &CatalogQuery, more: bool) -> Res<Page<CatalogItem>> {
        let q = q.clone();
        self.inner.paged(
            Key::shared("catalog", q.id()),
            SEARCH_AGE,
            more,
            move |f, cursor| {
                let p = q.params(cursor);
                async move {
                    let r = catalog::v2::get_search_items_details(&f.client, &p).await?;
                    Ok(Page {
                        items: r
                            .data
                            .unwrap_or_default()
                            .into_iter()
                            .filter_map(CatalogItem::from_search)
                            .collect(),
                        next: r.next_page_cursor.filter(|c| !c.is_empty()),
                    })
                }
            },
        )
    }

    /// The item page's details.
    pub fn item(&self, item: ItemRef) -> Res<ItemDetails> {
        self.inner.res(
            Key::shared("item", format!("{:?}:{}", item.kind, item.id)),
            ITEM_AGE,
            move |f| async move {
                let p = catalog::undocumented::GetItemDetailsParams {
                    item_type: item.kind.api_name().into(),
                };
                let r = catalog::undocumented::get_item_details(&f.client, item.id as i64, &p).await?;
                let mut d = ItemDetails::from_api(r)
                    .ok_or_else(|| Error::Message("This item doesn't exist.".into()))?;
                if item.kind == ItemKind::Asset
                    && (d.updated.is_empty() || d.created.is_empty())
                    && let Ok(eco) =
                        economy::undocumented::get_asset_economy_details(&f.client, item.id as i64).await
                {
                    if d.updated.is_empty() {
                        d.updated = eco.updated.unwrap_or_default();
                    }
                    if d.created.is_empty() {
                        d.created = eco.created.unwrap_or_default();
                    }
                }
                if d.updated.is_empty() {
                    d.updated = d.created.clone();
                }
                let topic_req = catalog::v1::TopicRequestModel {
                    gender_type: Some(catalog::v1::TopicRequestModelGenderType::UnknownValue),
                    input_query: Some(String::new()),
                    items: Some(vec![catalog::v1::AvatarItem {
                        item_type: Some(match item.kind {
                            ItemKind::Asset => catalog::v1::AvatarItemItemType::Asset,
                            ItemKind::Bundle => catalog::v1::AvatarItemItemType::Bundle,
                        }),
                        target_id: Some(item.id as i64),
                    }]),
                    max_result: Some(10),
                    select_topics: Some(Vec::new()),
                };
                if let Ok(topics) = catalog::v1::post_topic_get_topics(&f.client, &topic_req).await {
                    for t in topics.topics.unwrap_or_default() {
                        if t.original_topic_name.as_deref().unwrap_or("").is_empty()
                            && let Some(name) = t.display_name
                        {
                            let name = name.trim();
                            if !name.is_empty() && !d.tags.iter().any(|e| e.eq_ignore_ascii_case(name)) {
                                d.tags.push(name.to_string());
                            }
                        }
                    }
                }
                Ok(d)
            },
        )
    }

    /// Current tile data for a list of items (wishlist, recommendations). Unknown ids drop out.
    pub fn items(&self, items: &[ItemRef]) -> Res<Vec<CatalogItem>> {
        let items = items.to_vec();
        let id = items
            .iter()
            .map(|i| format!("{:?}{}", i.kind, i.id))
            .collect::<Vec<_>>()
            .join(",");
        self.inner
            .res(Key::shared("items", id), ITEM_AGE, move |f| async move {
                fetch_items(&f.client, &items).await
            })
    }

    /// Items like this one.
    pub fn item_recommendations(&self, item: ItemRef, asset_type: i32) -> Res<Vec<CatalogItem>> {
        self.inner.res(
            Key::shared("item_recs", format!("{:?}:{}:{}", item.kind, item.id, asset_type)),
            LIST_AGE,
            move |f| async move {
                let refs: Vec<ItemRef> = match item.kind {
                    ItemKind::Asset => {
                        let p = catalog::undocumented::GetAssetRecommendationsParams {
                            asset_type_id: asset_type,
                            context_asset_id: item.id as i64,
                            num_items: Some(30),
                        };
                        catalog::undocumented::get_asset_recommendations(&f.client, &p)
                            .await?
                            .data
                            .unwrap_or_default()
                            .into_iter()
                            .filter(|i| *i > 0)
                            .map(|i| ItemRef::asset(i as u64))
                            .collect()
                    }
                    ItemKind::Bundle => {
                        let p =
                            catalog::v1::GetBundlesByBundleIdRecommendationsParams { num_items: Some(30) };
                        catalog::v1::get_bundles_by_bundle_id_recommendations(&f.client, item.id as i64, &p)
                            .await?
                            .data
                            .unwrap_or_default()
                            .into_iter()
                            .filter_map(|b| b.id)
                            .map(|i| ItemRef::bundle(i as u64))
                            .collect()
                    }
                };
                fetch_items(&f.client, &refs).await
            },
        )
    }

    /// Resale listings, cheapest first.
    pub fn item_resellers(&self, collectible_id: &str, more: bool) -> Res<Page<Reseller>> {
        let id = collectible_id.to_string();
        self.inner
            .paged(Key::shared("resellers", &id), ITEM_AGE, more, move |f, cursor| {
                let id = id.clone();
                async move {
                    let p = apis::undocumented::GetItemResellersParams {
                        limit: Some(30),
                        cursor,
                    };
                    let r = apis::undocumented::get_item_resellers(&f.client, &id, &p).await?;
                    Ok(Page {
                        items: r
                            .data
                            .unwrap_or_default()
                            .into_iter()
                            .map(|x| {
                                let s = x.seller.unwrap_or_default();
                                Reseller {
                                    instance_id: x.collectible_item_instance_id.unwrap_or_default(),
                                    seller_id: s.seller_id.unwrap_or(0).max(0) as u64,
                                    seller_is_group: s.seller_type.as_deref() == Some("Group"),
                                    seller_name: s.name.unwrap_or_default(),
                                    seller_verified: s.has_verified_badge.unwrap_or(false),
                                    price: x.price.unwrap_or(0),
                                    serial: x.serial_number,
                                }
                            })
                            .collect(),
                        next: r.next_page_cursor.filter(|c| !c.is_empty()),
                    })
                }
            })
    }

    /// Resale price and volume by day.
    pub fn item_price_history(&self, collectible_id: &str) -> Res<PriceHistory> {
        let id = collectible_id.to_string();
        self.inner.res(
            Key::shared("price_history", &id),
            HISTORY_AGE,
            move |f| async move {
                let r = apis::undocumented::get_item_resale_data(&f.client, &id).await?;
                let series = |v: Option<Vec<apis::undocumented::ResalePoint>>| {
                    let mut s: Vec<(i64, f64)> = v
                        .unwrap_or_default()
                        .into_iter()
                        .filter_map(|p| Some((fmt::parse_iso(p.date.as_deref()?)?, p.value?)))
                        .collect();
                    s.sort_by_key(|p| p.0);
                    s
                };
                Ok(PriceHistory {
                    rap: r
                        .recent_average_price
                        .map(|p| p.round() as i64)
                        .filter(|p| *p > 0),
                    prices: series(r.price_data_points),
                    volume: series(r.volume_data_points),
                })
            },
        )
    }

    /// Copies of an asset and who has them. Signed out, owners are hidden.
    pub fn item_owners(&self, account: AccountId, asset_id: u64, more: bool) -> Res<Page<Owner>> {
        self.inner.paged(
            Key::private(account, "item_owners", asset_id),
            LIST_AGE,
            more,
            move |f, cursor| async move {
                let p = inventory::v2::GetAssetsByAssetIdOwnersParams {
                    limit: Some(100),
                    cursor,
                    sort_order: Some(inventory::v2::GetAssetsByAssetIdOwnersSortOrder::Asc),
                };
                let r = inventory::v2::get_assets_by_asset_id_owners(&f.client, asset_id as i64, &p).await?;
                Ok(Page {
                    items: r
                        .data
                        .unwrap_or_default()
                        .into_iter()
                        .map(|o| {
                            let who = o.owner.unwrap_or_default();
                            Owner {
                                user_id: who.id.filter(|i| *i > 0).map(|i| i as u64),
                                name: who.name.unwrap_or_default(),
                                serial: o.serial_number.filter(|s| *s > 0),
                                since: o.updated.or(o.created).unwrap_or_default(),
                            }
                        })
                        .collect(),
                    next: r.next_page_cursor.filter(|c| !c.is_empty()),
                })
            },
        )
    }

    /// Whether `account` owns the item. Asked for every signed-in account on an item page.
    pub fn owns(&self, account: AccountId, item: ItemRef) -> Res<bool> {
        if account.is_anonymous() {
            return Res::default();
        }
        self.inner.res(
            Key::private(account, "owns", format!("{:?}:{}", item.kind, item.id)),
            OWN_AGE,
            move |f| async move {
                inventory::v1::get_users_by_user_id_items_by_item_type_by_item_target_id_is_owned(
                    &f.client,
                    account.0 as i64,
                    item.kind.owned_type(),
                    item.id as i64,
                )
                .await
            },
        )
    }

    /// Whether `account` favorited the item.
    pub fn item_favorited(&self, account: AccountId, item: ItemRef) -> Res<bool> {
        if account.is_anonymous() {
            return Res::default();
        }
        self.inner.res(
            Key::private(account, "item_fav", format!("{:?}:{}", item.kind, item.id)),
            OWN_AGE,
            move |f| async move {
                let (u, i) = (account.0 as i64, item.id as i64);
                // The body is `null` when it isn't a favorite.
                Ok(match item.kind {
                    ItemKind::Asset => {
                        let req = catalog::v1::get_favorites_users_by_user_id_assets_by_asset_id_favorite_request(u, i)?;
                        f.client.send(req).await?.json::<Option<serde_json::Value>>()?
                    }
                    ItemKind::Bundle => {
                        let req = catalog::v1::get_favorites_users_by_user_id_bundles_by_bundle_id_favorite_request(u, i)?;
                        f.client.send(req).await?.json::<Option<serde_json::Value>>()?
                    }
                }
                .is_some_and(|v| !v.is_null()))
            },
        )
    }

    // ---- user actions (one click each) ----

    pub fn favorite_item(&self, account: AccountId, item: ItemRef, on: bool) {
        let key = Key::private(account, "item_fav", format!("{:?}:{}", item.kind, item.id));
        self.inner.res.update::<bool>(&key, |v| *v = on);
        let shared = Key::shared("item", format!("{:?}:{}", item.kind, item.id));
        self.inner
            .res
            .update::<ItemDetails>(&shared, |d| d.card.favorites += if on { 1 } else { -1 });
        self.inner.mutate(
            account,
            if on {
                "favorite the item"
            } else {
                "unfavorite the item"
            },
            "",
            &["item_fav"],
            move |f| async move {
                let (u, i) = (account.0 as i64, item.id as i64);
                match (item.kind, on) {
                    (ItemKind::Asset, true) => {
                        catalog::v1::post_favorites_users_by_user_id_assets_by_asset_id_favorite(
                            &f.client, u, i,
                        )
                        .await
                        .map(drop)
                    }
                    (ItemKind::Asset, false) => {
                        catalog::v1::delete_favorites_users_by_user_id_assets_by_asset_id_favorite(
                            &f.client, u, i,
                        )
                        .await
                        .map(drop)
                    }
                    (ItemKind::Bundle, true) => {
                        catalog::v1::post_favorites_users_by_user_id_bundles_by_bundle_id_favorite(
                            &f.client, u, i,
                        )
                        .await
                        .map(drop)
                    }
                    (ItemKind::Bundle, false) => {
                        catalog::v1::delete_favorites_users_by_user_id_bundles_by_bundle_id_favorite(
                            &f.client, u, i,
                        )
                        .await
                        .map(drop)
                    }
                }
            },
        );
    }

    /// The account's wishlist, newest first. Local only; Roblox has no wishlist.
    pub fn wishlist(&self, account: AccountId) -> Vec<WishItem> {
        self.store()
            .account_data(account)
            .map(|d| d.wishlist.clone())
            .unwrap_or_default()
    }

    pub fn in_wishlist(&self, account: AccountId, item: ItemRef) -> bool {
        self.store()
            .account_data(account)
            .is_some_and(|d| d.wishlist.iter().any(|w| w.item == item))
    }

    /// Add or remove; returns whether it is now on the list.
    pub fn toggle_wishlist(&self, account: AccountId, card: &CatalogItem) -> bool {
        let on = {
            let mut s = self.inner.store.write();
            let d = s.data.entry(account).or_default();
            let on = if d.wishlist.iter().any(|w| w.item == card.item) {
                d.wishlist.retain(|w| w.item != card.item);
                false
            } else {
                d.wishlist.insert(
                    0,
                    WishItem {
                        item: card.item,
                        name: card.name.clone(),
                        price_then: card.buy_price(),
                        added: now(),
                    },
                );
                true
            };
            self.inner
                .cache
                .put(Kind::Wishlist, account, "wishlist", &d.wishlist);
            on
        };
        self.inner.toast(
            Some(account),
            if on {
                "Added to wishlist"
            } else {
                "Removed from wishlist"
            },
            false,
        );
        on
    }
}

/// Command bar rows for a marketplace search (first 10 results).
pub(crate) async fn search_entries(
    c: &rbx_api::Client,
    q: &CatalogQuery,
    limit: i32,
) -> Result<Vec<crate::search::Entry>, Error> {
    use crate::search::{Entry, Group, Target};
    let mut p = q.params(None);
    p.limit = Some(limit);
    let r = catalog::v2::get_search_items_details(c, &p).await?;
    Ok(r.data
        .unwrap_or_default()
        .into_iter()
        .filter_map(CatalogItem::from_search)
        .map(|i| Entry {
            group: Group::Items,
            subtitle: format!("{} · {} · by {}", i.price_label(), i.type_name(), i.creator),
            title: i.name,
            aliases: Vec::new(),
            target: match i.item.kind {
                ItemKind::Asset => Target::Asset(i.item.id),
                ItemKind::Bundle => Target::Bundle(i.item.id),
            },
            boost: 0,
        })
        .collect())
}

/// Tile data for many items, 100 per request.
async fn fetch_items(c: &rbx_api::Client, items: &[ItemRef]) -> Result<Vec<CatalogItem>, Error> {
    use catalog::v1::CatalogSearchDetailedCompleteResponseItemItemType as T;
    let mut out = Vec::new();
    for chunk in items.chunks(100) {
        let body = catalog::v1::MultigetItemDetailsRequestModel {
            items: Some(
                chunk
                    .iter()
                    .map(|i| catalog::v1::MultigetItemDetailsRequestItem {
                        id: Some(i.id as i64),
                        item_type: Some(match i.kind {
                            ItemKind::Asset => T::Asset,
                            ItemKind::Bundle => T::Bundle,
                        }),
                    })
                    .collect(),
            ),
        };
        let r = catalog::v1::post_catalog_items_details(c, &body).await?;
        // Same shape as search results; decode through the v2 model so one mapping serves both.
        for x in r.data.unwrap_or_default() {
            let v = serde_json::to_value(x).map_err(|e| Error::Encode(e.to_string()))?;
            if let Ok(x) = serde_json::from_value::<catalog::v2::CatalogSearchDetailedResponseItemV2>(v)
                && let Some(i) = CatalogItem::from_search(x)
            {
                out.push(i);
            }
        }
    }
    // Keep the caller's order.
    out.sort_by_key(|i| items.iter().position(|r| *r == i.item).unwrap_or(usize::MAX));
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn by(creator: &str, id: u64, group: bool) -> CatalogItem {
        CatalogItem {
            creator: creator.into(),
            creator_id: id,
            creator_is_group: group,
            ..Default::default()
        }
    }

    #[test]
    fn flags_creators_posing_as_roblox() {
        assert!(!by("Roblox", ROBLOX_USER_ID, false).fake_roblox_creator());
        assert!(by("Roblox", 42, true).fake_roblox_creator());
        assert!(by("Roblox°°°°°°", 42, true).fake_roblox_creator());
        assert!(by("R0BL0X", 42, false).fake_roblox_creator());
        assert!(by("Rоblox", 42, true).fake_roblox_creator());
        assert!(by("R o b l o x", 42, true).fake_roblox_creator());
        assert!(by("Robiox", 42, false).fake_roblox_creator());
        assert!(!by("Robloxian", 42, false).fake_roblox_creator());
        assert!(!by("Builderman", 42, false).fake_roblox_creator());
        assert!(!by("", 42, false).fake_roblox_creator());
    }

    fn card(limited: Limited, price: Option<i64>, resale: Option<i64>, off_sale: bool) -> CatalogItem {
        CatalogItem {
            limited,
            price,
            lowest_resale: resale,
            off_sale,
            ..Default::default()
        }
    }

    #[test]
    fn buy_price_prefers_resale_for_off_sale_limiteds() {
        assert_eq!(card(Limited::No, Some(0), None, false).price_label(), "Free");
        assert_eq!(card(Limited::No, Some(96), None, false).price_label(), "R$ 96");
        assert_eq!(card(Limited::No, Some(96), None, true).price_label(), "Off sale");
        // Off-sale classic limited: only resellers sell it.
        let helm = card(Limited::Limited, None, Some(279_900), true);
        assert_eq!(helm.buy_price(), Some(279_900));
        assert_eq!(helm.price_label(), "R$ 279,900");
        // On-sale collectible with a cheaper resale copy: the cheaper one.
        assert_eq!(
            card(Limited::Collectible, Some(500), Some(485), false).buy_price(),
            Some(485)
        );
        assert_eq!(
            card(Limited::Collectible, Some(50), Some(485), false).buy_price(),
            Some(50)
        );
        // No resellers left.
        assert_eq!(
            card(Limited::LimitedU, None, None, true).price_label(),
            "Off sale"
        );
    }

    #[test]
    fn buy_offer_picks_creator_or_cheapest_reseller() {
        use crate::purchase::Seller;
        let offer = |c: CatalogItem| c.buy_offer();
        assert_eq!(
            offer(card(Limited::No, Some(0), None, false)),
            Some((Seller::Creator, 0))
        );
        assert_eq!(offer(card(Limited::No, Some(96), None, true)), None);
        // A non-resellable item never goes to resellers, whatever the API says.
        assert_eq!(offer(card(Limited::No, None, Some(5), false)), None);
        assert_eq!(
            offer(card(Limited::Limited, None, Some(279_900), true)),
            Some((Seller::Cheapest, 279_900))
        );
        assert_eq!(
            offer(card(Limited::Collectible, Some(500), Some(485), false)),
            Some((Seller::Cheapest, 485))
        );
        // Same price: a new copy from the creator.
        assert_eq!(
            offer(card(Limited::Collectible, Some(485), Some(485), false)),
            Some((Seller::Creator, 485))
        );
        // Creator's stock is gone: resellers only.
        let sold_out = CatalogItem {
            total_quantity: 100,
            units_left: 0,
            ..card(Limited::Collectible, Some(50), Some(485), false)
        };
        assert_eq!(offer(sold_out), Some((Seller::Cheapest, 485)));
    }

    #[test]
    fn only_limiteds_and_collectibles_buy_again() {
        assert!(!card(Limited::No, Some(0), None, false).buy_again());
        assert!(card(Limited::Limited, None, Some(10), true).buy_again());
        assert!(card(Limited::Collectible, Some(50), None, false).buy_again());
    }

    #[test]
    fn query_ids_and_filter_counts() {
        let mut q = CatalogQuery::default();
        assert_eq!(q.filter_count(), 0);
        let base = q.id();
        q.keyword = " Crown ".into();
        assert_ne!(q.id(), base);
        let mut q2 = q.clone();
        q2.keyword = "crown".into();
        assert_eq!(q.id(), q2.id(), "case and spaces don't make a new search");
        q.sales = SalesType::Limited;
        q.min_price = Some(10);
        q.creator = "Roblox".into();
        assert_eq!(q.filter_count(), 3);
        let p = q.params(None);
        assert_eq!(p.keyword.as_deref(), Some("Crown"));
        assert_eq!(p.creator_name.as_deref(), Some("Roblox"));
        assert_eq!(p.min_price, Some(10));
        assert!(p.taxonomy.is_none());
        assert_eq!(CatalogSort::from_key("price_low"), CatalogSort::PriceLow);
        assert_eq!(CatalogSort::from_key("junk"), CatalogSort::Relevance);
        q.keyword = "a|b".into();
        q.max_price = Some(500);
        q.include_off_sale = true;
        let back = CatalogQuery::from_id(&q.id()).unwrap();
        assert_eq!(back.id(), q.id(), "round trips");
        assert_eq!(back.keyword, "a|b");
        assert_eq!((back.min_price, back.max_price), (Some(10), Some(500)));
        assert!(CatalogQuery::from_id("junk").is_none());
    }
}

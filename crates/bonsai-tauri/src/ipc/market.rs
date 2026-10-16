//! Marketplace, item pages and inventories.
//!
//! Commands about "this item" read it from the argument (`item:<id>`, `bundle:<id>` or a bare
//! asset id) or else the page context. `list.more` keys here carry what they page:
//! `catalog:<query id>`, `resellers:<collectible id>`, `owners:<asset id>`,
//! `inventory:<user>:<slot>:<oldest first>`, and `passes:`, `badges:` and `places:<user>`.

use std::collections::HashMap;
use std::sync::Arc;

use rbx_core::commands::CommandId;
use rbx_core::inventory::{InvItem, InvSlot, stack, totals};
use rbx_core::market::{CatalogItem, CatalogQuery, CatalogSort, ItemKind, ItemRef, SalesType};
use rbx_core::purchase::Seller;
use rbx_core::values::{Demand, ItemValue, LimitedKind, Trend, ValueTable};
use rbx_core::{AccountId, Core};
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::State;

use super::{Id, Invocation, ItemCard, List, UserBadge, count, viewer};

/// A choice in a picker: value and label.
#[derive(Clone, Debug, Serialize, Type)]
pub struct Choice {
    pub value: String,
    pub label: String,
}

fn choice(value: impl Into<String>, label: impl Into<String>) -> Choice {
    Choice {
        value: value.into(),
        label: label.into(),
    }
}

/// A list plus the `list.more` argument that loads its next page.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Paged<T> {
    pub list: List<T>,
    pub more_key: String,
}

// ---------------------------------------------------------------- catalog

#[derive(Clone, Debug, Serialize, Type)]
pub struct CategoryView {
    pub name: String,
    /// What `CatalogFilter.taxonomy` takes.
    pub taxonomy: String,
    pub subcategories: Vec<Choice>,
}

/// Sort and sales-type choices for the catalog filters.
#[derive(Clone, Debug, Serialize, Type)]
pub struct CatalogOptions {
    pub sorts: Vec<Choice>,
    pub sales: Vec<Choice>,
}

/// One marketplace search, as the filters set it.
#[derive(Clone, Debug, Default, Deserialize, Type)]
#[serde(rename_all = "camelCase", default)]
pub struct CatalogFilter {
    pub taxonomy: String,
    pub keyword: String,
    /// A `CatalogOptions.sorts` value.
    pub sort: String,
    /// A `CatalogOptions.sales` value.
    pub sales: String,
    pub min_price: Option<u32>,
    pub max_price: Option<u32>,
    pub creator: String,
    pub include_off_sale: bool,
}

impl CatalogFilter {
    fn query(&self) -> CatalogQuery {
        CatalogQuery {
            taxonomy: self.taxonomy.clone(),
            keyword: self.keyword.clone(),
            sort: CatalogSort::from_key(&self.sort),
            sales: SalesType::from_key(&self.sales),
            min_price: self.min_price,
            max_price: self.max_price,
            creator: self.creator.clone(),
            include_off_sale: self.include_off_sale,
        }
    }
}

/// Values for collectibles, when the user turned them on and they loaded.
fn values(core: &Core) -> Option<Arc<ValueTable>> {
    core.item_values().data
}

/// Rolimons' tag for an item, when values are on and it is a limited Rolimons tracks.
pub(super) fn tag_of(values: Option<&ValueTable>, item: ItemRef) -> Option<ValueTag> {
    if item.kind != ItemKind::Asset {
        return None;
    }
    values?.get(item.id).map(ValueTag::from)
}

pub(super) fn card(i: &CatalogItem, values: Option<&ValueTable>) -> ItemCard {
    ItemCard {
        id: Id(i.item.id),
        bundle: i.item.kind == ItemKind::Bundle,
        name: i.name.clone(),
        creator: i.creator.clone(),
        creator_verified: i.creator_verified,
        copies: 1,
        asset_type: i.asset_type,
        price: i.buy_price().and_then(count),
        limited: i.limited.resellable(),
        serial: None,
        rap: None,
        values: if i.limited.resellable() {
            tag_of(values, i.item)
        } else {
            None
        },
    }
}

/// `asset_type` is the inventory slot's, or 0 when the list mixes types.
pub(super) fn inv_card(i: &InvItem, asset_type: i32, values: Option<&ValueTable>) -> ItemCard {
    ItemCard {
        id: Id(i.item.id),
        bundle: i.item.kind == ItemKind::Bundle,
        name: i.name.clone(),
        creator: String::new(),
        creator_verified: false,
        copies: 1,
        asset_type,
        price: None,
        limited: i.rap.is_some() || i.serial.is_some(),
        serial: i.serial.and_then(count),
        rap: i.rap.and_then(count),
        values: tag_of(values, i.item),
    }
}

#[tauri::command]
#[specta::specta]
pub fn catalog_categories(core: State<'_, Arc<Core>>) -> List<CategoryView> {
    List::of(core.catalog_categories(), |c| CategoryView {
        name: c.name.clone(),
        taxonomy: c.taxonomy.clone(),
        subcategories: c
            .subcategories
            .iter()
            .map(|s| choice(&s.taxonomy, &s.name))
            .collect(),
    })
}

#[tauri::command]
#[specta::specta]
pub fn catalog_options() -> CatalogOptions {
    CatalogOptions {
        sorts: CatalogSort::ALL
            .iter()
            .map(|s| choice(s.key(), s.label()))
            .collect(),
        sales: SalesType::ALL
            .iter()
            .map(|s| choice(s.key(), s.label()))
            .collect(),
    }
}

#[tauri::command]
#[specta::specta]
pub fn catalog_search(core: State<'_, Arc<Core>>, filter: CatalogFilter) -> Paged<ItemCard> {
    let q = filter.query();
    let v = values(&core);
    Paged {
        list: List::paged(core.catalog_search(&q, false), |i| card(i, v.as_deref())),
        more_key: format!("catalog:{}", q.id()),
    }
}

// ---------------------------------------------------------------- item page

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ItemView {
    pub loading: bool,
    pub error: Option<String>,
    pub info: Option<ItemInfo>,
    /// `None` signed out or while loading.
    pub favorited: Option<bool>,
    pub wishlisted: bool,
    /// Your accounts that own it.
    pub owned_by: Vec<String>,
    /// The active account owns it and can't own another copy, so "Buy" is off.
    pub owned: bool,
    /// Rolimons' data, when values are on.
    pub value: Option<ValueView>,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ItemInfo {
    pub id: Id,
    pub bundle: bool,
    pub name: String,
    pub creator: String,
    pub creator_id: Id,
    pub creator_is_group: bool,
    pub creator_verified: bool,
    /// The creator's name passes for Roblox but isn't Roblox's account.
    pub creator_fake: bool,
    /// What it costs now: list price or cheapest resale.
    pub price: Option<u32>,
    /// "R$ 1,234", "Free" or "Off sale".
    pub price_label: String,
    /// "Buy" buys the cheapest reseller's copy, not a new one from the creator.
    pub from_reseller: bool,
    /// "Hat", "Body bundle"...
    pub type_name: String,
    pub asset_type: i32,
    /// "Limited", "Limited U", "Collectible" or nothing.
    pub limited: Option<String>,
    pub off_sale: bool,
    pub favorites: u32,
    pub description: String,
    /// ISO dates; empty when unknown.
    pub created: String,
    pub updated: String,
    pub off_sale_deadline: String,
    pub category: String,
    pub genres: Vec<String>,
    pub tags: Vec<String>,
    pub has_resellers: bool,
    /// For resellers and price history; empty for items that don't resell.
    pub collectible_id: String,
    pub total_quantity: u32,
    pub units_left: u32,
    pub premium_only: bool,
    pub contents: Vec<BundlePartView>,
    /// The Marketplace lists it. Otherwise (models, decals, audio, plugins...) only economy
    /// details are known: no buying, wishlist, similar items or tags.
    pub in_catalog: bool,
    /// Anyone can take a copy, for assets outside the catalog.
    pub public_domain: bool,
    /// Copies sold or taken, for assets outside the catalog; 0 when unknown.
    pub sales: u32,
}

#[derive(Clone, Debug, Serialize, Type)]
pub struct BundlePartView {
    pub id: Id,
    pub name: String,
}

/// Rolimons' demand rating, lowest first.
#[derive(Clone, Copy, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum ValueDemand {
    Terrible,
    Low,
    Normal,
    High,
    Amazing,
}

impl From<Demand> for ValueDemand {
    fn from(d: Demand) -> Self {
        match d {
            Demand::Terrible => ValueDemand::Terrible,
            Demand::Low => ValueDemand::Low,
            Demand::Normal => ValueDemand::Normal,
            Demand::High => ValueDemand::High,
            Demand::Amazing => ValueDemand::Amazing,
        }
    }
}

/// Which way Rolimons sees an item's price going.
#[derive(Clone, Copy, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum ValueTrend {
    Lowering,
    Unstable,
    Stable,
    Raising,
    Fluctuating,
}

impl From<Trend> for ValueTrend {
    fn from(t: Trend) -> Self {
        match t {
            Trend::Lowering => ValueTrend::Lowering,
            Trend::Unstable => ValueTrend::Unstable,
            Trend::Stable => ValueTrend::Stable,
            Trend::Raising => ValueTrend::Raising,
            Trend::Fluctuating => ValueTrend::Fluctuating,
        }
    }
}

/// Limited 1.0 (`classic`) or Limited 2.0 (`new`).
#[derive(Clone, Copy, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum LimitedVersion {
    Classic,
    New,
    Unknown,
}

impl From<LimitedKind> for LimitedVersion {
    fn from(k: LimitedKind) -> Self {
        match k {
            LimitedKind::Classic => LimitedVersion::Classic,
            LimitedKind::New => LimitedVersion::New,
            LimitedKind::Unknown => LimitedVersion::Unknown,
        }
    }
}

/// What Rolimons says about one limited, small enough to put on every tile and trade row.
#[derive(Clone, Debug, Serialize, Type)]
pub struct ValueTag {
    /// `None` when Rolimons doesn't value it; trade math then uses RAP.
    pub value: Option<u32>,
    pub demand: Option<ValueDemand>,
    pub trend: Option<ValueTrend>,
    /// RAP was pushed up by a few sales and is likely to fall.
    pub projected: bool,
    pub hyped: bool,
    pub rare: bool,
}

impl From<&ItemValue> for ValueTag {
    fn from(v: &ItemValue) -> Self {
        ValueTag {
            value: v.value.and_then(count),
            demand: v.demand.map(Into::into),
            trend: v.trend.map(Into::into),
            projected: v.projected,
            hyped: v.hyped,
            rare: v.rare,
        }
    }
}

/// An item's Rolimons entry on its page.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ValueView {
    pub tag: ValueTag,
    /// RAP as Rolimons last saw it.
    pub rap: Option<u32>,
    pub acronym: String,
    pub version: LimitedVersion,
    /// When the table was fetched, ms since the epoch.
    pub fetched_at: Option<f64>,
}

fn ms(secs: i64) -> Option<f64> {
    (secs > 0).then_some(secs as f64 * 1000.0)
}

/// One row of the values table.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ValueRow {
    pub id: Id,
    pub name: String,
    pub acronym: String,
    pub rap: Option<u32>,
    pub version: LimitedVersion,
    pub tag: ValueTag,
}

/// Whether values are on and how fresh they are, without the table itself.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ValuesStatus {
    pub enabled: bool,
    pub loading: bool,
    pub error: Option<String>,
    /// ms since the epoch; `None` before the first load.
    pub fetched_at: Option<f64>,
    pub count: u32,
}

/// Status of the values table. Cheap, and asking refetches the table once it is two minutes
/// old, so screens that show values call it on a timer while open.
#[tauri::command]
#[specta::specta]
pub fn values_status(core: State<'_, Arc<Core>>) -> ValuesStatus {
    let r = core.item_values();
    ValuesStatus {
        enabled: core.values_enabled(),
        loading: r.loading,
        error: r.error,
        fetched_at: r.data.as_ref().and_then(|_| ms(r.fetched_at)),
        count: r.data.map_or(0, |t| t.items.len() as u32),
    }
}

/// Every limited Rolimons tracks, highest value first. Large: call it when
/// `ValuesStatus.fetched_at` changes, not on every store change.
#[tauri::command]
#[specta::specta]
pub fn values_table(core: State<'_, Arc<Core>>) -> Vec<ValueRow> {
    let Some(t) = core.item_values().data else {
        return Vec::new();
    };
    let mut rows: Vec<ValueRow> = t
        .items
        .iter()
        .map(|(id, v)| ValueRow {
            id: Id(*id),
            name: v.name.clone(),
            acronym: v.acronym.clone(),
            rap: v.rap.and_then(count),
            version: v.kind.into(),
            tag: v.into(),
        })
        .collect();
    rows.sort_by(|a, b| {
        let worth = |r: &ValueRow| r.tag.value.or(r.rap).unwrap_or(0);
        worth(b).cmp(&worth(a)).then_with(|| a.name.cmp(&b.name))
    });
    rows
}

fn item_ref(id: Id, bundle: bool) -> ItemRef {
    if bundle {
        ItemRef::bundle(id.0)
    } else {
        ItemRef::asset(id.0)
    }
}

#[tauri::command]
#[specta::specta]
pub fn item_page(core: State<'_, Arc<Core>>, id: Id, bundle: bool) -> ItemView {
    let item = item_ref(id, bundle);
    let r = core.item(item);
    let me = core.acting(None);
    let labels: Vec<(AccountId, String)> = core
        .store()
        .accounts
        .accounts
        .iter()
        .map(|a| (a.id, a.label.clone().unwrap_or_else(|| a.display_name.clone())))
        .collect();
    let owned_by = labels
        .into_iter()
        .filter(|(a, _)| core.owns(*a, item).data.is_some_and(|o| *o))
        .map(|(_, l)| l)
        .collect();
    let owned = me.is_some_and(|a| core.owns(a, item).data.is_some_and(|o| *o))
        && r.data.as_ref().is_some_and(|d| !d.card.buy_again());
    let value = if bundle {
        None
    } else {
        let r = core.item_values();
        r.data.as_ref().and_then(|t| t.get(id.0)).map(|v| ValueView {
            tag: v.into(),
            rap: v.rap.and_then(count),
            acronym: v.acronym.clone(),
            version: v.kind.into(),
            fetched_at: ms(r.fetched_at),
        })
    };
    // Assets the catalog doesn't list fall back to their economy details.
    if r.data.is_none() && r.error.is_some() && !bundle {
        let a = core.asset_info(id.0);
        return ItemView {
            loading: a.loading,
            error: a.data.is_none().then_some(a.error).flatten(),
            info: a.data.map(|x| asset_page_info(id, &x)),
            favorited: me.and_then(|a| core.item_favorited(a, item).data.map(|f| *f)),
            wishlisted: false,
            owned: false,
            owned_by,
            value,
        };
    }
    ItemView {
        loading: r.loading,
        error: r.error,
        info: r.data.map(|d| {
            let c = &d.card;
            ItemInfo {
                id,
                bundle,
                name: c.name.clone(),
                creator: c.creator.clone(),
                creator_id: Id(c.creator_id),
                creator_is_group: c.creator_is_group,
                creator_verified: c.creator_verified,
                creator_fake: c.fake_roblox_creator(),
                price: c.buy_price().and_then(count),
                price_label: c.price_label(),
                from_reseller: matches!(c.buy_offer(), Some((Seller::Cheapest, _))),
                type_name: c.type_name().to_owned(),
                asset_type: c.asset_type,
                limited: c.limited.label().map(str::to_owned),
                off_sale: c.off_sale,
                favorites: count(c.favorites).unwrap_or(0),
                description: d.description.clone(),
                created: d.created.clone(),
                updated: d.updated.clone(),
                off_sale_deadline: d.off_sale_deadline.clone(),
                category: d.category.clone(),
                genres: d.genres.clone(),
                tags: d.tags.clone(),
                has_resellers: d.has_resellers,
                collectible_id: c.collectible_id.clone(),
                total_quantity: count(c.total_quantity).unwrap_or(0),
                units_left: count(c.units_left).unwrap_or(0),
                premium_only: c.premium_only,
                contents: d
                    .contents
                    .iter()
                    .map(|p| BundlePartView {
                        id: Id(p.id),
                        name: p.name.clone(),
                    })
                    .collect(),
                in_catalog: true,
                public_domain: false,
                sales: 0,
            }
        }),
        favorited: me.and_then(|a| core.item_favorited(a, item).data.map(|f| *f)),
        wishlisted: me.is_some_and(|a| core.in_wishlist(a, item)),
        owned,
        owned_by,
        value,
    }
}

fn price_label(price: Option<i64>) -> String {
    match price {
        Some(0) => "Free".to_owned(),
        Some(p) => rbx_core::fmt::robux(p),
        None => "Off sale".to_owned(),
    }
}

/// An item page for an asset the catalog doesn't list, from its economy details.
fn asset_page_info(id: Id, x: &rbx_core::inventory::AssetInfo) -> ItemInfo {
    ItemInfo {
        id,
        bundle: false,
        name: x.name.clone(),
        creator: x.creator.clone(),
        creator_id: Id(x.creator_id),
        creator_is_group: x.creator_is_group,
        creator_verified: x.creator_verified,
        creator_fake: x.fake_roblox_creator(),
        price: None,
        price_label: if x.price.is_none() && x.public_domain {
            "Free to take".to_owned()
        } else {
            price_label(x.price)
        },
        from_reseller: false,
        type_name: rbx_core::market::asset_type_name(x.asset_type).to_owned(),
        asset_type: x.asset_type,
        limited: None,
        off_sale: x.price.is_none(),
        favorites: 0,
        description: x.description.clone(),
        created: x.created.clone(),
        updated: x.updated.clone(),
        off_sale_deadline: String::new(),
        category: String::new(),
        genres: Vec::new(),
        tags: Vec::new(),
        has_resellers: false,
        collectible_id: String::new(),
        total_quantity: 0,
        units_left: 0,
        premium_only: x.premium_only,
        contents: Vec::new(),
        in_catalog: false,
        public_domain: x.public_domain,
        sales: count(x.sales as i64).unwrap_or(0),
    }
}

/// A game pass's page.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PassView {
    pub loading: bool,
    pub error: Option<String>,
    pub info: Option<PassInfoView>,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PassInfoView {
    pub id: Id,
    pub name: String,
    pub description: String,
    pub creator: String,
    pub creator_id: Id,
    pub creator_is_group: bool,
    pub price: Option<u32>,
    /// "R$ 1,234", "Free" or "Off sale".
    pub price_label: String,
    pub sales: u32,
    /// ISO dates; empty when unknown.
    pub created: String,
    pub updated: String,
}

#[tauri::command]
#[specta::specta]
pub fn pass_page(core: State<'_, Arc<Core>>, id: Id) -> PassView {
    let r = core.pass_info(id.0);
    PassView {
        loading: r.loading,
        error: r.error,
        info: r.data.map(|x| PassInfoView {
            id,
            name: x.name.clone(),
            description: x.description.clone(),
            creator: x.creator.clone(),
            creator_id: Id(x.creator_id),
            creator_is_group: x.creator_is_group,
            price: x.price.and_then(count),
            price_label: price_label(x.price),
            sales: count(x.sales as i64).unwrap_or(0),
            created: x.created.clone(),
            updated: x.updated.clone(),
        }),
    }
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ResellerRow {
    /// The listed copy; `market.buy_resale` takes it.
    pub instance_id: String,
    pub seller_id: Id,
    pub seller: String,
    pub verified: bool,
    pub price: u32,
    pub serial: Option<u32>,
}

#[tauri::command]
#[specta::specta]
pub fn item_resellers(core: State<'_, Arc<Core>>, collectible_id: String) -> Paged<ResellerRow> {
    Paged {
        list: List::paged(core.item_resellers(&collectible_id, false), |r| ResellerRow {
            instance_id: r.instance_id.clone(),
            seller_id: Id(r.seller_id),
            seller: r.seller_name.clone(),
            verified: r.seller_verified,
            price: count(r.price).unwrap_or(0),
            serial: r.serial.and_then(count),
        }),
        more_key: format!("resellers:{collectible_id}"),
    }
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct OwnerRow {
    /// `None` when they hide their inventory.
    pub user_id: Option<Id>,
    pub name: String,
    pub serial: Option<u32>,
    /// ISO date the copy last changed hands.
    pub since: String,
}

#[tauri::command]
#[specta::specta]
pub fn item_owners(core: State<'_, Arc<Core>>, asset_id: Id) -> Paged<OwnerRow> {
    Paged {
        list: List::paged(core.item_owners(viewer(&core), asset_id.0, false), |o| OwnerRow {
            user_id: o.user_id.map(Id),
            name: o.name.clone(),
            serial: o.serial.and_then(count),
            since: o.since.clone(),
        }),
        more_key: format!("owners:{}", asset_id.0),
    }
}

/// A point on a chart: ms since the epoch, value.
#[derive(Clone, Copy, Debug, Serialize, Type)]
pub struct Point {
    pub t: f64,
    pub v: f64,
}

#[derive(Clone, Debug, Serialize, Type)]
pub struct PriceHistoryView {
    pub loading: bool,
    pub error: Option<String>,
    pub rap: Option<u32>,
    /// Daily average resale price, oldest first.
    pub prices: Vec<Point>,
    /// Copies sold per day, oldest first.
    pub volume: Vec<Point>,
}

#[tauri::command]
#[specta::specta]
pub fn item_price_history(core: State<'_, Arc<Core>>, collectible_id: String) -> PriceHistoryView {
    let r = core.item_price_history(&collectible_id);
    let points = |s: &[(i64, f64)]| {
        s.iter()
            .map(|(t, v)| Point {
                t: *t as f64 * 1000.0,
                v: *v,
            })
            .collect()
    };
    PriceHistoryView {
        loading: r.loading,
        error: r.error,
        rap: r.data.as_ref().and_then(|h| h.rap).and_then(count),
        prices: r.data.as_ref().map(|h| points(&h.prices)).unwrap_or_default(),
        volume: r.data.as_ref().map(|h| points(&h.volume)).unwrap_or_default(),
    }
}

#[tauri::command]
#[specta::specta]
pub fn item_recommendations(
    core: State<'_, Arc<Core>>,
    id: Id,
    bundle: bool,
    asset_type: i32,
) -> List<ItemCard> {
    let v = values(&core);
    List::of(core.item_recommendations(item_ref(id, bundle), asset_type), |i| {
        card(i, v.as_deref())
    })
}

/// The active account's wishlist, with today's prices.
#[tauri::command]
#[specta::specta]
pub fn wishlist(core: State<'_, Arc<Core>>) -> List<ItemCard> {
    let refs: Vec<ItemRef> = core
        .acting(None)
        .map(|a| core.wishlist(a).iter().map(|w| w.item).collect())
        .unwrap_or_default();
    if refs.is_empty() {
        return List {
            loading: false,
            ..List::loading()
        };
    }
    let v = values(&core);
    List::of(core.items(&refs), |i| card(i, v.as_deref()))
}

// ---------------------------------------------------------------- inventory

/// Who an inventory route names: an id, or a username to look up.
#[derive(Clone, Debug, Serialize, Type)]
pub struct UserLookup {
    pub loading: bool,
    pub error: Option<String>,
    pub id: Option<Id>,
}

#[tauri::command]
#[specta::specta]
pub fn find_user(core: State<'_, Arc<Core>>, who: String) -> UserLookup {
    let who = who.trim().trim_start_matches('@');
    if let Ok(id) = who.parse::<u64>() {
        return UserLookup {
            loading: false,
            error: None,
            id: Some(Id(id)),
        };
    }
    if who.is_empty() {
        return UserLookup {
            loading: false,
            error: Some("Which user?".to_owned()),
            id: None,
        };
    }
    let r = core.user_by_name(who);
    UserLookup {
        loading: r.loading,
        error: r.error,
        id: r.data.map(|id| Id(*id)),
    }
}

#[derive(Clone, Debug, Serialize, Type)]
pub struct InvCategoryView {
    pub name: String,
    /// `value` is what `inventory_items` takes as `slot`.
    pub slots: Vec<Choice>,
}

#[tauri::command]
#[specta::specta]
pub fn inventory_categories(core: State<'_, Arc<Core>>, user_id: Id) -> List<InvCategoryView> {
    let mut list = List::of(core.inventory_categories(user_id.0), |c| InvCategoryView {
        name: c.name.clone(),
        slots: c
            .slots
            .iter()
            .map(|(label, slot)| choice(slot.key(), label))
            .collect(),
    });
    list.items.insert(
        0,
        InvCategoryView {
            name: "Collectibles".to_owned(),
            slots: vec![choice(InvSlot::Collectibles.key(), "Collectibles")],
        },
    );
    list
}

/// Why an inventory can't be shown, or `None` when it can (or is still being checked).
fn private_inventory<T>(core: &Core, viewer: AccountId, user_id: u64) -> Option<List<T>> {
    let can = core.can_view_inventory(viewer, user_id);
    match can.data.as_deref() {
        Some(true) => None,
        Some(false) => Some(List {
            loading: false,
            error: Some("Their inventory is private.".to_owned()),
            items: Vec::new(),
            more: false,
        }),
        None => Some(List {
            loading: can.error.is_none(),
            error: can.error,
            items: Vec::new(),
            more: false,
        }),
    }
}

#[tauri::command]
#[specta::specta]
pub fn inventory_items(
    core: State<'_, Arc<Core>>,
    user_id: Id,
    slot: String,
    oldest_first: bool,
) -> Paged<ItemCard> {
    let more_key = format!("inventory:{}:{slot}:{oldest_first}", user_id.0);
    let a = viewer(&core);
    if let Some(list) = private_inventory(&core, a, user_id.0) {
        return Paged { list, more_key };
    }
    let Some(slot) = InvSlot::from_key(&slot) else {
        return Paged {
            list: List {
                loading: false,
                error: Some("Unknown category.".to_owned()),
                items: Vec::new(),
                more: false,
            },
            more_key,
        };
    };
    let v = values(&core);
    let asset_type = match slot {
        InvSlot::Asset(t) => t,
        _ => 0,
    };
    let r = core.inventory(a, user_id.0, slot, oldest_first, false);
    let page = r.data.clone();
    let mut list = List::paged(r, |_: &InvItem| ());
    let items = page.as_deref().map(|p| p.items.as_slice()).unwrap_or_default();
    // Creator and price come from the catalog, a page (100 items) per request so the cache
    // keys stay put as pages load.
    let mut known: HashMap<ItemRef, CatalogItem> = HashMap::new();
    for chunk in items.chunks(100) {
        let refs: Vec<ItemRef> = chunk.iter().map(|i| i.item).collect();
        if let Some(d) = core.items(&refs).data {
            known.extend(d.iter().map(|c| (c.item, c.clone())));
        }
    }
    let cards = stack(items)
        .into_iter()
        .map(|(i, n)| {
            let mut c = inv_card(i, asset_type, v.as_deref());
            c.copies = n;
            if let Some(k) = known.get(&i.item) {
                c.creator = k.creator.clone();
                c.creator_verified = k.creator_verified;
                c.price = k.buy_price().and_then(count);
                if c.asset_type == 0 {
                    c.asset_type = k.asset_type;
                }
            }
            c
        })
        .collect();
    Paged {
        list: List {
            loading: list.loading,
            error: list.error.take(),
            items: cards,
            more: list.more,
        },
        more_key,
    }
}

/// A game pass in someone's inventory.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PassCard {
    pub id: Id,
    pub name: String,
    pub description: String,
    pub creator: String,
    pub creator_id: Id,
    pub creator_is_group: bool,
    pub price: Option<u32>,
}

#[tauri::command]
#[specta::specta]
pub fn inventory_passes(core: State<'_, Arc<Core>>, user_id: Id) -> Paged<PassCard> {
    let more_key = format!("passes:{}", user_id.0);
    if let Some(list) = private_inventory(&core, viewer(&core), user_id.0) {
        return Paged { list, more_key };
    }
    Paged {
        list: List::paged(core.user_passes(user_id.0, false), |p| PassCard {
            id: Id(p.id),
            name: p.name.clone(),
            description: p.description.clone(),
            creator: p.creator.clone(),
            creator_id: Id(p.creator_id),
            creator_is_group: p.creator_is_group,
            price: p.price.and_then(count),
        }),
        more_key,
    }
}

/// Badges in someone's inventory, newest first.
#[tauri::command]
#[specta::specta]
pub fn inventory_badges(core: State<'_, Arc<Core>>, user_id: Id) -> Paged<UserBadge> {
    let more_key = format!("badges:{}", user_id.0);
    let a = viewer(&core);
    if let Some(list) = private_inventory(&core, a, user_id.0) {
        return Paged { list, more_key };
    }
    Paged {
        list: List::paged(core.user_badges(a, user_id.0, false), |b| UserBadge {
            id: Id(b.id),
            name: b.name.clone(),
            description: b.description.clone(),
            win_rate: b.win_rate,
            place_id: Id(b.place_id),
        }),
        more_key,
    }
}

/// A place someone made, in their inventory.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PlaceCard {
    pub place_id: Id,
    /// 0 when unknown.
    pub universe_id: Id,
    pub name: String,
    pub creator: String,
    /// Paid access price; `None` when free to play.
    pub price: Option<u32>,
}

#[tauri::command]
#[specta::specta]
pub fn inventory_places(core: State<'_, Arc<Core>>, user_id: Id) -> Paged<PlaceCard> {
    let more_key = format!("places:{}", user_id.0);
    if let Some(list) = private_inventory(&core, viewer(&core), user_id.0) {
        return Paged { list, more_key };
    }
    Paged {
        list: List::paged(core.user_places(user_id.0, false), |p| PlaceCard {
            place_id: Id(p.place_id),
            universe_id: Id(p.universe_id),
            name: p.name.clone(),
            creator: p.creator.clone(),
            price: p.price.and_then(count),
        }),
        more_key,
    }
}

/// What the inventory's inspection drawer shows about one item.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct InspectView {
    pub loading: bool,
    pub error: Option<String>,
    /// Empty until loaded.
    pub name: String,
    pub description: String,
    pub creator: String,
    pub creator_id: Id,
    pub creator_is_group: bool,
    pub creator_verified: bool,
    /// "Hat", "Decal", "Body bundle"...
    pub type_name: String,
    /// "Limited", "Limited U", "Collectible" or nothing.
    pub limited: Option<String>,
    /// "R$ 1,234", "Free" or "Off sale": what a new copy costs.
    pub price_label: String,
    /// Cheapest copy for sale by another user, for limiteds.
    pub lowest_resale: Option<u32>,
    /// ISO date it was made; empty when unknown.
    pub created: String,
    /// The Marketplace lists it, so its item page has more.
    pub in_catalog: bool,
}

impl InspectView {
    fn waiting(loading: bool, error: Option<String>) -> Self {
        InspectView {
            loading,
            error,
            name: String::new(),
            description: String::new(),
            creator: String::new(),
            creator_id: Id(0),
            creator_is_group: false,
            creator_verified: false,
            type_name: String::new(),
            limited: None,
            price_label: String::new(),
            lowest_resale: None,
            created: String::new(),
            in_catalog: false,
        }
    }
}

/// An inventory item's details. Assets the catalog doesn't list (decals, audio, models...) fall
/// back to their economy details.
#[tauri::command]
#[specta::specta]
pub fn inventory_inspect(core: State<'_, Arc<Core>>, id: Id, bundle: bool) -> InspectView {
    let r = core.item(item_ref(id, bundle));
    if let Some(d) = r.data {
        let c = &d.card;
        return InspectView {
            name: c.name.clone(),
            description: d.description.clone(),
            creator: c.creator.clone(),
            creator_id: Id(c.creator_id),
            creator_is_group: c.creator_is_group,
            creator_verified: c.creator_verified,
            type_name: c.type_name().to_owned(),
            limited: c.limited.label().map(str::to_owned),
            price_label: price_label(c.price.filter(|_| !c.off_sale)),
            lowest_resale: c
                .lowest_resale
                .filter(|p| *p > 0 && c.limited.resellable())
                .and_then(count),
            created: d.created.clone(),
            in_catalog: true,
            ..InspectView::waiting(r.loading, None)
        };
    }
    if bundle || r.error.is_none() {
        return InspectView::waiting(r.loading || r.error.is_none(), r.error);
    }
    let a = core.asset_info(id.0);
    let Some(x) = a.data else {
        return InspectView::waiting(a.loading || a.error.is_none(), a.error);
    };
    InspectView {
        name: x.name.clone(),
        description: x.description.clone(),
        creator: x.creator.clone(),
        creator_id: Id(x.creator_id),
        creator_is_group: x.creator_is_group,
        creator_verified: x.creator_verified,
        type_name: rbx_core::market::asset_type_name(x.asset_type).to_owned(),
        price_label: if x.price.is_none() && x.public_domain {
            "Free to take".to_owned()
        } else {
            price_label(x.price)
        },
        created: x.created.clone(),
        ..InspectView::waiting(a.loading, None)
    }
}

/// Totals over a user's collectibles.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct InvTotalsView {
    pub loading: bool,
    pub items: u32,
    pub rap: f64,
    /// With item values on; RAP stands in for items without a value.
    pub value: Option<f64>,
    pub on_hold: u32,
}

#[tauri::command]
#[specta::specta]
pub fn inventory_totals(core: State<'_, Arc<Core>>, user_id: Id) -> InvTotalsView {
    let a = viewer(&core);
    let empty = InvTotalsView {
        loading: false,
        items: 0,
        rap: 0.0,
        value: None,
        on_hold: 0,
    };
    if core.can_view_inventory(a, user_id.0).data.as_deref() != Some(&true) {
        return empty;
    }
    let r = core.collectibles(a, user_id.0);
    let Some(items) = r.data else {
        return InvTotalsView {
            loading: r.loading,
            ..empty
        };
    };
    let v = values(&core);
    let t = totals(&items, v.as_deref());
    InvTotalsView {
        loading: r.loading,
        items: t.items as u32,
        rap: t.rap as f64,
        value: t.value.map(|x| x as f64),
        on_hold: t.on_hold as u32,
    }
}

/// Buys an item from whoever "Buy" sells it for now (`CatalogItem::buy_offer`). Called only from
/// the dialog `market.buy` or `market.buy_as` opens, after the user saw `price`; nothing is
/// bought if the price changed. `account` buys as that account instead of the active one.
#[tauri::command]
#[specta::specta]
pub fn buy_item(
    core: State<'_, Arc<Core>>,
    id: Id,
    bundle: bool,
    price: u32,
    account: Option<Id>,
) -> Result<(), String> {
    let account = core
        .acting(account.map(|a| AccountId(a.0)))
        .ok_or_else(|| "Add an account first.".to_owned())?;
    let item = if bundle {
        ItemRef::bundle(id.0)
    } else {
        ItemRef::asset(id.0)
    };
    let d = core
        .item(item)
        .data
        .ok_or_else(|| "Still loading. Try again in a moment.".to_owned())?;
    let (seller, now) = d
        .card
        .buy_offer()
        .ok_or_else(|| "It's not for sale.".to_owned())?;
    if now != i64::from(price) {
        return Err("The price changed. Look again before buying.".to_owned());
    }
    if d.card.collectible_id.is_empty() {
        return Err("This item can only be bought on the website.".to_owned());
    }
    core.buy_item(
        account,
        d.card.collectible_id.clone(),
        d.card.name.clone(),
        now,
        seller,
    );
    Ok(())
}

/// Buys one reseller's copy. Called only from the dialog `market.buy_resale` opens, after the
/// user saw `price`.
#[tauri::command]
#[specta::specta]
pub fn buy_resale(
    core: State<'_, Arc<Core>>,
    id: Id,
    bundle: bool,
    instance_id: String,
    price: u32,
) -> Result<(), String> {
    let account = core
        .acting(None)
        .ok_or_else(|| "Add an account first.".to_owned())?;
    let item = if bundle {
        ItemRef::bundle(id.0)
    } else {
        ItemRef::asset(id.0)
    };
    let d = core
        .item(item)
        .data
        .ok_or_else(|| "Still loading. Try again in a moment.".to_owned())?;
    if d.card.collectible_id.is_empty() || instance_id.is_empty() {
        return Err("Nobody resells this.".to_owned());
    }
    core.buy_item(
        account,
        d.card.collectible_id.clone(),
        d.card.name.clone(),
        price.into(),
        Seller::Listing(instance_id),
    );
    Ok(())
}

// ---------------------------------------------------------------- commands

/// The item a command is about: `item:<id>`, `bundle:<id>` or a bare asset id in the argument,
/// else the page it ran from.
fn target(inv: &Invocation) -> Result<ItemRef, String> {
    let parse = |s: &str| -> Option<ItemRef> {
        let s = s.trim();
        if let Some(id) = s.strip_prefix("bundle:") {
            return id.parse().ok().map(ItemRef::bundle);
        }
        s.strip_prefix("item:")
            .unwrap_or(s)
            .parse()
            .ok()
            .map(ItemRef::asset)
    };
    inv.arg
        .as_deref()
        .filter(|a| !a.trim().is_empty())
        .and_then(parse)
        .or_else(|| inv.context.as_deref().and_then(parse))
        .ok_or_else(|| "Open an item first.".to_owned())
}

/// Runs a `market.*` or `inventory.*` command, or a `list.more` for one of this module's lists.
/// Returns `None` for anything else.
pub(super) fn run(
    core: &Core,
    id: CommandId,
    inv: &Invocation,
    account: Option<AccountId>,
) -> Option<Result<(), String>> {
    use CommandId as C;
    let need = || account.ok_or_else(|| "Add an account first.".to_owned());
    Some(match id {
        C::SearchMarketplace => Err("Open the Marketplace to search.".to_owned()),
        C::ViewInventory => Err("Open Inventory first.".to_owned()),
        // The item page handles these with a confirm dialog that shows the price.
        C::BuyItem | C::BuyItemAs | C::BuyResale => Err("Open the item to buy it.".to_owned()),
        C::FavoriteItem => (|| {
            let a = need()?;
            let item = target(inv)?;
            let now = core
                .item_favorited(a, item)
                .data
                .ok_or_else(|| "Still loading. Try again in a moment.".to_owned())?;
            core.favorite_item(a, item, !*now);
            Ok(())
        })(),
        C::WishlistItem => (|| {
            let a = need()?;
            let item = target(inv)?;
            let d = core
                .item(item)
                .data
                .ok_or_else(|| "Still loading. Try again in a moment.".to_owned())?;
            core.toggle_wishlist(a, &d.card);
            Ok(())
        })(),
        C::DownloadTemplate | C::DownloadMaskedTemplate => (|| {
            let a = need()?;
            let item = target(inv)?;
            if item.kind != ItemKind::Asset {
                return Err("Only shirts, pants and T-shirts have templates.".to_owned());
            }
            // Names the file. The job checks the asset type itself, so a cold cache only costs
            // the name.
            let name = core
                .item(item)
                .data
                .map(|d| d.card.name.clone())
                .unwrap_or_default();
            core.download_template(a, item.id, &name, id == C::DownloadMaskedTemplate);
            Ok(())
        })(),
        C::ToggleItemValues => {
            let on = !core.values_enabled();
            core.update_settings(|s| s.item_values = on);
            core.notice(
                if on {
                    "Item values from Rolimons are on."
                } else {
                    "Item values are off."
                },
                false,
            );
            Ok(())
        }
        C::DeleteInventoryItem => (|| {
            let a = need()?;
            let asset = target(inv)?;
            if asset.kind != ItemKind::Asset {
                return Err("Bundles can't be deleted.".to_owned());
            }
            core.delete_inventory_item(a, asset.id);
            Ok(())
        })(),
        C::LoadMore => return more(core, inv, account),
        _ => return None,
    })
}

fn more(core: &Core, inv: &Invocation, account: Option<AccountId>) -> Option<Result<(), String>> {
    let arg = inv.arg.as_deref()?.trim();
    let (what, rest) = arg.split_once(':')?;
    let viewer = account.unwrap_or(AccountId::ANONYMOUS);
    let r = match what {
        "catalog" => CatalogQuery::from_id(rest)
            .map(|q| drop(core.catalog_search(&q, true)))
            .ok_or_else(|| "Bad search.".to_owned()),
        "resellers" => {
            drop(core.item_resellers(rest, true));
            Ok(())
        }
        "owners" => rest
            .parse()
            .map(|id| drop(core.item_owners(viewer, id, true)))
            .map_err(|_| "Which item?".to_owned()),
        "passes" => rest
            .parse()
            .map(|id| drop(core.user_passes(id, true)))
            .map_err(|_| "Whose passes?".to_owned()),
        "badges" => rest
            .parse()
            .map(|id| drop(core.user_badges(viewer, id, true)))
            .map_err(|_| "Whose badges?".to_owned()),
        "places" => rest
            .parse()
            .map(|id| drop(core.user_places(id, true)))
            .map_err(|_| "Whose places?".to_owned()),
        "inventory" => (|| {
            let mut parts = rest.splitn(3, ':');
            let user = parts.next().and_then(|u| u.parse().ok());
            let slot = parts.next().and_then(InvSlot::from_key);
            let oldest = parts.next().and_then(|o| o.parse().ok());
            let (Some(user), Some(slot), Some(oldest)) = (user, slot, oldest) else {
                return Err("Which inventory?".to_owned());
            };
            drop(core.inventory(viewer, user, slot, oldest, true));
            Ok(())
        })(),
        _ => return None,
    };
    Some(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inv(arg: Option<&str>, context: Option<&str>) -> Invocation {
        Invocation {
            id: String::new(),
            arg: arg.map(str::to_owned),
            as_account: None,
            context: context.map(str::to_owned),
        }
    }

    #[test]
    fn item_targets() {
        assert_eq!(target(&inv(Some("bundle:5"), None)), Ok(ItemRef::bundle(5)));
        assert_eq!(target(&inv(Some("item:7"), None)), Ok(ItemRef::asset(7)));
        assert_eq!(target(&inv(Some("9"), Some("item:7"))), Ok(ItemRef::asset(9)));
        assert_eq!(target(&inv(None, Some("bundle:3"))), Ok(ItemRef::bundle(3)));
        assert!(target(&inv(None, Some("game:3"))).is_err());
    }
}

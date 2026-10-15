//! Inventories: any user's, as the viewing account is allowed to see it. Items by category,
//! collectibles with RAP, totals (with item values when the user opted in), and deleting
//! items from your own inventory where the site allows it.
//!
//! Lists are keyed by the viewing account: whether an inventory is visible depends on who looks.

use rbx_api::generated::apis::undocumented as apis;
use rbx_api::generated::{catalog, inventory};
use rbx_api::{AccountId, Error};
use serde::{Deserialize, Serialize};

use crate::Core;
use crate::market::{ItemKind, ItemRef};
use crate::res::{Key, Page, Res};
use crate::values::ValueTable;

const CATEGORIES_AGE: i64 = 3_600;
const LIST_AGE: i64 = 300;
const PRIVACY_AGE: i64 = 600;
/// Collectibles are read in full for totals; this caps it at 3,000 items.
const MAX_COLLECTIBLE_PAGES: usize = 30;

/// Which list of a user's inventory.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InvSlot {
    /// Limiteds and collectibles, with RAP.
    #[default]
    Collectibles,
    Asset(i32),
    Bundle(i32),
    /// Badges, from the badges API ([`Core::user_badges`]).
    Badges,
    /// Game passes ([`Core::user_passes`]).
    Passes,
}

impl InvSlot {
    /// Stable string for view state.
    pub fn key(self) -> String {
        match self {
            InvSlot::Collectibles => "collectibles".into(),
            InvSlot::Asset(t) => format!("a{t}"),
            InvSlot::Bundle(t) => format!("b{t}"),
            InvSlot::Badges => "badges".into(),
            InvSlot::Passes => "passes".into(),
        }
    }
    pub fn from_key(k: &str) -> Option<InvSlot> {
        match k {
            "collectibles" => return Some(InvSlot::Collectibles),
            "badges" => return Some(InvSlot::Badges),
            "passes" => return Some(InvSlot::Passes),
            _ => {}
        }
        let (kind, n) = k.split_at_checked(1)?;
        let n = n.parse().ok()?;
        match kind {
            "a" => Some(InvSlot::Asset(n)),
            "b" => Some(InvSlot::Bundle(n)),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct InvCategory {
    pub name: String,
    pub slots: Vec<(String, InvSlot)>,
}

/// One owned copy.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct InvItem {
    pub item: ItemRef,
    pub name: String,
    /// The copy's id (userAssetId); 0 for bundles.
    pub copy_id: u64,
    pub serial: Option<i64>,
    /// Recent average price, for limiteds.
    pub rap: Option<i64>,
    pub original_price: Option<i64>,
    /// Copies left for limited-stock items.
    pub stock: Option<i64>,
    /// Recently traded or bought: can't be traded yet.
    pub on_hold: bool,
    pub acquired: String,
}

/// Totals over a list of collectibles.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct InvTotals {
    pub items: usize,
    pub rap: i64,
    /// Value total, using RAP for items Rolimons doesn't value. `None` without values.
    pub value: Option<i64>,
    pub on_hold: usize,
}

/// Copies of the same item folded into one entry with a count, in first-seen order. Copies
/// with a serial number stay apart: each is its own collectible.
pub fn stack(items: &[InvItem]) -> Vec<(&InvItem, u32)> {
    let mut out: Vec<(&InvItem, u32)> = Vec::new();
    let mut at: std::collections::HashMap<ItemRef, usize> = std::collections::HashMap::new();
    for i in items {
        if i.serial.is_some() {
            out.push((i, 1));
            continue;
        }
        match at.get(&i.item) {
            Some(&n) => out[n].1 += 1,
            None => {
                at.insert(i.item, out.len());
                out.push((i, 1));
            }
        }
    }
    out
}

pub fn totals(items: &[InvItem], values: Option<&ValueTable>) -> InvTotals {
    InvTotals {
        items: items.len(),
        rap: items.iter().filter_map(|i| i.rap).sum(),
        value: values.map(|v| {
            items
                .iter()
                .map(|i| v.get(i.item.id).and_then(|x| x.worth()).or(i.rap).unwrap_or(0))
                .sum()
        }),
        on_hold: items.iter().filter(|i| i.on_hold).count(),
    }
}

/// A game pass someone owns.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OwnedPass {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub icon_asset_id: u64,
    pub creator: String,
    pub creator_id: u64,
    pub creator_is_group: bool,
    pub price: Option<i64>,
}

const PASS_PAGE: i32 = 100;

/// Inventory asset types listed by other APIs, as the site does.
const PLACE_TYPE: i32 = 9;
const BADGE_TYPE: i32 = 21;
const PASS_TYPE: i32 = 34;

impl Core {
    /// The categories a user's inventory has, in the site's order.
    pub fn inventory_categories(&self, user_id: u64) -> Res<Vec<InvCategory>> {
        self.inner.res(
            Key::shared("inv_categories", user_id),
            CATEGORIES_AGE,
            move |f| async move {
                use inventory::v1::AssetsExplorerCategoryItemModelType as T;
                let r = inventory::v1::get_users_by_user_id_categories(&f.client, user_id as i64).await?;
                Ok(r.categories
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|c| {
                        let slots: Vec<(String, InvSlot)> = c
                            .items
                            .unwrap_or_default()
                            .into_iter()
                            .filter_map(|i| {
                                let id = i32::try_from(i.id?).ok()?;
                                let slot = match i.type_? {
                                    T::AssetType if id == PLACE_TYPE => return None,
                                    T::AssetType if id == BADGE_TYPE => InvSlot::Badges,
                                    T::AssetType if id == PASS_TYPE => InvSlot::Passes,
                                    T::AssetType => InvSlot::Asset(id),
                                    T::Bundle => InvSlot::Bundle(id),
                                    _ => return None,
                                };
                                Some((i.display_name.or(i.name).unwrap_or_default(), slot))
                            })
                            .collect();
                        (!slots.is_empty()).then(|| InvCategory {
                            name: c.display_name.or(c.name).unwrap_or_default(),
                            slots,
                        })
                    })
                    .collect())
            },
        )
    }

    /// Whether `viewer` may see `user_id`'s inventory (always true for your own).
    pub fn can_view_inventory(&self, viewer: AccountId, user_id: u64) -> Res<bool> {
        if viewer.0 == user_id {
            return Res {
                data: Some(std::sync::Arc::new(true)),
                ..Default::default()
            };
        }
        self.inner.res(
            Key::private(viewer, "inv_visible", user_id),
            PRIVACY_AGE,
            move |f| async move {
                let r =
                    inventory::v1::get_users_by_user_id_can_view_inventory(&f.client, user_id as i64).await?;
                Ok(r.can_view.unwrap_or(false))
            },
        )
    }

    /// One list of a user's inventory, newest first unless `oldest_first`.
    pub fn inventory(
        &self,
        viewer: AccountId,
        user_id: u64,
        slot: InvSlot,
        oldest_first: bool,
        more: bool,
    ) -> Res<Page<InvItem>> {
        self.inner.paged(
            Key::private(
                viewer,
                "inventory",
                format!("{user_id}:{}:{}", slot.key(), oldest_first),
            ),
            LIST_AGE,
            more,
            move |f, cursor| async move { inventory_page(&f.client, user_id, slot, oldest_first, cursor).await },
        )
    }

    /// Every collectible a user owns, with RAP (up to 3,000).
    pub fn collectibles(&self, viewer: AccountId, user_id: u64) -> Res<Vec<InvItem>> {
        self.inner.res(
            Key::private(viewer, "collectibles", user_id),
            LIST_AGE,
            move |f| async move {
                let mut out = Vec::new();
                let mut cursor = None;
                for _ in 0..MAX_COLLECTIBLE_PAGES {
                    let p = inventory_page(&f.client, user_id, InvSlot::Collectibles, false, cursor).await?;
                    out.extend(p.items);
                    cursor = p.next;
                    if cursor.is_none() {
                        break;
                    }
                }
                Ok(out)
            },
        )
    }

    /// Game passes a user owns, newest first. Public.
    pub fn user_passes(&self, user_id: u64, more: bool) -> Res<Page<OwnedPass>> {
        self.inner.paged(
            Key::shared("user_passes", user_id),
            LIST_AGE,
            more,
            move |f, cursor| async move {
                let p = apis::GetUserGamePassesParams {
                    count: Some(PASS_PAGE),
                    exclusive_start_id: cursor.and_then(|c| c.parse().ok()),
                };
                let r = apis::get_user_game_passes(&f.client, user_id as i64, &p).await?;
                let raw = r.game_passes.unwrap_or_default();
                // The site asks for the page after the last id until one comes back empty; a
                // short page is already the last.
                let next = (raw.len() >= PASS_PAGE as usize)
                    .then(|| raw.last().and_then(|g| g.game_pass_id))
                    .flatten()
                    .map(|id| id.to_string());
                Ok(Page {
                    items: raw
                        .into_iter()
                        .filter_map(|g| {
                            let c = g.creator.unwrap_or_default();
                            Some(OwnedPass {
                                id: u64::try_from(g.game_pass_id?).ok()?,
                                name: g.name.unwrap_or_default(),
                                description: g.description.unwrap_or_default(),
                                icon_asset_id: g.icon_asset_id.unwrap_or(0).max(0) as u64,
                                creator: c.name.unwrap_or_default(),
                                creator_id: c.creator_id.unwrap_or(0).max(0) as u64,
                                creator_is_group: c.creator_type.as_deref() == Some("Group"),
                                price: g.price.filter(|_| g.is_for_sale == Some(true)),
                            })
                        })
                        .collect(),
                    next,
                })
            },
        )
    }

    /// Delete an item from the account's own inventory (asks first in the UI).
    pub fn delete_inventory_item(&self, account: AccountId, asset_id: u64) {
        self.inner.mutate(
            account,
            "delete the item",
            "Deleted from your inventory",
            &["inventory", "owns"],
            move |f| async move {
                inventory::v2::delete_inventory_asset_by_asset_id(&f.client, asset_id as i64).await?;
                Ok(())
            },
        );
    }
}

async fn inventory_page(
    c: &rbx_api::Client,
    user_id: u64,
    slot: InvSlot,
    oldest_first: bool,
    cursor: Option<String>,
) -> Result<Page<InvItem>, Error> {
    let uid = user_id as i64;
    match slot {
        // Shown from their own lists.
        InvSlot::Badges | InvSlot::Passes => Ok(Page::default()),
        InvSlot::Collectibles => {
            let p = inventory::v1::GetUsersByUserIdAssetsCollectiblesParams {
                limit: Some(100),
                cursor,
                sort_order: Some(if oldest_first {
                    inventory::v1::GetUsersByUserIdAssetsCollectiblesSortOrder::Asc
                } else {
                    inventory::v1::GetUsersByUserIdAssetsCollectiblesSortOrder::Desc
                }),
                ..Default::default()
            };
            let r = inventory::v1::get_users_by_user_id_assets_collectibles(c, uid, &p).await?;
            Ok(Page {
                items: r
                    .data
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|x| {
                        Some(InvItem {
                            item: ItemRef::asset(x.asset_id? as u64),
                            name: x.name.unwrap_or_default(),
                            copy_id: x.user_asset_id.unwrap_or(0).max(0) as u64,
                            serial: x.serial_number.filter(|s| *s > 0),
                            rap: x.recent_average_price.filter(|p| *p > 0),
                            original_price: x.original_price.filter(|p| *p > 0),
                            stock: x.asset_stock.filter(|s| *s > 0),
                            on_hold: x.is_on_hold.unwrap_or(false),
                            acquired: String::new(),
                        })
                    })
                    .collect(),
                next: r.next_page_cursor.filter(|c| !c.is_empty()),
            })
        }
        InvSlot::Asset(t) => {
            let p = inventory::v2::GetUsersByUserIdInventoryByAssetTypeIdParams {
                limit: Some(100),
                cursor,
                sort_order: Some(if oldest_first {
                    inventory::v2::GetAssetsByAssetIdOwnersSortOrder::Asc
                } else {
                    inventory::v2::GetAssetsByAssetIdOwnersSortOrder::Desc
                }),
            };
            let r = inventory::v2::get_users_by_user_id_inventory_by_asset_type_id(c, uid, t, &p).await?;
            Ok(Page {
                items: r
                    .data
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|x| {
                        Some(InvItem {
                            item: ItemRef::asset(x.asset_id? as u64),
                            name: x.asset_name.unwrap_or_default(),
                            copy_id: x.user_asset_id.unwrap_or(0).max(0) as u64,
                            serial: x.serial_number.filter(|s| *s > 0),
                            acquired: x.created.unwrap_or_default(),
                            ..Default::default()
                        })
                    })
                    .collect(),
                next: r.next_page_cursor.filter(|c| !c.is_empty()),
            })
        }
        InvSlot::Bundle(t) => {
            use catalog::v1::GetUsersByUserIdBundlesByBundleTypeBundleType as B;
            let kind = match t {
                1 => B::BodyParts,
                2 => B::AvatarAnimations,
                3 => B::Shoes,
                4 => B::DynamicHead,
                _ => return Ok(Page::default()),
            };
            let p = catalog::v1::GetUsersByUserIdBundlesByBundleTypeParams {
                cursor: cursor.unwrap_or_default(),
                limit: Some(100),
                sort_order: Some(if oldest_first {
                    catalog::v1::GetUsersByUserIdBundlesByBundleTypeSortOrder::Asc
                } else {
                    catalog::v1::GetUsersByUserIdBundlesByBundleTypeSortOrder::Desc
                }),
            };
            let r = catalog::v1::get_users_by_user_id_bundles_by_bundle_type(c, uid, kind, &p).await?;
            Ok(Page {
                items: r
                    .data
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|x| {
                        Some(InvItem {
                            item: ItemRef {
                                kind: ItemKind::Bundle,
                                id: x.id? as u64,
                            },
                            name: x.name.unwrap_or_default(),
                            ..Default::default()
                        })
                    })
                    .collect(),
                next: r.next_page_cursor.filter(|c| !c.is_empty()),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::values::ItemValue;

    #[test]
    fn slot_keys_round_trip() {
        for s in [
            InvSlot::Collectibles,
            InvSlot::Asset(8),
            InvSlot::Bundle(1),
            InvSlot::Badges,
            InvSlot::Passes,
        ] {
            assert_eq!(InvSlot::from_key(&s.key()), Some(s));
        }
        assert_eq!(InvSlot::from_key("x8"), None);
        assert_eq!(InvSlot::from_key(""), None);
    }

    #[test]
    fn stacks_copies_without_serials() {
        let item = |id, serial| InvItem {
            item: ItemRef::asset(id),
            serial,
            ..Default::default()
        };
        let items = vec![
            item(1, None),
            item(2, Some(4)),
            item(1, None),
            item(2, Some(9)),
            item(3, None),
        ];
        let s: Vec<(u64, u32)> = stack(&items).iter().map(|(i, n)| (i.item.id, *n)).collect();
        assert_eq!(s, [(1, 2), (2, 1), (2, 1), (3, 1)]);
    }

    #[test]
    fn totals_use_values_then_rap() {
        let item = |id, rap, on_hold| InvItem {
            item: ItemRef::asset(id),
            rap,
            on_hold,
            ..Default::default()
        };
        let items = vec![
            item(1, Some(100), false),
            item(2, Some(50), true),
            item(3, None, false),
        ];
        let t = totals(&items, None);
        assert_eq!((t.items, t.rap, t.value, t.on_hold), (3, 150, None, 1));
        let mut v = ValueTable::default();
        v.items.insert(
            1,
            ItemValue {
                value: Some(400),
                ..Default::default()
            },
        );
        // Item 1 counts at its value, item 2 at RAP, item 3 at nothing.
        assert_eq!(totals(&items, Some(&v)).value, Some(450));
    }
}

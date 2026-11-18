//! The avatar editor: what the active account wears, its outfits and its wardrobe (inventory
//! categories it can wear). `avatar.wear` takes `<asset id>` or `<asset id>:<asset type id>`;
//! with the type, a full slot swaps its oldest item out.

use std::collections::HashSet;
use std::sync::Arc;

use bonsai_core::avatar::{Group, Look, LookSource, Owned, Part, Placement, Rig, Scale, Worn};
use bonsai_core::commands::CommandId;
use bonsai_core::inventory::{InvCategory, InvSlot, NameQuery};
use bonsai_core::market::ItemRef;
use bonsai_core::res::Res;
use bonsai_core::{AccountId, Core};
use serde::Serialize;
use specta::Type;
use tauri::State;

use super::market::{self, Choice, Paged};
use super::{Id, Invocation, ItemCard, List, viewer};

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct WornView {
    pub id: Id,
    pub name: String,
    pub type_id: i32,
    pub type_name: String,
    /// Doesn't show on R6.
    pub r15_only: bool,
    /// Layer of layered clothing; higher sits on top.
    pub layer: Option<i32>,
    /// How a rigid accessory is moved, turned and sized; `None` when it is not adjusted.
    pub placement: Option<PlacementView>,
}

#[derive(Clone, Copy, Debug, Serialize, Type)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl From<[f64; 3]> for Vec3 {
    fn from([x, y, z]: [f64; 3]) -> Self {
        Vec3 { x, y, z }
    }
}

/// Position in studs, rotation in degrees, scale as a factor.
#[derive(Clone, Copy, Debug, Serialize, Type)]
pub struct PlacementView {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

impl From<Placement> for PlacementView {
    fn from(p: Placement) -> Self {
        PlacementView {
            position: p.position.into(),
            rotation: p.rotation.into(),
            scale: p.scale.into(),
        }
    }
}

/// How far accessories of one type can be adjusted.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct AdjustRuleView {
    pub type_id: i32,
    pub min: PlacementView,
    pub max: PlacementView,
}

/// What one press of a slider moves, per group.
#[derive(Clone, Debug, Serialize, Type)]
pub struct AdjustSteps {
    pub position: f64,
    pub rotation: f64,
    pub scale: f64,
}

fn worn_view(a: &Worn) -> WornView {
    WornView {
        id: Id(a.id),
        name: a.name.clone(),
        type_id: a.type_id as i32,
        type_name: a.type_name.clone(),
        r15_only: bonsai_core::avatar::r15_only(a.type_id),
        layer: a.layer,
        placement: a.placement.map(Into::into),
    }
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct AvatarView {
    pub loading: bool,
    pub error: Option<String>,
    /// "R6" or "R15"; empty until loaded.
    pub rig: String,
    pub wearing: Vec<WornView>,
    pub emotes: Vec<WornView>,
    /// One per body part, head first.
    pub colors: Vec<PartColor>,
    /// The scales Roblox lets this account change.
    pub scales: Vec<ScaleView>,
    /// Accessory types that can be moved, turned and sized, with their bounds.
    pub adjust_rules: Vec<AdjustRuleView>,
    pub adjust_steps: AdjustSteps,
    /// Changes when the avatar was read again, so the preview image reloads.
    pub rev: f64,
    /// The look differs from what is worn on Roblox.
    pub edited: bool,
    /// The saved outfit the look started from.
    pub outfit: Option<DraftOutfitView>,
    /// The look differs from that outfit.
    pub outfit_edited: bool,
    /// An outfit is still loading into the editor.
    pub loading_outfit: bool,
    pub can_undo: bool,
    pub can_redo: bool,
}

#[derive(Clone, Debug, Serialize, Type)]
pub struct DraftOutfitView {
    /// Outfit ids can pass 2^53.
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PartColor {
    /// `head`, `torso`, `left_arm`, ...
    pub part: String,
    pub label: String,
    /// BrickColor id; 0 until loaded.
    pub color: i32,
    /// `#RRGGBB`, empty when the palette has not loaded.
    pub hex: String,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ScaleView {
    /// The name `avatar.scale` takes.
    pub name: String,
    pub label: String,
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct SwatchView {
    pub id: i32,
    pub name: String,
    pub hex: String,
    pub basic: bool,
}

fn part_label(p: Part) -> &'static str {
    match p {
        Part::Head => "Head",
        Part::Torso => "Torso",
        Part::LeftArm => "Left arm",
        Part::RightArm => "Right arm",
        Part::LeftLeg => "Left leg",
        Part::RightLeg => "Right leg",
    }
}

fn scale_label(s: Scale) -> &'static str {
    match s {
        Scale::Height => "Height",
        Scale::Width => "Width",
        Scale::Head => "Head",
        Scale::Proportion => "Proportions",
        Scale::BodyType => "Body type",
    }
}

fn part_colors(look: &Look, hex: impl Fn(i32) -> String) -> Vec<PartColor> {
    Part::ALL
        .iter()
        .map(|&p| PartColor {
            part: p.key().to_owned(),
            label: part_label(p).to_owned(),
            color: look.colors.get(p),
            hex: look
                .color_hex
                .get(p.key())
                .cloned()
                .unwrap_or_else(|| hex(look.colors.get(p))),
        })
        .collect()
}

/// The active account's avatar: the look being edited when `edited`, otherwise what is worn
/// on Roblox now.
#[tauri::command]
#[specta::specta]
pub fn avatar(core: State<'_, Arc<Core>>, edited: bool) -> AvatarView {
    let me = viewer(&core);
    let d = if edited {
        core.draft(me)
    } else {
        bonsai_core::avatar::DraftState {
            look: core.avatar(me),
            ..Default::default()
        }
    };
    let r = &d.look;
    let look = r.data.as_deref();
    let rules = core.avatar_rules().data;
    let hex = |id: i32| {
        rules
            .as_deref()
            .and_then(|r| r.palette.iter().find(|s| s.id == id))
            .map(|s| s.hex.clone())
            .unwrap_or_default()
    };
    let colors = look.map(|l| part_colors(l, hex)).unwrap_or_default();
    let scales = match (look, rules.as_deref()) {
        (Some(l), Some(rules)) => Scale::ALL
            .iter()
            .filter(|s| rules.proportions || !matches!(s, Scale::Proportion | Scale::BodyType))
            .filter_map(|&s| {
                let rule = rules.scale(s)?;
                Some(ScaleView {
                    name: s.key().to_owned(),
                    label: scale_label(s).to_owned(),
                    value: s.get(&l.scales),
                    min: rule.min,
                    max: rule.max,
                    step: rule.step,
                })
            })
            .collect(),
        _ => Vec::new(),
    };
    AvatarView {
        loading: r.loading,
        error: r.error.clone(),
        rig: look.map(|l| l.rig.clone()).unwrap_or_default(),
        wearing: look
            .map(|l| l.assets.iter().map(worn_view).collect())
            .unwrap_or_default(),
        emotes: look
            .map(|l| {
                l.emotes
                    .iter()
                    .map(|e| WornView {
                        id: Id(e.id),
                        name: e.name.clone(),
                        type_id: e.slot,
                        type_name: format!("Slot {}", e.slot),
                        r15_only: false,
                        layer: None,
                        placement: None,
                    })
                    .collect()
            })
            .unwrap_or_default(),
        colors,
        scales,
        adjust_rules: rules
            .as_deref()
            .map(|r| {
                r.placements
                    .iter()
                    .map(|p| AdjustRuleView {
                        type_id: p.type_id as i32,
                        min: p.min.into(),
                        max: p.max.into(),
                    })
                    .collect()
            })
            .unwrap_or_default(),
        adjust_steps: AdjustSteps {
            position: Group::Position.step(),
            rotation: Group::Rotation.step(),
            scale: Group::Scale.step(),
        },
        rev: r.fetched_at as f64,
        edited: d.edited,
        outfit: d.outfit.as_ref().map(|o| DraftOutfitView {
            id: o.id.to_string(),
            name: o.name.clone(),
        }),
        outfit_edited: d.outfit_edited,
        loading_outfit: d.loading_outfit,
        can_undo: d.can_undo,
        can_redo: d.can_redo,
    }
}

/// Body colors to pick from, basic ones first.
#[tauri::command]
#[specta::specta]
pub fn body_palette(core: State<'_, Arc<Core>>) -> Vec<SwatchView> {
    let Some(rules) = core.avatar_rules().data else {
        return Vec::new();
    };
    let mut v: Vec<SwatchView> = rules
        .palette
        .iter()
        .map(|s| SwatchView {
            id: s.id,
            name: s.name.clone(),
            hex: s.hex.clone(),
            basic: s.basic,
        })
        .collect();
    v.sort_by_key(|s| !s.basic);
    v
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct RecentView {
    /// Outfit ids can pass 2^53.
    pub id: String,
    pub name: String,
    pub outfit: bool,
    pub type_id: i32,
    pub type_name: String,
}

/// Items and outfits the active account wore lately.
#[tauri::command]
#[specta::specta]
pub fn recent_items(core: State<'_, Arc<Core>>) -> List<RecentView> {
    List::of(core.recent_items(viewer(&core)), |r| RecentView {
        id: r.id.to_string(),
        name: r.name.clone(),
        outfit: r.outfit,
        type_id: r.type_id as i32,
        type_name: r.type_name.clone(),
    })
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct OutfitView {
    /// Outfit ids can pass 2^53.
    pub id: String,
    pub name: String,
    pub editable: bool,
    /// Changes when the list was read again, so the images reload after an outfit changes.
    pub rev: f64,
}

#[tauri::command]
#[specta::specta]
pub fn outfits(core: State<'_, Arc<Core>>) -> Paged<OutfitView> {
    let r = core.outfits(viewer(&core), false);
    let rev = r.fetched_at as f64;
    Paged {
        list: List::paged(r, |o| OutfitView {
            id: o.id.to_string(),
            name: o.name.clone(),
            editable: o.editable,
            rev,
        }),
        more_key: "outfits".to_owned(),
    }
}

/// Someone's look, or an outfit's: what it puts on and the body under it.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct LookView {
    pub loading: bool,
    pub error: Option<String>,
    /// "R6" or "R15"; empty until loaded.
    pub rig: String,
    /// In the order Roblox lists them.
    pub items: Vec<ItemCard>,
    /// One per body part, head first. Empty until loaded.
    pub colors: Vec<PartColor>,
}

/// A look as cards. Catalog details fill in price and creator; items the catalog doesn't list
/// (most classic clothing, anything off sale for years) keep the name and type the avatar gave.
fn look_view(core: &Core, r: Res<Look>) -> LookView {
    let Some(look) = r.data.as_deref() else {
        return LookView {
            loading: r.loading,
            error: r.error,
            rig: String::new(),
            items: Vec::new(),
            colors: Vec::new(),
        };
    };
    let refs: Vec<ItemRef> = look.assets.iter().map(|a| ItemRef::asset(a.id)).collect();
    let catalog = if refs.is_empty() {
        None
    } else {
        core.items(&refs).data
    };
    let values = core.item_values().data;
    let rules = core.avatar_rules().data;
    let hex = |id: i32| {
        rules
            .as_deref()
            .and_then(|r| r.palette.iter().find(|s| s.id == id))
            .map(|s| s.hex.clone())
            .unwrap_or_default()
    };
    LookView {
        loading: r.loading,
        error: r.error.clone(),
        rig: look.rig.clone(),
        items: look
            .assets
            .iter()
            .map(|a| {
                let known = catalog
                    .as_deref()
                    .and_then(|c| c.iter().find(|i| i.item.id == a.id));
                match known {
                    Some(i) => ItemCard {
                        // The catalog gives 0 for some types the avatar knows.
                        asset_type: if i.asset_type == 0 {
                            a.type_id as i32
                        } else {
                            i.asset_type
                        },
                        ..market::card(i, values.as_deref())
                    },
                    None => ItemCard {
                        id: Id(a.id),
                        bundle: false,
                        name: a.name.clone(),
                        creator: String::new(),
                        creator_verified: false,
                        copies: 1,
                        asset_type: a.type_id as i32,
                        price: None,
                        limited: false,
                        serial: None,
                        rap: None,
                        values: None,
                    },
                }
            })
            .collect(),
        colors: part_colors(look, hex),
    }
}

/// What a user wears now. Public.
#[tauri::command]
#[specta::specta]
pub fn profile_look(core: State<'_, Arc<Core>>, user_id: Id) -> LookView {
    look_view(&core, core.user_look(user_id.0))
}

/// Outfits a user made. Public. More loads with `list.more outfits` on their profile.
#[tauri::command]
#[specta::specta]
pub fn profile_outfits(core: State<'_, Arc<Core>>, user_id: Id) -> List<OutfitView> {
    let r = core.user_outfits(user_id.0, false);
    let rev = r.fetched_at as f64;
    List::paged(r, |o| OutfitView {
        id: o.id.to_string(),
        name: o.name.clone(),
        editable: o.editable,
        rev,
    })
}

/// What an outfit puts on. Public.
#[tauri::command]
#[specta::specta]
pub fn outfit_look(core: State<'_, Arc<Core>>, outfit_id: String) -> LookView {
    match outfit_id.parse() {
        Ok(id) => look_view(&core, core.outfit(id)),
        Err(_) => LookView {
            loading: false,
            error: Some("Not an outfit id.".to_owned()),
            rig: String::new(),
            items: Vec::new(),
            colors: Vec::new(),
        },
    }
}

/// A wardrobe section: inventory slots (`a<asset type>`) the avatar can wear.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct WardrobeSection {
    pub name: String,
    pub slots: Vec<Choice>,
}

/// The viewer's wearable inventory slots by section, or the list state to show instead.
fn wearable_sections<T>(core: &Core, a: AccountId) -> Result<Vec<InvCategory>, List<T>> {
    if a.is_anonymous() {
        return Err(List {
            loading: false,
            error: Some("Sign in to change your avatar.".to_owned()),
            items: Vec::new(),
            more: false,
        });
    }
    let rules = core.avatar_rules();
    let Some(rules) = rules.data else {
        return Err(List {
            loading: rules.loading || rules.error.is_none(),
            error: rules.error,
            items: Vec::new(),
            more: false,
        });
    };
    let cats = core.inventory_categories(a.0);
    let Some(data) = cats.data else {
        return Err(List {
            loading: cats.loading || cats.error.is_none(),
            error: cats.error,
            items: Vec::new(),
            more: false,
        });
    };
    let wearable = |s: &InvSlot| match s {
        InvSlot::Asset(t) => rules.slots.iter().any(|w| w.type_id == *t as i64 && w.max > 0),
        _ => false,
    };
    Ok(data
        .iter()
        .map(|c| {
            let slots = c.slots.iter().filter(|(_, s)| wearable(s)).cloned().collect();
            InvCategory {
                name: c.name.clone(),
                slots,
            }
        })
        .filter(|c| !c.slots.is_empty())
        .collect())
}

#[tauri::command]
#[specta::specta]
pub fn wardrobe(core: State<'_, Arc<Core>>) -> List<WardrobeSection> {
    let a = viewer(&core);
    match wearable_sections(&core, a) {
        Err(state) => state,
        Ok(sections) => List {
            loading: false,
            error: None,
            more: false,
            items: sections
                .into_iter()
                .map(|c| WardrobeSection {
                    name: c.name,
                    slots: c
                        .slots
                        .into_iter()
                        .map(|(label, s)| Choice {
                            value: s.key(),
                            label,
                        })
                        .collect(),
                })
                .collect(),
        },
    }
}

/// Most results a wardrobe search returns.
const SEARCH_MAX: usize = 300;

/// An item found by a wardrobe search, with the slot it was found in.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct WardrobeHit {
    pub item: ItemCard,
    pub slot: String,
}

/// Wardrobe search results across every wearable type.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct WardrobeSearch {
    pub loading: bool,
    pub error: Option<String>,
    pub items: Vec<WardrobeHit>,
    /// Types whose first page has loaded (or failed), of `total`.
    pub loaded: u32,
    pub total: u32,
    /// Types that failed to load.
    pub failed: u32,
    /// Some types have more pages than were searched.
    pub partial: bool,
    /// More matches than `items` holds.
    pub truncated: bool,
}

/// Searches item names in every wearable inventory type. Types not loaded yet start loading in
/// the background; their matches arrive with later `storeChanged` events.
#[tauri::command]
#[specta::specta]
pub fn wardrobe_search(core: State<'_, Arc<Core>>, query: String) -> WardrobeSearch {
    let a = viewer(&core);
    let mut out = WardrobeSearch {
        loading: false,
        error: None,
        items: Vec::new(),
        loaded: 0,
        total: 0,
        failed: 0,
        partial: false,
        truncated: false,
    };
    let sections = match wearable_sections::<()>(&core, a) {
        Ok(s) => s,
        Err(state) => {
            out.loading = state.loading;
            out.error = state.error;
            return out;
        }
    };
    let query = NameQuery::new(&query);
    if query.is_empty() {
        return out;
    }
    let mut seen = HashSet::new();
    for (label, slot) in sections.into_iter().flat_map(|c| c.slots) {
        let InvSlot::Asset(t) = slot else { continue };
        if !seen.insert(t) {
            continue;
        }
        out.total += 1;
        let r = core.inventory(a, a.0, slot, false, false);
        out.loading |= r.loading;
        let Some(page) = r.data else {
            if r.error.is_some() {
                out.loaded += 1;
                out.failed += 1;
            }
            continue;
        };
        out.loaded += 1;
        out.partial |= page.next.is_some();
        let cards = market::inventory_cards(&core, &page.items, t, true, |i| query.matches(&i.name));
        out.items.extend(cards.into_iter().map(|item| WardrobeHit {
            item,
            slot: label.clone(),
        }));
    }
    if out.items.len() > SEARCH_MAX {
        out.items.truncate(SEARCH_MAX);
        out.truncated = true;
    }
    if out.failed > 0 && out.failed == out.total {
        out.error = Some("Couldn't load your inventory.".to_owned());
    }
    out
}

/// One item of a look being copied, and whether the target account owns it.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct CopyItemView {
    pub id: Id,
    pub name: String,
    pub type_name: String,
    /// "yes", "no", "checking" or "unknown".
    pub owned: &'static str,
}

#[derive(Clone, Debug, Default, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct CopyPlanView {
    pub loading: bool,
    pub error: Option<String>,
    /// "R6" or "R15"; empty until loaded.
    pub rig: String,
    pub items: Vec<CopyItemView>,
    /// Every item has been checked; the copy can go ahead.
    pub ready: bool,
    /// What the copy wears: the items the target owns.
    pub wear: Vec<Id>,
}

/// What copying a look (`outfit:<id>`, `avatar:<user id>`, or empty for the active account's
/// avatar) to `target` would wear and what it lacks. Reads only.
#[tauri::command]
#[specta::specta]
pub fn copy_look_plan(core: State<'_, Arc<Core>>, source: String, target: Id) -> CopyPlanView {
    let from = match LookSource::parse(&source, core.acting(None)) {
        Ok(f) => f,
        Err(e) => {
            return CopyPlanView {
                error: Some(e),
                ..Default::default()
            };
        }
    };
    let plan = core.copy_plan(from, AccountId(target.0));
    CopyPlanView {
        loading: plan.loading,
        error: plan.error.clone(),
        rig: plan.rig.clone(),
        ready: plan.checked(),
        wear: plan.wearable().into_iter().map(Id).collect(),
        items: plan
            .items
            .into_iter()
            .map(|(w, o)| CopyItemView {
                id: Id(w.id),
                name: w.name,
                type_name: w.type_name,
                owned: match o {
                    Owned::Yes => "yes",
                    Owned::No => "no",
                    Owned::Checking => "checking",
                    Owned::Unknown => "unknown",
                },
            })
            .collect(),
    }
}

/// The confirm step of the dialog `avatar.copy_to` opens: wears `wear` (the items the plan
/// found `target` owns) and the look's body type, colors and scales on `target`.
#[tauri::command]
#[specta::specta]
pub fn copy_look(
    core: State<'_, Arc<Core>>,
    source: String,
    target: Id,
    wear: Vec<Id>,
) -> Result<(), String> {
    let from = LookSource::parse(&source, core.acting(None))?;
    core.copy_look(from, AccountId(target.0), wear.into_iter().map(|i| i.0).collect())
}

pub(super) fn run(
    core: &Core,
    id: CommandId,
    inv: &Invocation,
    account: Option<AccountId>,
) -> Option<Result<(), String>> {
    let arg = inv.arg.as_deref().map(str::trim).filter(|a| !a.is_empty());
    let need = || account.ok_or_else(|| "no account".to_owned());
    let r = match id {
        CommandId::WearItem => (|| {
            let a = need()?;
            let text = arg.ok_or("Which item?")?;
            let (asset, t) = text.split_once(':').unwrap_or((text, ""));
            let asset = asset.parse().map_err(|_| "Not an item id.".to_owned())?;
            core.toggle_wear(a, asset, t.parse().ok());
            Ok(())
        })(),
        CommandId::WearOutfit => (|| {
            let a = need()?;
            let outfit = arg.and_then(|o| o.parse().ok()).ok_or("Not an outfit id.")?;
            core.wear_outfit(a, outfit);
            Ok(())
        })(),
        CommandId::SaveOutfit => (|| core.save_outfit(need()?, arg.unwrap_or("")))(),
        CommandId::UpdateOutfit => (|| {
            let outfit = arg.and_then(|o| o.parse().ok()).ok_or("Not an outfit id.")?;
            core.update_outfit(need()?, outfit);
            Ok(())
        })(),
        CommandId::RenameOutfit => (|| {
            let a = need()?;
            let (outfit, name) = arg
                .and_then(|t| t.split_once(' '))
                .ok_or("Which outfit, and what name?")?;
            let outfit = outfit.parse().map_err(|_| "Not an outfit id.".to_owned())?;
            core.rename_outfit(a, outfit, name)
        })(),
        CommandId::DeleteOutfit => (|| {
            let outfit = arg.and_then(|o| o.parse().ok()).ok_or("Not an outfit id.")?;
            core.delete_outfit(need()?, outfit);
            Ok(())
        })(),
        CommandId::MoveLayer => (|| {
            let a = need()?;
            let (asset, dir) = arg
                .and_then(|t| t.split_once(' '))
                .ok_or("Which item, and up or down?")?;
            let asset = asset.parse().map_err(|_| "Not an item id.".to_owned())?;
            let outward = match dir.trim().to_ascii_lowercase().as_str() {
                "up" | "out" | "top" => true,
                "down" | "in" | "under" => false,
                _ => return Err("Say up or down.".to_owned()),
            };
            core.draft_layer(a, asset, outward)
        })(),
        CommandId::AdjustAccessory => (|| {
            let a = need()?;
            let (asset, adjust) = bonsai_core::avatar::placement::parse(arg.unwrap_or(""))?;
            core.draft_adjust(a, asset, adjust)
        })(),
        CommandId::SetBodyColor => (|| {
            let a = need()?;
            let text = arg.ok_or("Which color?")?;
            let (parts, color) = match text.rsplit_once(':') {
                Some((parts, c)) => (
                    parts
                        .split(',')
                        .map(|p| Part::parse(p).ok_or_else(|| format!("No body part called {}.", p.trim())))
                        .collect::<Result<Vec<_>, _>>()?,
                    c,
                ),
                None => (Vec::new(), text),
            };
            if color.trim().starts_with('#') {
                return core.draft_color_hex(a, parts, color);
            }
            let color = color
                .trim()
                .parse()
                .map_err(|_| "Use a color id or #RRGGBB.".to_owned())?;
            core.draft_color(a, parts, color)
        })(),
        CommandId::SetScale => (|| {
            let a = need()?;
            let (name, value) = arg
                .and_then(|t| t.rsplit_once(' '))
                .ok_or("Which scale, and what value?")?;
            let scale = Scale::parse(name).ok_or("Pick height, width, head, proportion or body type.")?;
            let value = value
                .trim()
                .trim_end_matches('%')
                .parse::<f64>()
                .map_err(|_| "Not a number.".to_owned())?;
            core.draft_scale(a, scale, value)
        })(),
        CommandId::SetEmote => (|| {
            let a = need()?;
            let text = arg.ok_or("Which slot?")?;
            let mut it = text.split_whitespace();
            let slot = it
                .next()
                .and_then(|s| s.parse().ok())
                .ok_or("Say a slot from 1 to 8.")?;
            let asset = match it.next() {
                Some(id) => Some(id.parse().map_err(|_| "Not an item id.".to_owned())?),
                None => None,
            };
            core.set_emote(a, slot, asset)
        })(),
        CommandId::RedrawAvatar => (|| {
            core.redraw_avatar(need()?);
            Ok(())
        })(),
        CommandId::SetRig => (|| {
            let a = need()?;
            // No argument: switch to the other one.
            let rig = match arg.map(Rig::parse) {
                Some(None) => return Err("Pick R6 or R15.".to_owned()),
                r => r.flatten(),
            };
            core.draft_rig(a, rig)
        })(),
        CommandId::TryItem => (|| {
            let a = need()?;
            let text = arg.ok_or("Which item?")?;
            let (asset, t) = text.split_once(':').unwrap_or((text, ""));
            let asset = asset.parse().map_err(|_| "Not an item id.".to_owned())?;
            core.draft_wear(a, asset, t.parse().ok())
        })(),
        CommandId::LoadOutfit => (|| {
            let a = need()?;
            let outfit = arg.and_then(|o| o.parse().ok()).ok_or("Not an outfit id.")?;
            core.load_outfit(a, outfit)
        })(),
        CommandId::WearDraft => (|| core.wear_draft(need()?))(),
        CommandId::UndoAvatar => (|| core.draft_undo(need()?))(),
        CommandId::RedoAvatar => (|| core.draft_redo(need()?))(),
        CommandId::RevertAvatar => (|| core.revert_draft(need()?))(),
        CommandId::LoadMore if arg == Some("outfits") => {
            drop(core.outfits(account.unwrap_or(AccountId::ANONYMOUS), true));
            Ok(())
        }
        _ => return None,
    };
    Some(r)
}

/// Whose avatar the 3D viewer shows.
#[derive(Clone, Debug, serde::Deserialize, Type)]
#[serde(tag = "kind", content = "id", rename_all = "camelCase")]
pub enum AvatarSource {
    /// The active account's avatar.
    Me,
    /// The look being edited in the avatar editor.
    Draft,
    /// A user's current avatar. Public.
    User(Id),
    /// A saved outfit. Public. The id is a string because outfit ids can exceed 2^53.
    Outfit(String),
}

/// What the 3D viewer needs to build an avatar: rig, assets, BrickColor ids and scales.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Avatar3dView {
    pub loading: bool,
    pub error: Option<String>,
    /// "R6" or "R15"; empty until loaded.
    pub rig: String,
    pub assets: Vec<WornView>,
    /// Equipped emotes; `typeId` is the slot.
    pub emotes: Vec<WornView>,
    /// BrickColor ids: head, torso, left arm, right arm, left leg, right leg.
    pub colors: Vec<i32>,
    pub scales: Avatar3dScales,
    /// Changes when the avatar was read again.
    pub rev: f64,
}

#[derive(Clone, Debug, Default, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Avatar3dScales {
    pub height: f64,
    pub width: f64,
    pub head: f64,
    pub depth: f64,
    pub proportion: f64,
    pub body_type: f64,
}

#[tauri::command]
#[specta::specta]
pub fn avatar_3d(core: State<'_, Arc<Core>>, source: AvatarSource) -> Avatar3dView {
    let r = match source {
        AvatarSource::Me => core.avatar(viewer(&core)),
        AvatarSource::Draft => core.draft(viewer(&core)).look,
        AvatarSource::User(id) => core.user_look(id.0),
        AvatarSource::Outfit(id) => match id.parse() {
            Ok(id) => core.outfit(id),
            Err(_) => Res {
                error: Some("Not an outfit id.".to_owned()),
                ..Res::default()
            },
        },
    };
    avatar_3d_view(r)
}

fn avatar_3d_view(r: Res<Look>) -> Avatar3dView {
    let look = r.data.as_deref();
    Avatar3dView {
        loading: r.loading,
        error: r.error.clone(),
        rig: look.map(|l| l.rig.clone()).unwrap_or_default(),
        assets: look
            .map(|l| l.assets.iter().map(worn_view).collect())
            .unwrap_or_default(),
        emotes: look
            .map(|l| {
                l.emotes
                    .iter()
                    .map(|e| WornView {
                        id: Id(e.id),
                        name: e.name.clone(),
                        type_id: e.slot,
                        type_name: "Emote".to_owned(),
                        r15_only: true,
                        layer: None,
                        placement: None,
                    })
                    .collect()
            })
            .unwrap_or_default(),
        colors: look
            .map(|l| Part::ALL.iter().map(|&p| l.colors.get(p)).collect())
            .unwrap_or_default(),
        scales: look
            .map(|l| Avatar3dScales {
                height: l.scales.height,
                width: l.scales.width,
                head: l.scales.head,
                depth: l.scales.depth,
                proportion: l.scales.proportion,
                body_type: l.scales.body_type,
            })
            .unwrap_or_default(),
        rev: r.fetched_at as f64,
    }
}

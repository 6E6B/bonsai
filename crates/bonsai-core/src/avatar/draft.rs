//! The look being put together in the avatar editor, before it is worn.
//!
//! Edits change a per-account draft kept in memory, never Roblox, so the editor can show them at
//! once. [`Core::wear_draft`] writes the draft to Roblox, and saving an outfit saves it. Every
//! edit, including loading an outfit, can be undone.
//!
//! A draft that matches what is worn is dropped, so the editor follows changes made elsewhere
//! until the next edit.

use std::sync::Arc;

use bonsai_api::generated::avatar;
use bonsai_api::{AccountId, Error};

use super::{
    Adjust, Look, Part, Recent, Rig, Rules, Scale, Scales, Worn, convert, exact_colors, forget_images,
    forget_outfit_image, normalize_body_hex, not_adjustable, note_worn, placement, scales, scales_model,
    set_rig, set_wearing, snap,
};
use crate::Core;
use crate::effects::Effect;
use crate::market::ItemRef;
use crate::res::{Key, Res};

/// Steps kept for undo.
const HISTORY: usize = 100;
/// Layered clothing asset types; each takes a layer order.
const LAYERED: std::ops::RangeInclusive<i64> = 64..=72;
const NOTHING: &str = "You are already wearing this look.";

/// The saved outfit a draft started from.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DraftOutfit {
    pub id: u64,
    pub name: String,
}

#[derive(Clone, Debug, Default)]
struct Step {
    /// `None` while the draft matches what is worn.
    look: Option<Look>,
    outfit: Option<DraftOutfit>,
    /// The outfit's contents as last loaded or saved.
    saved: Option<Look>,
}

#[derive(Debug, Default)]
pub(crate) struct Draft {
    now: Step,
    /// An outfit asked for whose contents are still loading.
    pending: Option<u64>,
    undo: Vec<Step>,
    redo: Vec<Step>,
}

/// The draft as the editor shows it.
#[derive(Clone, Debug, Default)]
pub struct DraftState {
    /// The draft, or what is worn when there are no edits. Emotes are always the worn ones.
    pub look: Res<Look>,
    /// Differs from what is worn.
    pub edited: bool,
    pub outfit: Option<DraftOutfit>,
    /// Differs from the outfit it started from.
    pub outfit_edited: bool,
    /// An outfit is still loading into the draft.
    pub loading_outfit: bool,
    pub can_undo: bool,
    pub can_redo: bool,
}

/// The same look to wear: names and emotes aside, placements within rounding, layered items in
/// the same order, and an unknown rig matching either.
pub fn same(a: &Look, b: &Look) -> bool {
    if !a.rig.is_empty() && !b.rig.is_empty() && a.rig != b.rig {
        return false;
    }
    if a.assets.len() != b.assets.len() {
        return false;
    }
    let placed = |w: &Worn| w.placement.unwrap_or_default();
    let assets = a.assets.iter().all(|x| {
        b.assets
            .iter()
            .find(|y| y.id == x.id)
            .is_some_and(|y| placed(x).same(&placed(y)))
    });
    assets
        && layer_order(a) == layer_order(b)
        && Part::ALL.iter().all(|&p| same_color(a, b, p))
        && same_scales(&a.scales, &b.scales)
}

fn layer_order(l: &Look) -> Vec<u64> {
    let mut v: Vec<(i32, u64)> = l.assets.iter().filter_map(|a| Some((a.layer?, a.id))).collect();
    v.sort();
    v.into_iter().map(|(_, id)| id).collect()
}

/// `RRGGBB` of a part's exact color, when known.
fn part_hex(l: &Look, p: Part) -> Option<String> {
    l.color_hex
        .get(p.key())
        .map(|h| h.trim_start_matches('#').to_ascii_uppercase())
}

fn same_color(a: &Look, b: &Look, p: Part) -> bool {
    match (part_hex(a, p), part_hex(b, p)) {
        (Some(x), Some(y)) => x == y,
        _ => a.colors.get(p) == b.colors.get(p),
    }
}

fn same_scales(a: &Scales, b: &Scales) -> bool {
    [
        (a.height, b.height),
        (a.width, b.width),
        (a.head, b.head),
        (a.depth, b.depth),
        (a.proportion, b.proportion),
        (a.body_type, b.body_type),
    ]
    .iter()
    .all(|(x, y)| (x - y).abs() < 1e-3)
}

/// Put `item` on, or take it off when it is worn. A full slot (`max` of its type) drops its
/// oldest item; new layered clothing goes on top.
pub(crate) fn toggle(assets: &mut Vec<Worn>, mut item: Worn, max: Option<i32>) {
    let before = assets.len();
    assets.retain(|a| a.id != item.id);
    if assets.len() < before {
        return;
    }
    if let Some(max) = max {
        let t = item.type_id;
        let n = assets.iter().filter(|a| a.type_id == t).count();
        let mut extra = (n + 1).saturating_sub(max.max(1) as usize);
        assets.retain(|a| {
            if extra > 0 && a.type_id == t {
                extra -= 1;
                false
            } else {
                true
            }
        });
    }
    if LAYERED.contains(&item.type_id) && item.layer.is_none() {
        item.layer = Some(assets.iter().filter_map(|a| a.layer).max().map_or(1, |m| m + 1));
    }
    assets.push(item);
}

/// Move `asset` one layer out (on top) or in, by swapping layer orders with its neighbour.
pub(crate) fn relayer(assets: &mut [Worn], asset: u64, outward: bool) -> Result<(), String> {
    let mut layers: Vec<(i32, usize)> = assets
        .iter()
        .enumerate()
        .filter_map(|(i, a)| Some((a.layer?, i)))
        .collect();
    layers.sort();
    let at = layers
        .iter()
        .position(|&(_, i)| assets[i].id == asset)
        .ok_or("That item has no layer to move.")?;
    let other = if outward {
        at + 1
    } else {
        at.checked_sub(1).ok_or("It is already the innermost layer.")?
    };
    let &(_, j) = layers.get(other).ok_or("It is already the outermost layer.")?;
    let i = layers[at].1;
    // Number the layers 1..n in their current order first, so equal orders can't hide a move
    // or collide with a third item, then swap the two neighbours.
    for (k, &(_, idx)) in layers.iter().enumerate() {
        assets[idx].layer = Some(k as i32 + 1);
    }
    assets[i].layer = Some(other as i32 + 1);
    assets[j].layer = Some(at as i32 + 1);
    Ok(())
}

/// Paint `parts` (every part when empty) `hex`, with BrickColor `id` when it is a palette color.
pub(crate) fn paint(look: &mut Look, parts: &[Part], id: Option<i32>, hex: &str) {
    for p in Part::ALL {
        if !parts.is_empty() && !parts.contains(&p) {
            continue;
        }
        look.color_hex.insert(p.key().to_owned(), hex.to_owned());
        if let Some(id) = id {
            let c = &mut look.colors;
            *match p {
                Part::Head => &mut c.head,
                Part::Torso => &mut c.torso,
                Part::LeftArm => &mut c.left_arm,
                Part::RightArm => &mut c.right_arm,
                Part::LeftLeg => &mut c.left_leg,
                Part::RightLeg => &mut c.right_leg,
            } = id;
        }
    }
}

/// What Roblox needs to wear `assets`: their metadata, with layer order and placement written in.
fn wear_models(assets: &[Worn]) -> Vec<avatar::v2::AssetWearModel> {
    assets
        .iter()
        .map(|a| {
            let mut meta = a.meta.clone();
            if let Some(l) = a.layer {
                let m = meta.get_or_insert_with(Default::default);
                m.order = Some(l);
                m.version.get_or_insert(1);
            }
            if let Some(p) = a.placement {
                meta = Some(placement::to_meta(meta, p));
            }
            avatar::v2::AssetWearModel {
                id: Some(a.id as i64),
                meta: convert(meta),
            }
        })
        .collect()
}

/// Exact colors for every part `look` knows, using `palette` for parts with only an id.
fn body_colors(look: &Look, rules: Option<&Rules>) -> avatar::v2::BodyColorsModelV2 {
    let hex = |p: Part| {
        part_hex(look, p).or_else(|| {
            let id = look.colors.get(p);
            rules?
                .palette
                .iter()
                .find(|s| s.id == id)
                .map(|s| s.hex.trim_start_matches('#').to_ascii_uppercase())
        })
    };
    avatar::v2::BodyColorsModelV2 {
        head_color3: hex(Part::Head),
        torso_color3: hex(Part::Torso),
        left_arm_color3: hex(Part::LeftArm),
        right_arm_color3: hex(Part::RightArm),
        left_leg_color3: hex(Part::LeftLeg),
        right_leg_color3: hex(Part::RightLeg),
    }
}

fn color_list(c: &avatar::v2::BodyColorsModelV2) -> [Option<String>; 6] {
    [
        &c.head_color3,
        &c.torso_color3,
        &c.left_arm_color3,
        &c.right_arm_color3,
        &c.left_leg_color3,
        &c.right_leg_color3,
    ]
    .map(|h| h.as_ref().map(|h| h.to_ascii_uppercase()))
}

/// `look` as outfit contents. `name` is left out when `None`, keeping the outfit's own.
fn outfit_body(
    look: &Look,
    name: Option<String>,
    colors: &avatar::v2::BodyColorsModelV2,
) -> avatar::v3::OutfitUpdateModelV3 {
    avatar::v3::OutfitUpdateModelV3 {
        assets: convert(Some(wear_models(&look.assets))),
        body_color3s: convert(Some(colors.clone())),
        name,
        outfit_type: Some(1),
        player_avatar_type: Rig::parse(&look.rig).map(|_| look.rig.clone()),
        scale: convert(Some(scales_model(look.scales))),
    }
}

impl Core {
    /// The draft `account` is editing: what is worn plus its edits.
    pub fn draft(&self, account: AccountId) -> DraftState {
        let worn = self.avatar(account);
        self.resolve_pending(account, &worn);
        let Some(base) = worn.data.clone() else {
            return DraftState {
                look: worn,
                ..Default::default()
            };
        };
        let (look, mut state) = {
            let mut drafts = self.inner.drafts.lock();
            let d = drafts.entry(account).or_default();
            // Caught up: what is worn now is the draft, through Wear or from elsewhere.
            if d.now.look.as_ref().is_some_and(|l| same(l, &base)) {
                d.now.look = None;
            }
            let mut look = d.now.look.clone().unwrap_or_else(|| (*base).clone());
            look.emotes = base.emotes.clone();
            let state = DraftState {
                edited: d.now.look.is_some(),
                outfit: d.now.outfit.clone(),
                outfit_edited: d.now.saved.as_ref().is_some_and(|s| !same(s, &look)),
                loading_outfit: d.pending.is_some(),
                can_undo: !d.undo.is_empty(),
                can_redo: !d.redo.is_empty(),
                ..Default::default()
            };
            (look, state)
        };
        let look = self.named(look);
        state.look = Res {
            data: Some(Arc::new(look)),
            ..worn
        };
        state
    }

    /// Items put on in the draft are known by id; their names come from the item cache.
    fn named(&self, mut look: Look) -> Look {
        for a in look.assets.iter_mut().filter(|a| a.name.is_empty()) {
            if let Some(d) = self.item(ItemRef::asset(a.id)).data {
                a.name = d.card.name.clone();
            }
        }
        look
    }

    /// Finish loading an outfit asked for before its contents arrived.
    fn resolve_pending(&self, account: AccountId, worn: &Res<Look>) {
        let Some(id) = self.inner.drafts.lock().get(&account).and_then(|d| d.pending) else {
            return;
        };
        let r = self.outfit(id);
        if let Some(look) = r.data {
            if worn.data.is_some() {
                let _ = self.take_outfit(account, id, &look);
            }
        } else if let Some(e) = r.error {
            if let Some(d) = self.inner.drafts.lock().get_mut(&account) {
                d.pending = None;
            }
            self.inner
                .toast(Some(account), format!("Could not load the outfit: {e}"), true);
        }
    }

    /// Change the draft. `f` gets the draft (or what is worn) and may refuse the change.
    fn edit(
        &self,
        account: AccountId,
        f: impl FnOnce(&mut Look) -> Result<(), String>,
    ) -> Result<(), String> {
        let worn = self
            .avatar(account)
            .data
            .ok_or("Your avatar has not loaded yet.")?;
        {
            let mut drafts = self.inner.drafts.lock();
            let d = drafts.entry(account).or_default();
            let before = d.now.look.clone().unwrap_or_else(|| (*worn).clone());
            let mut look = before.clone();
            f(&mut look)?;
            if same(&look, &before) {
                return Ok(());
            }
            let next = Step {
                look: (!same(&look, &worn)).then_some(look),
                ..d.now.clone()
            };
            push(d, next);
        }
        self.inner.repaint();
        Ok(())
    }

    /// Put an item on in the draft, or take it off. A full slot drops its oldest item.
    pub fn draft_wear(&self, account: AccountId, asset: u64, type_id: Option<i64>) -> Result<(), String> {
        let slot = type_id.and_then(|t| self.wear_slots()?.into_iter().find(|s| s.type_id == t));
        let max = slot.as_ref().map(|s| s.max);
        let item = Worn {
            id: asset,
            type_id: type_id.unwrap_or(0),
            type_name: slot.map(|s| s.name).unwrap_or_default(),
            ..Default::default()
        };
        self.edit(account, |l| {
            toggle(&mut l.assets, item, max);
            Ok(())
        })
    }

    /// Move a layered item one layer out (on top) or in, in the draft.
    pub fn draft_layer(&self, account: AccountId, asset: u64, outward: bool) -> Result<(), String> {
        self.edit(account, |l| relayer(&mut l.assets, asset, outward))
    }

    /// Paint `parts` (every part when empty) a palette color, in the draft.
    pub fn draft_color(&self, account: AccountId, parts: Vec<Part>, color: i32) -> Result<(), String> {
        let rules = self
            .avatar_rules()
            .data
            .ok_or("The body color palette has not loaded yet.")?;
        let hex = rules
            .palette
            .iter()
            .find(|s| s.id == color)
            .ok_or("That color is not in the palette.")?
            .hex
            .clone();
        let hex = normalize_body_hex(&hex)?;
        self.edit(account, |l| {
            paint(l, &parts, Some(color), &hex);
            Ok(())
        })
    }

    /// Paint `parts` (every part when empty) an exact RGB color, in the draft.
    pub fn draft_color_hex(&self, account: AccountId, parts: Vec<Part>, color: &str) -> Result<(), String> {
        let hex = normalize_body_hex(color)?;
        self.edit(account, |l| {
            paint(l, &parts, None, &hex);
            Ok(())
        })
    }

    /// Set one body scale in the draft, moved onto the range Roblox allows.
    pub fn draft_scale(&self, account: AccountId, scale: Scale, value: f64) -> Result<(), String> {
        let rules = self
            .avatar_rules()
            .data
            .ok_or("The avatar rules have not loaded yet.")?;
        let rule = rules
            .scale(scale)
            .ok_or("Roblox does not allow changing that scale.")?;
        if matches!(scale, Scale::Proportion | Scale::BodyType) && !rules.proportions {
            return Err("Roblox does not let this account change proportions.".into());
        }
        if !value.is_finite() {
            return Err("Not a number.".into());
        }
        let value = snap(value, rule);
        self.edit(account, |l| {
            scale.set(&mut l.scales, value);
            Ok(())
        })
    }

    /// Switch the draft to R6 or R15; `None` switches to the other one.
    pub fn draft_rig(&self, account: AccountId, rig: Option<Rig>) -> Result<(), String> {
        self.edit(account, |l| {
            let rig = rig.unwrap_or(if l.rig == "R6" { Rig::R15 } else { Rig::R6 });
            l.rig = match rig {
                Rig::R6 => "R6",
                Rig::R15 => "R15",
            }
            .into();
            Ok(())
        })
    }

    /// Move, turn or size an accessory in the draft, within the bounds Roblox allows its type.
    pub fn draft_adjust(&self, account: AccountId, asset: u64, adjust: Adjust) -> Result<(), String> {
        let rules = self
            .avatar_rules()
            .data
            .ok_or("The avatar rules have not loaded yet.")?;
        self.edit(account, |l| {
            let find = |id: u64| l.assets.iter().position(|a| a.id == id);
            let i = find(asset).ok_or("That item is not worn.")?;
            let rule = rules
                .placement(l.assets[i].type_id)
                .ok_or_else(|| not_adjustable(&l.assets[i].type_name))?;
            let source = match adjust.source() {
                Some(from) => Some(
                    l.assets[find(from).ok_or("The item to copy from is not worn.")?]
                        .placement
                        .unwrap_or_default(),
                ),
                None => None,
            };
            let current = l.assets[i].placement.unwrap_or_default();
            l.assets[i].placement = Some(placement::apply(current, &adjust, source, rule)?);
            Ok(())
        })
    }

    /// Load one of `account`'s outfits into the draft. Undo goes back to the look before.
    pub fn load_outfit(&self, account: AccountId, outfit: u64) -> Result<(), String> {
        let r = self.outfit(outfit);
        if let Some(look) = r.data {
            return self.take_outfit(account, outfit, &look);
        }
        if let Some(e) = r.error {
            return Err(e);
        }
        self.inner.drafts.lock().entry(account).or_default().pending = Some(outfit);
        self.inner.repaint();
        Ok(())
    }

    fn take_outfit(&self, account: AccountId, id: u64, look: &Look) -> Result<(), String> {
        let worn = self
            .avatar(account)
            .data
            .ok_or("Your avatar has not loaded yet.")?;
        let name = self
            .outfits(account, false)
            .data
            .and_then(|p| p.items.iter().find(|o| o.id == id).map(|o| o.name.clone()))
            .unwrap_or_default();
        let mut look = look.clone();
        if look.rig.is_empty() {
            look.rig = worn.rig.clone();
        }
        look.emotes = worn.emotes.clone();
        {
            let mut drafts = self.inner.drafts.lock();
            let d = drafts.entry(account).or_default();
            d.pending = None;
            let next = Step {
                look: (!same(&look, &worn)).then(|| look.clone()),
                outfit: Some(DraftOutfit { id, name }),
                saved: Some(look),
            };
            push(d, next);
        }
        self.inner.repaint();
        Ok(())
    }

    /// Step back through the draft's edits.
    pub fn draft_undo(&self, account: AccountId) -> Result<(), String> {
        self.step(account, true)
    }

    /// Step forward again after an undo.
    pub fn draft_redo(&self, account: AccountId) -> Result<(), String> {
        self.step(account, false)
    }

    fn step(&self, account: AccountId, back: bool) -> Result<(), String> {
        {
            let mut drafts = self.inner.drafts.lock();
            let d = drafts.entry(account).or_default();
            let (from, to) = if back {
                (&mut d.undo, &mut d.redo)
            } else {
                (&mut d.redo, &mut d.undo)
            };
            let prev = from.pop().ok_or(if back {
                "Nothing to undo."
            } else {
                "Nothing to redo."
            })?;
            to.push(std::mem::replace(&mut d.now, prev));
            d.pending = None;
        }
        self.inner.repaint();
        Ok(())
    }

    /// Drop the draft's edits and show what is worn. Undo brings them back.
    pub fn revert_draft(&self, account: AccountId) -> Result<(), String> {
        let worn = self.avatar(account).data;
        {
            let mut drafts = self.inner.drafts.lock();
            let d = drafts.entry(account).or_default();
            if d.now.look.is_none() {
                return Err("There are no changes to revert.".into());
            }
            // Keep the outfit only when what is worn is that outfit.
            let keep = matches!((&d.now.saved, &worn), (Some(s), Some(w)) if same(s, w));
            let next = Step {
                look: None,
                outfit: d.now.outfit.clone().filter(|_| keep),
                saved: d.now.saved.clone().filter(|_| keep),
            };
            push(d, next);
            d.pending = None;
        }
        self.inner.repaint();
        Ok(())
    }

    /// Wear the draft: body type, colors and scales where they differ, then the items.
    pub fn wear_draft(&self, account: AccountId) -> Result<(), String> {
        let (look, outfit) = {
            let drafts = self.inner.drafts.lock();
            let d = drafts.get(&account).ok_or(NOTHING)?;
            let look = d.now.look.clone().ok_or(NOTHING)?;
            let outfit = d
                .now
                .outfit
                .clone()
                .filter(|_| d.now.saved.as_ref().is_some_and(|s| same(s, &look)));
            (look, outfit)
        };
        let colors = body_colors(&look, self.avatar_rules().data.as_deref());
        self.inner.mutate(
            account,
            "wear the look",
            "",
            &[Effect::Avatar],
            move |f| async move {
                let now = avatar::v2::get_avatar_users_by_user_id_avatar(&f.client, account.0 as i64).await?;
                let rig = match now.player_avatar_type {
                    Some(avatar::v2::AvatarModelV3PlayerAvatarType::R6) => "R6",
                    Some(avatar::v2::AvatarModelV3PlayerAvatarType::R15) => "R15",
                    _ => "",
                };
                if let Some(want) = Rig::parse(&look.rig)
                    && look.rig != rig
                {
                    set_rig(&f.client, want).await?;
                }
                let have = exact_colors(now.body_color3s.unwrap_or_default());
                let have = Part::ALL.map(|p| {
                    have.get(p.key())
                        .map(|h| h.trim_start_matches('#').to_ascii_uppercase())
                });
                let want = color_list(&colors);
                if want.iter().zip(&have).any(|(w, h)| w.is_some() && w != h) {
                    let r = avatar::v2::post_avatar_set_body_colors(&f.client, &colors).await?;
                    if r.success == Some(false) {
                        return Err(Error::Message("Roblox would not apply those body colors.".into()));
                    }
                }
                let had = now.scales.map(|s| scales(convert(Some(s)).unwrap_or_default()));
                if !had.is_some_and(|s| same_scales(&s, &look.scales)) {
                    avatar::v1::post_avatar_set_scales(&f.client, &scales_model(look.scales)).await?;
                }
                set_wearing(&f.client, wear_models(&look.assets)).await?;
                forget_images(&f.inner, account.0);
                if let Some(o) = outfit {
                    note_worn(
                        &f.inner,
                        account,
                        Recent {
                            id: o.id,
                            name: o.name,
                            outfit: true,
                            ..Default::default()
                        },
                    );
                }
                Ok(())
            },
        );
        Ok(())
    }

    /// Save the draft (or what is worn) as a new outfit, which the draft then edits.
    #[allow(deprecated)]
    pub fn save_outfit(&self, account: AccountId, name: &str) -> Result<(), String> {
        let name = name.trim().to_string();
        if name.is_empty() {
            return Err("Give the outfit a name.".into());
        }
        let draft = self.saving(account);
        self.inner.mutate(
            account,
            "save the outfit",
            "Outfit saved.",
            &[Effect::Outfits],
            move |f| async move {
                let (body, look) = match draft {
                    Some((l, c)) => (outfit_body(&l, Some(name.clone()), &c), l),
                    None => {
                        let now = avatar::v2::get_avatar_users_by_user_id_avatar(&f.client, account.0 as i64)
                            .await?;
                        let look = worn_now(&now);
                        (super::current_look(now, name.clone()), look)
                    }
                };
                let made = avatar::v3::post_outfits_create(&f.client, &Default::default(), &body).await?;
                if let Some(id) = made.id.and_then(|i| u64::try_from(i).ok()) {
                    saved_as(&f.inner, account, DraftOutfit { id, name }, look);
                }
                Ok(())
            },
        );
        Ok(())
    }

    /// Replace what `outfit` holds with the draft (or what is worn). Keeps its name.
    #[allow(deprecated)]
    pub fn update_outfit(&self, account: AccountId, outfit: u64) {
        let draft = self.saving(account);
        self.inner.mutate(
            account,
            "update the outfit",
            "Outfit updated.",
            &[Effect::Outfits],
            move |f| async move {
                let o = super::rgb_outfit(&f.client, outfit).await?;
                let name = o.name.unwrap_or_default();
                let (body, look) = match draft {
                    Some((l, c)) => (outfit_body(&l, Some(name.clone()), &c), l),
                    None => {
                        let now = avatar::v2::get_avatar_users_by_user_id_avatar(&f.client, account.0 as i64)
                            .await?;
                        let look = worn_now(&now);
                        (super::current_look(now, name.clone()), look)
                    }
                };
                avatar::v3::patch_outfits_by_user_outfit_id(
                    &f.client,
                    outfit as i64,
                    &Default::default(),
                    &body,
                )
                .await?;
                forget_outfit_image(&f.inner, outfit);
                f.inner.res.invalidate_key(&Key::shared("outfit", outfit));
                saved_as(&f.inner, account, DraftOutfit { id: outfit, name }, look);
                Ok(())
            },
        );
    }

    /// The edited look and its colors, when the draft has edits. Read without fetching, so
    /// a save sends what the editor shows.
    fn saving(&self, account: AccountId) -> Option<(Look, avatar::v2::BodyColorsModelV2)> {
        let look = self.inner.drafts.lock().get(&account)?.now.look.clone()?;
        let colors = body_colors(&look, self.avatar_rules().data.as_deref());
        Some((look, colors))
    }
}

/// What `now` wears, as a look to compare drafts with.
fn worn_now(now: &avatar::v2::AvatarModelV3) -> Look {
    let mut l = super::look(convert(Some(now.clone())).unwrap_or_default());
    l.color_hex = exact_colors(now.body_color3s.clone().unwrap_or_default());
    l
}

/// After a save: the draft now edits that outfit, which holds `look`.
fn saved_as(inner: &crate::core::Inner, account: AccountId, outfit: DraftOutfit, look: Look) {
    if let Some(d) = inner.drafts.lock().get_mut(&account) {
        d.now.outfit = Some(outfit);
        d.now.saved = Some(look);
    }
    inner.repaint();
}

fn push(d: &mut Draft, next: Step) {
    d.undo.push(std::mem::replace(&mut d.now, next));
    if d.undo.len() > HISTORY {
        d.undo.remove(0);
    }
    d.redo.clear();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::avatar::Placement;

    fn item(id: u64, type_id: i64, layer: Option<i32>) -> Worn {
        Worn {
            id,
            type_id,
            layer,
            ..Default::default()
        }
    }

    fn ids(v: &[Worn]) -> Vec<u64> {
        v.iter().map(|a| a.id).collect()
    }

    #[test]
    fn toggles_like_the_website() {
        let now = vec![item(1, 8, None), item(2, 8, None), item(3, 11, None)];
        let mut v = now.clone();
        toggle(&mut v, item(2, 8, None), Some(3));
        assert_eq!(ids(&v), [1, 3], "off");
        let mut v = now.clone();
        toggle(&mut v, item(4, 8, None), Some(2));
        assert_eq!(ids(&v), [2, 3, 4], "a full slot drops its oldest");
        let mut v = now.clone();
        toggle(&mut v, item(5, 99, None), None);
        assert_eq!(ids(&v), [1, 2, 3, 5]);
    }

    #[test]
    fn new_layered_clothing_goes_on_top() {
        let mut v = vec![item(1, 65, Some(3)), item(2, 66, Some(7))];
        toggle(&mut v, item(3, 64, None), Some(10));
        assert_eq!(v[2].layer, Some(8));
        let mut v = Vec::new();
        toggle(&mut v, item(3, 64, None), Some(10));
        assert_eq!(v[0].layer, Some(1));
    }

    #[test]
    fn relayers_by_swapping_neighbours() {
        let mut v = vec![item(1, 8, None), item(2, 65, Some(1)), item(3, 66, Some(2))];
        relayer(&mut v, 2, true).unwrap();
        assert_eq!((v[1].layer, v[2].layer), (Some(2), Some(1)));
        assert!(relayer(&mut v, 2, true).is_err(), "already on top");
        assert!(relayer(&mut v, 1, true).is_err(), "not layered");
        let mut tied = vec![item(1, 65, Some(4)), item(2, 66, Some(4))];
        relayer(&mut tied, 1, true).unwrap();
        assert!(tied[0].layer > tied[1].layer);
        // A tie next to a third item: each move still changes the order, with no new tie.
        let mut three = vec![item(1, 65, Some(5)), item(2, 66, Some(5)), item(3, 67, Some(6))];
        relayer(&mut three, 1, true).unwrap();
        relayer(&mut three, 1, true).unwrap();
        let mut orders: Vec<_> = three.iter().map(|a| a.layer.unwrap()).collect();
        assert_eq!(three[0].layer, Some(3), "moved to the top");
        orders.sort();
        orders.dedup();
        assert_eq!(orders.len(), 3, "no two items share an order");
    }

    #[test]
    fn sameness_ignores_names_order_and_default_placements() {
        let mut a = Look {
            rig: "R15".into(),
            assets: vec![item(1, 8, None), item(2, 65, Some(1)), item(3, 66, Some(2))],
            ..Default::default()
        };
        let mut b = a.clone();
        b.assets.reverse();
        b.assets[0].name = "Hat".into();
        b.rig = String::new();
        assert!(same(&a, &b));
        b.assets[1].placement = Some(Placement::default());
        assert!(same(&a, &b), "a default placement is no placement");
        b.assets[1].layer = Some(5);
        assert!(!same(&a, &b), "layer order differs");
        b.assets[1].layer = Some(1);
        a.scales.height = 1.05;
        assert!(!same(&a, &b));
        a.scales.height = b.scales.height + 1e-5;
        assert!(same(&a, &b), "scales within rounding");
    }

    #[test]
    fn colors_compare_by_hex_when_known() {
        let mut a = Look::default();
        let mut b = Look::default();
        paint(&mut a, &[Part::Head], Some(1), "#F5CD30");
        assert!(!same(&a, &b));
        paint(&mut b, &[Part::Head], None, "#f5cd30");
        assert!(same(&a, &b), "ids differ, but the exact colors match");
        paint(&mut b, &[], None, "#000000");
        assert_eq!(b.color_hex.len(), 6, "no parts paints all of them");
    }

    #[test]
    fn writes_layer_and_placement_into_meta() {
        let mut hat = item(1, 8, None);
        hat.placement = Some(Placement {
            position: [0.0, 0.1, 0.0],
            ..Default::default()
        });
        let mut jacket = item(2, 65, Some(3));
        jacket.meta = Some(avatar::v1::AssetMetaModelV1 {
            puffiness: Some(0.5),
            order: Some(1),
            ..Default::default()
        });
        let v = serde_json::to_value(wear_models(&[hat, jacket, item(3, 11, None)])).unwrap();
        assert_eq!(v[0]["meta"]["position"]["Y"], 0.1);
        assert_eq!(v[1]["meta"]["order"], 3);
        assert_eq!(v[1]["meta"]["puffiness"], 0.5);
        assert!(v[2].get("meta").is_none_or(|m| m.is_null()));
    }
}

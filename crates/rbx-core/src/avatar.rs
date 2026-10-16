//! The signed-in user's avatar: what they wear, their body type, and their outfits. Wearing
//! an item or an outfit, switching R6/R15 and saving the current look as an outfit.
//!
//! Writes read the current avatar first, so a change made on the website in the meantime is
//! kept.

use rbx_api::generated::avatar;
use rbx_api::{AccountId, Client, Error};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::Core;
use crate::core::Inner;
use crate::res::{Key, Page, Res};
use crate::thumbs::ThumbKind;

const AGE: i64 = 120;
const RULES_AGE: i64 = 6 * 3_600;

/// An asset type an avatar can wear, and how many at once.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WearSlot {
    pub type_id: i64,
    pub name: String,
    /// 0 for types worn another way (emotes).
    pub max: i32,
}

/// One worn asset.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Worn {
    pub id: u64,
    pub name: String,
    pub type_id: i64,
    pub type_name: String,
    /// Layer of layered clothing; higher sits on top.
    pub layer: Option<i32>,
}

/// An equipped emote.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Emote {
    pub id: u64,
    pub name: String,
    pub slot: i32,
}

/// BrickColor ids per body part.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct BodyColors {
    pub head: i32,
    pub torso: i32,
    pub left_arm: i32,
    pub right_arm: i32,
    pub left_leg: i32,
    pub right_leg: i32,
}

/// A body part that takes its own color.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part {
    Head,
    Torso,
    LeftArm,
    RightArm,
    LeftLeg,
    RightLeg,
}

impl Part {
    pub const ALL: [Part; 6] = [
        Part::Head,
        Part::Torso,
        Part::LeftArm,
        Part::RightArm,
        Part::LeftLeg,
        Part::RightLeg,
    ];

    pub fn parse(s: &str) -> Option<Part> {
        Some(
            match s.trim().to_ascii_lowercase().replace([' ', '-'], "_").as_str() {
                "head" => Part::Head,
                "torso" => Part::Torso,
                "left_arm" => Part::LeftArm,
                "right_arm" => Part::RightArm,
                "left_leg" => Part::LeftLeg,
                "right_leg" => Part::RightLeg,
                _ => return None,
            },
        )
    }

    pub fn key(self) -> &'static str {
        match self {
            Part::Head => "head",
            Part::Torso => "torso",
            Part::LeftArm => "left_arm",
            Part::RightArm => "right_arm",
            Part::LeftLeg => "left_leg",
            Part::RightLeg => "right_leg",
        }
    }
}

impl BodyColors {
    pub fn get(&self, p: Part) -> i32 {
        match p {
            Part::Head => self.head,
            Part::Torso => self.torso,
            Part::LeftArm => self.left_arm,
            Part::RightArm => self.right_arm,
            Part::LeftLeg => self.left_leg,
            Part::RightLeg => self.right_leg,
        }
    }

    fn set(&mut self, p: Part, color: i32) {
        *match p {
            Part::Head => &mut self.head,
            Part::Torso => &mut self.torso,
            Part::LeftArm => &mut self.left_arm,
            Part::RightArm => &mut self.right_arm,
            Part::LeftLeg => &mut self.left_leg,
            Part::RightLeg => &mut self.right_leg,
        } = color;
    }
}

/// Body scales. Height, width, head and depth are factors around 1; proportion and body type
/// run from 0 to 1.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Scales {
    pub height: f64,
    pub width: f64,
    pub head: f64,
    pub depth: f64,
    pub proportion: f64,
    pub body_type: f64,
}

/// A scale the editor can change.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scale {
    Height,
    Width,
    Head,
    Proportion,
    BodyType,
}

impl Scale {
    pub const ALL: [Scale; 5] = [
        Scale::Height,
        Scale::Width,
        Scale::Head,
        Scale::Proportion,
        Scale::BodyType,
    ];

    /// The name the avatar rules use.
    pub fn key(self) -> &'static str {
        match self {
            Scale::Height => "height",
            Scale::Width => "width",
            Scale::Head => "head",
            Scale::Proportion => "proportion",
            Scale::BodyType => "bodyType",
        }
    }

    pub fn parse(s: &str) -> Option<Scale> {
        let s = s.trim().to_ascii_lowercase().replace([' ', '_', '-'], "");
        Scale::ALL.into_iter().find(|x| x.key().to_ascii_lowercase() == s)
    }

    pub fn get(self, s: &Scales) -> f64 {
        match self {
            Scale::Height => s.height,
            Scale::Width => s.width,
            Scale::Head => s.head,
            Scale::Proportion => s.proportion,
            Scale::BodyType => s.body_type,
        }
    }

    fn set(self, s: &mut Scales, v: f64) {
        *match self {
            Scale::Height => &mut s.height,
            Scale::Width => &mut s.width,
            Scale::Head => &mut s.head,
            Scale::Proportion => &mut s.proportion,
            Scale::BodyType => &mut s.body_type,
        } = v;
    }
}

/// What the avatar looks like now.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Look {
    /// "R6" or "R15".
    pub rig: String,
    pub assets: Vec<Worn>,
    pub emotes: Vec<Emote>,
    pub colors: BodyColors,
    pub scales: Scales,
}

/// A body color the editor offers.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Swatch {
    /// BrickColor id.
    pub id: i32,
    pub name: String,
    /// `#RRGGBB`.
    pub hex: String,
    /// In the short palette the website shows first.
    pub basic: bool,
}

/// The range a scale may take.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ScaleRule {
    /// As in [`Scale::key`].
    pub name: String,
    pub min: f64,
    pub max: f64,
    pub step: f64,
}

/// What an avatar may wear and how it may look. Public.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Rules {
    pub slots: Vec<WearSlot>,
    pub palette: Vec<Swatch>,
    pub scales: Vec<ScaleRule>,
    /// Proportion and body type can be changed.
    pub proportions: bool,
}

impl Rules {
    pub fn scale(&self, s: Scale) -> Option<&ScaleRule> {
        self.scales.iter().find(|r| r.name == s.key())
    }
}

/// Something worn or used recently.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Recent {
    pub id: u64,
    pub name: String,
    /// False for an asset.
    pub outfit: bool,
    pub type_id: i64,
    pub type_name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Outfit {
    pub id: u64,
    pub name: String,
    /// False for outfits that came with a bundle.
    pub editable: bool,
}

/// Body type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rig {
    R6,
    R15,
}

impl Rig {
    pub fn parse(s: &str) -> Option<Rig> {
        match s.trim().to_ascii_uppercase().as_str() {
            "R6" => Some(Rig::R6),
            "R15" => Some(Rig::R15),
            _ => None,
        }
    }
}

fn rig_name(t: Option<avatar::v1::AvatarModelV2PlayerAvatarType>) -> String {
    match t {
        Some(avatar::v1::AvatarModelV2PlayerAvatarType::R6) => "R6".into(),
        Some(avatar::v1::AvatarModelV2PlayerAvatarType::R15) => "R15".into(),
        _ => String::new(),
    }
}

fn look(a: avatar::v1::AvatarModelV2) -> Look {
    Look {
        rig: rig_name(a.player_avatar_type),
        assets: worn(a.assets),
        emotes: a
            .emotes
            .unwrap_or_default()
            .into_iter()
            .filter_map(|e| {
                Some(Emote {
                    id: u64::try_from(e.asset_id?).ok()?,
                    name: e.asset_name.unwrap_or_default(),
                    slot: e.position.unwrap_or(0),
                })
            })
            .collect(),
        colors: a.body_colors.map(colors).unwrap_or_default(),
        scales: a.scales.map(scales).unwrap_or_default(),
    }
}

fn worn(assets: Option<Vec<avatar::v1::AssetModelV2>>) -> Vec<Worn> {
    assets
        .unwrap_or_default()
        .into_iter()
        .filter_map(|x| {
            let t = x.asset_type.unwrap_or_default();
            Some(Worn {
                id: u64::try_from(x.id?).ok()?,
                name: x.name.unwrap_or_default(),
                type_id: t.id.unwrap_or(0),
                type_name: t.name.unwrap_or_default(),
                layer: x.meta.as_ref().and_then(|m| m.order),
            })
        })
        .collect()
}

/// What an outfit puts on. Outfits have no emotes.
fn outfit_look(o: avatar::v1::OutfitDetailsModel) -> Look {
    Look {
        rig: match o.player_avatar_type.as_deref().and_then(Rig::parse) {
            Some(Rig::R6) => "R6".into(),
            Some(Rig::R15) => "R15".into(),
            None => String::new(),
        },
        assets: worn(o.assets),
        emotes: Vec::new(),
        colors: o.body_colors.map(colors).unwrap_or_default(),
        scales: o.scale.map(scales).unwrap_or_default(),
    }
}

fn colors(c: avatar::v1::BodyColorsModel) -> BodyColors {
    BodyColors {
        head: c.head_color_id.unwrap_or(0),
        torso: c.torso_color_id.unwrap_or(0),
        left_arm: c.left_arm_color_id.unwrap_or(0),
        right_arm: c.right_arm_color_id.unwrap_or(0),
        left_leg: c.left_leg_color_id.unwrap_or(0),
        right_leg: c.right_leg_color_id.unwrap_or(0),
    }
}

fn colors_model(c: BodyColors) -> avatar::v1::BodyColorsModel {
    avatar::v1::BodyColorsModel {
        head_color_id: Some(c.head),
        torso_color_id: Some(c.torso),
        left_arm_color_id: Some(c.left_arm),
        right_arm_color_id: Some(c.right_arm),
        left_leg_color_id: Some(c.left_leg),
        right_leg_color_id: Some(c.right_leg),
    }
}

fn scales(s: avatar::v1::ScaleModel) -> Scales {
    Scales {
        height: s.height.unwrap_or(1.0),
        width: s.width.unwrap_or(1.0),
        head: s.head.unwrap_or(1.0),
        depth: s.depth.unwrap_or(1.0),
        proportion: s.proportion.unwrap_or(0.0),
        body_type: s.body_type.unwrap_or(0.0),
    }
}

fn scales_model(s: Scales) -> avatar::v1::ScaleModel {
    avatar::v1::ScaleModel {
        height: Some(s.height),
        width: Some(s.width),
        head: Some(s.head),
        depth: Some(s.depth),
        proportion: Some(s.proportion),
        body_type: Some(s.body_type),
    }
}

fn rules(r: avatar::v1::AvatarRulesModel) -> Rules {
    let basic: Vec<i32> = r
        .basic_body_colors_palette
        .unwrap_or_default()
        .into_iter()
        .filter_map(|c| c.brick_color_id)
        .collect();
    let mut scales: Vec<ScaleRule> = r
        .scales
        .unwrap_or_default()
        .into_iter()
        .filter_map(|(name, x)| {
            Some(ScaleRule {
                name,
                min: x.min?,
                max: x.max?,
                step: x.increment.filter(|i| *i > 0.0).unwrap_or(0.01),
            })
        })
        .collect();
    scales.sort_by_key(|r| {
        Scale::ALL
            .iter()
            .position(|s| s.key() == r.name)
            .unwrap_or(usize::MAX)
    });
    Rules {
        slots: r
            .wearable_asset_types
            .unwrap_or_default()
            .into_iter()
            .filter_map(|t| {
                Some(WearSlot {
                    type_id: t.id?,
                    name: t.name.unwrap_or_default(),
                    max: t.max_number.unwrap_or(1),
                })
            })
            .collect(),
        palette: r
            .body_colors_palette
            .unwrap_or_default()
            .into_iter()
            .filter_map(|c| {
                let id = c.brick_color_id?;
                Some(Swatch {
                    id,
                    name: c.name.unwrap_or_default(),
                    hex: c.hex_color.unwrap_or_default(),
                    basic: basic.contains(&id),
                })
            })
            .collect(),
        scales,
        proportions: r.proportions_and_body_type_enabled_for_user.unwrap_or(true),
    }
}

/// `v` moved onto the rule's range and steps.
fn snap(v: f64, r: &ScaleRule) -> f64 {
    let v = v.clamp(r.min, r.max);
    let steps = ((v - r.min) / r.step).round();
    let v = (r.min + steps * r.step).min(r.max);
    (v * 10_000.0).round() / 10_000.0
}

/// `worn` with `asset` taken off, or put on when it isn't worn. Putting on an item of a type
/// that is full (one shirt, three hats) takes off the oldest of that type.
fn toggled(
    mut worn: Vec<avatar::v1::AssetModelV2>,
    asset: u64,
    type_id: Option<i64>,
    max: Option<i32>,
) -> Vec<avatar::v1::AssetModelV2> {
    let before = worn.len();
    worn.retain(|a| a.id != Some(asset as i64));
    if worn.len() < before {
        return worn;
    }
    if let (Some(t), Some(max)) = (type_id, max) {
        let same = |a: &avatar::v1::AssetModelV2| a.asset_type.as_ref().and_then(|x| x.id) == Some(t);
        let n = worn.iter().filter(|a| same(a)).count();
        let extra = (n + 1).saturating_sub(max.max(1) as usize);
        let mut dropped = 0;
        worn.retain(|a| {
            if dropped < extra && same(a) {
                dropped += 1;
                false
            } else {
                true
            }
        });
    }
    worn.push(avatar::v1::AssetModelV2 {
        id: Some(asset as i64),
        asset_type: type_id.map(|t| avatar::v1::AssetTypeModel {
            id: Some(t),
            name: None,
        }),
        ..Default::default()
    });
    worn
}

/// `worn` with `asset` moved one layer out (on top) or in, by swapping layer orders with its
/// neighbour among layered items.
fn relayered(
    mut worn: Vec<avatar::v1::AssetModelV2>,
    asset: u64,
    outward: bool,
) -> Result<Vec<avatar::v1::AssetModelV2>, String> {
    let order = |a: &avatar::v1::AssetModelV2| a.meta.as_ref().and_then(|m| m.order);
    let mut layers: Vec<(i32, usize)> = worn
        .iter()
        .enumerate()
        .filter_map(|(i, a)| Some((order(a)?, i)))
        .collect();
    layers.sort();
    let at = layers
        .iter()
        .position(|&(_, i)| worn[i].id == Some(asset as i64))
        .ok_or("That item has no layer to move.")?;
    let other = if outward {
        at + 1
    } else {
        at.checked_sub(1).ok_or("It is already the innermost layer.")?
    };
    let &(theirs, j) = layers.get(other).ok_or("It is already the outermost layer.")?;
    let (mine, i) = layers[at];
    // Equal orders would not move anything; step past the neighbour instead.
    let (mine, theirs) = if mine == theirs {
        if outward {
            (theirs + 1, theirs)
        } else {
            (theirs - 1, theirs)
        }
    } else {
        (theirs, mine)
    };
    if let Some(m) = worn[i].meta.as_mut() {
        m.order = Some(mine);
    }
    if let Some(m) = worn[j].meta.as_mut() {
        m.order = Some(theirs);
    }
    Ok(worn)
}

/// Worn items of this asset type only show on R15: body parts, animations, layered clothing,
/// dynamic heads and their face parts, makeup.
pub fn r15_only(type_id: i64) -> bool {
    matches!(type_id, 27..=31 | 48..=56 | 64..=72 | 76..=79 | 88..=90)
}

/// API versions share the wire format of metadata, colors and scales; carry it across
/// unchanged.
fn convert<A: Serialize, B: DeserializeOwned>(a: Option<A>) -> Option<B> {
    a.and_then(|a| serde_json::to_value(a).ok())
        .and_then(|v| serde_json::from_value(v).ok())
}

fn wear_list(assets: Vec<avatar::v1::AssetModelV2>) -> Vec<avatar::v2::AssetWearModel> {
    assets
        .into_iter()
        .filter(|a| a.id.is_some())
        .map(|a| avatar::v2::AssetWearModel {
            id: a.id,
            meta: convert(a.meta),
        })
        .collect()
}

async fn set_wearing(client: &Client, assets: Vec<avatar::v2::AssetWearModel>) -> Result<(), Error> {
    let r = avatar::v2::post_avatar_set_wearing_assets(
        client,
        &avatar::v2::WearRequestModel { assets: Some(assets) },
    )
    .await?;
    let bad: Vec<String> = r
        .invalid_assets
        .unwrap_or_default()
        .into_iter()
        .filter_map(|a| a.name)
        .collect();
    if !bad.is_empty() {
        return Err(Error::Message(format!(
            "Roblox would not wear {}.",
            bad.join(", ")
        )));
    }
    if r.success == Some(false) || !r.invalid_asset_ids.unwrap_or_default().is_empty() {
        return Err(Error::Message("Roblox would not wear some of it.".into()));
    }
    Ok(())
}

async fn set_rig(client: &Client, rig: Rig) -> Result<(), Error> {
    avatar::v1::post_avatar_set_player_avatar_type(
        client,
        &avatar::v1::PlayerAvatarTypeModel {
            player_avatar_type: Some(match rig {
                Rig::R6 => avatar::v1::AvatarModelV2PlayerAvatarType::R6,
                Rig::R15 => avatar::v1::AvatarModelV2PlayerAvatarType::R15,
            }),
        },
    )
    .await?;
    Ok(())
}

/// The current look as outfit contents, named `name`.
fn current_look(now: avatar::v1::AvatarModelV2, name: String) -> avatar::v2::OutfitUpdateModelV2 {
    avatar::v2::OutfitUpdateModelV2 {
        assets: Some(wear_list(now.assets.unwrap_or_default())),
        body_colors: convert(now.body_colors),
        name: Some(name),
        outfit_type: Some(avatar::v2::OutfitUpdateModelV2OutfitType::Avatar),
        player_avatar_type: Some(rig_name(now.player_avatar_type)).filter(|r| !r.is_empty()),
        scale: convert(now.scales),
    }
}

/// The avatar images are rendered again after a change; drop ours so they reload.
fn forget_images(inner: &Inner, user: u64) {
    inner.forget_thumbs(&[ThumbKind::Avatar, ThumbKind::Headshot], user);
}

fn forget_outfit_image(inner: &Inner, outfit: u64) {
    inner.forget_thumbs(&[ThumbKind::Outfit], outfit);
}

impl Core {
    /// What `account` wears now.
    pub fn avatar(&self, account: AccountId) -> Res<Look> {
        if account.is_anonymous() {
            return Res::default();
        }
        self.inner
            .res(Key::private(account, "avatar", ""), AGE, |f| async move {
                Ok(look(avatar::v1::get_avatar(&f.client).await?))
            })
    }

    /// What an avatar may wear, the body colors and the scale ranges. Public.
    pub fn avatar_rules(&self) -> Res<Rules> {
        self.inner
            .res(Key::shared("avatar_rules", ""), RULES_AGE, |f| async move {
                Ok(rules(avatar::v1::get_avatar_rules(&f.client).await?))
            })
    }

    /// Asset types an avatar can wear. Public.
    pub fn wear_slots(&self) -> Option<Vec<WearSlot>> {
        self.avatar_rules().data.map(|r| r.slots.clone())
    }

    /// What `account` wore or used lately: items and outfits, newest first.
    pub fn recent_items(&self, account: AccountId) -> Res<Vec<Recent>> {
        if account.is_anonymous() {
            return Res::default();
        }
        self.inner
            .res(Key::private(account, "avatar_recent", ""), AGE, |f| async move {
                // List type 0 is every kind of item.
                let r = avatar::v1::get_recent_items_by_recent_item_list_type_list(&f.client, 0).await?;
                Ok(r.data
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|x| {
                        let t = x.asset_type.unwrap_or_default();
                        Some(Recent {
                            id: u64::try_from(x.id?).ok()?,
                            name: x.name.unwrap_or_default(),
                            outfit: x.type_ == Some(2) || x.is_editable.is_some(),
                            type_id: t.id.unwrap_or(0),
                            type_name: t.name.unwrap_or_default(),
                        })
                    })
                    .collect())
            })
    }

    /// `account`'s outfits, newest first, a page at a time.
    pub fn outfits(&self, account: AccountId, more: bool) -> Res<Page<Outfit>> {
        if account.is_anonymous() {
            return Res::default();
        }
        self.inner.paged(
            Key::private(account, "outfits", ""),
            AGE,
            more,
            move |f, cursor| async move {
                let r = avatar::v2::get_avatar_users_by_user_id_outfits(
                    &f.client,
                    account.0 as i64,
                    &avatar::v2::GetAvatarUsersByUserIdOutfitsParams {
                        pagination_token: cursor,
                        ..Default::default()
                    },
                )
                .await?;
                Ok(Page {
                    items: r
                        .data
                        .unwrap_or_default()
                        .into_iter()
                        .filter_map(|o| {
                            Some(Outfit {
                                id: u64::try_from(o.id?).ok()?,
                                name: o.name.unwrap_or_default(),
                                editable: o.is_editable.unwrap_or(false),
                            })
                        })
                        .collect(),
                    next: r.pagination_token.filter(|t| !t.is_empty()),
                })
            },
        )
    }

    /// What anyone can see `user_id` wearing. Public.
    pub fn user_look(&self, user_id: u64) -> Res<Look> {
        self.inner
            .res(Key::shared("user_look", user_id), AGE, move |f| async move {
                Ok(look(
                    avatar::v1::get_users_by_user_id_avatar(&f.client, user_id as i64).await?,
                ))
            })
    }

    /// Outfits `user_id` made, newest first, a page at a time. Bundle outfits are left out:
    /// they are what came with a bundle, not something the user put together.
    pub fn user_outfits(&self, user_id: u64, more: bool) -> Res<Page<Outfit>> {
        self.inner.paged(
            Key::shared("user_outfits", user_id),
            AGE,
            more,
            move |f, cursor| async move {
                let r = avatar::v2::get_avatar_users_by_user_id_outfits(
                    &f.client,
                    user_id as i64,
                    &avatar::v2::GetAvatarUsersByUserIdOutfitsParams {
                        pagination_token: cursor,
                        outfit_type: Some("Avatar".into()),
                        is_editable: Some(true),
                        ..Default::default()
                    },
                )
                .await?;
                Ok(Page {
                    items: r
                        .data
                        .unwrap_or_default()
                        .into_iter()
                        .filter_map(|o| {
                            Some(Outfit {
                                id: u64::try_from(o.id?).ok()?,
                                name: o.name.unwrap_or_default(),
                                editable: o.is_editable.unwrap_or(false),
                            })
                        })
                        .collect(),
                    next: r.pagination_token.filter(|t| !t.is_empty()),
                })
            },
        )
    }

    /// What an outfit is made of. Public.
    pub fn outfit(&self, outfit: u64) -> Res<Look> {
        self.inner
            .res(Key::shared("outfit", outfit), AGE, move |f| async move {
                Ok(outfit_look(
                    avatar::v1::get_outfits_by_user_outfit_id_details(&f.client, outfit as i64).await?,
                ))
            })
    }

    /// Put `asset` on, or take it off when it is already worn. With its asset type known, a
    /// full slot drops its oldest item, as on the website.
    pub fn toggle_wear(&self, account: AccountId, asset: u64, type_id: Option<i64>) {
        let max = type_id.and_then(|t| self.wear_slots()?.iter().find(|s| s.type_id == t).map(|s| s.max));
        self.inner.mutate(
            account,
            "change your avatar",
            "",
            &["avatar"],
            move |f| async move {
                let now = avatar::v1::get_avatar(&f.client).await?;
                let assets = toggled(now.assets.unwrap_or_default(), asset, type_id, max);
                set_wearing(&f.client, wear_list(assets)).await?;
                forget_images(&f.inner, account.0);
                Ok(())
            },
        );
    }

    /// Wear one of `account`'s outfits: its items, body type, colors and scales.
    pub fn wear_outfit(&self, account: AccountId, outfit: u64) {
        self.inner.mutate(
            account,
            "wear the outfit",
            "Outfit on.",
            &["avatar"],
            move |f| async move {
                let o = avatar::v1::get_outfits_by_user_outfit_id_details(&f.client, outfit as i64).await?;
                set_wearing(&f.client, wear_list(o.assets.unwrap_or_default())).await?;
                if let Some(rig) = o.player_avatar_type.as_deref().and_then(Rig::parse) {
                    set_rig(&f.client, rig).await?;
                }
                if let Some(c) = o.body_colors {
                    avatar::v1::post_avatar_set_body_colors(&f.client, &c).await?;
                }
                if let Some(s) = o.scale {
                    avatar::v1::post_avatar_set_scales(&f.client, &s).await?;
                }
                forget_images(&f.inner, account.0);
                Ok(())
            },
        );
    }

    /// Save what `account` wears now as a new outfit.
    pub fn save_outfit(&self, account: AccountId, name: &str) -> Result<(), String> {
        let name = name.trim().to_string();
        if name.is_empty() {
            return Err("Give the outfit a name.".into());
        }
        self.inner.mutate(
            account,
            "save the outfit",
            "Outfit saved.",
            &["outfits"],
            move |f| async move {
                let now = avatar::v1::get_avatar(&f.client).await?;
                avatar::v2::post_outfits_create(&f.client, &current_look(now, name)).await?;
                Ok(())
            },
        );
        Ok(())
    }

    /// Replace what `outfit` holds with what `account` wears now. Keeps its name.
    pub fn update_outfit(&self, account: AccountId, outfit: u64) {
        self.inner.mutate(
            account,
            "update the outfit",
            "Outfit updated.",
            &["outfits"],
            move |f| async move {
                let o = avatar::v1::get_outfits_by_user_outfit_id_details(&f.client, outfit as i64).await?;
                let now = avatar::v1::get_avatar(&f.client).await?;
                let body = current_look(now, o.name.unwrap_or_default());
                avatar::v2::post_outfits_by_user_outfit_id_update(&f.client, outfit as i64, &body).await?;
                forget_outfit_image(&f.inner, outfit);
                Ok(())
            },
        );
    }

    /// Rename `outfit`. Its contents stay as they are.
    pub fn rename_outfit(&self, account: AccountId, outfit: u64, name: &str) -> Result<(), String> {
        let name = name.trim().to_string();
        if name.is_empty() {
            return Err("Give the outfit a name.".into());
        }
        self.inner.mutate(
            account,
            "rename the outfit",
            "Outfit renamed.",
            &["outfits"],
            move |f| async move {
                let o = avatar::v1::get_outfits_by_user_outfit_id_details(&f.client, outfit as i64).await?;
                let body = avatar::v2::OutfitUpdateModelV2 {
                    assets: Some(wear_list(o.assets.unwrap_or_default())),
                    body_colors: convert(o.body_colors),
                    name: Some(name),
                    outfit_type: Some(avatar::v2::OutfitUpdateModelV2OutfitType::Avatar),
                    player_avatar_type: o.player_avatar_type.filter(|r| !r.is_empty()),
                    scale: convert(o.scale),
                };
                avatar::v2::post_outfits_by_user_outfit_id_update(&f.client, outfit as i64, &body).await?;
                Ok(())
            },
        );
        Ok(())
    }

    /// Delete one of `account`'s outfits. Outfits from bundles can't be deleted.
    pub fn delete_outfit(&self, account: AccountId, outfit: u64) {
        self.inner.mutate(
            account,
            "delete the outfit",
            "Outfit deleted.",
            &["outfits"],
            move |f| async move {
                avatar::v1::post_outfits_by_user_outfit_id_delete(&f.client, outfit as i64).await?;
                Ok(())
            },
        );
    }

    /// Move a worn layered item one layer out (on top) or in.
    pub fn move_layer(&self, account: AccountId, asset: u64, outward: bool) -> Result<(), String> {
        if let Some(look) = self.avatar(account).data {
            let layers = look.assets.iter().filter(|a| a.layer.is_some()).count();
            if !look.assets.iter().any(|a| a.id == asset && a.layer.is_some()) {
                return Err("That item has no layer to move.".into());
            }
            if layers < 2 {
                return Err("Only one layered item is worn.".into());
            }
        }
        self.inner
            .mutate(account, "move the layer", "", &["avatar"], move |f| async move {
                let now = avatar::v1::get_avatar(&f.client).await?;
                let assets =
                    relayered(now.assets.unwrap_or_default(), asset, outward).map_err(Error::Message)?;
                set_wearing(&f.client, wear_list(assets)).await?;
                forget_images(&f.inner, account.0);
                Ok(())
            });
        Ok(())
    }

    /// Paint `parts` (every part when empty) BrickColor `color`.
    pub fn set_body_color(&self, account: AccountId, parts: Vec<Part>, color: i32) -> Result<(), String> {
        if let Some(r) = self.avatar_rules().data
            && !r.palette.is_empty()
            && !r.palette.iter().any(|s| s.id == color)
        {
            return Err("That color is not in the palette.".into());
        }
        let parts = if parts.is_empty() {
            Part::ALL.to_vec()
        } else {
            parts
        };
        self.inner.mutate(
            account,
            "change the body color",
            "",
            &["avatar"],
            move |f| async move {
                let now = avatar::v1::get_avatar(&f.client).await?;
                let mut c = now.body_colors.map(colors).unwrap_or_default();
                for p in parts {
                    c.set(p, color);
                }
                avatar::v1::post_avatar_set_body_colors(&f.client, &colors_model(c)).await?;
                forget_images(&f.inner, account.0);
                Ok(())
            },
        );
        Ok(())
    }

    /// Set one body scale, moved onto the range Roblox allows.
    pub fn set_scale(&self, account: AccountId, scale: Scale, value: f64) -> Result<(), String> {
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
        self.inner.mutate(
            account,
            "change the body scale",
            "",
            &["avatar"],
            move |f| async move {
                let now = avatar::v1::get_avatar(&f.client).await?;
                let mut s = now.scales.map(scales).unwrap_or_default();
                scale.set(&mut s, value);
                avatar::v1::post_avatar_set_scales(&f.client, &scales_model(s)).await?;
                forget_images(&f.inner, account.0);
                Ok(())
            },
        );
        Ok(())
    }

    /// Put emote `asset` in hotbar `slot` (1 to 8), or clear the slot when `asset` is `None`.
    pub fn set_emote(&self, account: AccountId, slot: i32, asset: Option<u64>) -> Result<(), String> {
        if !(1..=8).contains(&slot) {
            return Err("Emote slots are 1 to 8.".into());
        }
        let (doing, done) = match asset {
            Some(_) => ("equip the emote", "Emote equipped."),
            None => ("clear the emote slot", "Emote slot cleared."),
        };
        self.inner
            .mutate(account, doing, done, &["avatar"], move |f| async move {
                match asset {
                    Some(id) => {
                        avatar::undocumented::post_emote_equip(&f.client, id as i64, slot).await?;
                    }
                    None => {
                        avatar::undocumented::delete_emote_unequip(&f.client, slot).await?;
                    }
                }
                Ok(())
            });
        Ok(())
    }

    /// Ask Roblox to render `account`'s avatar images again, for when they look stale.
    pub fn redraw_avatar(&self, account: AccountId) {
        self.inner.mutate(
            account,
            "redraw your avatar",
            "Avatar redrawn.",
            &["avatar"],
            move |f| async move {
                avatar::v1::post_avatar_redraw_thumbnail(&f.client).await?;
                forget_images(&f.inner, account.0);
                Ok(())
            },
        );
    }

    /// Switch `account` to R6 or R15. Going to R6 names the worn items that won't show there.
    pub fn set_rig(&self, account: AccountId, rig: Rig) {
        self.inner.mutate(
            account,
            "change the body type",
            "",
            &["avatar"],
            move |f| async move {
                let hidden: Vec<String> = if rig == Rig::R6 {
                    let now = avatar::v1::get_avatar(&f.client).await?;
                    now.assets
                        .unwrap_or_default()
                        .into_iter()
                        .filter(|a| a.asset_type.as_ref().and_then(|t| t.id).is_some_and(r15_only))
                        .filter_map(|a| a.name)
                        .collect()
                } else {
                    Vec::new()
                };
                set_rig(&f.client, rig).await?;
                forget_images(&f.inner, account.0);
                if !hidden.is_empty() {
                    f.inner.toast(
                        Some(account),
                        format!("R6 does not show {}.", hidden.join(", ")),
                        false,
                    );
                }
                Ok(())
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_rigs() {
        assert_eq!(Rig::parse("r15"), Some(Rig::R15));
        assert_eq!(Rig::parse(" R6 "), Some(Rig::R6));
        assert_eq!(Rig::parse("R10"), None);
    }

    fn worn(id: i64, t: i64) -> avatar::v1::AssetModelV2 {
        avatar::v1::AssetModelV2 {
            id: Some(id),
            asset_type: Some(avatar::v1::AssetTypeModel {
                id: Some(t),
                name: None,
            }),
            ..Default::default()
        }
    }

    #[test]
    fn toggling_respects_slot_limits() {
        let ids = |v: Vec<avatar::v1::AssetModelV2>| v.iter().map(|a| a.id.unwrap()).collect::<Vec<_>>();
        let now = vec![worn(1, 8), worn(2, 8), worn(3, 11)];
        assert_eq!(ids(toggled(now.clone(), 2, Some(8), Some(3))), [1, 3], "off");
        assert_eq!(ids(toggled(now.clone(), 4, Some(8), Some(3))), [1, 2, 3, 4]);
        assert_eq!(
            ids(toggled(now.clone(), 4, Some(8), Some(2))),
            [2, 3, 4],
            "oldest hat off"
        );
        assert_eq!(
            ids(toggled(now.clone(), 5, Some(11), Some(1))),
            [1, 2, 5],
            "shirt swapped"
        );
        assert_eq!(ids(toggled(now, 5, None, None)), [1, 2, 3, 5]);
    }

    fn layer(id: i64, order: i32) -> avatar::v1::AssetModelV2 {
        let mut a = worn(id, 67);
        a.meta = Some(avatar::v1::AssetMetaModelV1 {
            order: Some(order),
            version: Some(1),
            ..Default::default()
        });
        a
    }

    #[test]
    fn moves_layers() {
        let orders = |v: Vec<avatar::v1::AssetModelV2>| {
            v.iter()
                .map(|a| (a.id.unwrap(), a.meta.as_ref().and_then(|m| m.order)))
                .collect::<Vec<_>>()
        };
        let now = vec![worn(1, 8), layer(2, 5), layer(3, 9), layer(4, 7)];
        assert_eq!(
            orders(relayered(now.clone(), 2, true).unwrap()),
            [(1, None), (2, Some(7)), (3, Some(9)), (4, Some(5))]
        );
        assert_eq!(
            orders(relayered(now.clone(), 3, false).unwrap()),
            [(1, None), (2, Some(5)), (3, Some(7)), (4, Some(9))]
        );
        assert!(relayered(now.clone(), 3, true).is_err(), "already on top");
        assert!(relayered(now.clone(), 2, false).is_err(), "already innermost");
        assert!(relayered(now, 1, true).is_err(), "not layered");
        let tied = vec![layer(1, 4), layer(2, 4)];
        let r = orders(relayered(tied, 1, true).unwrap());
        assert!(r[0].1 > r[1].1, "{r:?}");
    }

    #[test]
    fn parses_parts_and_scales() {
        assert_eq!(Part::parse("Left Arm"), Some(Part::LeftArm));
        assert_eq!(Part::parse("right_leg"), Some(Part::RightLeg));
        assert_eq!(Part::parse("tail"), None);
        assert_eq!(Scale::parse("body type"), Some(Scale::BodyType));
        assert_eq!(Scale::parse("bodyType"), Some(Scale::BodyType));
        assert_eq!(Scale::parse("HEIGHT"), Some(Scale::Height));
        assert_eq!(Scale::parse("depth"), None);
    }

    #[test]
    fn snaps_scales_to_the_rule() {
        let r = ScaleRule {
            name: "height".into(),
            min: 0.9,
            max: 1.05,
            step: 0.01,
        };
        assert_eq!(snap(2.0, &r), 1.05);
        assert_eq!(snap(0.1, &r), 0.9);
        assert_eq!(snap(0.957, &r), 0.96);
        assert_eq!(snap(1.0, &r), 1.0);
    }

    #[test]
    fn reads_rules() {
        let r: avatar::v1::AvatarRulesModel = serde_json::from_str(
            r##"{"scales":{"bodyType":{"min":0,"max":1,"increment":0.01},"height":{"min":0.9,"max":1.05,"increment":0.01}},
            "bodyColorsPalette":[{"brickColorId":1,"hexColor":"#F2F3F3","name":"White"},{"brickColorId":26,"hexColor":"#1B2A35","name":"Black"}],
            "basicBodyColorsPalette":[{"brickColorId":26,"hexColor":"#1B2A35","name":"Black"}],
            "wearableAssetTypes":[{"id":8,"name":"Hat","maxNumber":3}]}"##,
        )
        .unwrap();
        let r = rules(r);
        assert_eq!(
            r.scales.iter().map(|s| s.name.as_str()).collect::<Vec<_>>(),
            ["height", "bodyType"]
        );
        assert_eq!(
            r.palette.iter().map(|s| s.basic).collect::<Vec<_>>(),
            [false, true]
        );
        assert_eq!(r.slots[0].max, 3);
        assert!(r.proportions);
    }

    #[test]
    fn carries_asset_meta_across_versions() {
        let a: avatar::v1::AssetModelV2 =
            serde_json::from_str(r#"{"id":5,"meta":{"order":3,"puffiness":0.5,"version":1}}"#).unwrap();
        let w = wear_list(vec![a]);
        let m = w[0].meta.as_ref().unwrap();
        assert_eq!((w[0].id, m.order, m.puffiness), (Some(5), Some(3), Some(0.5)));
    }
}

//! Item values for limiteds and trader stats for players, from Rolimons, a third-party site.
//! Opt-in (`Settings::item_values`): nothing is fetched until the user turns it on, every screen
//! works without it, and the UI always names the source. RAP itself comes from Roblox and needs
//! none of this.
//!
//! The request goes out with the anonymous client, and the transport never sends Roblox
//! credentials to a non-Roblox host anyway.

use std::collections::HashMap;

use bonsai_api::Error;
use bonsai_api::generated::api_rolimons_com;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::Core;
use crate::res::{Key, Res};

/// Shown next to every value.
pub const SOURCE: &str = "Rolimons";

/// How old the table may get before a screen that shows values refetches it (about 60 KB
/// gzipped). Nothing refetches while no such screen is open, since resources load on read.
const VALUES_AGE: i64 = 120;

/// Rolimons rescans a player every few hours at most, and rate limits per IP.
const PLAYER_AGE: i64 = 900;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Demand {
    Terrible,
    Low,
    Normal,
    High,
    Amazing,
}

impl Demand {
    pub fn label(self) -> &'static str {
        match self {
            Demand::Terrible => "Terrible",
            Demand::Low => "Low",
            Demand::Normal => "Normal",
            Demand::High => "High",
            Demand::Amazing => "Amazing",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Trend {
    Lowering,
    Unstable,
    Stable,
    Raising,
    Fluctuating,
}

impl Trend {
    pub fn label(self) -> &'static str {
        match self {
            Trend::Lowering => "Lowering",
            Trend::Unstable => "Unstable",
            Trend::Stable => "Stable",
            Trend::Raising => "Raising",
            Trend::Fluctuating => "Fluctuating",
        }
    }
}

/// Which limited system an item belongs to.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum LimitedKind {
    /// Limited 1.0: classic limiteds and limited uniques.
    Classic,
    /// Limited 2.0: the newer system, including UGC limiteds.
    New,
    #[default]
    Unknown,
}

/// One limited as Rolimons sees it.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ItemValue {
    pub name: String,
    pub acronym: String,
    pub rap: Option<i64>,
    /// Rolimons' value; `None` for items it doesn't value (RAP is used then).
    pub value: Option<i64>,
    pub demand: Option<Demand>,
    pub trend: Option<Trend>,
    /// RAP is inflated by a few sales and likely to fall.
    pub projected: bool,
    pub hyped: bool,
    pub rare: bool,
    pub kind: LimitedKind,
}

impl ItemValue {
    /// What the item is worth for trade math: the value when there is one, else RAP. Same as
    /// Rolimons' own `default_value` (index 4).
    pub fn worth(&self) -> Option<i64> {
        self.value.or(self.rap)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ValueTable {
    pub items: HashMap<u64, ItemValue>,
}

impl ValueTable {
    pub fn get(&self, asset_id: u64) -> Option<&ItemValue> {
        self.items.get(&asset_id)
    }
}

/// Decode `[name, acronym, rap, value, default_value, demand, trend, projected, hyped, rare,
/// limited_version, ...]`. `default_value` is skipped since [`ItemValue::worth`] derives it.
/// Anything malformed is skipped; extra trailing fields are ignored.
pub(crate) fn decode(items: HashMap<String, Vec<Value>>) -> ValueTable {
    let num = |v: Option<&Value>| v.and_then(Value::as_i64).filter(|n| *n >= 0);
    let flag = |v: Option<&Value>| v.and_then(Value::as_i64) == Some(1);
    let table = items
        .into_iter()
        .filter_map(|(id, a)| {
            let id = id.parse::<u64>().ok()?;
            let s = |i: usize| a.get(i).and_then(Value::as_str).unwrap_or("").to_string();
            let demand = match num(a.get(5)) {
                Some(0) => Some(Demand::Terrible),
                Some(1) => Some(Demand::Low),
                Some(2) => Some(Demand::Normal),
                Some(3) => Some(Demand::High),
                Some(4) => Some(Demand::Amazing),
                _ => None,
            };
            let trend = match num(a.get(6)) {
                Some(0) => Some(Trend::Lowering),
                Some(1) => Some(Trend::Unstable),
                Some(2) => Some(Trend::Stable),
                Some(3) => Some(Trend::Raising),
                Some(4) => Some(Trend::Fluctuating),
                _ => None,
            };
            let kind = match a.get(10).and_then(Value::as_i64) {
                Some(1) => LimitedKind::Classic,
                Some(2) => LimitedKind::New,
                _ => LimitedKind::Unknown,
            };
            Some((
                id,
                ItemValue {
                    name: s(0),
                    acronym: s(1),
                    rap: num(a.get(2)),
                    value: num(a.get(3)),
                    demand,
                    trend,
                    projected: flag(a.get(7)),
                    hyped: flag(a.get(8)),
                    rare: flag(a.get(9)),
                    kind,
                },
            ))
        })
        .collect();
    ValueTable { items: table }
}

/// What Rolimons knows about a player's limiteds. Times are unix seconds.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PlayerStats {
    /// Rolimons has scanned them. When false, everything else is empty.
    pub tracked: bool,
    /// Their limiteds' worth (value, else RAP, per item).
    pub value: Option<i64>,
    pub rap: Option<i64>,
    /// Place on Rolimons' leaderboard by value; `None` without value.
    pub rank: Option<i64>,
    pub premium: bool,
    /// Their inventory is private, so the stats are from before they hid it.
    pub private: bool,
    pub terminated: bool,
    pub stats_updated: Option<i64>,
    /// When and where Rolimons last saw them online.
    pub last_online: Option<i64>,
    pub last_location: String,
    /// In Rolimons' own order.
    pub badges: Vec<RoliBadge>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum BadgeGroup {
    /// Inventory worth a threshold.
    Value,
    /// Owning a famous item or serial.
    Item,
    /// Owning many items or much of one.
    Collection,
    /// Posting trade ads.
    Trading,
    /// Rolimons community: awards, events, supporters.
    Community,
    #[default]
    Unknown,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RoliBadge {
    pub key: String,
    /// The name rolimons.com shows.
    pub name: String,
    pub group: BadgeGroup,
    /// Tiers of one goal share a family, e.g. "value" for 100K+ to 20M+. Empty for one-offs.
    pub family: String,
    pub earned: i64,
}

/// Every badge rolimons.com/player shows, in its order, with the name it shows (taken from the
/// site's player22.js on 2026-10-14). The last field names a family of tiers ("value" for 100K+
/// to 20M+); within one, the highest tier comes first.
const BADGES: &[(&str, &str, BadgeGroup, &str)] = {
    use BadgeGroup::*;
    &[
        ("value_20m", "20M+", Value, "value"),
        ("value_10m", "10M+", Value, "value"),
        ("value_5m", "5M+", Value, "value"),
        ("value_1m", "1M+", Value, "value"),
        ("value_500k", "500K+", Value, "value"),
        ("value_100k", "100K+", Value, "value"),
        ("roli_award_winner", "Roli Award Winner", Community, "roli_award"),
        (
            "roli_award_nominee",
            "Roli Award Nominee",
            Community,
            "roli_award",
        ),
        ("own_lucky_cat_uaid", "Lucky Cat", Item, ""),
        ("own_1_serial_1", "Serial #1", Item, ""),
        ("own_1_serial_1337", "L337", Item, ""),
        ("own_1_sequential_serial", "Sequential Serial", Item, ""),
        ("own_1_serial_1_to_9", "Low Serial", Item, ""),
        ("own_1_big_dominus", "Big Dominator", Item, ""),
        ("own_1_dominus", "Dominator", Item, ""),
        ("own_1_stf", "Sparkly", Item, ""),
        ("own_1_valued_federation_item", "Federated", Item, ""),
        ("own_1_immortal_sword", "Enduring", Item, ""),
        ("own_epic_katana_set", "Epic Blade Collector", Item, ""),
        ("own_1_kotn_item", "Evening Royalty", Item, ""),
        ("own_15_noob", "Noobie", Collection, "noob"),
        ("own_5_noob", "Noob", Collection, "noob"),
        ("own_10_rares", "Rare Supremist", Collection, "rares"),
        ("own_3_rares", "Rare Enthusiast", Collection, "rares"),
        ("own_1_rare", "Rare Owner", Collection, "rares"),
        ("create_10000_trade_ads", "Boundless Trader", Trading, "trade_ads"),
        ("create_1000_trade_ads", "Active Trader", Trading, "trade_ads"),
        ("create_100_trade_ads", "Frequent Trader", Trading, "trade_ads"),
        ("create_10_trade_ads", "Trade Advertiser", Trading, "trade_ads"),
        ("own_all_asset_types", "Accessorized", Collection, ""),
        (
            "own_50_pct_of_1_item",
            "Uncontrollable Addiction",
            Collection,
            "share",
        ),
        ("own_25_pct_of_1_item", "Unhealthy Obsession", Collection, "share"),
        ("own_10_pct_of_1_item", "Modest Enthusiasm", Collection, "share"),
        ("own_100_of_1_item", "Mega Hoarder", Collection, "hoard"),
        ("own_50_of_1_item", "Hoarder", Collection, "hoard"),
        ("own_10_of_1_item", "Mini Hoarder", Collection, "hoard"),
        ("own_1000_items", "Incurable Collector", Collection, "items"),
        ("own_100_items", "Devout Collector", Collection, "items"),
        ("own_10_items", "Collector", Collection, "items"),
        ("contributor", "Contributor", Community, ""),
        (
            "sword_fighting_champion",
            "Sword Fighting Champion",
            Community,
            "",
        ),
        ("event_winner", "Event Winner", Community, ""),
        ("game_night_winner", "Game Night Winner", Community, ""),
        ("booster", "Booster", Community, ""),
        ("verified", "Verified", Community, ""),
        ("roligang", "Roligang", Community, ""),
    ]
};

/// Known badges in Rolimons' order, then any new ones by key with a name made from the key.
pub(crate) fn badges(raw: std::collections::HashMap<String, i64>) -> Vec<RoliBadge> {
    let rank = |k: &str| BADGES.iter().position(|b| b.0 == k).unwrap_or(usize::MAX);
    let mut out: Vec<RoliBadge> = raw
        .into_iter()
        .map(|(key, earned)| {
            let (name, group, family) = match BADGES.iter().find(|b| b.0 == key) {
                Some(b) => (b.1.to_string(), b.2, b.3.to_string()),
                None => {
                    let mut n = key.replace('_', " ");
                    if let Some(c) = n.get(..1) {
                        n.replace_range(..1, &c.to_uppercase());
                    }
                    (n, BadgeGroup::Unknown, String::new())
                }
            };
            RoliBadge {
                key,
                name,
                group,
                family,
                earned,
            }
        })
        .collect();
    out.sort_by(|a, b| rank(&a.key).cmp(&rank(&b.key)).then_with(|| a.key.cmp(&b.key)));
    out
}

pub(crate) fn player_stats(p: api_rolimons_com::undocumented::PlayerInfo) -> PlayerStats {
    let pos = |n: Option<i64>| n.filter(|n| *n > 0);
    PlayerStats {
        tracked: p.success.unwrap_or(true),
        value: p.value.filter(|n| *n >= 0),
        rap: p.rap.filter(|n| *n >= 0),
        rank: pos(p.rank),
        premium: p.premium.unwrap_or(false),
        private: p.privacy_enabled.unwrap_or(false),
        terminated: p.terminated.unwrap_or(false),
        stats_updated: pos(p.stats_updated),
        last_online: pos(p.last_online),
        last_location: p.last_location.unwrap_or_default(),
        badges: badges(p.rolibadges.unwrap_or_default()),
    }
}

impl Core {
    /// Whether the user turned item values on.
    pub fn values_enabled(&self) -> bool {
        self.settings().item_values
    }

    /// The value table, or nothing (and no request) while values are off.
    pub fn item_values(&self) -> Res<ValueTable> {
        if !self.values_enabled() {
            return Res::default();
        }
        self.inner
            .res(Key::shared("item_values", ""), VALUES_AGE, |f| async move {
                let r = api_rolimons_com::undocumented::get_item_values(&f.client).await?;
                Ok(decode(r.items.unwrap_or_default()))
            })
    }

    /// Rolimons' stats for a player, or nothing (and no request) while values are off. A
    /// player Rolimons never scanned comes back untracked rather than as an error.
    pub fn player_stats(&self, user_id: u64) -> Res<PlayerStats> {
        if !self.values_enabled() {
            return Res::default();
        }
        self.inner.res(
            Key::shared("player_stats", user_id),
            PLAYER_AGE,
            move |f| async move {
                match api_rolimons_com::undocumented::get_player_info(&f.client, user_id as i64).await {
                    Ok(p) => Ok(player_stats(p)),
                    Err(Error::Status { status: 404, .. }) => Ok(PlayerStats::default()),
                    Err(e) => Err(e),
                }
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn decodes_rows_and_skips_junk() {
        let mut m = HashMap::new();
        m.insert(
            "1029025".to_string(),
            serde_json::from_value(json!([
                "The Classic ROBLOX Fedora",
                "CF",
                406464,
                450000,
                450000,
                4,
                2,
                -1,
                -1,
                1,
                1
            ]))
            .unwrap(),
        );
        m.insert(
            "1028606".to_string(),
            serde_json::from_value(json!(["Red Baseball Cap", "", 1309, -1, 1309, -1, -1, 1, -1, -1]))
                .unwrap(),
        );
        m.insert(
            "7".to_string(),
            serde_json::from_value(json!(["New", "", 50, -1, 50, -1, -1, -1, -1, -1, 2])).unwrap(),
        );
        m.insert(
            "8".to_string(),
            serde_json::from_value(json!(["Future", "", 50, -1, 50, -1, -1, -1, -1, -1, 9])).unwrap(),
        );
        m.insert("nope".to_string(), vec![json!("x")]);
        m.insert("5".to_string(), vec![json!(1), json!(null)]);
        let t = decode(m);
        let f = t.get(1_029_025).unwrap();
        assert_eq!(f.acronym, "CF");
        assert_eq!((f.rap, f.value), (Some(406_464), Some(450_000)));
        assert_eq!(f.demand, Some(Demand::Amazing));
        assert_eq!(f.trend, Some(Trend::Stable));
        assert!(f.rare && !f.projected && !f.hyped);
        assert_eq!(f.worth(), Some(450_000));
        assert_eq!(f.kind, LimitedKind::Classic);
        let cap = t.get(1_028_606).unwrap();
        assert_eq!(cap.value, None);
        assert_eq!(cap.worth(), Some(1309), "falls back to RAP");
        assert!(cap.projected && cap.demand.is_none());
        assert_eq!(cap.kind, LimitedKind::Unknown, "row without index 10");
        assert_eq!(t.get(7).unwrap().kind, LimitedKind::New);
        assert_eq!(t.get(8).unwrap().kind, LimitedKind::Unknown);
        // A short or malformed row decodes to an empty value rather than failing the table.
        assert_eq!(t.get(5).unwrap(), &ItemValue::default());
        assert_eq!(t.items.len(), 5);
    }

    #[test]
    fn player_stats_order_badges_and_drop_empty_numbers() {
        let p: api_rolimons_com::undocumented::PlayerInfo = serde_json::from_value(json!({
            "success": true, "name": "John Doe", "value": 0, "rap": 0, "rank": null,
            "premium": false, "privacy_enabled": true, "terminated": false,
            "stats_updated": 1761971403, "last_online": 1619902655, "last_location": "Website",
            "rolibadges": {"own_10_items": 3, "value_1m": 1, "value_20m": 2, "brand_new_one": 4}
        }))
        .unwrap();
        let s = player_stats(p);
        assert!(s.tracked && s.private);
        assert_eq!((s.value, s.rap, s.rank), (Some(0), Some(0), None));
        assert_eq!(s.last_online, Some(1_619_902_655));
        let keys: Vec<_> = s.badges.iter().map(|b| b.key.as_str()).collect();
        assert_eq!(keys, ["value_20m", "value_1m", "own_10_items", "brand_new_one"]);
        assert_eq!(s.badges[0].name, "20M+");
        assert_eq!(s.badges[0].group, BadgeGroup::Value);
        assert_eq!(
            (s.badges[0].family.as_str(), s.badges[2].family.as_str()),
            ("value", "items")
        );
        assert_eq!(s.badges[3].name, "Brand new one");
        assert_eq!(s.badges[3].group, BadgeGroup::Unknown);
        assert!(!PlayerStats::default().tracked);
    }

    #[test]
    fn reads_rows_cached_before_kind_existed() {
        let v: ItemValue = serde_json::from_value(json!({"name": "Old", "rap": 5})).unwrap();
        assert_eq!((v.rap, v.kind), (Some(5), LimitedKind::Unknown));
    }
}

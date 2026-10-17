//! Transactions: Robux in and out for an account, or for a community it may see the funds of.
//!
//! Roblox lists one type at a time, newest first, with an offset cursor and no date filter. To
//! offer date ranges, all-time sums, search and CSV, rows are kept in a local ledger (SQLite,
//! private to the account). A sync reads each type from the newest row down to the first row we
//! already had settled; older history is fetched only when asked (`load_older`). Rows are never
//! dropped because a page left them out: under load Roblox sometimes answers with an empty page.
//!
//! The API answers 429 after a few quick calls. The transport paces the whole
//! `/transaction-records/` route for every account (see `CoreConfig::standard`), and a sync
//! that follows a recent one asks for the last day's totals first and reads only the types that
//! moved.
//!
//! Everything here reads. Nothing spends, sends or changes anything on Roblox.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use bonsai_api::generated::apis::undocumented as api;
use bonsai_api::{AccountId, Client, Error};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

use crate::Core;
use crate::cache::{Kind, now};
use crate::core::Inner;
use crate::fmt::parse_iso;
use crate::res::{Key, Res};

/// A sync on opening the screen runs at most this often.
const SYNC_AGE: i64 = 300;
const TOTALS_AGE: i64 = 300;
const KINDS_AGE: i64 = 3600;
const PAGE: i32 = 100;
/// A sync stops after this many pages of one type even without reaching known rows. Whatever
/// is left is fetched by `load_older`.
const HEAD_PAGES: usize = 20;
/// Pages per type in one `load_older` run.
const OLDER_PAGES: usize = 50;
/// A page that is rate limited is tried again this many times. The limiter holds the retry
/// until the block ends, so it doesn't add to it.
const RATE_WAITS: u32 = 4;
/// A sync uses the last day's totals to skip types only when the last sync is this recent.
const DAY_SKIP_AGE: i64 = 20 * 3600;
/// The new-sale check runs at most this often per account.
const SALES_WATCH: i64 = 300;

// ---------------------------------------------------------------- types

/// A transaction type, as the list API takes it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TxKind {
    Purchase,
    Sale,
    AffiliateSale,
    PremiumStipend,
    EngagementPayout,
    GroupPayout,
    GroupEngagementPayout,
    CurrencyPurchase,
    TradeRobux,
    AdSpend,
    DevEx,
    IndividualToGroup,
    CsAdjustment,
    CreatorRewardsPayout,
    CurrencyTransfer,
    #[serde(other)]
    Unknown,
}

impl TxKind {
    /// In the order the website's summary lists them.
    pub const ALL: [TxKind; 15] = [
        TxKind::PremiumStipend,
        TxKind::CurrencyPurchase,
        TxKind::Sale,
        TxKind::CreatorRewardsPayout,
        TxKind::EngagementPayout,
        TxKind::AffiliateSale,
        TxKind::GroupPayout,
        TxKind::GroupEngagementPayout,
        TxKind::TradeRobux,
        TxKind::CurrencyTransfer,
        TxKind::CsAdjustment,
        TxKind::Purchase,
        TxKind::DevEx,
        TxKind::AdSpend,
        TxKind::IndividualToGroup,
    ];

    /// Types a community's revenue page lists. Not run live (no test community); a type the API
    /// refuses for a community is skipped.
    pub const GROUP: [TxKind; 6] = [
        TxKind::Sale,
        TxKind::AffiliateSale,
        TxKind::GroupPayout,
        TxKind::IndividualToGroup,
        TxKind::CreatorRewardsPayout,
        TxKind::AdSpend,
    ];

    /// `transactionType` for the list API.
    pub fn api(self) -> &'static str {
        match self {
            TxKind::Purchase => "Purchase",
            TxKind::Sale => "Sale",
            TxKind::AffiliateSale => "AffiliateSale",
            TxKind::PremiumStipend => "PremiumStipend",
            TxKind::EngagementPayout => "EngagementPayout",
            TxKind::GroupPayout => "GroupPayout",
            TxKind::GroupEngagementPayout => "GroupEngagementPayout",
            TxKind::CurrencyPurchase => "CurrencyPurchase",
            TxKind::TradeRobux => "TradeRobux",
            TxKind::AdSpend => "AdSpend",
            TxKind::DevEx => "DevEx",
            TxKind::IndividualToGroup => "IndividualToGroup",
            TxKind::CsAdjustment => "CsAdjustment",
            TxKind::CreatorRewardsPayout => "CreatorRewardsPayout",
            TxKind::CurrencyTransfer => "CurrencyTransfer",
            TxKind::Unknown => "",
        }
    }

    /// The `Has...` flag in `transaction-types`.
    fn flag(self) -> String {
        match self {
            TxKind::CsAdjustment => "HasCSAdjustment".into(),
            TxKind::CurrencyTransfer => "HasTransfer".into(),
            k => format!("Has{}", k.api()),
        }
    }

    /// Stable short id for the UI, commands and CSV.
    pub fn key(self) -> &'static str {
        match self {
            TxKind::Purchase => "purchases",
            TxKind::Sale => "sales",
            TxKind::AffiliateSale => "commissions",
            TxKind::PremiumStipend => "stipends",
            TxKind::EngagementPayout => "premium_payouts",
            TxKind::GroupPayout => "group_payouts",
            TxKind::GroupEngagementPayout => "group_premium_payouts",
            TxKind::CurrencyPurchase => "robux_bought",
            TxKind::TradeRobux => "trade_robux",
            TxKind::AdSpend => "ad_spend",
            TxKind::DevEx => "devex",
            TxKind::IndividualToGroup => "to_communities",
            TxKind::CsAdjustment => "adjustments",
            TxKind::CreatorRewardsPayout => "creator_rewards",
            TxKind::CurrencyTransfer => "transfers",
            TxKind::Unknown => "other",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            TxKind::Purchase => "Purchases",
            TxKind::Sale => "Sales",
            TxKind::AffiliateSale => "Commissions",
            TxKind::PremiumStipend => "Premium stipends",
            TxKind::EngagementPayout => "Premium payouts",
            TxKind::GroupPayout => "Group payouts",
            TxKind::GroupEngagementPayout => "Group premium payouts",
            TxKind::CurrencyPurchase => "Robux bought",
            TxKind::TradeRobux => "Trade Robux",
            TxKind::AdSpend => "Ad spend",
            TxKind::DevEx => "DevEx",
            TxKind::IndividualToGroup => "Sent to communities",
            TxKind::CsAdjustment => "Adjustments",
            TxKind::CreatorRewardsPayout => "Creator rewards",
            TxKind::CurrencyTransfer => "Robux transfers",
            TxKind::Unknown => "Other",
        }
    }

    pub fn from_key(k: &str) -> Option<TxKind> {
        let k = k.trim().to_ascii_lowercase();
        TxKind::ALL
            .into_iter()
            .find(|t| t.key() == k || t.api().eq_ignore_ascii_case(&k) || t.label().eq_ignore_ascii_case(&k))
    }
}

/// Whose transactions: the account's own, or a community's.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Scope {
    Me,
    Group(u64),
}

impl Scope {
    pub fn key(self) -> String {
        match self {
            Scope::Me => "me".into(),
            Scope::Group(g) => format!("group:{g}"),
        }
    }
    pub fn from_key(k: &str) -> Option<Scope> {
        match k.trim() {
            "" | "me" => Some(Scope::Me),
            k => k
                .strip_prefix("group:")
                .unwrap_or(k)
                .parse()
                .ok()
                .map(Scope::Group),
        }
    }
}

/// The last day, week, month or year, as Roblox totals them.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Period {
    Day,
    Week,
    #[default]
    Month,
    Year,
}

impl Period {
    pub const ALL: [Period; 4] = [Period::Day, Period::Week, Period::Month, Period::Year];
    pub fn api(self) -> &'static str {
        match self {
            Period::Day => "Day",
            Period::Week => "Week",
            Period::Month => "Month",
            Period::Year => "Year",
        }
    }
    pub fn key(self) -> &'static str {
        match self {
            Period::Day => "day",
            Period::Week => "week",
            Period::Month => "month",
            Period::Year => "year",
        }
    }
    pub fn from_key(k: &str) -> Option<Period> {
        Period::ALL
            .into_iter()
            .find(|p| p.key().eq_ignore_ascii_case(k.trim()))
    }
    pub fn secs(self) -> i64 {
        match self {
            Period::Day => 86_400,
            Period::Week => 7 * 86_400,
            Period::Month => 30 * 86_400,
            Period::Year => 365 * 86_400,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Agent {
    pub id: u64,
    pub group: bool,
    pub name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TxPlace {
    pub place_id: u64,
    pub universe_id: u64,
    pub name: String,
}

/// What a transaction was for.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TxItem {
    pub id: u64,
    pub name: String,
    /// Asset, Bundle, GamePass, DeveloperProduct, PrivateServer, Place, ...
    pub kind: String,
    pub place: Option<TxPlace>,
}

/// One row of the ledger.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Txn {
    /// `idHash`; the API's numeric id is always 0.
    pub id: String,
    pub kind: TxKind,
    /// Unix seconds.
    pub at: i64,
    /// As Roblox sent it.
    pub created: String,
    pub pending: bool,
    /// Robux; negative when spent.
    pub amount: i64,
    /// The other side: seller, buyer, payer.
    pub agent: Option<Agent>,
    pub item: Option<TxItem>,
}

impl Txn {
    fn from_api(kind: TxKind, r: api::TransactionRecord) -> Option<Txn> {
        let id = r.id_hash.filter(|h| !h.is_empty())?;
        let created = r.created.unwrap_or_default();
        Some(Txn {
            id,
            kind,
            at: parse_iso(&created).unwrap_or(0),
            created,
            pending: r.is_pending.unwrap_or(false),
            amount: r.currency.and_then(|c| c.amount).unwrap_or(0),
            agent: r.agent.and_then(|a| {
                Some(Agent {
                    id: u64::try_from(a.id?).ok()?,
                    group: a.type_.as_deref() == Some("Group"),
                    name: a.name.unwrap_or_default(),
                })
            }),
            item: r.details.and_then(|d| {
                Some(TxItem {
                    id: u64::try_from(d.id?).ok()?,
                    name: d.name.unwrap_or_default(),
                    kind: d.type_.unwrap_or_default(),
                    place: d.place.map(|p| TxPlace {
                        place_id: p.place_id.unwrap_or(0).max(0) as u64,
                        universe_id: p.universe_id.unwrap_or(0).max(0) as u64,
                        name: p.name.unwrap_or_default(),
                    }),
                })
            }),
        })
    }

    /// The experience it happened in, if any.
    pub fn experience(&self) -> Option<&TxPlace> {
        self.item.as_ref()?.place.as_ref()
    }
}

// ---------------------------------------------------------------- ledger

/// How far one type has been read.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct KindSync {
    pub kind: Option<TxKind>,
    /// Where `load_older` goes on. `None` with `complete` false: never read.
    pub older: Option<String>,
    /// Every row of this type is in the ledger.
    pub complete: bool,
}

/// Saved with the ledger (as a `Res` row, so clearing the cache drops both).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
struct SyncState {
    synced_at: i64,
    kinds: Vec<KindSync>,
}

/// One scope's ledger as the screen sees it.
#[derive(Clone, Debug, Default)]
pub struct Ledger {
    /// Newest first.
    pub rows: Arc<Vec<Txn>>,
    pub loaded: bool,
    pub syncing: bool,
    pub loading_older: bool,
    pub synced_at: i64,
    pub error: Option<String>,
    pub kinds: Vec<KindSync>,
}

impl Ledger {
    /// The oldest row we have.
    pub fn oldest(&self) -> Option<i64> {
        self.rows.last().map(|r| r.at)
    }
    /// Every type that was read has been read to its first row.
    pub fn complete(&self) -> bool {
        !self.kinds.is_empty() && self.kinds.iter().all(|k| k.complete)
    }
    pub fn has_older(&self) -> bool {
        self.kinds.iter().any(|k| !k.complete)
    }
}

#[derive(Default)]
pub(crate) struct Ledgers {
    map: RwLock<HashMap<(AccountId, Scope), Ledger>>,
    /// Bumped to stop `load_older`.
    stop: RwLock<HashSet<(AccountId, Scope)>>,
}

impl Ledgers {
    fn get(&self, a: AccountId, s: Scope) -> Ledger {
        self.map.read().get(&(a, s)).cloned().unwrap_or_default()
    }
    fn edit(&self, a: AccountId, s: Scope, f: impl FnOnce(&mut Ledger)) {
        f(self.map.write().entry((a, s)).or_default());
    }
    pub(crate) fn clear(&self) {
        self.map.write().clear();
    }
    pub(crate) fn forget_account(&self, a: AccountId) {
        self.map.write().retain(|k, _| k.0 != a);
    }
}

/// Puts `new` into `rows` (newest first), replacing rows with the same type and id.
pub fn merge(rows: &[Txn], new: Vec<Txn>) -> Vec<Txn> {
    let mut by: HashMap<(TxKind, String), Txn> =
        rows.iter().map(|r| ((r.kind, r.id.clone()), r.clone())).collect();
    for r in new {
        by.insert((r.kind, r.id.clone()), r);
    }
    let mut out: Vec<Txn> = by.into_values().collect();
    out.sort_by(|a, b| b.at.cmp(&a.at).then_with(|| a.id.cmp(&b.id)));
    out
}

/// A sync of one type can stop at a page holding a row we already had settled. Pending rows
/// can still change (released or refunded), so they don't count.
pub fn reached_known(known: &HashMap<(TxKind, String), bool>, page: &[Txn]) -> bool {
    page.iter()
        .any(|r| known.get(&(r.kind, r.id.clone())) == Some(&false))
}

/// Types a [`Totals`] reports. A community's summary has no ad spend.
fn totals_cover(scope: Scope) -> &'static [TxKind] {
    match scope {
        Scope::Me => &TxKind::ALL,
        Scope::Group(_) => &[
            TxKind::Sale,
            TxKind::AffiliateSale,
            TxKind::GroupPayout,
            TxKind::IndividualToGroup,
            TxKind::CreatorRewardsPayout,
        ],
    }
}

/// The types a sync reads when the last sync is under a day old and `day` is Roblox's total
/// for the last day. A type with no Robux that day has no new rows, so it is skipped, unless
/// the totals don't report it, it was never read, or we hold pending rows of it (they settle
/// days later without moving the day's total). Pending Robux that day also means a new sale or
/// commission.
pub fn kinds_to_read(
    kinds: &[TxKind],
    day: &Totals,
    covered: &[TxKind],
    started: &HashSet<TxKind>,
    pending: &HashSet<TxKind>,
) -> Vec<TxKind> {
    kinds
        .iter()
        .copied()
        .filter(|k| {
            !covered.contains(k)
                || !started.contains(k)
                || pending.contains(k)
                || day.by_kind.iter().any(|(d, _)| d == k)
                || day.pending != 0 && matches!(k, TxKind::Sale | TxKind::AffiliateSale)
        })
        .collect()
}

// ---------------------------------------------------------------- filters and sums

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Filter {
    /// Empty: every type.
    pub kinds: Vec<TxKind>,
    /// Unix seconds, inclusive.
    pub from: Option<i64>,
    /// Unix seconds, exclusive.
    pub to: Option<i64>,
    /// Matches item, experience and the other side's name, and ids.
    pub text: String,
    /// At least this many Robux either way.
    pub min_amount: Option<i64>,
    pub pending_only: bool,
}

impl Filter {
    pub fn matches(&self, r: &Txn) -> bool {
        if !self.kinds.is_empty() && !self.kinds.contains(&r.kind) {
            return false;
        }
        if self.from.is_some_and(|f| r.at < f) || self.to.is_some_and(|t| r.at >= t) {
            return false;
        }
        if self.min_amount.is_some_and(|m| r.amount.abs() < m) {
            return false;
        }
        if self.pending_only && !r.pending {
            return false;
        }
        let q = self.text.trim().to_lowercase();
        if q.is_empty() {
            return true;
        }
        let item = r.item.as_ref();
        let hay = [
            item.map(|i| i.name.as_str()),
            r.experience().map(|p| p.name.as_str()),
            r.agent.as_ref().map(|a| a.name.as_str()),
        ];
        hay.into_iter().flatten().any(|s| s.to_lowercase().contains(&q))
            || item.is_some_and(|i| i.id.to_string() == q)
            || r.agent.as_ref().is_some_and(|a| a.id.to_string() == q)
    }
}

/// Robux in and out over some rows. Pending Robux is counted apart: it isn't spendable yet.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Sums {
    pub count: usize,
    pub incoming: i64,
    pub outgoing: i64,
    pub pending: i64,
    /// Per type, in [`TxKind::ALL`] order, only types that occur: (type, Robux, rows).
    pub by_kind: Vec<(TxKind, i64, usize)>,
}

impl Sums {
    pub fn net(&self) -> i64 {
        self.incoming + self.outgoing
    }
}

pub fn sums<'a>(rows: impl IntoIterator<Item = &'a Txn>) -> Sums {
    let mut s = Sums::default();
    let mut by: HashMap<TxKind, (i64, usize)> = HashMap::new();
    for r in rows {
        s.count += 1;
        if r.pending {
            s.pending += r.amount;
        } else if r.amount >= 0 {
            s.incoming += r.amount;
        } else {
            s.outgoing += r.amount;
        }
        let e = by.entry(r.kind).or_default();
        e.0 += r.amount;
        e.1 += 1;
    }
    s.by_kind = TxKind::ALL
        .into_iter()
        .chain([TxKind::Unknown])
        .filter_map(|k| by.get(&k).map(|(a, n)| (k, *a, *n)))
        .collect();
    s
}

/// Robux in and out per calendar month (`YYYY-MM` in local time), oldest first.
pub fn by_month<'a>(rows: impl IntoIterator<Item = &'a Txn>, offset_secs: i64) -> Vec<(String, i64, i64)> {
    let mut m: std::collections::BTreeMap<String, (i64, i64)> = Default::default();
    for r in rows {
        if r.pending || r.at <= 0 {
            continue;
        }
        let month = crate::fmt::iso_utc(r.at + offset_secs)
            .get(0..7)
            .unwrap_or_default()
            .to_string();
        let e = m.entry(month).or_default();
        if r.amount >= 0 {
            e.0 += r.amount;
        } else {
            e.1 += r.amount;
        }
    }
    m.into_iter().map(|(k, (i, o))| (k, i, o)).collect()
}

/// Something rows are grouped by, with its Robux total and row count.
#[derive(Clone, Debug, PartialEq)]
pub struct Group {
    pub id: u64,
    pub name: String,
    pub amount: i64,
    pub count: usize,
    /// For agents: a community, not a person.
    pub group: bool,
}

fn top(mut m: HashMap<u64, Group>, n: usize) -> Vec<Group> {
    let mut v: Vec<Group> = m.drain().map(|(_, g)| g).collect();
    v.sort_by(|a, b| {
        b.amount
            .abs()
            .cmp(&a.amount.abs())
            .then(b.count.cmp(&a.count))
            .then(a.name.cmp(&b.name))
    });
    v.truncate(n);
    v
}

/// Robux spent per experience (passes, developer products, private servers), most first.
pub fn spend_by_experience<'a>(rows: impl IntoIterator<Item = &'a Txn>, n: usize) -> Vec<Group> {
    let mut m: HashMap<u64, Group> = HashMap::new();
    for r in rows {
        if r.kind != TxKind::Purchase || r.amount >= 0 {
            continue;
        }
        let Some(p) = r.experience().filter(|p| p.universe_id > 0) else {
            continue;
        };
        let g = m.entry(p.universe_id).or_insert_with(|| Group {
            id: p.universe_id,
            name: p.name.clone(),
            amount: 0,
            count: 0,
            group: false,
        });
        g.amount += r.amount;
        g.count += 1;
    }
    top(m, n)
}

/// Best-selling items by Robux earned.
pub fn top_items<'a>(rows: impl IntoIterator<Item = &'a Txn>, n: usize) -> Vec<Group> {
    let mut m: HashMap<u64, Group> = HashMap::new();
    for r in rows {
        if r.kind != TxKind::Sale {
            continue;
        }
        let Some(i) = &r.item else { continue };
        let g = m.entry(i.id).or_insert_with(|| Group {
            id: i.id,
            name: i.name.clone(),
            amount: 0,
            count: 0,
            group: false,
        });
        g.amount += r.amount;
        g.count += 1;
    }
    top(m, n)
}

/// Who bought the most, by Robux earned from them.
pub fn top_buyers<'a>(rows: impl IntoIterator<Item = &'a Txn>, n: usize) -> Vec<Group> {
    let mut m: HashMap<u64, Group> = HashMap::new();
    for r in rows {
        if r.kind != TxKind::Sale {
            continue;
        }
        let Some(a) = &r.agent else { continue };
        let g = m.entry(a.id).or_insert_with(|| Group {
            id: a.id,
            name: a.name.clone(),
            amount: 0,
            count: 0,
            group: a.group,
        });
        g.amount += r.amount;
        g.count += 1;
    }
    top(m, n)
}

fn csv_field(s: &str) -> String {
    if s.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_owned()
    }
}

/// The rows as CSV, one line per transaction.
pub fn csv<'a>(rows: impl IntoIterator<Item = &'a Txn>) -> String {
    let mut out = String::from(
        "date,type,amount,pending,item,item_type,item_id,experience,universe_id,other_party,other_party_type,other_party_id,id\n",
    );
    for r in rows {
        let i = r.item.as_ref();
        let p = r.experience();
        let a = r.agent.as_ref();
        let line = [
            csv_field(&r.created),
            r.kind.key().to_string(),
            r.amount.to_string(),
            r.pending.to_string(),
            csv_field(i.map(|i| i.name.as_str()).unwrap_or("")),
            csv_field(i.map(|i| i.kind.as_str()).unwrap_or("")),
            i.map(|i| i.id.to_string()).unwrap_or_default(),
            csv_field(p.map(|p| p.name.as_str()).unwrap_or("")),
            p.map(|p| p.universe_id.to_string()).unwrap_or_default(),
            csv_field(a.map(|a| a.name.as_str()).unwrap_or("")),
            a.map(|a| if a.group { "group" } else { "user" })
                .unwrap_or("")
                .to_string(),
            a.map(|a| a.id.to_string()).unwrap_or_default(),
            csv_field(&r.id),
        ];
        out.push_str(&line.join(","));
        out.push('\n');
    }
    out
}

// ---------------------------------------------------------------- Roblox totals

/// Totals for a period as Roblox counts them.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Totals {
    pub incoming: i64,
    pub outgoing: i64,
    pub pending: i64,
    /// Non-zero types only, in [`TxKind::ALL`] order.
    pub by_kind: Vec<(TxKind, i64)>,
}

fn nonzero(v: Vec<(TxKind, Option<i64>)>) -> Vec<(TxKind, i64)> {
    v.into_iter()
        .filter_map(|(k, a)| a.filter(|a| *a != 0).map(|a| (k, a)))
        .collect()
}

fn user_totals(t: api::TransactionTotals) -> Totals {
    Totals {
        incoming: t.incoming_robux_total.unwrap_or(0),
        outgoing: t.outgoing_robux_total.unwrap_or(0),
        pending: t.pending_robux_total.unwrap_or(0),
        by_kind: nonzero(vec![
            (TxKind::PremiumStipend, t.premium_stipends_total),
            (TxKind::CurrencyPurchase, t.currency_purchases_total),
            (TxKind::Sale, t.sales_total),
            (TxKind::CreatorRewardsPayout, t.creator_rewards_payouts_total),
            (TxKind::EngagementPayout, t.premium_payouts_total),
            (
                TxKind::AffiliateSale,
                t.affiliate_sales_total.filter(|a| *a != -1),
            ),
            (TxKind::GroupPayout, t.group_payouts_total),
            (TxKind::GroupEngagementPayout, t.group_premium_payouts_total),
            (TxKind::TradeRobux, t.trade_system_earnings_total),
            (TxKind::CurrencyTransfer, t.incoming_robux_transfer_total),
            (TxKind::CsAdjustment, t.cs_adjustment_total),
            (TxKind::Purchase, t.purchases_total),
            (TxKind::DevEx, t.developer_exchange_total),
            (TxKind::AdSpend, t.ad_spend_total),
            (TxKind::IndividualToGroup, t.individual_to_group_total),
        ]),
    }
}

fn group_totals(t: api::GroupRevenueSummary) -> Totals {
    let by_kind = nonzero(vec![
        (TxKind::Sale, t.item_sale_robux),
        (TxKind::AffiliateSale, t.commission_robux),
        (TxKind::GroupPayout, t.group_payout_robux),
        (TxKind::IndividualToGroup, t.individual_to_group_robux),
        (TxKind::CreatorRewardsPayout, t.creator_rewards_payout_robux),
    ]);
    Totals {
        incoming: by_kind.iter().map(|(_, a)| *a).filter(|a| *a > 0).sum(),
        outgoing: by_kind.iter().map(|(_, a)| *a).filter(|a| *a < 0).sum(),
        pending: t.pending_robux.unwrap_or(0),
        by_kind,
    }
}

// ---------------------------------------------------------------- sync

async fn fetch_page(
    c: &Client,
    account: AccountId,
    scope: Scope,
    kind: TxKind,
    cursor: Option<String>,
) -> Result<(Vec<Txn>, Option<String>), Error> {
    let r = match scope {
        Scope::Me => {
            let p = api::GetUserTransactionsParams {
                transaction_type: kind.api().into(),
                limit: Some(PAGE),
                cursor,
                item_pricing_type: None,
            };
            api::get_user_transactions(c, account.0 as i64, &p).await?
        }
        Scope::Group(g) => {
            let p = api::GetGroupTransactionsParams {
                transaction_type: kind.api().into(),
                limit: Some(PAGE),
                cursor,
            };
            api::get_group_transactions(c, g as i64, &p).await?
        }
    };
    let rows = r
        .data
        .unwrap_or_default()
        .into_iter()
        .filter_map(|x| Txn::from_api(kind, x))
        .collect();
    Ok((rows, r.next_page_cursor.filter(|c| !c.is_empty())))
}

/// [`fetch_page`], waiting out up to [`RATE_WAITS`] rate limits unless `stopped`.
async fn fetch_waiting(
    c: &Client,
    account: AccountId,
    scope: Scope,
    kind: TxKind,
    cursor: Option<String>,
    stopped: impl Fn() -> bool,
) -> Result<(Vec<Txn>, Option<String>), Error> {
    let mut waits = 0;
    loop {
        match fetch_page(c, account, scope, kind, cursor.clone()).await {
            Err(Error::RateLimited { .. }) if waits < RATE_WAITS && !stopped() => waits += 1,
            r => return r,
        }
    }
}

/// Roblox's totals for the last day.
async fn day_totals(c: &Client, account: AccountId, scope: Scope) -> Result<Totals, Error> {
    Ok(match scope {
        Scope::Me => {
            let p = api::GetUserTransactionTotalsParams {
                time_frame: Period::Day.api().into(),
                transaction_type: "summary".into(),
            };
            user_totals(api::get_user_transaction_totals(c, account.0 as i64, &p).await?)
        }
        Scope::Group(g) => {
            group_totals(api::get_group_revenue_summary(c, g as i64, Period::Day.api()).await?)
        }
    })
}

fn kinds_id() -> String {
    "tx_kinds:".into()
}

fn state_id(scope: Scope) -> String {
    format!("ledger_sync:{}", scope.key())
}

impl Inner {
    fn ledger_load(&self, a: AccountId, s: Scope) {
        if self.ledgers.get(a, s).loaded {
            return;
        }
        let rows: Vec<Txn> = self.cache.ledger_all(a, &s.key());
        let st = self
            .cache
            .get::<SyncState>(Kind::Res, a, &state_id(s))
            .map(|c| c.value)
            .unwrap_or_default();
        self.ledgers.edit(a, s, |l| {
            if !l.loaded {
                l.rows = Arc::new(merge(&l.rows, rows));
                l.loaded = true;
                l.synced_at = st.synced_at;
                l.kinds = st.kinds;
            }
        });
    }

    fn ledger_save(&self, a: AccountId, s: Scope, new: &[Txn]) {
        let scope = s.key();
        self.cache.ledger_put(
            a,
            &scope,
            new.iter().map(|r| (r.kind.api(), r.id.as_str(), r.at, r)),
        );
        let l = self.ledgers.get(a, s);
        let st = SyncState {
            synced_at: l.synced_at,
            kinds: l.kinds.clone(),
        };
        self.cache.put(Kind::Res, a, &state_id(s), &st);
    }

    /// Adds rows to the in-memory ledger and saves them.
    fn ledger_add(&self, a: AccountId, s: Scope, new: Vec<Txn>, f: impl FnOnce(&mut Ledger)) {
        self.ledgers.edit(a, s, |l| {
            if !new.is_empty() {
                l.rows = Arc::new(merge(&l.rows, new.clone()));
            }
            f(l);
        });
        self.ledger_save(a, s, &new);
        self.repaint();
    }

    /// The types to read for this scope. Uses the `tx_kinds` copy when it is under an hour
    /// old, unless `fresh`.
    async fn ledger_kinds(
        &self,
        c: &Client,
        a: AccountId,
        s: Scope,
        fresh: bool,
    ) -> Result<Vec<TxKind>, Error> {
        if let Scope::Group(_) = s {
            return Ok(TxKind::GROUP.to_vec());
        }
        if !fresh {
            let key = Key::private(a, "tx_kinds", "");
            if let Some(r) = self.res.snapshot::<Vec<TxKind>>(&key)
                && let Some(k) = r.data.clone().filter(|_| r.age_secs() <= KINDS_AGE)
            {
                return Ok((*k).clone());
            }
            if let Some(k) = self
                .cache
                .get::<Vec<TxKind>>(Kind::Res, a, &kinds_id())
                .filter(|k| k.age_secs() <= KINDS_AGE)
            {
                return Ok(k.value);
            }
        }
        let flags = api::get_user_transaction_types(c, a.0 as i64).await?;
        let kinds: Vec<TxKind> = TxKind::ALL
            .into_iter()
            .filter(|k| flags.get(&k.flag()) == Some(&true))
            .collect();
        self.cache.put(Kind::Res, a, &kinds_id(), &kinds);
        Ok(kinds)
    }

    /// Of `kinds`, the ones a sync needs to read. All of them on a forced or first sync, or
    /// when the last sync is a day old.
    async fn kinds_due(
        &self,
        c: &Client,
        a: AccountId,
        s: Scope,
        kinds: Vec<TxKind>,
        force: bool,
    ) -> Result<Vec<TxKind>, Error> {
        let l = self.ledgers.get(a, s);
        if force || l.synced_at <= 0 || now() - l.synced_at > DAY_SKIP_AGE {
            return Ok(kinds);
        }
        let day = day_totals(c, a, s).await?;
        let started: HashSet<TxKind> = l
            .kinds
            .iter()
            .filter(|k| k.older.is_some() || k.complete)
            .filter_map(|k| k.kind)
            .collect();
        let pending: HashSet<TxKind> = l.rows.iter().filter(|r| r.pending).map(|r| r.kind).collect();
        Ok(kinds_to_read(&kinds, &day, totals_cover(s), &started, &pending))
    }

    /// Newest rows of every type that may have new ones, down to the first settled row we had.
    async fn ledger_sync(self: Arc<Self>, a: AccountId, s: Scope, force: bool) {
        self.ledger_load(a, s);
        let Some(c) = self.client(a) else {
            self.ledgers.edit(a, s, |l| {
                l.syncing = false;
                l.error = Some("This account is not signed in.".into());
            });
            self.repaint();
            return;
        };
        let kinds = match self.ledger_kinds(&c, a, s, force).await {
            Ok(k) => self.kinds_due(&c, a, s, k, force).await,
            Err(e) => Err(e),
        };
        let kinds = match kinds {
            Ok(k) => k,
            Err(e) => {
                self.note_error(a, &e);
                self.ledgers.edit(a, s, |l| {
                    l.syncing = false;
                    l.error = Some(e.user_message());
                });
                self.repaint();
                return;
            }
        };
        let mut error = None;
        'kinds: for kind in kinds {
            let known: HashMap<(TxKind, String), bool> = self
                .ledgers
                .get(a, s)
                .rows
                .iter()
                .filter(|r| r.kind == kind)
                .map(|r| ((r.kind, r.id.clone()), r.pending))
                .collect();
            let started = self
                .ledgers
                .get(a, s)
                .kinds
                .iter()
                .any(|k| k.kind == Some(kind) && (k.older.is_some() || k.complete));
            let mut cursor = None;
            for page in 0..HEAD_PAGES {
                let (rows, next) = match fetch_waiting(&c, a, s, kind, cursor.take(), || false).await {
                    Ok(r) => r,
                    // A community may not have this type; the others still load.
                    Err(Error::Status {
                        status: 400 | 404, ..
                    }) if matches!(s, Scope::Group(_)) => break,
                    Err(e) => {
                        self.note_error(a, &e);
                        error = Some(e.user_message());
                        break 'kinds;
                    }
                };
                if s == Scope::Me && kind == TxKind::Sale && page == 0 {
                    // The newest sales are in; the sale watch needn't ask again yet.
                    self.throttle.lock().insert(sales_watch_id(a), now());
                }
                let stop = reached_known(&known, &rows) || next.is_none();
                let done = next.is_none();
                // First read of a type: keep one page now; older pages come with `load_older`.
                let first = !started && page == 0;
                let older = if first { next.clone() } else { None };
                self.ledger_add(a, s, rows, |l| {
                    if first || done && !started {
                        l.kinds.retain(|k| k.kind != Some(kind));
                        l.kinds.push(KindSync {
                            kind: Some(kind),
                            older,
                            complete: done,
                        });
                    }
                });
                if stop || first {
                    break;
                }
                cursor = next;
            }
        }
        self.ledgers.edit(a, s, |l| {
            l.syncing = false;
            l.error = error;
            l.synced_at = now();
        });
        self.ledger_save(a, s, &[]);
        self.repaint();
    }

    /// Older pages of every type that isn't complete, until done, stopped or rate limited.
    async fn ledger_older(self: Arc<Self>, a: AccountId, s: Scope) {
        self.ledger_load(a, s);
        let Some(c) = self.client(a) else {
            self.ledgers.edit(a, s, |l| l.loading_older = false);
            self.repaint();
            return;
        };
        let todo: Vec<KindSync> = self
            .ledgers
            .get(a, s)
            .kinds
            .into_iter()
            .filter(|k| !k.complete && k.older.is_some())
            .collect();
        let mut error = None;
        'kinds: for k in todo {
            let Some(kind) = k.kind else { continue };
            let mut cursor = k.older.clone();
            let stopped = || self.ledgers.stop.read().contains(&(a, s));
            for _ in 0..OLDER_PAGES {
                if stopped() {
                    break 'kinds;
                }
                let (rows, next) = match fetch_waiting(&c, a, s, kind, cursor.clone(), stopped).await {
                    Ok(r) => r,
                    // Stopped while waiting out a rate limit.
                    Err(Error::RateLimited { .. }) if stopped() => break 'kinds,
                    Err(e) => {
                        self.note_error(a, &e);
                        error = Some(e.user_message());
                        break 'kinds;
                    }
                };
                let done = next.is_none();
                let older = next.clone();
                self.ledger_add(a, s, rows, |l| {
                    if let Some(ks) = l.kinds.iter_mut().find(|x| x.kind == Some(kind)) {
                        ks.older = older;
                        ks.complete = done;
                    }
                });
                if done {
                    break;
                }
                cursor = next;
            }
        }
        self.ledgers.stop.write().remove(&(a, s));
        self.ledgers.edit(a, s, |l| {
            l.loading_older = false;
            if error.is_some() {
                l.error = error;
            }
        });
        self.ledger_save(a, s, &[]);
        self.repaint();
    }
}

fn sales_watch_id(a: AccountId) -> String {
    format!("sales_watch:{}", a.0)
}

/// Sales in a fresh page that the ledger didn't have.
pub fn new_sales<'a>(known: &HashSet<&str>, page: &'a [Txn]) -> Vec<&'a Txn> {
    page.iter()
        .filter(|r| r.kind == TxKind::Sale && !known.contains(r.id.as_str()))
        .collect()
}

/// The toast for new sales: the item and buyer for one, a count and sum for more.
pub fn sale_notice(new: &[&Txn]) -> Option<String> {
    match new {
        [] => None,
        [r] => {
            let item = r.item.as_ref().map(|i| i.name.as_str()).filter(|n| !n.is_empty());
            let buyer = r
                .agent
                .as_ref()
                .map(|a| a.name.as_str())
                .filter(|n| !n.is_empty());
            let mut s = format!("Sold {}", item.unwrap_or("an item"));
            if let Some(b) = buyer {
                s += &format!(" to {b}");
            }
            s += &format!(" for {} Robux.", r.amount);
            Some(s)
        }
        _ => {
            let sum: i64 = new.iter().map(|r| r.amount).sum();
            Some(format!("{} new sales for {sum} Robux.", new.len()))
        }
    }
}

impl Inner {
    /// Checks the newest sales page and toasts any sale the ledger didn't have. Only reads and
    /// notifies. The first check of an account only fills the ledger, so nothing old shows up
    /// as new.
    pub(crate) async fn watch_sales(&self, c: &Client, a: AccountId) {
        if !self.settings.read().notify.sales || self.ledgers.get(a, Scope::Me).syncing {
            return;
        }
        if !self.due(&sales_watch_id(a), SALES_WATCH) {
            return;
        }
        self.ledger_load(a, Scope::Me);
        let (rows, next) = match fetch_page(c, a, Scope::Me, TxKind::Sale, None).await {
            Ok(r) => r,
            Err(e) => return self.note_error(a, &e),
        };
        let l = self.ledgers.get(a, Scope::Me);
        let started = l.kinds.iter().any(|k| k.kind == Some(TxKind::Sale));
        let known: HashSet<&str> = l
            .rows
            .iter()
            .filter(|r| r.kind == TxKind::Sale)
            .map(|r| r.id.as_str())
            .collect();
        let notice = if started {
            sale_notice(&new_sales(&known, &rows))
        } else {
            None
        };
        drop(known);
        // `merge` keeps one copy of each row and takes the newer pending state.
        self.ledger_add(a, Scope::Me, rows, |l| {
            if !started {
                l.kinds.push(KindSync {
                    kind: Some(TxKind::Sale),
                    complete: next.is_none(),
                    older: next,
                });
            }
        });
        if let Some(n) = notice {
            self.toast(Some(a), n.clone(), false);
            self.note(a, "account", "Sale", n.clone(), None);
            self.alert(a, "sales", "Sale", n);
        }
    }
}

impl Core {
    /// The ledger of `scope` as `account` sees it. Loads it from disk and syncs it when it is
    /// older than a few minutes (or always with `force`). Never blocks.
    pub fn ledger(&self, account: AccountId, scope: Scope, force: bool) -> Ledger {
        let l = self.inner.ledgers.get(account, scope);
        let stale = force || now() - l.synced_at > SYNC_AGE;
        if l.syncing || !stale && l.loaded {
            return l;
        }
        if !stale {
            // Saved copy only.
            let inner = self.inner.clone();
            self.inner.rt.spawn_blocking(move || {
                inner.ledger_load(account, scope);
                inner.repaint();
            });
            return l;
        }
        self.inner.ledgers.edit(account, scope, |l| {
            l.syncing = true;
            l.error = None;
        });
        let inner = self.inner.clone();
        self.inner.rt.spawn(inner.ledger_sync(account, scope, force));
        self.inner.ledgers.get(account, scope)
    }

    /// Fetch older history of every type. Runs until done, rate limited or stopped.
    pub fn ledger_load_older(&self, account: AccountId, scope: Scope) -> Result<(), String> {
        let l = self.inner.ledgers.get(account, scope);
        if l.loading_older {
            return Ok(());
        }
        if l.loaded && !l.has_older() {
            return Err("All history is already here.".into());
        }
        self.inner.ledgers.stop.write().remove(&(account, scope));
        self.inner
            .ledgers
            .edit(account, scope, |l| l.loading_older = true);
        let inner = self.inner.clone();
        self.inner.rt.spawn(inner.ledger_older(account, scope));
        self.inner.repaint();
        Ok(())
    }

    /// Stop fetching older history after the page in flight.
    pub fn ledger_stop(&self, account: AccountId, scope: Scope) {
        if self.inner.ledgers.get(account, scope).loading_older {
            self.inner.ledgers.stop.write().insert((account, scope));
        }
    }

    /// Roblox's totals for the last day, week, month or year.
    pub fn tx_totals(&self, account: AccountId, scope: Scope, period: Period) -> Res<Totals> {
        self.inner.res(
            Key::private(account, "tx_totals", format!("{}:{}", scope.key(), period.key())),
            TOTALS_AGE,
            move |f| async move {
                match scope {
                    Scope::Me => {
                        let p = api::GetUserTransactionTotalsParams {
                            time_frame: period.api().into(),
                            transaction_type: "summary".into(),
                        };
                        Ok(user_totals(
                            api::get_user_transaction_totals(&f.client, account.0 as i64, &p).await?,
                        ))
                    }
                    Scope::Group(g) => Ok(group_totals(
                        api::get_group_revenue_summary(&f.client, g as i64, period.api()).await?,
                    )),
                }
            },
        )
    }

    /// The types this account has any transactions of.
    pub fn tx_kinds(&self, account: AccountId) -> Res<Vec<TxKind>> {
        self.inner.res(
            Key::private(account, "tx_kinds", ""),
            KINDS_AGE,
            move |f| async move { f.inner.ledger_kinds(&f.client, account, Scope::Me, true).await },
        )
    }

    /// Save rows as CSV in the downloads folder. Returns the file's path.
    pub fn save_ledger_csv(&self, name: &str, rows: &[Txn]) -> Result<String, String> {
        let dir = self.inner.paths.downloads.clone();
        let path = crate::template::free_path(&dir, name, "csv");
        std::fs::create_dir_all(&dir)
            .and_then(|()| std::fs::write(&path, csv(rows)))
            .map_err(|e| format!("Can't save to {}: {e}", dir.display()))?;
        Ok(path.display().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(kind: TxKind, id: &str, at: i64, amount: i64) -> Txn {
        Txn {
            id: id.into(),
            kind,
            at,
            created: crate::fmt::iso_utc(at),
            pending: false,
            amount,
            agent: None,
            item: None,
        }
    }

    #[test]
    fn kinds_round_trip_and_flags() {
        for k in TxKind::ALL {
            assert_eq!(TxKind::from_key(k.key()), Some(k));
            assert_eq!(TxKind::from_key(k.api()), Some(k));
        }
        assert_eq!(TxKind::CsAdjustment.flag(), "HasCSAdjustment");
        assert_eq!(TxKind::CurrencyTransfer.flag(), "HasTransfer");
        assert_eq!(TxKind::Purchase.flag(), "HasPurchase");
        assert_eq!(
            serde_json::from_str::<TxKind>("\"Brand new\"").unwrap(),
            TxKind::Unknown
        );
        assert_eq!(Scope::from_key("group:7"), Some(Scope::Group(7)));
        assert_eq!(Scope::from_key("me"), Some(Scope::Me));
        assert_eq!(Period::from_key("Year"), Some(Period::Year));
    }

    #[test]
    fn merge_dedupes_updates_and_sorts_newest_first() {
        let mut a = row(TxKind::Sale, "x", 100, 10);
        a.pending = true;
        let rows = vec![a, row(TxKind::Sale, "y", 50, 5)];
        let mut a2 = row(TxKind::Sale, "x", 100, 10);
        a2.pending = false;
        // Same id in another type is another row.
        let out = merge(&rows, vec![a2, row(TxKind::Purchase, "x", 200, -3)]);
        assert_eq!(out.len(), 3);
        assert_eq!(out[0].kind, TxKind::Purchase);
        assert!(!out[1].pending, "a released sale replaces its pending copy");
        assert_eq!(out[2].id, "y");
    }

    #[test]
    fn new_sales_skip_known_rows_and_other_types() {
        let mut sold = row(TxKind::Sale, "s2", 300, 35);
        sold.item = Some(TxItem {
            id: 5,
            name: "Plain Shirt".into(),
            kind: "Asset".into(),
            place: None,
        });
        sold.agent = Some(Agent {
            id: 2,
            group: false,
            name: "user1".into(),
        });
        let page = vec![
            sold,
            row(TxKind::Sale, "s1", 200, 10),
            row(TxKind::Purchase, "p", 250, -5),
        ];
        let known: HashSet<&str> = ["s1"].into();
        let new = new_sales(&known, &page);
        assert_eq!(new.len(), 1);
        assert_eq!(
            sale_notice(&new).unwrap(),
            "Sold Plain Shirt to user1 for 35 Robux."
        );
        let none: HashSet<&str> = HashSet::new();
        assert_eq!(
            sale_notice(&new_sales(&none, &page)).unwrap(),
            "2 new sales for 45 Robux."
        );
        assert_eq!(sale_notice(&new_sales(&["s1", "s2"].into(), &page)), None);
        assert_eq!(
            sale_notice(&[&row(TxKind::Sale, "s3", 1, 7)]).unwrap(),
            "Sold an item for 7 Robux."
        );
    }

    #[test]
    fn sync_stops_only_at_a_settled_known_row() {
        let mut known = HashMap::new();
        known.insert((TxKind::Sale, "p".to_string()), true);
        known.insert((TxKind::Sale, "s".to_string()), false);
        assert!(!reached_known(&known, &[row(TxKind::Sale, "new", 3, 1)]));
        assert!(
            !reached_known(&known, &[row(TxKind::Sale, "p", 2, 1)]),
            "pending rows may change"
        );
        assert!(reached_known(
            &known,
            &[row(TxKind::Sale, "new", 3, 1), row(TxKind::Sale, "s", 1, 1)]
        ));
    }

    #[test]
    fn a_recent_sync_reads_only_types_that_moved_today() {
        use TxKind::*;
        let kinds = [
            PremiumStipend,
            Sale,
            AffiliateSale,
            Purchase,
            CurrencyPurchase,
            AdSpend,
        ];
        let started: HashSet<TxKind> = [PremiumStipend, Sale, AffiliateSale, Purchase, AdSpend].into();
        let mut day = Totals {
            by_kind: vec![(Purchase, -40)],
            ..Default::default()
        };
        let none = HashSet::new();
        let read = |day: &Totals, covered: &[TxKind], pending: &HashSet<TxKind>| {
            kinds_to_read(&kinds, day, covered, &started, pending)
        };
        assert_eq!(
            read(&day, &TxKind::ALL, &none),
            [Purchase, CurrencyPurchase],
            "moved today, or never read"
        );
        assert_eq!(
            read(&day, totals_cover(Scope::Group(1)), &none),
            [PremiumStipend, Purchase, CurrencyPurchase, AdSpend],
            "types the summary doesn't report are always read"
        );
        assert_eq!(
            read(&day, &TxKind::ALL, &[Sale].into()),
            [Sale, Purchase, CurrencyPurchase],
            "held pending rows may have settled"
        );
        day.pending = 35;
        assert_eq!(
            read(&day, &TxKind::ALL, &none),
            [Sale, AffiliateSale, Purchase, CurrencyPurchase]
        );
    }

    #[test]
    fn filters_by_type_range_text_amount_and_pending() {
        let mut r = row(TxKind::Purchase, "a", 1_000, -75);
        r.item = Some(TxItem {
            id: 42,
            name: "Sleepy Doggy".into(),
            kind: "Bundle".into(),
            place: None,
        });
        r.agent = Some(Agent {
            id: 9,
            group: true,
            name: "Nostalqiaa".into(),
        });
        let f = |f: Filter| f.matches(&r);
        assert!(f(Filter::default()));
        assert!(f(Filter {
            kinds: vec![TxKind::Purchase],
            ..Default::default()
        }));
        assert!(!f(Filter {
            kinds: vec![TxKind::Sale],
            ..Default::default()
        }));
        assert!(f(Filter {
            from: Some(1_000),
            to: Some(1_001),
            ..Default::default()
        }));
        assert!(!f(Filter {
            to: Some(1_000),
            ..Default::default()
        }));
        assert!(f(Filter {
            text: "doggy".into(),
            ..Default::default()
        }));
        assert!(f(Filter {
            text: "nostal".into(),
            ..Default::default()
        }));
        assert!(f(Filter {
            text: "42".into(),
            ..Default::default()
        }));
        assert!(!f(Filter {
            text: "4".into(),
            ..Default::default()
        }));
        assert!(f(Filter {
            min_amount: Some(75),
            ..Default::default()
        }));
        assert!(!f(Filter {
            min_amount: Some(76),
            ..Default::default()
        }));
        assert!(!f(Filter {
            pending_only: true,
            ..Default::default()
        }));
    }

    #[test]
    fn sums_keep_pending_apart() {
        let mut p = row(TxKind::Sale, "p", 3, 70);
        p.pending = true;
        let rows = [
            row(TxKind::Sale, "s", 4, 140),
            p,
            row(TxKind::Purchase, "b", 2, -100),
            row(TxKind::PremiumStipend, "st", 1, 450),
        ];
        let s = sums(&rows);
        assert_eq!((s.count, s.incoming, s.outgoing, s.pending), (4, 590, -100, 70));
        assert_eq!(s.net(), 490);
        assert_eq!(
            s.by_kind,
            vec![
                (TxKind::PremiumStipend, 450, 1),
                (TxKind::Sale, 210, 2),
                (TxKind::Purchase, -100, 1)
            ]
        );
    }

    #[test]
    fn months_experiences_items_and_buyers() {
        let day = 86_400;
        let jan = parse_iso("2026-01-15T00:00:00Z").unwrap();
        let feb = jan + 31 * day;
        let mut a = row(TxKind::Purchase, "a", jan, -10);
        a.item = Some(TxItem {
            id: 1,
            name: "Revive".into(),
            kind: "DeveloperProduct".into(),
            place: Some(TxPlace {
                place_id: 5,
                universe_id: 50,
                name: "Obby".into(),
            }),
        });
        let mut b = a.clone();
        b.id = "b".into();
        b.at = feb;
        b.amount = -15;
        let sale = |id: &str, item: u64, buyer: u64, amount: i64| {
            let mut s = row(TxKind::Sale, id, feb, amount);
            s.item = Some(TxItem {
                id: item,
                name: format!("item{item}"),
                kind: "Asset".into(),
                place: None,
            });
            s.agent = Some(Agent {
                id: buyer,
                group: false,
                name: format!("user{buyer}"),
            });
            s
        };
        let rows = [
            a,
            b,
            sale("s1", 7, 100, 35),
            sale("s2", 7, 101, 35),
            sale("s3", 8, 100, 50),
        ];
        assert_eq!(
            by_month(&rows, 0),
            vec![("2026-01".to_string(), 0, -10), ("2026-02".to_string(), 120, -15)]
        );
        let e = spend_by_experience(&rows, 5);
        assert_eq!((e[0].id, e[0].amount, e[0].count), (50, -25, 2));
        let i = top_items(&rows, 5);
        assert_eq!((i[0].id, i[0].amount, i[0].count), (7, 70, 2));
        let u = top_buyers(&rows, 1);
        assert_eq!((u.len(), u[0].id, u[0].amount), (1, 100, 85));
    }

    #[test]
    fn csv_quotes_fields_and_keeps_one_line_per_row() {
        let mut r = row(TxKind::Purchase, "h+/=", 0, -95);
        r.item = Some(TxItem {
            id: 3,
            name: ",, plaid \"bat\" beret".into(),
            kind: "Asset".into(),
            place: None,
        });
        let out = csv(&[r]);
        let lines: Vec<&str> = out.lines().collect();
        assert_eq!(lines.len(), 2);
        assert!(lines[1].contains("purchases,-95,false,\",, plaid \"\"bat\"\" beret\",Asset,3"));
    }

    #[test]
    fn roblox_totals_map_to_types() {
        let t = user_totals(api::TransactionTotals {
            purchases_total: Some(-479),
            currency_purchases_total: Some(500),
            incoming_robux_total: Some(500),
            outgoing_robux_total: Some(-479),
            affiliate_sales_total: Some(-1),
            ..Default::default()
        });
        assert_eq!((t.incoming, t.outgoing, t.pending), (500, -479, 0));
        assert_eq!(
            t.by_kind,
            vec![(TxKind::CurrencyPurchase, 500), (TxKind::Purchase, -479)],
            "-1 means no commissions, not a loss"
        );
        let g = group_totals(api::GroupRevenueSummary {
            item_sale_robux: Some(700),
            pending_robux: Some(300),
            group_payout_robux: Some(-100),
            ..Default::default()
        });
        assert_eq!((g.incoming, g.outgoing, g.pending), (700, -100, 300));
    }
}

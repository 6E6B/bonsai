//! Trades: inbound, outbound, completed and inactive lists, trade details, the trade calculator,
//! and send / counter / accept / decline. Every write is one click on a confirmed dialog;
//! nothing here trades on its own.

use bonsai_api::AccountId;
use bonsai_api::generated::trades;
use serde::{Deserialize, Serialize};

use crate::Core;
use crate::market::{ItemKind, ItemRef};
use crate::res::{Key, Page, Res};
use crate::store::UserInfo;
use crate::values::ValueTable;

const LIST_AGE: i64 = 60;
const TRADE_AGE: i64 = 60;
const META_AGE: i64 = 86_400;
const ELIGIBLE_AGE: i64 = 600;
const ITEMS_AGE: i64 = 120;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TradeBox {
    #[default]
    Inbound,
    Outbound,
    Completed,
    Inactive,
}

impl TradeBox {
    pub const ALL: [TradeBox; 4] = [
        TradeBox::Inbound,
        TradeBox::Outbound,
        TradeBox::Completed,
        TradeBox::Inactive,
    ];
    /// `tradeStatusType` in the trades API. Must be the name: a number is read as a trade id.
    fn api(self) -> &'static str {
        match self {
            TradeBox::Inbound => "Inbound",
            TradeBox::Outbound => "Outbound",
            TradeBox::Completed => "Completed",
            TradeBox::Inactive => "Inactive",
        }
    }
    pub fn key(self) -> &'static str {
        match self {
            TradeBox::Inbound => "inbound",
            TradeBox::Outbound => "outbound",
            TradeBox::Completed => "completed",
            TradeBox::Inactive => "inactive",
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            TradeBox::Inbound => "Inbound",
            TradeBox::Outbound => "Outbound",
            TradeBox::Completed => "Completed",
            TradeBox::Inactive => "Inactive",
        }
    }
    pub fn from_key(k: &str) -> Option<TradeBox> {
        TradeBox::ALL.into_iter().find(|b| b.key() == k)
    }
    /// Trades you can still act on.
    pub fn open(self) -> bool {
        matches!(self, TradeBox::Inbound | TradeBox::Outbound)
    }
}

/// A row in a trade list.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TradeRow {
    pub id: u64,
    pub partner: u64,
    pub partner_name: String,
    pub partner_display: String,
    pub created: String,
    pub expires: String,
    pub status: String,
    pub active: bool,
}

/// One copy of an item in a trade or a tradable inventory.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TradeItem {
    /// What the trades API sends back to name this copy.
    pub instance_id: String,
    pub item: ItemRef,
    pub name: String,
    pub serial: Option<i64>,
    pub rap: Option<i64>,
    pub original_price: Option<i64>,
    pub stock: Option<i64>,
    pub on_hold: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TradeSide {
    pub user_id: u64,
    pub robux: i64,
    pub items: Vec<TradeItem>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Trade {
    pub id: u64,
    pub status: String,
    /// What the viewing account gives.
    pub mine: TradeSide,
    /// What the viewing account gets.
    pub theirs: TradeSide,
}

/// Limits from the trade system.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct TradeRules {
    pub max_items: usize,
    /// Share of added Robux that the receiver loses to the fee (0.3).
    pub robux_fee: f64,
    /// Robux on one side can be at most this share of that side's item RAP.
    pub max_robux_share: f64,
}

impl Default for TradeRules {
    fn default() -> Self {
        TradeRules {
            max_items: 4,
            robux_fee: 0.3,
            max_robux_share: 0.5,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Eligibility {
    pub can_trade: bool,
    /// Why not, in plain words.
    pub reason: String,
}

/// A trade being put together.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Draft {
    pub partner: u64,
    pub give: Vec<TradeItem>,
    pub get: Vec<TradeItem>,
    pub give_robux: i64,
    pub get_robux: i64,
    /// Set when this draft counters a trade.
    pub counter_of: Option<u64>,
}

impl Draft {
    pub fn is_empty(&self) -> bool {
        self.give.is_empty() && self.get.is_empty()
    }
}

// ---------------------------------------------------------------- calculator

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SideEval {
    pub rap: i64,
    /// Item worth by value (RAP where there is no value). `None` without values.
    pub value: Option<i64>,
    /// Robux as it counts for this side: in full when you pay it, after the fee when you get it.
    pub robux: i64,
}

impl SideEval {
    pub fn total_rap(&self) -> i64 {
        self.rap + self.robux
    }
    pub fn total_value(&self) -> Option<i64> {
        self.value.map(|v| v + self.robux)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Warning {
    /// You get much less than you give (percent of what you give).
    Lopsided(i64),
    /// An item you'd get has a RAP Rolimons marks as projected (inflated).
    Projected(String),
    /// An item you give is on hold and can't be traded yet.
    OnHold(String),
    TooManyItems,
    /// Robux above the share of item RAP the trade system allows.
    TooMuchRobux,
    /// Nothing on one side.
    OneSided,
}

impl Warning {
    pub fn text(&self) -> String {
        match self {
            Warning::Lopsided(p) => format!("You lose {p}% of what you give."),
            Warning::Projected(n) => format!("{n} has a projected RAP. It may be worth less."),
            Warning::OnHold(n) => format!("{n} is on hold and can't be traded yet."),
            Warning::TooManyItems => "Too many items on one side.".into(),
            Warning::TooMuchRobux => "Too much Robux for the items on that side.".into(),
            Warning::OneSided => "One side of the trade is empty.".into(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Eval {
    pub give: SideEval,
    pub get: SideEval,
    pub warnings: Vec<Warning>,
}

impl Eval {
    /// What you gain (negative: lose), by value when there are values, else by RAP.
    pub fn net(&self) -> i64 {
        match (self.get.total_value(), self.give.total_value()) {
            (Some(g), Some(v)) => g - v,
            _ => self.get.total_rap() - self.give.total_rap(),
        }
    }
    pub fn net_rap(&self) -> i64 {
        self.get.total_rap() - self.give.total_rap()
    }
    /// Net as a percent of what you give.
    pub fn net_pct(&self) -> Option<i64> {
        let base = self.give.total_value().unwrap_or(self.give.total_rap());
        (base > 0).then(|| (self.net() * 100) / base)
    }
}

/// Losses beyond this share of what you give get a warning.
const LOPSIDED_PCT: i64 = 25;

/// Value both sides of a trade from the viewer's point of view.
pub fn evaluate(
    give: &[TradeItem],
    give_robux: i64,
    get: &[TradeItem],
    get_robux: i64,
    values: Option<&ValueTable>,
    rules: &TradeRules,
) -> Eval {
    let side = |items: &[TradeItem], robux: i64| SideEval {
        rap: items.iter().filter_map(|i| i.rap).sum(),
        value: values.map(|v| {
            items
                .iter()
                .map(|i| v.get(i.item.id).and_then(|x| x.worth()).or(i.rap).unwrap_or(0))
                .sum()
        }),
        robux,
    };
    let received = (get_robux as f64 * (1.0 - rules.robux_fee)).floor() as i64;
    let e = Eval {
        give: side(give, give_robux),
        get: side(get, received),
        warnings: Vec::new(),
    };
    let mut w = Vec::new();
    if give.is_empty() && give_robux == 0 || get.is_empty() && get_robux == 0 {
        w.push(Warning::OneSided);
    }
    if let Some(p) = e.net_pct()
        && p <= -LOPSIDED_PCT
    {
        w.push(Warning::Lopsided(-p));
    }
    if let Some(v) = values {
        for i in get {
            if v.get(i.item.id).is_some_and(|x| x.projected) {
                w.push(Warning::Projected(i.name.clone()));
            }
        }
    }
    for i in give.iter().filter(|i| i.on_hold) {
        w.push(Warning::OnHold(i.name.clone()));
    }
    if give.len() > rules.max_items || get.len() > rules.max_items {
        w.push(Warning::TooManyItems);
    }
    let cap = |items: &[TradeItem], robux: i64| {
        let rap: i64 = items.iter().filter_map(|i| i.rap).sum();
        robux > 0 && robux as f64 > rap as f64 * rules.max_robux_share
    };
    if cap(give, give_robux) || cap(get, get_robux) {
        w.push(Warning::TooMuchRobux);
    }
    Eval { warnings: w, ..e }
}

// ---------------------------------------------------------------- reads and writes

fn status_text(s: Option<trades::v1::TradeDetailResponseStatus>) -> String {
    use trades::v1::TradeDetailResponseStatus as S;
    match s {
        Some(S::Open) => "Open",
        Some(S::Pending) => "Pending",
        Some(S::Completed) => "Completed",
        Some(S::Expired) => "Expired",
        Some(S::Declined) => "Declined",
        Some(S::RejectedDueToError) => "Failed",
        Some(S::Countered) => "Countered",
        Some(S::Processing) => "Processing",
        Some(S::InterventionRequired) => "Needs review",
        Some(S::TwoStepVerificationRequired) => "Needs 2-step verification",
        _ => "",
    }
    .into()
}

fn item_of(
    x: trades::v2::TradableItemInstance,
    parent: Option<&trades::v2::ItemTarget>,
) -> Option<TradeItem> {
    use trades::v2::ItemTargetItemType as T;
    let target = x.item_target.as_ref().or(parent)?;
    let id = target.target_id.as_deref()?.parse::<u64>().ok()?;
    let kind = match target.item_type {
        Some(T::Bundle) => ItemKind::Bundle,
        _ => ItemKind::Asset,
    };
    Some(TradeItem {
        instance_id: x.collectible_item_instance_id.clone().unwrap_or_default(),
        item: ItemRef { kind, id },
        name: x.item_name.unwrap_or_default(),
        serial: x.serial_number.filter(|s| *s > 0),
        rap: x.recent_average_price.filter(|p| *p > 0),
        original_price: x.original_price.filter(|p| *p > 0),
        stock: x.asset_stock.filter(|s| *s > 0),
        on_hold: x.is_on_hold.unwrap_or(false),
    })
}

fn side_of(o: Option<trades::v2::TradeOffer>) -> TradeSide {
    let o = o.unwrap_or_default();
    TradeSide {
        user_id: o.user.and_then(|u| u.id).unwrap_or(0).max(0) as u64,
        robux: o.robux.unwrap_or(0),
        items: o
            .items
            .unwrap_or_default()
            .into_iter()
            .filter_map(|i| item_of(i, None))
            .collect(),
    }
}

fn eligibility_v2(r: trades::v2::CanTradeResponse) -> Eligibility {
    use trades::v2::CanTradeResponseTradeEligibility as E;
    let can = r.can_trade.unwrap_or(false);
    let reason = match r.trade_eligibility {
        _ if can => "",
        Some(E::IneligibleMissingPremiumMembership) => "Trading needs Premium.",
        Some(E::IneligibleTradeSystemDisabled) => "Trading is turned off right now.",
        Some(E::IneligibleFreeTradesLimitReached) => "You've used all your free trades for now.",
        Some(E::IneligibleAgeCheckRequired) => "Roblox needs an age check before you can trade.",
        Some(E::IneligibleLegalOrRegulatoryRestrictions) => "Trading isn't available where you are.",
        _ => "This account can't trade.",
    };
    Eligibility {
        can_trade: can,
        reason: reason.into(),
    }
}

impl Core {
    pub fn trades(&self, account: AccountId, b: TradeBox, more: bool) -> Res<Page<TradeRow>> {
        self.inner.paged(
            Key::private(account, "trades", b.key()),
            LIST_AGE,
            more,
            move |f, cursor| async move {
                let p = trades::v1::GetTradesByTradeStatusTypeParams {
                    limit: Some(25),
                    cursor,
                    sort_order: Some(trades::v1::GetTradesByTradeStatusTypeSortOrder::Desc),
                };
                let r = trades::v1::get_trades_by_trade_status_type(&f.client, b.api(), &p).await?;
                let rows: Vec<TradeRow> = r
                    .data
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|t| {
                        let u = t.user.unwrap_or_default();
                        Some(TradeRow {
                            id: t.id? as u64,
                            partner: u.id.unwrap_or(0).max(0) as u64,
                            partner_name: u.name.unwrap_or_default(),
                            partner_display: u.display_name.unwrap_or_default(),
                            created: t.created.unwrap_or_default(),
                            expires: t.expiration.unwrap_or_default(),
                            status: status_text(t.status),
                            active: t.is_active.unwrap_or(false),
                        })
                    })
                    .collect();
                f.note_users(rows.iter().filter(|r| r.partner > 0).map(|r| UserInfo {
                    id: r.partner,
                    name: r.partner_name.clone(),
                    display_name: r.partner_display.clone(),
                    verified: false,
                }));
                Ok(Page {
                    items: rows,
                    next: r.next_page_cursor.filter(|c| !c.is_empty()),
                })
            },
        )
    }

    /// One trade, split into what the account gives and gets.
    pub fn trade(&self, account: AccountId, trade_id: u64) -> Res<Trade> {
        self.inner.res(
            Key::private(account, "trade", trade_id),
            TRADE_AGE,
            move |f| async move {
                let r = trades::v2::get_trades_by_trade_id(&f.client, trade_id as i64).await?;
                let a = side_of(r.participant_a_offer);
                let b = side_of(r.participant_b_offer);
                let (mine, theirs) = if b.user_id == account.0 { (b, a) } else { (a, b) };
                use trades::v2::TradeDetailsResponseStatus as S2;
                let status = r
                    .status
                    .and_then(|s| (s != S2::UnknownValue).then(|| s.name().unwrap_or("").to_string()))
                    .unwrap_or_default();
                Ok(Trade {
                    id: trade_id,
                    status,
                    mine,
                    theirs,
                })
            },
        )
    }

    pub fn trade_rules(&self, account: AccountId) -> Res<TradeRules> {
        self.inner.res(
            Key::private(account, "trade_rules", ""),
            META_AGE,
            |f| async move {
                let m = trades::v1::get_trades_metadata(&f.client).await?;
                let d = TradeRules::default();
                Ok(TradeRules {
                    max_items: m
                        .max_items_per_side
                        .map(|n| n.max(1) as usize)
                        .unwrap_or(d.max_items),
                    robux_fee: m.trade_system_robux_fee.unwrap_or(d.robux_fee),
                    max_robux_share: m.trade_system_max_robux_percent.unwrap_or(d.max_robux_share),
                })
            },
        )
    }

    /// Whether this account can trade at all.
    pub fn can_trade(&self, account: AccountId) -> Res<Eligibility> {
        self.inner.res(
            Key::private(account, "can_trade", ""),
            ELIGIBLE_AGE,
            |f| async move {
                Ok(eligibility_v2(
                    trades::v2::get_users_me_can_trade(&f.client).await?,
                ))
            },
        )
    }

    /// Whether this account can trade with `user_id`.
    pub fn can_trade_with(&self, account: AccountId, user_id: u64) -> Res<Eligibility> {
        self.inner.res(
            Key::private(account, "can_trade_with", user_id),
            ELIGIBLE_AGE,
            move |f| async move {
                use trades::v2::CanTradeWithResponseMutualTradeEligibility as M;
                let r = trades::v2::get_users_by_user_id_can_trade_with(&f.client, user_id as i64).await?;
                let can = r.can_trade.unwrap_or(false);
                let reason = match r.mutual_trade_eligibility {
                    _ if can => "",
                    Some(M::CannotTradeWithSelf) => "You can't trade with yourself.",
                    Some(M::TargetUserIneligible) => "They can't trade right now.",
                    Some(M::CallingUserIneligible) => "This account can't trade.",
                    Some(M::CallingUserPrivacySettingsRestricted) => {
                        "Your trade privacy settings don't allow this. Change them in Settings on the website."
                    }
                    Some(M::CallingUserAgeCheckRequired) => "Roblox needs an age check before you can trade.",
                    _ => "You can't trade with them.",
                };
                Ok(Eligibility {
                    can_trade: can,
                    reason: reason.into(),
                })
            },
        )
    }

    /// Items `user_id` can trade, one row per copy, as `account` sees them.
    pub fn tradable_items(&self, account: AccountId, user_id: u64, more: bool) -> Res<Page<TradeItem>> {
        self.inner.paged(
            Key::private(account, "tradable", user_id),
            ITEMS_AGE,
            more,
            move |f, cursor| async move {
                let p = trades::v2::GetUsersByUserIdTradableItemsParams {
                    limit: Some(50),
                    cursor,
                    ..Default::default()
                };
                let r =
                    trades::v2::get_users_by_user_id_tradable_items(&f.client, user_id as i64, &p).await?;
                let mut items = Vec::new();
                for t in r.items.unwrap_or_default() {
                    let parent = t.item_target.clone();
                    let name = t.item_name.clone();
                    for mut i in t.instances.unwrap_or_default() {
                        if i.item_name.is_none() {
                            i.item_name = name.clone();
                        }
                        if i.recent_average_price.is_none() {
                            i.recent_average_price = t.recent_average_price;
                        }
                        if let Some(x) = item_of(i, parent.as_ref()) {
                            items.push(x);
                        }
                    }
                }
                Ok(Page {
                    items,
                    next: r.next_page_cursor.filter(|c| !c.is_empty()),
                })
            },
        )
    }

    // ---- user actions (one confirmed click each) ----

    pub fn accept_trade(&self, account: AccountId, trade_id: u64) {
        self.drop_trade(account, trade_id);
        self.inner.mutate(
            account,
            "accept the trade",
            "Trade accepted",
            &["trades", "trade", "tradable", "collectibles"],
            move |f| async move {
                trades::v1::post_trades_by_trade_id_accept(&f.client, trade_id as i64).await?;
                f.inner.refresh_unread(account).await;
                Ok(())
            },
        );
    }

    pub fn decline_trade(&self, account: AccountId, trade_id: u64) {
        self.drop_trade(account, trade_id);
        self.inner.mutate(
            account,
            "decline the trade",
            "Trade declined",
            &["trades", "trade"],
            move |f| async move {
                trades::v1::post_trades_by_trade_id_decline(&f.client, trade_id as i64).await?;
                f.inner.refresh_unread(account).await;
                Ok(())
            },
        );
    }

    /// Send a new trade, or a counter when the draft says so.
    pub fn send_trade(&self, account: AccountId, d: Draft) {
        if let Some(id) = d.counter_of {
            self.drop_trade(account, id);
        }
        let done = if d.counter_of.is_some() {
            "Counter offer sent"
        } else {
            "Trade sent"
        };
        self.inner.mutate(
            account,
            "send the trade",
            done,
            &["trades", "trade"],
            move |f| async move {
                let offer = |user: u64, items: &[TradeItem], robux: i64| trades::v2::TradeOfferRequest {
                    user_id: Some(user as i64),
                    robux: Some(robux),
                    collectible_item_instance_ids: Some(
                        items.iter().map(|i| i.instance_id.clone()).collect(),
                    ),
                };
                let body = trades::v2::TradeRequest {
                    sender_offer: Some(offer(account.0, &d.give, d.give_robux)),
                    recipient_offer: Some(offer(d.partner, &d.get, d.get_robux)),
                };
                match d.counter_of {
                    Some(id) => {
                        trades::v2::post_trades_by_trade_id_counter(&f.client, id as i64, &body).await?;
                    }
                    None => {
                        trades::v2::post_trades_send(&f.client, &body).await?;
                    }
                }
                Ok(())
            },
        );
    }

    // ---- the composer's draft (local until sent) ----

    pub fn trade_draft(&self, account: AccountId) -> Option<Draft> {
        self.store()
            .account_data(account)
            .and_then(|d| d.trade_draft.clone())
    }

    fn set_trade_draft(&self, account: AccountId, d: Option<Draft>) {
        self.inner
            .store
            .write()
            .data
            .entry(account)
            .or_default()
            .trade_draft = d;
        self.inner.repaint();
    }

    /// Start an empty trade with `partner`, replacing any draft.
    pub fn start_trade(&self, account: AccountId, partner: u64) {
        self.set_trade_draft(
            account,
            Some(Draft {
                partner,
                ..Draft::default()
            }),
        );
    }

    /// Start a counter offer holding the trade's current items.
    pub fn start_counter(&self, account: AccountId, trade_id: u64) -> Result<(), String> {
        let t = self
            .trade(account, trade_id)
            .data
            .ok_or_else(|| "Still loading the trade.".to_owned())?;
        self.set_trade_draft(
            account,
            Some(Draft {
                partner: t.theirs.user_id,
                give: t.mine.items.clone(),
                get: t.theirs.items.clone(),
                give_robux: t.mine.robux,
                get_robux: t.theirs.robux,
                counter_of: Some(trade_id),
            }),
        );
        Ok(())
    }

    pub fn discard_trade_draft(&self, account: AccountId) {
        self.set_trade_draft(account, None);
    }

    /// Change the draft. Does nothing without one.
    pub fn edit_trade_draft(&self, account: AccountId, f: impl FnOnce(&mut Draft)) {
        let changed = {
            let mut s = self.inner.store.write();
            match s.data.get_mut(&account).and_then(|d| d.trade_draft.as_mut()) {
                Some(d) => {
                    f(d);
                    true
                }
                None => false,
            }
        };
        if changed {
            self.inner.repaint();
        }
    }

    /// Add a copy to one side of the draft, or take it off. The copy comes from that side's
    /// loaded tradable items.
    pub fn toggle_draft_item(&self, account: AccountId, give: bool, instance_id: &str) -> Result<(), String> {
        let d = self
            .trade_draft(account)
            .ok_or_else(|| "Start a trade first.".to_owned())?;
        let side = if give { &d.give } else { &d.get };
        if side.iter().any(|i| i.instance_id == instance_id) {
            self.edit_trade_draft(account, |d| {
                let side = if give { &mut d.give } else { &mut d.get };
                side.retain(|i| i.instance_id != instance_id);
            });
            return Ok(());
        }
        let max = self.trade_rules(account).data.map_or(4, |r| r.max_items);
        if side.len() >= max {
            return Err(format!("Up to {max} items a side."));
        }
        let owner = if give { account.0 } else { d.partner };
        let item = self
            .tradable_items(account, owner, false)
            .data
            .and_then(|p| p.items.iter().find(|i| i.instance_id == instance_id).cloned())
            .ok_or_else(|| "That item isn't loaded.".to_owned())?;
        if give && item.on_hold {
            return Err(format!("{} is on hold and can't be traded yet.", item.name));
        }
        self.edit_trade_draft(account, |d| {
            if give { &mut d.give } else { &mut d.get }.push(item)
        });
        Ok(())
    }

    /// Send the draft, unless the trade system would refuse it.
    pub fn send_trade_draft(&self, account: AccountId) -> Result<(), String> {
        let d = self
            .trade_draft(account)
            .ok_or_else(|| "Start a trade first.".to_owned())?;
        let rules = self.trade_rules(account).data.map(|r| *r).unwrap_or_default();
        let e = evaluate(&d.give, d.give_robux, &d.get, d.get_robux, None, &rules);
        let blocking = e.warnings.iter().find(|w| {
            matches!(
                w,
                Warning::OneSided | Warning::TooManyItems | Warning::TooMuchRobux | Warning::OnHold(_)
            )
        });
        if let Some(w) = blocking {
            return Err(w.text());
        }
        self.set_trade_draft(account, None);
        self.send_trade(account, d);
        Ok(())
    }

    /// Take a trade out of the open lists right away (optimistic).
    fn drop_trade(&self, account: AccountId, trade_id: u64) {
        for b in [TradeBox::Inbound, TradeBox::Outbound] {
            self.inner
                .res
                .update::<Page<TradeRow>>(&Key::private(account, "trades", b.key()), |p| {
                    p.items.retain(|t| t.id != trade_id)
                });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::values::ItemValue;

    fn item(id: u64, rap: i64) -> TradeItem {
        TradeItem {
            item: ItemRef::asset(id),
            name: format!("Item {id}"),
            rap: Some(rap),
            ..Default::default()
        }
    }

    #[test]
    fn even_trade_has_no_warnings() {
        let r = TradeRules::default();
        let e = evaluate(&[item(1, 1000)], 0, &[item(2, 1100)], 0, None, &r);
        assert_eq!(e.give.total_rap(), 1000);
        assert_eq!(e.get.total_rap(), 1100);
        assert_eq!(e.net(), 100);
        assert_eq!(e.net_pct(), Some(10));
        assert!(e.warnings.is_empty(), "{:?}", e.warnings);
    }

    #[test]
    fn robux_you_get_is_after_the_fee() {
        let r = TradeRules::default();
        let e = evaluate(&[item(1, 1000)], 0, &[item(2, 500)], 500, None, &r);
        assert_eq!(e.get.robux, 350);
        assert_eq!(e.net_rap(), -150);
        // Robux you add counts in full against you.
        let e = evaluate(&[item(1, 1000)], 300, &[item(2, 1300)], 0, None, &r);
        assert_eq!(e.give.total_rap(), 1300);
        assert_eq!(e.net(), 0);
    }

    #[test]
    fn warnings() {
        let r = TradeRules::default();
        let mut big = item(1, 10_000);
        big.on_hold = true;
        let e = evaluate(&[big], 0, &[item(2, 1000)], 0, None, &r);
        assert!(e.warnings.contains(&Warning::Lopsided(90)), "{:?}", e.warnings);
        assert!(e.warnings.contains(&Warning::OnHold("Item 1".into())));
        let e = evaluate(&[], 0, &[item(2, 10)], 0, None, &r);
        assert!(e.warnings.contains(&Warning::OneSided));
        let five: Vec<TradeItem> = (0..5).map(|i| item(i, 10)).collect();
        let e = evaluate(&five, 0, &[item(9, 50)], 0, None, &r);
        assert!(e.warnings.contains(&Warning::TooManyItems));
        let e = evaluate(&[item(1, 100)], 80, &[item(2, 200)], 0, None, &r);
        assert!(e.warnings.contains(&Warning::TooMuchRobux));
    }

    #[test]
    fn values_replace_rap_and_flag_projected() {
        let r = TradeRules::default();
        let mut v = ValueTable::default();
        v.items.insert(
            2,
            ItemValue {
                value: Some(5000),
                ..Default::default()
            },
        );
        v.items.insert(
            3,
            ItemValue {
                projected: true,
                ..Default::default()
            },
        );
        let e = evaluate(
            &[item(1, 3000)],
            0,
            &[item(2, 1000), item(3, 2500)],
            0,
            Some(&v),
            &r,
        );
        // By RAP you'd only gain 500; by value item 2 is worth 5,000.
        assert_eq!(e.net_rap(), 500);
        assert_eq!(e.get.total_value(), Some(7500));
        assert_eq!(e.net(), 4500);
        assert!(e.warnings.contains(&Warning::Projected("Item 3".into())));
    }
}

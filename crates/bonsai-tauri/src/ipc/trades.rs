//! Trades: the four lists, one trade with both sides valued, and the composer's draft.
//!
//! The draft lives in `rbx-core` until `trade.send`. Adding or removing items and Robux are
//! steps inside the composer, so they are plain IPC calls, not registry commands. `list.more`
//! keys here are `trades:<box>` and `tradable:<user>`.

use std::sync::Arc;

use rbx_core::commands::CommandId;
use rbx_core::trades::{Draft, Eval, TradeBox, TradeItem, TradeRules, evaluate};
use rbx_core::values::ValueTable;
use rbx_core::{AccountId, Core};
use serde::Serialize;
use specta::Type;
use tauri::{AppHandle, State};

use super::market::{Paged, ValueTag, tag_of};
use super::{Id, Invocation, List, copy, count};

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct TradeRowView {
    pub id: Id,
    pub partner_id: Id,
    pub partner_name: String,
    pub partner_display: String,
    /// ISO dates.
    pub created: String,
    pub expires: String,
    pub status: String,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct TradeItemView {
    /// Names this copy to the trades API and to the draft calls.
    pub instance_id: String,
    pub id: Id,
    pub bundle: bool,
    pub name: String,
    pub serial: Option<u32>,
    pub rap: Option<u32>,
    /// Rolimons' data when values are on.
    pub values: Option<ValueTag>,
    pub original_price: Option<u32>,
    pub on_hold: bool,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct SideView {
    pub robux: u32,
    pub items: Vec<TradeItemView>,
}

/// Both sides valued from your point of view. Robux you get counts after the fee.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct EvalView {
    pub give_rap: f64,
    pub give_value: Option<f64>,
    pub get_rap: f64,
    pub get_value: Option<f64>,
    /// Robux you'd receive after the fee.
    pub get_robux_after_fee: f64,
    /// What you gain (negative: lose), by value when values are on, else RAP.
    pub net: f64,
    pub net_pct: Option<f64>,
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct TradeView {
    pub loading: bool,
    pub error: Option<String>,
    pub status: String,
    pub partner_id: Option<Id>,
    pub give: Option<SideView>,
    pub get: Option<SideView>,
    pub eval: Option<EvalView>,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Eligibility {
    pub loading: bool,
    pub can_trade: bool,
    /// Why not, in plain words.
    pub reason: String,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct DraftView {
    pub partner_id: Id,
    /// Empty until the user is known.
    pub partner_name: String,
    pub counter_of: Option<Id>,
    pub give: SideView,
    pub get: SideView,
    pub eval: EvalView,
    pub max_items: u32,
    /// Whether the partner can trade with you.
    pub partner_ok: Eligibility,
}

fn item_view(i: &TradeItem, values: Option<&ValueTable>) -> TradeItemView {
    TradeItemView {
        instance_id: i.instance_id.clone(),
        id: Id(i.item.id),
        bundle: i.item.kind == rbx_core::market::ItemKind::Bundle,
        name: i.name.clone(),
        serial: i.serial.and_then(count),
        rap: i.rap.and_then(count),
        values: tag_of(values, i.item),
        original_price: i.original_price.and_then(count),
        on_hold: i.on_hold,
    }
}

fn side_view(items: &[TradeItem], robux: i64, values: Option<&ValueTable>) -> SideView {
    SideView {
        robux: count(robux).unwrap_or(0),
        items: items.iter().map(|i| item_view(i, values)).collect(),
    }
}

fn eval_view(e: &Eval) -> EvalView {
    EvalView {
        give_rap: e.give.total_rap() as f64,
        give_value: e.give.total_value().map(|v| v as f64),
        get_rap: e.get.total_rap() as f64,
        get_value: e.get.total_value().map(|v| v as f64),
        get_robux_after_fee: e.get.robux as f64,
        net: e.net() as f64,
        net_pct: e.net_pct().map(|p| p as f64),
        warnings: e.warnings.iter().map(|w| w.text()).collect(),
    }
}

fn rules(core: &Core, a: AccountId) -> TradeRules {
    core.trade_rules(a).data.map(|r| *r).unwrap_or_default()
}

fn signed_out<T>() -> List<T> {
    List {
        loading: false,
        error: Some("Add an account to trade.".to_owned()),
        items: Vec::new(),
        more: false,
    }
}

#[tauri::command]
#[specta::specta]
pub fn trade_list(core: State<'_, Arc<Core>>, which: String) -> Paged<TradeRowView> {
    let more_key = format!("trades:{which}");
    let (Some(a), Some(b)) = (core.acting(None), TradeBox::from_key(&which)) else {
        return Paged {
            list: signed_out(),
            more_key,
        };
    };
    Paged {
        list: List::paged(core.trades(a, b, false), |t| TradeRowView {
            id: Id(t.id),
            partner_id: Id(t.partner),
            partner_name: t.partner_name.clone(),
            partner_display: t.partner_display.clone(),
            created: t.created.clone(),
            expires: t.expires.clone(),
            status: t.status.clone(),
        }),
        more_key,
    }
}

#[tauri::command]
#[specta::specta]
pub fn trade_detail(core: State<'_, Arc<Core>>, trade_id: Id) -> TradeView {
    let empty = TradeView {
        loading: false,
        error: None,
        status: String::new(),
        partner_id: None,
        give: None,
        get: None,
        eval: None,
    };
    let Some(a) = core.acting(None) else {
        return empty;
    };
    let r = core.trade(a, trade_id.0);
    let Some(t) = r.data else {
        return TradeView {
            loading: r.loading,
            error: r.error,
            ..empty
        };
    };
    let v = core.item_values().data;
    let e = evaluate(
        &t.mine.items,
        t.mine.robux,
        &t.theirs.items,
        t.theirs.robux,
        v.as_deref(),
        &rules(&core, a),
    );
    TradeView {
        loading: r.loading,
        error: r.error,
        status: t.status.clone(),
        partner_id: Some(Id(t.theirs.user_id)),
        give: Some(side_view(&t.mine.items, t.mine.robux, v.as_deref())),
        get: Some(side_view(&t.theirs.items, t.theirs.robux, v.as_deref())),
        eval: Some(eval_view(&e)),
    }
}

/// Whether the active account can trade at all.
#[tauri::command]
#[specta::specta]
pub fn trade_eligibility(core: State<'_, Arc<Core>>) -> Eligibility {
    let Some(a) = core.acting(None) else {
        return Eligibility {
            loading: false,
            can_trade: false,
            reason: "Add an account to trade.".to_owned(),
        };
    };
    let r = core.can_trade(a);
    Eligibility {
        loading: r.loading,
        can_trade: r.data.as_ref().is_some_and(|e| e.can_trade),
        reason: r.data.map(|e| e.reason.clone()).or(r.error).unwrap_or_default(),
    }
}

#[tauri::command]
#[specta::specta]
pub fn trade_draft(core: State<'_, Arc<Core>>) -> Option<DraftView> {
    let a = core.acting(None)?;
    let d: Draft = core.trade_draft(a)?;
    let v = core.item_values().data;
    let rules = rules(&core, a);
    let e = evaluate(&d.give, d.give_robux, &d.get, d.get_robux, v.as_deref(), &rules);
    let ok = core.can_trade_with(a, d.partner);
    let partner_name = core
        .store()
        .users
        .get(&d.partner)
        .map(|u| u.display_name.clone())
        .unwrap_or_default();
    if partner_name.is_empty() {
        core.want_users(&[d.partner]);
    }
    Some(DraftView {
        partner_id: Id(d.partner),
        partner_name,
        counter_of: d.counter_of.map(Id),
        give: side_view(&d.give, d.give_robux, v.as_deref()),
        get: side_view(&d.get, d.get_robux, v.as_deref()),
        eval: eval_view(&e),
        max_items: rules.max_items as u32,
        partner_ok: Eligibility {
            loading: ok.loading,
            can_trade: ok.data.as_ref().is_some_and(|e| e.can_trade),
            reason: ok.data.map(|e| e.reason.clone()).or(ok.error).unwrap_or_default(),
        },
    })
}

/// Copies a user can put in a trade: yours, or the partner's.
#[tauri::command]
#[specta::specta]
pub fn tradable_items(core: State<'_, Arc<Core>>, user_id: Id) -> Paged<TradeItemView> {
    let more_key = format!("tradable:{}", user_id.0);
    let Some(a) = core.acting(None) else {
        return Paged {
            list: signed_out(),
            more_key,
        };
    };
    let v = core.item_values().data;
    Paged {
        list: List::paged(core.tradable_items(a, user_id.0, false), |i| {
            item_view(i, v.as_deref())
        }),
        more_key,
    }
}

/// Puts a copy on a side of the draft, or takes it off. A composer step.
#[tauri::command]
#[specta::specta]
pub fn draft_toggle_item(core: State<'_, Arc<Core>>, give: bool, instance_id: String) -> Result<(), String> {
    let a = core
        .acting(None)
        .ok_or_else(|| "Add an account first.".to_owned())?;
    core.toggle_draft_item(a, give, &instance_id)
}

/// Sets the Robux on a side of the draft. A composer step.
#[tauri::command]
#[specta::specta]
pub fn draft_set_robux(core: State<'_, Arc<Core>>, give: bool, robux: u32) {
    if let Some(a) = core.acting(None) {
        core.edit_trade_draft(a, |d| {
            if give {
                d.give_robux = robux.into();
            } else {
                d.get_robux = robux.into();
            }
        });
    }
}

/// Throws the draft away. A composer step.
#[tauri::command]
#[specta::specta]
pub fn discard_trade_draft(core: State<'_, Arc<Core>>) {
    if let Some(a) = core.acting(None) {
        core.discard_trade_draft(a);
    }
}

/// Runs a `trade.*` command, or a `list.more` for a trade list. `None` for anything else.
pub(super) fn run(
    app: &AppHandle,
    core: &Core,
    id: CommandId,
    inv: &Invocation,
    account: Option<AccountId>,
) -> Option<Result<(), String>> {
    use CommandId as C;
    let need = || account.ok_or_else(|| "Add an account first.".to_owned());
    let trade_id = || {
        inv.arg
            .as_deref()
            .and_then(|a| a.trim().parse::<u64>().ok())
            .ok_or_else(|| "Which trade?".to_owned())
    };
    Some(match id {
        C::TradeWith => (|| {
            let a = need()?;
            let who = inv
                .arg
                .clone()
                .filter(|s| !s.trim().is_empty())
                .or_else(|| {
                    inv.context
                        .as_deref()
                        .filter(|c| c.starts_with("user:"))
                        .and_then(rbx_core::links::target_id)
                        .map(|id| id.to_string())
                })
                .ok_or_else(|| "Trade with whom?".to_owned())?;
            core.with_user(&who, move |c, user| {
                if user == a.0 {
                    c.notice("You can't trade with yourself.", true);
                } else {
                    c.start_trade(a, user);
                }
            })
        })(),
        C::AcceptTrade => (|| {
            core.accept_trade(need()?, trade_id()?);
            Ok(())
        })(),
        C::DeclineTrade => (|| {
            core.decline_trade(need()?, trade_id()?);
            Ok(())
        })(),
        C::CounterTrade => (|| core.start_counter(need()?, trade_id()?))(),
        C::SendTrade => (|| core.send_trade_draft(need()?))(),
        C::CopyTradesCsv => (|| {
            let a = need()?;
            let b = inv
                .context
                .as_deref()
                .and_then(|c| c.strip_prefix("trades:"))
                .and_then(TradeBox::from_key)
                .ok_or_else(|| "Open Trades first.".to_owned())?;
            let rows = core
                .trades(a, b, false)
                .data
                .map(|p| p.items.clone())
                .unwrap_or_default();
            if rows.is_empty() {
                return Err("No trades loaded.".to_owned());
            }
            let mut csv = String::from("trade_id,partner_id,partner,status,created,expires\n");
            for t in &rows {
                csv.push_str(&format!(
                    "{},{},{},{},{},{}\n",
                    t.id,
                    t.partner,
                    super::communities::csv_field(&t.partner_name),
                    t.status,
                    t.created,
                    t.expires
                ));
            }
            copy(app, core, csv, &format!("Copied {} trades as CSV.", rows.len()))
        })(),
        C::LoadMore => {
            let (what, rest) = inv.arg.as_deref()?.trim().split_once(':')?;
            let a = account?;
            match what {
                "trades" => TradeBox::from_key(rest)
                    .map(|b| drop(core.trades(a, b, true)))
                    .ok_or_else(|| "Which list?".to_owned()),
                "tradable" => rest
                    .parse()
                    .map(|u| drop(core.tradable_items(a, u, true)))
                    .map_err(|_| "Whose items?".to_owned()),
                _ => return None,
            }
        }
        _ => return None,
    })
}

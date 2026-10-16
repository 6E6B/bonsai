//! Transactions: one view per scope (you or a community) with Roblox's period totals, sums of
//! the filtered ledger and one window of its rows, plus insights and the all-accounts rollup.
//!
//! The ledger can hold tens of thousands of rows, so a view sends only the first `limit`
//! matches; the screen asks for more as it scrolls. Filtering runs on the in-memory ledger.
//! Commands read the screen's query from the context `transactions:<json>`.

use std::sync::Arc;

use bonsai_core::commands::CommandId;
use bonsai_core::transactions::{
    Filter, Group, Ledger, Period, Scope, Sums, Totals, TxKind, Txn, by_month, spend_by_experience, sums,
    top_buyers, top_items,
};
use bonsai_core::{AccountId, Core};
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::{AppHandle, State};

use super::{Id, Invocation, copy};

/// What the screen shows. Sent as an argument and, as JSON, in the command context.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Type)]
#[serde(rename_all = "camelCase", default)]
pub struct TxQuery {
    /// `me` or `group:<id>`.
    pub scope: String,
    /// day, week, month, year, all or custom.
    pub period: String,
    /// Custom range, ms since the epoch; `to` is exclusive.
    pub from: Option<f64>,
    pub to: Option<f64>,
    /// A type key (`sales`); empty for all.
    pub kind: String,
    pub text: String,
    pub min_amount: Option<f64>,
    pub pending_only: bool,
    /// Rows to send.
    pub limit: u32,
}

impl TxQuery {
    fn scope(&self) -> Scope {
        Scope::from_key(&self.scope).unwrap_or(Scope::Me)
    }

    fn period(&self) -> Option<Period> {
        Period::from_key(&self.period)
    }

    fn filter(&self) -> Filter {
        let secs = |ms: f64| (ms / 1000.0).floor() as i64;
        let (from, to) = match (self.period(), self.period.as_str()) {
            (Some(p), _) => (Some(bonsai_core::cache::now() - p.secs()), None),
            (None, "custom") => (self.from.map(secs), self.to.map(secs)),
            _ => (None, None),
        };
        Filter {
            kinds: TxKind::from_key(&self.kind).into_iter().collect(),
            from,
            to,
            text: self.text.clone(),
            min_amount: self.min_amount.map(|m| m.max(0.0) as i64).filter(|m| *m > 0),
            pending_only: self.pending_only,
        }
    }

    fn from_context(inv: &Invocation) -> Option<TxQuery> {
        serde_json::from_str(inv.context.as_deref()?.strip_prefix("transactions:")?).ok()
    }
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct KindAmount {
    pub key: String,
    pub label: String,
    pub amount: f64,
    /// Rows in the ledger; `None` for Roblox's totals.
    pub count: Option<u32>,
}

/// Robux in and out. Outgoing is negative.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct TotalsView {
    pub loading: bool,
    pub error: Option<String>,
    pub incoming: f64,
    pub outgoing: f64,
    pub pending: f64,
    pub by_kind: Vec<KindAmount>,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct TxRowView {
    pub id: String,
    pub kind: String,
    pub kind_label: String,
    /// ms since the epoch.
    pub at: f64,
    pub amount: f64,
    pub pending: bool,
    pub item_id: Option<Id>,
    pub item_name: String,
    /// Asset, Bundle, GamePass, DeveloperProduct, ...
    pub item_kind: String,
    pub place_id: Option<Id>,
    pub universe_id: Option<Id>,
    pub experience: String,
    pub agent_id: Option<Id>,
    pub agent_name: String,
    pub agent_group: bool,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct KindOption {
    pub key: String,
    pub label: String,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct TxView {
    pub signed_in: bool,
    pub scope: String,
    /// Robux of the account or the community's funds.
    pub balance: Option<f64>,
    pub syncing: bool,
    pub loading_older: bool,
    /// Some type isn't read to its first row yet.
    pub has_older: bool,
    /// ms since the epoch.
    pub synced_at: Option<f64>,
    pub oldest: Option<f64>,
    pub error: Option<String>,
    /// Rows in the whole ledger.
    pub ledger_count: u32,
    /// Types in the ledger or that the account has, for the type filter.
    pub kinds: Vec<KindOption>,
    /// Roblox's totals for day, week, month or year.
    pub roblox: Option<TotalsView>,
    /// Sums over every row that matches.
    pub sums: TotalsView,
    pub matched: u32,
    /// The first `limit` matches, newest first.
    pub rows: Vec<TxRowView>,
}

fn ms(secs: i64) -> f64 {
    secs as f64 * 1000.0
}

fn id(n: u64) -> Option<Id> {
    (n > 0).then_some(Id(n))
}

fn row_view(r: &Txn) -> TxRowView {
    let item = r.item.as_ref();
    let place = r.experience();
    let agent = r.agent.as_ref();
    TxRowView {
        id: r.id.clone(),
        kind: r.kind.key().into(),
        kind_label: r.kind.label().into(),
        at: ms(r.at),
        amount: r.amount as f64,
        pending: r.pending,
        item_id: item.and_then(|i| id(i.id)),
        item_name: item.map(|i| i.name.clone()).unwrap_or_default(),
        item_kind: item.map(|i| i.kind.clone()).unwrap_or_default(),
        place_id: place.and_then(|p| id(p.place_id)),
        universe_id: place.and_then(|p| id(p.universe_id)),
        experience: place.map(|p| p.name.clone()).unwrap_or_default(),
        agent_id: agent.and_then(|a| id(a.id)),
        agent_name: agent.map(|a| a.name.clone()).unwrap_or_default(),
        agent_group: agent.is_some_and(|a| a.group),
    }
}

fn kind_amount(k: TxKind, amount: i64, count: Option<usize>) -> KindAmount {
    KindAmount {
        key: k.key().into(),
        label: k.label().into(),
        amount: amount as f64,
        count: count.map(|n| n as u32),
    }
}

fn sums_view(s: &Sums) -> TotalsView {
    TotalsView {
        loading: false,
        error: None,
        incoming: s.incoming as f64,
        outgoing: s.outgoing as f64,
        pending: s.pending as f64,
        by_kind: s
            .by_kind
            .iter()
            .map(|(k, a, n)| kind_amount(*k, *a, Some(*n)))
            .collect(),
    }
}

fn totals_view(r: bonsai_core::res::Res<Totals>) -> TotalsView {
    let t = r.data.as_deref().cloned().unwrap_or_default();
    TotalsView {
        loading: r.loading && r.data.is_none(),
        error: r.error,
        incoming: t.incoming as f64,
        outgoing: t.outgoing as f64,
        pending: t.pending as f64,
        by_kind: t.by_kind.iter().map(|(k, a)| kind_amount(*k, *a, None)).collect(),
    }
}

fn matching<'a>(l: &'a Ledger, q: &TxQuery) -> Vec<&'a Txn> {
    let f = q.filter();
    l.rows.iter().filter(|r| f.matches(r)).collect()
}

fn signed_out(q: &TxQuery) -> TxView {
    let empty = TotalsView {
        loading: false,
        error: None,
        incoming: 0.0,
        outgoing: 0.0,
        pending: 0.0,
        by_kind: Vec::new(),
    };
    TxView {
        signed_in: false,
        scope: q.scope.clone(),
        balance: None,
        syncing: false,
        loading_older: false,
        has_older: false,
        synced_at: None,
        oldest: None,
        error: Some("Add an account to see transactions.".into()),
        ledger_count: 0,
        kinds: Vec::new(),
        roblox: None,
        sums: empty,
        matched: 0,
        rows: Vec::new(),
    }
}

#[tauri::command]
#[specta::specta]
pub fn transactions_view(core: State<'_, Arc<Core>>, q: TxQuery) -> TxView {
    let Some(a) = core.acting(None) else {
        return signed_out(&q);
    };
    let scope = q.scope();
    let l = core.ledger(a, scope, false);
    let rows = matching(&l, &q);
    let s = sums(rows.iter().copied());
    let balance = match scope {
        Scope::Me => core.store().data.get(&a).and_then(|d| d.robux).map(|r| r as f64),
        Scope::Group(g) => core.community_funds(a, g).data.map(|r| *r as f64),
    };
    let mut kinds: Vec<TxKind> = l.rows.iter().map(|r| r.kind).collect();
    if scope == Scope::Me
        && let Some(k) = core.tx_kinds(a).data
    {
        kinds.extend(k.iter().copied());
    }
    kinds.sort_by_key(|k| TxKind::ALL.iter().position(|x| x == k));
    kinds.dedup();
    let limit = q.limit.clamp(50, 20_000) as usize;
    TxView {
        signed_in: true,
        scope: scope.key(),
        balance,
        syncing: l.syncing,
        loading_older: l.loading_older,
        has_older: !l.loaded || l.has_older(),
        synced_at: (l.synced_at > 0).then(|| ms(l.synced_at)),
        oldest: l.oldest().map(ms),
        error: l.error.clone(),
        ledger_count: l.rows.len() as u32,
        kinds: kinds
            .into_iter()
            .filter(|k| *k != TxKind::Unknown)
            .map(|k| KindOption {
                key: k.key().into(),
                label: k.label().into(),
            })
            .collect(),
        roblox: q.period().map(|p| totals_view(core.tx_totals(a, scope, p))),
        sums: sums_view(&s),
        matched: rows.len() as u32,
        rows: rows.iter().take(limit).map(|r| row_view(r)).collect(),
    }
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct MonthView {
    /// `YYYY-MM`.
    pub month: String,
    pub incoming: f64,
    pub outgoing: f64,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GroupView {
    pub id: Id,
    pub name: String,
    pub amount: f64,
    pub count: u32,
    pub group: bool,
}

/// Beyond the website: the filtered ledger by month, experience, item and buyer.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct TxInsights {
    pub months: Vec<MonthView>,
    pub experiences: Vec<GroupView>,
    pub items: Vec<GroupView>,
    pub buyers: Vec<GroupView>,
}

fn group_view(g: &Group) -> GroupView {
    GroupView {
        id: Id(g.id),
        name: g.name.clone(),
        amount: g.amount as f64,
        count: g.count as u32,
        group: g.group,
    }
}

#[tauri::command]
#[specta::specta]
pub fn transaction_insights(core: State<'_, Arc<Core>>, q: TxQuery) -> TxInsights {
    let empty = TxInsights {
        months: Vec::new(),
        experiences: Vec::new(),
        items: Vec::new(),
        buyers: Vec::new(),
    };
    let Some(a) = core.acting(None) else {
        return empty;
    };
    let l = core.ledger(a, q.scope(), false);
    let rows = matching(&l, &q);
    let it = || rows.iter().copied();
    TxInsights {
        months: by_month(it(), bonsai_core::fmt::local_offset())
            .into_iter()
            .map(|(month, i, o)| MonthView {
                month,
                incoming: i as f64,
                outgoing: o as f64,
            })
            .collect(),
        experiences: spend_by_experience(it(), 8).iter().map(group_view).collect(),
        items: top_items(it(), 8).iter().map(group_view).collect(),
        buyers: top_buyers(it(), 8).iter().map(group_view).collect(),
    }
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct AccountTotalsView {
    pub id: Id,
    pub name: String,
    pub robux: Option<f64>,
    pub totals: TotalsView,
}

/// Beyond the website: every signed-in account's balance and totals for one period.
#[tauri::command]
#[specta::specta]
pub fn transaction_accounts(core: State<'_, Arc<Core>>, period: String) -> Vec<AccountTotalsView> {
    let p = Period::from_key(&period).unwrap_or_default();
    let accounts: Vec<(AccountId, String, Option<i64>)> = {
        let s = core.store();
        s.accounts
            .accounts
            .iter()
            .map(|m| {
                let name = m.label.clone().unwrap_or_else(|| m.display_name.clone());
                (m.id, name, s.data.get(&m.id).and_then(|d| d.robux))
            })
            .collect()
    };
    accounts
        .into_iter()
        .map(|(a, name, robux)| AccountTotalsView {
            id: Id(a.0),
            name,
            robux: robux.map(|r| r as f64),
            totals: totals_view(core.tx_totals(a, Scope::Me, p)),
        })
        .collect()
}

pub(super) fn run(
    app: &AppHandle,
    core: &Core,
    id: CommandId,
    inv: &Invocation,
    account: Option<AccountId>,
) -> Option<Result<(), String>> {
    use CommandId as C;
    if !matches!(
        id,
        C::TxSync | C::TxLoadOlder | C::TxStop | C::TxExport | C::TxCopyCsv
    ) {
        return None;
    }
    let Some(a) = account else {
        return Some(Err("Add an account first.".to_owned()));
    };
    let Some(q) = TxQuery::from_context(inv) else {
        return Some(Err("Open Transactions first.".to_owned()));
    };
    let scope = q.scope();
    Some(match id {
        C::TxSync => {
            drop(core.ledger(a, scope, true));
            Ok(())
        }
        C::TxLoadOlder => core.ledger_load_older(a, scope),
        C::TxStop => {
            core.ledger_stop(a, scope);
            Ok(())
        }
        C::TxExport | C::TxCopyCsv => (|| {
            let l = core.ledger(a, scope, false);
            let rows: Vec<Txn> = matching(&l, &q).into_iter().cloned().collect();
            if rows.is_empty() {
                return Err("No transactions to export.".to_owned());
            }
            if id == C::TxCopyCsv {
                let n = rows.len();
                return copy(
                    app,
                    core,
                    bonsai_core::transactions::csv(&rows),
                    &format!("Copied {n} transactions as CSV."),
                );
            }
            let name = match scope {
                Scope::Me => format!("transactions {}", a.0),
                Scope::Group(g) => format!("community {g} transactions"),
            };
            let path = core.save_ledger_csv(&name, &rows)?;
            core.notice(format!("Saved {} transactions to {path}", rows.len()), false);
            Ok(())
        })(),
        _ => return None,
    })
}

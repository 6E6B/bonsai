//! Discover: the site's Charts, with device and location filters, and "see all" for one chart.
//! `list.more` loads more charts with the key `charts:<device>:<country>`.

use std::sync::Arc;

use rbx_core::commands::CommandId;
use rbx_core::store::HomeSort;
use rbx_core::{AccountId, Core};
use serde::Serialize;
use specta::Type;
use tauri::State;

use super::market::Choice;
use super::{GameCard, Invocation, viewer};

#[derive(Clone, Debug, Serialize, Type)]
pub struct ChartFilterView {
    /// `device` or `country`.
    pub kind: String,
    pub title: String,
    pub selected: String,
    pub options: Vec<Choice>,
}

#[derive(Clone, Debug, Serialize, Type)]
pub struct ChartView {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    /// What the chart means, when Roblox says.
    pub info: String,
    pub games: Vec<GameCard>,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct DiscoverView {
    pub loading: bool,
    pub error: Option<String>,
    pub filters: Vec<ChartFilterView>,
    pub charts: Vec<ChartView>,
    pub more: bool,
    pub more_key: String,
}

fn chart_view(s: &HomeSort) -> ChartView {
    ChartView {
        id: s.id.clone(),
        title: s.title.clone(),
        subtitle: s.subtitle.clone(),
        info: s.info.clone(),
        games: s.games.iter().map(GameCard::from).collect(),
    }
}

/// Charts for a device and location; empty strings take Roblox's defaults.
#[tauri::command]
#[specta::specta]
pub fn discover(core: State<'_, Arc<Core>>, device: String, country: String) -> DiscoverView {
    let r = core.charts(viewer(&core), &device, &country, false);
    let sorts = r.data.as_ref().map(|p| p.items.as_slice()).unwrap_or_default();
    DiscoverView {
        loading: r.loading,
        error: r.error.clone(),
        filters: sorts
            .iter()
            .flat_map(|s| &s.filters)
            .map(|f| ChartFilterView {
                kind: f.kind.clone(),
                title: f.title.clone(),
                selected: f.selected.clone(),
                options: f
                    .options
                    .iter()
                    .map(|(value, label)| Choice {
                        value: value.clone(),
                        label: label.clone(),
                    })
                    .collect(),
            })
            .collect(),
        charts: sorts
            .iter()
            .filter(|s| !s.games.is_empty())
            .map(chart_view)
            .collect(),
        more: r.data.as_ref().is_some_and(|p| p.next.is_some()),
        more_key: format!("charts:{device}:{country}"),
    }
}

#[derive(Clone, Debug, Serialize, Type)]
pub struct ChartPage {
    pub loading: bool,
    pub error: Option<String>,
    pub chart: Option<ChartView>,
}

/// Every game in one chart ("See all").
#[tauri::command]
#[specta::specta]
pub fn discover_chart(
    core: State<'_, Arc<Core>>,
    sort_id: String,
    device: String,
    country: String,
) -> ChartPage {
    let r = core.chart(viewer(&core), &sort_id, &device, &country);
    ChartPage {
        loading: r.loading,
        error: r.error,
        chart: r.data.as_deref().map(chart_view),
    }
}

/// `list.more` for charts. `None` for other keys.
pub(super) fn run(
    core: &Core,
    id: CommandId,
    inv: &Invocation,
    account: Option<AccountId>,
) -> Option<Result<(), String>> {
    if id != CommandId::LoadMore {
        return None;
    }
    let rest = inv.arg.as_deref()?.trim().strip_prefix("charts:")?;
    let (device, country) = rest.split_once(':').unwrap_or((rest, ""));
    let a = account.unwrap_or(AccountId::ANONYMOUS);
    drop(core.charts(a, device, country, true));
    Some(Ok(()))
}

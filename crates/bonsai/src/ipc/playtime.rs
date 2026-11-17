//! The playtime log: your accounts' time per experience, account and week.

use std::sync::Arc;

use bonsai_core::commands::CommandId;
use bonsai_core::playtime::Range;
use bonsai_core::{AccountId, Core};
use serde::Serialize;
use specta::Type;
use tauri::State;

use super::{Id, Invocation};

/// Seconds one account spent.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PlaytimeAccount {
    pub account: Id,
    /// The account's label, display name or username.
    pub label: String,
    #[specta(type = specta_typescript::Number)]
    pub secs: f64,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PlaytimeExperience {
    pub universe_id: Id,
    /// The root place, to open the experience. 0 when unknown.
    pub place_id: Id,
    /// Empty until the experience has loaded.
    pub name: String,
    #[specta(type = specta_typescript::Number)]
    pub secs: f64,
    pub sessions: u32,
    /// Unix milliseconds.
    #[specta(type = specta_typescript::Number)]
    pub last_played: f64,
    pub accounts: Vec<PlaytimeAccount>,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PlaytimeWeek {
    /// Monday 00:00 local, in unix milliseconds.
    #[specta(type = specta_typescript::Number)]
    pub start: f64,
    #[specta(type = specta_typescript::Number)]
    pub secs: f64,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PlaytimeView {
    /// Recording is on in Settings.
    pub enabled: bool,
    /// Accounts other than the active one are recorded too.
    pub background: bool,
    pub range: String,
    #[specta(type = specta_typescript::Number)]
    pub secs: f64,
    pub sessions: u32,
    pub experiences: Vec<PlaytimeExperience>,
    /// Every account with time in the range, even when narrowed to one.
    pub accounts: Vec<PlaytimeAccount>,
    pub weeks: Vec<PlaytimeWeek>,
}

fn labels(core: &Core) -> Vec<(AccountId, String)> {
    core.store()
        .accounts
        .accounts
        .iter()
        .map(|a| {
            let label = a
                .label
                .clone()
                .filter(|l| !l.trim().is_empty())
                .or_else(|| (!a.display_name.is_empty()).then(|| a.display_name.clone()))
                .unwrap_or_else(|| a.name.clone());
            (a.id, label)
        })
        .collect()
}

/// Your accounts' playtime over `range` (`week`, `month`, `quarter`, `year` or `all`), narrowed
/// to `account` when given. `offset_minutes` is local time's offset from UTC, east positive.
#[tauri::command]
#[specta::specta]
pub fn playtime(
    core: State<'_, Arc<Core>>,
    range: String,
    account: Option<Id>,
    offset_minutes: i32,
) -> PlaytimeView {
    let r = Range::parse(&range).unwrap_or(Range::Week);
    let b = core.playtime(r, account.map(|a| AccountId(a.0)), i64::from(offset_minutes) * 60);
    let names = labels(&core);
    let label = |a: AccountId| {
        names
            .iter()
            .find(|(id, _)| *id == a)
            .map(|(_, l)| l.clone())
            .unwrap_or_default()
    };
    let accounts = |v: &[(AccountId, i64)]| {
        v.iter()
            .map(|(a, secs)| PlaytimeAccount {
                account: Id(a.0),
                label: label(*a),
                secs: *secs as f64,
            })
            .collect::<Vec<_>>()
    };
    let experiences = {
        let s = core.store();
        b.experiences
            .iter()
            .map(|g| {
                let info = s.games.get(&g.universe_id);
                let place = if g.place_id > 0 {
                    g.place_id
                } else {
                    info.map_or(0, |i| i.root_place_id)
                };
                PlaytimeExperience {
                    universe_id: Id(g.universe_id),
                    place_id: Id(place),
                    name: info.map(|i| i.name.clone()).unwrap_or_default(),
                    secs: g.secs as f64,
                    sessions: g.sessions,
                    last_played: g.last_played as f64 * 1000.0,
                    accounts: accounts(&g.accounts),
                }
            })
            .collect()
    };
    let (enabled, background) = {
        let s = core.settings();
        (s.playtime_log, s.background_accounts)
    };
    PlaytimeView {
        enabled,
        background,
        range: r.name().to_owned(),
        secs: b.secs as f64,
        sessions: b.sessions,
        experiences,
        accounts: accounts(&b.accounts),
        weeks: b
            .weeks
            .iter()
            .map(|(start, secs)| PlaytimeWeek {
                start: *start as f64 * 1000.0,
                secs: *secs as f64,
            })
            .collect(),
    }
}

/// Playtime commands that go to `bonsai-core`; `None` for any other command.
pub(super) fn run(core: &Core, id: CommandId, _inv: &Invocation) -> Option<Result<(), String>> {
    match id {
        CommandId::ClearPlaytime => {
            core.clear_playtime();
            Some(Ok(()))
        }
        _ => None,
    }
}

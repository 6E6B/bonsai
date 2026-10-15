//! App settings (no secrets) and per-account UI state.

use std::collections::HashMap;
use std::path::Path;

use rbx_api::AccountId;
use serde::{Deserialize, Serialize};

use crate::commands::{CommandId, Shortcut};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Theme {
    #[default]
    System,
    Dark,
    Light,
}

impl Theme {
    pub const ALL: [Theme; 3] = [Theme::System, Theme::Dark, Theme::Light];
    pub fn name(self) -> &'static str {
        match self {
            Theme::System => "system",
            Theme::Dark => "dark",
            Theme::Light => "light",
        }
    }
    pub fn from_name(s: &str) -> Option<Theme> {
        Theme::ALL
            .into_iter()
            .find(|t| t.name().eq_ignore_ascii_case(s.trim()))
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Density {
    #[default]
    Comfortable,
    Compact,
}

impl Density {
    pub const ALL: [Density; 2] = [Density::Comfortable, Density::Compact];
    pub fn name(self) -> &'static str {
        match self {
            Density::Comfortable => "comfortable",
            Density::Compact => "compact",
        }
    }
    pub fn from_name(s: &str) -> Option<Density> {
        Density::ALL
            .into_iter()
            .find(|d| d.name().eq_ignore_ascii_case(s.trim()))
    }
}

/// Which events the user wants to hear about. Saved here; watchers only notify, never write.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct NotifyPrefs {
    pub friend_joins: bool,
    pub trades: bool,
    pub chats: bool,
    pub friend_requests: bool,
    pub experience_updates: bool,
}

impl NotifyPrefs {
    pub const KINDS: [&'static str; 5] = [
        "friend_joins",
        "trades",
        "chats",
        "friend_requests",
        "experience_updates",
    ];
    pub fn get(&self, kind: &str) -> Option<bool> {
        Some(match kind {
            "friend_joins" => self.friend_joins,
            "trades" => self.trades,
            "chats" => self.chats,
            "friend_requests" => self.friend_requests,
            "experience_updates" => self.experience_updates,
            _ => return None,
        })
    }
    /// Sets one kind; false when the kind is unknown.
    pub fn set(&mut self, kind: &str, on: bool) -> bool {
        let slot = match kind {
            "friend_joins" => &mut self.friend_joins,
            "trades" => &mut self.trades,
            "chats" => &mut self.chats,
            "friend_requests" => &mut self.friend_requests,
            "experience_updates" => &mut self.experience_updates,
            _ => return false,
        };
        *slot = on;
        true
    }
}

impl Default for NotifyPrefs {
    fn default() -> Self {
        Self {
            friend_joins: true,
            trades: true,
            chats: true,
            friend_requests: false,
            experience_updates: false,
        }
    }
}

/// A BCP 47 tag for number and date formatting; empty means the system's.
pub fn valid_format_locale(s: &str) -> bool {
    s.len() <= 35
        && s.split('-')
            .all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_alphanumeric()))
        || s.is_empty()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub theme: Theme,
    pub density: Density,
    pub ui_scale: f32,
    /// Command used instead of the OS `roblox-player:` handler, with `{uri}`.
    pub player_command: Option<String>,
    /// Off by default: running several game clients at once is the user's call under Roblox's rules.
    pub allow_multiple_clients: bool,
    /// Keep realtime and unread polling for every signed-in account, not only the active one.
    pub background_accounts: bool,
    pub locale: String,
    /// Locale for numbers and dates in the UI (BCP 47, e.g. `de-DE`); empty follows the system.
    pub format_locale: String,
    pub notify: NotifyPrefs,
    pub shortcuts: HashMap<CommandId, Vec<Shortcut>>,
    /// Off by default: show limited item values from Rolimons (a third-party site) next to RAP.
    pub item_values: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: Theme::System,
            density: Density::Comfortable,
            ui_scale: 1.0,
            player_command: None,
            allow_multiple_clients: false,
            background_accounts: true,
            locale: "en_us".into(),
            format_locale: String::new(),
            notify: NotifyPrefs::default(),
            shortcuts: HashMap::new(),
            item_values: false,
        }
    }
}

impl Settings {
    pub fn load(p: &Path) -> Settings {
        std::fs::read_to_string(p)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }
    pub fn save(&self, p: &Path) -> std::io::Result<()> {
        write_json(p, self)
    }
}

/// An open tab.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TabRef {
    Experience { place_id: u64 },
    Profile { user_id: u64 },
    Item { asset_id: u64 },
    Bundle { bundle_id: u64 },
    Community { group_id: u64 },
}

/// Per-account UI state, restored exactly on switch and on restart.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UiState {
    pub section: String,
    pub tabs: Vec<TabRef>,
    /// `None` = the section page, `Some(i)` = tab i.
    pub active_tab: Option<usize>,
    pub closed_tabs: Vec<TabRef>,
    /// Scroll offsets by view id.
    pub scroll: HashMap<String, f32>,
    /// Filter and sort values by view id.
    pub filters: HashMap<String, String>,
    pub drawer_open: bool,
    pub sidebar_collapsed: bool,
    pub chat_open: Option<u64>,
}

impl UiState {
    pub fn fresh() -> UiState {
        UiState {
            section: "home".into(),
            drawer_open: true,
            ..Default::default()
        }
    }

    /// Open (or focus) a tab.
    pub fn open_tab(&mut self, t: TabRef, focus: bool) {
        let i = match self.tabs.iter().position(|x| *x == t) {
            Some(i) => i,
            None => {
                self.tabs.push(t);
                self.tabs.len() - 1
            }
        };
        if focus {
            self.active_tab = Some(i);
        }
    }

    pub fn close_tab(&mut self, i: usize) {
        if i >= self.tabs.len() {
            return;
        }
        let t = self.tabs.remove(i);
        self.closed_tabs.push(t);
        if self.closed_tabs.len() > 20 {
            self.closed_tabs.remove(0);
        }
        self.active_tab = match self.active_tab {
            Some(a) if a == i => {
                if self.tabs.is_empty() {
                    None
                } else {
                    Some(i.min(self.tabs.len() - 1))
                }
            }
            Some(a) if a > i => Some(a - 1),
            other => other,
        };
    }

    pub fn reopen_tab(&mut self) {
        if let Some(t) = self.closed_tabs.pop() {
            self.open_tab(t, true);
        }
    }

    pub fn cycle_tab(&mut self, forward: bool) {
        let n = self.tabs.len();
        if n == 0 {
            self.active_tab = None;
            return;
        }
        // The section page counts as position 0.
        let pos = self.active_tab.map_or(0, |i| i + 1);
        let next = if forward {
            (pos + 1) % (n + 1)
        } else {
            (pos + n) % (n + 1)
        };
        self.active_tab = if next == 0 { None } else { Some(next - 1) };
    }

    /// Put the tabs in a new order (dragged in the tab bar). The active tab stays active.
    /// Ignored unless `order` holds exactly the open tabs.
    pub fn reorder_tabs(&mut self, order: Vec<TabRef>) {
        let same_set = order.len() == self.tabs.len() && order.iter().all(|t| self.tabs.contains(t));
        if !same_set || order == self.tabs {
            return;
        }
        let active = self.active_tab.and_then(|i| self.tabs.get(i).cloned());
        self.tabs = order;
        self.active_tab = active.and_then(|t| self.tabs.iter().position(|x| *x == t));
    }
}

pub fn load_ui_states(p: &Path) -> HashMap<AccountId, UiState> {
    let raw: HashMap<String, UiState> = std::fs::read_to_string(p)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default();
    raw.into_iter()
        .filter_map(|(k, v)| k.parse().ok().map(|id| (AccountId(id), v)))
        .collect()
}

pub fn save_ui_states(p: &Path, s: &HashMap<AccountId, UiState>) -> std::io::Result<()> {
    let raw: HashMap<String, &UiState> = s.iter().map(|(k, v)| (k.0.to_string(), v)).collect();
    write_json(p, &raw)
}

fn write_json<T: Serialize>(p: &Path, v: &T) -> std::io::Result<()> {
    if let Some(d) = p.parent() {
        std::fs::create_dir_all(d)?;
    }
    let tmp = p.with_extension("json.tmp");
    std::fs::write(&tmp, serde_json::to_vec_pretty(v).unwrap_or_default())?;
    std::fs::rename(tmp, p)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tabs_open_close_reopen_cycle() {
        let mut s = UiState::fresh();
        s.open_tab(TabRef::Experience { place_id: 1 }, true);
        s.open_tab(TabRef::Profile { user_id: 2 }, true);
        s.open_tab(TabRef::Experience { place_id: 1 }, true);
        assert_eq!(s.tabs.len(), 2);
        assert_eq!(s.active_tab, Some(0));
        s.cycle_tab(true);
        assert_eq!(s.active_tab, Some(1));
        s.cycle_tab(true);
        assert_eq!(s.active_tab, None);
        s.cycle_tab(false);
        assert_eq!(s.active_tab, Some(1));
        s.close_tab(1);
        assert_eq!(s.active_tab, Some(0));
        s.reopen_tab();
        assert_eq!(s.tabs.len(), 2);
        assert_eq!(s.active_tab, Some(1));
    }

    #[test]
    fn reorder_keeps_active_tab() {
        let a = TabRef::Experience { place_id: 1 };
        let b = TabRef::Profile { user_id: 2 };
        let c = TabRef::Experience { place_id: 3 };
        let mut s = UiState::fresh();
        for t in [&a, &b, &c] {
            s.open_tab(t.clone(), true);
        }
        s.active_tab = Some(0);
        s.reorder_tabs(vec![b.clone(), c.clone(), a.clone()]);
        assert_eq!(s.tabs, vec![b.clone(), c.clone(), a.clone()]);
        assert_eq!(s.active_tab, Some(2));
        // Not a permutation of the open tabs: ignored.
        s.reorder_tabs(vec![a.clone(), b.clone()]);
        assert_eq!(s.tabs, vec![b, c, a]);
    }

    #[test]
    fn ui_state_round_trip() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("ui.json");
        let mut m = HashMap::new();
        let mut s = UiState::fresh();
        s.scroll.insert("home".into(), 120.0);
        m.insert(AccountId(5), s);
        save_ui_states(&p, &m).unwrap();
        assert_eq!(load_ui_states(&p), m);
    }
}

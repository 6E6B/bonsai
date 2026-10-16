//! App settings (no secrets) and per-account UI state.

use std::collections::HashMap;
use std::path::Path;

use rbx_api::AccountId;
use serde::{Deserialize, Serialize};

use crate::commands::{CommandId, Shortcut};

/// Which of the two appearance themes is on.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    /// The OS picks between [`Appearance::dark`] and [`Appearance::light`].
    #[default]
    System,
    Dark,
    Light,
}

impl Mode {
    pub const ALL: [Mode; 3] = [Mode::System, Mode::Dark, Mode::Light];
    pub fn name(self) -> &'static str {
        match self {
            Mode::System => "system",
            Mode::Dark => "dark",
            Mode::Light => "light",
        }
    }
    pub fn from_name(s: &str) -> Option<Mode> {
        Mode::ALL
            .into_iter()
            .find(|t| t.name().eq_ignore_ascii_case(s.trim()))
    }
}

/// The theme for each base, by id (see [`crate::themes`]), and which one is on.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Appearance {
    pub mode: Mode,
    pub dark: String,
    pub light: String,
    /// A picture behind every page, under whichever theme is on.
    pub background: Option<Background>,
}

/// A picture shown behind the app. Pages and panels turn see-through over it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Background {
    /// File name in `Paths::backgrounds_dir()` (see [`crate::backgrounds::valid_file`]).
    pub file: String,
    /// How strongly the picture shows through the theme's background, 0–100.
    pub image: u8,
    /// How opaque pages and panels stay over the picture, 0–100.
    pub panels: u8,
    /// Blur radius in px behind see-through cards, 0–40; 0 leaves them clear.
    pub frost: u8,
    /// Effects drawn on the picture. See [`Background::LEVELS`] for each range.
    pub blur: u8,
    pub dim: u8,
    pub saturation: u8,
    pub tint: u8,
    pub vignette: u8,
    pub scanlines: u8,
    pub grain: u8,
}

impl Background {
    /// Every adjustable level by name, with its maximum.
    pub const LEVELS: [(&'static str, u8); 10] = [
        ("image", 100),
        ("panels", 100),
        ("frost", 40),
        ("blur", 40),
        ("dim", 100),
        ("saturation", 200),
        ("tint", 100),
        ("vignette", 100),
        ("scanlines", 100),
        ("grain", 100),
    ];

    /// The level named `name`, as listed in [`Background::LEVELS`].
    pub fn level_mut(&mut self, name: &str) -> Option<&mut u8> {
        Some(match name {
            "image" => &mut self.image,
            "panels" => &mut self.panels,
            "frost" => &mut self.frost,
            "blur" => &mut self.blur,
            "dim" => &mut self.dim,
            "saturation" => &mut self.saturation,
            "tint" => &mut self.tint,
            "vignette" => &mut self.vignette,
            "scanlines" => &mut self.scanlines,
            "grain" => &mut self.grain,
            _ => return None,
        })
    }

    /// Turns every effect off, keeping the picture and its strength.
    pub fn reset_effects(&mut self) {
        *self = Background {
            file: std::mem::take(&mut self.file),
            image: self.image,
            panels: self.panels,
            frost: self.frost,
            ..Background::default()
        };
    }
}

impl Default for Background {
    fn default() -> Self {
        Background {
            file: String::new(),
            image: 70,
            panels: 75,
            frost: 0,
            blur: 0,
            dim: 0,
            saturation: 100,
            tint: 0,
            vignette: 0,
            scanlines: 0,
            grain: 0,
        }
    }
}

impl Default for Appearance {
    fn default() -> Self {
        Appearance {
            mode: Mode::System,
            dark: "dark".into(),
            light: "light".into(),
            background: None,
        }
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
    /// A new sale of something you sell.
    pub sales: bool,
}

impl NotifyPrefs {
    pub const KINDS: [&'static str; 6] = [
        "friend_joins",
        "trades",
        "chats",
        "friend_requests",
        "experience_updates",
        "sales",
    ];
    pub fn get(&self, kind: &str) -> Option<bool> {
        Some(match kind {
            "friend_joins" => self.friend_joins,
            "trades" => self.trades,
            "chats" => self.chats,
            "friend_requests" => self.friend_requests,
            "experience_updates" => self.experience_updates,
            "sales" => self.sales,
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
            "sales" => &mut self.sales,
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
            sales: true,
        }
    }
}

/// The sidebar as the user arranged it. Screens are named by their `nav.*` command without the
/// prefix (`friends`). Grouping is the UI's; this only keeps the order and what is hidden.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SidebarPrefs {
    /// Screens in the order they were last arranged. Screens missing here keep their default
    /// place.
    pub order: Vec<String>,
    pub hidden: Vec<String>,
}

impl Default for SidebarPrefs {
    fn default() -> Self {
        Self {
            order: Vec::new(),
            // The legacy inbox; chat covers most of it. Still in the command bar (G N).
            hidden: vec!["messages".into()],
        }
    }
}

impl SidebarPrefs {
    const MAX: usize = 64;

    fn valid(screen: &str) -> bool {
        !screen.is_empty()
            && screen.len() <= 32
            && screen
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
    }

    /// Replaces the order. Duplicates keep their first place. Errors on a malformed name.
    pub fn set_order<'a>(&mut self, order: impl IntoIterator<Item = &'a str>) -> Result<(), String> {
        let mut out: Vec<String> = Vec::new();
        for s in order {
            if !Self::valid(s) {
                return Err(format!("\"{s}\" isn't a screen."));
            }
            if !out.iter().any(|x| x == s) {
                out.push(s.to_owned());
            }
        }
        out.truncate(Self::MAX);
        self.order = out;
        Ok(())
    }

    pub fn is_hidden(&self, screen: &str) -> bool {
        self.hidden.iter().any(|x| x == screen)
    }

    /// Shows or hides one screen. Errors on a malformed name.
    pub fn set_hidden(&mut self, screen: &str, hidden: bool) -> Result<(), String> {
        if !Self::valid(screen) {
            return Err(format!("\"{screen}\" isn't a screen."));
        }
        self.hidden.retain(|x| x != screen);
        if hidden && self.hidden.len() < Self::MAX {
            self.hidden.push(screen.to_owned());
        }
        Ok(())
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
    pub appearance: Appearance,
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
    pub sidebar: SidebarPrefs,
    /// Avatar viewers open in 3D instead of on the image. Off by default: the engine is heavy.
    pub avatar_3d: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            appearance: Appearance::default(),
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
            sidebar: SidebarPrefs::default(),
            avatar_3d: false,
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
    fn sidebar_order_and_hidden() {
        let mut p = SidebarPrefs::default();
        assert!(p.is_hidden("messages"));
        p.set_order(["trades", "home", "trades", "friends"]).unwrap();
        assert_eq!(p.order, ["trades", "home", "friends"]);
        assert!(p.set_order(["Home"]).is_err());
        assert_eq!(p.order, ["trades", "home", "friends"]);
        p.set_hidden("friends", true).unwrap();
        p.set_hidden("friends", true).unwrap();
        p.set_hidden("messages", false).unwrap();
        assert_eq!(p.hidden, ["friends"]);
        assert!(p.set_hidden("", true).is_err());
    }

    #[test]
    fn sidebar_defaults_when_missing() {
        let s: Settings = serde_json::from_str(r#"{"appearance":{"mode":"Dark"}}"#).unwrap();
        assert_eq!(s.sidebar, SidebarPrefs::default());
        assert_eq!(s.appearance.mode, Mode::Dark);
        assert_eq!(s.appearance.light, "light");
        let s: Settings = serde_json::from_str(r#"{"sidebar":{"order":["home"]}}"#).unwrap();
        assert_eq!(s.sidebar.order, ["home"]);
        assert!(s.sidebar.is_hidden("messages"));
    }

    #[test]
    fn background_effects() {
        let mut b: Background = serde_json::from_str(r#"{"file":"a.png","image":40}"#).unwrap();
        assert_eq!((b.image, b.panels, b.saturation, b.blur), (40, 75, 100, 0));
        *b.level_mut("blur").unwrap() = 12;
        *b.level_mut("frost").unwrap() = 8;
        *b.level_mut("saturation").unwrap() = 0;
        assert!(b.level_mut("nope").is_none());
        b.reset_effects();
        assert_eq!(
            (b.file.as_str(), b.image, b.frost, b.blur, b.saturation),
            ("a.png", 40, 8, 0, 100)
        );
        for (name, _) in Background::LEVELS {
            assert!(b.level_mut(name).is_some(), "{name}");
        }
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

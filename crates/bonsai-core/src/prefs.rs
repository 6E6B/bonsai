//! The settings a user can change one at a time, with titles for the settings screen and the
//! command bar. `settings.set` and `settings.reset` go through here, so every setting listed
//! has one way to be read, changed and put back.
//!
//! Values cross as text: `on`/`off` for switches, the option's value for choices, a whole
//! number for ranges and the text itself for text.

use serde::{Deserialize, Serialize};

use crate::settings::{Density, Mode, SavedTabs, Settings, SidebarPrefs, valid_format_locale};

/// What kind of control a setting takes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrefKind {
    /// On or off.
    Bool,
    /// One of a short list: `(value, label)`.
    Choice(&'static [(&'static str, &'static str)]),
    /// A whole number in steps.
    Range { min: i32, max: i32, step: i32 },
    /// Free text. Empty means the default behaviour.
    Text { placeholder: &'static str },
}

/// One setting.
#[derive(Clone, Copy, Debug)]
pub struct Pref {
    /// Stable id, used in commands and deep links. Never a section name.
    pub key: &'static str,
    /// The settings section it is shown in (the UI's section ids).
    pub section: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub keywords: &'static [&'static str],
    pub kind: PrefKind,
}

const MODES: &[(&str, &str)] = &[("system", "System"), ("dark", "Dark"), ("light", "Light")];
const DENSITIES: &[(&str, &str)] = &[("comfortable", "Comfortable"), ("compact", "Compact")];
/// Window zoom in percent. The UI's zoom commands step through these.
pub const ZOOMS: &[(&str, &str)] = &[
    ("67", "67%"),
    ("75", "75%"),
    ("80", "80%"),
    ("90", "90%"),
    ("100", "100%"),
    ("110", "110%"),
    ("125", "125%"),
    ("150", "150%"),
    ("175", "175%"),
    ("200", "200%"),
];
const CACHE_DAYS: &[(&str, &str)] = &[
    ("7", "1 week"),
    ("30", "1 month"),
    ("90", "3 months"),
    ("365", "1 year"),
];
const IMAGE_CACHE: &[(&str, &str)] = &[
    ("256", "256 MB"),
    ("512", "512 MB"),
    ("1024", "1 GB"),
    ("2048", "2 GB"),
    ("4096", "4 GB"),
];
const VAULT_TIMEOUTS: &[(&str, &str)] = &[
    ("1", "1 minute"),
    ("5", "5 minutes"),
    ("15", "15 minutes"),
    ("30", "30 minutes"),
    ("60", "1 hour"),
];
const MOTIONS: &[(&str, &str)] = &[("system", "System"), ("reduce", "Reduced"), ("full", "Full")];
const TIME_FORMATS: &[(&str, &str)] = &[("auto", "Automatic"), ("12h", "12-hour"), ("24h", "24-hour")];
/// Screens a launch can open, by route name, and `last` for the tabs from last time.
pub const START_SCREENS: &[(&str, &str)] = &[
    ("home", "Home"),
    ("friends", "Friends"),
    ("discover", "Discover"),
    ("marketplace", "Marketplace"),
    ("avatar", "Avatar"),
    ("inventory", "Inventory"),
    ("trades", "Trades"),
    ("chat", "Chat"),
    ("communities", "Communities"),
    ("last", "Tabs from last time"),
];
const SIDEBAR_MODES: &[(&str, &str)] = &[
    ("auto", "Automatic"),
    ("expanded", "Expanded"),
    ("collapsed", "Collapsed"),
];
/// Whole hours of the day for quiet hours.
const HOURS: &[(&str, &str)] = &[
    ("0", "00:00"),
    ("1", "01:00"),
    ("2", "02:00"),
    ("3", "03:00"),
    ("4", "04:00"),
    ("5", "05:00"),
    ("6", "06:00"),
    ("7", "07:00"),
    ("8", "08:00"),
    ("9", "09:00"),
    ("10", "10:00"),
    ("11", "11:00"),
    ("12", "12:00"),
    ("13", "13:00"),
    ("14", "14:00"),
    ("15", "15:00"),
    ("16", "16:00"),
    ("17", "17:00"),
    ("18", "18:00"),
    ("19", "19:00"),
    ("20", "20:00"),
    ("21", "21:00"),
    ("22", "22:00"),
    ("23", "23:00"),
];
const AVATAR_VIEWS: &[(&str, &str)] = &[("2d", "Image"), ("3d", "3D")];

const fn pref(
    key: &'static str,
    section: &'static str,
    title: &'static str,
    description: &'static str,
    keywords: &'static [&'static str],
    kind: PrefKind,
) -> Pref {
    Pref {
        key,
        section,
        title,
        description,
        keywords,
        kind,
    }
}

/// Every setting, in the order the settings screen shows them.
pub const PREFS: &[Pref] = &[
    pref(
        "mode",
        "appearance",
        "Mode",
        "System switches between your dark and light themes with the OS.",
        &["theme", "dark", "light", "system"],
        PrefKind::Choice(MODES),
    ),
    pref(
        "density",
        "appearance",
        "Density",
        "Compact makes buttons and fields shorter.",
        &["compact", "comfortable", "spacing"],
        PrefKind::Choice(DENSITIES),
    ),
    pref(
        "ui_scale",
        "appearance",
        "Zoom",
        "Scales the whole window. Ctrl+= and Ctrl+- step through these.",
        &["zoom", "scale", "size", "larger", "smaller"],
        PrefKind::Choice(ZOOMS),
    ),
    pref(
        "motion",
        "appearance",
        "Motion",
        "Reduced turns off transitions and animations, except loading spinners. System follows the OS.",
        &["animation", "reduce motion", "transition", "accessibility"],
        PrefKind::Choice(MOTIONS),
    ),
    pref(
        "status_bar",
        "appearance",
        "Status bar",
        "The line at the bottom with the account, balance, running jobs and zoom.",
        &["footer", "bottom", "bar", "hide"],
        PrefKind::Bool,
    ),
    pref(
        "player_command",
        "general",
        "Launch Roblox with",
        "How Join opens the game. In a command, {uri} is replaced with the join link.",
        &["launch", "player", "sober", "flatpak", "wine"],
        PrefKind::Text {
            placeholder: "System default",
        },
    ),
    pref(
        "avatar_view",
        "general",
        "Avatars",
        "How avatars and items open. 3D shows a spinner until the model loads.",
        &["3d", "2d", "model", "image", "avatar"],
        PrefKind::Choice(AVATAR_VIEWS),
    ),
    pref(
        "format_locale",
        "general",
        "Number and date format",
        "Applies to counts, prices, dates and times like \"5m ago\".",
        &["language", "locale", "region", "date", "number"],
        PrefKind::Text {
            placeholder: "System",
        },
    ),
    pref(
        "time_format",
        "general",
        "Clock",
        "Automatic uses the number and date format's clock.",
        &["time", "12 hour", "24 hour", "am", "pm", "clock"],
        PrefKind::Choice(TIME_FORMATS),
    ),
    pref(
        "relative_dates",
        "general",
        "Relative times",
        "Show \"5m ago\" for the last week. Off shows the date and time.",
        &["ago", "date", "time", "relative", "absolute"],
        PrefKind::Bool,
    ),
    pref(
        "start_screen",
        "layout",
        "Open on launch",
        "Tabs from last time reopens the tabs that were open when the app closed.",
        &["start", "startup", "launch", "restore", "tabs", "session"],
        PrefKind::Choice(START_SCREENS),
    ),
    pref(
        "sidebar_mode",
        "layout",
        "Sidebar",
        "Automatic collapses it to icons when the window is narrow. Ctrl+B switches it.",
        &["sidebar", "collapse", "icons", "narrow"],
        PrefKind::Choice(SIDEBAR_MODES),
    ),
    pref(
        "drawer",
        "layout",
        "Friends and chat drawer",
        "The panel on the right with friends and recent chats.",
        &["drawer", "panel", "friends", "chat", "right"],
        PrefKind::Bool,
    ),
    pref(
        "home.friends",
        "layout",
        "Friends playing",
        "",
        &["home", "friends", "playing", "row"],
        PrefKind::Bool,
    ),
    pref(
        "home.continue_playing",
        "layout",
        "Continue",
        "Games you played recently.",
        &["home", "recent", "continue", "row"],
        PrefKind::Bool,
    ),
    pref(
        "home.favorites",
        "layout",
        "Favorites",
        "",
        &["home", "favorites", "row"],
        PrefKind::Bool,
    ),
    pref(
        "home.recommended",
        "layout",
        "Recommended",
        "Roblox's Home feed: picks, charts and topics.",
        &["home", "recommended", "feed", "sorts", "row"],
        PrefKind::Bool,
    ),
    pref(
        "notify.friend_joins",
        "notifications",
        "Friends joining games",
        "A desktop notification when a friend joins an experience.",
        &["friends", "presence", "join"],
        PrefKind::Bool,
    ),
    pref(
        "notify.friend_requests",
        "notifications",
        "Friend requests",
        "",
        &["friends", "request"],
        PrefKind::Bool,
    ),
    pref(
        "notify.chats",
        "notifications",
        "Chat messages",
        "",
        &["chat", "message"],
        PrefKind::Bool,
    ),
    pref(
        "notify.trades",
        "notifications",
        "Trade offers",
        "",
        &["trade", "offer"],
        PrefKind::Bool,
    ),
    pref(
        "notify.sales",
        "notifications",
        "Sales",
        "When someone buys something you sell. Checked every few minutes.",
        &["sale", "sold", "robux"],
        PrefKind::Bool,
    ),
    pref(
        "notify.experience_updates",
        "notifications",
        "Experience updates",
        "From experiences you turned Notify on for.",
        &["experience", "game", "update"],
        PrefKind::Bool,
    ),
    pref(
        "notify.plugins",
        "notifications",
        "Plugin alerts",
        "From plugins you allowed to notify, when something they watch happens.",
        &["plugin", "extension", "alert"],
        PrefKind::Bool,
    ),
    pref(
        "quiet_hours",
        "notifications",
        "Quiet hours",
        "No desktop notifications between the hours below. Badges and lists still update.",
        &["quiet", "do not disturb", "dnd", "night", "silent", "schedule"],
        PrefKind::Bool,
    ),
    pref(
        "quiet_start",
        "notifications",
        "Quiet from",
        "",
        &["quiet", "start", "night"],
        PrefKind::Choice(HOURS),
    ),
    pref(
        "quiet_end",
        "notifications",
        "Quiet until",
        "",
        &["quiet", "end", "morning"],
        PrefKind::Choice(HOURS),
    ),
    pref(
        "mute_in_game",
        "notifications",
        "Mute while playing",
        "No desktop notifications while the active account is in a game.",
        &["game", "playing", "mute", "silent", "do not disturb"],
        PrefKind::Bool,
    ),
    pref(
        "notify_sound",
        "notifications",
        "Sound",
        "Asks the system to play its message sound. Some desktops always or never play one.",
        &["sound", "audio", "chime", "ding"],
        PrefKind::Bool,
    ),
    pref(
        "notify_in_front",
        "notifications",
        "While the app is in front",
        "Off only notifies while the window is in the background, since badges show the rest.",
        &["focused", "foreground", "always"],
        PrefKind::Bool,
    ),
    pref(
        "tray",
        "desktop",
        "Tray icon",
        "An icon in the system tray to show the window or quit. Some Linux desktops need an extension to show tray icons.",
        &["tray", "system tray", "notification area", "menu bar", "icon"],
        PrefKind::Bool,
    ),
    pref(
        "close_to_tray",
        "desktop",
        "Close to tray",
        "Closing the window keeps the app running in the tray, with notifications. Quit from the tray menu or the command bar.",
        &["close", "minimize", "background", "keep running", "tray"],
        PrefKind::Bool,
    ),
    pref(
        "start_on_login",
        "desktop",
        "Start on login",
        "Open the app when you log in to this computer.",
        &["autostart", "startup", "login", "boot", "launch"],
        PrefKind::Bool,
    ),
    pref(
        "update_downloads",
        "desktop",
        "Download updates automatically",
        "After a check finds an update, download and verify it in the background. Installing still waits for you.",
        &["updates", "download", "background", "automatic"],
        PrefKind::Bool,
    ),
    pref(
        "update_checks",
        "desktop",
        "Check for updates automatically",
        "Check on startup and every six hours. Download and restart only when you choose.",
        &["updates", "upgrade", "version", "release", "automatic"],
        PrefKind::Bool,
    ),
    pref(
        "start_hidden",
        "desktop",
        "Start in the tray",
        "When started on login, stay in the tray instead of opening the window.",
        &["hidden", "minimized", "silent", "tray", "startup"],
        PrefKind::Bool,
    ),
    pref(
        "item_values",
        "data",
        "Item values",
        "Show Rolimons values, demand and trends for limiteds, the Values screen, and trading stats and last online on profiles. Refreshed every two minutes while a screen shows them.",
        &["values", "rolimons", "worth", "rap"],
        PrefKind::Bool,
    ),
    pref(
        "wiki",
        "data",
        "Wiki articles",
        "Show Roblox Wiki articles from Fandom on items, experiences and profiles, and the Wiki screen. An article is fetched when you open a page it could be about, and kept for 12 hours.",
        &["wiki", "fandom", "article", "trivia", "history"],
        PrefKind::Bool,
    ),
    pref(
        "cache_days",
        "data",
        "Keep unused data for",
        "Saved screens and images nobody opened for this long are deleted. Checked every few hours.",
        &["cache", "keep", "age", "days", "storage", "disk"],
        PrefKind::Choice(CACHE_DAYS),
    ),
    pref(
        "image_cache_mb",
        "data",
        "Image cache size",
        "Thumbnails and 3D models on disk. The least recently used go first past this.",
        &[
            "cache",
            "images",
            "thumbnails",
            "size",
            "limit",
            "storage",
            "disk",
        ],
        PrefKind::Choice(IMAGE_CACHE),
    ),
    pref(
        "background_accounts",
        "accounts",
        "Keep other accounts live",
        "Chat, presence and unread counts for every signed-in account, not only the active one.",
        &["background", "realtime", "unread", "accounts", "presence"],
        PrefKind::Bool,
    ),
    pref(
        "vault_idle_minutes",
        "accounts",
        "Lock after inactivity",
        "When PIN protection is on, lock after this long without input in Bonsai. System locking and sleep also lock Bonsai.",
        &["pin", "vault", "lock", "timeout", "idle", "security"],
        PrefKind::Choice(VAULT_TIMEOUTS),
    ),
    pref(
        "account_accent",
        "accounts",
        "Account color as accent",
        "Buttons, links and focus rings take the active account's color tag. Accounts with no color keep the theme's.",
        &["accent", "color", "colour", "tag", "theme"],
        PrefKind::Bool,
    ),
];

/// The setting with this key.
pub fn find(key: &str) -> Option<&'static Pref> {
    PREFS.iter().find(|p| p.key == key)
}

const fn on_off(b: bool) -> &'static str {
    if b { "on" } else { "off" }
}

fn parse_bool(v: &str) -> Option<bool> {
    match v.trim().to_ascii_lowercase().as_str() {
        "on" | "true" | "yes" | "1" => Some(true),
        "off" | "false" | "no" | "0" => Some(false),
        _ => None,
    }
}

/// The current value of `key` as text, or `None` for an unknown key.
pub fn get(s: &Settings, key: &str) -> Option<String> {
    if let Some(kind) = key.strip_prefix("notify.") {
        return s.notify.get(kind).map(|b| on_off(b).to_owned());
    }
    Some(match key {
        "mode" => s.appearance.mode.name().to_owned(),
        "density" => s.density.name().to_owned(),
        "ui_scale" => ((s.ui_scale * 100.0).round() as i32).to_string(),
        "background_accounts" => on_off(s.background_accounts).to_owned(),
        "account_accent" => on_off(s.account_accent).to_owned(),
        "vault_idle_minutes" => s.vault_idle_minutes.clamp(1, 60).to_string(),
        "player_command" => s.player_command.clone().unwrap_or_default(),
        "avatar_view" => if s.avatar_3d { "3d" } else { "2d" }.to_owned(),
        "format_locale" => s.format_locale.clone(),
        "item_values" => on_off(s.item_values).to_owned(),
        "wiki" => on_off(s.wiki).to_owned(),
        "motion" => s.motion.clone(),
        "time_format" => s.time_format.clone(),
        "relative_dates" => on_off(s.relative_dates).to_owned(),
        "status_bar" => on_off(s.status_bar).to_owned(),
        "start_screen" => s.start_screen.clone(),
        "sidebar_mode" => s.sidebar_mode.clone(),
        "drawer" => on_off(s.drawer).to_owned(),
        "home.friends" => on_off(s.home.friends).to_owned(),
        "home.continue_playing" => on_off(s.home.continue_playing).to_owned(),
        "home.favorites" => on_off(s.home.favorites).to_owned(),
        "home.recommended" => on_off(s.home.recommended).to_owned(),
        "quiet_hours" => on_off(s.quiet.on).to_owned(),
        "quiet_start" => s.quiet.start.to_string(),
        "quiet_end" => s.quiet.end.to_string(),
        "mute_in_game" => on_off(s.mute_in_game).to_owned(),
        "notify_sound" => on_off(s.notify_sound).to_owned(),
        "notify_in_front" => on_off(s.notify_in_front).to_owned(),
        "cache_days" => s.cache_days.to_string(),
        "image_cache_mb" => s.image_cache_mb.to_string(),
        "tray" => on_off(s.tray).to_owned(),
        "close_to_tray" => on_off(s.close_to_tray).to_owned(),
        "start_on_login" => on_off(s.start_on_login).to_owned(),
        "start_hidden" => on_off(s.start_hidden).to_owned(),
        "update_checks" => on_off(s.update_checks).to_owned(),
        "update_downloads" => on_off(s.update_downloads).to_owned(),
        _ => return None,
    })
}

/// The value `key` has in a fresh install.
pub fn default_value(key: &str) -> Option<String> {
    get(&Settings::default(), key)
}

/// Sets `key` from text. Errors say what to type instead.
pub fn set(s: &mut Settings, key: &str, value: &str) -> Result<(), String> {
    let p = find(key).ok_or_else(|| format!("\"{key}\" isn't a setting."))?;
    let value = value.trim();
    let bool = || parse_bool(value).ok_or_else(|| "Say on or off.".to_owned());
    if let PrefKind::Choice(options) = p.kind {
        let v = options
            .iter()
            .find(|(v, _)| v.eq_ignore_ascii_case(value))
            .map(|(v, _)| *v)
            .ok_or_else(|| {
                let names: Vec<&str> = options.iter().map(|(v, _)| *v).collect();
                format!("Pick one of: {}.", names.join(", "))
            })?;
        match key {
            "mode" => s.appearance.mode = Mode::from_name(v).unwrap_or_default(),
            "density" => s.density = Density::from_name(v).unwrap_or_default(),
            "ui_scale" => s.ui_scale = v.parse::<f32>().unwrap_or(100.0) / 100.0,
            "avatar_view" => s.avatar_3d = v == "3d",
            "motion" => v.clone_into(&mut s.motion),
            "time_format" => v.clone_into(&mut s.time_format),
            "start_screen" => v.clone_into(&mut s.start_screen),
            "sidebar_mode" => v.clone_into(&mut s.sidebar_mode),
            "quiet_start" => s.quiet.start = v.parse().unwrap_or(22),
            "quiet_end" => s.quiet.end = v.parse().unwrap_or(8),
            "cache_days" => s.cache_days = v.parse().unwrap_or(30),
            "image_cache_mb" => s.image_cache_mb = v.parse().unwrap_or(1024),
            "vault_idle_minutes" => s.vault_idle_minutes = v.parse().unwrap_or(5),
            _ => unreachable!("choice without a setter: {key}"),
        }
        return Ok(());
    }
    if let Some(kind) = key.strip_prefix("notify.") {
        s.notify.set(kind, bool()?);
        return Ok(());
    }
    match key {
        "item_values" => s.item_values = bool()?,
        "wiki" => s.wiki = bool()?,
        "background_accounts" => s.background_accounts = bool()?,
        "account_accent" => s.account_accent = bool()?,
        "relative_dates" => s.relative_dates = bool()?,
        "status_bar" => s.status_bar = bool()?,
        "drawer" => s.drawer = bool()?,
        "home.friends" => s.home.friends = bool()?,
        "home.continue_playing" => s.home.continue_playing = bool()?,
        "home.favorites" => s.home.favorites = bool()?,
        "home.recommended" => s.home.recommended = bool()?,
        "quiet_hours" => s.quiet.on = bool()?,
        "mute_in_game" => s.mute_in_game = bool()?,
        "notify_sound" => s.notify_sound = bool()?,
        "notify_in_front" => s.notify_in_front = bool()?,
        "tray" => s.tray = bool()?,
        "close_to_tray" => s.close_to_tray = bool()?,
        "start_on_login" => s.start_on_login = bool()?,
        "start_hidden" => s.start_hidden = bool()?,
        "update_checks" => s.update_checks = bool()?,
        "update_downloads" => s.update_downloads = bool()?,
        "player_command" => s.player_command = Some(value.to_owned()).filter(|c| !c.is_empty()),
        "format_locale" => {
            if !valid_format_locale(value) {
                return Err("Use a language tag like en-US or de-DE.".into());
            }
            value.clone_into(&mut s.format_locale);
        }
        _ => unreachable!("setting without a setter: {key}"),
    }
    Ok(())
}

/// Runs a `settings.set` argument: `<key> <value>`. A switch without a value flips.
pub fn apply(s: &mut Settings, arg: &str) -> Result<&'static Pref, String> {
    let arg = arg.trim();
    let (key, value) = arg.split_once(char::is_whitespace).unwrap_or((arg, ""));
    let p = find(key).ok_or_else(|| format!("\"{key}\" isn't a setting."))?;
    let value = value.trim();
    if value.is_empty() && p.kind == PrefKind::Bool {
        let on = get(s, key).as_deref() == Some("on");
        set(s, key, on_off(!on))?;
    } else {
        set(s, key, value)?;
    }
    Ok(p)
}

/// Settings not at their defaults, as `(key, value)`, for diagnostics. The launch command
/// only says whether it is set, since it can hold paths.
pub fn changed(s: &Settings) -> Vec<(&'static str, String)> {
    let fresh = Settings::default();
    PREFS
        .iter()
        .filter_map(|p| {
            let v = get(s, p.key)?;
            if get(&fresh, p.key).as_deref() == Some(v.as_str()) {
                return None;
            }
            let v = if p.key == "player_command" {
                "(set)".to_owned()
            } else {
                v
            };
            Some((p.key, v))
        })
        .collect()
}

/// Marks a file `export` wrote, so importing some other JSON file fails instead of resetting
/// everything to defaults.
const EXPORT_FORMAT: &str = "bonsai-settings";

#[derive(Serialize, Deserialize)]
struct Exported {
    format: String,
    version: u32,
    settings: Settings,
}

/// Settings as a file to take to another computer. The launch command and saved tabs belong to
/// this one and are left out.
pub fn export(s: &Settings) -> String {
    let mut settings = s.clone();
    settings.player_command = None;
    settings.session = SavedTabs::default();
    let file = Exported {
        format: EXPORT_FORMAT.to_owned(),
        version: 1,
        settings,
    };
    serde_json::to_string_pretty(&file).unwrap_or_default()
}

/// Takes the settings from a file `export` wrote. Each one is checked as if typed into
/// `settings.set` and skipped if bad. Shortcuts and the sidebar come too; the launch command,
/// themes, background and saved tabs stay as they are, since a file from someone else must
/// not choose what the app runs. Returns how many settings changed.
pub fn import(s: &mut Settings, text: &str) -> Result<usize, String> {
    let file: Exported = serde_json::from_str(text)
        .ok()
        .filter(|f: &Exported| f.format == EXPORT_FORMAT)
        .ok_or_else(|| "That isn't a settings file from this app.".to_owned())?;
    let new = file.settings;
    let mut changed = 0;
    for p in PREFS.iter().filter(|p| p.key != "player_command") {
        let Some(v) = get(&new, p.key) else { continue };
        if get(s, p.key).as_deref() != Some(v.as_str()) && set(s, p.key, &v).is_ok() {
            changed += 1;
        }
    }
    if s.shortcuts != new.shortcuts {
        s.shortcuts = new.shortcuts;
        changed += 1;
    }
    let mut plugin_shortcuts = new.plugin_shortcuts;
    plugin_shortcuts.retain(|k, v| {
        !v.is_empty()
            && matches!(
                crate::commands::CommandRef::parse(k),
                Some(crate::commands::CommandRef::Plugin(_))
            )
    });
    if s.plugin_shortcuts != plugin_shortcuts {
        s.plugin_shortcuts = plugin_shortcuts;
        changed += 1;
    }
    let mut sidebar = SidebarPrefs::default();
    let order_ok = sidebar
        .set_order(new.sidebar.order.iter().map(String::as_str))
        .is_ok();
    sidebar.hidden.clear();
    for h in &new.sidebar.hidden {
        let _ = sidebar.set_hidden(h, true);
    }
    if order_ok && s.sidebar != sidebar {
        s.sidebar = sidebar;
        changed += 1;
    }
    Ok(changed)
}

/// The settings `target` names: one key, every setting in a section, or `all`.
pub fn matching(target: &str) -> Vec<&'static Pref> {
    let t = target.trim();
    PREFS
        .iter()
        .filter(|p| t == "all" || p.key == t || p.section == t)
        .collect()
}

/// Puts back the defaults for `target` (see [`matching`]). Returns how many changed.
pub fn reset(s: &mut Settings, target: &str) -> Result<usize, String> {
    let prefs = matching(target);
    if prefs.is_empty() {
        return Err(format!("\"{}\" isn't a setting or a section.", target.trim()));
    }
    let fresh = Settings::default();
    let mut changed = 0;
    for p in prefs {
        let d = get(&fresh, p.key).unwrap_or_default();
        if get(s, p.key).as_deref() != Some(d.as_str()) {
            set(s, p.key, &d)?;
            changed += 1;
        }
    }
    Ok(changed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::NotifyPrefs;

    #[test]
    fn every_pref_round_trips_its_default() {
        for p in PREFS {
            let d = default_value(p.key).unwrap_or_else(|| panic!("no getter for {}", p.key));
            let mut s = Settings::default();
            set(&mut s, p.key, &d).unwrap_or_else(|e| panic!("{}: {e}", p.key));
            assert_eq!(s, Settings::default(), "{}", p.key);
        }
    }

    #[test]
    fn keys_are_unique_and_not_sections() {
        for (i, p) in PREFS.iter().enumerate() {
            assert!(PREFS[i + 1..].iter().all(|q| q.key != p.key), "{}", p.key);
            assert!(PREFS.iter().all(|q| q.section != p.key), "{}", p.key);
            assert_ne!(p.key, "all");
        }
    }

    #[test]
    fn every_notify_kind_is_listed() {
        for k in NotifyPrefs::KINDS {
            assert!(find(&format!("notify.{k}")).is_some(), "{k}");
        }
    }

    #[test]
    fn apply_sets_and_flips() {
        let mut s = Settings::default();
        apply(&mut s, "density compact").unwrap();
        assert_eq!(s.density, Density::Compact);
        apply(&mut s, "mode DARK").unwrap();
        assert_eq!(s.appearance.mode, Mode::Dark);
        apply(&mut s, "item_values").unwrap();
        assert!(s.item_values);
        apply(&mut s, "item_values").unwrap();
        assert!(!s.item_values);
        apply(&mut s, "wiki on").unwrap();
        assert!(s.wiki);
        apply(&mut s, "notify.sales off").unwrap();
        assert!(!s.notify.sales);
        apply(&mut s, "player_command flatpak run org.vinegarhq.Sober {uri}").unwrap();
        assert_eq!(
            s.player_command.as_deref(),
            Some("flatpak run org.vinegarhq.Sober {uri}")
        );
        apply(&mut s, "player_command").unwrap();
        assert_eq!(s.player_command, None);
        apply(&mut s, "ui_scale 125").unwrap();
        assert_eq!(s.ui_scale, 1.25);
        assert_eq!(get(&s, "ui_scale").as_deref(), Some("125"));
        assert!(apply(&mut s, "ui_scale 130").is_err());
        assert!(apply(&mut s, "density huge").is_err());
        assert!(apply(&mut s, "format_locale not a tag").is_err());
        assert!(apply(&mut s, "nope on").is_err());
        assert!(apply(&mut s, "notify.chats maybe").is_err());
    }

    #[test]
    fn reset_by_key_section_and_all() {
        let mut s = Settings::default();
        apply(&mut s, "density compact").unwrap();
        apply(&mut s, "notify.sales off").unwrap();
        apply(&mut s, "notify.trades off").unwrap();
        apply(&mut s, "item_values on").unwrap();
        assert_eq!(reset(&mut s, "density"), Ok(1));
        assert_eq!(s.density, Density::Comfortable);
        assert_eq!(reset(&mut s, "notifications"), Ok(2));
        assert!(s.notify.sales && s.notify.trades);
        assert_eq!(reset(&mut s, "all"), Ok(1));
        assert_eq!(s, Settings::default());
        assert_eq!(reset(&mut s, "all"), Ok(0));
        assert!(reset(&mut s, "nope").is_err());
    }

    #[test]
    fn import_takes_back_an_export_but_not_the_launch_command() {
        let mut a = Settings::default();
        apply(&mut a, "density compact").unwrap();
        apply(&mut a, "cache_days 7").unwrap();
        apply(&mut a, "player_command evil {uri}").unwrap();
        a.sidebar.set_hidden("trades", true).unwrap();
        let ctrl_j = vec![crate::commands::Shortcut::ctrl("J")];
        a.plugin_shortcuts.insert("plugin.x.hello".into(), ctrl_j.clone());
        a.plugin_shortcuts.insert("nav.home".into(), ctrl_j.clone());
        let file = export(&a);
        assert!(!file.contains("evil"));

        let mut b = Settings::default();
        apply(&mut b, "player_command mine {uri}").unwrap();
        assert_eq!(import(&mut b, &file), Ok(4));
        // Only plugin command keys are kept.
        assert_eq!(
            b.plugin_shortcuts.keys().collect::<Vec<_>>(),
            vec!["plugin.x.hello"]
        );
        assert_eq!(b.density, Density::Compact);
        assert_eq!(b.cache_days, 7);
        assert!(b.sidebar.is_hidden("trades"));
        assert_eq!(b.player_command.as_deref(), Some("mine {uri}"));
        assert_eq!(import(&mut b, &file), Ok(0));

        // A file that tries to sneak the command in anyway is ignored for it.
        let sneaky = file.replace("\"player_command\": null", "\"player_command\": \"evil {uri}\"");
        assert!(sneaky.contains("evil"));
        import(&mut b, &sneaky).unwrap();
        assert_eq!(b.player_command.as_deref(), Some("mine {uri}"));
    }

    #[test]
    fn import_rejects_other_files() {
        let mut s = Settings::default();
        assert!(import(&mut s, "{}").is_err());
        assert!(import(&mut s, "not json").is_err());
        assert_eq!(s, Settings::default());
    }
}

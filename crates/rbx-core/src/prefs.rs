//! The settings a user can change one at a time, with titles for the settings screen and the
//! command bar. `settings.set` and `settings.reset` go through here, so every setting listed
//! has one way to be read, changed and put back.
//!
//! Values cross as text: `on`/`off` for switches, the option's value for choices, a whole
//! number for ranges and the text itself for text.

use crate::settings::{Density, Mode, Settings, valid_format_locale};

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
        "Applies to counts, prices and dates.",
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
        "item_values",
        "data",
        "Item values",
        "Show Rolimons values, demand and trends for limiteds, the Values screen, and trading stats and last online on profiles. Refreshed every two minutes while a screen shows them.",
        &["values", "rolimons", "worth", "rap"],
        PrefKind::Bool,
    ),
    pref(
        "background_accounts",
        "accounts",
        "Keep other accounts live",
        "Chat, presence and unread counts for every signed-in account, not only the active one.",
        &["background", "realtime", "unread", "accounts", "presence"],
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
        "player_command" => s.player_command.clone().unwrap_or_default(),
        "avatar_view" => if s.avatar_3d { "3d" } else { "2d" }.to_owned(),
        "format_locale" => s.format_locale.clone(),
        "item_values" => on_off(s.item_values).to_owned(),
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
        "background_accounts" => s.background_accounts = bool()?,
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
}

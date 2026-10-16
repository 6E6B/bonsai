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
        "item_values",
        "data",
        "Item values",
        "Show Rolimons values, demand and trends for limiteds, the Values screen, and trading stats and last online on profiles. Refreshed every two minutes while a screen shows them.",
        &["values", "rolimons", "worth", "rap"],
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
        "player_command" => s.player_command.clone().unwrap_or_default(),
        "avatar_view" => if s.avatar_3d { "3d" } else { "2d" }.to_owned(),
        "format_locale" => s.format_locale.clone(),
        "item_values" => on_off(s.item_values).to_owned(),
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
            "avatar_view" => s.avatar_3d = v == "3d",
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

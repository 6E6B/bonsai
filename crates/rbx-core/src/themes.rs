//! Themes: the built-ins bundled from `rbx-core/themes/`, and the user's, one `<id>.json` each in
//! [`Paths::themes_dir`](crate::account::Paths::themes_dir).
//!
//! A theme is data only (see `docs/theming.md`). Here we check its shape and keep its values to a
//! small character set, so nothing in a theme can be read as CSS beyond a value, a URL or code.
//! The UI turns seeds into tokens and checks each value against its kind before applying it.

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::atomic::Ordering;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::settings::{Appearance, Mode};
use crate::{Cmd, Core};

/// Bump when the document changes shape in a way older apps can't read. Matches `SCHEMA` in
/// `ui/src/lib/theme/theme.ts`.
pub const SCHEMA: u32 = 1;

const MAX_BYTES: usize = 32 * 1024;
const MAX_TOKENS: usize = 256;
const MAX_TEXT: usize = 64;
const MAX_VALUE: usize = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Base {
    Dark,
    Light,
}

impl Base {
    pub fn name(self) -> &'static str {
        match self {
            Base::Dark => "dark",
            Base::Light => "light",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Theme {
    pub schema: u32,
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub version: String,
    pub base: Base,
    /// Inputs the UI derives every token from. Opaque here beyond their shape.
    #[serde(default)]
    pub seeds: Map<String, Value>,
    /// Pinned token values, by token name.
    #[serde(default)]
    pub tokens: BTreeMap<String, String>,
}

/// Lowercase letters, digits and dashes, starting with a letter or digit. Also the file name.
pub fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 64
        && id
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        && !id.starts_with('-')
}

/// Hex colors, px lengths, shadow layers, `var(--token)` and font ids need nothing more.
fn valid_value(v: &str) -> bool {
    v.len() <= MAX_VALUE
        && v.chars()
            .all(|c| c.is_ascii_alphanumeric() || " #.,()-".contains(c))
}

fn valid_seed(v: &Value, depth: u8) -> bool {
    match v {
        Value::String(s) => valid_value(s),
        Value::Number(n) => n.as_f64().is_some_and(f64::is_finite),
        Value::Object(m) if depth == 0 => m.iter().all(|(k, v)| valid_value(k) && valid_seed(v, depth + 1)),
        _ => false,
    }
}

fn clean_text(s: &str) -> String {
    s.trim()
        .chars()
        .filter(|c| !c.is_control())
        .take(MAX_TEXT)
        .collect()
}

impl Theme {
    /// Reads and checks a theme from untrusted JSON. Unknown top-level fields are ignored.
    pub fn parse(json: &str) -> Result<Theme, String> {
        if json.len() > MAX_BYTES {
            return Err("Theme is too large.".into());
        }
        let mut t: Theme = serde_json::from_str(json).map_err(|_| "Not a theme.".to_owned())?;
        t.check()?;
        t.name = clean_text(&t.name);
        if t.name.is_empty() {
            t.name = "Untitled".into();
        }
        t.author = clean_text(&t.author);
        t.version = clean_text(&t.version);
        if t.version.is_empty() {
            t.version = "1.0.0".into();
        }
        Ok(t)
    }

    pub fn check(&self) -> Result<(), String> {
        if self.schema == 0 {
            return Err("Not a theme.".into());
        }
        if self.schema > SCHEMA {
            return Err("Made with a newer version of the app.".into());
        }
        if !valid_id(&self.id) {
            return Err("Theme has no valid id.".into());
        }
        if self.tokens.len() > MAX_TOKENS || !self.tokens.iter().all(|(k, v)| valid_id(k) && valid_value(v)) {
            return Err("Theme has values that aren't colors, sizes or fonts.".into());
        }
        if !self.seeds.iter().all(|(k, v)| valid_id(k) && valid_seed(v, 0)) {
            return Err("Theme has seeds that aren't colors or numbers.".into());
        }
        Ok(())
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    /// The Google Fonts family a font seed names (`{"google": "Space Grotesk"}`), if any.
    pub fn google_font(&self, slot: FontSlot) -> Option<&str> {
        let name = self.seeds.get(slot.seed())?.get("google")?.as_str()?;
        crate::fonts::valid_family(name).then_some(name)
    }
}

/// Which of a theme's two fonts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FontSlot {
    /// Everything but code.
    Sans,
    Mono,
}

impl FontSlot {
    fn seed(self) -> &'static str {
        match self {
            FontSlot::Sans => "font",
            FontSlot::Mono => "mono",
        }
    }

    /// The bundled font typed by name, as its seed id: `Inter` or `System` for text,
    /// `JetBrains Mono` or `System` for code. Google has Inter and JetBrains Mono too; the
    /// bundled copies need no download.
    fn bundled(self, name: &str) -> Option<&'static str> {
        let n = name.to_ascii_lowercase().replace(' ', "");
        match (self, n.as_str()) {
            (FontSlot::Sans, "inter") => Some("inter"),
            (FontSlot::Mono, "jetbrains" | "jetbrainsmono") => Some("jetbrains"),
            (_, "system") => Some("system"),
            _ => None,
        }
    }
}

const BUILTIN: [&str; 14] = [
    include_str!("../themes/dark.json"),
    include_str!("../themes/light.json"),
    include_str!("../themes/midnight.json"),
    include_str!("../themes/fjord.json"),
    include_str!("../themes/ember.json"),
    include_str!("../themes/paper.json"),
    include_str!("../themes/moss.json"),
    include_str!("../themes/dusk.json"),
    include_str!("../themes/abyss.json"),
    include_str!("../themes/phosphor.json"),
    include_str!("../themes/sakura.json"),
    include_str!("../themes/glacier.json"),
    include_str!("../themes/adwaita.json"),
    include_str!("../themes/adwaita-dark.json"),
];

/// The theme used when a setting names one that is gone.
pub fn default_id(base: Base) -> &'static str {
    base.name()
}

/// Built-in and user themes. Built-ins come first, then the user's by name.
#[derive(Clone, Debug)]
pub struct Themes {
    builtin: Vec<Theme>,
    user: BTreeMap<String, Theme>,
}

impl Default for Themes {
    fn default() -> Self {
        Themes {
            builtin: BUILTIN
                .iter()
                .map(|j| Theme::parse(j).unwrap_or_else(|e| panic!("built-in theme: {e}")))
                .collect(),
            user: BTreeMap::new(),
        }
    }
}

impl Themes {
    /// The built-ins plus every readable theme in `dir`. Files that fail to parse, or reuse a
    /// built-in id, are skipped.
    pub fn load(dir: &Path) -> Themes {
        let mut t = Themes::default();
        let Ok(entries) = std::fs::read_dir(dir) else {
            return t;
        };
        for e in entries.flatten() {
            let p = e.path();
            if p.extension().is_none_or(|x| x != "json") {
                continue;
            }
            let parsed = std::fs::read_to_string(&p)
                .map_err(|e| e.to_string())
                .and_then(|s| Theme::parse(&s));
            match parsed {
                Ok(theme) if !t.is_builtin(&theme.id) => {
                    t.user.insert(theme.id.clone(), theme);
                }
                Ok(_) => tracing::warn!("theme {}: uses a built-in id", p.display()),
                Err(e) => tracing::warn!("theme {}: {e}", p.display()),
            }
        }
        t
    }

    pub fn is_builtin(&self, id: &str) -> bool {
        self.builtin.iter().any(|t| t.id == id)
    }

    pub fn get(&self, id: &str) -> Option<&Theme> {
        self.builtin
            .iter()
            .find(|t| t.id == id)
            .or_else(|| self.user.get(id))
    }

    /// Every theme in display order, with whether it is built in.
    pub fn all(&self) -> Vec<(&Theme, bool)> {
        let mut user: Vec<&Theme> = self.user.values().collect();
        user.sort_by_cached_key(|t| (t.name.to_lowercase(), t.id.clone()));
        self.builtin
            .iter()
            .map(|t| (t, true))
            .chain(user.into_iter().map(|t| (t, false)))
            .collect()
    }

    /// Adds or replaces a user theme. Built-ins can't be replaced.
    pub fn insert(&mut self, theme: Theme) -> Result<(), String> {
        theme.check()?;
        if self.is_builtin(&theme.id) {
            return Err("Built-in themes can't be changed. Duplicate it first.".into());
        }
        self.user.insert(theme.id.clone(), theme);
        Ok(())
    }

    pub fn remove(&mut self, id: &str) -> Result<Theme, String> {
        if self.is_builtin(id) {
            return Err("Built-in themes can't be deleted.".into());
        }
        self.user
            .remove(id)
            .ok_or_else(|| "No theme with that id.".to_owned())
    }
}

/// Writes a user theme to `<dir>/<id>.json`.
pub fn save(dir: &Path, theme: &Theme) -> std::io::Result<()> {
    std::fs::create_dir_all(dir)?;
    let p = dir.join(format!("{}.json", theme.id));
    let tmp = p.with_extension("json.tmp");
    std::fs::write(&tmp, theme.to_json())?;
    std::fs::rename(tmp, p)
}

pub fn delete(dir: &Path, id: &str) -> std::io::Result<()> {
    match std::fs::remove_file(dir.join(format!("{id}.json"))) {
        Err(e) if e.kind() != std::io::ErrorKind::NotFound => Err(e),
        _ => Ok(()),
    }
}

/// A new theme id from its name, like `dark-copy` or `dark-copy-2`.
fn new_id(name: &str, taken: impl Fn(&str) -> bool) -> String {
    let mut stem: String = name
        .to_ascii_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect();
    stem = stem
        .split('-')
        .filter(|p| !p.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    stem.truncate(48);
    let stem = format!(
        "{}-copy",
        if stem.is_empty() {
            "theme"
        } else {
            stem.trim_end_matches('-')
        }
    );
    (1..)
        .map(|n| {
            if n == 1 {
                stem.clone()
            } else {
                format!("{stem}-{n}")
            }
        })
        .find(|id| !taken(id))
        .unwrap_or(stem)
}

impl Base {
    fn mode(self) -> Mode {
        match self {
            Base::Dark => Mode::Dark,
            Base::Light => Mode::Light,
        }
    }
}

impl Appearance {
    pub fn slot(&self, base: Base) -> &str {
        match base {
            Base::Dark => &self.dark,
            Base::Light => &self.light,
        }
    }
    fn slot_mut(&mut self, base: Base) -> &mut String {
        match base {
            Base::Dark => &mut self.dark,
            Base::Light => &mut self.light,
        }
    }
}

impl Core {
    /// Every installed theme in display order, with whether it is built in.
    pub fn themes(&self) -> Vec<(Theme, bool)> {
        let t = self.inner.themes.read();
        t.all().into_iter().map(|(x, b)| (x.clone(), b)).collect()
    }

    /// The theme set for `base`, or the built-in one when that theme is gone.
    pub fn appearance_theme(&self, base: Base) -> Theme {
        let id = self.inner.settings.read().appearance.slot(base).to_owned();
        let t = self.inner.themes.read();
        t.get(&id)
            .filter(|x| x.base == base)
            .or_else(|| t.get(default_id(base)))
            .cloned()
            .unwrap_or_else(|| unreachable!("built-in themes are always there"))
    }

    /// Uses a theme for its base and switches to that base, so the change shows at once.
    pub fn apply_theme(&self, id: &str) -> Result<(), String> {
        let base = self
            .inner
            .themes
            .read()
            .get(id)
            .map(|t| t.base)
            .ok_or_else(|| "No theme with that id.".to_owned())?;
        self.update_settings(|s| {
            *s.appearance.slot_mut(base) = id.to_owned();
            s.appearance.mode = base.mode();
        });
        Ok(())
    }

    /// The UI reports the OS preference, so System mode knows which theme is on screen.
    pub fn set_system_dark(&self, dark: bool) {
        self.inner.system_dark.store(dark, Ordering::Relaxed);
    }

    /// The base on screen: the mode's, or in System mode the OS's.
    pub fn shown_base(&self) -> Base {
        match self.inner.settings.read().appearance.mode {
            Mode::Dark => Base::Dark,
            Mode::Light => Base::Light,
            Mode::System if self.inner.system_dark.load(Ordering::Relaxed) => Base::Dark,
            Mode::System => Base::Light,
        }
    }

    /// The Google Fonts families the dark and light themes use, without repeats.
    pub fn appearance_fonts(&self) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        for base in [Base::Dark, Base::Light] {
            let t = self.appearance_theme(base);
            for slot in [FontSlot::Sans, FontSlot::Mono] {
                if let Some(f) = t.google_font(slot)
                    && !out.iter().any(|x| x == f)
                {
                    out.push(f.to_owned());
                }
            }
        }
        out
    }

    /// Applies the theme after the one on screen, in display order.
    pub fn cycle_theme(&self) {
        let a = self.inner.settings.read().appearance.clone();
        let current = a.slot(self.shown_base()).to_owned();
        let ids: Vec<String> = self
            .inner
            .themes
            .read()
            .all()
            .iter()
            .map(|(t, _)| t.id.clone())
            .collect();
        let at = ids.iter().position(|x| *x == current);
        let next = at.map_or(0, |i| (i + 1) % ids.len());
        if let Some(id) = ids.get(next) {
            let _ = self.apply_theme(id);
        }
    }

    /// Adds or replaces a user theme and writes it to disk.
    pub fn save_theme(&self, theme: Theme) -> Result<(), String> {
        self.inner.themes.write().insert(theme.clone())?;
        self.send(Cmd::SaveTheme(Box::new(theme)));
        self.inner.repaint();
        Ok(())
    }

    /// Copies a theme as a new user theme named "<name> copy" and returns the copy's id.
    pub fn duplicate_theme(&self, id: &str) -> Result<String, String> {
        let mut t = self
            .inner
            .themes
            .read()
            .get(id)
            .cloned()
            .ok_or_else(|| "No theme with that id.".to_owned())?;
        t.id = new_id(&t.name, |x| self.inner.themes.read().get(x).is_some());
        t.name = format!("{} copy", t.name).chars().take(MAX_TEXT).collect();
        t.version = "1.0.0".into();
        let new = t.id.clone();
        self.save_theme(t)?;
        Ok(new)
    }

    /// Sets the text or code font of the theme on screen. `name` is a bundled font (Inter,
    /// JetBrains Mono, System) or a Google Fonts family. A built-in theme is copied first, and the
    /// copy takes its place for that base. The mode doesn't change.
    pub fn set_theme_font(&self, slot: FontSlot, name: &str) -> Result<(), String> {
        let name = name.trim();
        let seed = match slot.bundled(name) {
            Some(id) => Value::String(id.into()),
            None => {
                let Some(info) = self.font_info(name) else {
                    if self.font_catalog().data.is_none() {
                        return Err("The Google Fonts list is still loading. Try again in a moment.".into());
                    }
                    return Err(format!("Google Fonts has no font named {name}."));
                };
                let mut m = Map::new();
                m.insert("google".into(), Value::String(info.family));
                Value::Object(m)
            }
        };
        let shown = self.appearance_theme(self.shown_base());
        let mut theme = if self.inner.themes.read().is_builtin(&shown.id) {
            let id = self.duplicate_theme(&shown.id)?;
            self.inner.toast(
                None,
                format!("Made a copy of {} to change it.", shown.name),
                false,
            );
            self.inner
                .themes
                .read()
                .get(&id)
                .cloned()
                .ok_or_else(|| "No theme with that id.".to_owned())?
        } else {
            shown
        };
        theme.seeds.insert(slot.seed().into(), seed);
        let (id, base) = (theme.id.clone(), theme.base);
        self.save_theme(theme)?;
        // Only the slot: the mode stays, so System mode keeps following the OS.
        self.update_settings(|s| *s.appearance.slot_mut(base) = id);
        Ok(())
    }

    /// Deletes a user theme. A slot that used it goes back to the built-in for its base.
    pub fn delete_theme(&self, id: &str) -> Result<(), String> {
        let gone = self.inner.themes.write().remove(id)?;
        let a = self.inner.settings.read().appearance.clone();
        if a.slot(gone.base) == id {
            self.update_settings(|s| *s.appearance.slot_mut(gone.base) = default_id(gone.base).into());
        }
        self.send(Cmd::DeleteTheme(gone.id));
        self.inner.repaint();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn theme(id: &str) -> Theme {
        Theme::parse(&format!(r##"{{"schema":1,"id":"{id}","base":"dark"}}"##)).unwrap()
    }

    #[test]
    fn builtins_parse_with_unique_ids() {
        let t = Themes::default();
        let ids: Vec<_> = t.all().iter().map(|(t, _)| t.id.clone()).collect();
        assert_eq!(
            ids,
            [
                "dark",
                "light",
                "midnight",
                "fjord",
                "ember",
                "paper",
                "moss",
                "dusk",
                "abyss",
                "phosphor",
                "sakura",
                "glacier",
                "adwaita",
                "adwaita-dark"
            ]
        );
        assert_eq!(t.get("dark").unwrap().base, Base::Dark);
        assert_eq!(t.get("light").unwrap().base, Base::Light);
    }

    #[test]
    fn parse_fills_and_cleans_text() {
        let t = Theme::parse(r##"{"schema":1,"id":"x","base":"light","name":"  a\u0007b  ","extra":1}"##)
            .unwrap();
        assert_eq!(t.name, "ab");
        assert_eq!(t.version, "1.0.0");
        assert_eq!(theme("y").name, "Untitled");
    }

    #[test]
    fn parse_refuses_what_it_cant_use() {
        for (json, err) in [
            ("[]", "Not a theme."),
            (r##"{"schema":1,"id":"x"}"##, "Not a theme."),
            (
                r##"{"schema":2,"id":"x","base":"dark"}"##,
                "Made with a newer version of the app.",
            ),
            (
                r##"{"schema":1,"id":"../x","base":"dark"}"##,
                "Theme has no valid id.",
            ),
            (
                r##"{"schema":1,"id":"X","base":"dark"}"##,
                "Theme has no valid id.",
            ),
            (r##"{"schema":1,"id":"x","base":"sepia"}"##, "Not a theme."),
        ] {
            assert_eq!(Theme::parse(json).unwrap_err(), err, "{json}");
        }
    }

    #[test]
    fn values_stay_in_a_small_charset() {
        let bad = [
            r##"{"color-canvas":"url(https://x)"}"##,
            r##"{"color-canvas":"#fff; color: red"}"##,
            r##"{"color-canvas":"\"x\""}"##,
            r##"{"Color":"#fff"}"##,
        ];
        for tokens in bad {
            let json = format!(r##"{{"schema":1,"id":"x","base":"dark","tokens":{tokens}}}"##);
            assert!(Theme::parse(&json).is_err(), "{tokens}");
        }
        let ok = r##"{"schema":1,"id":"x","base":"dark","tokens":{"shadow-key":"inset 0 -1px 0 #00000059, 0 0 0 1px #ffffff14","color-focus":"var(--color-accent)"},"seeds":{"accent":"#ff0066","radius":2,"shadow":{"depth":0.5}}}"##;
        assert!(Theme::parse(ok).is_ok());
        let nested = r##"{"schema":1,"id":"x","base":"dark","seeds":{"shadow":{"a":{"b":1}}}}"##;
        assert!(Theme::parse(nested).is_err());
    }

    #[test]
    fn builtins_are_fixed() {
        let mut t = Themes::default();
        assert!(t.insert(theme("dark")).is_err());
        assert!(t.remove("light").is_err());
        t.insert(theme("mine")).unwrap();
        assert!(t.all().iter().any(|(x, b)| x.id == "mine" && !b));
        assert_eq!(t.remove("mine").unwrap().id, "mine");
        assert!(t.remove("mine").is_err());
    }

    #[test]
    fn save_load_delete() {
        let d = tempfile::tempdir().unwrap();
        let mut a = theme("b-theme");
        a.name = "B".into();
        save(d.path(), &a).unwrap();
        save(d.path(), &theme("dark")).unwrap(); // a file can't shadow a built-in
        std::fs::write(d.path().join("junk.json"), "{").unwrap();
        let t = Themes::load(d.path());
        assert_eq!(t.get("b-theme"), Some(&a));
        assert_eq!(t.all().len(), BUILTIN.len() + 1);
        delete(d.path(), "b-theme").unwrap();
        delete(d.path(), "b-theme").unwrap();
        assert!(Themes::load(d.path()).get("b-theme").is_none());
    }
}

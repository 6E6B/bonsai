//! Themes: the built-ins bundled from `bonsai-core/themes/`, and the user's, one `<id>.json` each in
//! [`Paths::themes_dir`](crate::account::Paths::themes_dir).
//!
//! A theme is data only (see `docs/theming.md`). Here we check its shape and keep its values to a
//! small character set, so nothing in a theme can be read as CSS beyond a value, a URL or code.
//! The UI turns seeds into tokens and checks each value against its kind before applying it.

use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::{Duration, SystemTime};

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::core::Inner;
use crate::settings::{Appearance, Background, Mode};
use crate::{Cmd, Core};

/// Bump when the document changes shape in a way older apps can't read. Matches `SCHEMA` in
/// `ui/src/lib/theme/theme.ts`.
pub const SCHEMA: u32 = 1;

const MAX_BYTES: usize = 32 * 1024;
/// Starts every share code. The number is the code format, not the document schema.
pub const CODE_PREFIX: &str = "bonsai-theme:1:";
/// What theme files are called when exported.
pub const FILE_EXT: &str = "bonsaitheme";
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
    /// Portable background URL and effects, included in exports and hub releases.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background: Option<Background>,
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
        if let Some(background) = &self.background {
            crate::backgrounds::validate_url(&background.url)?;
            let mut levels = background.clone();
            for (name, max) in Background::LEVELS {
                if levels.level_mut(name).is_some_and(|v| *v > max) {
                    return Err(format!("Background {name} is out of range."));
                }
            }
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

    /// The theme as one pasteable line: [`CODE_PREFIX`] and the deflated JSON in base64url.
    pub fn share_code(&self) -> String {
        let json = serde_json::to_vec(self).unwrap_or_default();
        let mut z = flate2::write::DeflateEncoder::new(Vec::new(), flate2::Compression::best());
        // Writing to a Vec can't fail.
        let _ = z.write_all(&json);
        let packed = z.finish().unwrap_or_default();
        format!("{CODE_PREFIX}{}", URL_SAFE_NO_PAD.encode(packed))
    }

    /// Reads a share code or a theme's JSON, as pasted or read from a `.bonsaitheme` file.
    pub fn decode(text: &str) -> Result<Theme, String> {
        let text = text.trim();
        if let Some(code) = text.strip_prefix(CODE_PREFIX) {
            // Chat apps wrap long lines; the code has no spaces of its own.
            let code: String = code.split_whitespace().collect();
            let packed = URL_SAFE_NO_PAD
                .decode(code.trim_end_matches('='))
                .map_err(|_| "That theme code is cut off or mistyped.".to_owned())?;
            let mut json = String::new();
            flate2::read::DeflateDecoder::new(packed.as_slice())
                .take(MAX_BYTES as u64 + 1)
                .read_to_string(&mut json)
                .map_err(|_| "That theme code is cut off or mistyped.".to_owned())?;
            return Theme::parse(&json);
        }
        if text.starts_with("bonsai-theme:") {
            return Err("Made with a newer version of the app.".into());
        }
        if text.starts_with('{') {
            return Theme::parse(text);
        }
        Err("Paste a theme code (bonsai-theme:1:…) or pick a .bonsaitheme file.".into())
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

/// Makes `<id>.json` match the theme in memory: written if it is there, deleted if not. Writes run
/// one at a time and read memory under the lock, so quick saves can't land out of order.
pub(crate) fn write_current(inner: &Inner, id: &str) {
    static IO: parking_lot::Mutex<()> = parking_lot::Mutex::new(());
    let _io = IO.lock();
    let dir = inner.paths.themes_dir();
    let current = {
        let t = inner.themes.read();
        t.get(id).filter(|_| !t.is_builtin(id)).cloned()
    };
    let r = match &current {
        Some(t) => save(&dir, t),
        None => delete(&dir, id),
    };
    if let Err(e) = r {
        tracing::warn!("writing theme {id}: {e}");
    }
    inner.theme_writes.fetch_sub(1, Ordering::AcqRel);
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

/// A theme read from a file or a code, shown in the import dialog until installed or cancelled.
#[derive(Clone, Debug)]
pub struct PendingImport {
    pub theme: Theme,
    /// The file name, or empty for a pasted code.
    pub source: String,
    /// The installed theme with the same id, if any: (name, built in).
    pub clash: Option<(String, bool)>,
}

/// How to install an import whose id is taken.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OnClash {
    Replace,
    KeepBoth,
}

/// Names, sizes and change times of the theme files: enough to notice an edit.
fn dir_stamp(dir: &Path) -> Vec<(String, u64, Option<SystemTime>)> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut out: Vec<_> = entries
        .flatten()
        .filter(|e| e.path().extension().is_some_and(|x| x == "json"))
        .filter_map(|e| {
            let m = e.metadata().ok()?;
            Some((
                e.file_name().to_string_lossy().into_owned(),
                m.len(),
                m.modified().ok(),
            ))
        })
        .collect();
    out.sort();
    out
}

/// Reloads the themes when a file in the themes folder changes, so a theme edited in a text
/// editor shows at once. Only reads; the app's own saves come back unchanged.
pub(crate) async fn watch_loop(inner: Arc<Inner>) {
    let dir = inner.paths.themes_dir();
    let mut last = dir_stamp(&dir);
    let mut tick = tokio::time::interval(Duration::from_secs(2));
    loop {
        tick.tick().await;
        let d = dir.clone();
        let Ok(stamp) = inner.rt.spawn_blocking(move || dir_stamp(&d)).await else {
            continue;
        };
        if stamp == last {
            continue;
        }
        last = stamp;
        let d = dir.clone();
        let Ok(fresh) = inner.rt.spawn_blocking(move || Themes::load(&d)).await else {
            continue;
        };
        // A change still queued for disk would be lost; look again next tick. Saves count
        // themselves before they touch memory, so this check holds under the lock.
        let mut themes = inner.themes.write();
        if inner.theme_writes.load(Ordering::Acquire) > 0 {
            last.clear();
            continue;
        }
        *themes = fresh;
        drop(themes);
        inner.repaint();
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
        if self.inner.system_dark.swap(dark, Ordering::Relaxed) != dark {
            self.inner.repaint();
        }
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
        theme.check()?;
        self.inner.theme_writes.fetch_add(1, Ordering::AcqRel);
        if let Err(e) = self.inner.themes.write().insert(theme.clone()) {
            self.inner.theme_writes.fetch_sub(1, Ordering::AcqRel);
            return Err(e);
        }
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
        self.inner.theme_writes.fetch_add(1, Ordering::AcqRel);
        let gone = match self.inner.themes.write().remove(id) {
            Ok(t) => t,
            Err(e) => {
                self.inner.theme_writes.fetch_sub(1, Ordering::AcqRel);
                return Err(e);
            }
        };
        let a = self.inner.settings.read().appearance.clone();
        if a.slot(gone.base) == id {
            self.update_settings(|s| *s.appearance.slot_mut(gone.base) = default_id(gone.base).into());
        }
        self.send(Cmd::DeleteTheme(gone.id));
        self.inner.repaint();
        Ok(())
    }

    /// Reads a share code, theme JSON or a path to a theme file and opens the import dialog with
    /// it. Reads the disk, so call it off the main thread.
    pub fn stage_theme_import(&self, text: &str) -> Result<(), String> {
        let text = text.trim();
        let (theme, source) = if text.starts_with(CODE_PREFIX) || text.starts_with('{') {
            (Theme::decode(text)?, String::new())
        } else {
            let path = Path::new(text);
            let size = std::fs::metadata(path)
                .map_err(|_| "Paste a theme code (bonsai-theme:1:…) or pick a .bonsaitheme file.".to_owned())?
                .len();
            if size > MAX_BYTES as u64 {
                return Err("Theme is too large.".into());
            }
            let body =
                std::fs::read_to_string(path).map_err(|e| format!("Could not read the file ({e})."))?;
            let name = path
                .file_name()
                .map(|n| clean_text(&n.to_string_lossy()))
                .unwrap_or_default();
            (Theme::decode(&body)?, name)
        };
        let clash = self
            .inner
            .themes
            .read()
            .get(&theme.id)
            .map(|t| (t.name.clone(), self.inner.themes.read().is_builtin(&t.id)));
        self.inner.store.write().theme_import = Some(PendingImport { theme, source, clash });
        self.inner.repaint();
        Ok(())
    }

    /// Installs the theme in the import dialog and returns its id. A built-in id always keeps both.
    pub fn finish_theme_import(&self, on_clash: OnClash) -> Result<String, String> {
        let Some(p) = self.inner.store.write().theme_import.take() else {
            return Err("Nothing to import.".into());
        };
        let mut t = p.theme;
        let builtin = self.inner.themes.read().is_builtin(&t.id);
        if p.clash.is_some() && (builtin || on_clash == OnClash::KeepBoth) {
            t.id = new_id(&t.name, |x| self.inner.themes.read().get(x).is_some());
        }
        let id = t.id.clone();
        self.save_theme(t)?;
        Ok(id)
    }

    pub fn cancel_theme_import(&self) {
        self.inner.store.write().theme_import = None;
        self.inner.repaint();
    }

    /// The theme a command names, or the one on screen when `id` is empty.
    pub fn theme_or_shown(&self, id: &str) -> Result<Theme, String> {
        let id = id.trim();
        if id.is_empty() {
            return Ok(self.appearance_theme(self.shown_base()));
        }
        self.inner
            .themes
            .read()
            .get(id)
            .cloned()
            .ok_or_else(|| "No theme with that id.".to_owned())
    }

    /// Writes a theme as a `.bonsaitheme` file. Writes the disk, so call it off the main thread.
    pub fn export_theme(&self, id: &str, path: &Path) -> Result<(), String> {
        let t = self.theme_or_shown(id)?;
        std::fs::write(path, t.to_json() + "\n").map_err(|e| format!("Could not save the theme ({e})."))
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
    fn builtin_fonts() {
        let t = Themes::default();
        assert_eq!(t.get("dark").unwrap().google_font(FontSlot::Sans), None);
        assert_eq!(t.get("adwaita").unwrap().google_font(FontSlot::Sans), None);
        assert_eq!(
            t.get("ember").unwrap().google_font(FontSlot::Sans),
            Some("Figtree")
        );
        // A seed that isn't a bundled id must be a valid Google family, or it would never load.
        for (theme, _) in t.all() {
            let seed = &theme.seeds["font"];
            assert!(
                seed.as_str().is_some_and(|s| s == "inter") || theme.google_font(FontSlot::Sans).is_some(),
                "{}",
                theme.id
            );
        }
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
    fn backgrounds_require_external_links_and_bounded_effects() {
        for background in [
            r#"{"url":"file:///tmp/a.png"}"#,
            r#"{"url":"data:image/png;base64,abc"}"#,
            r#"{"url":"https://example.com/a.png","blur":41}"#,
            r#"{"url":"https://example.com/a.png","image":101}"#,
            r#"{"file":"local.png"}"#,
        ] {
            let json = format!(r#"{{"schema":1,"id":"test","base":"dark","background":{background}}}"#);
            assert!(Theme::parse(&json).is_err(), "{background}");
        }
    }

    #[test]
    fn share_codes_round_trip() {
        for (t, _) in Themes::default().all() {
            let code = t.share_code();
            assert!(code.starts_with(CODE_PREFIX));
            assert!(code.len() < 1024, "{}: {} bytes", t.id, code.len());
            assert_eq!(&Theme::decode(&code).unwrap(), t);
        }
        // Wrapped by a chat app, with padding someone added.
        let code = theme("x").share_code();
        let (a, b) = code.split_at(code.len() / 2);
        assert_eq!(Theme::decode(&format!("  {a}\n {b}==  ")).unwrap().id, "x");
    }

    #[test]
    fn decode_takes_json_and_refuses_the_rest() {
        assert_eq!(
            Theme::decode(r##" {"schema":1,"id":"x","base":"dark"} "##)
                .unwrap()
                .id,
            "x"
        );
        let cut = &theme("x").share_code()[..30];
        for (text, err) in [
            (cut, "That theme code is cut off or mistyped."),
            ("bonsai-theme:1:!!!", "That theme code is cut off or mistyped."),
            ("bonsai-theme:2:abc", "Made with a newer version of the app."),
            (
                "hello",
                "Paste a theme code (bonsai-theme:1:…) or pick a .bonsaitheme file.",
            ),
        ] {
            assert_eq!(Theme::decode(text).unwrap_err(), err, "{text}");
        }
        // A small code that inflates past the size limit is refused, not read whole.
        let mut z = flate2::write::DeflateEncoder::new(Vec::new(), flate2::Compression::best());
        z.write_all(&vec![b' '; MAX_BYTES * 4]).unwrap();
        let bomb = format!("{CODE_PREFIX}{}", URL_SAFE_NO_PAD.encode(z.finish().unwrap()));
        assert_eq!(Theme::decode(&bomb).unwrap_err(), "Theme is too large.");
    }

    #[test]
    fn dir_stamp_sees_edits() {
        let d = tempfile::tempdir().unwrap();
        assert!(dir_stamp(d.path()).is_empty());
        save(d.path(), &theme("a")).unwrap();
        std::fs::write(d.path().join("notes.txt"), "x").unwrap();
        let first = dir_stamp(d.path());
        assert_eq!(first.len(), 1);
        let mut t = theme("a");
        t.name = "Longer name".into();
        save(d.path(), &t).unwrap();
        assert_ne!(dir_stamp(d.path()), first);
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

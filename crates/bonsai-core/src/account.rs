//! Accounts: the list (labels, order, color tags) in a small config file without secrets, and
//! cookies in the OS keychain.

use std::collections::HashMap;
use std::path::PathBuf;

use bonsai_api::{AccountId, Secret};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

/// Colour tag shown on the rail and status bar.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ColorTag {
    #[default]
    None,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Pink,
}

impl ColorTag {
    pub const ALL: [ColorTag; 8] = [
        ColorTag::None,
        ColorTag::Red,
        ColorTag::Orange,
        ColorTag::Yellow,
        ColorTag::Green,
        ColorTag::Blue,
        ColorTag::Purple,
        ColorTag::Pink,
    ];
    pub fn name(self) -> &'static str {
        match self {
            ColorTag::None => "none",
            ColorTag::Red => "red",
            ColorTag::Orange => "orange",
            ColorTag::Yellow => "yellow",
            ColorTag::Green => "green",
            ColorTag::Blue => "blue",
            ColorTag::Purple => "purple",
            ColorTag::Pink => "pink",
        }
    }
    pub fn from_name(s: &str) -> Option<ColorTag> {
        ColorTag::ALL
            .into_iter()
            .find(|c| c.name().eq_ignore_ascii_case(s.trim()))
    }
    /// sRGB, for the UI.
    pub fn rgb(self) -> Option<[u8; 3]> {
        Some(match self {
            ColorTag::None => return None,
            ColorTag::Red => [229, 72, 77],
            ColorTag::Orange => [247, 107, 21],
            ColorTag::Yellow => [255, 197, 61],
            ColorTag::Green => [48, 164, 108],
            ColorTag::Blue => [0, 144, 255],
            ColorTag::Purple => [142, 78, 198],
            ColorTag::Pink => [214, 64, 159],
        })
    }
}

/// One signed-in account, without its cookie.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AccountMeta {
    pub id: AccountId,
    pub name: String,
    pub display_name: String,
    /// Local label; shown instead of the display name when set.
    pub label: Option<String>,
    pub color: ColorTag,
    pub added_at: i64,
}

impl Default for AccountMeta {
    fn default() -> Self {
        Self {
            id: AccountId::ANONYMOUS,
            name: String::new(),
            display_name: String::new(),
            label: None,
            color: ColorTag::None,
            added_at: 0,
        }
    }
}

impl AccountMeta {
    pub fn title(&self) -> &str {
        self.label
            .as_deref()
            .filter(|l| !l.is_empty())
            .unwrap_or(&self.display_name)
    }
}

/// The saved account list. Order is rail order.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AccountList {
    pub accounts: Vec<AccountMeta>,
    pub active: Option<AccountId>,
    pub previous: Option<AccountId>,
}

impl AccountList {
    pub fn load(path: &std::path::Path) -> AccountList {
        std::fs::read_to_string(path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self, path: &std::path::Path) -> std::io::Result<()> {
        if let Some(d) = path.parent() {
            std::fs::create_dir_all(d)?;
        }
        let tmp = path.with_extension("json.tmp");
        std::fs::write(&tmp, serde_json::to_vec_pretty(self).unwrap_or_default())?;
        std::fs::rename(tmp, path)
    }

    pub fn get(&self, id: AccountId) -> Option<&AccountMeta> {
        self.accounts.iter().find(|a| a.id == id)
    }

    pub fn get_mut(&mut self, id: AccountId) -> Option<&mut AccountMeta> {
        self.accounts.iter_mut().find(|a| a.id == id)
    }

    /// Insert or update (re-adding an account refreshes its names and keeps its place).
    pub fn upsert(&mut self, meta: AccountMeta) {
        match self.get_mut(meta.id) {
            Some(a) => {
                a.name = meta.name;
                a.display_name = meta.display_name;
            }
            None => self.accounts.push(meta),
        }
    }

    pub fn remove(&mut self, id: AccountId) {
        self.accounts.retain(|a| a.id != id);
        if self.active == Some(id) {
            self.active = self
                .previous
                .filter(|p| *p != id)
                .or_else(|| self.accounts.first().map(|a| a.id));
        }
        if self.previous == Some(id) {
            self.previous = None;
        }
    }

    /// Move an account to a new rail position (clamped).
    pub fn reorder(&mut self, id: AccountId, to: usize) {
        if let Some(from) = self.accounts.iter().position(|a| a.id == id) {
            let a = self.accounts.remove(from);
            let to = to.min(self.accounts.len());
            self.accounts.insert(to, a);
        }
    }

    /// Switch; remembers the previous account for Ctrl+Tab. Returns false if unknown.
    pub fn switch(&mut self, id: AccountId) -> bool {
        if self.get(id).is_none() {
            return false;
        }
        if self.active != Some(id) {
            self.previous = self.active;
            self.active = Some(id);
        }
        true
    }

    /// Ctrl+1..9
    pub fn by_position(&self, index: usize) -> Option<AccountId> {
        self.accounts.get(index).map(|a| a.id)
    }
}

/// Where cookies live. Never a plain file.
pub trait CookieStore: Send + Sync {
    fn get(&self, id: AccountId) -> Result<Option<Secret>, String>;
    fn set(&self, id: AccountId, cookie: &Secret) -> Result<(), String>;
    fn delete(&self, id: AccountId) -> Result<(), String>;
}

const SERVICE: &str = "io.github.6e6b.bonsai";
/// The app was called Baseplate. Cookies saved under this name move to `SERVICE` when read.
const LEGACY_SERVICE: &str = "io.github._6E6B.Baseplate";

/// The OS keychain (Keychain Services, Windows Credential Manager, Secret Service).
pub struct Keychain;

fn keychain_get(service: &str, id: AccountId) -> Result<Option<Secret>, String> {
    let e = keyring::Entry::new(service, &id.to_string()).map_err(|e| e.to_string())?;
    match e.get_password() {
        Ok(p) => Ok(Some(Secret::new(p))),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

fn keychain_delete(service: &str, id: AccountId) -> Result<(), String> {
    let e = keyring::Entry::new(service, &id.to_string()).map_err(|e| e.to_string())?;
    match e.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

impl CookieStore for Keychain {
    fn get(&self, id: AccountId) -> Result<Option<Secret>, String> {
        if let Some(c) = keychain_get(SERVICE, id)? {
            return Ok(Some(c));
        }
        let Some(c) = keychain_get(LEGACY_SERVICE, id)? else {
            return Ok(None);
        };
        // Drop the old entry only once the new one is saved.
        if self.set(id, &c).is_ok() {
            let _ = keychain_delete(LEGACY_SERVICE, id);
        }
        Ok(Some(c))
    }
    fn set(&self, id: AccountId, cookie: &Secret) -> Result<(), String> {
        let e = keyring::Entry::new(SERVICE, &id.to_string()).map_err(|e| e.to_string())?;
        e.set_password(cookie.expose()).map_err(|e| e.to_string())
    }
    fn delete(&self, id: AccountId) -> Result<(), String> {
        let _ = keychain_delete(LEGACY_SERVICE, id);
        keychain_delete(SERVICE, id)
    }
}

/// In-memory store: tests, and sessions where the keychain is unavailable.
#[derive(Default)]
pub struct MemoryCookies {
    map: Mutex<HashMap<AccountId, Secret>>,
}

impl CookieStore for MemoryCookies {
    fn get(&self, id: AccountId) -> Result<Option<Secret>, String> {
        Ok(self.map.lock().get(&id).cloned())
    }
    fn set(&self, id: AccountId, cookie: &Secret) -> Result<(), String> {
        self.map.lock().insert(id, cookie.clone());
        Ok(())
    }
    fn delete(&self, id: AccountId) -> Result<(), String> {
        self.map.lock().remove(&id);
        Ok(())
    }
}

/// Keychain first; if it fails, keep the cookie in memory for this session only.
pub struct KeychainWithFallback {
    keychain: Keychain,
    memory: MemoryCookies,
    pub keychain_failed: Mutex<Option<String>>,
}

impl Default for KeychainWithFallback {
    fn default() -> Self {
        Self {
            keychain: Keychain,
            memory: MemoryCookies::default(),
            keychain_failed: Mutex::new(None),
        }
    }
}

impl CookieStore for KeychainWithFallback {
    fn get(&self, id: AccountId) -> Result<Option<Secret>, String> {
        if let Ok(Some(c)) = self.memory.get(id) {
            return Ok(Some(c));
        }
        self.keychain.get(id).or_else(|e| {
            *self.keychain_failed.lock() = Some(e);
            Ok(None)
        })
    }
    fn set(&self, id: AccountId, cookie: &Secret) -> Result<(), String> {
        if let Err(e) = self.keychain.set(id, cookie) {
            tracing::warn!("keychain unavailable, keeping the cookie for this session only: {e}");
            *self.keychain_failed.lock() = Some(e);
            self.memory.set(id, cookie)?;
        }
        Ok(())
    }
    fn delete(&self, id: AccountId) -> Result<(), String> {
        let _ = self.memory.delete(id);
        self.keychain.delete(id)
    }
}

/// Accepts the raw `.ROBLOSECURITY` value (with or without the warning prefix), a
/// `.ROBLOSECURITY=...` pair, or a whole `Cookie:` header, and returns just the value.
pub fn parse_cookie_input(input: &str) -> Option<Secret> {
    let s = input.trim().trim_matches('"').trim();
    let s = s
        .strip_prefix("Cookie:")
        .or_else(|| s.strip_prefix("cookie:"))
        .unwrap_or(s)
        .trim();
    let value = if s.contains(".ROBLOSECURITY=") {
        s.split(';')
            .map(str::trim)
            .find_map(|p| p.strip_prefix(".ROBLOSECURITY="))
            .map(str::to_string)?
    } else if s.contains(';') || s.contains('=') && !s.starts_with("_|WARNING") {
        // Some other cookie header without our cookie in it.
        return None;
    } else {
        s.to_string()
    };
    let value = value.trim().trim_matches('"').to_string();
    // Real cookies are long; anything short is a mistake (or a pasted username).
    let ok = value.len() >= 100 && value.chars().all(|c| c.is_ascii_graphic());
    ok.then(|| Secret::new(value))
}

/// The app was called Baseplate. Moves its folder to the new name, unless the new one exists.
fn move_legacy_dir(old: &std::path::Path, new: &std::path::Path) {
    if old == new || new.exists() || !old.is_dir() {
        return;
    }
    if let Some(parent) = new.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Err(e) = std::fs::rename(old, new) {
        tracing::warn!("cannot move {} to {}: {e}", old.display(), new.display());
    }
}

/// Default config, cache and download locations.
#[derive(Clone, Debug)]
pub struct Paths {
    pub config: PathBuf,
    pub cache: PathBuf,
    /// Where files the user asks to save (clothing templates) go.
    pub downloads: PathBuf,
}

impl Paths {
    pub fn standard() -> Paths {
        let downloads = directories::UserDirs::new()
            .and_then(|u| {
                u.download_dir()
                    .map(|d| d.to_path_buf())
                    .or_else(|| Some(u.home_dir().to_path_buf()))
            })
            .unwrap_or_else(std::env::temp_dir);
        match directories::ProjectDirs::from("io.github", "6e6b", "bonsai") {
            Some(d) => {
                let paths = Paths {
                    config: d.config_dir().to_path_buf(),
                    cache: d.cache_dir().to_path_buf(),
                    downloads,
                };
                if let Some(old) = directories::ProjectDirs::from("io.github", "_6E6B", "Baseplate") {
                    move_legacy_dir(old.config_dir(), &paths.config);
                    move_legacy_dir(old.cache_dir(), &paths.cache);
                }
                paths
            }
            None => {
                let t = std::env::temp_dir().join("bonsai");
                Paths {
                    config: t.join("config"),
                    cache: t.join("cache"),
                    downloads,
                }
            }
        }
    }
    pub fn in_dir(dir: &std::path::Path) -> Paths {
        Paths {
            config: dir.join("config"),
            cache: dir.join("cache"),
            downloads: dir.join("downloads"),
        }
    }
    /// Creates the config and cache folders so only this user can open them. They hold cached
    /// messages and trades, plugin storage and the activity log, and the files inside get the
    /// default umask, which on many systems lets other users read them.
    pub fn make_private(&self) {
        for dir in [&self.config, &self.cache] {
            if let Err(e) = std::fs::create_dir_all(dir) {
                tracing::warn!("cannot create {}: {e}", dir.display());
                continue;
            }
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Err(e) = std::fs::set_permissions(dir, std::fs::Permissions::from_mode(0o700)) {
                    tracing::warn!("cannot restrict {}: {e}", dir.display());
                }
            }
        }
    }
    pub fn accounts_file(&self) -> PathBuf {
        self.config.join("accounts.json")
    }
    pub fn ui_state_file(&self) -> PathBuf {
        self.config.join("ui-state.json")
    }
    pub fn settings_file(&self) -> PathBuf {
        self.config.join("settings.json")
    }
    pub fn db_file(&self) -> PathBuf {
        self.cache.join("cache.sqlite3")
    }
    pub fn thumbs_dir(&self) -> PathBuf {
        self.cache.join("thumbs")
    }
    pub fn themes_dir(&self) -> PathBuf {
        self.config.join("themes")
    }
    /// The app background picture, copied or downloaded here so it outlives the original.
    pub fn backgrounds_dir(&self) -> PathBuf {
        self.config.join("backgrounds")
    }
    /// Installed plugins, one folder each.
    pub fn plugins_dir(&self) -> PathBuf {
        self.config.join("plugins")
    }
    /// Which plugins are on and what each may use.
    pub fn plugins_file(&self) -> PathBuf {
        self.config.join("plugins.json")
    }
    /// Every plugin write, one JSON line each.
    pub fn plugin_activity_file(&self) -> PathBuf {
        self.config.join("plugin-activity.jsonl")
    }
    /// What plugins keep with `ctx.store`: a folder per plugin, a file per account.
    pub fn plugin_data_dir(&self) -> PathBuf {
        self.config.join("plugin-data")
    }
    /// Downloaded Google Fonts. Safe to delete: a theme that uses one downloads it again.
    pub fn fonts_dir(&self) -> PathBuf {
        self.cache.join("fonts")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(unix)]
    #[test]
    fn data_folders_are_private() {
        use std::os::unix::fs::PermissionsExt;
        let t = tempfile::tempdir().unwrap();
        let paths = Paths::in_dir(t.path());
        std::fs::create_dir_all(&paths.cache).unwrap();
        std::fs::set_permissions(&paths.cache, std::fs::Permissions::from_mode(0o755)).unwrap();
        paths.make_private();
        for dir in [&paths.config, &paths.cache] {
            let mode = std::fs::metadata(dir).unwrap().permissions().mode();
            assert_eq!(mode & 0o777, 0o700, "{}", dir.display());
        }
    }

    #[test]
    fn legacy_dir_moves_once() {
        let dir = tempfile::tempdir().unwrap();
        let old = dir.path().join("Baseplate/config");
        let new = dir.path().join("Bonsai/config");
        std::fs::create_dir_all(&old).unwrap();
        std::fs::write(old.join("accounts.json"), "[]").unwrap();
        move_legacy_dir(&old, &new);
        assert!(new.join("accounts.json").exists());
        assert!(!old.exists());

        // An existing new folder is never overwritten.
        std::fs::create_dir_all(&old).unwrap();
        std::fs::write(old.join("accounts.json"), "old").unwrap();
        move_legacy_dir(&old, &new);
        assert_eq!(std::fs::read_to_string(new.join("accounts.json")).unwrap(), "[]");
    }

    fn fake() -> String {
        format!(
            "_|WARNING:-DO-NOT-SHARE-THIS.--Sharing-this-will-allow-someone-to-log-in-as-you.|_{}",
            "A1".repeat(80)
        )
    }

    #[test]
    fn parses_every_paste_shape() {
        let c = fake();
        let raw_value = c.trim_start_matches(|_: char| false).to_string();
        for input in [
            c.clone(),
            format!("  {c}\n"),
            format!(".ROBLOSECURITY={c}"),
            format!("Cookie: RBXEventTrackerV2=x; .ROBLOSECURITY={c}; other=1"),
            format!("\"{c}\""),
            c.split("|_").nth(1).unwrap().to_string(),
        ] {
            let got = parse_cookie_input(&input).unwrap_or_else(|| panic!("rejected {input:.40}"));
            assert!(raw_value.ends_with(got.expose()));
        }
        assert!(parse_cookie_input("builderman").is_none());
        assert!(parse_cookie_input("Cookie: a=b; c=d").is_none());
    }

    #[test]
    fn list_switch_reorder_remove() {
        let mut l = AccountList::default();
        for i in 1..=3 {
            l.upsert(AccountMeta {
                id: AccountId(i),
                display_name: format!("A{i}"),
                ..Default::default()
            });
        }
        assert!(l.switch(AccountId(1)));
        assert!(l.switch(AccountId(3)));
        assert_eq!(l.previous, Some(AccountId(1)));
        assert!(!l.switch(AccountId(9)));
        l.reorder(AccountId(3), 0);
        assert_eq!(l.by_position(0), Some(AccountId(3)));
        l.remove(AccountId(3));
        assert_eq!(l.active, Some(AccountId(1)));
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("a.json");
        l.save(&p).unwrap();
        assert_eq!(AccountList::load(&p), l);
        assert!(!std::fs::read_to_string(&p).unwrap().contains("WARNING"));
    }
}

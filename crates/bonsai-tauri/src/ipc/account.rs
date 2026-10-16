//! Settings: your account (what Roblox has on file, display name, privacy), private servers
//! across experiences, and app preferences (theme, density, launch command, disk cache).
//! `list.more` loads more private servers with the key `my_servers:<owned|invited>`.

use std::sync::Arc;

use rbx_core::commands::{CommandId, Registry, Shortcut};
use rbx_core::experience::ServerChange;
use rbx_core::me::ServersTab;
use rbx_core::prefs::{self, PrefKind};
use rbx_core::settings::{
    Density, Mode, NotifyPrefs, SavedTab, SavedTabs, SidebarPrefs, valid_format_locale,
};
use rbx_core::themes::{Base, FontSlot, Theme};
use rbx_core::{AccountId, Core};
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::{AppHandle, Manager, State};

use super::{Id, Invocation, List, parse_id};

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfoView {
    pub loading: bool,
    pub error: Option<String>,
    /// Masked by Roblox; empty when none is set.
    pub email: String,
    pub email_verified: bool,
    /// Empty when none is set.
    pub phone: String,
    pub phone_verified: bool,
    /// ISO date (`2000-01-02`).
    pub birthdate: Option<String>,
    /// None until known.
    pub premium: Option<bool>,
    /// Premium tier name, e.g. `Premium 1000`; None without a subscription.
    pub premium_plan: Option<String>,
    /// ISO date the subscription renews on.
    pub premium_renews: Option<String>,
    /// ISO date the subscription ends on.
    pub premium_expires: Option<String>,
    /// None until known.
    pub age_verified: Option<bool>,
    pub voice: Option<VoiceView>,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct VoiceView {
    pub voice_enabled: bool,
    pub voice_eligible: bool,
    pub voice_verified: bool,
    pub banned: bool,
    pub camera_enabled: bool,
    pub camera_eligible: bool,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct SessionView {
    pub location: String,
    pub device: String,
    /// Unix seconds; 0 when unknown.
    pub last_active: f64,
    pub current: bool,
}

#[derive(Clone, Debug, Default, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct SessionsView {
    pub loading: bool,
    pub error: Option<String>,
    pub sessions: Vec<SessionView>,
}

/// Signed-in sessions of the active account, current one first.
#[tauri::command]
#[specta::specta]
pub fn account_sessions(core: State<'_, Arc<Core>>) -> SessionsView {
    let r = core.acting(None).map(|a| core.sessions(a)).unwrap_or_default();
    SessionsView {
        loading: r.first_load(),
        error: r.error.clone(),
        sessions: r
            .data
            .as_deref()
            .map(|d| {
                d.iter()
                    .map(|s| SessionView {
                        location: s.location.clone(),
                        device: s.device.clone(),
                        last_active: s.last_active as f64,
                        current: s.current,
                    })
                    .collect()
            })
            .unwrap_or_default(),
    }
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PrivacyRow {
    /// Roblox's key, like `whoCanSeeMyInventory`.
    pub key: String,
    pub value: String,
    pub options: Vec<PrivacyChoice>,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PrivacyChoice {
    pub value: String,
    /// False when Roblox needs something first (age verification, a parent).
    pub allowed: bool,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct MyServerRow {
    pub id: Id,
    pub name: String,
    pub active: bool,
    pub universe_id: Id,
    pub place_id: Id,
    pub game: String,
    pub owner_id: Id,
    /// Empty until known.
    pub owner_name: String,
    /// Robux a month; None when free.
    pub price: Option<u32>,
    pub renews: bool,
    /// ISO date, or empty.
    pub expires: String,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct NotifyView {
    pub friend_joins: bool,
    pub trades: bool,
    pub chats: bool,
    pub friend_requests: bool,
    pub experience_updates: bool,
    pub sales: bool,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    /// `system`, `dark` or `light`.
    pub mode: String,
    /// Theme ids for each base. Always installed themes of that base.
    pub dark_theme: String,
    pub light_theme: String,
    /// `comfortable` or `compact`.
    pub density: String,
    /// Empty uses the system's `roblox-player:` handler.
    pub player_command: String,
    /// BCP 47 tag for numbers and dates; empty follows the system.
    pub format_locale: String,
    pub notify: NotifyView,
    /// Bytes on disk, once measured.
    pub cache_data: Option<f64>,
    pub cache_images: Option<f64>,
}

/// Email, phone, birthday and Premium of the active account.
#[tauri::command]
#[specta::specta]
pub fn account_info(core: State<'_, Arc<Core>>) -> AccountInfoView {
    let r = core
        .acting(None)
        .map(|a| core.account_info(a))
        .unwrap_or_default();
    let i = r.data.as_deref().cloned().unwrap_or_default();
    AccountInfoView {
        loading: r.first_load(),
        error: r.error.clone(),
        email: i.email,
        email_verified: i.email_verified,
        phone: i.phone,
        phone_verified: i.phone_verified,
        birthdate: i.birthdate.map(|(y, m, d)| format!("{y:04}-{m:02}-{d:02}")),
        premium: i.premium,
        premium_plan: i.plan.as_ref().map(|p| p.name.clone()).filter(|n| !n.is_empty()),
        premium_renews: i.plan.as_ref().and_then(|p| p.renews.clone()),
        premium_expires: i.plan.as_ref().and_then(|p| p.expires.clone()),
        age_verified: i.age_verified,
        voice: i.voice.map(|v| VoiceView {
            voice_enabled: v.voice_enabled,
            voice_eligible: v.voice_eligible,
            voice_verified: v.voice_verified,
            banned: v.banned,
            camera_enabled: v.camera_enabled,
            camera_eligible: v.camera_eligible,
        }),
    }
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PinView {
    pub loading: bool,
    pub error: Option<String>,
    /// The account has a PIN that locks settings.
    pub enabled: bool,
    /// Unix seconds the PIN stays unlocked until; 0 when locked.
    pub unlocked_until: f64,
}

/// Account PIN of the active account.
#[tauri::command]
#[specta::specta]
pub fn pin_status(core: State<'_, Arc<Core>>) -> PinView {
    let r = core.acting(None).map(|a| core.pin_status(a)).unwrap_or_default();
    let p = r.data.as_deref().cloned().unwrap_or_default();
    PinView {
        loading: r.first_load(),
        error: r.error.clone(),
        enabled: p.enabled,
        unlocked_until: p.unlocked_until as f64,
    }
}

/// Unlocks the account PIN. Called from the dialog `account.unlock_pin` opens.
#[tauri::command]
#[specta::specta]
pub fn unlock_pin(core: State<'_, Arc<Core>>, pin: String) -> Result<(), String> {
    let a = core
        .acting(None)
        .ok_or_else(|| "Add an account first.".to_owned())?;
    core.unlock_pin(a, &pin)
}

/// Privacy settings of the active account.
#[tauri::command]
#[specta::specta]
pub fn privacy_settings(core: State<'_, Arc<Core>>) -> List<PrivacyRow> {
    let Some(a) = core.acting(None) else {
        return List {
            loading: false,
            ..List::loading()
        };
    };
    List::of(core.privacy(a), |s| PrivacyRow {
        key: s.key.clone(),
        value: s.value.clone(),
        options: s
            .options
            .iter()
            .map(|o| PrivacyChoice {
                value: o.value.clone(),
                allowed: o.allowed,
            })
            .collect(),
    })
}

/// Private servers of the active account across experiences: `owned` or `invited`.
#[tauri::command]
#[specta::specta]
pub fn my_private_servers(core: State<'_, Arc<Core>>, tab: String) -> List<MyServerRow> {
    let Some(a) = core.acting(None) else {
        return List {
            loading: false,
            ..List::loading()
        };
    };
    let r = core.my_private_servers(a, servers_tab(&tab), false);
    let (view, unnamed) = {
        let s = core.store();
        let mut unnamed = Vec::new();
        let view = List::paged(r, |p| {
            let owner_name = s
                .users
                .get(&p.owner_id)
                .map(|u| u.display_name.clone())
                .unwrap_or_default();
            if owner_name.is_empty() {
                unnamed.push(p.owner_id);
            }
            MyServerRow {
                id: Id(p.id),
                name: p.name.clone(),
                active: p.active,
                universe_id: Id(p.universe_id),
                place_id: Id(p.place_id),
                game: p.game.clone(),
                owner_id: Id(p.owner_id),
                owner_name,
                price: p.price.map(|x| x.clamp(0, u32::MAX as i64) as u32),
                renews: p.renews,
                expires: p.expires.clone(),
            }
        });
        (view, unnamed)
    };
    core.want_users(&unnamed);
    view
}

/// App preferences, and the disk cache size once measured.
#[tauri::command]
#[specta::specta]
pub fn app_settings(core: State<'_, Arc<Core>>) -> AppSettings {
    core.measure_cache();
    let s = core.settings();
    let usage = core.store().cache_usage;
    AppSettings {
        mode: s.appearance.mode.name().to_owned(),
        dark_theme: core.appearance_theme(Base::Dark).id,
        light_theme: core.appearance_theme(Base::Light).id,
        density: s.density.name().to_owned(),
        player_command: s.player_command.unwrap_or_default(),
        format_locale: s.format_locale,
        notify: NotifyView {
            friend_joins: s.notify.friend_joins,
            trades: s.notify.trades,
            chats: s.notify.chats,
            friend_requests: s.notify.friend_requests,
            experience_updates: s.notify.experience_updates,
            sales: s.notify.sales,
        },
        cache_data: usage.map(|u| u.data as f64),
        cache_images: usage.map(|u| u.images as f64),
    }
}

/// One setting from `rbx_core::prefs`, with its value now and in a fresh install.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PrefView {
    pub key: String,
    /// The settings section it sits in.
    pub section: String,
    pub title: String,
    pub description: String,
    pub keywords: Vec<String>,
    /// `bool`, `choice`, `range` or `text`.
    pub kind: String,
    /// `choice` only: `(value, label)` pairs.
    pub options: Vec<PrefOption>,
    /// `range` only.
    pub min: i32,
    pub max: i32,
    pub step: i32,
    /// `text` only: shown when empty.
    pub placeholder: String,
    /// `on`/`off` for `bool`, the option for `choice`, a number for `range`, else the text.
    pub value: String,
    pub default: String,
}

#[derive(Clone, Debug, Serialize, Type)]
pub struct PrefOption {
    pub value: String,
    pub label: String,
}

/// A tab to open at launch, or one the UI saves for next time.
#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct TabView {
    /// An app route like `/games/123`.
    pub path: String,
    pub pinned: bool,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct StartTabs {
    pub tabs: Vec<TabView>,
    /// Index into `tabs`.
    pub active: u32,
}

/// The tabs a launch opens: the ones from last time, or the start screen.
#[tauri::command]
#[specta::specta]
pub fn start_tabs(core: State<'_, Arc<Core>>) -> StartTabs {
    let s = core.settings();
    let saved = s.session.clone().sanitized();
    if s.start_screen == "last" && !saved.tabs.is_empty() {
        return StartTabs {
            active: saved.active as u32,
            tabs: saved
                .tabs
                .into_iter()
                .map(|t| TabView {
                    path: t.path,
                    pinned: t.pinned,
                })
                .collect(),
        };
    }
    let screen = match s.start_screen.as_str() {
        "last" | "home" => "",
        other if prefs::START_SCREENS.iter().any(|(v, _)| *v == other) => other,
        _ => "",
    };
    StartTabs {
        tabs: vec![TabView {
            path: format!("/{screen}"),
            pinned: false,
        }],
        active: 0,
    }
}

/// Remembers the open tabs for "Open on launch: Tabs from last time". Saved only while that is
/// picked, so tab changes don't rewrite settings otherwise.
#[tauri::command]
#[specta::specta]
pub fn save_tabs(core: State<'_, Arc<Core>>, tabs: Vec<TabView>, active: u32) {
    if core.settings().start_screen != "last" {
        return;
    }
    let saved = SavedTabs {
        tabs: tabs
            .into_iter()
            .map(|t| SavedTab {
                path: t.path,
                pinned: t.pinned,
            })
            .collect(),
        active: active as usize,
    }
    .sanitized();
    if core.settings().session != saved {
        core.update_settings(|s| s.session = saved);
    }
}

/// Every setting that `settings.set` and `settings.reset` take.
#[tauri::command]
#[specta::specta]
pub fn prefs(core: State<'_, Arc<Core>>) -> Vec<PrefView> {
    let s = core.settings();
    prefs::PREFS
        .iter()
        .map(|p| {
            let (kind, options, (min, max, step), placeholder) = match p.kind {
                PrefKind::Bool => ("bool", &[][..], (0, 0, 0), ""),
                PrefKind::Choice(o) => ("choice", o, (0, 0, 0), ""),
                PrefKind::Range { min, max, step } => ("range", &[][..], (min, max, step), ""),
                PrefKind::Text { placeholder } => ("text", &[][..], (0, 0, 0), placeholder),
            };
            PrefView {
                key: p.key.to_owned(),
                section: p.section.to_owned(),
                title: p.title.to_owned(),
                description: p.description.to_owned(),
                keywords: p.keywords.iter().map(|k| (*k).to_owned()).collect(),
                kind: kind.to_owned(),
                options: options
                    .iter()
                    .map(|(v, l)| PrefOption {
                        value: (*v).to_owned(),
                        label: (*l).to_owned(),
                    })
                    .collect(),
                min,
                max,
                step,
                placeholder: placeholder.to_owned(),
                value: prefs::get(&s, p.key).unwrap_or_default(),
                default: prefs::default_value(p.key).unwrap_or_default(),
            }
        })
        .collect()
}

/// An installed theme. The UI reads `doc` with `readTheme` and derives the tokens itself.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ThemeView {
    pub id: String,
    pub name: String,
    pub author: String,
    /// `dark` or `light`.
    pub base: String,
    pub builtin: bool,
    /// The theme document as JSON.
    pub doc: String,
}

/// What the window should look like: the mode and the theme for each base.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct AppearanceView {
    /// `system`, `dark` or `light`.
    pub mode: String,
    /// Theme documents as JSON.
    pub dark: String,
    pub light: String,
    /// The Google Fonts the two themes use, to register with the webview as they download.
    pub fonts: Vec<FontView>,
    /// The picture behind the app, if any.
    pub background: Option<BackgroundView>,
}

/// The app background picture and how see-through things are over it.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct BackgroundView {
    /// File name under `bg://`.
    pub file: String,
    /// How strongly the picture shows, 0–100.
    pub image: u8,
    /// How opaque pages and panels stay, 0–100.
    pub panels: u8,
    /// Blur radius in px, 0–40.
    pub blur: u8,
    /// Darkening, 0–100.
    pub dim: u8,
    /// Saturation in percent, 0–200; 100 leaves it.
    pub saturation: u8,
    /// Accent-color wash, 0–100.
    pub tint: u8,
    /// Darkened edges, 0–100.
    pub vignette: u8,
    /// CRT-style lines, 0–100.
    pub scanlines: u8,
    /// Film grain, 0–100.
    pub grain: u8,
}

/// A Google Fonts family on disk, served over `font://`.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct FontView {
    pub family: String,
    /// Empty until downloaded.
    pub files: Vec<FontFileView>,
    pub loading: bool,
    pub error: Option<String>,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct FontFileView {
    pub weight: u16,
    /// Path under `font://`, like `space-grotesk/400.ttf`.
    pub path: String,
}

#[tauri::command]
#[specta::specta]
pub fn appearance(core: State<'_, Arc<Core>>) -> AppearanceView {
    AppearanceView {
        mode: core.settings().appearance.mode.name().to_owned(),
        dark: core.appearance_theme(Base::Dark).to_json(),
        light: core.appearance_theme(Base::Light).to_json(),
        fonts: core
            .appearance_fonts()
            .into_iter()
            .map(|family| font_view(&core, family))
            .collect(),
        background: core.settings().appearance.background.map(|b| BackgroundView {
            file: b.file,
            image: b.image,
            panels: b.panels,
            blur: b.blur,
            dim: b.dim,
            saturation: b.saturation,
            tint: b.tint,
            vignette: b.vignette,
            scanlines: b.scanlines,
            grain: b.grain,
        }),
    }
}

/// A Google family's files, downloading them on first ask.
fn font_view(core: &Core, family: String) -> FontView {
    let r = core.google_font(&family);
    FontView {
        files: r
            .data
            .map(|d| {
                d.files
                    .iter()
                    .map(|f| FontFileView {
                        weight: f.weight,
                        path: f.path.clone(),
                    })
                    .collect()
            })
            .unwrap_or_default(),
        loading: r.loading,
        error: r.error,
        family,
    }
}

/// Google Fonts families the theme editor previews before they are saved in a theme in use.
/// Downloads happen on the worker; this only reads what is on disk so far.
#[tauri::command]
#[specta::specta]
pub fn font_files(core: State<'_, Arc<Core>>, families: Vec<String>) -> Vec<FontView> {
    families
        .into_iter()
        .take(4)
        .filter(|f| rbx_core::fonts::valid_family(f))
        .map(|f| font_view(&core, f))
        .collect()
}

/// The UI reports whether the OS prefers dark, so System mode knows which theme is on screen.
#[tauri::command]
#[specta::specta]
pub fn set_system_dark(core: State<'_, Arc<Core>>, dark: bool) {
    core.set_system_dark(dark);
}

/// Google Fonts families for the font picker.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct FontCatalogView {
    /// Matches for the query, most popular first, at most `FONT_MATCHES`.
    pub fonts: Vec<FontInfoView>,
    pub loading: bool,
    pub error: Option<String>,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct FontInfoView {
    pub family: String,
    pub category: String,
}

const FONT_MATCHES: usize = 50;

/// Families whose name contains `query` (any case), only monospace ones when `mono`. Names that
/// start with the query come first.
#[tauri::command]
#[specta::specta]
pub fn font_catalog(core: State<'_, Arc<Core>>, query: String, mono: bool) -> FontCatalogView {
    let r = core.font_catalog();
    let q = query.trim().to_ascii_lowercase();
    let mut fonts: Vec<(bool, FontInfoView)> = r
        .data
        .iter()
        .flat_map(|c| c.iter())
        .filter(|f| !mono || f.monospace())
        .filter_map(|f| {
            let name = f.family.to_ascii_lowercase();
            name.contains(&q).then(|| {
                (
                    !name.starts_with(&q),
                    FontInfoView {
                        family: f.family.clone(),
                        category: f.category.clone(),
                    },
                )
            })
        })
        .collect();
    // Stable: popularity order holds within each group.
    fonts.sort_by_key(|f| f.0);
    fonts.truncate(FONT_MATCHES);
    FontCatalogView {
        fonts: fonts.into_iter().map(|f| f.1).collect(),
        loading: r.loading,
        error: r.error,
    }
}

/// Every installed theme: built-ins first, then the user's by name.
#[tauri::command]
#[specta::specta]
pub fn themes(core: State<'_, Arc<Core>>) -> Vec<ThemeView> {
    core.themes()
        .into_iter()
        .map(|(t, builtin)| ThemeView {
            doc: t.to_json(),
            base: t.base.name().to_owned(),
            id: t.id,
            name: t.name,
            author: t.author,
            builtin,
        })
        .collect()
}

fn servers_tab(s: &str) -> ServersTab {
    if s.trim() == "invited" {
        ServersTab::Invited
    } else {
        ServersTab::Owned
    }
}

/// Opens the system file picker (XDG desktop portal, or zenity without one) for a background
/// picture.
fn pick_background(app: &AppHandle) {
    let app = app.clone();
    // The dialog blocks until the user answers, so it gets its own thread.
    std::thread::spawn(move || {
        let mut dialog = rfd::FileDialog::new()
            .set_title("Background picture")
            .add_filter("Images", &["png", "jpg", "jpeg", "gif", "webp"]);
        let window = app.get_webview_window("main");
        if let Some(w) = &window {
            dialog = dialog.set_parent(w);
        }
        let Some(path) = dialog.pick_file() else {
            return;
        };
        if let Some(core) = app.try_state::<Arc<Core>>()
            && let Err(e) = core.set_background(&path.to_string_lossy())
        {
            core.notice(e, true);
        }
    });
}

/// Asks where to save the settings file, then writes it. The dialog gets its own thread.
fn export_settings(app: &AppHandle) {
    let app = app.clone();
    std::thread::spawn(move || {
        let mut dialog = rfd::FileDialog::new()
            .set_title("Save settings")
            .set_file_name("bonsai-settings.json")
            .add_filter("Settings", &["json"]);
        if let Some(w) = app.get_webview_window("main") {
            dialog = dialog.set_parent(&w);
        }
        let Some(path) = dialog.save_file() else {
            return;
        };
        let Some(core) = app.try_state::<Arc<Core>>() else {
            return;
        };
        match core.export_settings(&path) {
            Ok(()) => core.notice(format!("Saved settings to {}", path.display()), false),
            Err(e) => core.notice(e, true),
        }
    });
}

/// Asks for a settings file, then takes its settings. The dialog gets its own thread.
fn import_settings(app: &AppHandle) {
    let app = app.clone();
    std::thread::spawn(move || {
        let mut dialog = rfd::FileDialog::new()
            .set_title("Load settings")
            .add_filter("Settings", &["json"]);
        if let Some(w) = app.get_webview_window("main") {
            dialog = dialog.set_parent(&w);
        }
        let Some(path) = dialog.pick_file() else {
            return;
        };
        let Some(core) = app.try_state::<Arc<Core>>() else {
            return;
        };
        match core.import_settings(&path) {
            Ok(0) => core.notice("Those settings match yours already.", false),
            Ok(1) => core.notice("Loaded 1 setting.", false),
            Ok(n) => core.notice(format!("Loaded {n} settings."), false),
            Err(e) => core.notice(e, true),
        }
    });
}

/// Plain text for a bug report: versions, platform, account count, cache size and the
/// settings not at their defaults. No names, ids, paths or tokens.
fn diagnostics(app: &AppHandle, core: &Core) -> String {
    use std::fmt::Write;
    let info = app.package_info();
    let store = core.store();
    let mut out = String::new();
    let _ = writeln!(out, "{} {}", info.name, info.version);
    let _ = writeln!(out, "OS: {} {}", std::env::consts::OS, std::env::consts::ARCH);
    let webview = tauri::webview_version().unwrap_or_else(|e| format!("unknown ({e})"));
    let _ = writeln!(out, "Webview: {webview}");
    if cfg!(target_os = "linux") {
        for var in ["XDG_SESSION_TYPE", "XDG_CURRENT_DESKTOP"] {
            let v = std::env::var(var).unwrap_or_default();
            let _ = writeln!(out, "{var}: {v}");
        }
    }
    let _ = writeln!(out, "Accounts: {}", store.accounts.accounts.len());
    let _ = writeln!(out, "Offline: {}", store.offline);
    if let Some(u) = store.cache_usage {
        let _ = writeln!(
            out,
            "Cache: {} MB data, {} MB images",
            u.data >> 20,
            u.images >> 20
        );
    }
    drop(store);
    let changed = prefs::changed(&core.settings());
    if changed.is_empty() {
        out.push_str("Settings: defaults\n");
    } else {
        out.push_str("Settings:\n");
        for (k, v) in changed {
            let _ = writeln!(out, "  {k} = {v}");
        }
    }
    out
}

/// Splits `"<first> <rest>"` from a command argument; `what` names the parts in the error.
fn two(inv: &Invocation, what: &str) -> Result<(String, String), String> {
    let a = inv.arg.as_deref().unwrap_or_default().trim();
    a.split_once(' ')
        .map(|(x, y)| (x.trim().to_owned(), y.trim().to_owned()))
        .filter(|(x, y)| !x.is_empty() && !y.is_empty())
        .ok_or_else(|| format!("Give {what}."))
}

/// Settings commands and `list.more` for private servers. `None` for other commands.
pub(super) fn run(
    app: &AppHandle,
    core: &Core,
    id: CommandId,
    inv: &Invocation,
    account: Option<AccountId>,
) -> Option<Result<(), String>> {
    let acting = || account.ok_or_else(|| "Add an account first.".to_owned());
    let arg = || inv.arg.clone().unwrap_or_default();
    let r = match id {
        CommandId::SetPrivacy => (|| {
            let (key, value) = two(inv, "a setting and a value")?;
            core.set_privacy(acting()?, &key, &value)
        })(),
        CommandId::SetDisplayName => (|| core.set_display_name(acting()?, &arg()))(),
        CommandId::AddServerUser => (|| {
            let (server, who) = two(inv, "a server id and a user")?;
            let server = parse_id(Some(server), "server")?;
            let a = acting()?;
            core.with_user(&who, move |c, user| {
                c.change_server(a, server, ServerChange::AddUser(user))
            })
        })(),
        CommandId::RemoveServerUser => (|| {
            let (server, user) = two(inv, "a server id and a user id")?;
            let server = parse_id(Some(server), "server")?;
            let user = parse_id(Some(user), "user")?;
            core.change_server(acting()?, server, ServerChange::RemoveUser(user));
            Ok(())
        })(),
        CommandId::SetMode => match Mode::from_name(&arg()) {
            Some(m) => {
                core.update_settings(|s| s.appearance.mode = m);
                Ok(())
            }
            None => Err("Pick system, dark or light.".into()),
        },
        CommandId::ApplyTheme => core.apply_theme(arg().trim()),
        CommandId::CycleTheme => {
            core.cycle_theme();
            Ok(())
        }
        CommandId::DeleteTheme => core.delete_theme(arg().trim()),
        CommandId::DuplicateTheme => core
            .duplicate_theme(arg().trim())
            .and_then(|id| core.apply_theme(&id)),
        CommandId::SetThemeFont => core.set_theme_font(FontSlot::Sans, &arg()),
        CommandId::SetThemeMonoFont => core.set_theme_font(FontSlot::Mono, &arg()),
        // The editor sends the whole document; it is checked like an imported one.
        CommandId::SaveTheme => Theme::parse(&arg()).and_then(|t| core.save_theme(t)),
        CommandId::SetBackground => core.set_background(&arg()),
        CommandId::PickBackground => {
            pick_background(app);
            Ok(())
        }
        CommandId::ClearBackground => {
            core.clear_background();
            Ok(())
        }
        CommandId::SetBackgroundLevel => core.set_background_level(&arg()),
        CommandId::ResetBackgroundEffects => core.reset_background_effects(),
        CommandId::SetSetting => {
            // Checked on a copy first so a bad value changes nothing.
            let a = arg();
            prefs::apply(&mut core.settings(), &a).map(|p| {
                core.update_settings(|s| {
                    let _ = prefs::apply(s, &a);
                });
                if p.section == "data" {
                    core.tidy_soon();
                }
            })
        }
        CommandId::ResetSettings => {
            let target = arg();
            let target = if target.trim().is_empty() {
                "all"
            } else {
                target.trim()
            };
            prefs::reset(&mut core.settings(), target).map(|n| {
                core.update_settings(|s| {
                    let _ = prefs::reset(s, target);
                });
                let msg = match n {
                    0 => "Already at the defaults.".to_owned(),
                    1 => "Reset 1 setting.".to_owned(),
                    n => format!("Reset {n} settings."),
                };
                core.notice(&msg, false);
            })
        }
        CommandId::SetShortcut => (|| {
            // "<command> <shortcut>" binds it; "<command>" alone leaves the command without one.
            let a = arg();
            let a = a.trim();
            let (key, keys) = a.split_once(char::is_whitespace).unwrap_or((a, ""));
            let id = CommandId::ALL
                .iter()
                .copied()
                .find(|c| c.key() == key)
                .ok_or_else(|| format!("\"{key}\" isn't a command."))?;
            let shortcut = match keys.trim() {
                "" => None,
                k => Some(Shortcut::parse(k)?),
            };
            let mut registry = Registry::default();
            registry.load_overrides(core.settings().shortcuts);
            let taken = registry.assign(id, shortcut);
            let overrides = registry.overrides().clone();
            core.update_settings(|s| s.shortcuts = overrides);
            if let Some(first) = taken.first() {
                let title = registry.get(*first).title;
                core.notice(format!("Removed it from \"{title}\"."), false);
            }
            Ok(())
        })(),
        CommandId::ResetShortcuts => {
            // An argument resets one command; none resets them all.
            let a = arg();
            match a.trim() {
                "" => {
                    core.update_settings(|s| s.shortcuts.clear());
                    Ok(())
                }
                key => match CommandId::ALL.iter().copied().find(|c| c.key() == key) {
                    Some(id) => {
                        core.update_settings(|s| {
                            s.shortcuts.remove(&id);
                        });
                        Ok(())
                    }
                    None => Err(format!("\"{key}\" isn't a command.")),
                },
            }
        }
        CommandId::SetDensity => match Density::from_name(&arg()) {
            Some(d) => {
                core.update_settings(|s| s.density = d);
                Ok(())
            }
            None => Err("Pick compact or comfortable.".into()),
        },
        CommandId::SetAvatarView => match arg().trim().to_ascii_lowercase().as_str() {
            "3d" => {
                core.update_settings(|s| s.avatar_3d = true);
                Ok(())
            }
            "2d" | "image" => {
                core.update_settings(|s| s.avatar_3d = false);
                Ok(())
            }
            _ => Err("Pick 3d or 2d.".into()),
        },
        CommandId::SetPlayerCommand => {
            // `{uri}` marks where the join link goes; the launcher adds it at the end if missing.
            let cmd = arg().trim().to_owned();
            core.update_settings(|s| s.player_command = Some(cmd).filter(|c| !c.is_empty()));
            core.notice("Launch command saved.", false);
            Ok(())
        }
        CommandId::SetFormatLocale => {
            let tag = arg().trim().to_owned();
            if !valid_format_locale(&tag) {
                return Some(Err("Use a language tag like en-US or de-DE.".into()));
            }
            core.update_settings(|s| s.format_locale = tag);
            Ok(())
        }
        CommandId::SetNotify => (|| {
            // "<kind>" flips it; "<kind> on" or "<kind> off" sets it.
            let a = arg();
            let mut parts = a.split_whitespace();
            let kind = parts.next().unwrap_or_default();
            let current = core
                .settings()
                .notify
                .get(kind)
                .ok_or_else(|| format!("Pick one of: {}.", NotifyPrefs::KINDS.join(", ")))?;
            let on = match parts.next() {
                Some("on") => true,
                Some("off") => false,
                None => !current,
                Some(_) => return Err("Say on or off.".to_owned()),
            };
            core.update_settings(|s| {
                s.notify.set(kind, on);
            });
            Ok(())
        })(),
        CommandId::ToggleSidebarScreen => (|| {
            // "<screen>" flips it; "<screen> on" shows it, "<screen> off" hides it.
            let a = arg();
            let mut parts = a.split_whitespace();
            let screen = parts.next().ok_or_else(|| "Which screen?".to_owned())?;
            let hidden = match parts.next() {
                Some("on") => false,
                Some("off") => true,
                None => !core.settings().sidebar.is_hidden(screen),
                Some(_) => return Err("Say on or off.".to_owned()),
            };
            let mut prefs = core.settings().sidebar;
            prefs.set_hidden(screen, hidden)?;
            core.update_settings(|s| s.sidebar = prefs);
            Ok(())
        })(),
        CommandId::SetSidebarOrder => (|| {
            let mut prefs = core.settings().sidebar;
            prefs.set_order(arg().split_whitespace())?;
            core.update_settings(|s| s.sidebar = prefs);
            Ok(())
        })(),
        CommandId::ResetSidebar => {
            core.update_settings(|s| s.sidebar = SidebarPrefs::default());
            Ok(())
        }
        CommandId::SignOutOthers => acting().map(|a| core.sign_out_others(a)),
        CommandId::LockPin => acting().map(|a| core.lock_pin(a)),
        // The settings screen asks for the PIN and calls `unlock_pin`.
        CommandId::UnlockPin => Err("Open Settings to enter the PIN.".into()),
        // The dialog takes the code and calls `approve_enter`.
        CommandId::ApproveCode => Err("Open the command bar to enter the code.".into()),
        CommandId::ExportSettings => {
            export_settings(app);
            Ok(())
        }
        CommandId::ImportSettings => {
            import_settings(app);
            Ok(())
        }
        CommandId::CopyDiagnostics => super::copy(app, core, diagnostics(app, core), "Copied diagnostics."),
        CommandId::ClearCache => {
            core.clear_cache();
            Ok(())
        }
        CommandId::LoadMore => {
            let tab = inv.arg.as_deref()?.trim().strip_prefix("my_servers:")?;
            acting().map(|a| drop(core.my_private_servers(a, servers_tab(tab), true)))
        }
        _ => return None,
    };
    Some(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inv(arg: &str) -> Invocation {
        Invocation {
            id: String::new(),
            arg: Some(arg.into()),
            as_account: None,
            context: None,
        }
    }

    #[test]
    fn checks_format_locales() {
        assert!(valid_format_locale(""));
        assert!(valid_format_locale("de-DE"));
        assert!(valid_format_locale("zh-Hant-TW"));
        assert!(!valid_format_locale("de_DE"));
        assert!(!valid_format_locale("de--DE"));
        assert!(!valid_format_locale("<script>"));
    }

    #[test]
    fn notify_kinds_round_trip() {
        let mut n = NotifyPrefs::default();
        for k in NotifyPrefs::KINDS {
            let was = n.get(k).unwrap();
            assert!(n.set(k, !was));
            assert_eq!(n.get(k), Some(!was));
        }
        assert!(!n.set("nope", true));
        assert_eq!(n.get("nope"), None);
    }

    #[test]
    fn splits_two_part_arguments() {
        assert_eq!(
            two(&inv(" whoCanTradeWithMe  Friends "), "x").unwrap(),
            ("whoCanTradeWithMe".into(), "Friends".into())
        );
        assert!(two(&inv("onlyone"), "x").is_err());
        assert!(two(&inv(""), "x").is_err());
        assert_eq!(servers_tab("invited"), ServersTab::Invited);
        assert_eq!(servers_tab("anything"), ServersTab::Owned);
    }
}

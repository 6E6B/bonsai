//! Settings: your account (what Roblox has on file, display name, privacy), private servers
//! across experiences, and app preferences (theme, density, launch command, disk cache).
//! `list.more` loads more private servers with the key `my_servers:<owned|invited>`.

use std::sync::Arc;

use rbx_core::commands::CommandId;
use rbx_core::experience::ServerChange;
use rbx_core::me::ServersTab;
use rbx_core::settings::{Density, Mode, NotifyPrefs, SidebarPrefs, valid_format_locale};
use rbx_core::themes::{Base, FontSlot};
use rbx_core::{AccountId, Core};
use serde::Serialize;
use specta::Type;
use tauri::State;

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
            .map(|family| {
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
            })
            .collect(),
    }
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
        CommandId::SetDensity => match Density::from_name(&arg()) {
            Some(d) => {
                core.update_settings(|s| s.density = d);
                Ok(())
            }
            None => Err("Pick compact or comfortable.".into()),
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

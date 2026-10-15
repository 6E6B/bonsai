//! Settings: your account (what Roblox has on file, display name, privacy), private servers
//! across experiences, and app preferences (theme, density, launch command, disk cache).
//! `list.more` loads more private servers with the key `my_servers:<owned|invited>`.

use std::sync::Arc;

use rbx_core::commands::CommandId;
use rbx_core::experience::ServerChange;
use rbx_core::me::ServersTab;
use rbx_core::settings::{Density, NotifyPrefs, Theme, valid_format_locale};
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
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    /// `system`, `dark` or `light`.
    pub theme: String,
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
    }
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
        theme: s.theme.name().to_owned(),
        density: s.density.name().to_owned(),
        player_command: s.player_command.unwrap_or_default(),
        format_locale: s.format_locale,
        notify: NotifyView {
            friend_joins: s.notify.friend_joins,
            trades: s.notify.trades,
            chats: s.notify.chats,
            friend_requests: s.notify.friend_requests,
            experience_updates: s.notify.experience_updates,
        },
        cache_data: usage.map(|u| u.data as f64),
        cache_images: usage.map(|u| u.images as f64),
    }
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
        CommandId::SetTheme => match Theme::from_name(&arg()) {
            Some(t) => {
                core.update_settings(|s| s.theme = t);
                Ok(())
            }
            None => Err("Pick system, dark or light.".into()),
        },
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

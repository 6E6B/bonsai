//! Settings > Plugins and the install dialog. See `docs/plugins.md`.

use std::sync::Arc;

use bonsai_core::commands::{CommandId, Registry};
use bonsai_core::plugins::{PendingInstall, Permission, PluginInfo, Refused, Status, action_tier};
use bonsai_core::{AccountId, Core};
use serde::Serialize;
use specta::Type;
use tauri::{AppHandle, Manager, State};

use super::{Id, Invocation};

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PermissionView {
    pub key: String,
    pub label: String,
    /// Granted per account, so it needs accounts ticked.
    pub per_account: bool,
}

/// A built-in command a plugin may run or propose.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ActionView {
    pub key: String,
    pub title: String,
    /// 1 harmless, 2 social, 3 value.
    pub tier: u8,
}

/// Something the manifest asked for that the plugin won't get.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct RefusedView {
    pub what: String,
    pub why: String,
}

#[derive(Clone, Copy, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum PluginState {
    Off,
    Loading,
    Running,
    /// Asks for more than was agreed to.
    Review,
    Failed,
}

/// What a plugin asks for, shown in Settings and the install dialog.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Asks {
    pub permissions: Vec<PermissionView>,
    pub actions: Vec<ActionView>,
    pub refused: Vec<RefusedView>,
    /// It needs accounts ticked to do anything per account.
    pub needs_accounts: bool,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PluginView {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: Option<String>,
    /// The folder, for unpacked plugins.
    pub path: Option<String>,
    pub enabled: bool,
    pub state: PluginState,
    /// Why it failed to load.
    pub error: Option<String>,
    /// What it asks for beyond what was agreed to, while in review.
    pub review: Vec<String>,
    /// Titles of the commands it registered.
    pub commands: Vec<String>,
    pub asks: Option<Asks>,
    /// Accounts it may use.
    pub accounts: Vec<Id>,
}

/// A plugin folder waiting in the install dialog.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PluginInstallView {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: Option<String>,
    pub source: String,
    pub unpacked: bool,
    /// The installed version it replaces.
    pub replaces: Option<String>,
    /// Permission and action keys not agreed to before.
    pub added: Vec<String>,
    pub asks: Asks,
    /// Accounts ticked to start with.
    pub accounts: Vec<Id>,
}

fn permission_label(p: Permission) -> &'static str {
    match p {
        Permission::ReadPublic => "Read users, games, items and communities",
        Permission::ReadAccount => "Read friends, presence, inventory, trades and messages",
        Permission::Notify => "Show desktop alerts",
        Permission::ClipboardWrite => "Copy text",
    }
}

fn asks(m: &bonsai_core::plugins::Manifest) -> Asks {
    let registry = Registry::default();
    let actions: Vec<ActionView> = m
        .actions
        .iter()
        .filter_map(|a| {
            let id = CommandId::from_key(a)?;
            Some(ActionView {
                key: a.clone(),
                title: registry.get(id).title.to_owned(),
                tier: action_tier(a)? as u8,
            })
        })
        .collect();
    Asks {
        needs_accounts: !actions.is_empty() || m.permissions.iter().any(|p| p.per_account()),
        permissions: m
            .permissions
            .iter()
            .map(|p| PermissionView {
                key: p.key().to_owned(),
                label: permission_label(*p).to_owned(),
                per_account: p.per_account(),
            })
            .collect(),
        actions,
        refused: m.refused.iter().map(refused).collect(),
    }
}

fn refused(r: &Refused) -> RefusedView {
    RefusedView {
        what: r.what.clone(),
        why: r.why.clone(),
    }
}

fn view(p: PluginInfo) -> PluginView {
    let (state, error, review, commands) = match p.status {
        Status::Off => (PluginState::Off, None, Vec::new(), Vec::new()),
        Status::Loading => (PluginState::Loading, None, Vec::new(), Vec::new()),
        Status::Running { commands } => (
            PluginState::Running,
            None,
            Vec::new(),
            commands.into_iter().map(|c| c.title).collect(),
        ),
        Status::NeedsReview(more) => (PluginState::Review, None, more, Vec::new()),
        Status::Failed(e) => (PluginState::Failed, Some(e), Vec::new(), Vec::new()),
    };
    let m = p.manifest.as_ref();
    PluginView {
        name: m.map(|m| m.name.clone()).unwrap_or_else(|| p.id.clone()),
        version: m.map(|m| m.version.clone()).unwrap_or_default(),
        author: m.and_then(|m| m.author.clone()),
        path: p.unpacked.then(|| p.dir.display().to_string()),
        enabled: p.enabled,
        state,
        error,
        review,
        commands,
        asks: m.map(asks),
        accounts: p.accounts.iter().map(|a| Id(a.0)).collect(),
        id: p.id,
    }
}

pub fn install_view(p: &PendingInstall) -> PluginInstallView {
    let m = &p.manifest;
    PluginInstallView {
        id: m.id.clone(),
        name: m.name.clone(),
        version: m.version.clone(),
        author: m.author.clone(),
        source: p.source.display().to_string(),
        unpacked: p.unpacked,
        replaces: p.replaces.clone(),
        added: p.added.clone(),
        asks: asks(m),
        accounts: p.accounts.iter().map(|a| Id(a.0)).collect(),
    }
}

/// Installed and unpacked plugins.
#[tauri::command]
#[specta::specta]
pub fn plugins(core: State<'_, Arc<Core>>) -> Vec<PluginView> {
    core.plugins().into_iter().map(view).collect()
}

/// Installs the plugin in the dialog, allowed to use `accounts`.
#[tauri::command]
#[specta::specta]
pub fn plugin_install_finish(core: State<'_, Arc<Core>>, accounts: Vec<Id>) -> Result<(), String> {
    core.finish_plugin_install(accounts.into_iter().map(|a| AccountId(a.0)).collect())
}

#[tauri::command]
#[specta::specta]
pub fn plugin_install_cancel(core: State<'_, Arc<Core>>) {
    core.cancel_plugin_install();
}

/// Reads a plugin folder for the install dialog: `path`, or one the user picks when it is
/// empty. Off the main thread, since both read the disk or wait on a dialog.
fn stage(app: &AppHandle, path: String, unpacked: bool) {
    let app = app.clone();
    std::thread::spawn(move || {
        let path = if path.trim().is_empty() {
            let mut dialog = rfd::FileDialog::new().set_title(if unpacked {
                "Load unpacked plugin"
            } else {
                "Install plugin"
            });
            if let Some(w) = app.get_webview_window("main") {
                dialog = dialog.set_parent(&w);
            }
            let Some(dir) = dialog.pick_folder() else {
                return;
            };
            dir.to_string_lossy().into_owned()
        } else {
            path
        };
        if let Some(core) = app.try_state::<Arc<Core>>()
            && let Err(e) = core.stage_plugin_install(&path, unpacked)
        {
            core.notice(e, true);
        }
    });
}

pub fn run(
    app: &AppHandle,
    core: &Core,
    id: CommandId,
    inv: &Invocation,
    account: Option<AccountId>,
) -> Option<Result<(), String>> {
    let arg = || inv.arg.clone().unwrap_or_default().trim().to_owned();
    let acting = || account.ok_or_else(|| "Add an account first.".to_owned());
    Some(match id {
        CommandId::InstallPlugin => {
            stage(app, arg(), false);
            Ok(())
        }
        CommandId::LoadUnpackedPlugin => {
            stage(app, arg(), true);
            Ok(())
        }
        CommandId::EnablePlugin => core.set_plugin_enabled(&arg(), true),
        CommandId::DisablePlugin => core.set_plugin_enabled(&arg(), false),
        CommandId::ReloadPlugin => core.reload_plugin(&arg()),
        CommandId::ReviewPlugin => core.review_plugin(&arg()),
        CommandId::RemovePlugin => core.remove_plugin(&arg()),
        CommandId::GrantPlugin => acting().and_then(|a| core.set_plugin_account(&arg(), a, true)),
        CommandId::RevokePlugin => acting().and_then(|a| core.set_plugin_account(&arg(), a, false)),
        _ => return None,
    })
}

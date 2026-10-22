//! Settings > Plugins, the install and action dialogs, and plugin commands. See
//! `docs/plugins.md`.

use std::sync::Arc;

use bonsai_core::commands::{CommandId, CommandRef, PluginCmd, Registry};
use bonsai_core::plugins::{
    Activity, Arg, ButtonStyle, Cell, DEFAULT_CAP, Node, OpenView, PACKAGE_EXTENSION, PendingActions,
    PendingBudget, PendingInstall, Permission, PluginBudget, PluginCap, PluginCommand, PluginInfo,
    PluginOpen, Refused, Setting, SettingKind, Status, action_tier, cost_label,
};
use bonsai_core::store::StoreData;
use bonsai_core::{AccountId, Core};
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::{AppHandle, Manager, State};

use super::{CommandInfo, Id, Invocation};

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
    /// Events it listens to, while running.
    pub listens: Vec<String>,
    /// Seconds between runs of each timer, while running.
    pub timers: Vec<u32>,
    pub settings: Vec<PluginSettingView>,
    /// Hourly caps on tier 2 actions it runs without a click.
    pub caps: Vec<PluginCapView>,
    /// Daily budgets for tier 3 actions it runs without a click, by account it may use. Empty
    /// when it has no actions a budget covers.
    pub budgets: Vec<PluginBudgetView>,
    pub asks: Option<Asks>,
    /// Accounts it may use.
    pub accounts: Vec<Id>,
    /// SHA-256 of the files installed, as hex. None for unpacked plugins.
    pub hash: Option<String>,
}

#[derive(Clone, Copy, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum PluginSettingKind {
    Bool,
    Choice,
    Range,
    Text,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PluginOptionView {
    pub value: String,
    pub label: String,
}

/// One of a plugin's `[[settings]]`. Values cross as text, as `plugin.set` takes them.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PluginSettingView {
    pub key: String,
    pub title: String,
    pub description: String,
    pub kind: PluginSettingKind,
    pub options: Vec<PluginOptionView>,
    pub min: i32,
    pub max: i32,
    pub step: i32,
    pub placeholder: String,
    /// `on`/`off`, an option value, a number or text.
    pub value: String,
    pub default: String,
}

/// How often a plugin may run one tier 2 action an hour without a click.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PluginCapView {
    pub key: String,
    pub title: String,
    pub value: u32,
    /// The most it can be; the user can only lower it.
    pub max: u32,
}

/// What a plugin may spend on one account a day without a click, and what it spent in the last
/// 24 hours. Zero is no budget: it asks each time.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PluginBudgetView {
    pub account: Id,
    pub robux: u32,
    pub trades: u32,
    pub spent_robux: Id,
    pub spent_trades: u32,
    /// Robux paid out of community funds; the account's own Robux are `robux`.
    pub payouts: u32,
    pub spent_payouts: Id,
    /// It may run actions that spend Robux, accept trades, or pay out community funds.
    pub has_robux: bool,
    pub has_trades: bool,
    pub has_payouts: bool,
}

/// The budget dialog's plugin and account.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct BudgetDialogView {
    pub plugin: String,
    pub plugin_name: String,
    pub account_name: String,
    pub budget: PluginBudgetView,
}

fn budget_view(b: &PluginBudget) -> PluginBudgetView {
    PluginBudgetView {
        account: Id(b.account.0),
        robux: b.budget.robux,
        trades: b.budget.trades,
        spent_robux: Id(b.spent.robux.max(0) as u64),
        spent_trades: b.spent.trades,
        payouts: b.budget.payouts,
        spent_payouts: Id(b.spent.payouts.max(0) as u64),
        has_robux: b.robux,
        has_trades: b.trades,
        has_payouts: b.payouts,
    }
}

pub fn budget_dialog_view(p: &PendingBudget, s: &StoreData) -> BudgetDialogView {
    BudgetDialogView {
        plugin: p.plugin.clone(),
        plugin_name: p.plugin_name.clone(),
        account_name: s
            .accounts
            .accounts
            .iter()
            .find(|a| a.id == p.current.account)
            .map(|a| a.title().to_owned())
            .unwrap_or_default(),
        budget: budget_view(&p.current),
    }
}

/// One plugin write, or one it tried and was refused.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ActivityView {
    /// Unix seconds.
    pub at: Id,
    pub plugin_name: String,
    pub account: Id,
    pub account_name: String,
    pub command: String,
    pub title: String,
    pub arg: Option<String>,
    /// Ran without a click.
    pub unattended: bool,
    /// Why it failed or was refused.
    pub error: Option<String>,
    /// What it spent, for actions that spend Robux.
    pub robux: Option<Id>,
}

/// The account in the "Allow automation" dialog.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct AutomationView {
    pub account: Id,
    /// The username to type.
    pub name: String,
    pub display_name: String,
}

pub fn automation_view(account: AccountId, s: &StoreData) -> AutomationView {
    let a = s.accounts.accounts.iter().find(|a| a.id == account);
    AutomationView {
        account: Id(account.0),
        name: a.map(|a| a.name.clone()).unwrap_or_default(),
        display_name: a.map(|a| a.title().to_owned()).unwrap_or_default(),
    }
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
    /// SHA-256 of the files it would install, as hex, to compare with what the author published.
    pub hash: Option<String>,
    /// It came from a `.bonsaiplugin` package.
    pub package: bool,
}

/// One action in the action dialog.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ProposedView {
    pub key: String,
    pub title: String,
    pub arg: Option<String>,
    pub tier: u8,
    /// What it may cost: "Up to R$ 50", "R$ 100" or "1 trade".
    pub cost: Option<String>,
}

/// Actions a plugin command proposed, for the user to run or cancel.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PluginActionsView {
    pub plugin: String,
    pub plugin_name: String,
    pub account: Id,
    /// The account's label or display name.
    pub account_name: String,
    pub actions: Vec<ProposedView>,
}

/// A screen a plugin asked to open, as a built-in command to run once per `seq`.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PluginOpenView {
    pub seq: Id,
    pub command: String,
    pub arg: String,
}

/// The open plugin view, for the shell. The tree itself comes from `plugin_view`, fetched when
/// `rev` changes.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PluginViewHead {
    pub seq: Id,
    pub rev: Id,
    pub plugin_name: String,
    pub title: String,
    /// The plugin is handling an event; controls wait.
    pub busy: bool,
    pub error: Option<String>,
}

pub fn view_head(o: &OpenView) -> PluginViewHead {
    PluginViewHead {
        seq: Id(o.seq),
        rev: Id(o.rev),
        plugin_name: o.plugin_name.clone(),
        title: o.title.clone(),
        busy: o.busy,
        error: o.error.clone(),
    }
}

/// A table cell or stat value. Numbers are shown in the user's number format.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(untagged)]
pub enum ViewCell {
    Text(String),
    Number(f64),
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ViewColumn {
    pub label: String,
    /// Right-aligned.
    pub end: bool,
}

#[derive(Clone, Copy, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum ViewButtonStyle {
    Default,
    Primary,
    Danger,
}

/// One part of a plugin view. See `docs/plugins.md#views`.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ViewNode {
    Text {
        text: String,
        muted: bool,
        mono: bool,
    },
    Heading {
        text: String,
    },
    Row {
        children: Vec<ViewNode>,
    },
    Column {
        children: Vec<ViewNode>,
    },
    Stat {
        label: String,
        value: ViewCell,
    },
    Table {
        columns: Vec<ViewColumn>,
        rows: Vec<Vec<ViewCell>>,
        /// Sent with the row number when a row is picked.
        action: Option<String>,
    },
    Input {
        key: String,
        label: String,
        value: String,
        placeholder: String,
        /// Sent when Enter is pressed in it.
        action: Option<String>,
    },
    Button {
        label: String,
        action: String,
        style: ViewButtonStyle,
    },
}

fn cell_view(c: &Cell) -> ViewCell {
    match c {
        Cell::Text(t) => ViewCell::Text(t.clone()),
        Cell::Number(n) => ViewCell::Number(*n),
    }
}

fn node_view(n: &Node) -> ViewNode {
    match n {
        Node::Text { text, muted, mono } => ViewNode::Text {
            text: text.clone(),
            muted: *muted,
            mono: *mono,
        },
        Node::Heading(text) => ViewNode::Heading { text: text.clone() },
        Node::Row(c) => ViewNode::Row {
            children: c.iter().map(node_view).collect(),
        },
        Node::Column(c) => ViewNode::Column {
            children: c.iter().map(node_view).collect(),
        },
        Node::Stat { label, value } => ViewNode::Stat {
            label: label.clone(),
            value: cell_view(value),
        },
        Node::Table {
            columns,
            rows,
            action,
        } => ViewNode::Table {
            columns: columns
                .iter()
                .map(|c| ViewColumn {
                    label: c.label.clone(),
                    end: c.end,
                })
                .collect(),
            rows: rows.iter().map(|r| r.iter().map(cell_view).collect()).collect(),
            action: action.clone(),
        },
        Node::Input {
            key,
            label,
            value,
            placeholder,
            action,
        } => ViewNode::Input {
            key: key.clone(),
            label: label.clone(),
            value: value.clone(),
            placeholder: placeholder.clone(),
            action: action.clone(),
        },
        Node::Button { label, action, style } => ViewNode::Button {
            label: label.clone(),
            action: action.clone(),
            style: match style {
                ButtonStyle::Default => ViewButtonStyle::Default,
                ButtonStyle::Primary => ViewButtonStyle::Primary,
                ButtonStyle::Danger => ViewButtonStyle::Danger,
            },
        },
    }
}

/// The open view's tree.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PluginViewBody {
    pub seq: Id,
    pub rev: Id,
    pub root: ViewNode,
}

/// One input's text, sent back with a view event.
#[derive(Clone, Debug, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ViewValue {
    pub key: String,
    pub value: String,
}

pub fn actions_view(p: &PendingActions, s: &StoreData) -> PluginActionsView {
    let registry = Registry::default();
    PluginActionsView {
        plugin: p.plugin.clone(),
        plugin_name: p.plugin_name.clone(),
        account: Id(p.account.0),
        account_name: s
            .accounts
            .accounts
            .iter()
            .find(|a| a.id == p.account)
            .map(|a| a.title().to_owned())
            .unwrap_or_default(),
        actions: p
            .actions
            .iter()
            .map(|a| ProposedView {
                key: a.id.key().to_owned(),
                title: registry.get(a.id).title.to_owned(),
                arg: a.arg.clone(),
                tier: a.tier as u8,
                cost: cost_label(a),
            })
            .collect(),
    }
}

pub fn open_view(o: &PluginOpen) -> PluginOpenView {
    PluginOpenView {
        seq: Id(o.seq),
        command: o.command.key().to_owned(),
        arg: o.arg.clone(),
    }
}

/// A plugin command for the command bar, titled with the plugin's name. It has no shortcuts
/// until the user gives it some.
pub fn command_info(c: PluginCommand, registry: &Registry) -> CommandInfo {
    let mut keywords = c.command.keywords;
    keywords.push("plugin".into());
    keywords.extend(c.plugin_name.split_whitespace().map(str::to_lowercase));
    let cmd = CommandRef::Plugin(PluginCmd {
        plugin: c.plugin,
        id: c.command.id,
    });
    CommandInfo {
        id: cmd.key(),
        title: format!("{}: {}", c.plugin_name, c.command.title),
        keywords,
        arg: match c.command.arg {
            Arg::None => super::Arg::None,
            Arg::User => super::Arg::User,
            Arg::Account => super::Arg::Account,
            // Plain ids; the command bar has no game or item picker that gives one.
            Arg::Text | Arg::Game | Arg::Item => super::Arg::Text,
        },
        shortcuts: registry
            .shortcuts_of(&cmd)
            .iter()
            .map(super::Keys::from)
            .collect(),
        rebound: registry.is_rebound_ref(&cmd),
        context: c.command.context,
    }
}

fn permission_label(p: Permission) -> &'static str {
    match p {
        Permission::ReadPublic => "Read users, games, items and communities",
        Permission::ReadAccount => "Read friends, presence, inventory, trades and messages",
        Permission::Notify => "Show desktop alerts",
        Permission::ClipboardWrite => "Copy text",
        Permission::Net => "Send credential-free HTTPS requests to the hosts below",
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
            .chain(m.network_hosts.iter().map(|h| PermissionView {
                key: format!("net:{h}"),
                label: format!("HTTPS: {h}"),
                per_account: true,
            }))
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

fn setting_view(s: &Setting, value: &serde_json::Value) -> PluginSettingView {
    let mut v = PluginSettingView {
        key: s.key.clone(),
        title: s.title.clone(),
        description: s.description.clone(),
        kind: PluginSettingKind::Bool,
        options: Vec::new(),
        min: 0,
        max: 0,
        step: 1,
        placeholder: String::new(),
        value: Setting::text(value),
        default: Setting::text(&s.default),
    };
    match &s.kind {
        SettingKind::Bool => {}
        SettingKind::Choice(options) => {
            v.kind = PluginSettingKind::Choice;
            v.options = options
                .iter()
                .map(|(value, label)| PluginOptionView {
                    value: value.clone(),
                    label: label.clone(),
                })
                .collect();
        }
        SettingKind::Range { min, max, step } => {
            v.kind = PluginSettingKind::Range;
            let small = |n: i64| n.clamp(i32::MIN.into(), i32::MAX.into()) as i32;
            (v.min, v.max, v.step) = (small(*min), small(*max), small(*step));
        }
        SettingKind::Text { placeholder } => {
            v.kind = PluginSettingKind::Text;
            v.placeholder = placeholder.clone();
        }
    }
    v
}

fn cap_view(c: &PluginCap) -> PluginCapView {
    PluginCapView {
        key: c.command.key().to_owned(),
        title: Registry::default().get(c.command).title.to_owned(),
        value: c.value,
        max: DEFAULT_CAP,
    }
}

fn view(
    p: PluginInfo,
    settings: Vec<(Setting, serde_json::Value)>,
    caps: Vec<PluginCap>,
    budgets: Vec<PluginBudget>,
) -> PluginView {
    let (state, error, review, commands, listens, timers) = match p.status {
        Status::Off => (
            PluginState::Off,
            None,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ),
        Status::Loading => (
            PluginState::Loading,
            None,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ),
        Status::Running {
            commands,
            events,
            timers,
        } => (
            PluginState::Running,
            None,
            Vec::new(),
            commands.into_iter().map(|c| c.title).collect(),
            events,
            timers
                .into_iter()
                .map(|t| t.min(u32::MAX.into()) as u32)
                .collect(),
        ),
        Status::NeedsReview(more) => (
            PluginState::Review,
            None,
            more,
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ),
        Status::Failed(e) => (
            PluginState::Failed,
            Some(e),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ),
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
        listens,
        timers,
        settings: settings.iter().map(|(s, v)| setting_view(s, v)).collect(),
        caps: caps.iter().map(cap_view).collect(),
        budgets: budgets.iter().map(budget_view).collect(),
        asks: m.map(asks),
        accounts: p.accounts.iter().map(|a| Id(a.0)).collect(),
        hash: p.hash,
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
        hash: p.hash.clone(),
        package: p.staged.is_some(),
    }
}

/// Installed and unpacked plugins.
#[tauri::command]
#[specta::specta]
pub fn plugins(core: State<'_, Arc<Core>>) -> Vec<PluginView> {
    core.plugins()
        .into_iter()
        .map(|p| {
            let settings = core.plugin_settings(&p.id);
            let caps = core.plugin_caps(&p.id);
            let budgets = core.plugin_budgets(&p.id);
            view(p, settings, caps, budgets)
        })
        .collect()
}

/// Accounts plugins may automate.
#[tauri::command]
#[specta::specta]
pub fn plugin_automation(core: State<'_, Arc<Core>>) -> Vec<Id> {
    core.plugin_automation().into_iter().map(|a| Id(a.0)).collect()
}

/// Turns automation on for the account in the dialog, when `typed` is its name.
#[tauri::command]
#[specta::specta]
pub fn plugin_automation_confirm(core: State<'_, Arc<Core>>, typed: String) -> Result<(), String> {
    core.confirm_automation(&typed)
}

#[tauri::command]
#[specta::specta]
pub fn plugin_automation_cancel(core: State<'_, Arc<Core>>) {
    core.cancel_automation();
}

/// Saves the budget in the budget dialog. Zero turns it off.
#[tauri::command]
#[specta::specta]
pub fn plugin_budget_save(
    core: State<'_, Arc<Core>>,
    robux: u32,
    trades: u32,
    payouts: u32,
) -> Result<(), String> {
    core.save_budget(robux, trades, payouts)
}

#[tauri::command]
#[specta::specta]
pub fn plugin_budget_cancel(core: State<'_, Arc<Core>>) {
    core.cancel_budget();
}

/// Most activity entries sent at once.
const ACTIVITY: usize = 1000;

/// Plugin writes, newest first.
#[tauri::command]
#[specta::specta]
pub fn plugin_activity(core: State<'_, Arc<Core>>) -> Vec<ActivityView> {
    let registry = Registry::default();
    let s = core.store();
    core.plugin_activity(ACTIVITY)
        .into_iter()
        .map(|a: Activity| ActivityView {
            at: Id(a.at.max(0) as u64),
            plugin_name: a.plugin_name,
            account: Id(a.account.0),
            account_name: s
                .accounts
                .accounts
                .iter()
                .find(|m| m.id == a.account)
                .map(|m| m.title().to_owned())
                .unwrap_or_else(|| a.account.0.to_string()),
            title: CommandId::from_key(&a.command)
                .map_or_else(|| a.command.clone(), |c| registry.get(c).title.to_owned()),
            command: a.command,
            arg: a.arg,
            unattended: a.unattended,
            error: a.error,
            robux: a.robux.map(|r| Id(r.max(0) as u64)),
        })
        .collect()
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

/// The open plugin view's tree, or none when it is closed.
#[tauri::command]
#[specta::specta]
pub fn plugin_view(core: State<'_, Arc<Core>>) -> Option<PluginViewBody> {
    let s = core.store();
    let o = s.plugin_view.as_ref()?;
    Some(PluginViewBody {
        seq: Id(o.seq),
        rev: Id(o.rev),
        root: node_view(&o.root),
    })
}

/// Sends a button press, Enter in an input or a picked table row to the plugin, with every
/// input's text. `row` counts from 1.
#[tauri::command]
#[specta::specta]
pub fn plugin_view_event(
    core: State<'_, Arc<Core>>,
    seq: Id,
    action: String,
    values: Vec<ViewValue>,
    row: Option<u32>,
) -> Result<(), String> {
    core.plugin_view_event(
        seq.0,
        &action,
        values.into_iter().map(|v| (v.key, v.value)).collect(),
        row.map(|r| r as usize),
    )
}

#[tauri::command]
#[specta::specta]
pub fn plugin_view_close(core: State<'_, Arc<Core>>) {
    core.close_plugin_view();
}

/// Runs the actions in the action dialog.
#[tauri::command]
#[specta::specta]
pub fn plugin_actions_run(core: State<'_, Arc<Core>>) -> Result<(), String> {
    core.confirm_plugin_actions()
}

#[tauri::command]
#[specta::specta]
pub fn plugin_actions_cancel(core: State<'_, Arc<Core>>) {
    core.cancel_plugin_actions();
}

/// Reads a plugin folder for the install dialog: `path`, or one the user picks when it is
/// empty. Off the main thread, since both read the disk or wait on a dialog.
fn stage(app: &AppHandle, path: String, unpacked: bool) {
    let app = app.clone();
    std::thread::spawn(move || {
        let path = if path.trim().is_empty() {
            let mut dialog = rfd::FileDialog::new();
            if let Some(w) = app.get_webview_window("main") {
                dialog = dialog.set_parent(&w);
            }
            // A package, or a folder's plugin.toml; a picker can't take both files and folders.
            let picked = if unpacked {
                dialog.set_title("Load unpacked plugin").pick_folder()
            } else {
                dialog
                    .set_title("Install plugin")
                    .add_filter("Bonsai plugins", &[PACKAGE_EXTENSION, "toml"])
                    .pick_file()
            };
            let Some(picked) = picked else {
                return;
            };
            picked.to_string_lossy().into_owned()
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

/// Asks where to save a plugin's package, then writes it.
fn pack(app: &AppHandle, id: String) {
    let app = app.clone();
    std::thread::spawn(move || {
        let mut dialog = rfd::FileDialog::new()
            .set_title("Pack plugin")
            .set_file_name(format!("{id}.{PACKAGE_EXTENSION}"))
            .add_filter("Bonsai plugins", &[PACKAGE_EXTENSION]);
        if let Some(w) = app.get_webview_window("main") {
            dialog = dialog.set_parent(&w);
        }
        let Some(path) = dialog.save_file() else {
            return;
        };
        let Some(core) = app.try_state::<Arc<Core>>() else {
            return;
        };
        match core.pack_plugin(&id, &path) {
            Ok(hash) => core.notice(format!("Packed to {}. SHA-256 {hash}", path.display()), false),
            Err(e) => core.notice(e, true),
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
        CommandId::PackPlugin => {
            let id = arg();
            if core.plugins().iter().any(|p| p.id == id) {
                pack(app, id);
                Ok(())
            } else {
                Err(format!("No plugin \"{id}\"."))
            }
        }
        CommandId::RemovePlugin => core.remove_plugin(&arg()),
        CommandId::GrantPlugin => acting().and_then(|a| core.set_plugin_account(&arg(), a, true)),
        CommandId::RevokePlugin => acting().and_then(|a| core.set_plugin_account(&arg(), a, false)),
        CommandId::SetPluginCap => {
            // "plugin action number"
            let raw = arg();
            let mut words = raw.split_whitespace();
            match (words.next(), words.next(), words.next().map(str::parse::<u32>)) {
                (Some(plugin), Some(action), Some(Ok(n))) => core.set_plugin_cap(plugin, action, n),
                _ => Err("Give a plugin, an action and a number.".into()),
            }
        }
        CommandId::SetPluginBudget => acting().and_then(|a| core.request_budget(&arg(), a)),
        CommandId::AllowAutomation => acting().and_then(|a| core.request_automation(a)),
        CommandId::BlockAutomation => acting().map(|a| {
            core.block_automation(a);
            core.notice("Automation is off for this account.", false);
        }),
        CommandId::StopPlugins => {
            core.stop_plugins();
            Ok(())
        }
        CommandId::SetPluginSetting => {
            // "plugin key value"; the value is the rest, spaces and all.
            let raw = inv.arg.as_deref().unwrap_or_default().trim_start();
            let (plugin, rest) = raw.split_once(' ').unwrap_or((raw, ""));
            let rest = rest.trim_start();
            let (key, value) = rest.split_once(' ').unwrap_or((rest, ""));
            if key.is_empty() {
                Err("Give a plugin, a setting and a value.".into())
            } else {
                core.set_plugin_setting(plugin, key.trim(), value)
            }
        }
        _ => return None,
    })
}

//! Installed plugins: one folder each under [`Paths::plugins_dir`](crate::account::Paths), plus
//! unpacked folders loaded in place for development. Each enabled plugin runs in its own VM on its
//! own thread. Which plugins are on, which accounts each may use and what the user agreed to are
//! kept in `plugins.json`. Tier checks live here; see `docs/plugins.md`.

use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::{Arc, Weak};
use std::time::Duration;

use bonsai_plugin::{Ctx, Limits, Outcome, Plugin, Read};
use serde::{Deserialize, Serialize};

use crate::AccountId;
use crate::commands::{CommandId, Effect, PluginCmd, Registry, Tier};
use crate::core::{Core, Inner};

pub use bonsai_plugin::{Arg, Command, Manifest, Permission, Refused, Setting, SettingKind};

mod automation;
mod budget;
mod events;
mod reads;
mod storage;

pub use automation::{Activity, DEFAULT_CAP, PluginCap, unattended_refusal};
pub use budget::{Budget, Charge, PendingBudget, PluginBudget, Spent, budget_refusal, cost_label};

pub(crate) use reads::{presence_view, status};

pub const MANIFEST: &str = "plugin.toml";
/// What one installed plugin may hold, so a stray folder can't fill the disk.
const MAX_FILES: usize = 500;
const MAX_BYTES: u64 = 20 * 1024 * 1024;
const MAX_DEPTH: usize = 8;
/// Most actions one command may propose at once.
const MAX_ACTIONS: usize = 100;
/// Calls queued per plugin before more are refused as busy.
const QUEUE: usize = 8;

/// Runs a built-in command as an account on a plugin's behalf. The shell sets it, since that is
/// where built-in commands are dispatched; core decides what may run.
pub type ActionRunner = Arc<dyn Fn(AccountId, CommandId, Option<String>) -> Result<(), String> + Send + Sync>;

/// Why a manifest action is refused, or `None` to allow it: only tier 1 to 3 writes.
pub fn action_refusal(registry: &Registry, key: &str) -> Option<String> {
    let Some(id) = CommandId::from_key(key) else {
        return Some("Not a command in this version of Bonsai.".into());
    };
    match registry.get(id).effect {
        Effect::Read | Effect::Local => Some("Not an action plugins can run.".into()),
        Effect::Write(Tier::Account) => Some("Account control is never available to plugins.".into()),
        Effect::Write(_) => None,
    }
}

/// Reads `plugin.toml` in `dir` and refuses actions plugins can't have.
pub fn read_manifest(dir: &Path) -> Result<Manifest, String> {
    let path = dir.join(MANIFEST);
    let size = std::fs::metadata(&path)
        .map_err(|_| format!("No {MANIFEST} in {}.", dir.display()))?
        .len();
    if size > 64 * 1024 {
        return Err(format!("{MANIFEST} is too large."));
    }
    let text = std::fs::read_to_string(&path).map_err(|e| format!("Can't read {MANIFEST}: {e}"))?;
    let mut m = Manifest::parse(&text).map_err(|e| e.to_string())?;
    let registry = Registry::default();
    m.refuse_actions(|a| action_refusal(&registry, a));
    if dir.file_name().and_then(|n| n.to_str()) != Some(m.id.as_str()) {
        return Err(format!(
            "The folder must be named \"{}\", after the plugin id.",
            m.id
        ));
    }
    Ok(m)
}

/// The tier of an allowed action, for the install dialog.
pub fn action_tier(key: &str) -> Option<Tier> {
    CommandId::from_key(key).and_then(|id| Registry::default().get(id).effect.tier())
}

/// What the user agreed to for one plugin. Unknown fields are ignored.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
struct Saved {
    #[serde(default)]
    enabled: bool,
    /// Accounts it may use.
    #[serde(default)]
    accounts: Vec<AccountId>,
    /// Permissions and actions agreed to. A manifest that asks for more waits for review.
    #[serde(default)]
    permissions: Vec<String>,
    #[serde(default)]
    actions: Vec<String>,
    /// Values the user changed from the manifest's `[[settings]]` defaults.
    #[serde(default)]
    settings: BTreeMap<String, serde_json::Value>,
    /// Hourly caps the user lowered, by tier 2 action.
    #[serde(default)]
    caps: BTreeMap<String, u32>,
    /// What it may spend a day without a click, by account. See `budget.rs`.
    #[serde(default)]
    budgets: BTreeMap<AccountId, budget::Budget>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct SavedFile {
    #[serde(default)]
    plugins: BTreeMap<String, Saved>,
    /// Folders loaded in place with "Load unpacked plugin".
    #[serde(default)]
    unpacked: Vec<PathBuf>,
    /// Accounts plugins may run tier 2 actions on without a click. Off by default.
    #[serde(default)]
    automation: Vec<AccountId>,
}

impl SavedFile {
    fn load(path: &Path) -> SavedFile {
        std::fs::read_to_string(path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }
}

/// Where a plugin is in its life.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Status {
    Off,
    Loading,
    Running {
        commands: Vec<Command>,
        /// Events it listens to.
        events: Vec<String>,
        /// Seconds between runs of each timer.
        timers: Vec<u64>,
    },
    /// It asks for more than the user agreed to; listed until they review it.
    NeedsReview(Vec<String>),
    Failed(String),
}

/// One installed or unpacked plugin, for Settings > Plugins.
#[derive(Clone, Debug)]
pub struct PluginInfo {
    /// The folder name, which is the manifest id when the manifest reads.
    pub id: String,
    pub dir: PathBuf,
    pub unpacked: bool,
    pub manifest: Option<Manifest>,
    pub enabled: bool,
    pub accounts: Vec<AccountId>,
    pub status: Status,
}

/// A plugin folder picked to install or load, shown in the install dialog until confirmed.
#[derive(Clone, Debug)]
pub struct PendingInstall {
    pub manifest: Manifest,
    pub source: PathBuf,
    /// Loaded in place instead of copied.
    pub unpacked: bool,
    /// The version it replaces, when it is an update or a review of one already installed.
    pub replaces: Option<String>,
    /// Permissions and actions not agreed to before. Everything, for a new plugin.
    pub added: Vec<String>,
    /// Accounts ticked to start with: the current grants.
    pub accounts: Vec<AccountId>,
}

/// A command a running plugin registered, for the command bar and menus.
#[derive(Clone, Debug)]
pub struct PluginCommand {
    pub plugin: String,
    pub plugin_name: String,
    pub command: Command,
}

/// A built-in command a plugin proposed, checked against what the user agreed to.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProposedAction {
    pub id: CommandId,
    pub arg: Option<String>,
    pub tier: Tier,
}

/// Actions waiting in the action dialog. They run only when the user confirms, and are checked
/// again then.
#[derive(Clone, Debug)]
pub struct PendingActions {
    pub plugin: String,
    pub plugin_name: String,
    pub account: AccountId,
    pub actions: Vec<ProposedAction>,
}

/// A screen a plugin asked to open, as a built-in open command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PluginOpen {
    pub seq: u64,
    pub command: CommandId,
    pub arg: String,
}

/// Plugin state in [`Inner`].
#[derive(Default)]
pub(crate) struct Plugins {
    entries: Vec<Entry>,
    saved: SavedFile,
    /// Bumped per worker start, so a load that finishes after a reload is ignored.
    seq: u64,
    runner: Option<ActionRunner>,
    /// `ctx.store` files read so far.
    data: storage::Data,
    log: automation::Log,
}

struct Entry {
    info: PluginInfo,
    /// Hash of the folder's file names, sizes and times; a change reloads the plugin.
    stamp: u64,
    worker: Option<Worker>,
}

/// A running plugin's thread. Dropping it stops the thread once the current call ends.
struct Worker {
    seq: u64,
    tx: std::sync::mpsc::SyncSender<Msg>,
}

/// Calls to a plugin's thread.
enum Msg {
    Run {
        command: String,
        ctx: Ctx,
        arg: Option<String>,
    },
    Event {
        event: &'static str,
        ctx: Ctx,
        payload: serde_json::Value,
    },
}

/// What a plugin ran when it returned an outcome.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Source {
    /// The user ran one of its commands.
    Command,
    /// An event or timer, with nobody watching.
    Background,
}

/// What [`Inner::sync_plugins`] should redo even when nothing on disk changed.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Force<'a> {
    None,
    One(&'a str),
}

/// The plugin side of [`bonsai_plugin::Host`], on the plugin's thread.
struct CoreHost {
    inner: Weak<Inner>,
}

impl bonsai_plugin::Host for CoreHost {
    fn run(&self, plugin: &str, account: u64, action: &str, arg: Option<&str>) -> Result<(), String> {
        let inner = self.inner.upgrade().ok_or("Bonsai is closing.")?;
        inner.plugin_run(plugin, account, action, arg)
    }
    fn read(&self, plugin: &str, account: u64, read: &Read) -> Result<serde_json::Value, String> {
        let inner = self.inner.upgrade().ok_or("Bonsai is closing.")?;
        inner.plugin_read(plugin, account, read)
    }
    fn log(&self, plugin: &str, line: &str) {
        let line: String = line.chars().take(500).collect();
        tracing::info!(target: "plugin", "{plugin}: {line}");
    }
    fn store_get(&self, plugin: &str, account: u64, key: &str) -> Result<serde_json::Value, String> {
        let inner = self.inner.upgrade().ok_or("Bonsai is closing.")?;
        inner.plugin_store_get(plugin, account, key)
    }
    fn store_set(
        &self,
        plugin: &str,
        account: u64,
        key: &str,
        value: serde_json::Value,
    ) -> Result<(), String> {
        let inner = self.inner.upgrade().ok_or("Bonsai is closing.")?;
        inner.plugin_store_set(plugin, account, key, value)
    }
    fn settings(&self, plugin: &str) -> serde_json::Map<String, serde_json::Value> {
        let Some(inner) = self.inner.upgrade() else {
            return serde_json::Map::new();
        };
        let p = inner.plugins.lock();
        let Some(m) = p
            .entries
            .iter()
            .find(|e| e.info.id == plugin)
            .and_then(|e| e.info.manifest.as_ref())
        else {
            return serde_json::Map::new();
        };
        setting_values(m, p.saved.plugins.get(plugin))
    }
}

/// Each manifest setting's value: the saved one when it is still valid, else the default.
fn setting_values(m: &Manifest, saved: Option<&Saved>) -> serde_json::Map<String, serde_json::Value> {
    m.settings
        .iter()
        .map(|s| {
            let v = saved
                .and_then(|x| x.settings.get(&s.key))
                .filter(|v| s.valid(v))
                .unwrap_or(&s.default);
            (s.key.clone(), v.clone())
        })
        .collect()
}

/// Serializes plugin disk work: installs, removes, saves and rescans.
static IO: parking_lot::Mutex<()> = parking_lot::Mutex::new(());

fn permission_keys(m: &Manifest) -> Vec<String> {
    m.permissions.iter().map(|p| p.key().to_owned()).collect()
}

/// What `m` asks for beyond what was agreed to.
fn asks_more(m: &Manifest, saved: Option<&Saved>) -> Vec<String> {
    let (perms, acts) = saved.map_or((&[][..], &[][..]), |s| (&s.permissions[..], &s.actions[..]));
    let mut out: Vec<String> = permission_keys(m)
        .into_iter()
        .filter(|p| !perms.contains(p))
        .collect();
    out.extend(m.actions.iter().filter(|a| !acts.contains(a)).cloned());
    out
}

/// Hashes the names, sizes and change times of a plugin's files, skipping dot files.
fn stamp(dir: &Path) -> u64 {
    let mut h = std::collections::hash_map::DefaultHasher::new();
    let mut seen = 0;
    walk(dir, 0, &mut |rel, meta| {
        rel.hash(&mut h);
        meta.len().hash(&mut h);
        meta.modified().ok().hash(&mut h);
        seen += 1;
        seen <= MAX_FILES
    });
    h.finish()
}

/// Calls `f` for each file under `dir` (sorted, dot names and symlinks skipped) until it returns
/// false.
fn walk(dir: &Path, depth: usize, f: &mut dyn FnMut(&Path, &std::fs::Metadata) -> bool) -> bool {
    walk_in(dir, Path::new(""), depth, f)
}

fn walk_in(
    root: &Path,
    rel: &Path,
    depth: usize,
    f: &mut dyn FnMut(&Path, &std::fs::Metadata) -> bool,
) -> bool {
    let Ok(rd) = std::fs::read_dir(root.join(rel)) else {
        return true;
    };
    let mut entries: Vec<_> = rd.flatten().collect();
    entries.sort_by_key(|e| e.file_name());
    for e in entries {
        let name = e.file_name();
        if name.to_string_lossy().starts_with('.') {
            continue;
        }
        let Ok(meta) = std::fs::symlink_metadata(e.path()) else {
            continue;
        };
        let path = rel.join(&name);
        if meta.is_dir() {
            if depth < MAX_DEPTH && !walk_in(root, &path, depth + 1, f) {
                return false;
            }
        } else if meta.is_file() && !f(&path, &meta) {
            return false;
        }
    }
    true
}

/// Copies a plugin folder: files only, no dot files or symlinks, within the size caps.
fn copy_plugin(from: &Path, to: &Path) -> Result<(), String> {
    let mut files = Vec::new();
    let mut bytes = 0;
    let mut too_big = false;
    walk(from, 0, &mut |rel, meta| {
        bytes += meta.len();
        files.push(rel.to_path_buf());
        too_big = files.len() > MAX_FILES || bytes > MAX_BYTES;
        !too_big
    });
    if too_big {
        return Err(format!(
            "The plugin is too large: at most {MAX_FILES} files and {} MB.",
            MAX_BYTES / 1024 / 1024
        ));
    }
    let err = |e: std::io::Error| format!("Can't copy the plugin: {e}");
    for rel in files {
        let dest = to.join(&rel);
        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent).map_err(err)?;
        }
        std::fs::copy(from.join(&rel), dest).map_err(err)?;
    }
    Ok(())
}

fn remove_dir(dir: &Path) -> std::io::Result<()> {
    match std::fs::remove_dir_all(dir) {
        Err(e) if e.kind() != std::io::ErrorKind::NotFound => Err(e),
        _ => Ok(()),
    }
}

/// A plugin folder read from disk, before it is matched with the running one.
struct Scanned {
    info: PluginInfo,
    stamp: u64,
}

/// Reads every installed and unpacked plugin. Disk only; touches no state.
fn scan(dir: &Path, saved: &SavedFile) -> Vec<Scanned> {
    let mut dirs: Vec<(PathBuf, bool)> = std::fs::read_dir(dir)
        .map(|rd| {
            rd.flatten()
                .filter(|e| !e.file_name().to_string_lossy().starts_with('.'))
                .filter(|e| e.file_type().is_ok_and(|t| t.is_dir()))
                .map(|e| (e.path(), false))
                .collect()
        })
        .unwrap_or_default();
    dirs.sort();
    dirs.extend(saved.unpacked.iter().map(|p| (p.clone(), true)));

    let mut out: Vec<Scanned> = Vec::new();
    for (path, unpacked) in dirs {
        let folder = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        let read = read_manifest(&path);
        let id = read.as_ref().map(|m| m.id.clone()).unwrap_or(folder);
        let s = saved.plugins.get(&id);
        let enabled = s.is_some_and(|s| s.enabled);
        let taken = out.iter().any(|o| o.info.id == id);
        let (manifest, status) = match read {
            _ if taken => (None, Status::Failed("Another plugin has the same id.".into())),
            Err(e) => (None, Status::Failed(e)),
            Ok(m) => {
                let more = asks_more(&m, s);
                let status = if !enabled {
                    Status::Off
                } else if more.is_empty() {
                    Status::Loading
                } else {
                    Status::NeedsReview(more)
                };
                (Some(m), status)
            }
        };
        out.push(Scanned {
            stamp: stamp(&path),
            info: PluginInfo {
                id,
                dir: path,
                unpacked,
                manifest,
                enabled,
                accounts: s.map(|s| s.accounts.clone()).unwrap_or_default(),
                status,
            },
        });
    }
    out
}

impl Inner {
    fn save_plugins(&self) {
        let saved = self.plugins.lock().saved.clone();
        let path = self.paths.plugins_file();
        let r = serde_json::to_string_pretty(&saved)
            .map_err(std::io::Error::other)
            .and_then(|json| {
                if let Some(p) = path.parent() {
                    std::fs::create_dir_all(p)?;
                }
                std::fs::write(&path, json)
            });
        if let Err(e) = r {
            tracing::warn!("writing plugins.json: {e}");
        }
    }

    /// Rereads plugins from disk and starts, stops or restarts each to match. A plugin whose
    /// files and settings didn't change keeps running. Blocking; call with [`IO`] held.
    fn sync_plugins(self: &Arc<Self>, force: Force<'_>) {
        let saved = self.plugins.lock().saved.clone();
        let fresh = scan(&self.paths.plugins_dir(), &saved);
        let mut p = self.plugins.lock();
        let mut old = std::mem::take(&mut p.entries);
        let mut changed = false;
        for s in fresh {
            let prev = old
                .iter()
                .position(|e| e.info.id == s.info.id && e.info.dir == s.info.dir)
                .map(|i| old.swap_remove(i));
            // A plugin that failed stays failed until its files change or it is reloaded.
            let same = prev.as_ref().is_some_and(|e| {
                e.stamp == s.stamp
                    && e.info.enabled == s.info.enabled
                    && e.info.manifest == s.info.manifest
                    && force != Force::One(&s.info.id)
            });
            if let (true, Some(mut e)) = (same, prev) {
                changed |= e.info.accounts != s.info.accounts;
                e.info.accounts = s.info.accounts;
                p.entries.push(e);
                continue;
            }
            changed = true;
            let mut e = Entry {
                info: s.info,
                stamp: s.stamp,
                worker: None,
            };
            if e.info.status == Status::Loading {
                p.seq += 1;
                let seq = p.seq;
                self.start_plugin(&mut e, seq);
            }
            p.entries.push(e);
        }
        drop(p);
        // Whatever is left in `old` was removed; dropping it stops its thread.
        changed |= !old.is_empty();
        drop(old);
        // The watcher calls this every few seconds; only a change repaints.
        if changed {
            self.store.write().plugin_rev += 1;
            self.repaint();
        }
    }

    fn start_plugin(self: &Arc<Self>, e: &mut Entry, seq: u64) {
        let Some(manifest) = e.info.manifest.clone() else {
            return;
        };
        let (tx, rx) = std::sync::mpsc::sync_channel::<Msg>(QUEUE);
        let weak = Arc::downgrade(self);
        let (id, dir) = (e.info.id.clone(), e.info.dir.clone());
        let spawned = std::thread::Builder::new()
            .name(format!("plugin-{id}"))
            .spawn(move || {
                let host = Rc::new(CoreHost { inner: weak.clone() });
                let loaded = Plugin::load(&dir, manifest, host, Limits::default());
                let status = match &loaded {
                    Ok(p) => Status::Running {
                        commands: p.commands().to_vec(),
                        events: p.events(),
                        timers: p.timers().iter().map(Duration::as_secs).collect(),
                    },
                    Err(e) => Status::Failed(e.to_string()),
                };
                if let Some(inner) = weak.upgrade() {
                    inner.plugin_loaded(&id, seq, status);
                }
                let Ok(plugin) = loaded else {
                    return;
                };
                // The first run of each timer is one interval after the plugin starts.
                let start = std::time::Instant::now();
                let mut due: Vec<std::time::Instant> = plugin.timers().iter().map(|d| start + *d).collect();
                // Keeps the VM until the plugin is stopped or reloaded, which drops the sender.
                loop {
                    let next = due.iter().copied().enumerate().min_by_key(|(_, t)| *t);
                    let msg = match next {
                        None => rx.recv().map_err(|_| ()),
                        Some((i, t)) => {
                            match rx.recv_timeout(t.saturating_duration_since(std::time::Instant::now())) {
                                Ok(m) => Ok(m),
                                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => Err(()),
                                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                                    due[i] = std::time::Instant::now() + plugin.timers()[i];
                                    let Some(inner) = weak.upgrade() else {
                                        return;
                                    };
                                    let Some(accounts) = inner.timer_accounts(&id, seq) else {
                                        return;
                                    };
                                    drop(inner);
                                    for account in accounts {
                                        let ctx = Ctx {
                                            account: account.0,
                                            now: crate::cache::now(),
                                        };
                                        let out = plugin.run_timer(i, ctx);
                                        let Some(inner) = weak.upgrade() else {
                                            return;
                                        };
                                        inner.plugin_outcome(&id, seq, account, Source::Background, out);
                                    }
                                    continue;
                                }
                            }
                        }
                    };
                    let Ok(msg) = msg else {
                        return;
                    };
                    let (account, source, out) = match msg {
                        Msg::Run { command, ctx, arg } => (
                            ctx.account,
                            Source::Command,
                            plugin.run_command(&command, ctx, arg.as_deref()),
                        ),
                        Msg::Event { event, ctx, payload } => (
                            ctx.account,
                            Source::Background,
                            plugin.run_event(event, ctx, &payload),
                        ),
                    };
                    let Some(inner) = weak.upgrade() else {
                        return;
                    };
                    inner.plugin_outcome(&id, seq, AccountId(account), source, out);
                }
            });
        match spawned {
            Ok(_) => e.worker = Some(Worker { seq, tx }),
            Err(err) => e.info.status = Status::Failed(format!("Can't start: {err}")),
        }
    }

    fn plugin_loaded(&self, id: &str, seq: u64, status: Status) {
        let mut p = self.plugins.lock();
        let Some(e) = p.entries.iter_mut().find(|e| e.info.id == id) else {
            return;
        };
        if e.worker.as_ref().is_some_and(|w| w.seq == seq) {
            if matches!(status, Status::Failed(_)) {
                e.worker = None;
            }
            e.info.status = status;
            drop(p);
            self.store.write().plugin_rev += 1;
            self.repaint();
        }
    }

    /// The name of the plugin whose worker `seq` is current, or `None` for a stale result.
    fn plugin_name(&self, id: &str, seq: u64) -> Option<String> {
        let p = self.plugins.lock();
        let e = p.entries.iter().find(|e| e.info.id == id)?;
        e.worker.as_ref().filter(|w| w.seq == seq)?;
        Some(
            e.info
                .manifest
                .as_ref()
                .map_or_else(|| id.to_owned(), |m| m.name.clone()),
        )
    }

    /// What a plugin returned. Errors and messages are toasts naming the plugin. From an event
    /// or timer, an alert is a desktop alert when the plugin may notify and it can't open
    /// screens. Returned actions run at once when they all may run unattended (tier 1, or tier 2
    /// under the cap with automation on); otherwise they wait in the action dialog.
    fn plugin_outcome(
        self: &Arc<Self>,
        id: &str,
        seq: u64,
        account: AccountId,
        source: Source,
        out: Result<Outcome, bonsai_plugin::Error>,
    ) {
        let Some(name) = self.plugin_name(id, seq) else {
            return;
        };
        let say = |text: &str, error: bool| {
            let text: String = text.chars().take(300).collect();
            self.toast(None, format!("{name}: {text}"), error);
            self.repaint();
        };
        match out {
            Err(e) => say(&e.to_string(), true),
            Ok(Outcome::Nothing) => {}
            Ok(Outcome::Toast(t)) => say(&t, false),
            Ok(Outcome::Alert { title, body, .. }) => {
                if source == Source::Background && account.0 != 0 && self.may_notify(id, account) {
                    let title: String = format!("{name}: {title}").chars().take(120).collect();
                    let body: String = body.chars().take(300).collect();
                    self.alert(account, "plugins", title, body);
                } else {
                    say(&format!("{title}. {body}"), false);
                }
            }
            Ok(Outcome::Open { .. }) if source == Source::Background => {
                say("Only commands can open screens.", true);
            }
            Ok(Outcome::Open { kind, id }) => match open_command(&kind, id) {
                Some((command, arg)) => {
                    let mut s = self.store.write();
                    let seq = s.plugin_open.as_ref().map_or(0, |o| o.seq) + 1;
                    s.plugin_open = Some(PluginOpen { seq, command, arg });
                    drop(s);
                    self.repaint();
                }
                None => say(&format!("Can't open \"{kind}\"."), true),
            },
            Ok(Outcome::Actions(list)) => {
                let list: Vec<(String, Option<String>)> = list.into_iter().map(|a| (a.id, a.arg)).collect();
                match self.check_actions(id, account, &list) {
                    Err(e) => say(&e, true),
                    Ok(actions) if actions.is_empty() => {}
                    // Tier 1 is harmless and easy to undo. After a click it counts as the user's.
                    Ok(actions) if actions.iter().all(|a| a.tier == Tier::Harmless) => {
                        let unattended = source == Source::Background;
                        self.run_actions(id, name.clone(), account, actions, unattended);
                    }
                    Ok(actions) if self.all_unattended(id, account, &actions) => {
                        self.run_actions(id, name.clone(), account, actions, true);
                    }
                    Ok(actions) => {
                        let mut s = self.store.write();
                        if s.plugin_actions.is_some() {
                            drop(s);
                            say("Another plugin's actions are waiting for you first.", true);
                            return;
                        }
                        s.plugin_actions = Some(PendingActions {
                            plugin: id.to_owned(),
                            plugin_name: name.clone(),
                            account,
                            actions,
                        });
                        drop(s);
                        self.repaint();
                    }
                }
            }
        }
    }

    /// Checks proposed actions against the plugin's grants: it is running, may use `account`,
    /// and the user agreed to each action, which is a tier 1 to 3 write.
    fn check_actions(
        &self,
        plugin: &str,
        account: AccountId,
        list: &[(String, Option<String>)],
    ) -> Result<Vec<ProposedAction>, String> {
        if list.len() > MAX_ACTIONS {
            return Err(format!("Proposed more than {MAX_ACTIONS} actions at once."));
        }
        let known = self
            .store
            .read()
            .accounts
            .accounts
            .iter()
            .any(|a| a.id == account);
        let p = self.plugins.lock();
        let running = p
            .entries
            .iter()
            .any(|e| e.info.id == plugin && matches!(e.info.status, Status::Running { .. }));
        if !running {
            return Err("It isn't running.".into());
        }
        let saved = p.saved.plugins.get(plugin);
        if !known || !saved.is_some_and(|s| s.accounts.contains(&account)) {
            return Err("It may not use this account. Allow it in Settings > Plugins.".into());
        }
        let agreed = saved.map(|s| &s.actions[..]).unwrap_or_default();
        let registry = Registry::default();
        list.iter()
            .map(|(key, arg)| {
                if let Some(why) = action_refusal(&registry, key) {
                    return Err(format!("Can't run \"{key}\": {why}"));
                }
                if !agreed.contains(key) {
                    return Err(format!("Isn't allowed to run \"{key}\"."));
                }
                let id = CommandId::from_key(key).ok_or_else(|| format!("No command \"{key}\"."))?;
                let tier = registry
                    .get(id)
                    .effect
                    .tier()
                    .ok_or_else(|| format!("Can't run \"{key}\"."))?;
                let arg: Option<String> = arg
                    .as_deref()
                    .map(str::trim)
                    .filter(|a| !a.is_empty())
                    .map(|a| a.chars().take(200).collect());
                if let Some(Err(e)) = budget::Spend::parse(id, arg.as_deref()) {
                    return Err(format!("Can't run \"{key}\": {e}"));
                }
                Ok(ProposedAction { id, arg, tier })
            })
            .collect()
    }

    /// Every action may run without a click now, counting the batch against the caps and
    /// budgets. Purchases count at the most they may cost.
    fn all_unattended(&self, plugin: &str, account: AccountId, actions: &[ProposedAction]) -> bool {
        let mut seen: std::collections::HashMap<CommandId, u32> = std::collections::HashMap::new();
        let mut pending = budget::Spent::default();
        actions.iter().all(|a| {
            let n = seen.entry(a.id).or_default();
            let ok = self.may_run_unattended(plugin, account, a, *n, pending).is_ok();
            *n += 1;
            if let Some(c) = budget::charge(a) {
                pending.add(c);
            }
            ok
        })
    }

    /// Runs checked actions one after another off the caller's thread, logs each and then says
    /// how it went. An unattended batch checks the automation switch, cap and budget before each
    /// one; the stop button drops the rest.
    fn run_actions(
        self: &Arc<Self>,
        plugin: &str,
        name: String,
        account: AccountId,
        actions: Vec<ProposedAction>,
        unattended: bool,
    ) {
        let epoch = self.plugins.lock().log.stop_gen;
        let inner = self.clone();
        let plugin = plugin.to_owned();
        self.rt.spawn_blocking(move || {
            let total = actions.len();
            let mut failed = Vec::new();
            for a in actions {
                let r = inner.run_logged(&plugin, &name, account, &a, unattended, epoch);
                if let Err(e) = r {
                    let stopped = inner.plugins.lock().log.stop_gen != epoch;
                    failed.push(e);
                    if stopped {
                        break;
                    }
                }
            }
            if inner.plugins.lock().log.stop_gen != epoch {
                return;
            }
            let ran = total - failed.len();
            let text = match failed.first() {
                None if total == 1 => format!("{name}: Ran 1 action."),
                None => format!("{name}: Ran {total} actions."),
                Some(e) => format!("{name}: Ran {ran} of {total} actions. {e}"),
            };
            inner.toast(Some(account), text, !failed.is_empty());
            inner.repaint();
        });
    }

    /// Runs disk work for plugins off the caller's thread, one job at a time.
    fn plugin_job(self: &Arc<Self>, job: impl FnOnce(&Arc<Inner>) -> Result<(), String> + Send + 'static) {
        let inner = self.clone();
        self.rt.spawn_blocking(move || {
            let _io = IO.lock();
            if let Err(e) = job(&inner) {
                inner.toast(None, e, true);
                inner.repaint();
            }
        });
    }
}

/// The built-in command that opens a screen for a returned `open`.
fn open_command(kind: &str, id: u64) -> Option<(CommandId, String)> {
    Some(match kind {
        "user" => (CommandId::OpenProfile, id.to_string()),
        "game" => (CommandId::OpenGame, id.to_string()),
        "group" => (CommandId::OpenCommunity, id.to_string()),
        "item" => (CommandId::OpenItem, format!("item:{id}")),
        "bundle" => (CommandId::OpenItem, format!("bundle:{id}")),
        _ => return None,
    })
}

/// Loads plugins at start and reloads one when its folder changes, so a plugin edited in a text
/// editor reloads by itself.
pub(crate) async fn watch_loop(inner: Arc<Inner>) {
    let mut tick = tokio::time::interval(Duration::from_secs(2));
    loop {
        tick.tick().await;
        let i = inner.clone();
        let _ = inner
            .rt
            .spawn_blocking(move || {
                let _io = IO.lock();
                i.sync_plugins(Force::None);
            })
            .await;
    }
}

/// Reads `plugins.json` and starts enabled plugins. Called once at start.
pub(crate) fn load(inner: &Arc<Inner>) {
    inner.plugins.lock().saved = SavedFile::load(&inner.paths.plugins_file());
    automation::load_log(inner);
    inner.plugin_job(|i| {
        i.sync_plugins(Force::None);
        Ok(())
    });
}

impl Core {
    /// Installed and unpacked plugins in folder order.
    pub fn plugins(&self) -> Vec<PluginInfo> {
        self.inner
            .plugins
            .lock()
            .entries
            .iter()
            .map(|e| e.info.clone())
            .collect()
    }

    /// Commands of running plugins.
    pub fn plugin_commands(&self) -> Vec<PluginCommand> {
        let p = self.inner.plugins.lock();
        p.entries
            .iter()
            .filter_map(|e| match (&e.info.status, &e.info.manifest) {
                (Status::Running { commands, .. }, Some(m)) => Some((&e.info.id, &m.name, commands)),
                _ => None,
            })
            .flat_map(|(id, name, cmds)| {
                cmds.iter().map(|c| PluginCommand {
                    plugin: id.clone(),
                    plugin_name: name.clone(),
                    command: c.clone(),
                })
            })
            .collect()
    }

    /// Sets how plugin actions run built-in commands. The shell calls it once at start.
    pub fn set_plugin_runner(&self, runner: ActionRunner) {
        self.inner.plugins.lock().runner = Some(runner);
    }

    /// Runs a plugin command as `account` on the plugin's thread. Returns once it is queued; the
    /// outcome arrives as a toast, an opened screen or the action dialog. With no `arg`, a
    /// command that takes a user, game or item uses `context` (`user:1`) when it is one.
    pub fn run_plugin_command(
        &self,
        cmd: &PluginCmd,
        arg: Option<String>,
        account: Option<AccountId>,
        context: Option<&str>,
    ) -> Result<(), String> {
        let p = self.inner.plugins.lock();
        let e = p
            .entries
            .iter()
            .find(|e| e.info.id == cmd.plugin)
            .ok_or_else(|| format!("No plugin \"{}\".", cmd.plugin))?;
        let name = e.info.manifest.as_ref().map_or(&e.info.id, |m| &m.name);
        let (Status::Running { commands, .. }, Some(worker)) = (&e.info.status, &e.worker) else {
            return Err(format!("{name} isn't running."));
        };
        let c = commands
            .iter()
            .find(|c| c.id == cmd.id)
            .ok_or_else(|| format!("{name} has no command \"{}\".", cmd.id))?;
        let kind = match c.arg {
            Arg::User => Some("user"),
            Arg::Game => Some("game"),
            Arg::Item => Some("item"),
            _ => None,
        };
        let arg = arg.filter(|a| !a.trim().is_empty()).or_else(|| {
            let (k, id) = context?.split_once(':')?;
            (Some(k) == kind).then(|| id.to_owned())
        });
        // It only learns an account it was allowed to use; otherwise it sees 0.
        let granted = account.filter(|a| {
            p.saved
                .plugins
                .get(&cmd.plugin)
                .is_some_and(|s| s.accounts.contains(a))
        });
        let msg = Msg::Run {
            command: cmd.id.clone(),
            ctx: Ctx {
                account: granted.map_or(0, |a| a.0),
                now: crate::cache::now(),
            },
            arg,
        };
        worker.tx.try_send(msg).map_err(|e| match e {
            std::sync::mpsc::TrySendError::Full(_) => format!("{name} is busy. Try again in a moment."),
            std::sync::mpsc::TrySendError::Disconnected(_) => format!("{name} stopped."),
        })
    }

    /// Runs the actions in the action dialog, after checking them again.
    pub fn confirm_plugin_actions(&self) -> Result<(), String> {
        let Some(pending) = self.inner.store.write().plugin_actions.take() else {
            return Err("Nothing to run.".into());
        };
        self.inner.repaint();
        let list: Vec<(String, Option<String>)> = pending
            .actions
            .iter()
            .map(|a| (a.id.key().to_owned(), a.arg.clone()))
            .collect();
        let actions = self
            .inner
            .check_actions(&pending.plugin, pending.account, &list)
            .map_err(|e| format!("{}: {e}", pending.plugin_name))?;
        self.inner.run_actions(
            &pending.plugin,
            pending.plugin_name,
            pending.account,
            actions,
            false,
        );
        Ok(())
    }

    pub fn cancel_plugin_actions(&self) {
        self.inner.store.write().plugin_actions = None;
        self.inner.repaint();
    }

    /// Reads a plugin folder (or its `plugin.toml`) and opens the install dialog. `unpacked`
    /// loads it in place for development instead of copying it. Reads the disk, so call it off
    /// the main thread.
    pub fn stage_plugin_install(&self, path: &str, unpacked: bool) -> Result<(), String> {
        let mut dir = PathBuf::from(path.trim());
        if dir.file_name().is_some_and(|n| n == MANIFEST) {
            dir.pop();
        }
        let dir =
            std::fs::canonicalize(&dir).map_err(|_| "Pick a plugin folder with a plugin.toml.".to_owned())?;
        let manifest = read_manifest(&dir)?;
        let installed = self.inner.paths.plugins_dir();
        if !unpacked && dir.parent() == std::fs::canonicalize(&installed).ok().as_deref() {
            return Err("That plugin is already installed.".into());
        }
        let p = self.inner.plugins.lock();
        let current = p.entries.iter().find(|e| e.info.id == manifest.id);
        let replaces = match current {
            Some(e) if e.info.unpacked != unpacked || (unpacked && e.info.dir != dir) => {
                return Err(format!(
                    "{} is already {}. Remove it first.",
                    manifest.name,
                    if e.info.unpacked {
                        "loaded unpacked"
                    } else {
                        "installed"
                    }
                ));
            }
            Some(e) => Some(
                e.info
                    .manifest
                    .as_ref()
                    .map(|m| m.version.clone())
                    .unwrap_or_default(),
            ),
            None => None,
        };
        let saved = p.saved.plugins.get(&manifest.id);
        let pending = PendingInstall {
            added: asks_more(&manifest, saved),
            accounts: saved.map(|s| s.accounts.clone()).unwrap_or_default(),
            manifest,
            source: dir,
            unpacked,
            replaces,
        };
        drop(p);
        self.inner.store.write().plugin_install = Some(pending);
        self.inner.repaint();
        Ok(())
    }

    /// Opens the install dialog again for a plugin whose manifest asks for more than was agreed.
    pub fn review_plugin(&self, id: &str) -> Result<(), String> {
        let p = self.inner.plugins.lock();
        let e = p
            .entries
            .iter()
            .find(|e| e.info.id == id)
            .ok_or_else(|| format!("No plugin \"{id}\"."))?;
        let manifest = e
            .info
            .manifest
            .clone()
            .ok_or_else(|| "The plugin doesn't load; fix it first.".to_owned())?;
        let saved = p.saved.plugins.get(id);
        let pending = PendingInstall {
            added: asks_more(&manifest, saved),
            accounts: e.info.accounts.clone(),
            replaces: Some(manifest.version.clone()),
            manifest,
            source: e.info.dir.clone(),
            unpacked: e.info.unpacked,
        };
        drop(p);
        self.inner.store.write().plugin_install = Some(pending);
        self.inner.repaint();
        Ok(())
    }

    /// Installs the plugin in the dialog, turned on and allowed to use `accounts`. Copies it in
    /// the background; errors come back as a toast.
    pub fn finish_plugin_install(&self, accounts: Vec<AccountId>) -> Result<(), String> {
        let Some(pending) = self.inner.store.write().plugin_install.take() else {
            return Err("Nothing to install.".into());
        };
        self.inner.repaint();
        let known: Vec<AccountId> = self.store().accounts.accounts.iter().map(|a| a.id).collect();
        let accounts: Vec<AccountId> = accounts.into_iter().filter(|a| known.contains(a)).collect();
        self.inner.plugin_job(move |inner| {
            let m = &pending.manifest;
            let target = inner.paths.plugins_dir().join(&m.id);
            if !pending.unpacked && pending.source != target {
                let tmp = inner.paths.plugins_dir().join(format!(".{}.new", m.id));
                remove_dir(&tmp).map_err(|e| e.to_string())?;
                if let Err(e) = copy_plugin(&pending.source, &tmp) {
                    let _ = remove_dir(&tmp);
                    return Err(e);
                }
                remove_dir(&target).map_err(|e| format!("Can't replace the old version: {e}"))?;
                std::fs::rename(&tmp, &target).map_err(|e| format!("Can't install: {e}"))?;
            }
            {
                let mut p = inner.plugins.lock();
                if pending.unpacked && !p.saved.unpacked.contains(&pending.source) {
                    p.saved.unpacked.push(pending.source.clone());
                }
                // Settings, caps and budgets survive an update.
                let settings = p
                    .saved
                    .plugins
                    .get(&m.id)
                    .map(|s| s.settings.clone())
                    .unwrap_or_default();
                let caps = p
                    .saved
                    .plugins
                    .get(&m.id)
                    .map(|s| s.caps.clone())
                    .unwrap_or_default();
                let mut budgets = p
                    .saved
                    .plugins
                    .get(&m.id)
                    .map(|s| s.budgets.clone())
                    .unwrap_or_default();
                budgets.retain(|a, _| accounts.contains(a));
                p.saved.plugins.insert(
                    m.id.clone(),
                    Saved {
                        enabled: true,
                        accounts,
                        permissions: permission_keys(m),
                        actions: m.actions.clone(),
                        settings,
                        caps,
                        budgets,
                    },
                );
            }
            inner.save_plugins();
            inner.sync_plugins(Force::One(&m.id));
            inner.toast(None, format!("Installed {}.", m.name), false);
            Ok(())
        });
        Ok(())
    }

    pub fn cancel_plugin_install(&self) {
        self.inner.store.write().plugin_install = None;
        self.inner.repaint();
    }

    fn change_plugin(&self, id: &str, f: impl FnOnce(&mut Saved)) -> Result<(), String> {
        {
            let mut p = self.inner.plugins.lock();
            if !p.entries.iter().any(|e| e.info.id == id) {
                return Err(format!("No plugin \"{id}\"."));
            }
            f(p.saved.plugins.entry(id.to_owned()).or_default());
        }
        self.inner.plugin_job(|inner| {
            inner.save_plugins();
            inner.sync_plugins(Force::None);
            Ok(())
        });
        Ok(())
    }

    /// Turns a plugin on or off. Off stops its thread and drops its commands.
    pub fn set_plugin_enabled(&self, id: &str, on: bool) -> Result<(), String> {
        let unreviewed = {
            let p = self.inner.plugins.lock();
            p.entries.iter().any(|e| {
                e.info.id == id
                    && e.info
                        .manifest
                        .as_ref()
                        .is_some_and(|m| !asks_more(m, p.saved.plugins.get(id)).is_empty())
            })
        };
        // Never-agreed permissions go through the dialog, not a switch.
        if on && unreviewed {
            return self.review_plugin(id);
        }
        self.change_plugin(id, |s| s.enabled = on)
    }

    /// Lets a plugin use an account, or stops it.
    pub fn set_plugin_account(&self, id: &str, account: AccountId, on: bool) -> Result<(), String> {
        if !self.store().accounts.accounts.iter().any(|a| a.id == account) {
            return Err("Pick one of your accounts.".into());
        }
        self.change_plugin(id, |s| {
            s.accounts.retain(|a| *a != account);
            if on {
                s.accounts.push(account);
            } else {
                s.budgets.remove(&account);
            }
        })
    }

    /// Changes one of a plugin's `[[settings]]`. Empty text puts back the default.
    pub fn set_plugin_setting(&self, id: &str, key: &str, text: &str) -> Result<(), String> {
        let value = {
            let p = self.inner.plugins.lock();
            let e = p
                .entries
                .iter()
                .find(|e| e.info.id == id)
                .ok_or_else(|| format!("No plugin \"{id}\"."))?;
            let m = e
                .info
                .manifest
                .as_ref()
                .ok_or_else(|| "The plugin doesn't load; fix it first.".to_owned())?;
            let s = m
                .settings
                .iter()
                .find(|s| s.key == key)
                .ok_or_else(|| format!("{} has no setting \"{key}\".", m.name))?;
            if text.trim().is_empty() {
                None
            } else {
                Some(s.parse(text)?)
            }
        };
        {
            let mut p = self.inner.plugins.lock();
            let saved = p.saved.plugins.entry(id.to_owned()).or_default();
            match value {
                Some(v) => saved.settings.insert(key.to_owned(), v),
                None => saved.settings.remove(key),
            };
        }
        self.inner.store.write().plugin_rev += 1;
        self.inner.repaint();
        self.inner.plugin_job(|inner| {
            inner.save_plugins();
            Ok(())
        });
        Ok(())
    }

    /// Each setting of a plugin with its current value.
    pub fn plugin_settings(&self, id: &str) -> Vec<(bonsai_plugin::Setting, serde_json::Value)> {
        let p = self.inner.plugins.lock();
        let Some(m) = p
            .entries
            .iter()
            .find(|e| e.info.id == id)
            .and_then(|e| e.info.manifest.as_ref())
        else {
            return Vec::new();
        };
        let mut values = setting_values(m, p.saved.plugins.get(id));
        m.settings
            .iter()
            .map(|s| {
                (
                    s.clone(),
                    values.remove(&s.key).unwrap_or(serde_json::Value::Null),
                )
            })
            .collect()
    }

    /// Restarts a plugin from its files.
    pub fn reload_plugin(&self, id: &str) -> Result<(), String> {
        if !self.inner.plugins.lock().entries.iter().any(|e| e.info.id == id) {
            return Err(format!("No plugin \"{id}\"."));
        }
        let id = id.to_owned();
        self.inner.plugin_job(move |inner| {
            inner.sync_plugins(Force::One(&id));
            Ok(())
        });
        Ok(())
    }

    /// Uninstalls a plugin: deletes its folder, or forgets an unpacked one without touching it.
    pub fn remove_plugin(&self, id: &str) -> Result<(), String> {
        let (dir, unpacked) = {
            let p = self.inner.plugins.lock();
            let e = p
                .entries
                .iter()
                .find(|e| e.info.id == id)
                .ok_or_else(|| format!("No plugin \"{id}\"."))?;
            (e.info.dir.clone(), e.info.unpacked)
        };
        let id = id.to_owned();
        self.inner.plugin_job(move |inner| {
            {
                let mut p = inner.plugins.lock();
                // Stop it before its files go.
                p.entries.retain(|e| e.info.id != id);
                p.saved.plugins.remove(&id);
                p.saved.unpacked.retain(|d| *d != dir);
            }
            inner.repaint();
            if !unpacked {
                remove_dir(&dir).map_err(|e| format!("Can't delete the plugin folder: {e}"))?;
            }
            inner
                .drop_plugin_data(&id)
                .map_err(|e| format!("Can't delete the plugin's data: {e}"))?;
            inner.save_plugins();
            inner.sync_plugins(Force::None);
            Ok(())
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manifest(perms: &str, actions: &str) -> Manifest {
        Manifest::parse(&format!(
            "id = \"x\"\nentry = \"main.luau\"\npermissions = [{perms}]\nactions = [{actions}]"
        ))
        .unwrap()
    }

    #[test]
    fn manifest_actions_are_tier_checked() {
        let r = Registry::default();
        assert!(action_refusal(&r, "user.unfriend").is_none());
        assert!(action_refusal(&r, "market.buy").is_none());
        assert!(action_refusal(&r, "account.remove").is_some());
        assert!(action_refusal(&r, "theme.apply").is_some());
        assert!(action_refusal(&r, "nav.home").is_some());
        assert!(action_refusal(&r, "not.a_command").is_some());
    }

    #[test]
    fn asks_more_lists_new_permissions_and_actions() {
        let m = manifest("\"read.public\", \"notify\"", "\"user.unfriend\"");
        assert_eq!(
            asks_more(&m, None),
            vec!["read.public", "notify", "user.unfriend"]
        );
        let agreed = Saved {
            permissions: vec!["read.public".into()],
            actions: vec!["user.unfriend".into()],
            ..Default::default()
        };
        assert_eq!(asks_more(&m, Some(&agreed)), vec!["notify"]);
    }

    #[test]
    fn copy_skips_dot_files_and_symlinks_and_caps_size() {
        let src = tempfile::tempdir().unwrap();
        let dst = tempfile::tempdir().unwrap();
        std::fs::write(src.path().join("main.luau"), "return 1").unwrap();
        std::fs::create_dir_all(src.path().join("lib")).unwrap();
        std::fs::write(src.path().join("lib/util.luau"), "return 2").unwrap();
        std::fs::create_dir_all(src.path().join(".git")).unwrap();
        std::fs::write(src.path().join(".git/HEAD"), "x").unwrap();
        #[cfg(unix)]
        std::os::unix::fs::symlink("/etc/passwd", src.path().join("passwd")).unwrap();
        let to = dst.path().join("x");
        copy_plugin(src.path(), &to).unwrap();
        assert!(to.join("main.luau").is_file());
        assert!(to.join("lib/util.luau").is_file());
        assert!(!to.join(".git").exists());
        assert!(!to.join("passwd").exists());

        for i in 0..=MAX_FILES {
            std::fs::write(src.path().join(format!("f{i}.luau")), "").unwrap();
        }
        assert!(copy_plugin(src.path(), &dst.path().join("y")).is_err());
    }
}

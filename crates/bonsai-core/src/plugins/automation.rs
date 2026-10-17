//! Plugin writes without a click: `ctx.run`, actions that run unattended, the per-account
//! automation switch, hourly caps, the activity log and the kill switch. See `docs/plugins.md`.

use std::collections::VecDeque;
use std::io::Write as _;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::AccountId;
use crate::commands::{CommandId, Registry, Tier};
use crate::core::{Core, Inner};

use super::{Force, ProposedAction, Status};

/// Most times a plugin may run one tier 2 action unattended on one account in an hour. The
/// user may lower it; a plugin can't raise it.
pub const DEFAULT_CAP: u32 = 20;
const HOUR: i64 = 3600;
/// Entries kept beyond the last hour, which caps count from.
const KEEP_OLD: usize = 500;
/// Most entries kept, however recent.
const KEEP_MAX: usize = 5000;
/// The log file is rewritten from memory once it grows past this.
const FILE_MAX: u64 = 2 * 1024 * 1024;
/// A repeat of the same refusal within this many seconds isn't logged again.
const QUIET: i64 = 60;

/// One plugin write, or one it tried and was refused.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Activity {
    /// Unix seconds.
    pub at: i64,
    pub plugin: String,
    pub plugin_name: String,
    pub account: AccountId,
    /// The built-in command's key.
    pub command: String,
    #[serde(default)]
    pub arg: Option<String>,
    /// Run without a click: `ctx.run`, or actions an event, timer or automation ran.
    #[serde(default)]
    pub unattended: bool,
    /// Why it failed or was refused; `None` when it ran.
    #[serde(default)]
    pub error: Option<String>,
}

/// The activity log in memory, oldest first.
#[derive(Default)]
pub(super) struct Log {
    entries: VecDeque<Activity>,
    /// Bumped when the stop button is pressed; a batch started before it stops.
    pub(super) stop_gen: u64,
}

impl Log {
    /// Unattended runs of `command` that went through in the last hour.
    fn used(&self, plugin: &str, account: AccountId, command: &str, now: i64) -> u32 {
        self.entries
            .iter()
            .rev()
            .take_while(|a| a.at > now - HOUR)
            .filter(|a| {
                a.unattended
                    && a.error.is_none()
                    && a.plugin == plugin
                    && a.account == account
                    && a.command == command
            })
            .count() as u32
    }

    /// Adds `a` unless it repeats a recent refusal. Returns whether it was added.
    fn push(&mut self, a: Activity) -> bool {
        if a.error.is_some()
            && self
                .entries
                .iter()
                .rev()
                .take_while(|b| b.at > a.at - QUIET)
                .any(|b| {
                    b.plugin == a.plugin
                        && b.account == a.account
                        && b.command == a.command
                        && b.error == a.error
                })
        {
            return false;
        }
        self.entries.push_back(a);
        self.trim();
        true
    }

    fn trim(&mut self) {
        let now = self.entries.back().map_or(0, |a| a.at);
        let recent = self
            .entries
            .iter()
            .rev()
            .take_while(|a| a.at > now - HOUR)
            .count();
        let keep = (recent + KEEP_OLD).min(KEEP_MAX);
        while self.entries.len() > keep {
            self.entries.pop_front();
        }
    }
}

/// Why `tier` can't run without a click, or `None` when it can. `used` counts runs in the last
/// hour against `cap`.
pub fn unattended_refusal(tier: Tier, automation: bool, used: u32, cap: u32) -> Option<String> {
    match tier {
        Tier::Harmless => None,
        Tier::Social if !automation => {
            Some("Automation is off for this account. Turn it on in Settings > Plugins.".into())
        }
        Tier::Social if used >= cap => Some(format!("Reached its cap of {cap} an hour.")),
        Tier::Social => None,
        Tier::Value => Some("Robux and items need you to confirm each time.".into()),
        Tier::Account => Some("Account control is never available to plugins.".into()),
    }
}

/// Serializes appends to the log file.
static FILE: parking_lot::Mutex<()> = parking_lot::Mutex::new(());

impl Inner {
    fn automation_on(&self, account: AccountId) -> bool {
        self.plugins.lock().saved.automation.contains(&account)
    }

    /// The hourly cap of `command` for `plugin`.
    fn cap(&self, plugin: &str, command: &str) -> u32 {
        let p = self.plugins.lock();
        p.saved
            .plugins
            .get(plugin)
            .and_then(|s| s.caps.get(command))
            .map_or(DEFAULT_CAP, |c| (*c).min(DEFAULT_CAP))
    }

    /// Why `a` can't run unattended now, counting `extra` runs not logged yet.
    pub(super) fn may_run_unattended(
        &self,
        plugin: &str,
        account: AccountId,
        a: &ProposedAction,
        extra: u32,
    ) -> Result<(), String> {
        let key = a.id.key();
        let used = self
            .plugins
            .lock()
            .log
            .used(plugin, account, key, crate::cache::now());
        let cap = self.cap(plugin, key);
        match unattended_refusal(a.tier, self.automation_on(account), used + extra, cap) {
            Some(why) => Err(why),
            None => Ok(()),
        }
    }

    /// Records a plugin write. A write that went through lights the stop indicator.
    pub(super) fn log_activity(&self, a: Activity) {
        let wrote = a.error.is_none();
        let at = a.at;
        let line = serde_json::to_string(&a).ok();
        if !self.plugins.lock().log.push(a) {
            return;
        }
        if wrote {
            self.store.write().plugin_wrote = Some(at);
        }
        // Every screen refetches on a repaint, the activity list included.
        self.repaint();
        if let Some(line) = line {
            self.append_activity(&line);
        }
    }

    /// Appends one line to the log file, or rewrites it from memory once it is large.
    fn append_activity(&self, line: &str) {
        let _f = FILE.lock();
        let path = self.paths.plugin_activity_file();
        let large = std::fs::metadata(&path).is_ok_and(|m| m.len() > FILE_MAX);
        let r = if large {
            let mut text = String::new();
            for a in &self.plugins.lock().log.entries {
                if let Ok(l) = serde_json::to_string(a) {
                    text.push_str(&l);
                    text.push('\n');
                }
            }
            std::fs::write(&path, text)
        } else {
            path.parent()
                .map_or(Ok(()), std::fs::create_dir_all)
                .and_then(|()| std::fs::OpenOptions::new().create(true).append(true).open(&path))
                .and_then(|mut f| writeln!(f, "{line}"))
        };
        if let Err(e) = r {
            tracing::warn!("writing plugin activity: {e}");
        }
    }

    /// `ctx.run(id, arg)`: checks the grants, tier, automation switch and cap, then runs the
    /// action now as `account`. On the plugin's thread. Never falls back to asking.
    pub(super) fn plugin_run(
        self: &Arc<Self>,
        plugin: &str,
        account: u64,
        key: &str,
        arg: Option<&str>,
    ) -> Result<(), String> {
        if account == 0 {
            return Err("ctx.run needs an account the plugin may use.".into());
        }
        let account = AccountId(account);
        let name = self
            .plugins
            .lock()
            .entries
            .iter()
            .find(|e| e.info.id == plugin)
            .map(|e| {
                e.info
                    .manifest
                    .as_ref()
                    .map_or_else(|| plugin.to_owned(), |m| m.name.clone())
            });
        let Some(name) = name else {
            return Err("This plugin isn't installed.".into());
        };
        let epoch = self.plugins.lock().log.stop_gen;
        let list = [(key.to_owned(), arg.map(str::to_owned))];
        let checked = self.check_actions(plugin, account, &list).and_then(|mut a| {
            let a = a.pop().ok_or_else(|| "Nothing to run.".to_owned())?;
            self.may_run_unattended(plugin, account, &a, 0)?;
            Ok(a)
        });
        let result = checked
            .clone()
            .and_then(|a| self.run_one(plugin, account, &a, true, epoch));
        let (arg, command) = match &checked {
            Ok(a) => (a.arg.clone(), a.id.key().to_owned()),
            Err(_) => (
                arg.map(|a| a.chars().take(200).collect()),
                key.chars().take(100).collect(),
            ),
        };
        self.log_activity(Activity {
            at: crate::cache::now(),
            plugin: plugin.to_owned(),
            plugin_name: name,
            account,
            command,
            arg,
            unattended: true,
            error: result.as_ref().err().cloned(),
        });
        result
    }

    /// Runs one checked action unless the stop button was pressed since `epoch`, the plugin
    /// stopped, or (unattended) automation went off. Doesn't log.
    pub(super) fn run_one(
        &self,
        plugin: &str,
        account: AccountId,
        a: &ProposedAction,
        unattended: bool,
        epoch: u64,
    ) -> Result<(), String> {
        let runner = {
            let p = self.plugins.lock();
            if p.log.stop_gen != epoch {
                return Err("Stopped.".into());
            }
            let running = p
                .entries
                .iter()
                .any(|e| e.info.id == plugin && matches!(e.info.status, Status::Running { .. }));
            if !running {
                return Err("It isn't running.".into());
            }
            if unattended && a.tier >= Tier::Social && !p.saved.automation.contains(&account) {
                return Err("Automation is off for this account.".into());
            }
            p.runner.clone().ok_or("Actions can't run here.")?
        };
        runner(account, a.id, a.arg.clone())
    }
}

/// Reads the end of the log file so caps survive a restart.
pub(super) fn load_log(inner: &Inner) {
    let text = std::fs::read_to_string(inner.paths.plugin_activity_file()).unwrap_or_default();
    let mut p = inner.plugins.lock();
    for line in text.lines() {
        if let Ok(a) = serde_json::from_str::<Activity>(line) {
            p.log.entries.push_back(a);
        }
    }
    p.log.trim();
}

/// One tier 2 action's cap for one plugin.
#[derive(Clone, Debug)]
pub struct PluginCap {
    pub command: CommandId,
    pub value: u32,
}

impl Core {
    /// Plugin writes, newest first.
    pub fn plugin_activity(&self, limit: usize) -> Vec<Activity> {
        let p = self.inner.plugins.lock();
        p.log.entries.iter().rev().take(limit).cloned().collect()
    }

    /// Accounts plugins may automate.
    pub fn plugin_automation(&self) -> Vec<AccountId> {
        self.inner.plugins.lock().saved.automation.clone()
    }

    /// Opens the dialog that turns automation on for `account`.
    pub fn request_automation(&self, account: AccountId) -> Result<(), String> {
        if !self.store().accounts.accounts.iter().any(|a| a.id == account) {
            return Err("Pick one of your accounts.".into());
        }
        if self.inner.automation_on(account) {
            return Err("Automation is already on for this account.".into());
        }
        self.inner.store.write().plugin_automation = Some(account);
        self.inner.repaint();
        Ok(())
    }

    /// Turns automation on for the account in the dialog when `typed` is its name.
    pub fn confirm_automation(&self, typed: &str) -> Result<(), String> {
        let Some(account) = self.store().plugin_automation else {
            return Err("Nothing to confirm.".into());
        };
        let name = self
            .store()
            .accounts
            .accounts
            .iter()
            .find(|a| a.id == account)
            .map(|a| a.name.clone())
            .ok_or("That account was removed.")?;
        if !typed.trim().eq_ignore_ascii_case(&name) {
            return Err(format!("Type {name} to confirm."));
        }
        self.inner.store.write().plugin_automation = None;
        {
            let mut p = self.inner.plugins.lock();
            if !p.saved.automation.contains(&account) {
                p.saved.automation.push(account);
            }
        }
        self.saved_plugins_changed();
        Ok(())
    }

    pub fn cancel_automation(&self) {
        self.inner.store.write().plugin_automation = None;
        self.inner.repaint();
    }

    /// Turns automation off for `account`. Takes effect for the next write, including ones a
    /// batch has queued.
    pub fn block_automation(&self, account: AccountId) {
        self.inner
            .plugins
            .lock()
            .saved
            .automation
            .retain(|a| *a != account);
        self.saved_plugins_changed();
    }

    /// Each tier 2 action a plugin may run, with its hourly cap.
    pub fn plugin_caps(&self, id: &str) -> Vec<PluginCap> {
        let registry = Registry::default();
        let p = self.inner.plugins.lock();
        let Some(saved) = p.saved.plugins.get(id) else {
            return Vec::new();
        };
        saved
            .actions
            .iter()
            .filter_map(|k| CommandId::from_key(k))
            .filter(|c| registry.get(*c).effect.tier() == Some(Tier::Social))
            .map(|command| PluginCap {
                command,
                value: saved
                    .caps
                    .get(command.key())
                    .map_or(DEFAULT_CAP, |c| (*c).min(DEFAULT_CAP)),
            })
            .collect()
    }

    /// Lowers (or puts back) a plugin's hourly cap for one tier 2 action. It can't go above
    /// [`DEFAULT_CAP`].
    pub fn set_plugin_cap(&self, id: &str, command: &str, value: u32) -> Result<(), String> {
        let cmd = CommandId::from_key(command).ok_or_else(|| format!("No command \"{command}\"."))?;
        {
            let mut p = self.inner.plugins.lock();
            let saved = p
                .saved
                .plugins
                .get_mut(id)
                .ok_or_else(|| format!("No plugin \"{id}\"."))?;
            if Registry::default().get(cmd).effect.tier() != Some(Tier::Social)
                || !saved.actions.iter().any(|a| a == command)
            {
                return Err(format!("{command} has no cap for this plugin."));
            }
            if value >= DEFAULT_CAP {
                saved.caps.remove(command);
            } else {
                saved.caps.insert(command.to_owned(), value);
            }
        }
        self.saved_plugins_changed();
        Ok(())
    }

    /// The stop button: turns every plugin off, drops writes they queued and closes the action
    /// dialog. Only short locks, so it is safe on the main thread.
    pub fn stop_plugins(&self) {
        {
            let mut p = self.inner.plugins.lock();
            p.log.stop_gen += 1;
            for s in p.saved.plugins.values_mut() {
                s.enabled = false;
            }
            for e in &mut p.entries {
                e.info.enabled = false;
                // Dropping the worker drops its queued calls and ends its thread.
                e.worker = None;
                if !matches!(e.info.status, Status::NeedsReview(_) | Status::Failed(_)) {
                    e.info.status = Status::Off;
                }
            }
        }
        {
            let mut s = self.inner.store.write();
            s.plugin_actions = None;
            s.plugin_wrote = None;
        }
        self.inner.toast(None, "Stopped all plugins.", false);
        self.saved_plugins_changed();
    }

    /// Repaints and saves `plugins.json` after a change to what was agreed.
    fn saved_plugins_changed(&self) {
        self.inner.store.write().plugin_rev += 1;
        self.inner.repaint();
        self.inner.plugin_job(|inner| {
            inner.save_plugins();
            inner.sync_plugins(Force::None);
            Ok(())
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(at: i64, command: &str, unattended: bool, error: Option<&str>) -> Activity {
        Activity {
            at,
            plugin: "p".into(),
            plugin_name: "P".into(),
            account: AccountId(1),
            command: command.into(),
            arg: None,
            unattended,
            error: error.map(Into::into),
        }
    }

    #[test]
    fn tiers_without_a_click() {
        assert_eq!(unattended_refusal(Tier::Harmless, false, 99, 0), None);
        assert!(
            unattended_refusal(Tier::Social, false, 0, 20)
                .unwrap()
                .contains("Automation is off")
        );
        assert_eq!(unattended_refusal(Tier::Social, true, 19, 20), None);
        assert!(
            unattended_refusal(Tier::Social, true, 20, 20)
                .unwrap()
                .contains("cap of 20")
        );
        assert!(unattended_refusal(Tier::Value, true, 0, 20).is_some());
        assert!(
            unattended_refusal(Tier::Account, true, 0, 20)
                .unwrap()
                .contains("never")
        );
    }

    #[test]
    fn caps_count_unattended_runs_in_the_last_hour() {
        let mut log = Log::default();
        let now = 10_000;
        log.push(entry(now - HOUR - 1, "user.add_friend", true, None));
        log.push(entry(now - 10, "user.add_friend", true, None));
        log.push(entry(now - 9, "user.add_friend", false, None));
        log.push(entry(now - 8, "user.add_friend", true, Some("Failed.")));
        log.push(entry(now - 7, "user.unfriend", true, None));
        assert_eq!(log.used("p", AccountId(1), "user.add_friend", now), 1);
        assert_eq!(log.used("p", AccountId(2), "user.add_friend", now), 0);
        assert_eq!(log.used("q", AccountId(1), "user.add_friend", now), 0);
    }

    #[test]
    fn repeated_refusals_are_logged_once_a_minute() {
        let mut log = Log::default();
        assert!(log.push(entry(100, "user.unfriend", true, Some("Off."))));
        assert!(!log.push(entry(130, "user.unfriend", true, Some("Off."))));
        assert!(log.push(entry(130, "user.unfriend", true, Some("Other."))));
        assert!(log.push(entry(200, "user.unfriend", true, Some("Off."))));
        // Runs that went through are always logged.
        assert!(log.push(entry(200, "user.unfriend", true, None)));
        assert!(log.push(entry(200, "user.unfriend", true, None)));
    }

    #[test]
    fn the_log_keeps_the_last_hour_and_some_older() {
        let mut log = Log::default();
        for i in 0..KEEP_OLD + 100 {
            log.push(entry(i as i64, "x", false, None));
        }
        // All within an hour of the last.
        assert_eq!(log.entries.len(), KEEP_OLD + 100);
        log.push(entry(100_000, "x", false, None));
        assert_eq!(log.entries.len(), KEEP_OLD + 1);
    }
}

//! Events and timers for plugins. Watchers call [`Inner::plugin_event`]; each running plugin
//! that listens, was allowed `read.account` and may use the account gets it on its own thread.
//! A plugin that is busy misses the event rather than holding up the watcher.

use serde_json::Value;

use crate::AccountId;
use crate::core::Inner;

use super::{Msg, Status};

impl Inner {
    /// Some running plugin listens to `event`, so a watcher should look for it.
    pub(crate) fn plugins_listen(&self, event: &str) -> bool {
        self.plugins.lock().entries.iter().any(
            |e| matches!(&e.info.status, Status::Running { events, .. } if events.iter().any(|x| x == event)),
        )
    }

    /// Sends `event` to every plugin that may see it for `account`.
    pub(crate) fn plugin_event(&self, account: AccountId, event: &'static str, payload: Value) {
        let p = self.plugins.lock();
        for e in &p.entries {
            let (Status::Running { events, .. }, Some(w)) = (&e.info.status, &e.worker) else {
                continue;
            };
            if !events.iter().any(|x| x == event) {
                continue;
            }
            let Some(saved) = p.saved.plugins.get(&e.info.id) else {
                continue;
            };
            if !saved.accounts.contains(&account) || !saved.permissions.iter().any(|x| x == "read.account") {
                continue;
            }
            let msg = Msg::Event {
                event,
                ctx: bonsai_plugin::Ctx {
                    account: account.0,
                    now: crate::cache::now(),
                },
                payload: payload.clone(),
            };
            if let Err(std::sync::mpsc::TrySendError::Full(_)) = w.tx.try_send(msg) {
                tracing::warn!("plugin {} is busy; dropped a {event} event", e.info.id);
            }
        }
    }

    /// The accounts a timer runs as: each granted account that is signed in, or 0 when there
    /// are none. `None` when worker `seq` is no longer current.
    pub(super) fn timer_accounts(&self, id: &str, seq: u64) -> Option<Vec<AccountId>> {
        let known: Vec<AccountId> = self.store.read().accounts.accounts.iter().map(|a| a.id).collect();
        let p = self.plugins.lock();
        let e = p.entries.iter().find(|e| e.info.id == id)?;
        e.worker.as_ref().filter(|w| w.seq == seq)?;
        let mut out: Vec<AccountId> = p
            .saved
            .plugins
            .get(id)
            .map(|s| s.accounts.iter().copied().filter(|a| known.contains(a)).collect())
            .unwrap_or_default();
        if out.is_empty() {
            out.push(AccountId(0));
        }
        Some(out)
    }

    /// The plugin was allowed `notify` and may use `account`.
    pub(super) fn may_notify(&self, id: &str, account: AccountId) -> bool {
        self.plugins
            .lock()
            .saved
            .plugins
            .get(id)
            .is_some_and(|s| s.accounts.contains(&account) && s.permissions.iter().any(|p| p == "notify"))
    }
}

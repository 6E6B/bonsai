//! Which user commands still have work in flight, so the button that started one can show a
//! spinner until Roblox answers.
//!
//! Commands return before their requests finish: [`Inner::mutate`] and [`Core::send`] hand the
//! work to a task. The host wraps each command it runs in a [`Scope`]; anything that starts
//! work inside it takes a [`Busy`] guard that lives as long as the task does. The store keeps a
//! count per command, account and argument while any guard for it is alive.

use std::cell::RefCell;
use std::sync::Arc;

use bonsai_api::AccountId;

use crate::core::{Core, Inner};

/// A command as the user ran it: who it acts as, its id and its argument.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RunKey {
    pub account: Option<AccountId>,
    pub command: String,
    pub arg: Option<String>,
}

thread_local! {
    static CURRENT: RefCell<Option<RunKey>> = const { RefCell::new(None) };
}

/// While alive, work started on this thread counts toward `key`. Restores the previous key
/// when dropped, so scopes nest.
pub struct Scope {
    prev: Option<RunKey>,
}

impl Drop for Scope {
    fn drop(&mut self) {
        let prev = self.prev.take();
        CURRENT.with(|c| *c.borrow_mut() = prev);
    }
}

impl Core {
    /// Marks work started on this thread until the scope ends as `key`'s. The host calls this
    /// around each command a user runs.
    pub fn running_scope(&self, key: RunKey) -> Scope {
        Scope {
            prev: CURRENT.with(|c| c.borrow_mut().replace(key)),
        }
    }
}

/// Counts as running for its key until dropped.
pub(crate) struct Busy {
    inner: Arc<Inner>,
    key: RunKey,
}

impl Busy {
    /// Counts work started on this thread as this guard's command until the scope ends. A
    /// task that waits first and then starts more work, like resolving a username, enters it
    /// around that synchronous call. Never hold the scope across an `.await`.
    pub(crate) fn enter(&self) -> Scope {
        Scope {
            prev: CURRENT.with(|c| c.borrow_mut().replace(self.key.clone())),
        }
    }
}

impl Drop for Busy {
    fn drop(&mut self) {
        {
            let mut s = self.inner.store.write();
            if let Some(n) = s.running.get_mut(&self.key) {
                *n = n.saturating_sub(1);
                if *n == 0 {
                    s.running.remove(&self.key);
                }
            }
        }
        self.inner.repaint();
    }
}

impl Inner {
    /// A guard for the command running on this thread, if any. Move it into the task doing
    /// the command's work.
    pub(crate) fn busy(self: &Arc<Self>) -> Option<Busy> {
        let key = CURRENT.with(|c| c.borrow().clone())?;
        *self.store.write().running.entry(key.clone()).or_default() += 1;
        self.repaint();
        Some(Busy {
            inner: self.clone(),
            key,
        })
    }
}

//! Cached resources: every screen renders from cache first, then revalidates in the background.
//!
//! A screen asks for a resource every frame (`core.res(..)` via an area function). The first
//! call starts a task that loads the saved copy from SQLite, shows it, and then fetches a fresh
//! one if the saved copy is older than the resource's max age. Calls never block.
//!
//! Keys carry an [`AccountId`]: private data is keyed by the account that sees it, shared public
//! data by `AccountId::ANONYMOUS`.

use std::any::Any;
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;

use parking_lot::RwLock;
use rbx_api::{AccountId, Client, Error};
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::cache::{Kind, now};
use crate::core::Inner;

/// Errors are retried at most this often.
const ERROR_BACKOFF_SECS: i64 = 30;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Key {
    pub account: AccountId,
    pub kind: &'static str,
    pub id: String,
}

impl Key {
    /// Data only `account` may see (inventory, trades, messages, ...).
    pub fn private(account: AccountId, kind: &'static str, id: impl ToString) -> Key {
        Key {
            account,
            kind,
            id: id.to_string(),
        }
    }

    /// Public data shared by every account (users, games, items, groups).
    pub fn shared(kind: &'static str, id: impl ToString) -> Key {
        Key::private(AccountId::ANONYMOUS, kind, id)
    }

    fn cache_id(&self) -> String {
        format!("{}:{}", self.kind, self.id)
    }
}

/// A snapshot of a resource for one frame.
#[derive(Debug)]
pub struct Res<T> {
    pub data: Option<Arc<T>>,
    pub loading: bool,
    pub error: Option<String>,
    pub fetched_at: i64,
}

impl<T> Clone for Res<T> {
    fn clone(&self) -> Self {
        Res {
            data: self.data.clone(),
            loading: self.loading,
            error: self.error.clone(),
            fetched_at: self.fetched_at,
        }
    }
}

impl<T> Default for Res<T> {
    fn default() -> Self {
        Res {
            data: None,
            loading: false,
            error: None,
            fetched_at: 0,
        }
    }
}

impl<T> Res<T> {
    pub fn age_secs(&self) -> i64 {
        now() - self.fetched_at
    }
    /// Nothing to show yet and still working on it.
    pub fn first_load(&self) -> bool {
        self.data.is_none() && (self.loading || self.error.is_none())
    }
}

/// One page-accumulating list (cursor pagination).
#[derive(Clone, Debug, Default, PartialEq, Serialize, serde::Deserialize)]
pub struct Page<T> {
    pub items: Vec<T>,
    /// Cursor for the next page; `None` when there is none.
    pub next: Option<String>,
}

struct Slot {
    data: Option<Arc<dyn Any + Send + Sync>>,
    loading: bool,
    error: Option<String>,
    error_at: i64,
    fetched_at: i64,
    stale: bool,
}

#[derive(Default)]
pub(crate) struct Resources {
    slots: RwLock<HashMap<Key, Slot>>,
}

impl Resources {
    pub(crate) fn snapshot<T: Send + Sync + 'static>(&self, key: &Key) -> Option<Res<T>> {
        let s = self.slots.read();
        let slot = s.get(key)?;
        Some(Res {
            data: slot.data.clone().and_then(|d| d.downcast::<T>().ok()),
            loading: slot.loading,
            error: slot.error.clone(),
            fetched_at: slot.fetched_at,
        })
    }

    /// Mark matching resources stale so the next frame refetches them.
    pub(crate) fn invalidate(&self, account: AccountId, kind: &str) {
        for (k, s) in self.slots.write().iter_mut() {
            if k.account == account && k.kind == kind {
                s.stale = true;
            }
        }
    }

    /// Change a cached value in place (optimistic updates after a write).
    pub(crate) fn update<T: Clone + Send + Sync + 'static>(&self, key: &Key, f: impl FnOnce(&mut T)) {
        let mut s = self.slots.write();
        let Some(slot) = s.get_mut(key) else { return };
        let Some(cur) = slot.data.clone().and_then(|d| d.downcast::<T>().ok()) else {
            return;
        };
        let mut v = (*cur).clone();
        f(&mut v);
        slot.data = Some(Arc::new(v));
    }

    /// Change every cached value of one kind for `account`, like each page's follow states.
    pub(crate) fn update_kind<T: Clone + Send + Sync + 'static>(
        &self,
        account: AccountId,
        kind: &str,
        f: impl Fn(&mut T),
    ) {
        for (k, slot) in self.slots.write().iter_mut() {
            if k.account != account || k.kind != kind {
                continue;
            }
            let Some(cur) = slot.data.clone().and_then(|d| d.downcast::<T>().ok()) else {
                continue;
            };
            let mut v = (*cur).clone();
            f(&mut v);
            slot.data = Some(Arc::new(v));
        }
    }

    /// Forget every loaded resource; screens load them again.
    pub(crate) fn clear(&self) {
        self.slots.write().clear();
    }

    pub(crate) fn forget_account(&self, account: AccountId) {
        self.slots.write().retain(|k, _| k.account != account);
    }
}

/// What a fetch gets to work with.
pub(crate) struct Fetch {
    pub client: Client,
    pub inner: Arc<Inner>,
}

impl Fetch {
    /// Remember user names seen in a response so lists can show them without another call.
    pub fn note_users(&self, users: impl IntoIterator<Item = crate::store::UserInfo>) {
        let mut s = self.inner.store.write();
        for u in users {
            // Keep what we know, but let a later response fill in a missing @name.
            match s.users.get(&u.id) {
                Some(have) if !have.name.is_empty() || u.name.is_empty() => {}
                _ => {
                    s.users.insert(u.id, u);
                }
            }
        }
    }
}

const NOT_SIGNED_IN: &str = "This account is not signed in.";

impl Inner {
    /// The current snapshot of `key`; starts a load when missing or older than `max_age` secs.
    pub(crate) fn res<T, F, Fut>(self: &Arc<Self>, key: Key, max_age: i64, fetch: F) -> Res<T>
    where
        T: Serialize + DeserializeOwned + Send + Sync + 'static,
        F: FnOnce(Fetch) -> Fut + Send + 'static,
        Fut: Future<Output = Result<T, Error>> + Send + 'static,
    {
        let t = now();
        {
            let s = self.res.slots.read();
            if let Some(slot) = s.get(&key) {
                let fresh = slot.data.is_some() && !slot.stale && t - slot.fetched_at <= max_age;
                let backing_off =
                    slot.error.is_some() && t - slot.error_at < ERROR_BACKOFF_SECS && !slot.stale;
                if slot.loading || fresh || backing_off {
                    drop(s);
                    return self.res.snapshot(&key).unwrap_or_default();
                }
            }
        }
        let first = {
            let mut s = self.res.slots.write();
            let slot = s.entry(key.clone()).or_insert(Slot {
                data: None,
                loading: false,
                error: None,
                error_at: 0,
                fetched_at: 0,
                stale: false,
            });
            if slot.loading {
                return Res {
                    data: slot.data.clone().and_then(|d| d.downcast::<T>().ok()),
                    loading: true,
                    error: slot.error.clone(),
                    fetched_at: slot.fetched_at,
                };
            }
            slot.loading = true;
            slot.data.is_none()
        };
        let inner = self.clone();
        let k = key.clone();
        self.rt.spawn(async move {
            // Saved copy first.
            if first && let Some(c) = inner.cache.get::<T>(Kind::Res, k.account, &k.cache_id()) {
                let fresh = t - c.fetched_at <= max_age;
                {
                    let mut s = inner.res.slots.write();
                    if let Some(slot) = s.get_mut(&k) {
                        slot.data = Some(Arc::new(c.value));
                        slot.fetched_at = c.fetched_at;
                        if fresh {
                            slot.loading = false;
                        }
                    }
                }
                inner.repaint();
                if fresh {
                    return;
                }
            }
            match inner.client(k.account) {
                Some(client) => {
                    let r = fetch(Fetch {
                        client,
                        inner: inner.clone(),
                    })
                    .await;
                    inner.finish(&k, r);
                }
                None => inner.fail_msg(&k, NOT_SIGNED_IN.into()),
            }
        });
        self.res.snapshot(&key).unwrap_or_default()
    }

    fn finish<T: Serialize + Send + Sync + 'static>(&self, k: &Key, r: Result<T, Error>) {
        match r {
            Ok(v) => {
                self.cache.put(Kind::Res, k.account, &k.cache_id(), &v);
                let mut s = self.res.slots.write();
                if let Some(slot) = s.get_mut(k) {
                    slot.data = Some(Arc::new(v));
                    slot.fetched_at = now();
                    slot.loading = false;
                    slot.error = None;
                    slot.stale = false;
                }
            }
            Err(e) => {
                self.note_error(k.account, &e);
                // A 401 here does not by itself mean the session expired; `note_error` checks
                // that and the account rail says so if it did.
                let msg = match e {
                    Error::Unauthorized if k.account.is_anonymous() => "Sign in to see this.".into(),
                    Error::Unauthorized => "Roblox did not allow this request.".into(),
                    e => e.user_message(),
                };
                self.fail_msg(k, msg);
                return;
            }
        }
        self.repaint();
    }

    fn fail_msg(&self, k: &Key, msg: String) {
        {
            let mut s = self.res.slots.write();
            if let Some(slot) = s.get_mut(k) {
                slot.loading = false;
                slot.error = Some(msg);
                slot.error_at = now();
                slot.stale = false;
            }
        }
        self.repaint();
    }

    /// A cursor-paged list. `more` loads the next page onto the end (if any and not loading).
    pub(crate) fn paged<T, F, Fut>(
        self: &Arc<Self>,
        key: Key,
        max_age: i64,
        more: bool,
        fetch: F,
    ) -> Res<Page<T>>
    where
        T: Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
        F: Fn(Fetch, Option<String>) -> Fut + Send + Sync + Clone + 'static,
        Fut: Future<Output = Result<Page<T>, Error>> + Send + 'static,
    {
        let f = fetch.clone();
        let cur = self.res(key.clone(), max_age, move |x| f(x, None));
        if !more || cur.loading {
            return cur;
        }
        let Some(next) = cur.data.as_ref().and_then(|p| p.next.clone()) else {
            return cur;
        };
        {
            let mut s = self.res.slots.write();
            let Some(slot) = s.get_mut(&key) else { return cur };
            if slot.loading {
                return cur;
            }
            slot.loading = true;
        }
        let inner = self.clone();
        let k = key.clone();
        self.rt.spawn(async move {
            let Some(client) = inner.client(k.account) else {
                inner.fail_msg(&k, NOT_SIGNED_IN.into());
                return;
            };
            let r = fetch(
                Fetch {
                    client,
                    inner: inner.clone(),
                },
                Some(next),
            )
            .await;
            match r {
                Ok(page) => {
                    let merged = {
                        let s = inner.res.slots.read();
                        let old = s
                            .get(&k)
                            .and_then(|x| x.data.clone())
                            .and_then(|d| d.downcast::<Page<T>>().ok());
                        let mut items = old.map(|o| o.items.clone()).unwrap_or_default();
                        items.extend(page.items);
                        Page {
                            items,
                            next: page.next,
                        }
                    };
                    let fetched_at = inner.res.slots.read().get(&k).map(|s| s.fetched_at);
                    inner.finish(&k, Ok(merged));
                    // Keep the first page's age: loading more is not a refresh.
                    if let Some(t) = fetched_at
                        && let Some(slot) = inner.res.slots.write().get_mut(&k)
                    {
                        slot.fetched_at = t;
                    }
                }
                Err(e) => inner.finish::<Page<T>>(&k, Err(e)),
            }
        });
        self.res.snapshot(&key).unwrap_or_default()
    }

    /// Run a user-initiated write as `account`. Shows `done` on success (if not empty) or the
    /// error, then marks `invalidate` kinds stale so screens refetch.
    pub(crate) fn mutate<F, Fut>(
        self: &Arc<Self>,
        account: AccountId,
        what: &str,
        done: &str,
        invalidate: &[&'static str],
        f: F,
    ) where
        F: FnOnce(Fetch) -> Fut + Send + 'static,
        Fut: Future<Output = Result<(), Error>> + Send + 'static,
    {
        let inner = self.clone();
        let what = what.to_string();
        let done = done.to_string();
        let invalidate: Vec<&'static str> = invalidate.to_vec();
        self.store.write().jobs += 1;
        self.rt.spawn(async move {
            let r = match inner.client(account).filter(|c| c.signed_in()) {
                Some(client) => {
                    f(Fetch {
                        client,
                        inner: inner.clone(),
                    })
                    .await
                }
                None => Err(Error::Message(NOT_SIGNED_IN.into())),
            };
            match r {
                Ok(()) => {
                    if !done.is_empty() {
                        inner.toast(Some(account), done, false);
                    }
                }
                Err(e) => {
                    inner.note_error(account, &e);
                    let msg = match &e {
                        Error::Challenge(_) => {
                            "Roblox wants extra verification. Do this once on the website, then try again."
                                .to_string()
                        }
                        e => e.user_message(),
                    };
                    inner.toast(Some(account), format!("Could not {what}: {msg}"), true);
                }
            }
            for k in invalidate {
                inner.res.invalidate(account, k);
                inner.res.invalidate(AccountId::ANONYMOUS, k);
            }
            {
                let mut s = inner.store.write();
                s.jobs = s.jobs.saturating_sub(1);
            }
            inner.repaint();
        });
    }
}

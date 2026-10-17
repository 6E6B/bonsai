//! The app's own notifications: things bonsai noticed that Roblox's stream doesn't say, such as
//! a friend who removed you or a new sale. They are kept per account in the local cache and
//! listed with Roblox's notifications. Nothing in here sends a request.

use std::sync::Arc;

use bonsai_api::AccountId;

use crate::Core;
use crate::cache::{Kind, now};
use crate::core::Inner;
use crate::fmt::{iso_utc, parse_iso};
use crate::messages::StreamNotification;
use crate::res::{Page, Res};

pub(crate) const FEED: &str = "feed";

/// Entries kept per account; older ones are dropped.
const KEEP: usize = 200;

/// Ids of app entries start with this, so they never clash with Roblox's.
pub const PREFIX: &str = "app:";

/// Roblox's stream and the app's entries, newest first. While older stream pages are still to
/// load, app entries older than the last loaded one wait, so loading more doesn't put Roblox
/// entries above them.
pub fn merge(
    stream: &[StreamNotification],
    more: bool,
    local: &[StreamNotification],
) -> Vec<StreamNotification> {
    let at = |n: &StreamNotification| parse_iso(&n.when).unwrap_or(0);
    let cutoff = if more { stream.iter().map(at).min() } else { None };
    let mut out: Vec<StreamNotification> = stream
        .iter()
        .chain(local.iter().filter(|n| cutoff.is_none_or(|c| at(n) >= c)))
        .cloned()
        .collect();
    // Stable: equal times keep Roblox's order, then the app's.
    out.sort_by_key(|n| std::cmp::Reverse(at(n)));
    out
}

/// Unread app entries, for the badge.
pub fn unread(local: &[StreamNotification]) -> i64 {
    local.iter().filter(|n| !n.read).count() as i64
}

impl Inner {
    /// Add an entry to `account`'s list.
    pub(crate) fn note(
        &self,
        account: AccountId,
        category: &str,
        title: impl Into<String>,
        body: impl Into<String>,
        user_id: Option<u64>,
    ) {
        if account.is_anonymous() {
            return;
        }
        let t = now();
        let list = {
            let mut s = self.store.write();
            let d = s.data.entry(account).or_default();
            // Unique even for several entries in one second.
            let seq = d
                .feed
                .first()
                .and_then(|n| n.id.rsplit(':').next()?.parse::<u64>().ok())
                .map_or(0, |n| n + 1);
            d.feed.insert(
                0,
                StreamNotification {
                    id: format!("{PREFIX}{t}:{seq}"),
                    kind: String::new(),
                    title: title.into(),
                    body: body.into(),
                    when: iso_utc(t),
                    read: false,
                    link: None,
                    user_id,
                    category: category.to_owned(),
                },
            );
            d.feed.truncate(KEEP);
            d.feed.clone()
        };
        self.cache.put(Kind::Local, account, FEED, &list);
        self.repaint();
    }

    fn mark_feed_read(&self, account: AccountId) {
        let list = {
            let mut s = self.store.write();
            let Some(d) = s.data.get_mut(&account) else {
                return;
            };
            if d.feed.iter().all(|n| n.read) {
                return;
            }
            for n in &mut d.feed {
                n.read = true;
            }
            d.feed.clone()
        };
        self.cache.put(Kind::Local, account, FEED, &list);
        self.repaint();
    }
}

impl Core {
    /// Roblox's notification stream with the app's own entries, newest first.
    pub fn notification_feed(&self, account: AccountId) -> Res<Page<StreamNotification>> {
        let mut r = self.notifications(account, false);
        let local = self
            .inner
            .store
            .read()
            .account_data(account)
            .map(|d| d.feed.clone())
            .unwrap_or_default();
        if local.is_empty() {
            return r;
        }
        let (items, next) = match r.data.as_deref() {
            Some(p) => (merge(&p.items, p.next.is_some(), &local), p.next.clone()),
            None => (local, None),
        };
        r.data = Some(Arc::new(Page { items, next }));
        r
    }

    /// Mark the app's entries read. Clearing notifications does this along with Roblox's.
    pub(crate) fn mark_feed_read(&self, account: AccountId) {
        self.inner.mark_feed_read(account);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn n(id: &str, when: &str) -> StreamNotification {
        StreamNotification {
            id: id.to_owned(),
            when: when.to_owned(),
            ..Default::default()
        }
    }

    fn ids(v: &[StreamNotification]) -> Vec<&str> {
        v.iter().map(|n| n.id.as_str()).collect()
    }

    #[test]
    fn merges_newest_first() {
        let stream = [n("r2", "2026-01-03T00:00:00.5Z"), n("r1", "2026-01-01T00:00:00Z")];
        let local = [n("a2", "2026-01-04T00:00:00Z"), n("a1", "2026-01-02T00:00:00Z")];
        assert_eq!(ids(&merge(&stream, false, &local)), ["a2", "r2", "a1", "r1"]);
    }

    #[test]
    fn older_app_entries_wait_for_more_pages() {
        let stream = [n("r2", "2026-01-03T00:00:00Z"), n("r1", "2026-01-02T00:00:00Z")];
        let local = [n("a2", "2026-01-02T00:00:00Z"), n("a1", "2026-01-01T00:00:00Z")];
        assert_eq!(ids(&merge(&stream, true, &local)), ["r2", "r1", "a2"]);
        assert_eq!(ids(&merge(&stream, false, &local)), ["r2", "r1", "a2", "a1"]);
        assert_eq!(ids(&merge(&[], true, &local)), ["a2", "a1"]);
    }

    #[test]
    fn counts_unread() {
        let mut a = n("a", "");
        a.read = true;
        assert_eq!(unread(&[a, n("b", "")]), 1);
    }
}

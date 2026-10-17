//! Desktop alerts: things worth a system notification while the app is in the background.
//!
//! Watchers (realtime events, the health loop, presence refreshes, the sales watcher) call
//! [`Inner::alert`], which keeps only the kinds turned on in Settings → Notifications. The
//! shell takes them with [`Core::take_alerts`] and shows them. Nothing here writes to Roblox.

use std::collections::{HashMap, HashSet};

use bonsai_api::AccountId;
use bonsai_api::generated::notifications;
use serde_json::json;

use crate::Core;
use crate::core::Inner;
use crate::store::{Presence, PresenceKind, Unread};

/// Alerts kept for the shell to pick up; older ones are dropped.
const KEEP: usize = 20;

#[derive(Clone, Debug, PartialEq)]
pub struct Alert {
    pub account: AccountId,
    /// The [`crate::settings::NotifyPrefs`] kind that allowed it.
    pub kind: &'static str,
    pub title: String,
    pub body: String,
}

/// What the alert state remembers between checks, per account. Nothing alerts until a first
/// live value has been seen this run, so old news from before startup stays quiet.
#[derive(Default)]
pub(crate) struct AlertState {
    queue: Vec<Alert>,
    unread: HashMap<AccountId, Unread>,
    stream: HashMap<AccountId, HashSet<String>>,
}

/// Alerts for counts that went up: (kind, text). Messages, chat and the notification stream
/// have their own alerts, so only friend requests and trades are read from the counts.
pub fn unread_alerts(before: &Unread, after: &Unread) -> Vec<(&'static str, String)> {
    let mut out = Vec::new();
    let more = |a: i64, b: i64| (b > a).then_some(b - a);
    if let Some(n) = more(before.friend_requests, after.friend_requests) {
        out.push(("friend_requests", plural(n, "new friend request")));
    }
    if let Some(n) = more(before.trades, after.trades) {
        out.push(("trades", plural(n, "new trade offer")));
    }
    out
}

/// Friends who started playing: in a game now and known, but not in a game, before.
pub fn joined_games(before: &HashMap<u64, Presence>, after: &HashMap<u64, Presence>) -> Vec<u64> {
    let mut ids: Vec<u64> = after
        .iter()
        .filter(|(id, p)| {
            p.kind == PresenceKind::InGame && before.get(id).is_some_and(|b| b.kind != PresenceKind::InGame)
        })
        .map(|(id, _)| *id)
        .collect();
    ids.sort_unstable();
    ids
}

fn plural(n: i64, what: &str) -> String {
    if n == 1 {
        format!("1 {what}")
    } else {
        format!("{n} {what}s")
    }
}

impl Inner {
    /// Queue an alert if its kind is turned on.
    pub(crate) fn alert(
        &self,
        account: AccountId,
        kind: &'static str,
        title: impl Into<String>,
        body: impl Into<String>,
    ) {
        if !self.settings.read().notify.get(kind).unwrap_or(false) {
            return;
        }
        let mut s = self.alerts.lock();
        s.queue.push(Alert {
            account,
            kind,
            title: title.into(),
            body: body.into(),
        });
        let extra = s.queue.len().saturating_sub(KEEP);
        s.queue.drain(..extra);
        drop(s);
        self.repaint();
    }

    /// A fresh unread count from Roblox: alert for what went up since the last one.
    pub(crate) fn note_unread(&self, account: AccountId, now: Unread) {
        let before = self.alerts.lock().unread.insert(account, now);
        let Some(before) = before else { return };
        for (kind, text) in unread_alerts(&before, &now) {
            // Roblox's stream has friend requests but not trade offers.
            if kind == "trades" {
                self.note(account, "trades", "Trade offers", text.clone(), None);
            }
            self.alert(account, kind, self.account_title(account), text);
        }
        for (event, a, b) in [
            ("friend_request", before.friend_requests, now.friend_requests),
            ("trade", before.trades, now.trades),
            ("message", before.messages, now.messages),
        ] {
            if b > a {
                self.plugin_event(account, event, json!({ "new": b - a, "total": b }));
            }
        }
    }

    /// Tells plugins about friends whose status changed, as one `presence` event.
    pub(crate) fn note_presence_changes(
        &self,
        account: AccountId,
        before: &HashMap<u64, Presence>,
        after: &HashMap<u64, Presence>,
    ) {
        if !self.plugins_listen("presence") {
            return;
        }
        let mut changed: Vec<(u64, PresenceKind, &Presence)> = after
            .iter()
            .filter(|(id, _)| **id != account.0)
            .filter_map(|(id, p)| {
                before
                    .get(id)
                    .filter(|b| b.kind != p.kind)
                    .map(|b| (*id, b.kind, p))
            })
            .collect();
        if changed.is_empty() {
            return;
        }
        changed.sort_unstable_by_key(|(id, ..)| *id);
        let list: Vec<serde_json::Value> = changed
            .into_iter()
            .map(|(id, was, p)| {
                let mut v = crate::plugins::presence_view(id, p);
                v["was"] = crate::plugins::status(was).into();
                v
            })
            .collect();
        self.plugin_event(account, "presence", serde_json::Value::Array(list));
    }

    /// A fresh presence map for friends: alert for those who started playing.
    pub(crate) fn note_joins(
        &self,
        account: AccountId,
        before: &HashMap<u64, Presence>,
        after: &HashMap<u64, Presence>,
    ) {
        // The account's own presence is in the map too; joining a game yourself is no news.
        for id in joined_games(before, after)
            .into_iter()
            .filter(|u| *u != account.0)
        {
            let name = self.store.read().user_title(id);
            let place = &after[&id].last_location;
            let body = if place.is_empty() {
                "Started playing".to_owned()
            } else {
                format!("Playing {place}")
            };
            self.alert(account, "friend_joins", name, body);
        }
    }

    /// A chat message arrived over realtime.
    pub(crate) fn note_chat(&self, account: AccountId, sender: u64, conversation: &str) {
        if sender == account.0 {
            return;
        }
        self.plugin_event(
            account,
            "chat",
            json!({ "from": sender, "conversation": conversation }),
        );
        let name = self.store.read().user_title(sender);
        self.alert(account, "chats", name, "New chat message");
    }

    /// The notification stream changed: read the newest entries and alert for experience
    /// updates not seen before. The first read this run only remembers what is there.
    pub(crate) async fn note_stream(&self, account: AccountId) {
        let plugins = self.plugins_listen("game_update");
        if !self.settings.read().notify.experience_updates && !plugins {
            return;
        }
        let Some(c) = self.client(account) else { return };
        let p = notifications::v2::GetStreamNotificationsGetRecentParams {
            start_index: Some(0),
            max_rows: Some(10),
        };
        let Ok(list) = notifications::v2::get_stream_notifications_get_recent(&c, &p).await else {
            return;
        };
        let entries: Vec<_> = list.into_iter().map(crate::messages::stream_entry).collect();
        let fresh: Vec<_> = {
            let mut s = self.alerts.lock();
            let first = !s.stream.contains_key(&account);
            let seen = s.stream.entry(account).or_default();
            entries
                .iter()
                .filter(|n| seen.insert(n.id.clone()) && !first && !n.read && n.category == "experiences")
                .cloned()
                .collect()
        };
        for n in fresh {
            let title = if n.title.is_empty() {
                "Experience update".to_owned()
            } else {
                n.title
            };
            if plugins {
                self.plugin_event(account, "game_update", json!({ "title": title, "body": n.body }));
            }
            self.alert(account, "experience_updates", title, n.body);
        }
    }

    fn account_title(&self, account: AccountId) -> String {
        self.store
            .read()
            .accounts
            .accounts
            .iter()
            .find(|a| a.id == account)
            .map(|a| a.label.clone().unwrap_or_else(|| a.display_name.clone()))
            .unwrap_or_default()
    }
}

impl Core {
    /// Alerts since the last call, oldest first.
    pub fn take_alerts(&self) -> Vec<Alert> {
        std::mem::take(&mut self.inner.alerts.lock().queue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_that_went_up() {
        let a = Unread {
            friend_requests: 1,
            trades: 2,
            ..Default::default()
        };
        let b = Unread {
            friend_requests: 3,
            trades: 2,
            messages: 9,
            ..Default::default()
        };
        assert_eq!(
            unread_alerts(&a, &b),
            vec![("friend_requests", "2 new friend requests".to_owned())]
        );
        let c = Unread { trades: 3, ..b };
        assert_eq!(
            unread_alerts(&b, &c),
            vec![("trades", "1 new trade offer".to_owned())]
        );
        assert!(unread_alerts(&b, &a).is_empty());
    }

    #[test]
    fn friends_who_started_playing() {
        let p = |kind| Presence {
            kind,
            ..Default::default()
        };
        let before = HashMap::from([(1, p(PresenceKind::Online)), (2, p(PresenceKind::InGame))]);
        let after = HashMap::from([
            (1, p(PresenceKind::InGame)),
            (2, p(PresenceKind::InGame)),
            // Not known before: the first presence read, not a join.
            (3, p(PresenceKind::InGame)),
        ]);
        assert_eq!(joined_games(&before, &after), vec![1]);
        assert!(joined_games(&after, &before).is_empty());
    }
}

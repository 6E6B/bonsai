//! Public servers an account played in over the last day. Roblox doesn't keep this, so it comes
//! from the account's own presence, which carries the job id of the server it is in. Kept per
//! account in the local cache; nothing in here sends a request except the presence checks right
//! after a launch, which are reads.

use std::sync::Arc;
use std::time::Duration;

use bonsai_api::AccountId;
use serde::{Deserialize, Serialize};

use crate::cache::{Kind, now};
use crate::core::Inner;
use crate::store::{Presence, PresenceKind};

pub(crate) const RECENT_SERVERS: &str = "recent_servers";

/// How long a server stays in the list after the account was last seen in it.
pub const KEEP_SECS: i64 = 24 * 60 * 60;

/// At most this many servers per account, across all experiences.
const MAX: usize = 100;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecentServer {
    pub job_id: String,
    pub place_id: u64,
    #[serde(default)]
    pub root_place_id: Option<u64>,
    /// First and last time the account was seen in it, in unix seconds.
    #[serde(default)]
    pub first_seen: i64,
    #[serde(default)]
    pub last_seen: i64,
}

impl RecentServer {
    /// Played in `place_id`, or in a place of the experience whose root is `place_id`.
    pub fn is_for(&self, place_id: u64) -> bool {
        self.place_id == place_id || self.root_place_id == Some(place_id)
    }
}

/// Drop servers last seen more than [`KEEP_SECS`] before `at`. True when anything went.
pub fn prune(list: &mut Vec<RecentServer>, at: i64) -> bool {
    let n = list.len();
    list.retain(|s| at - s.last_seen < KEEP_SECS);
    list.len() != n
}

/// Note that the account is in the server its presence `p` names, at `at`. Newest first.
/// True when the list changed.
pub fn note(list: &mut Vec<RecentServer>, p: &Presence, at: i64) -> bool {
    let changed = prune(list, at);
    let (Some(job_id), Some(place_id)) = (p.job_id.as_deref(), p.place_id) else {
        return changed;
    };
    if p.kind != PresenceKind::InGame || job_id.is_empty() {
        return changed;
    }
    let first_seen = match list.iter().position(|s| s.job_id == job_id) {
        Some(0) if list[0].last_seen == at => return changed,
        Some(i) => list.remove(i).first_seen,
        None => at,
    };
    list.insert(
        0,
        RecentServer {
            job_id: job_id.to_owned(),
            place_id,
            root_place_id: p.root_place_id,
            first_seen,
            last_seen: at,
        },
    );
    list.truncate(MAX);
    true
}

impl Inner {
    /// A fresh presence for `account` itself: remember the server it is in.
    pub(crate) fn note_own_presence(&self, account: AccountId, p: &Presence) {
        if account.is_anonymous() {
            return;
        }
        let snapshot = {
            let mut s = self.store.write();
            let d = s.data.entry(account).or_default();
            if !note(&mut d.recent_servers, p, now()) {
                return;
            }
            d.recent_servers.clone()
        };
        self.cache.put(Kind::Local, account, RECENT_SERVERS, &snapshot);
    }

    /// After a launch, read the account's own presence a few times until it shows a new server,
    /// so a short session is still remembered.
    pub(crate) fn watch_launch(self: &Arc<Self>, account: AccountId) {
        let inner = self.clone();
        let job = move |inner: &Inner| {
            inner
                .store
                .read()
                .data
                .get(&account)
                .and_then(|d| d.presence.get(&account.0))
                .filter(|p| p.kind == PresenceKind::InGame)
                .and_then(|p| p.job_id.clone())
        };
        let before = job(self);
        self.rt.spawn(async move {
            for _ in 0..8 {
                tokio::time::sleep(Duration::from_secs(15)).await;
                inner.refresh_presence(account, &[]).await;
                let now = job(&inner);
                if now.is_some() && now != before {
                    inner.repaint();
                    return;
                }
            }
        });
    }
}

impl crate::core::Core {
    /// Servers `account` played in for the experience at `place_id` over the last day, most
    /// recent first.
    pub fn recent_servers(&self, account: AccountId, place_id: u64) -> Vec<RecentServer> {
        let at = now();
        self.store()
            .data
            .get(&account)
            .map(|d| {
                d.recent_servers
                    .iter()
                    .filter(|s| s.is_for(place_id) && at - s.last_seen < KEEP_SECS)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn in_game(job: &str, place: u64) -> Presence {
        Presence {
            kind: PresenceKind::InGame,
            place_id: Some(place),
            root_place_id: Some(1),
            universe_id: Some(9),
            job_id: Some(job.into()),
            last_location: String::new(),
        }
    }

    #[test]
    fn notes_newest_first_and_keeps_first_seen() {
        let mut l = Vec::new();
        assert!(note(&mut l, &in_game("a", 1), 100));
        assert!(note(&mut l, &in_game("b", 1), 200));
        assert!(note(&mut l, &in_game("a", 1), 300));
        assert_eq!(l.len(), 2);
        assert_eq!(l[0].job_id, "a");
        assert_eq!((l[0].first_seen, l[0].last_seen), (100, 300));
        assert_eq!(l[1].job_id, "b");
    }

    #[test]
    fn same_sighting_is_no_change() {
        let mut l = Vec::new();
        note(&mut l, &in_game("a", 1), 100);
        assert!(!note(&mut l, &in_game("a", 1), 100));
    }

    #[test]
    fn ignores_presence_without_a_server() {
        let mut l = Vec::new();
        let online = Presence {
            kind: PresenceKind::Online,
            ..in_game("a", 1)
        };
        assert!(!note(&mut l, &online, 100));
        let hidden = Presence {
            job_id: None,
            ..in_game("a", 1)
        };
        assert!(!note(&mut l, &hidden, 100));
        assert!(l.is_empty());
    }

    #[test]
    fn drops_servers_after_a_day() {
        let mut l = Vec::new();
        note(&mut l, &in_game("a", 1), 0);
        note(&mut l, &in_game("b", 1), 10);
        assert!(prune(&mut l, KEEP_SECS + 5));
        assert_eq!(l.len(), 1);
        assert_eq!(l[0].job_id, "b");
    }

    #[test]
    fn matches_sub_places_by_root() {
        let s = RecentServer {
            job_id: "a".into(),
            place_id: 5,
            root_place_id: Some(1),
            ..Default::default()
        };
        assert!(s.is_for(5));
        assert!(s.is_for(1));
        assert!(!s.is_for(2));
    }
}

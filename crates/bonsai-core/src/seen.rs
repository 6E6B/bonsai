//! What Roblox doesn't keep for you: when a user was last seen online, and which friends
//! removed you. Both come from presence and friends-list reads the app already makes, are kept
//! per account in the local cache, and survive "clear cache". Nothing in here sends a request.

use std::collections::{HashMap, HashSet};

use bonsai_api::AccountId;

use crate::cache::{Kind, now};
use crate::core::Inner;
use crate::store::PresenceKind;

pub(crate) const LAST_SEEN: &str = "last_seen";
pub(crate) const UNFRIENDED: &str = "unfriended";

/// Online, in a game or in Studio. Invisible only shows for your own account.
pub fn is_online(k: PresenceKind) -> bool {
    matches!(
        k,
        PresenceKind::Online | PresenceKind::InGame | PresenceKind::InStudio
    )
}

/// Stamp everyone in `seen` who is online with `at`. True when anything changed.
pub fn stamp(last: &mut HashMap<u64, i64>, seen: &[(u64, PresenceKind)], at: i64) -> bool {
    let mut changed = false;
    for (id, k) in seen {
        if is_online(*k) && last.insert(*id, at) != Some(at) {
            changed = true;
        }
    }
    changed
}

/// Friends in `before` that are gone from `after`, minus the ones you removed yourself. An empty
/// `after` against a list of several is taken as a bad response, not a mass unfriending.
pub fn removed(before: &[u64], after: &[u64], by_me: &HashSet<u64>) -> Vec<u64> {
    if after.is_empty() && before.len() > 3 {
        return Vec::new();
    }
    let now: HashSet<u64> = after.iter().copied().collect();
    before
        .iter()
        .filter(|id| !now.contains(id) && !by_me.contains(id))
        .copied()
        .collect()
}

impl Inner {
    /// Remember who `account` just saw online.
    pub(crate) fn note_presence(&self, account: AccountId, seen: &[(u64, PresenceKind)]) {
        if account.is_anonymous() {
            return;
        }
        let snapshot = {
            let mut s = self.store.write();
            let d = s.data.entry(account).or_default();
            if !stamp(&mut d.last_seen, seen, now()) {
                return;
            }
            d.last_seen.clone()
        };
        self.cache.put(Kind::Local, account, LAST_SEEN, &snapshot);
    }

    /// Compare a fresh friends list with the one before it and say who removed `account`.
    /// `before` is `None` on the first fetch ever, when there is nothing to compare with.
    pub(crate) fn note_friends(&self, account: AccountId, before: Option<&[u64]>, after: &[u64]) {
        let (gone, unfriended) = {
            let mut s = self.store.write();
            let d = s.data.entry(account).or_default();
            let gone = before
                .map(|b| removed(b, after, &d.removed_by_me))
                .unwrap_or_default();
            // Once they're out of the list, a later removal is theirs again.
            d.removed_by_me.retain(|id| after.contains(id));
            let back = after.iter().any(|id| d.unfriended.contains_key(id));
            if gone.is_empty() && !back {
                return;
            }
            d.unfriended.retain(|id, _| !after.contains(id));
            let t = now();
            for id in &gone {
                d.unfriended.insert(*id, t);
            }
            (gone, d.unfriended.clone())
        };
        self.cache.put(Kind::Local, account, UNFRIENDED, &unfriended);
        for id in gone {
            let name = self.store.read().user_title(id);
            self.toast(Some(account), format!("{name} is no longer your friend."), false);
            self.note(account, "friends", name, "Removed you as a friend", Some(id));
        }
    }

    /// You removed `user_id`: drop them from the list now so the next refresh doesn't report it.
    pub(crate) fn forget_friend(&self, account: AccountId, user_id: u64) {
        let list = {
            let mut s = self.store.write();
            let d = s.data.entry(account).or_default();
            d.removed_by_me.insert(user_id);
            let Some(list) = d.friends.data.as_mut() else {
                return;
            };
            list.retain(|id| *id != user_id);
            list.clone()
        };
        self.cache.put(Kind::Friends, account, "friends", &list);
        self.repaint();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stamps_only_online_users() {
        let mut last = HashMap::from([(3, 50)]);
        let seen = [
            (1, PresenceKind::InGame),
            (2, PresenceKind::Offline),
            (3, PresenceKind::Invisible),
            (4, PresenceKind::InStudio),
        ];
        assert!(stamp(&mut last, &seen, 100));
        assert_eq!(last, HashMap::from([(1, 100), (3, 50), (4, 100)]));
        assert!(!stamp(&mut last, &seen, 100), "same stamp is no change");
        assert!(!stamp(&mut last, &[(2, PresenceKind::Offline)], 200));
    }

    #[test]
    fn finds_who_left() {
        let none = HashSet::new();
        assert_eq!(removed(&[1, 2, 3], &[1, 3, 4], &none), vec![2]);
        assert!(removed(&[1, 2], &[1, 2], &none).is_empty());
        assert_eq!(
            removed(&[1, 2, 3], &[3], &HashSet::from([1])),
            vec![2],
            "your own removals don't count"
        );
        assert!(
            removed(&[1, 2, 3, 4, 5], &[], &none).is_empty(),
            "an empty list is a bad response"
        );
        assert_eq!(removed(&[1], &[], &none), vec![1]);
    }
}

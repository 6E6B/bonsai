//! Social overlap: which of your own accounts' friends also know someone, and the friends two
//! of your accounts share. Read-only and computed from cached friend lists.

use std::collections::HashSet;

use bonsai_api::AccountId;

use crate::Core;
use crate::core::Cmd;

/// One of your accounts and how it connects to the person on screen.
#[derive(Clone, Debug, PartialEq)]
pub struct AccountOverlap {
    pub account: AccountId,
    /// The account is friends with them itself.
    pub friend: bool,
    /// Friends of this account who are also their friends, in their list's order.
    pub mutuals: Vec<u64>,
}

/// Your accounts' connections to one user.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Overlap {
    pub loading: bool,
    pub error: Option<String>,
    /// How many of their friends have loaded.
    pub checked: usize,
    /// All of their friends have loaded.
    pub complete: bool,
    /// Only accounts that are friends with them or share a friend.
    pub accounts: Vec<AccountOverlap>,
}

/// The friends two of your accounts share.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Shared {
    /// Either list has not loaded yet.
    pub loading: bool,
    pub a_count: usize,
    pub b_count: usize,
    /// The two accounts are friends with each other.
    pub linked: bool,
    /// In `a`'s order.
    pub friends: Vec<u64>,
}

/// For each own account (`own`: id and friends), whether it is friends with `target` and which of
/// its friends appear in `theirs`. Accounts with no connection are left out; `target` is never one
/// of its own mutuals.
pub fn overlap(theirs: &[u64], target: u64, own: &[(AccountId, Vec<u64>)]) -> Vec<AccountOverlap> {
    own.iter()
        .filter(|(a, _)| a.0 != target)
        .filter_map(|(a, friends)| {
            let set: HashSet<u64> = friends.iter().copied().collect();
            let mutuals: Vec<u64> = theirs
                .iter()
                .copied()
                .filter(|id| *id != target && set.contains(id))
                .collect();
            let friend = set.contains(&target);
            (friend || !mutuals.is_empty()).then_some(AccountOverlap {
                account: *a,
                friend,
                mutuals,
            })
        })
        .collect()
}

/// Friends in both lists, in `a`'s order, leaving out the two accounts themselves.
pub fn shared(a: AccountId, a_friends: &[u64], b: AccountId, b_friends: &[u64]) -> Vec<u64> {
    let set: HashSet<u64> = b_friends.iter().copied().collect();
    a_friends
        .iter()
        .copied()
        .filter(|id| *id != a.0 && *id != b.0 && set.contains(id))
        .collect()
}

impl Core {
    /// Your signed-in accounts' cached friend lists, skipping ones that have not loaded.
    fn own_friend_lists(&self) -> Vec<(AccountId, Vec<u64>)> {
        let s = self.store();
        s.accounts
            .accounts
            .iter()
            .map(|a| a.id)
            .filter(|a| !a.is_anonymous())
            .filter_map(|a| Some((a, s.account_data(a)?.friends.data.clone()?)))
            .collect()
    }

    /// How your accounts connect to `user_id`, using their friends as `viewer` has loaded them.
    /// Shows what is cached at once; loading more of their friends goes through `list.more`.
    pub fn social_overlap(&self, viewer: AccountId, user_id: u64) -> Overlap {
        let own = self.own_friend_lists();
        if viewer.is_anonymous() {
            return Overlap {
                accounts: overlap(&[], user_id, &own),
                ..Default::default()
            };
        }
        let r = self.friends_of(viewer, user_id, false);
        let (theirs, complete) = match &r.data {
            Some(p) => (p.items.as_slice(), p.next.is_none()),
            None => (&[][..], false),
        };
        Overlap {
            loading: r.loading,
            error: r.error.clone(),
            checked: theirs.len(),
            complete,
            accounts: overlap(theirs, user_id, &own),
        }
    }

    /// The friends accounts `a` and `b` share, from their cached friend lists. Asks for a
    /// refresh of any list that has not loaded.
    pub fn shared_friends(&self, a: AccountId, b: AccountId) -> Shared {
        let (fa, fb) = {
            let s = self.store();
            let get = |id: AccountId| s.account_data(id).and_then(|d| d.friends.data.clone());
            (get(a), get(b))
        };
        for (id, list) in [(a, &fa), (b, &fb)] {
            if list.is_none() && !id.is_anonymous() {
                self.send(Cmd::Refresh(id));
            }
        }
        let (Some(fa), Some(fb)) = (fa, fb) else {
            return Shared {
                loading: true,
                ..Default::default()
            };
        };
        Shared {
            loading: false,
            a_count: fa.len(),
            b_count: fb.len(),
            linked: fa.contains(&b.0) || fb.contains(&a.0),
            friends: shared(a, &fa, b, &fb),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlap_keeps_connected_accounts_in_their_order() {
        let own = vec![
            (AccountId(1), vec![10, 11, 12]),
            (AccountId(2), vec![99]),
            (AccountId(3), vec![50, 12]),
            (AccountId(4), vec![7]),
        ];
        let got = overlap(&[12, 13, 10, 7], 50, &own);
        assert_eq!(
            got,
            vec![
                AccountOverlap {
                    account: AccountId(1),
                    friend: false,
                    mutuals: vec![12, 10],
                },
                AccountOverlap {
                    account: AccountId(3),
                    friend: true,
                    mutuals: vec![12],
                },
                AccountOverlap {
                    account: AccountId(4),
                    friend: false,
                    mutuals: vec![7],
                },
            ]
        );
    }

    #[test]
    fn overlap_skips_the_person_themselves() {
        let own = vec![(AccountId(50), vec![10]), (AccountId(1), vec![50])];
        let got = overlap(&[10, 50], 50, &own);
        assert_eq!(got.len(), 1);
        assert_eq!(got[0].account, AccountId(1));
        assert!(got[0].friend);
        assert!(got[0].mutuals.is_empty());
    }

    #[test]
    fn shared_leaves_out_both_accounts() {
        assert_eq!(
            shared(AccountId(1), &[2, 5, 6, 7], AccountId(2), &[1, 7, 5]),
            vec![5, 7]
        );
        assert!(shared(AccountId(1), &[], AccountId(2), &[3]).is_empty());
    }
}

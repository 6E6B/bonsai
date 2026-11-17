//! Social overlap views: how your accounts connect to someone, and friends two accounts share.

use std::sync::Arc;

use bonsai_core::{AccountId, Core};
use serde::Serialize;
use specta::Type;
use tauri::State;

use super::Id;

/// A person in an overlap list. Names are empty until the user is known.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct OverlapPerson {
    pub id: Id,
    pub name: String,
    pub display_name: String,
}

/// One of your accounts and how it connects to the person on screen.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct OverlapAccount {
    pub account: Id,
    /// The account's label, display name or username.
    pub label: String,
    /// The account is friends with them itself.
    pub friend: bool,
    pub mutuals: Vec<OverlapPerson>,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct OverlapView {
    pub loading: bool,
    pub error: Option<String>,
    /// How many of their friends have loaded.
    pub checked: u32,
    /// All of their friends have loaded.
    pub complete: bool,
    pub accounts: Vec<OverlapAccount>,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct SharedFriendsView {
    pub loading: bool,
    pub a_count: u32,
    pub b_count: u32,
    /// The two accounts are friends with each other.
    pub linked: bool,
    pub friends: Vec<OverlapPerson>,
}

/// People with names from the store; asks for any that are missing.
fn people(core: &Core, ids: &[u64]) -> Vec<OverlapPerson> {
    let mut unnamed = Vec::new();
    let out = {
        let s = core.store();
        ids.iter()
            .map(|id| {
                let u = s.users.get(id);
                if !u.is_some_and(|u| u.is_complete()) {
                    unnamed.push(*id);
                }
                OverlapPerson {
                    id: Id(*id),
                    name: u.map(|u| u.name.clone()).unwrap_or_default(),
                    display_name: u.map(|u| u.display_name.clone()).unwrap_or_default(),
                }
            })
            .collect()
    };
    core.want_users(&unnamed);
    out
}

fn account_label(core: &Core, id: AccountId) -> String {
    let s = core.store();
    s.accounts
        .accounts
        .iter()
        .find(|a| a.id == id)
        .map(|a| {
            a.label
                .clone()
                .filter(|l| !l.trim().is_empty())
                .or_else(|| (!a.display_name.is_empty()).then(|| a.display_name.clone()))
                .unwrap_or_else(|| a.name.clone())
        })
        .unwrap_or_default()
}

/// Which of your accounts are friends with `user_id` or share friends with them, using their
/// friends as the active account has loaded them.
#[tauri::command]
#[specta::specta]
pub fn social_overlap(core: State<'_, Arc<Core>>, user_id: Id) -> OverlapView {
    let viewer = core.acting(None).unwrap_or(AccountId::ANONYMOUS);
    let o = core.social_overlap(viewer, user_id.0);
    let accounts = o
        .accounts
        .iter()
        .map(|a| OverlapAccount {
            account: Id(a.account.0),
            label: account_label(&core, a.account),
            friend: a.friend,
            mutuals: people(&core, &a.mutuals),
        })
        .collect();
    OverlapView {
        loading: o.loading,
        error: o.error,
        checked: o.checked as u32,
        complete: o.complete,
        accounts,
    }
}

/// Friends that two of your accounts share, from their cached friend lists.
#[tauri::command]
#[specta::specta]
pub fn shared_friends(core: State<'_, Arc<Core>>, a: Id, b: Id) -> SharedFriendsView {
    let r = core.shared_friends(AccountId(a.0), AccountId(b.0));
    SharedFriendsView {
        loading: r.loading,
        a_count: r.a_count as u32,
        b_count: r.b_count as u32,
        linked: r.linked,
        friends: people(&core, &r.friends),
    }
}

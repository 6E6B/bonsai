//! Local bookmarks. Public entity pages and wiki are shared; other views belong to the
//! account that saved them. Local cache rows survive cache clearing and pruning.

use bonsai_api::AccountId;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

use crate::{Core, cache};

const KEY: &str = "bookmarks";
pub const TITLE_MAX: usize = 120;
pub const MAX_BOOKMARKS: usize = 1000;
// Serialize read/modify/write across account and shared rows.
static EDIT: Mutex<()> = Mutex::new(());

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BookmarkKind {
    Experience,
    Person,
    Item,
    Community,
    View,
    #[default]
    #[serde(other)]
    Unknown,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Bookmark {
    pub id: u64,
    pub path: String,
    pub title: String,
    pub kind: BookmarkKind,
    pub entity_id: Option<u64>,
    pub account: Option<AccountId>,
    pub added_at: i64,
    pub position: usize,
}

/// Only complete entity routes are public. New and unknown routes default to private.
pub fn entity(path: &str) -> Option<(&str, u64)> {
    let pathname = path.split(['?', '#']).next()?;
    let mut parts = pathname.strip_prefix('/')?.split('/');
    let route = parts.next()?;
    let raw = parts.next()?;
    if raw.starts_with('0') || !raw.bytes().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let id = raw.parse::<u64>().ok().filter(|id| *id > 0)?;
    if parts.next().is_some()
        || !matches!(
            route,
            "games" | "users" | "catalog" | "bundles" | "passes" | "communities"
        )
    {
        return None;
    }
    Some((route, id))
}

pub fn kind(path: &str) -> BookmarkKind {
    match entity(path).map(|(route, _)| route) {
        Some("games") => BookmarkKind::Experience,
        Some("users") => BookmarkKind::Person,
        Some("catalog" | "bundles" | "passes") => BookmarkKind::Item,
        Some("communities") => BookmarkKind::Community,
        _ => BookmarkKind::View,
    }
}

pub fn shared(path: &str) -> bool {
    let pathname = path.split(['?', '#']).next().unwrap_or("");
    entity(path).is_some() || pathname == "/wiki" || pathname.starts_with("/wiki/")
}

/// Keep the original query when opening, but deduplicate an entity across its sub-tabs.
pub fn identity(path: &str) -> &str {
    if entity(path).is_some() {
        path.split(['?', '#']).next().unwrap_or(path)
    } else {
        path
    }
}

fn validate_path(path: &str) -> Result<(), String> {
    if !path.starts_with('/')
        || path.starts_with("//")
        || path.len() > 4096
        || path.contains(['\\', '#'])
        || path.chars().any(char::is_control)
        || path.split(['/', '?']).any(|p| p == "." || p == "..")
    {
        return Err("Choose a page in Bonsai.".into());
    }
    Ok(())
}

fn clean_title(title: &str) -> Result<String, String> {
    let title: String = title.trim().chars().take(TITLE_MAX).collect();
    if title.is_empty() {
        return Err("Name the bookmark.".into());
    }
    Ok(title)
}

pub fn toggle(all: &mut Vec<Bookmark>, account: AccountId, path: &str, title: &str) -> Result<bool, String> {
    validate_path(path)?;
    let scope = if shared(path) { None } else { Some(account) };
    if scope.is_some_and(|a| a.is_anonymous()) {
        return Err("Sign in to bookmark this page.".into());
    }
    if let Some(at) = all
        .iter()
        .position(|b| b.account == scope && identity(&b.path) == identity(path))
    {
        all.remove(at);
        return Ok(false);
    }
    if all.len() >= MAX_BOOKMARKS {
        return Err(format!("Keep up to {MAX_BOOKMARKS} bookmarks."));
    }
    let title = clean_title(title)?;
    let kind = kind(path);
    let position = all
        .iter()
        .filter(|b| b.kind == kind)
        .map(|b| b.position)
        .max()
        .map_or(0, |p| p.saturating_add(1));
    let mut id = fastrand::u64(1..(1 << 53));
    while all.iter().any(|b| b.id == id) {
        id = fastrand::u64(1..(1 << 53));
    }
    all.push(Bookmark {
        id,
        path: path.into(),
        title,
        kind,
        entity_id: entity(path).map(|(_, id)| id),
        account: scope,
        added_at: cache::now(),
        position,
    });
    Ok(true)
}

pub fn rename(all: &mut [Bookmark], id: u64, title: &str) -> Result<(), String> {
    let title = clean_title(title)?;
    all.iter_mut()
        .find(|b| b.id == id)
        .ok_or("No such bookmark.")?
        .title = title;
    Ok(())
}

pub fn remove(all: &mut Vec<Bookmark>, id: u64) -> Result<(), String> {
    let at = all.iter().position(|b| b.id == id).ok_or("No such bookmark.")?;
    all.remove(at);
    Ok(())
}

/// Move one place within the kind, or before a target row when dragging.
pub fn reorder(all: &mut [Bookmark], id: u64, by: isize, before: Option<u64>) -> Result<(), String> {
    let kind = all.iter().find(|b| b.id == id).ok_or("No such bookmark.")?.kind;
    let mut group: Vec<_> = all
        .iter()
        .filter(|b| b.kind == kind)
        .map(|b| (b.id, b.position, b.added_at))
        .collect();
    group.sort_by_key(|b| (b.1, b.2, b.0));
    let at = group.iter().position(|b| b.0 == id).ok_or("No such bookmark.")?;
    let to = if let Some(before) = before {
        group
            .iter()
            .position(|b| b.0 == before)
            .ok_or("Move within the same group.")?
    } else {
        at.saturating_add_signed(by).min(group.len() - 1)
    };
    let row = group.remove(at);
    group.insert(if before.is_some() && at < to { to - 1 } else { to }, row);
    for (position, row) in group.iter().enumerate() {
        if let Some(b) = all.iter_mut().find(|b| b.id == row.0) {
            b.position = position;
        }
    }
    Ok(())
}

impl Core {
    /// Read existing resource snapshots, without scheduling a fetch.
    pub fn cached_bookmark_community(
        &self,
        id: u64,
    ) -> Option<std::sync::Arc<crate::communities::Community>> {
        self.inner
            .res
            .snapshot::<crate::communities::Community>(&crate::res::Key::shared("community", id))?
            .data
    }

    pub fn cached_bookmark_item(&self, path: &str) -> Option<crate::market::CatalogItem> {
        let (route, id) = entity(path)?;
        let item = match route {
            "catalog" => crate::market::ItemRef::asset(id),
            "bundles" => crate::market::ItemRef::bundle(id),
            _ => return None,
        };
        self.inner
            .res
            .snapshot::<crate::market::ItemDetails>(&crate::res::Key::shared(
                "item",
                format!("{:?}:{}", item.kind, item.id),
            ))?
            .data
            .as_ref()
            .map(|d| d.card.clone())
    }

    pub fn bookmarks(&self, account: AccountId) -> Vec<Bookmark> {
        if self.accounts_locked() {
            return Vec::new();
        }
        let mut all: Vec<Bookmark> = self.local(AccountId::ANONYMOUS, KEY);
        all.retain(|b| b.account.is_none());
        if !account.is_anonymous() {
            all.extend(
                self.local::<Vec<Bookmark>>(account, KEY)
                    .into_iter()
                    .filter(|b| b.account == Some(account)),
            );
        }
        all.sort_by_key(|b| (b.position, b.added_at, b.id));
        all
    }

    pub fn edit_bookmarks<T>(
        &self,
        account: AccountId,
        f: impl FnOnce(&mut Vec<Bookmark>) -> Result<T, String>,
    ) -> Result<T, String> {
        let _edit = EDIT.lock();
        if self.accounts_locked() {
            return Err("Unlock Bonsai to access bookmarks.".into());
        }
        let mut all = self.bookmarks(account);
        let before = all.len();
        let out = f(&mut all)?;
        if all.len() > before {
            let accounts: Vec<_> = self
                .store()
                .accounts
                .accounts
                .iter()
                .map(|a| a.id)
                .filter(|a| *a != account && !a.is_anonymous())
                .collect();
            let hidden: usize = accounts
                .into_iter()
                .map(|a| self.local::<Vec<Bookmark>>(a, KEY).len())
                .sum();
            if all.len().saturating_add(hidden) > MAX_BOOKMARKS {
                return Err(format!("Keep up to {MAX_BOOKMARKS} bookmarks."));
            }
        }
        self.set_local(
            AccountId::ANONYMOUS,
            KEY,
            &all.iter().filter(|b| b.account.is_none()).collect::<Vec<_>>(),
        );
        if !account.is_anonymous() {
            self.set_local(
                account,
                KEY,
                &all.iter()
                    .filter(|b| b.account == Some(account))
                    .collect::<Vec<_>>(),
            );
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[test]
    fn paths_kinds_and_scope_are_conservative() {
        for (path, expected) in [
            ("/games/1?tab=servers", BookmarkKind::Experience),
            ("/users/2", BookmarkKind::Person),
            ("/catalog/3", BookmarkKind::Item),
            ("/bundles/4", BookmarkKind::Item),
            ("/passes/5", BookmarkKind::Item),
            ("/communities/6", BookmarkKind::Community),
        ] {
            assert_eq!(kind(path), expected);
            assert!(shared(path));
        }
        for path in [
            "/settings",
            "/inventory/2",
            "/chat?conversation=1",
            "/games/1/private",
            "/users",
            "/games/nope",
            "/games/01",
            "/future",
            "/transactions",
        ] {
            assert!(!shared(path), "{path}");
            assert_eq!(kind(path), BookmarkKind::View);
        }
        assert!(shared("/wiki/Roblox"));
        assert_eq!(
            serde_json::from_str::<BookmarkKind>("\"future\"").unwrap(),
            BookmarkKind::Unknown
        );
        assert_eq!(
            serde_json::from_str::<Bookmark>("{}").unwrap(),
            Bookmark::default()
        );
    }

    #[test]
    fn toggle_deduplicates_entities_but_keeps_queries_and_scopes() {
        let mut all = vec![];
        let a = AccountId(1);
        assert_eq!(toggle(&mut all, a, "/games/1?tab=servers", "Game"), Ok(true));
        assert_eq!(toggle(&mut all, AccountId(2), "/games/1", "Game"), Ok(false));
        assert!(all.is_empty());
        for (a, path) in [(a, "/chat?a=1"), (a, "/chat?a=2"), (AccountId(2), "/chat?a=1")] {
            toggle(&mut all, a, path, "Chat").unwrap();
        }
        assert_eq!(all.len(), 3);
        assert!(toggle(&mut all, AccountId::ANONYMOUS, "/chat", "Chat").is_err());
        for path in [
            "https://example.com",
            "//example.com",
            "/\\example.com",
            "/../chat",
            "/chat\n",
        ] {
            assert!(toggle(&mut all, a, path, "Bad").is_err());
        }
    }

    #[test]
    fn rename_reorder_remove_and_caps() {
        let a = AccountId(1);
        let mut all = vec![];
        for path in ["/games/1", "/users/2", "/games/3", "/games/4"] {
            toggle(&mut all, a, path, "Name").unwrap();
        }
        let ids: Vec<_> = all.iter().map(|b| b.id).collect();
        reorder(&mut all, ids[3], -1, Some(ids[0])).unwrap();
        assert_eq!(all[3].position, 0);
        reorder(&mut all, ids[3], 1, None).unwrap();
        assert_eq!(all[3].position, 1);
        assert!(reorder(&mut all, ids[0], 0, Some(ids[1])).is_err());
        rename(&mut all, ids[0], &"é".repeat(200)).unwrap();
        assert_eq!(all[0].title.chars().count(), TITLE_MAX);
        assert!(rename(&mut all, ids[0], "  ").is_err());
        remove(&mut all, ids[0]).unwrap();
        assert!(remove(&mut all, ids[0]).is_err());
        all.resize(MAX_BOOKMARKS, Bookmark::default());
        assert!(toggle(&mut all, a, "/games/100", "Game").is_err());
    }
}

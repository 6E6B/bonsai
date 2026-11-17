//! Local bookmarks. Public entity pages and wiki are shared; other views belong to the
//! account that saved them. Local cache rows survive cache clearing and pruning.

use bonsai_api::AccountId;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

use crate::{Core, cache};

const KEY: &str = "bookmarks";
pub const TITLE_MAX: usize = 120;
pub const MAX_BOOKMARKS: usize = 1000;
pub const FOLDER_MAX: usize = 40;
pub const TAG_MAX: usize = 24;
pub const MAX_TAGS: usize = 8;
/// Largest bookmark file `import` reads.
pub const FILE_MAX: u64 = 4 << 20;
const FORMAT: u32 = 1;
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
    /// Empty when the bookmark sits in its kind's group.
    pub folder: String,
    pub tags: Vec<String>,
}

impl Bookmark {
    /// Rows that order together: one folder, or one kind outside folders.
    fn same_group(&self, other: &Bookmark) -> bool {
        if self.folder.is_empty() {
            other.folder.is_empty() && other.kind == self.kind
        } else {
            other.folder == self.folder
        }
    }
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
    add(all, scope, path, title, "", Vec::new())?;
    Ok(true)
}

fn add(
    all: &mut Vec<Bookmark>,
    scope: Option<AccountId>,
    path: &str,
    title: &str,
    folder: &str,
    tags: Vec<String>,
) -> Result<(), String> {
    if all.len() >= MAX_BOOKMARKS {
        return Err(format!("Keep up to {MAX_BOOKMARKS} bookmarks."));
    }
    let title = clean_title(title)?;
    let mut id = fastrand::u64(1..(1 << 53));
    while all.iter().any(|b| b.id == id) {
        id = fastrand::u64(1..(1 << 53));
    }
    let mut row = Bookmark {
        id,
        path: path.into(),
        title,
        kind: kind(path),
        entity_id: entity(path).map(|(_, id)| id),
        account: scope,
        added_at: cache::now(),
        position: 0,
        folder: existing_folder(all, &clean_folder(folder)),
        tags,
    };
    row.position = next_position(all, &row);
    all.push(row);
    Ok(())
}

fn next_position(all: &[Bookmark], row: &Bookmark) -> usize {
    all.iter()
        .filter(|b| b.id != row.id && row.same_group(b))
        .map(|b| b.position)
        .max()
        .map_or(0, |p| p.saturating_add(1))
}

/// Trimmed, single-spaced, no control characters, at most [`FOLDER_MAX`] characters.
fn clean_folder(name: &str) -> String {
    name.split(|c: char| c.is_whitespace() || c.is_control())
        .filter(|w| !w.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .take(FOLDER_MAX)
        .collect::<String>()
        .trim_end()
        .to_owned()
}

/// Reuse an existing folder's spelling so "games" and "Games" stay one folder.
fn existing_folder(all: &[Bookmark], name: &str) -> String {
    all.iter()
        .find(|b| !b.folder.is_empty() && b.folder.to_lowercase() == name.to_lowercase())
        .map_or_else(|| name.to_owned(), |b| b.folder.clone())
}

/// Comma- or space-separated tags, without `#`, deduplicated ignoring case.
pub fn parse_tags(text: &str) -> Result<Vec<String>, String> {
    let tags = tag_list(text);
    if tags.len() > MAX_TAGS {
        return Err(format!("Use up to {MAX_TAGS} tags."));
    }
    Ok(tags)
}

/// [`parse_tags`] without the limit.
fn tag_list(text: &str) -> Vec<String> {
    let mut tags: Vec<String> = Vec::new();
    for raw in text.split(|c: char| c == ',' || c.is_whitespace()) {
        let tag: String = raw
            .trim_start_matches('#')
            .chars()
            .filter(|c| !c.is_control())
            .take(TAG_MAX)
            .collect();
        if tag.is_empty() || tags.iter().any(|t| t.to_lowercase() == tag.to_lowercase()) {
            continue;
        }
        tags.push(tag);
    }
    tags
}

/// Folder names in use, sorted ignoring case.
pub fn folders(all: &[Bookmark]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for b in all {
        if !b.folder.is_empty() && !out.contains(&b.folder) {
            out.push(b.folder.clone());
        }
    }
    out.sort_by_key(|f| f.to_lowercase());
    out
}

/// Put a bookmark in a folder, creating it, or take it out with an empty name.
pub fn set_folder(all: &mut [Bookmark], id: u64, folder: &str) -> Result<(), String> {
    let folder = existing_folder(all, &clean_folder(folder));
    let at = all.iter().position(|b| b.id == id).ok_or("No such bookmark.")?;
    if all[at].folder == folder {
        return Ok(());
    }
    all[at].folder = folder;
    all[at].position = next_position(all, &all[at]);
    Ok(())
}

/// Rename a folder. Renaming onto another folder merges the two.
pub fn rename_folder(all: &mut [Bookmark], from: &str, to: &str) -> Result<(), String> {
    let to = clean_folder(to);
    if to.is_empty() {
        return Err("Name the folder.".into());
    }
    if !all.iter().any(|b| b.folder == from) {
        return Err("No such folder.".into());
    }
    let merge = all
        .iter()
        .any(|b| b.folder != from && !b.folder.is_empty() && b.folder.to_lowercase() == to.to_lowercase());
    if !merge {
        for b in all.iter_mut().filter(|b| b.folder == from) {
            b.folder.clone_from(&to);
        }
        return Ok(());
    }
    let mut moving: Vec<_> = all
        .iter()
        .filter(|b| b.folder == from)
        .map(|b| (b.position, b.added_at, b.id))
        .collect();
    moving.sort_unstable();
    for (_, _, id) in moving {
        set_folder(all, id, &to)?;
    }
    Ok(())
}

/// Remove a folder. Its bookmarks go back to their kind's group.
pub fn remove_folder(all: &mut [Bookmark], folder: &str) -> Result<(), String> {
    let mut moving: Vec<_> = all
        .iter()
        .filter(|b| b.folder == folder)
        .map(|b| (b.position, b.added_at, b.id))
        .collect();
    if folder.is_empty() || moving.is_empty() {
        return Err("No such folder.".into());
    }
    moving.sort_unstable();
    for (_, _, id) in moving {
        set_folder(all, id, "")?;
    }
    Ok(())
}

pub fn set_tags(all: &mut [Bookmark], id: u64, text: &str) -> Result<(), String> {
    let tags = parse_tags(text)?;
    all.iter_mut()
        .find(|b| b.id == id)
        .ok_or("No such bookmark.")?
        .tags = tags;
    Ok(())
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct File {
    bonsai_bookmarks: u32,
    bookmarks: Vec<Entry>,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
struct Entry {
    path: String,
    title: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    folder: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tags: Vec<String>,
}

/// The bookmarks as a file `import` reads back: paths, names, folders and tags only.
pub fn export(all: &[Bookmark]) -> String {
    let mut rows: Vec<&Bookmark> = all.iter().collect();
    rows.sort_by_key(|b| {
        (
            b.folder.to_lowercase(),
            b.kind as u8,
            b.position,
            b.added_at,
            b.id,
        )
    });
    let file = File {
        bonsai_bookmarks: FORMAT,
        bookmarks: rows
            .into_iter()
            .map(|b| Entry {
                path: b.path.clone(),
                title: b.title.clone(),
                folder: b.folder.clone(),
                tags: b.tags.clone(),
            })
            .collect(),
    };
    serde_json::to_string_pretty(&file).unwrap_or_default()
}

/// What an import did.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Imported {
    pub added: usize,
    /// Already bookmarked, or not a page this account can bookmark.
    pub skipped: usize,
}

/// Adds the bookmarks in a file `export` wrote. Existing bookmarks are kept as they are.
pub fn import(all: &mut Vec<Bookmark>, account: AccountId, text: &str) -> Result<Imported, String> {
    let file: File = serde_json::from_str(text).map_err(|_| "This isn't a Bonsai bookmarks file.")?;
    if file.bonsai_bookmarks == 0 {
        return Err("This isn't a Bonsai bookmarks file.".into());
    }
    if file.bonsai_bookmarks > FORMAT {
        return Err("This bookmarks file is from a newer Bonsai.".into());
    }
    let mut out = Imported::default();
    for e in file.bookmarks {
        let scope = if shared(&e.path) { None } else { Some(account) };
        let exists = all
            .iter()
            .any(|b| b.account == scope && identity(&b.path) == identity(&e.path));
        if exists || validate_path(&e.path).is_err() || scope.is_some_and(|a| a.is_anonymous()) {
            out.skipped += 1;
            continue;
        }
        let title = if e.title.trim().is_empty() {
            e.path.clone()
        } else {
            e.title
        };
        let mut tags = tag_list(&e.tags.join(","));
        tags.truncate(MAX_TAGS);
        add(all, scope, &e.path, &title, &e.folder, tags)?;
        out.added += 1;
    }
    Ok(out)
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

/// Move one place within the folder (or kind, outside folders), or before a target row when dragging.
pub fn reorder(all: &mut [Bookmark], id: u64, by: isize, before: Option<u64>) -> Result<(), String> {
    let this = all
        .iter()
        .find(|b| b.id == id)
        .ok_or("No such bookmark.")?
        .clone();
    let mut group: Vec<_> = all
        .iter()
        .filter(|b| this.same_group(b))
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

    /// Writes the account's visible bookmarks (shared and its own) to a file. No credentials.
    pub fn export_bookmarks(&self, account: AccountId, path: &std::path::Path) -> Result<usize, String> {
        if self.accounts_locked() {
            return Err("Unlock Bonsai to access bookmarks.".into());
        }
        let all = self.bookmarks(account);
        std::fs::write(path, export(&all)).map_err(|e| format!("Can't save {}: {e}", path.display()))?;
        Ok(all.len())
    }

    /// Adds the bookmarks from a file [`Core::export_bookmarks`] wrote.
    pub fn import_bookmarks(&self, account: AccountId, path: &std::path::Path) -> Result<Imported, String> {
        let read = |e: std::io::Error| format!("Can't read {}: {e}", path.display());
        if std::fs::metadata(path).map_err(read)?.len() > FILE_MAX {
            return Err("This file is too big to be a bookmarks file.".into());
        }
        let text = std::fs::read_to_string(path).map_err(read)?;
        self.edit_bookmarks(account, |all| import(all, account, &text))
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

    #[test]
    fn folders_create_rename_merge_and_remove() {
        let a = AccountId(1);
        let mut all = vec![];
        for path in ["/games/1", "/users/2", "/games/3", "/games/4"] {
            toggle(&mut all, a, path, "Name").unwrap();
        }
        let ids: Vec<_> = all.iter().map(|b| b.id).collect();
        set_folder(&mut all, ids[0], "  Horror\tgames ").unwrap();
        assert_eq!(all[0].folder, "Horror games");
        // Same name in another case joins the folder, and mixes kinds.
        set_folder(&mut all, ids[1], "horror GAMES").unwrap();
        assert_eq!(all[1].folder, "Horror games");
        assert_eq!(all[1].position, all[0].position + 1);
        assert_eq!(folders(&all), ["Horror games"]);
        // Reordering stays inside the folder.
        reorder(&mut all, ids[1], 0, Some(ids[0])).unwrap();
        assert!(all[1].position < all[0].position);
        assert!(reorder(&mut all, ids[1], 0, Some(ids[2])).is_err());

        set_folder(&mut all, ids[2], "Later").unwrap();
        rename_folder(&mut all, "Later", "Soon").unwrap();
        assert_eq!(folders(&all), ["Horror games", "Soon"]);
        rename_folder(&mut all, "Soon", "horror games").unwrap();
        assert_eq!(folders(&all), ["Horror games"]);
        assert!(all[2].position > all[0].position.max(all[1].position));
        assert!(rename_folder(&mut all, "Nope", "X").is_err());
        assert!(rename_folder(&mut all, "Horror games", " ").is_err());

        remove_folder(&mut all, "Horror games").unwrap();
        assert!(folders(&all).is_empty());
        assert_eq!(all.len(), 4);
        assert!(remove_folder(&mut all, "Horror games").is_err());
        assert!(set_folder(&mut all, 7, "X").is_err());
        assert_eq!(clean_folder(&"x".repeat(100)).len(), FOLDER_MAX);
    }

    #[test]
    fn tags_are_cleaned_and_capped() {
        assert_eq!(
            parse_tags("#Obby, obby  speedrun,,").unwrap(),
            ["Obby", "speedrun"]
        );
        assert_eq!(parse_tags("").unwrap(), Vec::<String>::new());
        assert!(parse_tags("a b c d e f g h i").is_err());
        let mut all = vec![];
        toggle(&mut all, AccountId(1), "/games/1", "Game").unwrap();
        let id = all[0].id;
        set_tags(&mut all, id, "fun, #later").unwrap();
        assert_eq!(all[0].tags, ["fun", "later"]);
    }

    #[test]
    fn export_and_import_round_trip_without_duplicates() {
        let a = AccountId(1);
        let mut all = vec![];
        for path in ["/games/1", "/chat?conversation=2", "/settings"] {
            toggle(&mut all, a, path, "Name").unwrap();
        }
        let ids: Vec<_> = all.iter().map(|b| b.id).collect();
        set_folder(&mut all, ids[0], "Fav").unwrap();
        set_tags(&mut all, ids[1], "work").unwrap();
        let text = export(&all);
        assert!(text.contains("\"bonsaiBookmarks\": 1"));
        assert!(!text.contains("account"));

        // Importing into the same set adds nothing.
        assert_eq!(
            import(&mut all, a, &text).unwrap(),
            Imported { added: 0, skipped: 3 }
        );

        // Another account gets the private views; the shared game already exists.
        let other = AccountId(2);
        assert_eq!(
            import(&mut all, other, &text).unwrap(),
            Imported { added: 2, skipped: 1 }
        );
        let chat = all
            .iter()
            .find(|b| b.account == Some(other) && b.path.starts_with("/chat"))
            .unwrap();
        assert_eq!(chat.tags, ["work"]);

        // Signed out: only shared pages import.
        let mut fresh = vec![];
        let got = import(&mut fresh, AccountId::ANONYMOUS, &text).unwrap();
        assert_eq!(got, Imported { added: 1, skipped: 2 });
        assert_eq!(fresh[0].folder, "Fav");

        let bad = r#"{"bonsaiBookmarks":1,"bookmarks":[{"path":"https://x.com","title":"X"},{"path":"/games/9","tags":["a","b","c","d","e","f","g","h","i"]},{"unknown":1}]}"#;
        let mut fresh = vec![];
        assert_eq!(
            import(&mut fresh, a, bad).unwrap(),
            Imported { added: 1, skipped: 2 }
        );
        assert_eq!(fresh[0].title, "/games/9");
        assert_eq!(fresh[0].tags.len(), MAX_TAGS);
        assert!(import(&mut fresh, a, "{}").is_err());
        assert!(import(&mut fresh, a, "[]").is_err());
        assert!(import(&mut fresh, a, r#"{"bonsaiBookmarks":2}"#).is_err());
    }
}

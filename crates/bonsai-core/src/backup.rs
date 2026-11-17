//! One account's data saved to a folder: `backup.json` with everything, plus CSV files for the
//! lists a spreadsheet can use. Lists load page by page through the same cached resources the
//! screens use, so a backup right after browsing them asks Roblox for little.
//!
//! Never includes the cookie or any other credential: everything here is built from views that
//! don't hold one.

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use bonsai_api::AccountId;
use serde::Serialize;

use crate::avatar::Look;
use crate::bookmarks::{Bookmark, BookmarkKind};
use crate::core::Core;
use crate::inventory::{InvItem, InvSlot};
use crate::market::ItemKind;
use crate::people::UserNote;
use crate::playlists::Playlist;
use crate::store::UserInfo;

/// Version of the `backup.json` layout.
pub const FORMAT: u32 = 1;
/// A backup stops waiting after this long and saves what it has, listing the rest as incomplete.
const DEADLINE: Duration = Duration::from_secs(600);
/// How long to wait for names before saving ids without them.
const NAMES_WAIT: Duration = Duration::from_secs(30);
/// Most items kept per inventory type, and most outfits.
const MAX_LIST: usize = 10_000;
/// Most outfits whose items are saved; each one is its own request.
const MAX_OUTFIT_LOOKS: usize = 500;

/// One backup at a time, so two don't compete for the same rate limit.
static RUNNING: AtomicBool = AtomicBool::new(false);

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Backup {
    pub bonsai_backup: u32,
    /// UTC, ISO 8601.
    pub exported_at: String,
    pub account: Person,
    /// Lists that didn't load in full, with why. Empty when everything did.
    pub incomplete: Vec<String>,
    pub friends: Vec<Person>,
    /// What the account wears now.
    pub avatar: Option<Look>,
    pub outfits: Vec<SavedOutfit>,
    pub inventory: Vec<Owned>,
    pub playlists: Vec<Playlist>,
    pub bookmarks: Vec<SavedBookmark>,
    /// Nicknames and notes this account keeps about people, friends or not.
    pub notes: Vec<Person>,
}

/// A user, with what you wrote about them.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub id: u64,
    pub name: String,
    pub display_name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub nickname: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub note: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedOutfit {
    pub id: u64,
    pub name: String,
    /// False for outfits that came with a bundle.
    pub editable: bool,
    /// What it's made of; `None` when that didn't load.
    pub look: Option<Look>,
}

/// One owned copy of an item.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Owned {
    /// `asset` or `bundle`.
    pub kind: &'static str,
    pub id: u64,
    pub name: String,
    /// The inventory list it came from, as the site names it ("Hats").
    pub category: String,
    /// The copy's id; 0 for bundles.
    pub copy_id: u64,
    pub serial: Option<i64>,
    pub acquired: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedBookmark {
    pub path: String,
    pub title: String,
    pub kind: BookmarkKind,
}

/// The lists as they stand after one look at the caches.
#[derive(Default)]
struct Round {
    backup: Backup,
    /// Still loading or with more pages to fetch.
    pending: Vec<String>,
    /// Users whose names aren't known yet.
    unnamed: Vec<u64>,
}

impl Core {
    /// Saves `account`'s backup into a new folder inside `parent`. Loads what isn't cached on a
    /// background task and says when it's done.
    pub fn export_account(&self, account: AccountId, parent: PathBuf) -> Result<(), String> {
        if self.accounts_locked() {
            return Err("Unlock Bonsai to back up an account.".into());
        }
        if account.is_anonymous() {
            return Err("Add an account first.".into());
        }
        let name = self
            .store()
            .accounts
            .get(account)
            .map(|a| a.name.clone())
            .unwrap_or_else(|| account.0.to_string());
        if RUNNING.swap(true, Ordering::SeqCst) {
            return Err("A backup is already running.".into());
        }
        self.notice(
            format!("Backing up {name}. Large inventories take a few minutes."),
            false,
        );
        let core = self.handle();
        self.inner.rt.spawn(async move {
            let backup = core.collect_backup(account).await;
            let saved = tokio::task::spawn_blocking(move || write(&parent, &name, &backup))
                .await
                .map_err(|e| e.to_string())
                .and_then(|r| r);
            RUNNING.store(false, Ordering::SeqCst);
            match saved {
                Ok((dir, missing)) if missing.is_empty() => {
                    core.notice(format!("Saved backup to {}", dir.display()), false)
                }
                Ok((dir, missing)) => core.notice(
                    format!(
                        "Saved backup to {}. Not complete: {}.",
                        dir.display(),
                        missing.join(", ")
                    ),
                    true,
                ),
                Err(e) => core.notice(e, true),
            }
        });
        Ok(())
    }

    async fn collect_backup(&self, account: AccountId) -> Backup {
        let start = Instant::now();
        // Ids already asked for, so people who show up in a later round still get names.
        let mut asked: HashSet<u64> = HashSet::new();
        loop {
            let changed = self.store_changed();
            tokio::pin!(changed);
            changed.as_mut().enable();
            let mut round = self.backup_round(account);
            let waiting_names = !round.unnamed.is_empty() && start.elapsed() < NAMES_WAIT;
            let new: Vec<u64> = round
                .unnamed
                .iter()
                .copied()
                .filter(|id| asked.insert(*id))
                .collect();
            if !new.is_empty() {
                self.want_users(&new);
            }
            if (round.pending.is_empty() && !waiting_names) || start.elapsed() >= DEADLINE {
                round
                    .backup
                    .incomplete
                    .extend(round.pending.into_iter().map(|p| format!("{p} (took too long)")));
                return round.backup;
            }
            let _ = tokio::time::timeout(Duration::from_secs(1), changed).await;
        }
    }

    /// Reads every list from the caches, asking for the next page of any that has one.
    fn backup_round(&self, account: AccountId) -> Round {
        let mut r = Round::default();
        let b = &mut r.backup;
        let me = account.0;
        b.bonsai_backup = FORMAT;
        b.exported_at = crate::fmt::iso_utc(crate::cache::now());

        let notes = self.user_notes(account);
        let mut people: Vec<u64> = vec![me];

        // Friends come with the account's regular refresh.
        let friends = self
            .store()
            .account_data(account)
            .map(|d| d.friends.clone())
            .unwrap_or_default();
        match (&friends.data, &friends.error) {
            (Some(ids), _) => people.extend(ids),
            (None, Some(e)) => b.incomplete.push(format!("friends: {e}")),
            (None, None) => r.pending.push("friends".into()),
        }

        let avatar = self.avatar(account);
        b.avatar = avatar.data.as_deref().cloned();
        if b.avatar.is_none() {
            match avatar.error {
                Some(e) => b.incomplete.push(format!("avatar: {e}")),
                None => r.pending.push("avatar".into()),
            }
        }

        let outfits = self.outfits(account, true);
        let list = outfits
            .data
            .as_deref()
            .map(|p| p.items.as_slice())
            .unwrap_or_default();
        list_state(&mut r, "outfits", &outfits, list.len());
        for (i, o) in list.iter().take(MAX_LIST).enumerate() {
            let look = if i < MAX_OUTFIT_LOOKS {
                let res = self.outfit(o.id);
                if res.data.is_none() && res.error.is_none() {
                    r.pending.push(format!("outfit {}", o.name));
                }
                res.data.as_deref().cloned()
            } else {
                None
            };
            r.backup.outfits.push(SavedOutfit {
                id: o.id,
                name: o.name.clone(),
                editable: o.editable,
                look,
            });
        }

        let categories = self.inventory_categories(me);
        match categories.data.as_deref() {
            Some(cats) => {
                let mut seen = HashSet::new();
                for (label, slot) in cats.iter().flat_map(|c| &c.slots) {
                    if !matches!(slot, InvSlot::Asset(_) | InvSlot::Bundle(_)) || !seen.insert(*slot) {
                        continue;
                    }
                    let res = self.inventory(account, me, *slot, false, true);
                    let items = res
                        .data
                        .as_deref()
                        .map(|p| p.items.as_slice())
                        .unwrap_or_default();
                    list_state(&mut r, &format!("inventory: {label}"), &res, items.len());
                    r.backup
                        .inventory
                        .extend(items.iter().take(MAX_LIST).map(|i| owned(i, label)));
                }
            }
            None => match categories.error {
                Some(e) => r.backup.incomplete.push(format!("inventory: {e}")),
                None => r.pending.push("inventory".into()),
            },
        }

        let b = &mut r.backup;
        b.playlists = self.playlists(account);
        b.bookmarks = saved_bookmarks(self.bookmarks(account));

        people.extend(notes.keys());
        let store = self.store();
        let person = |id: u64| {
            let u = store.users.get(&id).cloned().unwrap_or(UserInfo {
                id,
                ..Default::default()
            });
            let n = notes.get(&id).cloned().unwrap_or_default();
            Person {
                id,
                name: u.name,
                display_name: u.display_name,
                nickname: n.nickname,
                note: n.note,
            }
        };
        let meta = store.accounts.get(account);
        b.account = Person {
            name: meta.map(|m| m.name.clone()).unwrap_or_default(),
            display_name: meta.map(|m| m.display_name.clone()).unwrap_or_default(),
            ..person(me)
        };
        if let Some(ids) = &friends.data {
            b.friends = ids.iter().map(|&id| person(id)).collect();
        }
        b.notes = noted_ids(&notes).into_iter().map(person).collect();
        r.unnamed = people
            .into_iter()
            .filter(|id| *id != me && store.users.get(id).is_none_or(|u| u.name.is_empty()))
            .collect();
        r
    }
}

/// Marks a paged list pending while it loads or has pages left under the cap, and incomplete
/// when it failed or hit the cap.
fn list_state<T>(r: &mut Round, what: &str, res: &crate::res::Res<crate::res::Page<T>>, len: usize) {
    let more = res.data.as_ref().is_some_and(|p| p.next.is_some());
    if let Some(e) = &res.error {
        r.backup.incomplete.push(format!("{what}: {e}"));
    } else if res.loading || res.data.is_none() || (more && len < MAX_LIST) {
        r.pending.push(what.to_owned());
    } else if more {
        r.backup.incomplete.push(format!("{what} (first {MAX_LIST})"));
    }
}

fn owned(i: &InvItem, category: &str) -> Owned {
    Owned {
        kind: match i.item.kind {
            ItemKind::Asset => "asset",
            ItemKind::Bundle => "bundle",
        },
        id: i.item.id,
        name: i.name.clone(),
        category: category.to_owned(),
        copy_id: i.copy_id,
        serial: i.serial,
        acquired: i.acquired.clone(),
    }
}

fn saved_bookmarks(all: Vec<Bookmark>) -> Vec<SavedBookmark> {
    all.into_iter()
        .map(|b| SavedBookmark {
            path: b.path,
            title: b.title,
            kind: b.kind,
        })
        .collect()
}

/// Users with a nickname or note, by id.
fn noted_ids(notes: &std::collections::HashMap<u64, UserNote>) -> Vec<u64> {
    let mut ids: Vec<u64> = notes
        .iter()
        .filter(|(_, n)| !n.is_empty())
        .map(|(id, _)| *id)
        .collect();
    ids.sort_unstable();
    ids
}

fn csv_field(s: &str) -> String {
    // A leading = + - @ makes spreadsheets run the cell as a formula.
    let s = if s.starts_with(['=', '+', '-', '@']) {
        format!("'{s}")
    } else {
        s.to_owned()
    };
    if s.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s
    }
}

fn csv_rows<'a>(header: &str, rows: impl IntoIterator<Item = Vec<String>> + 'a) -> String {
    let mut out = format!("{header}\n");
    for row in rows {
        out.push_str(&row.join(","));
        out.push('\n');
    }
    out
}

fn people_csv(people: &[Person]) -> String {
    csv_rows(
        "id,name,display_name,nickname,note",
        people.iter().map(|p| {
            vec![
                p.id.to_string(),
                csv_field(&p.name),
                csv_field(&p.display_name),
                csv_field(&p.nickname),
                csv_field(&p.note),
            ]
        }),
    )
}

/// The files a backup is saved as, by name.
pub fn files(b: &Backup) -> Vec<(&'static str, String)> {
    let json = serde_json::to_string_pretty(b).unwrap_or_default();
    let inventory = csv_rows(
        "kind,id,name,category,copy_id,serial,acquired",
        b.inventory.iter().map(|i| {
            vec![
                i.kind.to_owned(),
                i.id.to_string(),
                csv_field(&i.name),
                csv_field(&i.category),
                i.copy_id.to_string(),
                i.serial.map(|s| s.to_string()).unwrap_or_default(),
                csv_field(&i.acquired),
            ]
        }),
    );
    let outfits = csv_rows(
        "id,name,editable,item_ids",
        b.outfits.iter().map(|o| {
            let items = o
                .look
                .as_ref()
                .map(|l| {
                    l.assets
                        .iter()
                        .map(|a| a.id.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                })
                .unwrap_or_default();
            vec![
                o.id.to_string(),
                csv_field(&o.name),
                o.editable.to_string(),
                items,
            ]
        }),
    );
    vec![
        ("backup.json", json),
        ("friends.csv", people_csv(&b.friends)),
        ("notes.csv", people_csv(&b.notes)),
        ("inventory.csv", inventory),
        ("outfits.csv", outfits),
    ]
}

/// A folder name that's safe on every system.
fn folder_name(account_name: &str, exported_at: &str) -> String {
    let name: String = account_name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect();
    let day = exported_at.get(..10).unwrap_or(exported_at);
    format!("bonsai-backup-{name}-{day}")
}

/// Writes the backup into a new folder in `parent`. Returns the folder and what was incomplete.
fn write(parent: &Path, account_name: &str, b: &Backup) -> Result<(PathBuf, Vec<String>), String> {
    let base = folder_name(account_name, &b.exported_at);
    let mut dir = parent.join(&base);
    let mut n = 2;
    while dir.exists() {
        dir = parent.join(format!("{base}-{n}"));
        n += 1;
    }
    let fail = |e: std::io::Error| format!("Can't save to {}: {e}", dir.display());
    std::fs::create_dir_all(&dir).map_err(fail)?;
    for (file, text) in files(b) {
        std::fs::write(dir.join(file), text).map_err(fail)?;
    }
    let mut missing: Vec<String> = Vec::new();
    for what in &b.incomplete {
        let what = what.split([':', '(']).next().unwrap_or(what).trim().to_owned();
        if !missing.contains(&what) {
            missing.push(what);
        }
    }
    Ok((dir, missing))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Backup {
        Backup {
            bonsai_backup: FORMAT,
            exported_at: "2026-11-16T12:00:00Z".into(),
            account: Person {
                id: 1,
                name: "me".into(),
                ..Default::default()
            },
            friends: vec![Person {
                id: 2,
                name: "pal".into(),
                display_name: "Pal, \"the\" best".into(),
                nickname: "=cmd".into(),
                ..Default::default()
            }],
            inventory: vec![Owned {
                kind: "asset",
                id: 10,
                name: "Hat".into(),
                category: "Hats".into(),
                copy_id: 99,
                serial: Some(7),
                acquired: "2020-01-01".into(),
            }],
            ..Default::default()
        }
    }

    #[test]
    fn writes_json_and_csv_without_credentials() {
        let files = files(&sample());
        let names: Vec<_> = files.iter().map(|(n, _)| *n).collect();
        assert_eq!(
            names,
            [
                "backup.json",
                "friends.csv",
                "notes.csv",
                "inventory.csv",
                "outfits.csv"
            ]
        );
        let json = &files[0].1;
        assert!(json.contains("\"bonsaiBackup\": 1"));
        assert!(!json.to_lowercase().contains("cookie"));
        assert!(!json.contains("\"note\""), "empty notes are left out");
        assert_eq!(
            files[1].1,
            "id,name,display_name,nickname,note\n2,pal,\"Pal, \"\"the\"\" best\",'=cmd,\n"
        );
        assert_eq!(
            files[3].1,
            "kind,id,name,category,copy_id,serial,acquired\nasset,10,Hat,Hats,99,7,2020-01-01\n"
        );
    }

    #[test]
    fn folder_names_are_safe_and_dated() {
        assert_eq!(
            folder_name("a b/c", "2026-11-16T12:00:00Z"),
            "bonsai-backup-a-b-c-2026-11-16"
        );
    }

    #[test]
    fn writes_a_new_folder_each_time() {
        let parent = tempfile::tempdir().unwrap();
        let mut b = sample();
        let (first, missing) = write(parent.path(), "me", &b).unwrap();
        assert!(missing.is_empty());
        assert!(first.join("backup.json").exists());
        b.incomplete.push("inventory: Hats (first 10000)".into());
        b.incomplete.push("inventory: Shirts: Roblox is busy".into());
        let (second, missing) = write(parent.path(), "me", &b).unwrap();
        assert_ne!(first, second);
        assert_eq!(missing, ["inventory"]);
    }
}

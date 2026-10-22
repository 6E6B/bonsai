//! SQLite entity cache with fetched-at stamps (stale-while-revalidate).
//!
//! Shared public data (users, universes, items, groups) is stored under `AccountId::ANONYMOUS`
//! and shared by every account. Private data (inventory, messages, trades, friends) is keyed by
//! the account that fetched it and never read by another.

use crate::vault::Vault;
use std::path::Path;
use std::sync::Arc;
use zeroize::Zeroizing;

use bonsai_api::AccountId;
use parking_lot::{Mutex, RwLock};
use rusqlite::{Connection, OptionalExtension, params};
use serde::Serialize;
use serde::de::DeserializeOwned;

/// What a cached row is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Kind {
    // shared
    User,
    Universe,
    Place,
    Item,
    Group,
    ThumbUrl,
    Servers,
    Search,
    // private
    Me,
    Friends,
    Robux,
    Home,
    Unread,
    Favorites,
    Recent,
    /// Items an account wants (local; Roblox has no wishlist).
    Wishlist,
    /// Small local per-account state Roblox doesn't keep (pinned chats, seen shouts).
    Local,
    /// Generic resources (see `res`); the account is part of the key.
    Res,
}

impl Kind {
    fn name(self) -> &'static str {
        match self {
            Kind::User => "user",
            Kind::Universe => "universe",
            Kind::Place => "place",
            Kind::Item => "item",
            Kind::Group => "group",
            Kind::ThumbUrl => "thumb",
            Kind::Servers => "servers",
            Kind::Search => "search",
            Kind::Me => "me",
            Kind::Friends => "friends",
            Kind::Robux => "robux",
            Kind::Home => "home",
            Kind::Unread => "unread",
            Kind::Favorites => "favorites",
            Kind::Recent => "recent",
            Kind::Wishlist => "wishlist",
            Kind::Local => "local",
            Kind::Res => "res",
        }
    }
    pub fn is_shared(self) -> bool {
        matches!(
            self,
            Kind::User
                | Kind::Universe
                | Kind::Place
                | Kind::Item
                | Kind::Group
                | Kind::ThumbUrl
                | Kind::Servers
                | Kind::Search
        )
    }
}

/// Rows Roblox doesn't keep for us. Clearing or pruning the cache never touches them.
const LOCAL_KINDS: [Kind; 3] = [Kind::Local, Kind::Wishlist, Kind::Recent];

/// A cached value and how old it is.
#[derive(Clone, Debug)]
pub struct Cached<T> {
    pub value: T,
    pub fetched_at: i64,
}

impl<T> Cached<T> {
    pub fn age_secs(&self) -> i64 {
        now() - self.fetched_at
    }
    pub fn is_fresh(&self, max_age_secs: i64) -> bool {
        self.age_secs() <= max_age_secs
    }
}

pub fn now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

pub struct Cache {
    db: Mutex<Connection>,
    vault: RwLock<Option<Arc<Vault>>>,
}

impl Cache {
    pub fn attach_vault(&self, vault: Arc<Vault>) {
        *self.vault.write() = Some(vault);
    }
    fn encode_private(&self, aad: &[u8], json: &str) -> Option<String> {
        match &*self.vault.read() {
            Some(v) if v.blocked() => None,
            Some(v) if v.protected() => {
                if v.blocked() {
                    return None;
                }
                v.encrypt_cache(aad, json.as_bytes()).ok()
            }
            _ => Some(json.to_owned()),
        }
    }
    fn decode_private(&self, aad: &[u8], json: &str) -> Option<Zeroizing<Vec<u8>>> {
        match &*self.vault.read() {
            Some(v) if v.blocked() => None,
            Some(v) if v.protected() => {
                if v.blocked() {
                    return None;
                }
                v.decrypt_cache(aad, json).ok()
            }
            _ if json.starts_with("vault1:") => None,
            _ => Some(Zeroizing::new(json.as_bytes().to_vec())),
        }
    }
    fn encode(&self, kind: Kind, account: AccountId, id: &str, json: &str) -> Option<String> {
        if kind.is_shared() {
            Some(json.to_owned())
        } else {
            self.encode_private(
                &entity_aad(kind.name(), Self::account_for(kind, account), id),
                json,
            )
        }
    }
    fn decode(&self, kind: Kind, account: AccountId, id: &str, json: &str) -> Option<Zeroizing<Vec<u8>>> {
        if kind.is_shared() {
            Some(Zeroizing::new(json.as_bytes().to_vec()))
        } else {
            self.decode_private(
                &entity_aad(kind.name(), Self::account_for(kind, account), id),
                json,
            )
        }
    }
    /// Called with account admission closed. The SQL transaction is restartable; all payloads
    /// are authenticated against their complete key. VACUUM/checkpoint remove old logical
    /// plaintext pages, but cannot erase snapshots, filesystem history or SSD remnants.
    pub fn migrate_vault(&self, vault: &Vault, encrypt: bool) -> Result<(), String> {
        let mut db = self.db.lock();
        db.execute_batch("PRAGMA secure_delete = ON;")
            .map_err(|_| "Cannot prepare cache migration.")?;
        let tx = db.transaction().map_err(|_| "Cannot start cache migration.")?;
        let shared = [
            "user", "universe", "place", "item", "group", "thumb", "servers", "search",
        ];
        {
            let mut query = tx
                .prepare("SELECT kind, account, id, json FROM entities")
                .map_err(|_| "Cannot read cache migration.")?;
            let rows = query
                .query_map([], |r| {
                    Ok((
                        r.get::<_, String>(0)?,
                        r.get::<_, i64>(1)?,
                        r.get::<_, String>(2)?,
                        r.get::<_, String>(3)?,
                    ))
                })
                .map_err(|_| "Cannot read cache migration.")?;
            for row in rows {
                let (kind, account, id, raw) = row.map_err(|_| "Cannot read cache row.")?;
                let raw = Zeroizing::new(raw);
                if shared.contains(&kind.as_str()) {
                    continue;
                }
                let aad = entity_aad(&kind, account, &id);
                let value = migrate_payload(vault, &aad, &raw, encrypt)?;
                tx.execute(
                    "UPDATE entities SET json = ?1 WHERE kind = ?2 AND account = ?3 AND id = ?4",
                    params![value.as_str(), kind, account, id],
                )
                .map_err(|_| "Cannot migrate private cache row.")?;
            }
        }
        {
            let mut query = tx
                .prepare("SELECT account, scope, kind, id, at, json FROM ledger")
                .map_err(|_| "Cannot read ledger migration.")?;
            let rows = query
                .query_map([], |r| {
                    Ok((
                        r.get::<_, i64>(0)?,
                        r.get::<_, String>(1)?,
                        r.get::<_, String>(2)?,
                        r.get::<_, String>(3)?,
                        r.get::<_, i64>(4)?,
                        r.get::<_, String>(5)?,
                    ))
                })
                .map_err(|_| "Cannot read ledger migration.")?;
            for row in rows {
                let (account, scope, kind, id, at, raw) = row.map_err(|_| "Cannot read ledger row.")?;
                let value = migrate_payload(
                    vault,
                    &ledger_aad(account, &scope, &kind, &id, at),
                    &Zeroizing::new(raw),
                    encrypt,
                )?;
                tx.execute(
                    "UPDATE ledger SET json = ?1 WHERE account = ?2 AND scope = ?3 AND kind = ?4 AND id = ?5",
                    params![value.as_str(), account, scope, kind, id],
                )
                .map_err(|_| "Cannot migrate private ledger row.")?;
            }
        }
        tx.commit()
            .map_err(|_| "Cannot commit private cache migration.")?;
        let (busy, _, _): (i32, i32, i32) = db
            .query_row("PRAGMA wal_checkpoint(TRUNCATE)", [], |r| {
                Ok((r.get(0)?, r.get(1)?, r.get(2)?))
            })
            .map_err(|_| "Cannot checkpoint private cache.")?;
        if busy != 0 {
            return Err("Private cache is busy. Close other readers and try again.".into());
        }
        db.execute_batch("VACUUM;")
            .map_err(|_| "Cannot compact private cache migration.")?;
        let (busy, _, _): (i32, i32, i32) = db
            .query_row("PRAGMA wal_checkpoint(TRUNCATE)", [], |r| {
                Ok((r.get(0)?, r.get(1)?, r.get(2)?))
            })
            .map_err(|_| "Cannot checkpoint private cache.")?;
        if busy != 0 {
            return Err("Private cache checkpoint is busy.".into());
        }
        Ok(())
    }
    /// Forgotten-PIN recovery removes private rows without decrypting them.
    pub fn forget_private(&self) -> Result<(), String> {
        let mut db = self.db.lock();
        db.execute_batch("PRAGMA synchronous = FULL; PRAGMA secure_delete = ON;")
            .map_err(|_| "Cannot prepare account data reset.")?;
        let tx = db.transaction().map_err(|_| "Cannot start account data reset.")?;
        tx.execute("DELETE FROM entities WHERE kind NOT IN ('user','universe','place','item','group','thumb','servers','search')", [])
            .map_err(|_| "Cannot clear private account data.")?;
        tx.execute("DELETE FROM ledger", [])
            .map_err(|_| "Cannot clear private transaction data.")?;
        tx.commit().map_err(|_| "Cannot commit account data reset.")?;
        db.execute_batch("VACUUM;")
            .map_err(|_| "Cannot compact account data reset.")?;
        let (busy, _, _): (i32, i32, i32) = db
            .query_row("PRAGMA wal_checkpoint(TRUNCATE)", [], |r| {
                Ok((r.get(0)?, r.get(1)?, r.get(2)?))
            })
            .map_err(|_| "Cannot checkpoint account data reset.")?;
        if busy != 0 {
            return Err("Account data is busy. Restart Bonsai and try again.".into());
        }
        Ok(())
    }
    pub fn open(path: &Path) -> rusqlite::Result<Cache> {
        if let Some(d) = path.parent() {
            let _ = std::fs::create_dir_all(d);
        }
        let db = Connection::open(path)?;
        Self::init(db)
    }

    pub fn in_memory() -> rusqlite::Result<Cache> {
        Self::init(Connection::open_in_memory()?)
    }

    fn init(db: Connection) -> rusqlite::Result<Cache> {
        // Takes effect on a new database only; `prune` converts an old one.
        db.execute_batch(
            "PRAGMA auto_vacuum = INCREMENTAL;
             PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             CREATE TABLE IF NOT EXISTS entities (
                 kind TEXT NOT NULL,
                 account INTEGER NOT NULL,
                 id TEXT NOT NULL,
                 json TEXT NOT NULL,
                 fetched_at INTEGER NOT NULL,
                 PRIMARY KEY (kind, account, id)
             );
             CREATE TABLE IF NOT EXISTS ledger (
                 account INTEGER NOT NULL,
                 scope TEXT NOT NULL,
                 kind TEXT NOT NULL,
                 id TEXT NOT NULL,
                 at INTEGER NOT NULL,
                 json TEXT NOT NULL,
                 PRIMARY KEY (account, scope, kind, id)
             );",
        )?;
        Ok(Cache {
            db: Mutex::new(db),
            vault: RwLock::new(None),
        })
    }

    /// The local time zone's offset from UTC in seconds, as the C library sees it right now.
    pub fn local_offset(&self) -> Option<i64> {
        self.db
            .lock()
            .query_row(
                "SELECT CAST(strftime('%s', 'now', 'localtime') AS INTEGER) - CAST(strftime('%s', 'now') AS INTEGER)",
                [],
                |r| r.get::<_, i64>(0),
            )
            .ok()
    }

    fn account_for(kind: Kind, account: AccountId) -> i64 {
        if kind.is_shared() { 0 } else { account.0 as i64 }
    }

    pub fn get<T: DeserializeOwned>(&self, kind: Kind, account: AccountId, id: &str) -> Option<Cached<T>> {
        let db = self.db.lock();
        let row: Option<(String, i64)> = db
            .query_row(
                "SELECT json, fetched_at FROM entities WHERE kind = ?1 AND account = ?2 AND id = ?3",
                params![kind.name(), Self::account_for(kind, account), id],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .optional()
            .ok()
            .flatten();
        let (json, fetched_at) = row?;
        serde_json::from_slice(&self.decode(kind, account, id, &json)?)
            .ok()
            .map(|value| Cached { value, fetched_at })
    }

    pub fn put<T: Serialize>(&self, kind: Kind, account: AccountId, id: &str, value: &T) {
        let Ok(json) = serde_json::to_string(value).map(Zeroizing::new) else {
            return;
        };
        // Hold the database lock across the admission check and write. Otherwise a
        // plaintext payload encoded just before setup could arrive after cache encryption.
        let db = self.db.lock();
        let Some(json) = self.encode(kind, account, id, &json) else {
            return;
        };
        let r = db.execute(
            "INSERT INTO entities (kind, account, id, json, fetched_at) VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT (kind, account, id) DO UPDATE SET json = excluded.json, fetched_at = excluded.fetched_at",
            params![kind.name(), Self::account_for(kind, account), id, json, now()],
        );
        if let Err(e) = r {
            tracing::warn!("cache write failed: {e}");
        }
    }

    pub fn delete(&self, kind: Kind, account: AccountId, id: &str) {
        let _ = self.db.lock().execute(
            "DELETE FROM entities WHERE kind = ?1 AND account = ?2 AND id = ?3",
            params![kind.name(), Self::account_for(kind, account), id],
        );
    }

    /// Every row of a kind for an account (for warming the in-memory store at start).
    pub fn all<T: DeserializeOwned>(&self, kind: Kind, account: AccountId) -> Vec<(String, Cached<T>)> {
        let db = self.db.lock();
        let Ok(mut st) =
            db.prepare("SELECT id, json, fetched_at FROM entities WHERE kind = ?1 AND account = ?2")
        else {
            return Vec::new();
        };
        let rows = st.query_map(params![kind.name(), Self::account_for(kind, account)], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, i64>(2)?,
            ))
        });
        let Ok(rows) = rows else { return Vec::new() };
        rows.flatten()
            .filter_map(|(id, json, t)| {
                serde_json::from_slice(&self.decode(kind, account, &id, &json)?)
                    .ok()
                    .map(|v| {
                        (
                            id,
                            Cached {
                                value: v,
                                fetched_at: t,
                            },
                        )
                    })
            })
            .collect()
    }

    /// Add or replace transaction rows (see `transactions`). Always private to `account`.
    pub fn ledger_put<'a, T: Serialize + 'a>(
        &self,
        account: AccountId,
        scope: &str,
        rows: impl IntoIterator<Item = (&'a str, &'a str, i64, &'a T)>,
    ) {
        let mut db = self.db.lock();
        let r = (|| -> rusqlite::Result<()> {
            let tx = db.transaction()?;
            {
                let mut st = tx.prepare(
                    "INSERT INTO ledger (account, scope, kind, id, at, json) VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                     ON CONFLICT (account, scope, kind, id) DO UPDATE SET at = excluded.at, json = excluded.json",
                )?;
                for (kind, id, at, v) in rows {
                    let Ok(json) = serde_json::to_string(v).map(Zeroizing::new) else {
                        continue;
                    };
                    let aad = ledger_aad(account.0 as i64, scope, kind, id, at);
                    let Some(json) = self.encode_private(&aad, &json) else {
                        continue;
                    };
                    st.execute(params![account.0 as i64, scope, kind, id, at, json])?;
                }
            }
            tx.commit()
        })();
        if let Err(e) = r {
            tracing::warn!("ledger write failed: {e}");
        }
    }

    /// Every transaction row of one scope, newest first.
    pub fn ledger_all<T: DeserializeOwned>(&self, account: AccountId, scope: &str) -> Vec<T> {
        let db = self.db.lock();
        let Ok(mut st) = db.prepare(
            "SELECT kind, id, at, json FROM ledger WHERE account = ?1 AND scope = ?2 ORDER BY at DESC",
        ) else {
            return Vec::new();
        };
        let rows = st.query_map(params![account.0 as i64, scope], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, i64>(2)?,
                r.get::<_, String>(3)?,
            ))
        });
        let Ok(rows) = rows else { return Vec::new() };
        rows.flatten()
            .filter_map(|(kind, id, at, json)| {
                serde_json::from_slice(
                    &self.decode_private(&ledger_aad(account.0 as i64, scope, &kind, &id, at), &json)?,
                )
                .ok()
            })
            .collect()
    }

    /// Bytes the database takes on disk.
    pub fn size_bytes(&self) -> u64 {
        self.db
            .lock()
            .query_row(
                "SELECT page_count * page_size FROM pragma_page_count(), pragma_page_size()",
                [],
                |r| r.get::<_, i64>(0),
            )
            .map_or(0, |n| n.max(0) as u64)
    }

    /// Delete everything fetched from Roblox. Local state Roblox doesn't keep (drafts, pinned
    /// chats, wishlist, recent games) stays.
    pub fn clear_fetched(&self) {
        let keep = LOCAL_KINDS.map(Kind::name);
        let db = self.db.lock();
        let r = db.execute(
            "DELETE FROM entities WHERE kind NOT IN (?1, ?2, ?3)",
            params![keep[0], keep[1], keep[2]],
        );
        if let Err(e) = r {
            tracing::warn!("cache clear failed: {e}");
        }
        let _ = db.execute("DELETE FROM ledger", []);
        let _ = db.execute_batch("PRAGMA wal_checkpoint(TRUNCATE); VACUUM;");
    }

    /// Delete fetched rows last written before `cutoff` (unix secs) and give the space back.
    /// Local state and transaction history stay. Returns how many rows went.
    pub fn prune(&self, cutoff: i64) -> usize {
        let keep = LOCAL_KINDS.map(Kind::name);
        let db = self.db.lock();
        let n = db
            .execute(
                "DELETE FROM entities WHERE fetched_at < ?1 AND kind NOT IN (?2, ?3, ?4)",
                params![cutoff, keep[0], keep[1], keep[2]],
            )
            .unwrap_or_else(|e| {
                tracing::warn!("cache prune failed: {e}");
                0
            });
        let mode = db
            .query_row("PRAGMA auto_vacuum", [], |r| r.get::<_, i64>(0))
            .unwrap_or(0);
        // 2 = incremental. An older database needs one full VACUUM to switch.
        let r = if mode == 2 {
            db.execute_batch("PRAGMA incremental_vacuum;")
        } else {
            db.execute_batch("PRAGMA auto_vacuum = INCREMENTAL; VACUUM;")
        };
        if let Err(e) = r {
            tracing::warn!("cache vacuum failed: {e}");
        }
        let _ = db.execute_batch("PRAGMA wal_checkpoint(TRUNCATE);");
        n
    }

    /// Drop everything private to an account (sign out, remove).
    pub fn forget_account(&self, account: AccountId) {
        if account.is_anonymous() {
            return;
        }
        let db = self.db.lock();
        let _ = db.execute(
            "DELETE FROM entities WHERE account = ?1",
            params![account.0 as i64],
        );
        let _ = db.execute("DELETE FROM ledger WHERE account = ?1", params![account.0 as i64]);
    }
}

fn entity_aad(kind: &str, account: i64, id: &str) -> Vec<u8> {
    // Length-delimited JSON tuples avoid ambiguous concatenated keys.
    serde_json::to_vec(&("entities", kind, account, id)).unwrap_or_default()
}
fn ledger_aad(account: i64, scope: &str, kind: &str, id: &str, at: i64) -> Vec<u8> {
    serde_json::to_vec(&("ledger", account, scope, kind, id, at)).unwrap_or_default()
}
fn migrate_payload(vault: &Vault, aad: &[u8], raw: &str, encrypt: bool) -> Result<Zeroizing<String>, String> {
    if encrypt {
        if raw.starts_with("vault1:") {
            vault.decrypt_cache(aad, raw)?;
            Ok(Zeroizing::new(raw.to_owned()))
        } else {
            vault.encrypt_cache(aad, raw.as_bytes()).map(Zeroizing::new)
        }
    } else {
        let bytes = vault.decrypt_cache(aad, raw)?;
        String::from_utf8(bytes.to_vec())
            .map(Zeroizing::new)
            .map_err(|_| "Invalid private cache text.".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_rows_are_shared_private_rows_are_not() {
        let c = Cache::in_memory().unwrap();
        c.put(Kind::User, AccountId(1), "156", &"builderman");
        assert_eq!(
            c.get::<String>(Kind::User, AccountId(2), "156").unwrap().value,
            "builderman"
        );
        c.put(Kind::Friends, AccountId(1), "list", &vec![1u64, 2]);
        assert!(c.get::<Vec<u64>>(Kind::Friends, AccountId(2), "list").is_none());
        assert_eq!(
            c.get::<Vec<u64>>(Kind::Friends, AccountId(1), "list")
                .unwrap()
                .value,
            vec![1, 2]
        );
        c.forget_account(AccountId(1));
        assert!(c.get::<Vec<u64>>(Kind::Friends, AccountId(1), "list").is_none());
        assert!(c.get::<String>(Kind::User, AccountId(1), "156").is_some());
        let x = c.get::<String>(Kind::User, AccountId(1), "156").unwrap();
        assert!(x.is_fresh(60));
    }

    #[test]
    fn prune_drops_old_fetched_rows_and_keeps_local_state() {
        let c = Cache::in_memory().unwrap();
        c.put(Kind::User, AccountId(1), "1", &"old");
        c.put(Kind::Res, AccountId(1), "feed", &"old");
        c.put(Kind::Wishlist, AccountId(1), "wishlist", &"mine");
        c.put(Kind::Local, AccountId(1), "pins", &"mine");
        c.ledger_put(AccountId(1), "me", [("Sale", "x", 10, &"kept".to_string())]);
        // Everything above was written "now"; prune as if a minute later.
        assert_eq!(c.prune(now() + 60), 2);
        c.put(Kind::User, AccountId(1), "2", &"new");
        assert_eq!(c.prune(now() - 60), 0);
        assert!(c.get::<String>(Kind::User, AccountId(1), "1").is_none());
        assert!(c.get::<String>(Kind::Res, AccountId(1), "feed").is_none());
        assert!(c.get::<String>(Kind::User, AccountId(1), "2").is_some());
        assert!(
            c.get::<String>(Kind::Wishlist, AccountId(1), "wishlist")
                .is_some()
        );
        assert!(c.get::<String>(Kind::Local, AccountId(1), "pins").is_some());
        assert_eq!(c.ledger_all::<String>(AccountId(1), "me"), vec!["kept"]);
        c.delete(Kind::User, AccountId(9), "2");
        assert!(c.get::<String>(Kind::User, AccountId(1), "2").is_none());
    }

    #[test]
    fn prune_switches_an_old_database_to_incremental_vacuum() {
        let d = tempfile::tempdir().unwrap();
        let path = d.path().join("db.sqlite");
        {
            let db = Connection::open(&path).unwrap();
            db.execute_batch("PRAGMA auto_vacuum = NONE; CREATE TABLE t (x);")
                .unwrap();
        }
        let c = Cache::open(&path).unwrap();
        let mode = |c: &Cache| {
            c.db.lock()
                .query_row("PRAGMA auto_vacuum", [], |r| r.get::<_, i64>(0))
                .unwrap()
        };
        assert_eq!(mode(&c), 0);
        c.prune(0);
        assert_eq!(mode(&c), 2);
    }

    #[test]
    fn ledger_rows_are_private_upserted_and_newest_first() {
        let c = Cache::in_memory().unwrap();
        let (a, b) = ("old".to_string(), "new".to_string());
        c.ledger_put(AccountId(1), "me", [("Sale", "x", 10, &a), ("Sale", "y", 20, &b)]);
        let changed = "old, released".to_string();
        c.ledger_put(AccountId(1), "me", [("Sale", "x", 10, &changed)]);
        assert_eq!(
            c.ledger_all::<String>(AccountId(1), "me"),
            vec!["new", "old, released"]
        );
        assert!(c.ledger_all::<String>(AccountId(2), "me").is_empty());
        assert!(c.ledger_all::<String>(AccountId(1), "group:5").is_empty());
        c.forget_account(AccountId(1));
        assert!(c.ledger_all::<String>(AccountId(1), "me").is_empty());
    }
}

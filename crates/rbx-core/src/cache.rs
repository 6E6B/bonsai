//! SQLite entity cache with fetched-at stamps (stale-while-revalidate).
//!
//! Shared public data (users, universes, items, groups) is stored under `AccountId::ANONYMOUS`
//! and shared by every account. Private data (inventory, messages, trades, friends) is keyed by
//! the account that fetched it and never read by another.

use std::path::Path;

use parking_lot::Mutex;
use rbx_api::AccountId;
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
}

impl Cache {
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
        db.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             CREATE TABLE IF NOT EXISTS entities (
                 kind TEXT NOT NULL,
                 account INTEGER NOT NULL,
                 id TEXT NOT NULL,
                 json TEXT NOT NULL,
                 fetched_at INTEGER NOT NULL,
                 PRIMARY KEY (kind, account, id)
             );",
        )?;
        Ok(Cache { db: Mutex::new(db) })
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
        serde_json::from_str(&json)
            .ok()
            .map(|value| Cached { value, fetched_at })
    }

    pub fn put<T: Serialize>(&self, kind: Kind, account: AccountId, id: &str, value: &T) {
        let Ok(json) = serde_json::to_string(value) else {
            return;
        };
        let db = self.db.lock();
        let r = db.execute(
            "INSERT INTO entities (kind, account, id, json, fetched_at) VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT (kind, account, id) DO UPDATE SET json = excluded.json, fetched_at = excluded.fetched_at",
            params![kind.name(), Self::account_for(kind, account), id, json, now()],
        );
        if let Err(e) = r {
            tracing::warn!("cache write failed: {e}");
        }
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
                serde_json::from_str(&json).ok().map(|v| {
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
        let keep = [Kind::Local, Kind::Wishlist, Kind::Recent].map(Kind::name);
        let db = self.db.lock();
        let r = db.execute(
            "DELETE FROM entities WHERE kind NOT IN (?1, ?2, ?3)",
            params![keep[0], keep[1], keep[2]],
        );
        if let Err(e) = r {
            tracing::warn!("cache clear failed: {e}");
        }
        let _ = db.execute_batch("PRAGMA wal_checkpoint(TRUNCATE); VACUUM;");
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
}

//! The read side of the plugin host: `ctx.user`, `ctx.friends` and the rest. Each read checks
//! the plugin is running and was granted what it needs, then waits on the plugin's own thread for
//! the same cached resources the screens use. See `docs/plugins.md#reads`.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use bonsai_api::generated::presence;
use bonsai_plugin::{Permission, Read, TradeStatus};
use serde_json::{Value, json};

use crate::AccountId;
use crate::core::Inner;
use crate::fmt::parse_iso;
use crate::inventory::{InvItem, InvSlot};
use crate::market::{CatalogItem, ItemDetails, ItemKind, ItemRef, Limited};
use crate::people::{FriendRequest, Profile};
use crate::res::{Page, Res};
use crate::store::{GameInfo, Presence, PresenceKind};
use crate::trades::{TradeBox, TradeRow};

use super::Status;

/// Longest a read waits for Roblox before it fails.
const WAIT: Duration = Duration::from_secs(15);
const POLL: Duration = Duration::from_millis(20);

impl Inner {
    /// Runs `read` for `plugin` as `account`. Blocks; call it only from the plugin's thread.
    pub(super) fn plugin_read(
        self: &Arc<Self>,
        plugin: &str,
        account: u64,
        read: &Read,
    ) -> Result<Value, String> {
        let perm = read.permission();
        let account = AccountId(account);
        self.may_read(plugin, account, perm)?;
        match read {
            Read::User(id) => Ok(wait(|| self.profile(*id))?.map_or(Value::Null, |p| user_view(&p))),
            Read::Game(place) => self.read_game(*place),
            Read::Item { id, bundle } => {
                let item = if *bundle {
                    ItemRef::bundle(*id)
                } else {
                    ItemRef::asset(*id)
                };
                Ok(wait(|| self.item(item))?.map_or(Value::Null, |d| item_view(&d)))
            }
            Read::Presence(ids) => self.read_presence(account, ids),
            Read::Friends => self.read_friends(account),
            Read::Inventory {
                user,
                asset_type,
                bundle_type,
                limit,
            } => {
                let slot = match (asset_type, bundle_type) {
                    (Some(t), _) => InvSlot::Asset(*t),
                    (_, Some(t)) => InvSlot::Bundle(*t),
                    _ => InvSlot::Collectibles,
                };
                let user = user.unwrap_or(account.0);
                let items = pages(*limit, |more| self.inventory(account, user, slot, false, more))?;
                Ok(Value::Array(items.iter().map(inv_view).collect()))
            }
            Read::Trades { status, limit } => {
                let b = match status {
                    TradeStatus::Inbound => TradeBox::Inbound,
                    TradeStatus::Outbound => TradeBox::Outbound,
                    TradeStatus::Completed => TradeBox::Completed,
                    TradeStatus::Inactive => TradeBox::Inactive,
                };
                let rows = pages(*limit, |more| self.trades(account, b, more))?;
                Ok(Value::Array(rows.iter().map(trade_view).collect()))
            }
            Read::FriendRequests { limit } => {
                let rows = pages(*limit, |more| self.friend_requests(account, more))?;
                Ok(Value::Array(rows.iter().map(request_view).collect()))
            }
        }
    }

    /// The plugin is running and agreed to `perm`, and for private reads was granted `account`.
    fn may_read(&self, plugin: &str, account: AccountId, perm: Permission) -> Result<(), String> {
        let p = self.plugins.lock();
        let e = p
            .entries
            .iter()
            .find(|e| e.info.id == plugin)
            .ok_or("This plugin isn't installed.")?;
        if !matches!(e.info.status, Status::Running { .. }) {
            return Err("This plugin isn't running.".into());
        }
        let agreed = p
            .saved
            .plugins
            .get(plugin)
            .is_some_and(|s| s.permissions.iter().any(|k| k == perm.key()));
        if !agreed {
            return Err(format!("\"{}\" wasn't agreed to.", perm.key()));
        }
        if perm.per_account() {
            if account.0 == 0 || !e.info.accounts.contains(&account) {
                return Err("It may not use this account. Allow it in Settings > Plugins.".into());
            }
            drop(p);
            if self.store.read().accounts.get(account).is_none() {
                return Err("This account isn't signed in.".into());
            }
        }
        Ok(())
    }

    pub(super) fn read_game(&self, place: u64) -> Result<Value, String> {
        let cached = |inner: &Inner| inner.store.read().game_for_place(place).map(game_view);
        if let Some(v) = cached(self) {
            return Ok(v);
        }
        self.block(self.load_place(AccountId::ANONYMOUS, place))?;
        Ok(cached(self).unwrap_or(Value::Null))
    }

    /// Friends' presence from the friends list refresh; anyone else is asked for, without
    /// touching the friends map, so it can't set off join alerts.
    fn read_presence(&self, account: AccountId, ids: &[u64]) -> Result<Value, String> {
        let known: HashMap<u64, Presence> = {
            let s = self.store.read();
            let d = s.account_data(account);
            ids.iter()
                .filter_map(|id| Some((*id, d?.presence.get(id)?.clone())))
                .collect()
        };
        let ask: Vec<u64> = ids.iter().filter(|i| !known.contains_key(i)).copied().collect();
        let mut all = known;
        if !ask.is_empty() {
            let c = self.client(account).ok_or("This account isn't signed in.")?;
            for chunk in ask.chunks(50) {
                let body = presence::v1::UserPresenceRequest {
                    user_ids: Some(chunk.iter().map(|i| *i as i64).collect()),
                };
                let r = self
                    .block(presence::v1::post_presence_users(&c, &body))?
                    .map_err(|e| e.user_message())?;
                all.extend(
                    r.user_presences
                        .unwrap_or_default()
                        .into_iter()
                        .filter_map(Presence::from_api),
                );
            }
        }
        Ok(Value::Array(
            ids.iter()
                .filter_map(|id| Some(presence_view(*id, all.get(id)?)))
                .collect(),
        ))
    }

    fn read_friends(self: &Arc<Self>, account: AccountId) -> Result<Value, String> {
        let loaded = |inner: &Inner| {
            inner
                .store
                .read()
                .account_data(account)
                .and_then(|d| d.friends.data.clone())
        };
        let ids = match loaded(self) {
            Some(ids) => ids,
            None => {
                self.block(self.refresh_friends(account))?;
                let s = self.store.read();
                let f = s.account_data(account).map(|d| &d.friends);
                match f.and_then(|f| f.data.clone()) {
                    Some(ids) => ids,
                    None => {
                        return Err(f
                            .and_then(|f| f.error.clone())
                            .unwrap_or_else(|| "Can't load friends.".into()));
                    }
                }
            }
        };
        let s = self.store.read();
        let presence = s.account_data(account).map(|d| &d.presence);
        Ok(Value::Array(
            ids.iter()
                .map(|id| {
                    let u = s.users.get(id);
                    json!({
                        "id": id,
                        "name": u.map(|u| u.name.as_str()),
                        "display_name": u.map(|u| u.display_name.as_str()),
                        "status": status(presence.and_then(|p| p.get(id)).map_or(PresenceKind::Offline, |p| p.kind)),
                    })
                })
                .collect(),
        ))
    }

    /// Waits for `f` on the runtime, at most [`WAIT`].
    fn block<F: Future>(&self, f: F) -> Result<F::Output, String> {
        self.rt
            .block_on(async { tokio::time::timeout(WAIT, f).await })
            .map_err(|_| timed_out())
    }
}

fn timed_out() -> String {
    format!("Roblox didn't answer within {} s.", WAIT.as_secs())
}

/// Polls a resource until `f` gives data and isn't loading. `None` when it has nothing to give.
pub(super) fn wait<T>(f: impl Fn() -> Res<T>) -> Result<Option<Arc<T>>, String> {
    let deadline = Instant::now() + WAIT;
    loop {
        let r = f();
        if !r.loading {
            if r.data.is_some() {
                return Ok(r.data);
            }
            if let Some(e) = r.error {
                return Err(e);
            }
        }
        if Instant::now() > deadline {
            return Err(timed_out());
        }
        std::thread::sleep(POLL);
    }
}

/// Up to `limit` rows of a paged resource, asking for more pages while there are any.
pub(super) fn pages<T: Clone>(limit: usize, page: impl Fn(bool) -> Res<Page<T>>) -> Result<Vec<T>, String> {
    let Some(mut p) = wait(|| page(false))? else {
        return Ok(Vec::new());
    };
    while p.items.len() < limit && p.next.is_some() {
        let had = p.items.len();
        page(true);
        match wait(|| page(false))? {
            Some(next) if next.items.len() > had => p = next,
            _ => break,
        }
    }
    let mut items = p.items.clone();
    items.truncate(limit);
    Ok(items)
}

/// ISO dates as Unix seconds, so plugins can compare them with `ctx.now`.
fn time(s: &str) -> Option<i64> {
    parse_iso(s)
}

/// `-1` and other unknown counts as `nil`.
fn count(n: i64) -> Option<i64> {
    (n >= 0).then_some(n)
}

pub(crate) fn status(k: PresenceKind) -> &'static str {
    match k {
        PresenceKind::Offline => "offline",
        PresenceKind::Online => "online",
        PresenceKind::InGame => "in_game",
        PresenceKind::InStudio => "in_studio",
        PresenceKind::Invisible => "invisible",
    }
}

fn kind(k: ItemKind) -> &'static str {
    match k {
        ItemKind::Asset => "asset",
        ItemKind::Bundle => "bundle",
    }
}

fn user_view(p: &Profile) -> Value {
    json!({
        "id": p.id,
        "name": p.name,
        "display_name": p.display_name,
        "description": p.description,
        "created": time(&p.created),
        "verified": p.verified,
        "banned": p.banned,
        "friends": count(p.friends),
        "followers": count(p.followers),
        "following": count(p.following),
    })
}

fn game_view(g: &GameInfo) -> Value {
    json!({
        "universe_id": g.universe_id,
        "place_id": g.root_place_id,
        "name": g.name,
        "description": g.description,
        "creator": {
            "id": g.creator_id,
            "name": g.creator_name,
            "is_group": g.creator_is_group,
            "verified": g.creator_verified,
        },
        "playing": g.playing,
        "visits": g.visits,
        "favorites": g.favorites,
        "max_players": g.max_players,
        "created": time(&g.created),
        "updated": time(&g.updated),
        "genre": g.genre,
        "price": g.price,
        "up_votes": g.up_votes,
        "down_votes": g.down_votes,
    })
}

fn card_view(c: &CatalogItem) -> serde_json::Map<String, Value> {
    let v = json!({
        "id": c.item.id,
        "kind": kind(c.item.kind),
        "name": c.name,
        "creator": {
            "id": c.creator_id,
            "name": c.creator,
            "is_group": c.creator_is_group,
            "verified": c.creator_verified,
        },
        "price": c.price,
        "lowest_resale": c.lowest_resale,
        "off_sale": c.off_sale,
        "limited": match c.limited {
            Limited::No => Value::Null,
            Limited::Limited => "limited".into(),
            Limited::LimitedU => "limited_u".into(),
            Limited::Collectible => "collectible".into(),
        },
        "favorites": c.favorites,
        "asset_type": c.asset_type,
        "bundle_type": c.bundle_type,
        "total_quantity": c.total_quantity,
        "units_left": c.units_left,
        "premium_only": c.premium_only,
    });
    match v {
        Value::Object(m) => m,
        _ => serde_json::Map::new(),
    }
}

fn item_view(d: &ItemDetails) -> Value {
    let mut m = card_view(&d.card);
    m.insert("description".into(), d.description.clone().into());
    m.insert("created".into(), json!(time(&d.created)));
    m.insert("updated".into(), json!(time(&d.updated)));
    m.insert("category".into(), d.category.clone().into());
    m.insert("tags".into(), json!(d.tags));
    Value::Object(m)
}

fn request_view(r: &FriendRequest) -> Value {
    json!({
        "id": r.user_id,
        "name": r.name,
        "display_name": r.display_name,
        "sent": time(&r.sent_at),
        "mutuals": r.mutuals,
        "origin": r.origin,
    })
}

pub(crate) fn presence_view(id: u64, p: &Presence) -> Value {
    json!({
        "id": id,
        "status": status(p.kind),
        "place_id": p.place_id,
        "universe_id": p.universe_id,
        "location": p.last_location,
    })
}

fn inv_view(i: &InvItem) -> Value {
    json!({
        "id": i.item.id,
        "kind": kind(i.item.kind),
        "name": i.name,
        "copy_id": (i.copy_id > 0).then_some(i.copy_id),
        "serial": i.serial,
        "rap": i.rap,
        "original_price": i.original_price,
        "on_hold": i.on_hold,
        "acquired": time(&i.acquired),
    })
}

fn trade_view(t: &TradeRow) -> Value {
    json!({
        "id": t.id,
        "partner": {
            "id": t.partner,
            "name": t.partner_name,
            "display_name": t.partner_display,
        },
        "created": time(&t.created),
        "expires": time(&t.expires),
        "status": t.status,
        "active": t.active,
    })
}

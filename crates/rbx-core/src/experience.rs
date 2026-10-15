//! Everything on an experience page beyond the header and public servers: media, your vote,
//! favorite and follow ("Notify") state, maturity label, store (passes), badges, events,
//! recommendations and private servers.
//!
//! Reads are cached resources; writes are user actions from the command registry. Buying a
//! private server or a pass only happens after the UI's confirm dialog (passes: `purchase.rs`).

use std::collections::HashMap;
use std::sync::Arc;

use rbx_api::AccountId;
use rbx_api::generated::{apis, badges, followings, games, www};
use serde::{Deserialize, Serialize};

use crate::launcher::JoinTarget;
use crate::people::Badge;
use crate::res::{Key, Page, Res};
use crate::store::{GameCard, UserInfo};
use crate::store::{ServerPick, pick_server};
use crate::{Cmd, Core};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Media {
    /// Image asset id (for videos, the preview image if any).
    pub image_id: u64,
    pub video_title: Option<String>,
    pub alt: String,
}

/// Approved items with an image, each image once. A preview video repeats the id of the image
/// it previews, so only the first entry for an id is kept.
fn media_list(items: Vec<games::v2::GameMediaItemResponseV2>) -> Vec<Media> {
    let mut seen = std::collections::HashSet::new();
    items
        .into_iter()
        .filter(|m| m.approved != Some(false))
        .filter_map(|m| {
            Some(Media {
                image_id: m.image_id.filter(|i| *i > 0)? as u64,
                video_title: m.video_title.filter(|t| !t.is_empty()),
                alt: m.alt_text.unwrap_or_default(),
            })
        })
        .filter(|m| seen.insert(m.image_id))
        .collect()
}

/// The signed-in account's own state for an experience.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MyGameState {
    pub can_vote: bool,
    /// `Some(true)` liked, `Some(false)` disliked.
    pub vote: Option<bool>,
    /// Why voting is off, in plain words.
    pub vote_hint: String,
    pub favorited: bool,
    /// Following an experience is the site's "Notify" bell: you get its update notifications.
    pub following: bool,
    pub can_follow: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Maturity {
    /// "Minimal", "Mild", "Moderate", "Restricted".
    pub label: String,
    pub minimum_age: i32,
    pub descriptors: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Pass {
    pub id: u64,
    /// What the purchase endpoint takes; 0 when Roblox didn't send it.
    #[serde(default)]
    pub product_id: u64,
    pub name: String,
    pub description: String,
    /// None when off sale.
    pub price: Option<i64>,
    pub owned: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub start: String,
    pub end: String,
    pub host: String,
    pub going: bool,
    pub image_id: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PrivateServer {
    pub id: u64,
    pub name: String,
    pub owner_id: u64,
    pub playing: i32,
    pub max_players: i32,
    pub access_code: String,
}

/// A public server with friends of the account in it.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FriendServer {
    pub job_id: String,
    pub playing: i32,
    pub max_players: i32,
    pub ping: Option<i32>,
    /// Friends in it, in Roblox's order.
    pub friends: Vec<u64>,
}

/// What the site's game page says about buying a private server.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PrivateServerOffer {
    pub enabled: bool,
    /// Robux; 0 is free.
    pub price: Option<i64>,
    pub can_create: bool,
}

/// One of your own private servers, for configuring.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ServerConfig {
    pub id: u64,
    pub name: String,
    pub active: bool,
    pub link: String,
    pub friends_allowed: bool,
    pub renews: bool,
    pub expires: String,
    pub price: i64,
    /// Users allowed in besides friends, in Roblox's order.
    #[serde(default)]
    pub users: Vec<u64>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ServerChange {
    Rename(String),
    Active(bool),
    NewLink,
    FriendsAllowed(bool),
    /// Turn auto-renew on or off.
    Renew(bool),
    /// Let a user in.
    AddUser(u64),
    /// Take a user off the list.
    RemoveUser(u64),
}

const INFO_AGE: i64 = 600;
const STATE_AGE: i64 = 120;
const LIST_AGE: i64 = 300;

impl Core {
    pub fn game_media(&self, universe_id: u64) -> Res<Vec<Media>> {
        self.inner.res(
            Key::shared("game_media", universe_id),
            INFO_AGE,
            move |f| async move {
                let r = games::v2::get_games_by_universe_id_media(&f.client, universe_id as i64).await?;
                Ok(media_list(r.data.unwrap_or_default()))
            },
        )
    }

    /// Your vote, favorite and follow state. Empty when signed out.
    pub fn my_game_state(&self, account: AccountId, universe_id: u64) -> Res<MyGameState> {
        if account.is_anonymous() {
            return Res::default();
        }
        self.inner.res(
            Key::private(account, "my_game_state", universe_id),
            STATE_AGE,
            move |f| async move {
                let c = &f.client;
                let u = universe_id as i64;
                let (v, fav, fol) = tokio::join!(
                    games::v1::get_games_by_universe_id_votes_user(c, u),
                    games::v1::get_games_by_universe_id_favorites(c, u),
                    followings::v1::get_users_by_user_id_universes_by_universe_id_status(
                        c,
                        account.0 as i64,
                        u
                    ),
                );
                let v = v?;
                let fol = fol.ok().unwrap_or_default();
                Ok(MyGameState {
                    can_vote: v.can_vote.unwrap_or(false),
                    vote: v.user_vote,
                    vote_hint: vote_hint(v.reason_for_not_voteable.as_deref().unwrap_or("")),
                    favorited: fav?.is_favorited.unwrap_or(false),
                    following: fol.is_following.unwrap_or(false),
                    can_follow: fol.can_follow.unwrap_or(false),
                })
            },
        )
    }

    pub fn maturity(&self, universe_id: u64) -> Res<Maturity> {
        self.inner.res(
            Key::shared("maturity", universe_id),
            INFO_AGE,
            move |f| async move {
                let body = apis::undocumented::AgeRecommendationRequest {
                    universe_id: Some(universe_id.to_string()),
                };
                let r = apis::undocumented::post_age_recommendation(&f.client, &body).await?;
                let d = r.age_recommendation_details.unwrap_or_default();
                let a = d.summary.and_then(|s| s.age_recommendation).unwrap_or_default();
                Ok(Maturity {
                    label: a.display_name.unwrap_or_default(),
                    minimum_age: a.minimum_age.unwrap_or(0),
                    descriptors: d
                        .descriptor_usages
                        .unwrap_or_default()
                        .into_iter()
                        .filter_map(|u| u.descriptor.and_then(|d| d.display_name))
                        .collect(),
                })
            },
        )
    }

    /// Passes in the experience's store. Ownership is per account.
    pub fn game_passes(&self, account: AccountId, universe_id: u64, more: bool) -> Res<Page<Pass>> {
        self.inner.paged(
            Key::private(account, "game_passes", universe_id),
            LIST_AGE,
            more,
            move |f, cursor| async move {
                let p = apis::undocumented::GetUniverseGamePassesParams {
                    pass_view: Some("Full".into()),
                    page_size: Some(50),
                    page_token: cursor,
                };
                let r =
                    apis::undocumented::get_universe_game_passes(&f.client, universe_id as i64, &p).await?;
                Ok(Page {
                    items: r
                        .game_passes
                        .unwrap_or_default()
                        .into_iter()
                        .filter_map(|g| {
                            let for_sale = g.is_for_sale.unwrap_or(false);
                            Some(Pass {
                                id: g.id? as u64,
                                product_id: g.product_id.unwrap_or(0).max(0) as u64,
                                name: g.display_name.or(g.name).unwrap_or_default(),
                                description: g.display_description.unwrap_or_default(),
                                price: g.price.filter(|_| for_sale),
                                owned: g.is_owned.unwrap_or(false),
                            })
                        })
                        .collect(),
                    next: r.next_page_token.filter(|t| !t.is_empty()),
                })
            },
        )
    }

    pub fn game_badges(&self, universe_id: u64, more: bool) -> Res<Page<Badge>> {
        self.inner.paged(
            Key::shared("game_badges", universe_id),
            LIST_AGE,
            more,
            move |f, cursor| async move {
                let p = badges::v1::GetUniversesByUniverseIdBadgesParams {
                    limit: Some(100),
                    cursor,
                    ..Default::default()
                };
                let r = badges::v1::get_universes_by_universe_id_badges(&f.client, universe_id as i64, &p)
                    .await?;
                Ok(Page {
                    items: r
                        .data
                        .unwrap_or_default()
                        .into_iter()
                        .filter_map(|b| {
                            let s = b.statistics.unwrap_or_default();
                            let u = b.awarding_universe.unwrap_or_default();
                            Some(Badge {
                                id: b.id? as u64,
                                name: b.display_name.or(b.name).unwrap_or_default(),
                                description: b.display_description.or(b.description).unwrap_or_default(),
                                universe_id: u.id.unwrap_or(universe_id as i64) as u64,
                                universe_name: u.name.unwrap_or_default(),
                                place_id: u.root_place_id.unwrap_or(0) as u64,
                                win_rate: s.win_rate_percentage.unwrap_or(0.0),
                                awarded: s.awarded_count.unwrap_or(0),
                                icon_image_id: b.icon_image_id.unwrap_or(0) as u64,
                            })
                        })
                        .collect(),
                    next: r.next_page_cursor,
                })
            },
        )
    }

    /// When `account` earned each of these badges (badge id → ISO date). Missing = not earned.
    pub fn badges_earned(&self, account: AccountId, badge_ids: &[u64]) -> Res<HashMap<u64, String>> {
        if account.is_anonymous() || badge_ids.is_empty() {
            return Res::default();
        }
        let ids: Vec<i64> = badge_ids.iter().take(100).map(|i| *i as i64).collect();
        let key_id = format!("{}:{}", ids.len(), ids.first().copied().unwrap_or(0));
        self.inner.res(
            Key::private(account, "badges_earned", key_id),
            LIST_AGE,
            move |f| async move {
                let p = badges::v1::GetUsersByUserIdBadgesAwardedDatesParams { badge_ids: ids };
                let r =
                    badges::v1::get_users_by_user_id_badges_awarded_dates(&f.client, account.0 as i64, &p)
                        .await?;
                Ok(r.data
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|a| Some((a.badge_id? as u64, a.awarded_date.unwrap_or_default())))
                    .collect())
            },
        )
    }

    /// Upcoming and current events. `going` is per account.
    pub fn game_events(&self, account: AccountId, universe_id: u64) -> Res<Vec<Event>> {
        self.inner.res(
            Key::private(account, "game_events", universe_id),
            LIST_AGE,
            move |f| async move {
                // From the start of today so events running now still show.
                let today = crate::cache::now() / 86_400 * 86_400;
                let p = apis::undocumented::GetUniverseEventsParams {
                    from_utc: Some(crate::fmt::iso_utc(today - 30 * 86_400)),
                    limit: Some(25),
                    cursor: None,
                };
                let r = apis::undocumented::get_universe_events(&f.client, universe_id as i64, &p).await?;
                let now = crate::fmt::iso_utc(crate::cache::now());
                Ok(r.data
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|e| {
                        let t = e.event_time.unwrap_or_default();
                        let end = t.end_utc.unwrap_or_default();
                        // ISO strings in the same format compare in time order.
                        if !end.is_empty() && end.as_str() < now.as_str() {
                            return None;
                        }
                        let mut thumbs = e.thumbnails.unwrap_or_default();
                        thumbs.sort_by_key(|t| t.rank.unwrap_or(0));
                        Some(Event {
                            id: e.id?,
                            title: e.display_title.unwrap_or_default(),
                            subtitle: e.display_subtitle.unwrap_or_default(),
                            description: e.display_description.unwrap_or_default(),
                            start: t.start_utc.unwrap_or_default(),
                            end,
                            host: e.host.and_then(|h| h.host_name).unwrap_or_default(),
                            going: e.user_rsvp_status.as_deref() == Some("going"),
                            image_id: thumbs.first().and_then(|t| t.media_id).unwrap_or(0) as u64,
                        })
                    })
                    .collect())
            },
        )
    }

    pub fn recommended_games(&self, universe_id: u64) -> Res<Vec<GameCard>> {
        self.inner.res(
            Key::shared("recommended_games", universe_id),
            INFO_AGE,
            move |f| async move {
                let p = games::v1::GetGamesRecommendationsGameByUniverseIdParams {
                    max_rows: 12,
                    ..Default::default()
                };
                let r = games::v1::get_games_recommendations_game_by_universe_id(
                    &f.client,
                    universe_id as i64,
                    &p,
                )
                .await?;
                Ok(r.games
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|g| {
                        Some(GameCard {
                            universe_id: g.universe_id? as u64,
                            place_id: g.place_id? as u64,
                            name: g.name.unwrap_or_default(),
                            playing: g.player_count.unwrap_or(0) as i64,
                            up_votes: g.total_up_votes.unwrap_or(0),
                            down_votes: g.total_down_votes.unwrap_or(0),
                            creator: g.creator_name.unwrap_or_default(),
                            sponsored: g.is_sponsored.unwrap_or(false),
                            genre: g.genre.unwrap_or_default(),
                        })
                    })
                    .collect())
            },
        )
    }

    /// Private servers `account` can join in this place (its own and ones it was invited to).
    pub fn private_servers(&self, account: AccountId, place_id: u64, more: bool) -> Res<Page<PrivateServer>> {
        self.inner.paged(
            Key::private(account, "private_servers", place_id),
            STATE_AGE,
            more,
            move |f, cursor| async move {
                let p = games::v1::GetGamesByPlaceIdPrivateServersParams {
                    limit: Some(25),
                    cursor,
                    sort_order: None,
                };
                let r =
                    games::v1::get_games_by_place_id_private_servers(&f.client, place_id as i64, &p).await?;
                let mut users = Vec::new();
                let items = r
                    .data
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|s| {
                        let o = s.owner.unwrap_or_default();
                        let owner_id = o.id.unwrap_or(0) as u64;
                        if owner_id > 0 {
                            users.push(UserInfo {
                                id: owner_id,
                                name: o.name.clone().unwrap_or_default(),
                                display_name: o.display_name.clone().or(o.name.clone()).unwrap_or_default(),
                                verified: o.has_verified_badge.unwrap_or(false),
                            });
                        }
                        Some(PrivateServer {
                            id: s.vip_server_id? as u64,
                            name: s.name.unwrap_or_default(),
                            owner_id,
                            playing: s.playing.unwrap_or(0),
                            max_players: s.max_players.unwrap_or(0),
                            access_code: s.access_code.unwrap_or_default(),
                        })
                    })
                    .collect();
                f.note_users(users);
                Ok(Page {
                    items,
                    next: r.next_page_cursor,
                })
            },
        )
    }

    /// Whether you can buy a private server here and for how much. The price is only on the
    /// site's game page (see specs/undocumented/www.roblox.com.yaml).
    pub fn private_server_offer(
        &self,
        account: AccountId,
        universe_id: u64,
        place_id: u64,
    ) -> Res<PrivateServerOffer> {
        self.inner.res(
            Key::private(account, "private_server_offer", universe_id),
            INFO_AGE,
            move |f| async move {
                let c = &f.client;
                let enabled =
                    games::v1::get_private_servers_enabled_in_universe_by_universe_id(c, universe_id as i64)
                        .await?;
                let enabled = enabled.private_servers_enabled.unwrap_or(false);
                if !enabled {
                    return Ok(PrivateServerOffer::default());
                }
                let page = c
                    .send(www::undocumented::page_game_request(place_id as i64)?)
                    .await?;
                let html = String::from_utf8_lossy(&page.body);
                let price = data_attr(&html, "data-private-server-price").and_then(|v| v.parse().ok());
                Ok(PrivateServerOffer {
                    enabled,
                    price,
                    can_create: data_attr(&html, "data-can-create-server").as_deref() == Some("True"),
                })
            },
        )
    }

    /// Settings of one of your own private servers.
    pub fn server_config(&self, account: AccountId, server_id: u64) -> Res<ServerConfig> {
        if account.is_anonymous() {
            return Res::default();
        }
        self.inner.res(
            Key::private(account, "server_config", server_id),
            STATE_AGE,
            move |f| async move {
                let r = games::v1::get_vip_servers_by_id(&f.client, server_id as i64).await?;
                let users = r.permissions.iter().flat_map(|p| p.users.iter().flatten());
                f.note_users(
                    users
                        .filter_map(|u| {
                            let name = u.name.clone().unwrap_or_default();
                            Some(UserInfo {
                                id: u.id? as u64,
                                display_name: u.display_name.clone().unwrap_or_else(|| name.clone()),
                                name,
                                verified: false,
                            })
                        })
                        .collect::<Vec<_>>(),
                );
                Ok(server_config(server_id, r))
            },
        )
    }

    /// Servers of a place that friends of the account are in (Roblox's "friends" server
    /// type). Empty signed out.
    pub fn friend_servers(&self, account: AccountId, place_id: u64) -> Res<Vec<FriendServer>> {
        if account.is_anonymous() {
            return Res::default();
        }
        self.inner.res(
            Key::private(account, "friend_servers", place_id),
            STATE_AGE,
            move |f| async move {
                let p = games::v1::GetGamesByPlaceIdServersByServerTypeParams {
                    limit: Some(100),
                    ..Default::default()
                };
                let r = games::v1::get_games_by_place_id_servers_by_server_type(
                    &f.client,
                    place_id as i64,
                    1,
                    &p,
                )
                .await?;
                let mut users = Vec::new();
                let list = r
                    .data
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|s| {
                        let friends = s
                            .players
                            .unwrap_or_default()
                            .into_iter()
                            .filter_map(|u| {
                                let id = u.id.filter(|i| *i > 0)? as u64;
                                let name = u.name.unwrap_or_default();
                                users.push(UserInfo {
                                    id,
                                    display_name: u.display_name.unwrap_or_else(|| name.clone()),
                                    name,
                                    verified: false,
                                });
                                Some(id)
                            })
                            .collect();
                        Some(FriendServer {
                            job_id: s.id.filter(|i| !i.is_empty())?,
                            playing: s.playing.unwrap_or(0),
                            max_players: s.max_players.unwrap_or(0),
                            ping: s.ping,
                            friends,
                        })
                    })
                    .collect();
                f.note_users(users);
                Ok(list)
            },
        )
    }

    /// Reload a place's public and friends' servers.
    pub fn refresh_servers(&self, account: AccountId, place_id: u64) {
        self.inner.res.invalidate(account, "friend_servers");
        self.send(Cmd::LoadServers {
            account,
            place_id,
            more: false,
        });
    }

    // ----- actions -----

    pub fn vote(&self, account: AccountId, universe_id: u64, up: bool) {
        self.set_game_state(account, universe_id, |s| s.vote = Some(up));
        self.inner
            .mutate(account, "vote", "", &["my_game_state"], move |f| async move {
                let body = games::v1::SetUserGameVoteRequest { vote: Some(up) };
                games::v1::patch_games_by_universe_id_user_votes(&f.client, universe_id as i64, &body)
                    .await?;
                Ok(())
            });
    }

    pub fn favorite(&self, account: AccountId, universe_id: u64, on: bool) {
        self.set_game_state(account, universe_id, |s| s.favorited = on);
        self.inner.mutate(
            account,
            if on { "favorite" } else { "unfavorite" },
            "",
            &["my_game_state"],
            move |f| async move {
                let body = games::v1::GameFavoritesRequest {
                    is_favorited: Some(on),
                };
                games::v1::post_games_by_universe_id_favorites(&f.client, universe_id as i64, &body).await?;
                Ok(())
            },
        );
    }

    /// The site's "Notify" bell: follow an experience to get its update notifications.
    pub fn follow_game(&self, account: AccountId, universe_id: u64, on: bool) {
        self.set_game_state(account, universe_id, |s| s.following = on);
        self.inner.mutate(
            account,
            if on {
                "turn on notifications"
            } else {
                "turn off notifications"
            },
            if on {
                "You'll be notified about updates."
            } else {
                ""
            },
            &["my_game_state"],
            move |f| async move {
                let (u, g) = (account.0 as i64, universe_id as i64);
                if on {
                    followings::v1::post_users_by_user_id_universes_by_universe_id(&f.client, u, g).await?;
                } else {
                    followings::v1::delete_users_by_user_id_universes_by_universe_id(&f.client, u, g).await?;
                }
                Ok(())
            },
        );
    }

    /// "Notify me" on an event (RSVP going), or undo it.
    pub fn rsvp(&self, account: AccountId, universe_id: u64, event_id: &str, going: bool) {
        let id = event_id.to_string();
        self.inner
            .res
            .update::<Vec<Event>>(&Key::private(account, "game_events", universe_id), |v| {
                for e in v.iter_mut().filter(|e| e.id == id) {
                    e.going = going;
                }
            });
        self.inner.repaint();
        let id = event_id.to_string();
        self.inner.mutate(
            account,
            "update the event",
            if going {
                "You'll be notified when it starts."
            } else {
                ""
            },
            &["game_events"],
            move |f| async move {
                let body = apis::undocumented::RsvpRequest {
                    rsvp_status: Some(if going { "going" } else { "notGoing" }.into()),
                };
                apis::undocumented::post_event_rsvp(&f.client, &id, &body).await?;
                Ok(())
            },
        );
    }

    /// Buy a private server. Only call after the user confirmed `price` in the UI.
    pub fn buy_private_server(&self, account: AccountId, universe_id: u64, name: String, price: i64) {
        self.inner.mutate(
            account,
            "buy the private server",
            "Private server bought.",
            &["private_servers", "robux"],
            move |f| async move {
                let body = games::v1::CreateVipServersRequest {
                    expected_price: Some(price),
                    is_purchase_confirmed: Some(true),
                    name: Some(name),
                };
                games::v1::post_games_vip_servers_by_universe_id(&f.client, universe_id as i64, &body)
                    .await?;
                Ok(())
            },
        );
    }

    pub fn configure_server(&self, account: AccountId, server_id: u64, change: ServerChange) {
        let done = match &change {
            ServerChange::NewLink => "New link made. The old one stops working.",
            ServerChange::Renew(false) => "It won't renew. It stays until it expires.",
            ServerChange::AddUser(_) => "Added to the server.",
            ServerChange::RemoveUser(_) => "Removed from the server.",
            _ => "",
        };
        self.inner.mutate(
            account,
            "change the private server",
            done,
            &["server_config", "private_servers", "my_private_servers"],
            move |f| async move {
                let (c, id) = (&f.client, server_id as i64);
                match change {
                    ServerChange::Rename(n) => {
                        let b = games::v1::VipServerUpdateRequest {
                            name: Some(n),
                            ..Default::default()
                        };
                        games::v1::patch_vip_servers_by_id(c, id, &b).await?;
                    }
                    ServerChange::Active(on) => {
                        let b = games::v1::VipServerUpdateRequest {
                            active: Some(on),
                            ..Default::default()
                        };
                        games::v1::patch_vip_servers_by_id(c, id, &b).await?;
                    }
                    ServerChange::NewLink => {
                        let b = games::v1::VipServerUpdateRequest {
                            new_join_code: Some(true),
                            ..Default::default()
                        };
                        games::v1::patch_vip_servers_by_id(c, id, &b).await?;
                    }
                    ServerChange::FriendsAllowed(on) => {
                        let b = games::v1::VipServerUpdatePermissionsRequest {
                            friends_allowed: Some(on),
                            ..Default::default()
                        };
                        games::v1::patch_vip_servers_by_id_permissions(c, id, &b).await?;
                    }
                    ServerChange::AddUser(u) | ServerChange::RemoveUser(u) => {
                        let add = matches!(change, ServerChange::AddUser(_));
                        let b = games::v1::VipServerUpdatePermissionsRequest {
                            users_to_add: add.then(|| vec![u as i64]),
                            users_to_remove: (!add).then(|| vec![u as i64]),
                            ..Default::default()
                        };
                        games::v1::patch_vip_servers_by_id_permissions(c, id, &b).await?;
                    }
                    ServerChange::Renew(on) => {
                        let b = games::v1::VipServerUpdateSubscriptionRequest {
                            active: Some(on),
                            price: None,
                        };
                        games::v1::patch_vip_servers_by_id_subscription(c, id, &b).await?;
                    }
                }
                Ok(())
            },
        );
    }

    // ----- page buttons: toggle from the current state, or say why not -----

    /// Like or dislike. The error says why voting is off.
    pub fn try_vote(&self, account: AccountId, universe_id: u64, up: bool) -> Result<(), String> {
        let st = self.loaded_state(account, universe_id)?;
        if !st.can_vote {
            return Err(match st.vote_hint.as_str() {
                "" => "You can't vote on this experience.".into(),
                hint => hint.into(),
            });
        }
        self.vote(account, universe_id, up);
        Ok(())
    }

    pub fn toggle_favorite(&self, account: AccountId, universe_id: u64) -> Result<(), String> {
        let st = self.loaded_state(account, universe_id)?;
        self.favorite(account, universe_id, !st.favorited);
        Ok(())
    }

    pub fn toggle_follow_game(&self, account: AccountId, universe_id: u64) -> Result<(), String> {
        let st = self.loaded_state(account, universe_id)?;
        if !st.following && !st.can_follow {
            return Err("This experience can't send you updates.".into());
        }
        self.follow_game(account, universe_id, !st.following);
        Ok(())
    }

    pub fn toggle_rsvp(&self, account: AccountId, universe_id: u64, event_id: &str) -> Result<(), String> {
        let going = self
            .game_events(account, universe_id)
            .data
            .and_then(|v| v.iter().find(|e| e.id == event_id).map(|e| e.going))
            .ok_or_else(|| "That event isn't listed for this experience.".to_owned())?;
        self.rsvp(account, universe_id, event_id, !going);
        Ok(())
    }

    /// Join a public server chosen by `how` from the loaded list.
    pub fn join_picked(&self, account: AccountId, place_id: u64, how: ServerPick) -> Result<(), String> {
        let job_id = {
            let s = self.store();
            let list = s
                .servers
                .get(&place_id)
                .and_then(|l| l.data.as_ref())
                .ok_or_else(|| "Servers are still loading.".to_owned())?;
            pick_server(list, how)
                .map(|s| s.job_id.clone())
                .ok_or_else(|| "No server has room.".to_owned())?
        };
        self.send(Cmd::Launch {
            account,
            target: JoinTarget::Server { place_id, job_id },
        });
        Ok(())
    }

    /// Join one of the private servers `account` sees in this place.
    pub fn join_private_server(
        &self,
        account: AccountId,
        place_id: u64,
        server_id: u64,
    ) -> Result<(), String> {
        let target = move |code: String| JoinTarget::PrivateServer {
            place_id,
            link_code: None,
            access_code: Some(code),
        };
        let r = self.private_servers(account, place_id, false);
        if let Some(p) = r.data {
            let code = server_code(&p, server_id).ok_or_else(|| NOT_LISTED.to_owned())?;
            self.send(Cmd::Launch {
                account,
                target: target(code),
            });
            return Ok(());
        }
        if let Some(e) = r.error {
            return Err(e);
        }
        // Not loaded yet (joined from the list across experiences): wait for it, then join.
        let inner = self.inner.clone();
        let key = Key::private(account, "private_servers", place_id);
        self.inner.rt.spawn(async move {
            for _ in 0..100 {
                let r = inner
                    .res
                    .snapshot::<Page<PrivateServer>>(&key)
                    .unwrap_or_default();
                if let Some(p) = r.data {
                    match server_code(&p, server_id) {
                        Some(code) => inner.launch(account, target(code)).await,
                        None => inner.toast(Some(account), NOT_LISTED, true),
                    }
                    return;
                }
                if let Some(e) = r.error {
                    inner.toast(Some(account), format!("Could not join: {e}"), true);
                    return;
                }
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
            inner.toast(
                Some(account),
                "Could not join: the server list didn't load.",
                true,
            );
        });
        Ok(())
    }

    /// A private server toggle from its current state: `Active`, `FriendsAllowed` or `Renew`
    /// with the flag flipped. Errors until the settings have loaded.
    pub fn toggle_server(
        &self,
        account: AccountId,
        server_id: u64,
        which: fn(bool) -> ServerChange,
    ) -> Result<(), String> {
        let c = self.loaded_config(account, server_id)?;
        let change = which(true);
        let on = match change {
            ServerChange::Active(_) => !c.active,
            ServerChange::FriendsAllowed(_) => !c.friends_allowed,
            ServerChange::Renew(_) => !c.renews,
            _ => return Err("Not a toggle.".into()),
        };
        self.change_server(account, server_id, which(on));
        Ok(())
    }

    /// Applies a change to your private server, showing it at once.
    pub fn change_server(&self, account: AccountId, server_id: u64, change: ServerChange) {
        self.inner
            .res
            .update::<ServerConfig>(
                &Key::private(account, "server_config", server_id),
                |c| match &change {
                    ServerChange::Rename(n) => c.name = n.clone(),
                    ServerChange::Active(on) => c.active = *on,
                    ServerChange::FriendsAllowed(on) => c.friends_allowed = *on,
                    ServerChange::Renew(on) => c.renews = *on,
                    ServerChange::AddUser(u) if !c.users.contains(u) => c.users.push(*u),
                    ServerChange::RemoveUser(u) => c.users.retain(|x| x != u),
                    ServerChange::AddUser(_) | ServerChange::NewLink => {}
                },
            );
        self.inner.repaint();
        self.configure_server(account, server_id, change);
    }

    /// The share link of your private server, for copying.
    pub fn server_link(&self, account: AccountId, server_id: u64) -> Result<String, String> {
        let link = self.loaded_config(account, server_id)?.link.clone();
        if link.is_empty() {
            return Err("This server has no link yet. Make a new one first.".into());
        }
        Ok(link)
    }

    fn loaded_config(&self, account: AccountId, server_id: u64) -> Result<Arc<ServerConfig>, String> {
        if account.is_anonymous() {
            return Err("Add an account first.".into());
        }
        self.server_config(account, server_id)
            .data
            .ok_or_else(|| "Still loading. Try again in a moment.".to_owned())
    }

    fn loaded_state(&self, account: AccountId, universe_id: u64) -> Result<Arc<MyGameState>, String> {
        if account.is_anonymous() {
            return Err("Add an account first.".into());
        }
        self.my_game_state(account, universe_id)
            .data
            .ok_or_else(|| "Still loading. Try again in a moment.".to_owned())
    }

    fn set_game_state(&self, account: AccountId, universe_id: u64, f: impl FnOnce(&mut MyGameState)) {
        self.inner
            .res
            .update::<MyGameState>(&Key::private(account, "my_game_state", universe_id), f);
        self.inner.repaint();
    }
}

fn vote_hint(reason: &str) -> String {
    match reason {
        "" => String::new(),
        "PlayGame" => "Play it first to vote.".into(),
        "FloodCheckThresholdMet" => "Too many votes. Try again later.".into(),
        "UserVoteThresholdMet" | "TooManyVotes" => "Vote limit reached. Try again later.".into(),
        "EmailOrPhoneIsVerified" | "EmailIsVerified" => "Verify your email or phone to vote.".into(),
        _ => "You can't vote on this experience.".into(),
    }
}

const NOT_LISTED: &str = "That private server isn't in your list.";

/// The access code of a server in a place's private server list.
fn server_code(p: &Page<PrivateServer>, server_id: u64) -> Option<String> {
    p.items
        .iter()
        .find(|s| s.id == server_id)
        .map(|s| s.access_code.clone())
        .filter(|c| !c.is_empty())
}

fn server_config(id: u64, r: games::v1::VipServerResponse) -> ServerConfig {
    let sub = r.subscription.unwrap_or_default();
    let perms = r.permissions.unwrap_or_default();
    let users = perms
        .users
        .unwrap_or_default()
        .into_iter()
        .filter_map(|u| u.id.map(|i| i as u64))
        .collect();
    ServerConfig {
        id,
        name: r.name.unwrap_or_default(),
        active: r.active.unwrap_or(false),
        link: r.link.unwrap_or_default(),
        friends_allowed: perms.friends_allowed.unwrap_or(false),
        renews: sub.active.unwrap_or(false),
        expires: sub.expiration_date.unwrap_or_default(),
        price: sub.price.unwrap_or(0),
        users,
    }
}

/// Value of the first `name="..."` attribute in `html`.
pub(crate) fn data_attr(html: &str, name: &str) -> Option<String> {
    let pat = format!("{name}=\"");
    let start = html.find(&pat)? + pat.len();
    let end = html[start..].find('"')? + start;
    Some(html[start..end].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn media_keeps_each_image_once() {
        // Shape of games v2 media for a game with a preview video (2026-10-14).
        let items: Vec<games::v2::GameMediaItemResponseV2> = serde_json::from_str(
            r#"[
                {"assetType":"GamePreviewVideo","imageId":11,"videoTitle":null,"approved":true,"videoId":"9"},
                {"assetType":"Image","imageId":11,"approved":true,"altText":""},
                {"assetType":"Image","imageId":12,"approved":true,"altText":"Lobby"},
                {"assetType":"Image","imageId":13,"approved":false},
                {"assetType":"Image","imageId":0,"approved":true}
            ]"#,
        )
        .unwrap();
        let ids: Vec<u64> = media_list(items).iter().map(|m| m.image_id).collect();
        assert_eq!(ids, [11, 12]);
    }

    #[test]
    fn reads_data_attributes() {
        let h = r#"<div id="x" data-private-server-price="100" data-can-create-server="True">"#;
        assert_eq!(data_attr(h, "data-private-server-price").as_deref(), Some("100"));
        assert_eq!(data_attr(h, "data-can-create-server").as_deref(), Some("True"));
        assert_eq!(data_attr(h, "data-missing"), None);
    }

    #[test]
    fn vote_hints_are_plain() {
        assert_eq!(vote_hint("PlayGame"), "Play it first to vote.");
        assert_eq!(vote_hint(""), "");
        assert_eq!(
            vote_hint("EmailOrPhoneIsVerified"),
            "Verify your email or phone to vote."
        );
        assert_eq!(vote_hint("SomethingNew"), "You can't vote on this experience.");
    }
}

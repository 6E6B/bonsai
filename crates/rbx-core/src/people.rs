//! Profiles, friends (connections), followers, friend requests, badges and blocking.
//!
//! Reads are cached resources (see [`crate::res`]); writes are user actions that go through
//! the command registry in the UI and end up in the `*_user` functions here. Nothing in here
//! runs on its own: friend requests are only ever sent on a click.

use std::collections::HashMap;

use rbx_api::generated::{apis, badges, friends, games, groups, presence, users};
use rbx_api::{AccountId, Error};
use serde::{Deserialize, Serialize};

use crate::Core;
use crate::res::{Key, Page, Res};
use crate::store::{Presence, PresenceKind, UserInfo};

/// Profile header data, shared by every viewer.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    pub id: u64,
    pub name: String,
    pub display_name: String,
    pub description: String,
    /// ISO date-time.
    pub created: String,
    pub verified: bool,
    pub banned: bool,
    pub friends: i64,
    pub followers: i64,
    pub following: i64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum FriendState {
    #[default]
    NotFriends,
    Friends,
    RequestSent,
    RequestReceived,
    Unknown,
}

/// How the viewing account relates to a user.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Relation {
    pub friend: FriendState,
    pub following: bool,
    pub followed_by: bool,
    pub blocked: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FriendRequest {
    pub user_id: u64,
    pub name: String,
    pub display_name: String,
    pub sent_at: String,
    pub mutuals: Vec<String>,
    /// Where it was sent from, e.g. "InGame" or "UserProfile".
    pub origin: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct GroupRole {
    pub group_id: u64,
    pub name: String,
    pub members: i64,
    pub verified: bool,
    pub role: String,
    pub rank: i32,
    pub primary: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Badge {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub universe_id: u64,
    pub universe_name: String,
    pub place_id: u64,
    /// Percent of players who earned it (rarity).
    pub win_rate: f64,
    pub awarded: i64,
    pub icon_image_id: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Creation {
    pub universe_id: u64,
    pub place_id: u64,
    pub name: String,
    pub visits: i64,
    pub updated: String,
}

/// What you wrote down about someone. Kept on this computer only, per account.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserNote {
    #[serde(default)]
    pub nickname: String,
    #[serde(default)]
    pub note: String,
}

impl UserNote {
    pub fn is_empty(&self) -> bool {
        self.nickname.is_empty() && self.note.is_empty()
    }
}

/// A user found by name. `requested` is the lowercased name asked for, which can be a past
/// username of theirs.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NamedUser {
    pub id: u64,
    #[serde(default)]
    pub requested: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub display_name: String,
    #[serde(default)]
    pub verified: bool,
}

/// Most names `users_by_names` looks up at once: the endpoint's limit.
const MAX_NAMES: usize = 100;

/// Longest nickname and note kept, in characters.
const NICKNAME_MAX: usize = 32;
const NOTE_MAX: usize = 500;

const PROFILE_AGE: i64 = 600;
const LIST_AGE: i64 = 300;
const RELATION_AGE: i64 = 60;

impl Core {
    /// Profile header for any user.
    pub fn profile(&self, user_id: u64) -> Res<Profile> {
        self.inner.res(
            Key::shared("profile", user_id),
            PROFILE_AGE,
            move |f| async move {
                let id = user_id as i64;
                let c = &f.client;
                let (u, fr, fo, fg) = tokio::join!(
                    users::v1::get_users_by_user_id(c, id),
                    friends::v1::get_users_by_user_id_friends_count(c, id),
                    friends::v1::get_users_by_target_user_id_followers_count(c, id),
                    friends::v1::get_users_by_target_user_id_followings_count(c, id),
                );
                let u = u?;
                Ok(Profile {
                    id: user_id,
                    name: u.name.unwrap_or_default(),
                    display_name: u.display_name.unwrap_or_default(),
                    description: u.description.unwrap_or_default(),
                    created: u.created.unwrap_or_default(),
                    verified: u.has_verified_badge.unwrap_or(false),
                    banned: u.is_banned.unwrap_or(false),
                    friends: fr.ok().and_then(|r| r.count).unwrap_or(-1) as i64,
                    followers: fo.ok().and_then(|r| r.count).unwrap_or(-1),
                    following: fg.ok().and_then(|r| r.count).unwrap_or(-1),
                })
            },
        )
    }

    /// How `account` relates to `user_id` (friend state, follow, block).
    pub fn relation(&self, account: AccountId, user_id: u64) -> Res<Relation> {
        if account.is_anonymous() || account.0 == user_id {
            return Res::default();
        }
        self.inner.res(
            Key::private(account, "relation", user_id),
            RELATION_AGE,
            move |f| async move {
                let c = &f.client;
                let st = friends::v1::GetUsersByUserIdFriendsStatusesParams {
                    user_ids: vec![user_id as i64],
                };
                let body = friends::v1::FollowingExistsRequestModel {
                    target_user_ids: Some(vec![user_id as i64]),
                };
                let (s, fo, bl) = tokio::join!(
                    friends::v1::get_users_by_user_id_friends_statuses(c, account.0 as i64, &st),
                    friends::v1::post_user_following_exists(c, &body),
                    apis::undocumented::get_is_blocked(c, user_id as i64),
                );
                use friends::v1::FriendStatusResponseStatus as S;
                let friend = match s?.data.unwrap_or_default().first().and_then(|x| x.status) {
                    Some(S::Friends) => FriendState::Friends,
                    Some(S::RequestSent) => FriendState::RequestSent,
                    Some(S::RequestReceived) => FriendState::RequestReceived,
                    Some(S::NotFriends) | None => FriendState::NotFriends,
                    Some(_) => FriendState::Unknown,
                };
                let fe = fo
                    .ok()
                    .and_then(|r| r.followings)
                    .unwrap_or_default()
                    .into_iter()
                    .find(|x| x.user_id == Some(user_id as i64));
                Ok(Relation {
                    friend,
                    following: fe.as_ref().and_then(|x| x.is_following).unwrap_or(false),
                    followed_by: fe.as_ref().and_then(|x| x.is_followed).unwrap_or(false),
                    blocked: bl.unwrap_or(false),
                })
            },
        )
    }

    /// Where a user is right now, as `account` can see it. Friends' presence comes from the
    /// friends list refresh; this is for everyone else.
    pub fn presence_of(&self, account: AccountId, user_id: u64) -> Res<Presence> {
        if let Some(p) = self
            .store()
            .account_data(account)
            .and_then(|d| d.presence.get(&user_id).cloned())
        {
            return Res {
                data: Some(std::sync::Arc::new(p)),
                ..Default::default()
            };
        }
        self.inner.res(
            Key::private(account, "presence", user_id),
            RELATION_AGE,
            move |f| async move {
                let body = presence::v1::UserPresenceRequest {
                    user_ids: Some(vec![user_id as i64]),
                };
                let r = presence::v1::post_presence_users(&f.client, &body).await?;
                let p = r
                    .user_presences
                    .unwrap_or_default()
                    .into_iter()
                    .find_map(Presence::from_api)
                    .map(|(_, p)| p)
                    .unwrap_or_default();
                f.inner.note_presence(account, &[(user_id, p.kind)]);
                Ok(p)
            },
        )
    }

    /// Asks where a user is right now and says so in a toast. Presence changes by the minute,
    /// so this skips the cache, then marks it stale so an open profile picks up the answer.
    pub fn find_user(&self, account: AccountId, user_id: u64) {
        let inner = self.inner.clone();
        let Some(client) = inner.client(account) else {
            self.notice("No account to ask with.", true);
            return;
        };
        inner.store.write().jobs += 1;
        self.inner.rt.spawn(async move {
            let body = presence::v1::UserPresenceRequest {
                user_ids: Some(vec![user_id as i64]),
            };
            match presence::v1::post_presence_users(&client, &body).await {
                Ok(r) => {
                    let p = r
                        .user_presences
                        .unwrap_or_default()
                        .into_iter()
                        .find_map(Presence::from_api)
                        .map(|(_, p)| p)
                        .unwrap_or_default();
                    inner.note_presence(account, &[(user_id, p.kind)]);
                    let text = {
                        let s = inner.store.read();
                        let name = s
                            .users
                            .get(&user_id)
                            .map(|u| u.display_name.clone())
                            .filter(|n| !n.is_empty())
                            .unwrap_or_else(|| format!("User {user_id}"));
                        let game = p
                            .universe_id
                            .and_then(|u| s.games.get(&u))
                            .map(|g| g.name.as_str());
                        whereabouts(&name, &p, game)
                    };
                    inner.toast(Some(account), text, false);
                    inner.res.invalidate(account, "presence");
                }
                Err(e) => {
                    inner.note_error(account, &e);
                    inner.toast(
                        Some(account),
                        format!("Could not find them: {}", e.user_message()),
                        true,
                    );
                }
            }
            {
                let mut s = inner.store.write();
                s.jobs = s.jobs.saturating_sub(1);
            }
            inner.repaint();
        });
    }

    /// Pending friend requests to `account`, newest first.
    pub fn friend_requests(&self, account: AccountId, more: bool) -> Res<Page<FriendRequest>> {
        self.inner.paged(
            Key::private(account, "friend_requests", ""),
            LIST_AGE,
            more,
            move |f, cursor| async move {
                let p = friends::v1::GetMyFriendsRequestsParams {
                    limit: Some(50),
                    cursor,
                    ..Default::default()
                };
                let r = friends::v1::get_my_friends_requests(&f.client, &p).await?;
                let items: Vec<FriendRequest> = r
                    .data
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|x| {
                        let fr = x.friend_request.unwrap_or_default();
                        Some(FriendRequest {
                            user_id: x.id.or(fr.sender_id)? as u64,
                            name: x.name.unwrap_or_default(),
                            display_name: x.display_name.unwrap_or_default(),
                            sent_at: fr.sent_at.unwrap_or_default(),
                            mutuals: x.mutual_friends_list.unwrap_or_default(),
                            origin: fr
                                .origin_source_type
                                .and_then(|o| o.name())
                                .map(str::to_string)
                                .unwrap_or_default(),
                        })
                    })
                    .collect();
                f.note_users(items.iter().filter(|x| !x.name.is_empty()).map(|x| UserInfo {
                    id: x.user_id,
                    name: x.name.clone(),
                    display_name: x.display_name.clone(),
                    verified: false,
                }));
                Ok(Page {
                    items,
                    next: r.next_page_cursor,
                })
            },
        )
    }

    /// Friends of any user (ids; names come from [`Core::want_users`]).
    pub fn friends_of(&self, user_id: u64, more: bool) -> Res<Page<u64>> {
        self.inner.paged(
            Key::shared("friends_of", user_id),
            LIST_AGE,
            more,
            move |f, cursor| async move {
                let p = friends::v1::GetUsersByUserIdFriendsFindParams {
                    limit: Some(50),
                    cursor,
                    ..Default::default()
                };
                let r = friends::v1::get_users_by_user_id_friends_find(&f.client, user_id as i64, &p).await?;
                Ok(Page {
                    items: ids(r.page_items.unwrap_or_default().into_iter().map(|x| x.id)),
                    next: r.next_cursor.filter(|c| !c.is_empty()),
                })
            },
        )
    }

    /// Friends of any user whose names match `query`, searched by Roblox so it finds people on
    /// pages not loaded yet. Needs a signed-in viewer.
    pub fn search_friends_of(
        &self,
        viewer: AccountId,
        user_id: u64,
        query: &str,
        more: bool,
    ) -> Res<Page<u64>> {
        let q = query.trim().to_lowercase();
        self.inner.paged(
            Key::private(viewer, "friends_search", format!("{user_id}:{q}")),
            LIST_AGE,
            more,
            move |f, cursor| {
                let q = q.clone();
                async move {
                    let p = friends::v1::GetUsersByUserIdFriendsSearchParams {
                        query: Some(q),
                        limit: Some(50),
                        cursor,
                    };
                    let r = friends::v1::get_users_by_user_id_friends_search(&f.client, user_id as i64, &p)
                        .await?;
                    Ok(Page {
                        items: ids(r.page_items.unwrap_or_default().into_iter().map(|x| x.id)),
                        next: r.next_cursor.filter(|c| !c.is_empty()),
                    })
                }
            },
        )
    }

    /// Roblox only serves followers, followings and badges to a signed-in viewer.
    pub fn followers(&self, viewer: AccountId, user_id: u64, more: bool) -> Res<Page<u64>> {
        self.inner.paged(
            Key::private(viewer, "followers", user_id),
            LIST_AGE,
            more,
            move |f, cursor| async move {
                let p = friends::v1::GetUsersByTargetUserIdFollowersParams {
                    limit: Some(100),
                    cursor,
                    ..Default::default()
                };
                let r =
                    friends::v1::get_users_by_target_user_id_followers(&f.client, user_id as i64, &p).await?;
                Ok(Page {
                    items: ids(r.data.unwrap_or_default().into_iter().map(|x| x.id)),
                    next: r.next_page_cursor,
                })
            },
        )
    }

    pub fn followings(&self, viewer: AccountId, user_id: u64, more: bool) -> Res<Page<u64>> {
        self.inner.paged(
            Key::private(viewer, "followings", user_id),
            LIST_AGE,
            more,
            move |f, cursor| async move {
                let p = friends::v1::GetUsersByTargetUserIdFollowingsParams {
                    limit: Some(100),
                    cursor,
                    ..Default::default()
                };
                let r = friends::v1::get_users_by_target_user_id_followings(&f.client, user_id as i64, &p)
                    .await?;
                Ok(Page {
                    items: ids(r.data.unwrap_or_default().into_iter().map(|x| x.id)),
                    next: r.next_page_cursor,
                })
            },
        )
    }

    pub fn username_history(&self, user_id: u64) -> Res<Vec<String>> {
        self.inner.res(
            Key::shared("past_names", user_id),
            PROFILE_AGE,
            move |f| async move {
                let p = users::v1::GetUsersByUserIdUsernameHistoryParams {
                    limit: Some(50),
                    ..Default::default()
                };
                let r =
                    users::v1::get_users_by_user_id_username_history(&f.client, user_id as i64, &p).await?;
                Ok(r.data
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|x| x.name)
                    .collect())
            },
        )
    }

    /// Communities a user is in, primary first, then by rank.
    pub fn user_groups(&self, user_id: u64) -> Res<Vec<GroupRole>> {
        self.inner.res(
            Key::shared("user_groups", user_id),
            PROFILE_AGE,
            move |f| async move {
                let p = groups::v1::GetUsersByUserIdGroupsRolesParams::default();
                let r = groups::v1::get_users_by_user_id_groups_roles(&f.client, user_id as i64, &p).await?;
                let mut v: Vec<GroupRole> = r
                    .data
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|m| {
                        let g = m.group?;
                        let role = m.role.unwrap_or_default();
                        Some(GroupRole {
                            group_id: g.id? as u64,
                            name: g.name.unwrap_or_default(),
                            members: g.member_count.unwrap_or(0) as i64,
                            verified: g.has_verified_badge.unwrap_or(false),
                            role: role.name.unwrap_or_default(),
                            rank: role.rank.unwrap_or(0),
                            primary: m.is_primary_group.unwrap_or(false),
                        })
                    })
                    .collect();
                v.sort_by_key(|g| (!g.primary, -g.rank, -g.members));
                Ok(v)
            },
        )
    }

    pub fn user_badges(&self, viewer: AccountId, user_id: u64, more: bool) -> Res<Page<Badge>> {
        self.inner.paged(
            Key::private(viewer, "user_badges", user_id),
            LIST_AGE,
            more,
            move |f, cursor| async move {
                let p = badges::v1::GetUsersByUserIdBadgesParams {
                    limit: Some(100),
                    cursor,
                    sort_order: Some(badges::v1::GetUniversesByUniverseIdBadgesSortOrder::Desc),
                };
                let r = badges::v1::get_users_by_user_id_badges(&f.client, user_id as i64, &p).await?;
                Ok(Page {
                    items: r.data.unwrap_or_default().into_iter().filter_map(badge).collect(),
                    next: r.next_page_cursor,
                })
            },
        )
    }

    /// Public experiences a user made.
    pub fn user_creations(&self, user_id: u64) -> Res<Vec<Creation>> {
        self.inner.res(
            Key::shared("creations", user_id),
            PROFILE_AGE,
            move |f| async move {
                let p = games::v2::GetUsersByUserIdGamesParams {
                    limit: Some(50),
                    ..Default::default()
                };
                let r = games::v2::get_users_by_user_id_games(&f.client, user_id as i64, &p).await?;
                Ok(r.data
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|g| {
                        Some(Creation {
                            universe_id: g.id? as u64,
                            place_id: g.root_place.and_then(|p| p.id).unwrap_or(0) as u64,
                            name: g.name.unwrap_or_default(),
                            visits: g.place_visits.unwrap_or(0),
                            updated: g.updated.unwrap_or_default(),
                        })
                    })
                    .collect())
            },
        )
    }

    /// Which of `users` `account` follows, asked in one batch. Callers pass a page of a list
    /// at a time, so each page keeps its own entry.
    pub fn follows(&self, account: AccountId, users: &[u64]) -> Res<HashMap<u64, bool>> {
        let users: Vec<u64> = users.iter().copied().filter(|u| *u != account.0).collect();
        if account.is_anonymous() || users.is_empty() {
            return Res::default();
        }
        let key = users.iter().map(u64::to_string).collect::<Vec<_>>().join(",");
        self.inner.res(
            Key::private(account, "follows", key),
            RELATION_AGE,
            move |f| async move {
                let body = friends::v1::FollowingExistsRequestModel {
                    target_user_ids: Some(users.iter().map(|u| *u as i64).collect()),
                };
                let r = friends::v1::post_user_following_exists(&f.client, &body).await?;
                Ok(r.followings
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|x| Some((x.user_id? as u64, x.is_following.unwrap_or(false))))
                    .collect())
            },
        )
    }

    /// Nicknames and notes `account` keeps, by user id.
    pub fn user_notes(&self, account: AccountId) -> HashMap<u64, UserNote> {
        self.local::<HashMap<String, UserNote>>(account, "user_notes")
            .into_iter()
            .filter_map(|(k, v)| Some((k.parse().ok()?, v)))
            .collect()
    }

    pub fn user_note(&self, account: AccountId, user_id: u64) -> UserNote {
        self.user_notes(account).remove(&user_id).unwrap_or_default()
    }

    /// Call someone by another name in this app. Empty clears it.
    pub fn set_nickname(&self, account: AccountId, user_id: u64, nickname: &str) {
        self.edit_note(account, user_id, |n| n.nickname = clip(nickname, NICKNAME_MAX));
    }

    /// Keep a private note about someone. Empty clears it.
    pub fn set_note(&self, account: AccountId, user_id: u64, note: &str) {
        self.edit_note(account, user_id, |n| n.note = clip(note, NOTE_MAX));
    }

    fn edit_note(&self, account: AccountId, user_id: u64, f: impl FnOnce(&mut UserNote)) {
        let mut all: HashMap<String, UserNote> = self.local(account, "user_notes");
        let k = user_id.to_string();
        let mut n = all.remove(&k).unwrap_or_default();
        f(&mut n);
        if !n.is_empty() {
            all.insert(k, n);
        }
        self.set_local(account, "user_notes", &all);
    }

    /// Users `account` has blocked.
    pub fn blocked_users(&self, account: AccountId) -> Res<Vec<u64>> {
        self.inner.res(
            Key::private(account, "blocked", ""),
            LIST_AGE,
            move |f| async move {
                let mut out = Vec::new();
                let mut cursor = None;
                loop {
                    let p = apis::undocumented::GetBlockedUsersParams { cursor, count: 49 };
                    let r = apis::undocumented::get_blocked_users(&f.client, &p).await?;
                    let d = r.data.unwrap_or_default();
                    out.extend(ids(d.blocked_user_ids.unwrap_or_default().into_iter().map(Some)));
                    cursor = d.cursor.filter(|c| !c.is_empty());
                    if cursor.is_none() || out.len() > 10_000 {
                        break;
                    }
                }
                Ok(out)
            },
        )
    }

    /// The users `names` name (case-insensitive, `@` optional), in one request. Names nobody
    /// has are left out. Used to resolve `@mentions` in descriptions and bios.
    pub fn users_by_names(&self, names: &[String]) -> Res<Vec<NamedUser>> {
        let mut names: Vec<String> = names
            .iter()
            .map(|n| n.trim().trim_start_matches('@').to_lowercase())
            .filter(|n| !n.is_empty())
            .collect();
        names.sort();
        names.dedup();
        names.truncate(MAX_NAMES);
        if names.is_empty() {
            return Res {
                data: Some(std::sync::Arc::new(Vec::new())),
                ..Res::default()
            };
        }
        self.inner.res(
            Key::shared("users_by_names", names.join(",")),
            PROFILE_AGE,
            move |f| async move {
                let body = users::v1::MultiGetByUsernameRequest {
                    usernames: Some(names),
                    exclude_banned_users: Some(true),
                };
                let r = users::v1::post_usernames_users(&f.client, &body).await?;
                Ok(r.data
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|u| {
                        Some(NamedUser {
                            id: u.id? as u64,
                            requested: u.requested_username.unwrap_or_default().to_lowercase(),
                            name: u.name.unwrap_or_default(),
                            display_name: u.display_name.unwrap_or_default(),
                            verified: u.has_verified_badge.unwrap_or(false),
                        })
                    })
                    .collect())
            },
        )
    }

    /// The id of the user named `name` (case-insensitive). Errors when nobody has that name.
    pub fn user_by_name(&self, name: &str) -> Res<u64> {
        let name = name.trim().trim_start_matches('@').trim().to_string();
        if let Some(u) = self
            .store()
            .users
            .values()
            .find(|u| u.name.eq_ignore_ascii_case(&name))
        {
            return Res {
                data: Some(std::sync::Arc::new(u.id)),
                ..Res::default()
            };
        }
        self.inner.res(
            Key::shared("user_by_name", name.to_lowercase()),
            PROFILE_AGE,
            move |f| async move {
                let body = users::v1::MultiGetByUsernameRequest {
                    usernames: Some(vec![name.clone()]),
                    exclude_banned_users: Some(false),
                };
                let r = users::v1::post_usernames_users(&f.client, &body).await?;
                r.data
                    .unwrap_or_default()
                    .into_iter()
                    .find_map(|u| u.id)
                    .map(|id| id as u64)
                    .ok_or_else(|| Error::Message(format!("No user is named {name}.")))
            },
        )
    }

    /// Runs `f` with the user `who` names: an id, or a username (current or past, with or
    /// without `@`). Names not seen yet are looked up first; a miss shows a toast.
    pub fn with_user(&self, who: &str, f: impl FnOnce(&Core, u64) + Send + 'static) -> Result<(), String> {
        let who = who.trim().trim_start_matches('@').trim();
        if who.is_empty() {
            return Err("Which user?".into());
        }
        if let Ok(id) = who.parse::<u64>() {
            f(self, id);
            return Ok(());
        }
        let known = self
            .store()
            .users
            .values()
            .find(|u| u.name.eq_ignore_ascii_case(who))
            .map(|u| u.id);
        if let Some(id) = known {
            f(self, id);
            return Ok(());
        }
        let core = self.handle();
        let name = who.to_string();
        self.inner.rt.spawn(async move {
            let body = users::v1::MultiGetByUsernameRequest {
                usernames: Some(vec![name.clone()]),
                exclude_banned_users: Some(false),
            };
            match users::v1::post_usernames_users(&core.inner.anon, &body).await {
                Ok(r) => match r.data.unwrap_or_default().into_iter().find_map(|u| u.id) {
                    Some(id) => f(&core, id as u64),
                    None => core.notice(format!("No user is named {name}."), true),
                },
                Err(e) => core.notice(format!("Could not look up {name}: {}", e.user_message()), true),
            }
        });
        Ok(())
    }

    // ---- user actions (one click each) ----

    pub fn add_friend(&self, account: AccountId, user_id: u64) {
        self.inner.mutate(
            account,
            "send the friend request",
            "Friend request sent",
            &["relation", "profile"],
            move |f| async move {
                let body = friends::undocumented::FriendshipRequest {
                    friendship_origin_source_type: Some("UserProfile".into()),
                };
                let r =
                    friends::undocumented::post_request_friendship(&f.client, user_id as i64, &body).await?;
                if r.is_captcha_required == Some(true) {
                    return Err(captcha());
                }
                Ok(())
            },
        );
    }

    pub fn unfriend(&self, account: AccountId, user_id: u64) {
        self.inner.forget_friend(account, user_id);
        self.inner.mutate(
            account,
            "remove the friend",
            "Removed from friends",
            &["relation", "profile", "friends_of"],
            move |f| async move {
                friends::undocumented::post_unfriend(&f.client, user_id as i64).await?;
                Ok(())
            },
        );
    }

    pub fn accept_friend(&self, account: AccountId, user_id: u64) {
        self.drop_request(account, user_id);
        self.inner.mutate(
            account,
            "accept the request",
            "Friend request accepted",
            &["relation", "profile", "friend_requests", "friends_of"],
            move |f| async move {
                friends::undocumented::post_accept_friend_request(&f.client, user_id as i64).await?;
                Ok(())
            },
        );
    }

    pub fn decline_friend(&self, account: AccountId, user_id: u64) {
        self.drop_request(account, user_id);
        self.inner.mutate(
            account,
            "decline the request",
            "",
            &["relation", "friend_requests"],
            move |f| async move {
                friends::undocumented::post_decline_friend_request(&f.client, user_id as i64).await?;
                Ok(())
            },
        );
    }

    pub fn decline_all_friends(&self, account: AccountId) {
        self.inner
            .res
            .update::<Page<FriendRequest>>(&Key::private(account, "friend_requests", ""), |p| {
                p.items.clear();
                p.next = None;
            });
        self.inner.mutate(
            account,
            "decline the requests",
            "All friend requests declined",
            &["relation", "friend_requests"],
            move |f| async move {
                friends::undocumented::post_decline_all_friend_requests(&f.client).await?;
                Ok(())
            },
        );
    }

    pub fn follow(&self, account: AccountId, user_id: u64) {
        self.set_relation(account, user_id, |r| r.following = true);
        self.set_follows(account, user_id, true);
        self.inner.mutate(
            account,
            "follow",
            "",
            &["relation", "profile", "followings"],
            move |f| async move {
                let r = friends::v1::post_users_by_target_user_id_follow(
                    &f.client,
                    user_id as i64,
                    &friends::v1::CaptchaTokenRequest::default(),
                )
                .await?;
                if r.is_captcha_required == Some(true) {
                    return Err(captcha());
                }
                Ok(())
            },
        );
    }

    pub fn unfollow(&self, account: AccountId, user_id: u64) {
        self.set_relation(account, user_id, |r| r.following = false);
        self.set_follows(account, user_id, false);
        self.inner.mutate(
            account,
            "unfollow",
            "",
            &["relation", "profile", "followings"],
            move |f| async move {
                friends::v1::post_users_by_target_user_id_unfollow(&f.client, user_id as i64).await?;
                Ok(())
            },
        );
    }

    pub fn block(&self, account: AccountId, user_id: u64) {
        self.set_relation(account, user_id, |r| r.blocked = true);
        self.inner.mutate(
            account,
            "block",
            "Blocked",
            &["relation", "blocked"],
            move |f| async move {
                apis::undocumented::post_block_user(&f.client, user_id as i64).await?;
                Ok(())
            },
        );
    }

    pub fn unblock(&self, account: AccountId, user_id: u64) {
        self.set_relation(account, user_id, |r| r.blocked = false);
        self.inner
            .res
            .update::<Vec<u64>>(&Key::private(account, "blocked", ""), |v| {
                v.retain(|x| *x != user_id)
            });
        self.inner.mutate(
            account,
            "unblock",
            "Unblocked",
            &["relation", "blocked"],
            move |f| async move {
                apis::undocumented::post_unblock_user(&f.client, user_id as i64).await?;
                Ok(())
            },
        );
    }

    fn drop_request(&self, account: AccountId, user_id: u64) {
        self.inner
            .res
            .update::<Page<FriendRequest>>(&Key::private(account, "friend_requests", ""), |p| {
                p.items.retain(|r| r.user_id != user_id)
            });
    }

    fn set_follows(&self, account: AccountId, user_id: u64, on: bool) {
        self.inner
            .res
            .update_kind::<HashMap<u64, bool>>(account, "follows", |m| {
                if let Some(v) = m.get_mut(&user_id) {
                    *v = on;
                }
            });
    }

    fn set_relation(&self, account: AccountId, user_id: u64, f: impl FnOnce(&mut Relation)) {
        self.inner
            .res
            .update::<Relation>(&Key::private(account, "relation", user_id), f);
    }
}

/// Someone's friends with the viewer's own friends (`mine`) moved to the front, each part in
/// the order Roblox sent. Also returns how many are mutual.
pub fn mutuals_first(ids: &[u64], mine: &std::collections::HashSet<u64>) -> (Vec<u64>, usize) {
    let (mut out, rest): (Vec<u64>, Vec<u64>) = ids.iter().partition(|id| mine.contains(id));
    let n = out.len();
    out.extend(rest);
    (out, n)
}

/// `s` trimmed to at most `max` characters.
fn clip(s: &str, max: usize) -> String {
    s.trim()
        .chars()
        .take(max)
        .collect::<String>()
        .trim_end()
        .to_string()
}

fn ids(it: impl Iterator<Item = Option<i64>>) -> Vec<u64> {
    it.flatten().filter(|i| *i > 0).map(|i| i as u64).collect()
}

fn badge(b: badges::v1::GetBadgesByUserResponse) -> Option<Badge> {
    let u = b.awarder.unwrap_or_default();
    let st = b.statistics.unwrap_or_default();
    Some(Badge {
        id: b.id? as u64,
        name: b.display_name.or(b.name).unwrap_or_default(),
        description: b.display_description.or(b.description).unwrap_or_default(),
        // The awarder is the place that awards it.
        universe_id: 0,
        universe_name: String::new(),
        place_id: u.id.unwrap_or(0) as u64,
        win_rate: st.win_rate_percentage.unwrap_or(0.0),
        awarded: st.awarded_count.unwrap_or(0),
        icon_image_id: b.display_icon_image_id.or(b.icon_image_id).unwrap_or(0) as u64,
    })
}

fn captcha() -> Error {
    Error::Message("Roblox wants a captcha for this. Do it once on the website, then try again.".into())
}

/// One line saying where a user is. `game` is the experience's name when it is cached.
fn whereabouts(name: &str, p: &Presence, game: Option<&str>) -> String {
    match p.kind {
        PresenceKind::InGame => {
            let game = game
                .or_else(|| (!p.last_location.is_empty()).then_some(p.last_location.as_str()))
                .unwrap_or("an experience");
            if p.job_id.is_some() {
                format!("{name} is in {game}. Use Join to follow them.")
            } else {
                format!("{name} is in {game}. Their server is hidden from you.")
            }
        }
        PresenceKind::InStudio => format!("{name} is in Studio."),
        PresenceKind::Online => format!("{name} is online, not in an experience."),
        PresenceKind::Offline | PresenceKind::Invisible => format!("{name} is offline."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutuals_go_first_in_order() {
        let mine = [3, 5, 9].into_iter().collect();
        assert_eq!(mutuals_first(&[1, 5, 2, 3, 4], &mine), (vec![5, 3, 1, 2, 4], 2));
        assert_eq!(mutuals_first(&[1, 2], &mine), (vec![1, 2], 0));
        assert_eq!(mutuals_first(&[], &mine), (vec![], 0));
    }

    #[test]
    fn clips_notes() {
        assert_eq!(clip("  hi  ", 10), "hi");
        assert_eq!(clip("abcdef", 3), "abc");
        assert_eq!(clip("ab cd", 3), "ab");
        assert_eq!(clip("ééé", 2), "éé");
    }

    #[test]
    fn whereabouts_lines() {
        let mut p = Presence {
            kind: PresenceKind::InGame,
            last_location: "Obby".into(),
            ..Default::default()
        };
        assert_eq!(
            whereabouts("Ann", &p, None),
            "Ann is in Obby. Their server is hidden from you."
        );
        p.job_id = Some("abc".into());
        assert_eq!(
            whereabouts("Ann", &p, Some("Tower")),
            "Ann is in Tower. Use Join to follow them."
        );
        p.last_location.clear();
        assert!(whereabouts("Ann", &p, None).contains("an experience"));
        p.kind = PresenceKind::Invisible;
        assert_eq!(whereabouts("Ann", &p, None), "Ann is offline.");
    }
}

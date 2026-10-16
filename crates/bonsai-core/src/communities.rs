//! Communities (groups): details and shout, your membership and permissions, members by role,
//! forums (they replaced the wall), experiences, store, affiliates, and the admin views (join
//! requests, audit log, settings, funds and payouts).
//!
//! Public data (details, roles, experiences, store, affiliates) is shared by every account;
//! anything Roblox answers differently per viewer (membership, members, forums, admin views) is
//! keyed by the viewing account. Every write is one user action and confirmed in the UI where it
//! can't be undone or affects someone else.

use std::collections::HashMap;

use bonsai_api::AccountId;
use bonsai_api::generated::{catalog, economy, games, groups};
use serde::{Deserialize, Serialize};

use crate::Core;
use crate::market::CatalogItem;
use crate::people::Creation;
use crate::res::{Key, Page, Res};
use crate::store::UserInfo;

const DETAIL_AGE: i64 = 300;
const LIST_AGE: i64 = 300;
const ADMIN_AGE: i64 = 60;
const FORUM_AGE: i64 = 120;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Community {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub owner_id: u64,
    pub owner: String,
    pub owner_verified: bool,
    pub members: i64,
    pub verified: bool,
    /// Anyone can join without approval.
    pub public_entry: bool,
    /// Locked or deactivated by Roblox.
    pub locked: bool,
    pub shout: Option<Shout>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Shout {
    pub text: String,
    pub poster_id: u64,
    pub poster: String,
    #[serde(default)]
    pub poster_username: String,
    #[serde(default)]
    pub poster_verified: bool,
    /// ISO date-time of the last change.
    pub updated: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Role {
    pub id: u64,
    pub name: String,
    pub rank: i32,
    pub members: i64,
    pub description: String,
}

/// What the viewing account may do in a community.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Perms {
    pub post_shout: bool,
    pub view_shout: bool,
    pub change_rank: bool,
    pub remove_members: bool,
    pub ban_members: bool,
    /// Accept and decline join requests.
    pub invite_members: bool,
    pub view_audit_log: bool,
    pub manage_relationships: bool,
    pub spend_funds: bool,
    pub view_payouts: bool,
    pub manage_settings: bool,
    pub view_forums: bool,
    pub create_posts: bool,
    pub create_comments: bool,
    pub remove_posts: bool,
    pub remove_comments: bool,
}

impl Perms {
    /// Anything an admin tab shows.
    pub fn any_admin(&self) -> bool {
        self.invite_members || self.view_audit_log || self.manage_settings || self.view_payouts
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Membership {
    /// Your role, `None` when you're not a member.
    pub role: Option<Role>,
    pub primary: bool,
    /// You asked to join and wait for approval.
    pub pending: bool,
    pub owner: bool,
    pub can_configure: bool,
    pub can_view_members: bool,
    pub funds_visible: bool,
    pub games_visible: bool,
    pub perms: Perms,
}

impl Membership {
    pub fn is_member(&self) -> bool {
        self.role.as_ref().is_some_and(|r| r.rank > 0)
    }
}

/// A community in a list (search, affiliates).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CommunityRow {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub members: i64,
    pub verified: bool,
    pub public_entry: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Member {
    pub user_id: u64,
    pub name: String,
    pub display_name: String,
    pub verified: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct JoinRequest {
    pub user_id: u64,
    pub name: String,
    pub display_name: String,
    /// ISO date-time.
    pub created: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AuditEntry {
    pub actor_id: u64,
    pub actor: String,
    pub actor_role: String,
    pub action: String,
    /// What happened, in words.
    pub text: String,
    /// ISO date-time.
    pub created: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub approval_required: bool,
    pub enemies_allowed: bool,
    pub funds_visible: bool,
    pub games_visible: bool,
}

/// One community setting, for [`Core::set_community_setting`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Setting {
    ApprovalRequired,
    EnemiesAllowed,
    FundsVisible,
    GamesVisible,
}

impl Setting {
    pub const ALL: [Setting; 4] = [
        Setting::ApprovalRequired,
        Setting::EnemiesAllowed,
        Setting::FundsVisible,
        Setting::GamesVisible,
    ];
    pub fn key(self) -> &'static str {
        match self {
            Setting::ApprovalRequired => "approval",
            Setting::EnemiesAllowed => "enemies",
            Setting::FundsVisible => "funds",
            Setting::GamesVisible => "games",
        }
    }
    pub fn from_key(k: &str) -> Option<Setting> {
        Setting::ALL.into_iter().find(|s| s.key() == k)
    }
    pub fn label(self) -> &'static str {
        match self {
            Setting::ApprovalRequired => "Manual approval to join",
            Setting::EnemiesAllowed => "Allow enemy declarations",
            Setting::FundsVisible => "Show funds to everyone",
            Setting::GamesVisible => "Show experiences on the community page",
        }
    }
    pub fn get(self, s: &Settings) -> bool {
        match self {
            Setting::ApprovalRequired => s.approval_required,
            Setting::EnemiesAllowed => s.enemies_allowed,
            Setting::FundsVisible => s.funds_visible,
            Setting::GamesVisible => s.games_visible,
        }
    }
    fn set(self, s: &mut Settings, v: bool) {
        match self {
            Setting::ApprovalRequired => s.approval_required = v,
            Setting::EnemiesAllowed => s.enemies_allowed = v,
            Setting::FundsVisible => s.funds_visible = v,
            Setting::GamesVisible => s.games_visible = v,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Payout {
    pub user_id: u64,
    pub name: String,
    pub percent: i32,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SocialLink {
    pub title: String,
    pub url: String,
    pub kind: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Affiliates {
    pub allies: Vec<CommunityRow>,
    pub enemies: Vec<CommunityRow>,
}

/// Ally or enemy, for [`Core::add_affiliate`] and [`Core::remove_affiliate`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Affiliation {
    Ally,
    Enemy,
}

impl Affiliation {
    fn api(self) -> &'static str {
        match self {
            Affiliation::Ally => "Allies",
            Affiliation::Enemy => "Enemies",
        }
    }
}

/// Checks a recurring payout split: each share 1 to 100 percent, everyone once, 100 in all at
/// most. An empty split stops recurring payouts.
pub fn check_split(split: &[(u64, i32)]) -> Result<(), String> {
    if split.iter().any(|(_, p)| !(1..=100).contains(p)) {
        return Err("Each share is 1 to 100 percent.".into());
    }
    let mut ids: Vec<u64> = split.iter().map(|(u, _)| *u).collect();
    ids.sort_unstable();
    ids.dedup();
    if ids.len() != split.len() {
        return Err("List each member once.".into());
    }
    let total: i32 = split.iter().map(|(_, p)| p).sum();
    if total > 100 {
        return Err(format!("The shares add up to {total}%. Keep it at 100% or less."));
    }
    Ok(())
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ForumCategory {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ForumPost {
    pub id: String,
    pub category_id: String,
    pub title: String,
    pub author_id: u64,
    pub author: String,
    pub author_role: String,
    pub created: String,
    /// ISO date-time of the latest activity on the thread.
    #[serde(default)]
    pub updated: String,
    /// Replies, not counting the post's own text.
    pub replies: i64,
    pub pinned: bool,
    pub locked: bool,
    pub unread: bool,
    pub text: String,
    /// The comment that holds the post's own text (replies point at it).
    pub first_comment: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ForumComment {
    pub id: String,
    pub author_id: u64,
    pub author: String,
    pub author_role: String,
    pub created: String,
    pub text: String,
}

/// Your accounts that are in a community, with their roles (cross-account view).
#[derive(Clone, Debug, PartialEq)]
pub struct AccountMembership {
    pub account: AccountId,
    pub role: String,
    pub rank: i32,
    pub primary: bool,
}

fn row(g: groups::v1::GroupDetailResponse) -> Option<CommunityRow> {
    Some(CommunityRow {
        id: g.id.filter(|i| *i > 0)? as u64,
        name: g.name.unwrap_or_default(),
        description: g.description.unwrap_or_default(),
        members: g.member_count.unwrap_or(0) as i64,
        verified: g.has_verified_badge.unwrap_or(false),
        public_entry: g.public_entry_allowed.unwrap_or(false),
    })
}

fn role(r: groups::v1::GroupRoleResponse) -> Option<Role> {
    Some(Role {
        id: r.id.filter(|i| *i > 0)? as u64,
        name: r.name.unwrap_or_default(),
        rank: r.rank.unwrap_or(0),
        members: r.member_count.unwrap_or(0),
        description: r.description.unwrap_or_default(),
    })
}

fn user_of(u: Option<groups::v1::UserModel>) -> (u64, String, String, bool) {
    let u = u.unwrap_or_default();
    (
        u.user_id.unwrap_or(0).max(0) as u64,
        u.username.unwrap_or_default(),
        u.display_name.unwrap_or_default(),
        u.has_verified_badge.unwrap_or(false),
    )
}

fn forum_comment(c: groups::undocumented::ForumComment) -> Option<ForumComment> {
    let who = c.creator_info.unwrap_or_default();
    Some(ForumComment {
        id: c.id.filter(|i| !i.is_empty())?,
        author_id: c.created_by.unwrap_or(0).max(0) as u64,
        author: who.display_name.unwrap_or_default(),
        author_role: who.group_role_name.unwrap_or_default(),
        created: c.created_at.unwrap_or_default(),
        text: c.content.and_then(|x| x.plain_text).unwrap_or_default(),
    })
}

fn forum_post(p: groups::undocumented::ForumPost) -> Option<ForumPost> {
    let first = p.first_comment.and_then(forum_comment).unwrap_or_default();
    let created = p.created_at.unwrap_or_else(|| first.created.clone());
    let updated = p
        .updated_at
        .filter(|u| !u.is_empty())
        .unwrap_or_else(|| created.clone());
    Some(ForumPost {
        id: p.id.filter(|i| !i.is_empty())?,
        category_id: p.category_id.unwrap_or_default(),
        title: p.name.unwrap_or_default(),
        author_id: p.created_by.map_or(first.author_id, |c| c.max(0) as u64),
        author: first.author.clone(),
        author_role: first.author_role.clone(),
        created,
        updated,
        replies: (p.comment_count.unwrap_or(1) as i64 - 1).max(0),
        pinned: p.is_pinned.unwrap_or(false),
        locked: p.is_locked.unwrap_or(false),
        unread: p.is_unread.unwrap_or(false),
        text: first.text,
        first_comment: first.id,
    })
}

/// The audit log's description in words, e.g. "changed Ann's rank from Member to Admin".
pub fn describe_audit(action: &str, d: &serde_json::Value) -> String {
    let s = |k: &str| {
        d.get(k)
            .map(|v| match v {
                serde_json::Value::String(s) => s.clone(),
                other => other.to_string(),
            })
            .unwrap_or_default()
    };
    let target = s("TargetName");
    match action {
        "ChangeRank" => format!(
            "changed {target}'s rank from {} to {}",
            s("OldRoleSetName"),
            s("NewRoleSetName")
        ),
        "AcceptJoinRequest" => format!("accepted {target}'s join request"),
        "DeclineJoinRequest" => format!("declined {target}'s join request"),
        "RemoveMember" => format!("kicked {target}"),
        "BanMember" => format!("banned {target}"),
        "UnbanMember" => format!("unbanned {target}"),
        "PostStatus" => format!("posted a shout: “{}”", s("Text")),
        "DeletePost" => format!("deleted a post by {}", s("TargetName")),
        "SpendGroupFunds" => format!("spent R$ {} on {}", s("Amount"), s("ItemDescription")),
        "UpdateGroupDescription" => "changed the description".into(),
        "Rename" => format!("renamed the community to {}", s("NewName")),
        "SendAllyRequest" => format!("sent an ally request to {}", s("TargetGroupName")),
        "AcceptAllyRequest" => format!("accepted the ally request from {}", s("TargetGroupName")),
        "DeleteAlly" => format!("removed the ally {}", s("TargetGroupName")),
        "CreateEnemy" => format!("declared {} an enemy", s("TargetGroupName")),
        "DeleteEnemy" => format!("removed the enemy {}", s("TargetGroupName")),
        _ => {
            let mut words = String::new();
            for (i, c) in action.chars().enumerate() {
                if c.is_uppercase() && i > 0 {
                    words.push(' ');
                }
                words.push(c.to_ascii_lowercase());
            }
            let mut extra: Vec<String> = d
                .as_object()
                .map(|o| {
                    o.iter()
                        .filter(|(k, v)| {
                            !k.ends_with("Id") && !v.is_null() && !v.is_object() && !v.is_array()
                        })
                        .map(|(k, v)| {
                            format!(
                                "{k}: {}",
                                v.as_str().map_or_else(|| v.to_string(), str::to_string)
                            )
                        })
                        .collect()
                })
                .unwrap_or_default();
            extra.sort();
            if extra.is_empty() {
                words
            } else {
                format!("{words} ({})", extra.join(", "))
            }
        }
    }
}

impl Core {
    /// A community's page header: name, owner, members, shout.
    pub fn community(&self, group_id: u64) -> Res<Community> {
        self.inner.res(
            Key::shared("community", group_id),
            DETAIL_AGE,
            move |f| async move {
                let g = groups::v1::get_groups_by_group_id(&f.client, group_id as i64).await?;
                let (owner_id, owner_name, owner_display, owner_verified) = user_of(g.owner);
                let shout = g
                    .shout
                    .filter(|s| s.body_.as_deref().is_some_and(|b| !b.trim().is_empty()));
                Ok(Community {
                    id: group_id,
                    name: g.name.unwrap_or_default(),
                    description: g.description.unwrap_or_default(),
                    owner_id,
                    owner: if owner_display.is_empty() {
                        owner_name
                    } else {
                        owner_display
                    },
                    owner_verified,
                    members: g.member_count.unwrap_or(0) as i64,
                    verified: g.has_verified_badge.unwrap_or(false),
                    public_entry: g.public_entry_allowed.unwrap_or(false),
                    locked: g.is_locked.unwrap_or(false) || g.is_deactivated.unwrap_or(false),
                    shout: shout.map(|s| {
                        let (poster_id, name, display, poster_verified) = user_of(s.poster);
                        Shout {
                            text: s.body_.unwrap_or_default(),
                            poster_id,
                            poster: if display.is_empty() { name.clone() } else { display },
                            poster_username: name,
                            poster_verified,
                            updated: s.updated.or(s.created).unwrap_or_default(),
                        }
                    }),
                })
            },
        )
    }

    /// Your role and permissions in a community.
    pub fn membership(&self, account: AccountId, group_id: u64) -> Res<Membership> {
        if account.is_anonymous() {
            return Res::default();
        }
        self.inner.res(
            Key::private(account, "community_membership", group_id),
            ADMIN_AGE,
            move |f| async move {
                let p = groups::v1::GetGroupsByGroupIdMembershipParams {
                    include_notification_preferences: false,
                };
                let m = groups::v1::get_groups_by_group_id_membership(&f.client, group_id as i64, &p).await?;
                let p = m.permissions.unwrap_or_default();
                let posts = p.group_posts_permissions.unwrap_or_default();
                let mem = p.group_membership_permissions.unwrap_or_default();
                let man = p.group_management_permissions.unwrap_or_default();
                let eco = p.group_economy_permissions.unwrap_or_default();
                let fo = p.group_forums_permissions.unwrap_or_default();
                let b = |x: Option<bool>| x.unwrap_or(false);
                let owner = b(m.is_owner);
                Ok(Membership {
                    role: m.user_role.and_then(|r| r.role).and_then(role),
                    primary: b(m.is_primary),
                    pending: b(m.is_pending_join),
                    owner,
                    can_configure: b(m.can_configure),
                    can_view_members: m.can_view_member_list.unwrap_or(true),
                    funds_visible: b(m.are_group_funds_visible),
                    games_visible: m.are_group_games_visible.unwrap_or(true),
                    perms: Perms {
                        post_shout: b(posts.post_to_status),
                        view_shout: posts.view_status.unwrap_or(true),
                        change_rank: b(mem.change_rank),
                        remove_members: b(mem.remove_members),
                        ban_members: b(mem.ban_members),
                        invite_members: b(mem.invite_members),
                        view_audit_log: b(man.view_audit_logs),
                        manage_relationships: b(man.manage_relationships),
                        spend_funds: b(eco.spend_group_funds),
                        view_payouts: b(eco.view_group_payouts) || owner,
                        manage_settings: b(m.can_configure) || owner,
                        view_forums: fo.view_forums.unwrap_or(true),
                        create_posts: b(fo.create_posts),
                        create_comments: b(fo.create_comments),
                        remove_posts: b(fo.remove_posts),
                        remove_comments: b(fo.remove_comments),
                    },
                })
            },
        )
    }

    /// Roles from the top rank down, without Guest.
    pub fn community_roles(&self, group_id: u64) -> Res<Vec<Role>> {
        self.inner.res(
            Key::shared("community_roles", group_id),
            DETAIL_AGE,
            move |f| async move {
                let p = groups::v1::GetGroupsByGroupIdRolesParams {
                    include_private: false,
                };
                let r = groups::v1::get_groups_by_group_id_roles(&f.client, group_id as i64, &p).await?;
                let mut v: Vec<Role> = r
                    .roles
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(role)
                    .filter(|r| r.rank > 0)
                    .collect();
                v.sort_by_key(|r| (-r.rank, r.id));
                // The site lists a role once even when Roblox sends a base copy of it.
                v.dedup_by(|a, b| a.rank == b.rank && a.name == b.name);
                Ok(v)
            },
        )
    }

    /// Members with one role, 100 at a time, as `viewer` sees them.
    pub fn community_members(
        &self,
        viewer: AccountId,
        group_id: u64,
        role_id: u64,
        more: bool,
    ) -> Res<Page<Member>> {
        self.inner.paged(
            Key::private(viewer, "community_members", format!("{group_id}:{role_id}")),
            LIST_AGE,
            more,
            move |f, cursor| async move {
                let p = groups::v1::GetGroupsByGroupIdRolesByRoleSetIdUsersParams {
                    limit: Some(100),
                    cursor,
                    sort_order: Some(groups::v1::GetGroupsByGroupIdAuditLogSortOrder::Desc),
                };
                let r = groups::v1::get_groups_by_group_id_roles_by_role_set_id_users(
                    &f.client,
                    group_id as i64,
                    role_id as i64,
                    &p,
                )
                .await?;
                let items: Vec<Member> = r
                    .data
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|u| {
                        let (user_id, name, display_name, verified) = user_of(Some(u));
                        (user_id > 0).then_some(Member {
                            user_id,
                            name,
                            display_name,
                            verified,
                        })
                    })
                    .collect();
                f.note_users(items.iter().map(|m| UserInfo {
                    id: m.user_id,
                    name: m.name.clone(),
                    display_name: m.display_name.clone(),
                    verified: m.verified,
                }));
                Ok(Page {
                    items,
                    next: r.next_page_cursor.filter(|c| !c.is_empty()),
                })
            },
        )
    }

    /// Community search, best match first.
    pub fn search_communities(&self, query: &str, more: bool) -> Res<Page<CommunityRow>> {
        let q = query.trim().to_string();
        if q.chars().count() < 2 {
            return Res::default();
        }
        self.inner.paged(
            Key::shared("community_search", q.to_lowercase()),
            LIST_AGE,
            more,
            move |f, cursor| {
                let q = q.clone();
                async move {
                    let p = groups::v1::GetGroupsSearchParams {
                        keyword: q,
                        prioritize_exact_match: Some(true),
                        limit: Some(25),
                        cursor,
                    };
                    let r = groups::v1::get_groups_search(&f.client, &p).await?;
                    Ok(Page {
                        items: r
                            .data
                            .unwrap_or_default()
                            .into_iter()
                            .filter_map(|g| {
                                Some(CommunityRow {
                                    id: g.id.filter(|i| *i > 0)? as u64,
                                    name: g.name.unwrap_or_default(),
                                    description: g.description.unwrap_or_default(),
                                    members: g.member_count.unwrap_or(0) as i64,
                                    verified: g.has_verified_badge.unwrap_or(false),
                                    public_entry: g.public_entry_allowed.unwrap_or(false),
                                })
                            })
                            .collect(),
                        next: r.next_page_cursor.filter(|c| !c.is_empty()),
                    })
                }
            },
        )
    }

    /// Communities `account` asked to join that haven't answered yet.
    pub fn pending_communities(&self, account: AccountId) -> Res<Vec<CommunityRow>> {
        if account.is_anonymous() {
            return Res::default();
        }
        self.inner.res(
            Key::private(account, "community_pending", ""),
            LIST_AGE,
            move |f| async move {
                let r = groups::v1::get_user_groups_pending(&f.client).await?;
                Ok(r.data.unwrap_or_default().into_iter().filter_map(row).collect())
            },
        )
    }

    /// Public experiences a community made.
    pub fn community_games(&self, group_id: u64) -> Res<Vec<Creation>> {
        self.inner.res(
            Key::shared("community_games", group_id),
            DETAIL_AGE,
            move |f| async move {
                // accessFilter 2 = Public, as on the community page.
                let p = games::v2::GetGroupsByGroupIdGamesV2Params {
                    access_filter: Some(2),
                    limit: Some(50),
                    ..Default::default()
                };
                let r = games::v2::get_groups_by_group_id_games_v2(&f.client, group_id as i64, &p).await?;
                let mut v: Vec<Creation> = r
                    .data
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
                    .collect();
                v.sort_by_key(|g| -g.visits);
                Ok(v)
            },
        )
    }

    /// Items a community sells, newest first.
    pub fn community_store(&self, group_id: u64, more: bool) -> Res<Page<CatalogItem>> {
        self.inner.paged(
            Key::shared("community_store", group_id),
            DETAIL_AGE,
            more,
            move |f, cursor| async move {
                use catalog::v2::{
                    GetSearchItemsDetailsCreatorType as Creator, GetSearchItemsDetailsSortType as Sort,
                };
                let p = catalog::v2::GetSearchItemsDetailsParams {
                    creator_type: Some(Creator::Group),
                    creator_target_id: Some(group_id as i64),
                    sort_type: Some(Sort::Updated),
                    limit: Some(30),
                    cursor,
                    ..Default::default()
                };
                let r = catalog::v2::get_search_items_details(&f.client, &p).await?;
                Ok(Page {
                    items: r
                        .data
                        .unwrap_or_default()
                        .into_iter()
                        .filter_map(CatalogItem::from_search)
                        .collect(),
                    next: r.next_page_cursor.filter(|c| !c.is_empty()),
                })
            },
        )
    }

    /// Allies and enemies (first 100 of each).
    pub fn community_affiliates(&self, group_id: u64) -> Res<Affiliates> {
        self.inner.res(
            Key::shared("community_affiliates", group_id),
            DETAIL_AGE,
            move |f| async move {
                let p = groups::v1::GetGroupsByGroupIdRelationshipsByGroupRelationshipTypeParams {
                    start_row_index: 0,
                    max_rows: 100,
                };
                let c = &f.client;
                let (a, e) = tokio::join!(
                    groups::v1::get_groups_by_group_id_relationships_by_group_relationship_type(
                        c,
                        group_id as i64,
                        Affiliation::Ally.api(),
                        &p
                    ),
                    groups::v1::get_groups_by_group_id_relationships_by_group_relationship_type(
                        c,
                        group_id as i64,
                        Affiliation::Enemy.api(),
                        &p
                    ),
                );
                let list = |r: groups::v1::GroupRelationshipsResponse| {
                    r.related_groups
                        .unwrap_or_default()
                        .into_iter()
                        .filter_map(row)
                        .collect()
                };
                Ok(Affiliates {
                    allies: list(a?),
                    // Enemies are off in most communities; Roblox answers an error then.
                    enemies: e.map(list).unwrap_or_default(),
                })
            },
        )
    }

    /// Links the community shows on its page (Discord, YouTube, ...). Needs sign-in.
    pub fn community_links(&self, viewer: AccountId, group_id: u64) -> Res<Vec<SocialLink>> {
        if viewer.is_anonymous() {
            return Res::default();
        }
        self.inner.res(
            Key::private(viewer, "community_links", group_id),
            DETAIL_AGE,
            move |f| async move {
                let r = groups::v1::get_groups_by_group_id_social_links(&f.client, group_id as i64).await?;
                Ok(r.data
                    .unwrap_or_default()
                    .into_iter()
                    .filter(|l| l.url.as_deref().is_some_and(|u| u.starts_with("https://")))
                    .map(|l| SocialLink {
                        title: l.title.unwrap_or_default(),
                        url: l.url.unwrap_or_default(),
                        kind: l.type_.and_then(|t| t.name()).unwrap_or_default().to_string(),
                    })
                    .collect())
            },
        )
    }

    /// Names the community had before.
    pub fn community_name_history(&self, group_id: u64) -> Res<Vec<String>> {
        self.inner.res(
            Key::shared("community_names", group_id),
            DETAIL_AGE,
            move |f| async move {
                let p = groups::v1::GetGroupsByGroupIdNameHistoryParams {
                    limit: Some(50),
                    ..Default::default()
                };
                let r =
                    groups::v1::get_groups_by_group_id_name_history(&f.client, group_id as i64, &p).await?;
                Ok(r.data
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|n| n.name)
                    .collect())
            },
        )
    }

    // ---- forums (signed in only) ----

    pub fn forum_categories(&self, viewer: AccountId, group_id: u64) -> Res<Vec<ForumCategory>> {
        if viewer.is_anonymous() {
            return Res::default();
        }
        self.inner.res(
            Key::private(viewer, "forum_categories", group_id),
            FORUM_AGE,
            move |f| async move {
                let p = groups::undocumented::GetForumCategoriesParams { archived: None };
                let r = groups::undocumented::get_forum_categories(&f.client, group_id as i64, &p).await?;
                let mut v: Vec<(Option<i32>, ForumCategory)> = r
                    .data
                    .unwrap_or_default()
                    .into_iter()
                    .filter(|c| !c.is_archived.unwrap_or(false))
                    .filter_map(|c| {
                        Some((
                            c.rank,
                            ForumCategory {
                                id: c.id.filter(|i| !i.is_empty())?,
                                name: c.name.unwrap_or_default(),
                                description: c.description.unwrap_or_default(),
                            },
                        ))
                    })
                    .collect();
                // Ranked categories in rank order, the rest by name (as the site does).
                v.sort_by(|a, b| match (a.0, b.0) {
                    (Some(x), Some(y)) => x.cmp(&y),
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (None, None) => a.1.name.to_uppercase().cmp(&b.1.name.to_uppercase()),
                });
                Ok(v.into_iter().map(|x| x.1).collect())
            },
        )
    }

    /// Posts of a category: pinned first, then the rest, 20 at a time.
    pub fn forum_posts(
        &self,
        viewer: AccountId,
        group_id: u64,
        category: &str,
        more: bool,
    ) -> Res<Page<ForumPost>> {
        if viewer.is_anonymous() || category.is_empty() {
            return Res::default();
        }
        let cat = category.to_string();
        self.inner.paged(
            Key::private(viewer, "forum_posts", format!("{group_id}:{cat}")),
            FORUM_AGE,
            more,
            move |f, cursor| {
                let cat = cat.clone();
                async move {
                    let c = &f.client;
                    let first = cursor.is_none();
                    let p = groups::undocumented::GetForumPostsParams {
                        include_comment_count: Some(true),
                        limit: Some(20),
                        cursor,
                    };
                    let r = groups::undocumented::get_forum_posts(c, group_id as i64, &cat, &p).await?;
                    let mut items: Vec<ForumPost> = Vec::new();
                    if first {
                        let p = groups::undocumented::GetForumPinnedPostsParams {
                            include_comment_count: Some(true),
                        };
                        if let Ok(pinned) =
                            groups::undocumented::get_forum_pinned_posts(c, group_id as i64, &cat, &p).await
                        {
                            items.extend(
                                pinned
                                    .data
                                    .unwrap_or_default()
                                    .into_iter()
                                    .filter_map(forum_post)
                                    .map(|mut p| {
                                        p.pinned = true;
                                        p
                                    }),
                            );
                        }
                    }
                    for p in r.data.unwrap_or_default().into_iter().filter_map(forum_post) {
                        if !items.iter().any(|x| x.id == p.id) {
                            items.push(p);
                        }
                    }
                    Ok(Page {
                        items,
                        next: r.next_page_cursor.filter(|c| !c.is_empty()),
                    })
                }
            },
        )
    }

    /// Replies to a post, oldest first (without the post's own text).
    pub fn forum_comments(
        &self,
        viewer: AccountId,
        group_id: u64,
        category: &str,
        post: &str,
        more: bool,
    ) -> Res<Page<ForumComment>> {
        if viewer.is_anonymous() || post.is_empty() {
            return Res::default();
        }
        let (cat, post) = (category.to_string(), post.to_string());
        self.inner.paged(
            Key::private(viewer, "forum_comments", format!("{group_id}:{cat}:{post}")),
            FORUM_AGE,
            more,
            move |f, cursor| {
                let (cat, post) = (cat.clone(), post.clone());
                async move {
                    let p = groups::undocumented::GetForumCommentsParams {
                        limit: Some(50),
                        cursor,
                    };
                    let r =
                        groups::undocumented::get_forum_comments(&f.client, group_id as i64, &cat, &post, &p)
                            .await?;
                    Ok(Page {
                        items: r
                            .data
                            .unwrap_or_default()
                            .into_iter()
                            .filter_map(forum_comment)
                            .collect(),
                        next: r.next_page_cursor.filter(|c| !c.is_empty()),
                    })
                }
            },
        )
    }

    // ---- admin views ----

    pub fn join_requests(&self, account: AccountId, group_id: u64, more: bool) -> Res<Page<JoinRequest>> {
        if account.is_anonymous() {
            return Res::default();
        }
        self.inner.paged(
            Key::private(account, "community_requests", group_id),
            ADMIN_AGE,
            more,
            move |f, cursor| async move {
                let p = groups::v1::GetGroupsByGroupIdJoinRequestsParams {
                    limit: Some(100),
                    cursor,
                    sort_order: Some(groups::v1::GetGroupsByGroupIdAuditLogSortOrder::Desc),
                };
                let r =
                    groups::v1::get_groups_by_group_id_join_requests(&f.client, group_id as i64, &p).await?;
                let items: Vec<JoinRequest> = r
                    .data
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|j| {
                        let (user_id, name, display_name, _) = user_of(j.requester);
                        (user_id > 0).then(|| JoinRequest {
                            user_id,
                            name,
                            display_name,
                            created: j.created.unwrap_or_default(),
                        })
                    })
                    .collect();
                f.note_users(items.iter().map(|j| UserInfo {
                    id: j.user_id,
                    name: j.name.clone(),
                    display_name: j.display_name.clone(),
                    verified: false,
                }));
                Ok(Page {
                    items,
                    next: r.next_page_cursor.filter(|c| !c.is_empty()),
                })
            },
        )
    }

    pub fn audit_log(&self, account: AccountId, group_id: u64, more: bool) -> Res<Page<AuditEntry>> {
        if account.is_anonymous() {
            return Res::default();
        }
        self.inner.paged(
            Key::private(account, "community_audit", group_id),
            ADMIN_AGE,
            more,
            move |f, cursor| async move {
                let p = groups::v1::GetGroupsByGroupIdAuditLogParams {
                    limit: Some(50),
                    cursor,
                    sort_order: Some(groups::v1::GetGroupsByGroupIdAuditLogSortOrder::Desc),
                    ..Default::default()
                };
                let r = groups::v1::get_groups_by_group_id_audit_log(&f.client, group_id as i64, &p).await?;
                Ok(Page {
                    items: r
                        .data
                        .unwrap_or_default()
                        .into_iter()
                        .map(|e| {
                            let a = e.actor.unwrap_or_default();
                            let (actor_id, name, display, _) = user_of(a.user);
                            let action = e.action_type.unwrap_or_default();
                            AuditEntry {
                                actor_id,
                                actor: if display.is_empty() { name } else { display },
                                actor_role: a.role.and_then(|r| r.name).unwrap_or_default(),
                                text: describe_audit(&action, &e.description.unwrap_or_default()),
                                action,
                                created: e.created.unwrap_or_default(),
                            }
                        })
                        .collect(),
                    next: r.next_page_cursor.filter(|c| !c.is_empty()),
                })
            },
        )
    }

    pub fn community_settings(&self, account: AccountId, group_id: u64) -> Res<Settings> {
        if account.is_anonymous() {
            return Res::default();
        }
        self.inner.res(
            Key::private(account, "community_settings", group_id),
            ADMIN_AGE,
            move |f| async move {
                let s = groups::v1::get_groups_by_group_id_settings(&f.client, group_id as i64).await?;
                Ok(Settings {
                    approval_required: s.is_approval_required.unwrap_or(false),
                    enemies_allowed: s.are_enemies_allowed.unwrap_or(false),
                    funds_visible: s.are_group_funds_visible.unwrap_or(false),
                    games_visible: s.are_group_games_visible.unwrap_or(true),
                })
            },
        )
    }

    /// The community's Robux, when `account` may see them.
    pub fn community_funds(&self, account: AccountId, group_id: u64) -> Res<i64> {
        if account.is_anonymous() {
            return Res::default();
        }
        self.inner.res(
            Key::private(account, "community_funds", group_id),
            ADMIN_AGE,
            move |f| async move {
                let r = economy::undocumented::get_group_funds(&f.client, group_id as i64).await?;
                Ok(r.robux.unwrap_or(0))
            },
        )
    }

    /// Recurring payout split (owners and those who may see payouts).
    pub fn community_payouts(&self, account: AccountId, group_id: u64) -> Res<Vec<Payout>> {
        if account.is_anonymous() {
            return Res::default();
        }
        self.inner.res(
            Key::private(account, "community_payouts", group_id),
            ADMIN_AGE,
            move |f| async move {
                let r = groups::v1::get_groups_by_group_id_payouts(&f.client, group_id as i64).await?;
                let mut v: Vec<Payout> = r
                    .data
                    .unwrap_or_default()
                    .into_iter()
                    .map(|p| {
                        let (user_id, name, display, _) = user_of(p.user);
                        Payout {
                            user_id,
                            name: if display.is_empty() { name } else { display },
                            percent: p.percentage.unwrap_or(0),
                        }
                    })
                    .collect();
                v.sort_by_key(|p| -p.percent);
                Ok(v)
            },
        )
    }

    /// Which of your accounts are in this community (from each account's public community
    /// list), so the page can say "Alt is an Admin here".
    pub fn my_accounts_in(&self, group_id: u64) -> Vec<AccountMembership> {
        let ids: Vec<AccountId> = self.store().accounts.accounts.iter().map(|a| a.id).collect();
        ids.into_iter()
            .filter_map(|a| {
                let g = self.user_groups(a.0).data?;
                let r = g.iter().find(|r| r.group_id == group_id)?;
                Some(AccountMembership {
                    account: a,
                    role: r.role.clone(),
                    rank: r.rank,
                    primary: r.primary,
                })
            })
            .collect()
    }

    // ---- shouts you haven't seen (local) ----

    /// When you last looked at each community's shout (ISO time of the shout you saw).
    pub fn seen_shouts(&self, account: AccountId) -> HashMap<String, String> {
        self.local(account, "seen_shouts")
    }

    /// Note the current shout of a community as seen.
    pub fn mark_shout_seen(&self, account: AccountId, group_id: u64, updated: &str) {
        let mut m = self.seen_shouts(account);
        let k = group_id.to_string();
        if m.get(&k).map(String::as_str) == Some(updated) {
            return;
        }
        m.insert(k, updated.to_string());
        self.set_local(account, "seen_shouts", &m);
    }

    // ---- user actions ----

    pub fn join_community(&self, account: AccountId, group_id: u64) {
        self.inner.mutate(
            account,
            "join the community",
            "",
            &[
                "community_membership",
                "user_groups",
                "community_pending",
                "community",
            ],
            move |f| async move {
                groups::v1::post_groups_by_group_id_users(
                    &f.client,
                    group_id as i64,
                    &Default::default(),
                    &groups::v1::JoinGroupRequest::default(),
                )
                .await?;
                f.inner.toast(
                    Some(account),
                    "Joined, or asked to join if it needs approval",
                    false,
                );
                Ok(())
            },
        );
    }

    pub fn leave_community(&self, account: AccountId, group_id: u64) {
        self.drop_user_group(account, group_id);
        self.inner.mutate(
            account,
            "leave the community",
            "Left the community",
            &["community_membership", "user_groups", "community"],
            move |f| async move {
                groups::v1::delete_groups_by_group_id_users_by_user_id(
                    &f.client,
                    group_id as i64,
                    account.0 as i64,
                )
                .await?;
                Ok(())
            },
        );
    }

    /// Leave several communities in one confirmed action.
    pub fn leave_communities(&self, account: AccountId, ids: Vec<u64>) {
        for g in &ids {
            self.drop_user_group(account, *g);
        }
        let n = ids.len();
        self.inner.mutate(
            account,
            "leave the communities",
            &format!("Left {n} {}", if n == 1 { "community" } else { "communities" }),
            &["community_membership", "user_groups", "community"],
            move |f| async move {
                for g in ids {
                    groups::v1::delete_groups_by_group_id_users_by_user_id(
                        &f.client,
                        g as i64,
                        account.0 as i64,
                    )
                    .await?;
                }
                Ok(())
            },
        );
    }

    /// Make a community your primary one, or clear it (`None`).
    pub fn set_primary_community(&self, account: AccountId, group_id: Option<u64>) {
        self.inner
            .res
            .update::<Vec<crate::people::GroupRole>>(&Key::shared("user_groups", account.0), |v| {
                for g in v.iter_mut() {
                    g.primary = Some(g.group_id) == group_id;
                }
            });
        self.inner.mutate(
            account,
            "change your primary community",
            if group_id.is_some() {
                "Primary community set"
            } else {
                "Primary community cleared"
            },
            &["community_membership", "user_groups"],
            move |f| async move {
                match group_id {
                    Some(g) => {
                        let body = groups::v1::PrimaryGroupRequest {
                            group_id: Some(g as i64),
                        };
                        groups::v1::post_user_groups_primary(&f.client, &body).await?;
                    }
                    None => {
                        groups::v1::delete_user_groups_primary(&f.client).await?;
                    }
                }
                Ok(())
            },
        );
    }

    pub fn post_shout(&self, account: AccountId, group_id: u64, text: &str) {
        let body = groups::v1::PostGroupStatusRequest {
            message: Some(text.trim().to_string()),
        };
        self.inner.mutate(
            account,
            "post the shout",
            if text.trim().is_empty() {
                "Shout cleared"
            } else {
                "Shout posted"
            },
            &["community", "community_audit"],
            move |f| async move {
                groups::v1::patch_groups_by_group_id_status(&f.client, group_id as i64, &body).await?;
                Ok(())
            },
        );
    }

    /// Accept or decline join requests (one, or the ones you picked).
    pub fn answer_join_requests(&self, account: AccountId, group_id: u64, users: Vec<u64>, accept: bool) {
        if users.is_empty() {
            return;
        }
        self.inner
            .res
            .update::<Page<JoinRequest>>(&Key::private(account, "community_requests", group_id), |p| {
                p.items.retain(|r| !users.contains(&r.user_id))
            });
        let n = users.len();
        let done = match (accept, n) {
            (true, 1) => "Request accepted".to_string(),
            (false, 1) => "Request declined".to_string(),
            (true, n) => format!("{n} requests accepted"),
            (false, n) => format!("{n} requests declined"),
        };
        self.inner.mutate(
            account,
            if accept {
                "accept the request"
            } else {
                "decline the request"
            },
            &done,
            &["community_requests", "community", "community_audit"],
            move |f| async move {
                let c = &f.client;
                let g = group_id as i64;
                if let [one] = users.as_slice() {
                    if accept {
                        groups::v1::post_groups_by_group_id_join_requests_users_by_user_id(c, g, *one as i64)
                            .await?;
                    } else {
                        groups::v1::delete_groups_by_group_id_join_requests_users_by_user_id(
                            c,
                            g,
                            *one as i64,
                        )
                        .await?;
                    }
                } else {
                    let body = groups::v1::MembersRequest {
                        user_ids: Some(users.iter().map(|u| *u as i64).collect()),
                    };
                    if accept {
                        groups::v1::post_groups_by_group_id_join_requests(c, g, &body).await?;
                    } else {
                        groups::v1::delete_groups_by_group_id_join_requests(c, g, &body).await?;
                    }
                }
                Ok(())
            },
        );
    }

    pub fn set_member_role(&self, account: AccountId, group_id: u64, user_id: u64, role_id: u64) {
        self.inner.mutate(
            account,
            "change the rank",
            "Rank changed",
            &["community_members", "community_roles", "community_audit"],
            move |f| async move {
                let body = groups::v1::UpdateUserRoleRequest {
                    role_id: Some(role_id as i64),
                };
                groups::v1::patch_groups_by_group_id_users_by_user_id(
                    &f.client,
                    group_id as i64,
                    user_id as i64,
                    &body,
                )
                .await?;
                Ok(())
            },
        );
    }

    pub fn kick_member(&self, account: AccountId, group_id: u64, user_id: u64) {
        self.inner.mutate(
            account,
            "remove them",
            "Removed from the community",
            &[
                "community_members",
                "community_roles",
                "community_audit",
                "community",
            ],
            move |f| async move {
                groups::v1::delete_groups_by_group_id_users_by_user_id(
                    &f.client,
                    group_id as i64,
                    user_id as i64,
                )
                .await?;
                Ok(())
            },
        );
    }

    pub fn set_community_setting(&self, account: AccountId, group_id: u64, setting: Setting, on: bool) {
        self.inner
            .res
            .update::<Settings>(&Key::private(account, "community_settings", group_id), |s| {
                setting.set(s, on)
            });
        let mut body = groups::v1::UpdateGroupSettingsRequest::default();
        match setting {
            Setting::ApprovalRequired => body.is_approval_required = Some(on),
            Setting::EnemiesAllowed => body.are_enemies_allowed = Some(on),
            Setting::FundsVisible => body.are_group_funds_visible = Some(on),
            Setting::GamesVisible => body.are_group_games_visible = Some(on),
        }
        self.inner.mutate(
            account,
            "change the setting",
            "Saved",
            &["community_settings", "community_membership", "community"],
            move |f| async move {
                groups::v1::patch_groups_by_group_id_settings(&f.client, group_id as i64, &body).await?;
                Ok(())
            },
        );
    }

    pub fn create_forum_post(
        &self,
        account: AccountId,
        group_id: u64,
        category: &str,
        title: &str,
        text: &str,
    ) {
        let cat = category.to_string();
        let body = groups::undocumented::ForumNewPost {
            title: Some(title.trim().to_string()),
            content: Some(text.trim().to_string()),
        };
        self.inner
            .mutate(account, "post", "Posted", &["forum_posts"], move |f| async move {
                groups::undocumented::post_forum_post(&f.client, group_id as i64, &cat, &body).await?;
                Ok(())
            });
    }

    pub fn reply_forum_post(
        &self,
        account: AccountId,
        group_id: u64,
        category: &str,
        post: &ForumPost,
        text: &str,
    ) {
        let (cat, id) = (category.to_string(), post.id.clone());
        let body = groups::undocumented::ForumNewComment {
            replies_to_post_comment_id: Some(post.first_comment.clone()).filter(|c| !c.is_empty()),
            content: Some(text.trim().to_string()),
        };
        self.inner.mutate(
            account,
            "reply",
            "Replied",
            &["forum_comments", "forum_posts"],
            move |f| async move {
                groups::undocumented::post_forum_comment(&f.client, group_id as i64, &cat, &id, &body)
                    .await?;
                Ok(())
            },
        );
    }

    pub fn delete_forum_post(&self, account: AccountId, group_id: u64, category: &str, post: &str) {
        let (cat, id) = (category.to_string(), post.to_string());
        self.inner.res.update::<Page<ForumPost>>(
            &Key::private(account, "forum_posts", format!("{group_id}:{cat}")),
            |p| p.items.retain(|x| x.id != id),
        );
        self.inner.mutate(
            account,
            "delete the post",
            "Post deleted",
            &["forum_posts", "community_audit"],
            move |f| async move {
                groups::undocumented::delete_forum_post(&f.client, group_id as i64, &cat, &id).await?;
                Ok(())
            },
        );
    }

    pub fn delete_forum_comment(
        &self,
        account: AccountId,
        group_id: u64,
        category: &str,
        post: &str,
        comment: &str,
    ) {
        let (cat, post, id) = (category.to_string(), post.to_string(), comment.to_string());
        self.inner.res.update::<Page<ForumComment>>(
            &Key::private(account, "forum_comments", format!("{group_id}:{cat}:{post}")),
            |p| p.items.retain(|x| x.id != id),
        );
        self.inner.mutate(
            account,
            "delete the reply",
            "Reply deleted",
            &["forum_comments", "forum_posts"],
            move |f| async move {
                groups::undocumented::delete_forum_comment(&f.client, group_id as i64, &cat, &post, &id)
                    .await?;
                Ok(())
            },
        );
    }

    /// Pay `amount` Robux from the community's funds to a member, once.
    pub fn pay_out(&self, account: AccountId, group_id: u64, user_id: u64, amount: i64) {
        self.inner
            .res
            .update::<i64>(&Key::private(account, "community_funds", group_id), |f| {
                *f -= amount
            });
        let body = groups::v1::PayoutRequest {
            idempotency_key: Some(crate::core::random_uuid()),
            payout_type: Some(groups::v1::PayoutRequestPayoutType::FixedAmount),
            recipients: Some(vec![groups::v1::PayoutRecipientRequest {
                amount: Some(amount),
                recipient_id: Some(user_id as i64),
                recipient_type: Some(groups::v1::PayoutRecipientRequestRecipientType::User),
            }]),
            watermark_contributions: None,
        };
        self.inner.mutate(
            account,
            "pay out",
            &format!("Paid out R$ {amount}"),
            &["community_funds", "community_audit"],
            move |f| async move {
                groups::v1::post_groups_by_group_id_payouts(&f.client, group_id as i64, &body).await?;
                Ok(())
            },
        );
    }

    /// Replace the recurring payout split with `split` (member, percent). Members left out
    /// stop getting a share; an empty split stops recurring payouts.
    pub fn set_recurring_payouts(
        &self,
        account: AccountId,
        group_id: u64,
        split: Vec<(u64, i32)>,
    ) -> Result<(), String> {
        check_split(&split)?;
        self.inner
            .res
            .update::<Vec<Payout>>(&Key::private(account, "community_payouts", group_id), |v| {
                let names: HashMap<u64, String> = v.iter().map(|p| (p.user_id, p.name.clone())).collect();
                *v = split
                    .iter()
                    .map(|(u, p)| Payout {
                        user_id: *u,
                        name: names.get(u).cloned().unwrap_or_default(),
                        percent: *p,
                    })
                    .collect();
                v.sort_by_key(|p| -p.percent);
            });
        let body = groups::v1::PayoutRequest {
            idempotency_key: Some(crate::core::random_uuid()),
            payout_type: Some(groups::v1::PayoutRequestPayoutType::Percentage),
            recipients: Some(
                split
                    .iter()
                    .map(|(u, p)| groups::v1::PayoutRecipientRequest {
                        amount: Some(i64::from(*p)),
                        recipient_id: Some(*u as i64),
                        recipient_type: Some(groups::v1::PayoutRecipientRequestRecipientType::User),
                    })
                    .collect(),
            ),
            watermark_contributions: None,
        };
        self.inner.mutate(
            account,
            "change the recurring payouts",
            "Recurring payouts saved",
            &["community_payouts", "community_audit"],
            move |f| async move {
                groups::v1::post_groups_by_group_id_payouts_recurring(&f.client, group_id as i64, &body)
                    .await?;
                Ok(())
            },
        );
        Ok(())
    }

    /// Send an ally request to `other`, or declare it an enemy.
    pub fn add_affiliate(&self, account: AccountId, group_id: u64, kind: Affiliation, other: u64) {
        let (what, done) = match kind {
            Affiliation::Ally => ("send the ally request", "Ally request sent"),
            Affiliation::Enemy => ("declare the enemy", "Enemy declared"),
        };
        self.inner.mutate(
            account,
            what,
            done,
            &["community_affiliates", "community_audit"],
            move |f| async move {
                groups::v1::post_groups_by_group_id_relationships_by_group_relationship_type_by_related_group_id(
                    &f.client,
                    group_id as i64,
                    kind.api(),
                    other as i64,
                )
                .await?;
                Ok(())
            },
        );
    }

    /// End an alliance or an enmity with `other`.
    pub fn remove_affiliate(&self, account: AccountId, group_id: u64, kind: Affiliation, other: u64) {
        self.inner
            .res
            .update::<Affiliates>(&Key::shared("community_affiliates", group_id), |a| {
                match kind {
                    Affiliation::Ally => &mut a.allies,
                    Affiliation::Enemy => &mut a.enemies,
                }
                .retain(|g| g.id != other)
            });
        let (what, done) = match kind {
            Affiliation::Ally => ("remove the ally", "Ally removed"),
            Affiliation::Enemy => ("remove the enemy", "Enemy removed"),
        };
        self.inner.mutate(
            account,
            what,
            done,
            &["community_affiliates", "community_audit"],
            move |f| async move {
                groups::v1::delete_groups_by_group_id_relationships_by_group_relationship_type_by_related_group_id(
                    &f.client,
                    group_id as i64,
                    kind.api(),
                    other as i64,
                )
                .await?;
                Ok(())
            },
        );
    }

    fn drop_user_group(&self, account: AccountId, group_id: u64) {
        self.inner
            .res
            .update::<Vec<crate::people::GroupRole>>(&Key::shared("user_groups", account.0), |v| {
                v.retain(|g| g.group_id != group_id)
            });
        self.inner
            .res
            .update::<Membership>(&Key::private(account, "community_membership", group_id), |m| {
                m.role = None;
                m.primary = false;
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn audit_lines() {
        assert_eq!(
            describe_audit(
                "ChangeRank",
                &json!({"TargetName": "Ann", "OldRoleSetName": "Member", "NewRoleSetName": "Admin"})
            ),
            "changed Ann's rank from Member to Admin"
        );
        assert_eq!(
            describe_audit("PostStatus", &json!({"Text": "hi"})),
            "posted a shout: “hi”"
        );
        assert_eq!(
            describe_audit(
                "ConfigureItems",
                &json!({"Type": 2, "TargetId": 5, "AssetName": "Hat"})
            ),
            "configure items (AssetName: Hat, Type: 2)"
        );
        assert_eq!(describe_audit("Lock", &serde_json::Value::Null), "lock");
    }

    #[test]
    fn payout_splits() {
        assert!(check_split(&[]).is_ok(), "empty stops recurring payouts");
        assert!(check_split(&[(1, 60), (2, 40)]).is_ok());
        assert!(check_split(&[(1, 60), (2, 41)]).unwrap_err().contains("101%"));
        assert!(check_split(&[(1, 0)]).is_err());
        assert!(check_split(&[(1, 10), (1, 20)]).is_err());
    }

    #[test]
    fn settings_keys_round_trip() {
        for s in Setting::ALL {
            assert_eq!(Setting::from_key(s.key()), Some(s));
            let mut v = Settings::default();
            s.set(&mut v, true);
            assert!(s.get(&v));
        }
    }

    #[test]
    fn forum_post_counts_replies_without_its_own_text() {
        let p = forum_post(groups::undocumented::ForumPost {
            id: Some("p".into()),
            comment_count: Some(3),
            first_comment: Some(groups::undocumented::ForumComment {
                id: Some("c".into()),
                content: Some(groups::undocumented::ForumContent {
                    plain_text: Some("hello".into()),
                }),
                ..Default::default()
            }),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(
            (p.replies, p.text.as_str(), p.first_comment.as_str()),
            (2, "hello", "c")
        );
    }
}

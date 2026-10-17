//! Communities (groups): your list, search, a community's page, its forum and admin tools.
//!
//! Commands about "this community" read it from the page context `group:<id>`, or
//! `group:<id>/forum/<category>/<post>` while a forum post is open.

use std::sync::Arc;

use bonsai_core::commands::CommandId;
use bonsai_core::communities::{Affiliation, Setting};
use bonsai_core::resolve::{Pasted, resolve};
use bonsai_core::{AccountId, Core};
use serde::Serialize;
use specta::Type;
use tauri::{AppHandle, State};

use super::{GameCard, GroupCard, Id, Invocation, ItemCard, List, copy, count, viewer};

/// A community in search results.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct CommunityHit {
    pub id: Id,
    pub name: String,
    pub description: String,
    pub members: u32,
    pub verified: bool,
    /// Anyone can join without approval.
    pub public_entry: bool,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct CommunityView {
    pub loading: bool,
    pub error: Option<String>,
    pub info: Option<CommunityInfo>,
    /// The active account's place in it; `None` when signed out or still loading.
    pub me: Option<MyMembership>,
    /// Your other accounts in it, like "Alt · Admin".
    pub alts: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct CommunityInfo {
    pub id: Id,
    pub name: String,
    pub description: String,
    pub owner_id: Id,
    pub owner: String,
    pub owner_verified: bool,
    pub members: u32,
    pub verified: bool,
    pub public_entry: bool,
    pub locked: bool,
    pub shout: Option<ShoutView>,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ShoutView {
    pub text: String,
    pub poster_id: Id,
    pub poster: String,
    pub poster_username: String,
    pub poster_verified: bool,
    /// ISO date-time.
    pub updated: String,
}

/// What the active account is and may do in a community.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct MyMembership {
    /// Role name; `None` when not a member.
    pub role: Option<String>,
    pub rank: i32,
    pub primary: bool,
    /// Asked to join, waiting for approval.
    pub pending: bool,
    pub owner: bool,
    pub can_view_members: bool,
    pub post_shout: bool,
    pub change_rank: bool,
    pub remove_members: bool,
    /// Accept and decline join requests.
    pub manage_requests: bool,
    pub view_audit_log: bool,
    pub manage_settings: bool,
    pub view_forums: bool,
    pub create_posts: bool,
    pub create_comments: bool,
    pub remove_posts: bool,
    pub remove_comments: bool,
    /// Send ally requests, declare enemies and remove either.
    pub manage_relationships: bool,
    pub spend_funds: bool,
    pub view_payouts: bool,
}

/// A game pass sold in one of the community's experiences.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct CommunityPass {
    pub id: Id,
    pub name: String,
    /// Robux; `None` when off sale.
    pub price: Option<u32>,
    pub owned: bool,
    /// The experience that sells it.
    pub place_id: Id,
    pub game: String,
}

/// The community's funds and recurring payouts, as the active account may see them.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct RevenueView {
    /// Robux in the funds; `None` while loading or when hidden from you.
    pub funds: Option<f64>,
    pub funds_error: Option<String>,
    pub split: List<PayoutRow>,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PayoutRow {
    pub id: Id,
    pub name: String,
    pub percent: i32,
}

/// Allied and enemy communities.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct AffiliatesView {
    pub loading: bool,
    pub error: Option<String>,
    pub allies: Vec<CommunityHit>,
    pub enemies: Vec<CommunityHit>,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct RoleRow {
    pub id: Id,
    pub name: String,
    pub rank: i32,
    pub members: u32,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct MemberRow {
    pub id: Id,
    pub name: String,
    pub display_name: String,
    pub verified: bool,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct JoinRequestRow {
    pub id: Id,
    pub name: String,
    pub display_name: String,
    /// ISO date-time.
    pub created: String,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct AuditRow {
    pub actor_id: Id,
    pub actor: String,
    pub actor_role: String,
    pub action: String,
    /// What happened, in words.
    pub text: String,
    /// ISO date-time.
    pub created: String,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct SettingRow {
    /// The `group.setting` argument.
    pub key: String,
    pub label: String,
    pub on: bool,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ForumCategoryRow {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ForumPostRow {
    pub id: String,
    pub title: String,
    pub author_id: Id,
    pub author: String,
    pub author_role: String,
    /// ISO date-time.
    pub created: String,
    /// ISO date-time of the latest activity on the thread.
    pub updated: String,
    pub replies: u32,
    pub pinned: bool,
    pub locked: bool,
    pub unread: bool,
    pub text: String,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ForumCommentRow {
    pub id: String,
    pub author_id: Id,
    pub author: String,
    pub author_role: String,
    /// ISO date-time.
    pub created: String,
    pub text: String,
}

/// The active account's communities, primary first.
#[tauri::command]
#[specta::specta]
pub fn my_communities(core: State<'_, Arc<Core>>) -> List<GroupCard> {
    let Some(a) = core.acting(None) else {
        return List {
            loading: false,
            ..List::loading()
        };
    };
    List::of(core.user_groups(a.0), |g| GroupCard::from(g))
}

#[tauri::command]
#[specta::specta]
pub fn community_search(core: State<'_, Arc<Core>>, query: String) -> List<CommunityHit> {
    if query.trim().chars().count() < 2 {
        return List {
            loading: false,
            ..List::loading()
        };
    }
    List::paged(core.search_communities(&query, false), |g| CommunityHit::from(g))
}

impl From<&bonsai_core::communities::CommunityRow> for CommunityHit {
    fn from(g: &bonsai_core::communities::CommunityRow) -> Self {
        CommunityHit {
            id: Id(g.id),
            name: g.name.clone(),
            description: g.description.clone(),
            members: count(g.members).unwrap_or(0),
            verified: g.verified,
            public_entry: g.public_entry,
        }
    }
}

#[tauri::command]
#[specta::specta]
pub fn community(core: State<'_, Arc<Core>>, group_id: Id) -> CommunityView {
    let g = group_id.0;
    let r = core.community(g);
    let active = core.acting(None);
    let me = active.and_then(|a| core.membership(a, g).data).map(|m| {
        let p = &m.perms;
        MyMembership {
            role: m.role.as_ref().filter(|r| r.rank > 0).map(|r| r.name.clone()),
            rank: m.role.as_ref().map_or(0, |r| r.rank),
            primary: m.primary,
            pending: m.pending,
            owner: m.owner,
            can_view_members: m.can_view_members,
            post_shout: p.post_shout,
            change_rank: p.change_rank,
            remove_members: p.remove_members,
            manage_requests: p.invite_members,
            view_audit_log: p.view_audit_log,
            manage_settings: p.manage_settings,
            view_forums: p.view_forums,
            create_posts: p.create_posts,
            create_comments: p.create_comments,
            remove_posts: p.remove_posts,
            remove_comments: p.remove_comments,
            manage_relationships: p.manage_relationships,
            spend_funds: p.spend_funds,
            view_payouts: p.view_payouts,
        }
    });
    let alts = {
        let labels: Vec<(AccountId, String)> = core
            .store()
            .accounts
            .accounts
            .iter()
            .map(|a| (a.id, a.label.clone().unwrap_or_else(|| a.display_name.clone())))
            .collect();
        core.my_accounts_in(g)
            .into_iter()
            .filter(|m| Some(m.account) != active)
            .filter_map(|m| {
                let (_, label) = labels.iter().find(|(id, _)| *id == m.account)?;
                Some(format!("{label} · {}", m.role))
            })
            .collect()
    };
    CommunityView {
        loading: r.loading,
        error: r.error,
        info: r.data.map(|c| CommunityInfo {
            id: Id(c.id),
            name: c.name.clone(),
            description: c.description.clone(),
            owner_id: Id(c.owner_id),
            owner: c.owner.clone(),
            owner_verified: c.owner_verified,
            members: count(c.members).unwrap_or(0),
            verified: c.verified,
            public_entry: c.public_entry,
            locked: c.locked,
            shout: c.shout.as_ref().map(|s| ShoutView {
                text: s.text.clone(),
                poster_id: Id(s.poster_id),
                poster: s.poster.clone(),
                poster_username: s.poster_username.clone(),
                poster_verified: s.poster_verified,
                updated: s.updated.clone(),
            }),
        }),
        me,
        alts,
    }
}

#[tauri::command]
#[specta::specta]
pub fn community_games(core: State<'_, Arc<Core>>, group_id: Id) -> List<GameCard> {
    let r = core.community_games(group_id.0);
    let s = core.store();
    List::of(r, |c| match s.games.get(&c.universe_id) {
        Some(g) => GameCard::from(g),
        None => GameCard {
            universe_id: Id(c.universe_id),
            place_id: Id(c.place_id),
            name: c.name.clone(),
            creator: String::new(),
            playing: 0,
            rating: None,
            sponsored: false,
            genre: String::new(),
        },
    })
}

/// Clothing and other avatar items the community sells, newest first.
#[tauri::command]
#[specta::specta]
pub fn community_store(core: State<'_, Arc<Core>>, group_id: Id) -> List<ItemCard> {
    let values = core.item_values().data;
    List::paged(core.community_store(group_id.0, false), |i| {
        super::market::card(i, values.as_deref())
    })
}

/// Experiences whose passes the store lists, most visited first.
const PASS_GAMES: usize = 10;

/// Game passes sold in the community's most visited experiences.
#[tauri::command]
#[specta::specta]
pub fn community_passes(core: State<'_, Arc<Core>>, group_id: Id) -> List<CommunityPass> {
    let games = core.community_games(group_id.0);
    let Some(list) = games.data else {
        return List {
            loading: games.loading,
            error: games.error,
            ..List::loading()
        };
    };
    let viewer = viewer(&core);
    let mut out = List {
        loading: false,
        error: None,
        items: Vec::new(),
        more: false,
    };
    for g in list.iter().take(PASS_GAMES) {
        let r = core.game_passes(viewer, g.universe_id, false);
        out.loading |= r.loading;
        out.items
            .extend(r.data.iter().flat_map(|p| &p.items).map(|p| CommunityPass {
                id: Id(p.id),
                name: p.name.clone(),
                price: p.price.and_then(count),
                owned: p.owned,
                place_id: Id(g.place_id),
                game: g.name.clone(),
            }));
    }
    out
}

#[tauri::command]
#[specta::specta]
pub fn community_revenue(core: State<'_, Arc<Core>>, group_id: Id) -> RevenueView {
    let Some(a) = core.acting(None) else {
        return RevenueView {
            funds: None,
            funds_error: None,
            split: List::loading(),
        };
    };
    let funds = core.community_funds(a, group_id.0);
    RevenueView {
        funds: funds.data.map(|r| *r as f64),
        funds_error: funds.error,
        split: List::of(core.community_payouts(a, group_id.0), |p| PayoutRow {
            id: Id(p.user_id),
            name: p.name.clone(),
            percent: p.percent,
        }),
    }
}

#[tauri::command]
#[specta::specta]
pub fn community_affiliates(core: State<'_, Arc<Core>>, group_id: Id) -> AffiliatesView {
    let r = core.community_affiliates(group_id.0);
    let hits = |v: &[bonsai_core::communities::CommunityRow]| v.iter().map(CommunityHit::from).collect();
    AffiliatesView {
        loading: r.loading,
        error: r.error,
        allies: r.data.as_ref().map(|a| hits(&a.allies)).unwrap_or_default(),
        enemies: r.data.as_ref().map(|a| hits(&a.enemies)).unwrap_or_default(),
    }
}

#[tauri::command]
#[specta::specta]
pub fn community_roles(core: State<'_, Arc<Core>>, group_id: Id) -> List<RoleRow> {
    List::of(core.community_roles(group_id.0), |r| RoleRow {
        id: Id(r.id),
        name: r.name.clone(),
        rank: r.rank,
        members: count(r.members).unwrap_or(0),
    })
}

#[tauri::command]
#[specta::specta]
pub fn community_members(core: State<'_, Arc<Core>>, group_id: Id, role_id: Id) -> List<MemberRow> {
    List::paged(
        core.community_members(viewer(&core), group_id.0, role_id.0, false),
        |m| MemberRow {
            id: Id(m.user_id),
            name: m.name.clone(),
            display_name: m.display_name.clone(),
            verified: m.verified,
        },
    )
}

#[tauri::command]
#[specta::specta]
pub fn community_requests(core: State<'_, Arc<Core>>, group_id: Id) -> List<JoinRequestRow> {
    let Some(a) = core.acting(None) else {
        return List::loading();
    };
    List::paged(core.join_requests(a, group_id.0, false), |r| JoinRequestRow {
        id: Id(r.user_id),
        name: r.name.clone(),
        display_name: r.display_name.clone(),
        created: r.created.clone(),
    })
}

#[tauri::command]
#[specta::specta]
pub fn community_audit(core: State<'_, Arc<Core>>, group_id: Id) -> List<AuditRow> {
    let Some(a) = core.acting(None) else {
        return List::loading();
    };
    List::paged(core.audit_log(a, group_id.0, false), |e| AuditRow {
        actor_id: Id(e.actor_id),
        actor: e.actor.clone(),
        actor_role: e.actor_role.clone(),
        action: e.action.clone(),
        text: e.text.clone(),
        created: e.created.clone(),
    })
}

#[tauri::command]
#[specta::specta]
pub fn community_settings(core: State<'_, Arc<Core>>, group_id: Id) -> List<SettingRow> {
    let Some(a) = core.acting(None) else {
        return List::loading();
    };
    let r = core.community_settings(a, group_id.0);
    List {
        loading: r.loading,
        error: r.error,
        more: false,
        items: r
            .data
            .map(|s| {
                Setting::ALL
                    .into_iter()
                    .map(|k| SettingRow {
                        key: k.key().to_owned(),
                        label: k.label().to_owned(),
                        on: k.get(&s),
                    })
                    .collect()
            })
            .unwrap_or_default(),
    }
}

#[tauri::command]
#[specta::specta]
pub fn forum_categories(core: State<'_, Arc<Core>>, group_id: Id) -> List<ForumCategoryRow> {
    List::of(core.forum_categories(viewer(&core), group_id.0), |c| {
        ForumCategoryRow {
            id: c.id.clone(),
            name: c.name.clone(),
            description: c.description.clone(),
        }
    })
}

#[tauri::command]
#[specta::specta]
pub fn forum_posts(core: State<'_, Arc<Core>>, group_id: Id, category: String) -> List<ForumPostRow> {
    List::paged(
        core.forum_posts(viewer(&core), group_id.0, &category, false),
        |p| ForumPostRow {
            id: p.id.clone(),
            title: p.title.clone(),
            author_id: Id(p.author_id),
            author: p.author.clone(),
            author_role: p.author_role.clone(),
            created: p.created.clone(),
            updated: p.updated.clone(),
            replies: count(p.replies).unwrap_or(0),
            pinned: p.pinned,
            locked: p.locked,
            unread: p.unread,
            text: p.text.clone(),
        },
    )
}

#[tauri::command]
#[specta::specta]
pub fn forum_comments(
    core: State<'_, Arc<Core>>,
    group_id: Id,
    category: String,
    post: String,
) -> List<ForumCommentRow> {
    List::paged(
        core.forum_comments(viewer(&core), group_id.0, &category, &post, false),
        |c| ForumCommentRow {
            id: c.id.clone(),
            author_id: Id(c.author_id),
            author: c.author.clone(),
            author_role: c.author_role.clone(),
            created: c.created.clone(),
            text: c.text.clone(),
        },
    )
}

/// Posts to a forum category. Called only from the dialog `group.new_post` opens.
#[tauri::command]
#[specta::specta]
pub fn create_forum_post(
    core: State<'_, Arc<Core>>,
    group_id: Id,
    category: String,
    title: String,
    text: String,
) -> Result<(), String> {
    let a = core
        .acting(None)
        .ok_or_else(|| "Add an account first.".to_owned())?;
    if title.trim().is_empty() || text.trim().is_empty() {
        return Err("Give the post a title and some text.".to_owned());
    }
    core.create_forum_post(a, group_id.0, &category, &title, &text);
    Ok(())
}

/// The community (and open forum post) a command ran from.
struct Page {
    group: u64,
    /// (category, post) while a post is open.
    post: Option<(String, String)>,
}

fn page(context: Option<&str>) -> Option<Page> {
    let rest = context?.strip_prefix("group:")?;
    let mut parts = rest.split('/');
    let group = parts.next()?.parse().ok()?;
    let post = match (parts.next(), parts.next(), parts.next()) {
        (Some("forum"), Some(cat), Some(post)) if !cat.is_empty() && !post.is_empty() => {
            Some((cat.to_owned(), post.to_owned()))
        }
        _ => None,
    };
    Some(Page { group, post })
}

/// Ids in an argument like `1,2,3`.
fn ids(arg: &str) -> Vec<u64> {
    arg.split([',', ' '])
        .filter_map(|s| s.trim().parse().ok())
        .collect()
}

/// A recurring payout split from an argument like `156:60, 2:40`; `none` stops payouts.
fn split_arg(arg: &str) -> Result<Vec<(u64, i32)>, String> {
    if arg.trim().eq_ignore_ascii_case("none") {
        return Ok(Vec::new());
    }
    arg.split(',')
        .filter(|s| !s.trim().is_empty())
        .map(|s| {
            s.split_once(':')
                .and_then(|(u, p)| {
                    Some((
                        u.trim().parse().ok()?,
                        p.trim().trim_end_matches('%').parse().ok()?,
                    ))
                })
                .ok_or_else(|| format!("Can't read “{}”. Use member:percent, like 156:50.", s.trim()))
        })
        .collect()
}

/// A community from an argument: its id or a link to it.
fn community_arg(arg: &str) -> Option<u64> {
    match resolve(arg)? {
        Pasted::Group(id) | Pasted::AnyId(id) => Some(id),
        _ => None,
    }
}

/// Runs a `group.*` command, or returns `None` when `id` isn't one.
pub(super) fn run(
    app: &AppHandle,
    core: &Core,
    id: CommandId,
    inv: &Invocation,
    account: Option<AccountId>,
) -> Option<Result<(), String>> {
    use CommandId as C;
    let handled = matches!(
        id,
        C::JoinCommunity
            | C::LeaveCommunity
            | C::SetPrimaryCommunity
            | C::PostShout
            | C::AcceptJoinRequest
            | C::DeclineJoinRequest
            | C::AcceptSelectedRequests
            | C::DeclineSelectedRequests
            | C::ChangeRank
            | C::KickMember
            | C::NewForumPost
            | C::ReplyForumPost
            | C::DeleteForumPost
            | C::DeleteForumReply
            | C::LeaveSelectedCommunities
            | C::CopyMembersCsv
            | C::ToggleCommunitySetting
            | C::SearchCommunities
            | C::PayOut
            | C::SetRecurringPayouts
            | C::SendAllyRequest
            | C::RemoveAlly
            | C::DeclareEnemy
            | C::RemoveEnemy
    );
    if !handled {
        return None;
    }
    Some(run_group(app, core, id, inv, account))
}

fn run_group(
    app: &AppHandle,
    core: &Core,
    id: CommandId,
    inv: &Invocation,
    account: Option<AccountId>,
) -> Result<(), String> {
    use CommandId as C;
    let a = account.ok_or_else(|| "Add an account first.".to_owned())?;
    let arg = inv.arg.as_deref().map(str::trim).filter(|s| !s.is_empty());
    let here = page(inv.context.as_deref());
    let group = || {
        here.as_ref()
            .map(|p| p.group)
            .ok_or_else(|| "Open a community first.".to_owned())
    };
    // The argument names the community, else the page it ran from.
    let target = || match arg {
        Some(s) => s.parse::<u64>().map_err(|_| "Which community?".to_owned()),
        None => group(),
    };
    match id {
        C::JoinCommunity => {
            core.join_community(a, target()?);
            Ok(())
        }
        C::LeaveCommunity | C::LeaveSelectedCommunities => {
            let list = match arg {
                Some(s) => ids(s),
                None if id == C::LeaveCommunity => vec![group()?],
                None => Vec::new(),
            };
            match list.as_slice() {
                [] => Err("Select communities first.".to_owned()),
                [one] => {
                    core.leave_community(a, *one);
                    Ok(())
                }
                _ => {
                    core.leave_communities(a, list);
                    Ok(())
                }
            }
        }
        C::SetPrimaryCommunity => {
            let g = target()?;
            let m = core
                .membership(a, g)
                .data
                .ok_or_else(|| "Still loading. Try again in a moment.".to_owned())?;
            if !m.is_member() {
                return Err("Join it first.".to_owned());
            }
            core.set_primary_community(a, (!m.primary).then_some(g));
            Ok(())
        }
        C::PostShout => {
            // An empty shout clears it.
            core.post_shout(a, group()?, arg.unwrap_or(""));
            Ok(())
        }
        C::AcceptJoinRequest
        | C::DeclineJoinRequest
        | C::AcceptSelectedRequests
        | C::DeclineSelectedRequests => {
            let users = arg.map(ids).unwrap_or_default();
            if users.is_empty() {
                return Err("Select requests first.".to_owned());
            }
            let accept = matches!(id, C::AcceptJoinRequest | C::AcceptSelectedRequests);
            core.answer_join_requests(a, group()?, users, accept);
            Ok(())
        }
        C::ChangeRank => {
            let (user, role) = arg
                .and_then(|s| s.split_once(':'))
                .and_then(|(u, r)| Some((u.trim().parse().ok()?, r.trim().parse().ok()?)))
                .ok_or_else(|| "Give a user id and a role id, like 156:1234.".to_owned())?;
            core.set_member_role(a, group()?, user, role);
            Ok(())
        }
        C::KickMember => {
            let user = arg
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| "Which member?".to_owned())?;
            core.kick_member(a, group()?, user);
            Ok(())
        }
        C::ReplyForumPost => {
            let text = arg.ok_or_else(|| "Write a reply.".to_owned())?;
            let p = here
                .as_ref()
                .ok_or_else(|| "Open a community first.".to_owned())?;
            let (cat, post_id) = p.post.clone().ok_or_else(|| "Open a post first.".to_owned())?;
            let post = core
                .forum_posts(a, p.group, &cat, false)
                .data
                .and_then(|pg| pg.items.iter().find(|x| x.id == post_id).cloned())
                .ok_or_else(|| "Still loading the post.".to_owned())?;
            core.reply_forum_post(a, p.group, &cat, &post, text);
            Ok(())
        }
        C::DeleteForumPost => {
            let (cat, post) = arg
                .and_then(|s| s.split_once('/'))
                .ok_or_else(|| "Which post?".to_owned())?;
            core.delete_forum_post(a, group()?, cat, post);
            Ok(())
        }
        C::DeleteForumReply => {
            let mut parts = arg.unwrap_or("").splitn(3, '/');
            let (Some(cat), Some(post), Some(comment)) = (parts.next(), parts.next(), parts.next()) else {
                return Err("Which reply?".to_owned());
            };
            core.delete_forum_comment(a, group()?, cat, post, comment);
            Ok(())
        }
        C::ToggleCommunitySetting => {
            let setting = arg
                .and_then(Setting::from_key)
                .ok_or_else(|| "Which setting? approval, enemies, funds or games.".to_owned())?;
            let g = group()?;
            let now = core
                .community_settings(a, g)
                .data
                .ok_or_else(|| "Still loading. Try again in a moment.".to_owned())?;
            core.set_community_setting(a, g, setting, !setting.get(&now));
            Ok(())
        }
        C::CopyMembersCsv => {
            let g = group()?;
            let roles = core.community_roles(g).data.unwrap_or_default();
            let mut csv = String::from("user_id,username,display_name,role\n");
            let mut rows = 0;
            for r in roles.iter() {
                let Some(page) = core.community_members(a, g, r.id, false).data else {
                    continue;
                };
                for m in &page.items {
                    rows += 1;
                    csv.push_str(&format!(
                        "{},{},{},{}\n",
                        m.user_id,
                        csv_field(&m.name),
                        csv_field(&m.display_name),
                        csv_field(&r.name)
                    ));
                }
            }
            if rows == 0 {
                return Err("No members loaded yet. Open the Members tab first.".to_owned());
            }
            copy(app, core, csv, &format!("Copied {rows} members as CSV."))
        }
        C::PayOut => {
            // A plugin names the community: "group:<id> <user id> <amount>".
            let (g, arg) = match arg.map(str::trim).and_then(|s| s.strip_prefix("group:")) {
                Some(rest) => {
                    let (g, rest) = rest.split_once(' ').unwrap_or((rest, ""));
                    let g = g.parse::<u64>().map_err(|_| "Which community?".to_owned())?;
                    (Some(g), Some(rest))
                }
                None => (None, arg),
            };
            let (who, amount) = arg
                .and_then(|s| s.trim().rsplit_once(' '))
                .and_then(|(who, n)| Some((who.trim().to_owned(), n.trim().parse::<i64>().ok()?)))
                .filter(|(who, n)| !who.is_empty() && *n > 0)
                .ok_or_else(|| "Give a member and an amount, like builderman 100.".to_owned())?;
            let g = g.map_or_else(group, Ok)?;
            core.with_user(&who, move |c, user| c.pay_out(a, g, user, amount))
        }
        C::SetRecurringPayouts => {
            let split = split_arg(arg.ok_or_else(|| {
                "Give member:percent pairs, like 156:50, or none to stop them.".to_owned()
            })?)?;
            core.set_recurring_payouts(a, group()?, split)
        }
        C::SendAllyRequest | C::RemoveAlly | C::DeclareEnemy | C::RemoveEnemy => {
            let other = arg
                .and_then(community_arg)
                .ok_or_else(|| "Which community? Give its ID or link.".to_owned())?;
            let g = group()?;
            if other == g {
                return Err("That's this community.".to_owned());
            }
            let kind = if matches!(id, C::SendAllyRequest | C::RemoveAlly) {
                Affiliation::Ally
            } else {
                Affiliation::Enemy
            };
            if matches!(id, C::SendAllyRequest | C::DeclareEnemy) {
                core.add_affiliate(a, g, kind, other);
            } else {
                core.remove_affiliate(a, g, kind, other);
            }
            Ok(())
        }
        // These open a dialog or a screen in the UI; they only land here without one.
        C::NewForumPost => Err("Open a community's forum first.".to_owned()),
        C::SearchCommunities => Err("Open Communities to search.".to_owned()),
        _ => Err(format!("not wired yet: {}", id.key())),
    }
}

/// `list.more` on a community page: `members:<roleId>`, `requests`, `store`, `audit`,
/// `posts:<category>` or `comments` (of the open post).
pub(super) fn more(core: &Core, inv: &Invocation, account: Option<AccountId>) -> Result<(), String> {
    let p = page(inv.context.as_deref()).ok_or_else(|| "Open a community first.".to_owned())?;
    let viewer = account.unwrap_or(AccountId::ANONYMOUS);
    let which = inv.arg.as_deref().unwrap_or("").trim();
    let (what, rest) = which.split_once(':').unwrap_or((which, ""));
    match what {
        "members" => {
            let role = rest.parse().map_err(|_| "Which role?".to_owned())?;
            drop(core.community_members(viewer, p.group, role, true));
        }
        "requests" => drop(core.join_requests(viewer, p.group, true)),
        "store" => drop(core.community_store(p.group, true)),
        "audit" => drop(core.audit_log(viewer, p.group, true)),
        "posts" => drop(core.forum_posts(viewer, p.group, rest, true)),
        "comments" => {
            let (cat, post) = p.post.ok_or_else(|| "Open a post first.".to_owned())?;
            drop(core.forum_comments(viewer, p.group, &cat, &post, true));
        }
        _ => return Err("Nothing more to load here.".to_owned()),
    }
    Ok(())
}

pub(super) fn csv_field(s: &str) -> String {
    if s.contains([',', '"', '\n']) {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_owned()
    }
}

impl From<&bonsai_core::people::GroupRole> for GroupCard {
    fn from(g: &bonsai_core::people::GroupRole) -> Self {
        GroupCard {
            id: Id(g.group_id),
            name: g.name.clone(),
            members: count(g.members).unwrap_or(0),
            verified: g.verified,
            role: g.role.clone(),
            primary: g.primary,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_context() {
        let p = page(Some("group:7")).unwrap();
        assert_eq!((p.group, p.post), (7, None));
        let p = page(Some("group:7/forum/cat/post")).unwrap();
        assert_eq!(p.post, Some(("cat".into(), "post".into())));
        assert!(page(Some("game:7")).is_none());
        assert!(page(Some("group:x")).is_none());
    }

    #[test]
    fn id_lists_and_csv() {
        assert_eq!(ids("1, 2,x,3"), vec![1, 2, 3]);
        assert_eq!(csv_field("a,b"), "\"a,b\"");
        assert_eq!(csv_field("say \"hi\""), "\"say \"\"hi\"\"\"");
        assert_eq!(csv_field("plain"), "plain");
    }

    #[test]
    fn payout_split_and_community_args() {
        assert_eq!(split_arg("156:60, 2:40%"), Ok(vec![(156, 60), (2, 40)]));
        assert_eq!(split_arg("None"), Ok(vec![]));
        assert!(split_arg("156=60").is_err());
        assert_eq!(community_arg("7001"), Some(7001));
        assert_eq!(
            community_arg("https://www.roblox.com/communities/7001/Test-Community"),
            Some(7001)
        );
        assert_eq!(community_arg("https://www.roblox.com/users/1/profile"), None);
    }
}

//! Remote search for the command bar and the search results page. Each source runs in
//! parallel; a failed source just contributes no rows. A query that is exactly a username puts
//! that user first, and a pasted link or bare id gets rows for what it points at.

use bonsai_api::generated::{apis, groups, users};
use bonsai_api::{AccountId, Client};

use crate::Core;
use crate::core::Inner;
use crate::res::{Key, Res};
use crate::resolve::{Pasted, resolve};
use crate::search::{Entry, Group, Scope, Target};
use crate::store::GameCard;

/// Whether the remote search should run for `q` (the query without its scope prefix): three
/// letters, or something that resolves to an id or link.
pub(crate) fn searchable(q: &str) -> bool {
    q.chars().count() >= 3 || resolve(q).is_some()
}

/// Could be a Roblox username: 3 to 20 letters, digits or underscores.
fn username_like(q: &str) -> bool {
    (3..=20).contains(&q.len()) && q.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// Rows for a pasted link or bare id, without a network call. A bare id gets one row per kind
/// of thing it could be; the user row comes from `user_by_id` instead.
fn link_entries(p: &Pasted) -> Vec<Entry> {
    let row = |title: String, target: Target| Entry {
        group: Group::Links,
        title,
        subtitle: String::new(),
        aliases: Vec::new(),
        target,
        boost: 0,
    };
    match *p {
        Pasted::Place(id)
        | Pasted::Server { place_id: id, .. }
        | Pasted::PrivateServerLink { place_id: id, .. } => {
            vec![row(format!("Experience {id}"), Target::Place(id))]
        }
        Pasted::User(id) => vec![row(format!("User {id}"), Target::User(id))],
        Pasted::Asset(id) => vec![row(format!("Item {id}"), Target::Asset(id))],
        Pasted::Bundle(id) => vec![row(format!("Bundle {id}"), Target::Bundle(id))],
        Pasted::Group(id) => vec![row(format!("Community {id}"), Target::Group(id))],
        Pasted::AnyId(id) => vec![
            row(format!("Experience with place ID {id}"), Target::Place(id)),
            row(format!("Item with ID {id}"), Target::Asset(id)),
            row(format!("Community with ID {id}"), Target::Group(id)),
        ],
        _ => Vec::new(),
    }
}

fn user_entry(id: u64, name: String, display: Option<String>, boost: u32) -> Entry {
    Entry {
        group: Group::Users,
        title: display.filter(|d| !d.is_empty()).unwrap_or_else(|| name.clone()),
        subtitle: format!("@{name}"),
        aliases: vec![name],
        target: Target::User(id),
        boost,
    }
}

/// `full` asks for more rows per group, for the search results page; the command bar takes
/// the first few of each.
pub(crate) async fn search(inner: &Inner, c: &Client, scope: Scope, q: &str, full: bool) -> Vec<Entry> {
    let want = |g: Group| g.in_scope(scope);
    // Roblox only takes certain page sizes: 10 or 25 for people and communities, 10 or 30 for items.
    let (people_n, groups_n, items_n, games_n) = if full { (25, 25, 30, 40) } else { (10, 10, 10, 10) };
    let pasted = resolve(q);
    let links = match &pasted {
        Some(p) if scope == Scope::All => link_entries(p),
        _ => Vec::new(),
    };
    // A link or id is not a name; only search by name for plain text.
    let text = pasted.is_none() && q.chars().count() >= 3;
    let exact = async {
        if !want(Group::Users) {
            return None;
        }
        match pasted {
            Some(Pasted::User(id) | Pasted::AnyId(id)) => {
                let u = users::v1::get_users_by_user_id(c, id as i64).await.ok()?;
                Some(user_entry(id, u.name?, u.display_name, 100))
            }
            None if username_like(q) => {
                let body = users::v1::MultiGetByUsernameRequest {
                    usernames: Some(vec![q.to_string()]),
                    exclude_banned_users: Some(true),
                };
                let u = users::v1::post_usernames_users(c, &body)
                    .await
                    .ok()?
                    .data?
                    .into_iter()
                    .next()?;
                Some(user_entry(u.id? as u64, u.name?, u.display_name, 100))
            }
            _ => None,
        }
    };
    let people = async {
        if !want(Group::Users) || !text {
            return Vec::new();
        }
        let p = users::v1::GetUsersSearchParams {
            keyword: q.to_string(),
            limit: Some(people_n),
            ..Default::default()
        };
        users::v1::get_users_search(c, &p)
            .await
            .map(|r| {
                r.data
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|u| {
                        let mut e = user_entry(u.id? as u64, u.name?, u.display_name, 0);
                        e.aliases.extend(u.previous_usernames.unwrap_or_default());
                        Some(e)
                    })
                    .collect()
            })
            .unwrap_or_default()
    };
    let communities = async {
        if !want(Group::Communities) || !text {
            return Vec::new();
        }
        let p = groups::v1::GetGroupsSearchParams {
            keyword: q.to_string(),
            limit: Some(groups_n),
            ..Default::default()
        };
        groups::v1::get_groups_search(c, &p)
            .await
            .map(|r| {
                r.data
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|g| {
                        let id = g.id? as u64;
                        Some(Entry {
                            group: Group::Communities,
                            title: g.name.unwrap_or_default(),
                            subtitle: format!("{} members", g.member_count.unwrap_or(0)),
                            aliases: Vec::new(),
                            target: Target::Group(id),
                            boost: 0,
                        })
                    })
                    .collect()
            })
            .unwrap_or_default()
    };
    let games = async {
        if !want(Group::Experiences) || !text {
            return Vec::new();
        }
        let p = apis::undocumented::GetOmniSearchParams {
            search_query: q.to_string(),
            session_id: inner.session_id.clone(),
            page_type: Some("games".into()),
            ..Default::default()
        };
        apis::undocumented::get_omni_search(c, &p)
            .await
            .map(|r| {
                r.search_results
                    .unwrap_or_default()
                    .into_iter()
                    .filter(|g| g.content_group_type.as_deref() == Some("Game"))
                    .flat_map(|g| g.contents.unwrap_or_default())
                    .filter_map(GameCard::from_api)
                    .filter(|g| !g.sponsored)
                    .take(games_n)
                    .map(|g| Entry {
                        group: Group::Experiences,
                        subtitle: game_subtitle(&g),
                        title: g.name,
                        aliases: Vec::new(),
                        target: Target::Experience {
                            universe_id: g.universe_id,
                            place_id: g.place_id,
                        },
                        boost: 0,
                    })
                    .collect()
            })
            .unwrap_or_default()
    };
    let items = async {
        if !want(Group::Items) || !text {
            return Vec::new();
        }
        let q = crate::market::CatalogQuery {
            keyword: q.to_string(),
            ..Default::default()
        };
        crate::market::search_entries(c, &q, items_n)
            .await
            .unwrap_or_default()
    };
    let (exact, mut a, b, g, i) = tokio::join!(exact, games, people, communities, items);
    let mut out = links;
    out.extend(exact.clone());
    out.append(&mut a);
    // The exact match is already first; drop its copy from the keyword results.
    out.extend(
        b.into_iter()
            .filter(|e| exact.as_ref().is_none_or(|x| x.target != e.target)),
    );
    out.extend(g);
    out.extend(i);
    out
}

/// Recent searches kept per account.
const RECENT_SEARCHES: usize = 20;

/// The search results page wants a query again within this long.
const RESULTS_AGE: i64 = 300;

impl Core {
    /// Results for the search results page: every kind of thing, or one kind (`scope`) with
    /// more rows. Cached per query, unlike the command bar's single in-flight search.
    pub fn search_results(&self, account: AccountId, scope: Scope, query: &str) -> Res<Vec<Entry>> {
        let q = query.trim().to_string();
        if !searchable(&q) {
            return Res::default();
        }
        self.inner.res(
            Key::private(
                account,
                "search_results",
                format!("{scope:?}:{}", q.to_lowercase()),
            ),
            RESULTS_AGE,
            move |f| async move { Ok(search(&f.inner, &f.client, scope, &q, scope != Scope::All).await) },
        )
    }

    /// What `account` searched for lately, newest first. Kept on this computer only.
    pub fn recent_searches(&self, account: AccountId) -> Vec<String> {
        self.local(account, "recent_searches")
    }

    /// Put `query` at the top of the recent searches.
    pub fn note_search(&self, account: AccountId, query: &str) {
        let q = query.trim();
        if q.is_empty() {
            return;
        }
        let mut v = self.recent_searches(account);
        v.retain(|x| !x.eq_ignore_ascii_case(q));
        v.insert(0, q.to_string());
        v.truncate(RECENT_SEARCHES);
        self.set_local(account, "recent_searches", &v);
    }

    pub fn clear_searches(&self, account: AccountId) {
        self.set_local(account, "recent_searches", &Vec::<String>::new());
    }
}

fn game_subtitle(g: &GameCard) -> String {
    let mut parts = vec![format!("{} playing", crate::fmt::count(g.playing))];
    if let Some(r) = g.rating() {
        parts.push(format!("{r}% rating"));
    }
    if !g.creator.is_empty() {
        parts.push(format!("by {}", g.creator));
    }
    parts.join(" · ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn searchable_queries() {
        assert!(searchable("adopt"));
        assert!(!searchable("ad"));
        assert!(searchable("1"));
        assert!(searchable("roblox.com/users/1/profile"));
    }

    #[test]
    fn usernames() {
        assert!(username_like("builder_man1"));
        assert!(!username_like("ab"));
        assert!(!username_like("adopt me"));
        assert!(!username_like("a_name_that_is_far_too_long"));
    }

    #[test]
    fn links_and_ids() {
        let rows = link_entries(&Pasted::AnyId(42));
        let targets: Vec<_> = rows.iter().map(|e| e.target.clone()).collect();
        assert_eq!(
            targets,
            vec![Target::Place(42), Target::Asset(42), Target::Group(42)]
        );
        let rows = link_entries(&resolve("https://www.roblox.com/games/920587237/Adopt-Me").unwrap());
        assert_eq!(rows[0].target, Target::Place(920587237));
        assert!(link_entries(&Pasted::Badge(1)).is_empty());
    }
}

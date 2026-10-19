//! Roblox Wiki articles as the wiki screens and tabs show them. Nothing is fetched until the
//! user turns the wiki on; see `bonsai_core::wiki`.

use std::sync::Arc;

use bonsai_core::Core;
use bonsai_core::commands::CommandId;
use bonsai_core::res::Res;
use bonsai_core::wiki::{self, About, Article, Block, Hit, Page, RobloxRef};
use serde::Serialize;
use specta::Type;
use tauri::State;

use super::{Id, Invocation};

/// Text is plain with `[[Title]]` and `[[Title|label]]` links to other articles.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum WikiBlock {
    P {
        text: String,
    },
    List {
        ordered: bool,
        items: Vec<String>,
    },
    Table {
        columns: Vec<String>,
        rows: Vec<Vec<String>>,
    },
    Note {
        text: String,
    },
}

impl From<&Block> for WikiBlock {
    fn from(b: &Block) -> Self {
        match b {
            Block::P { text } => WikiBlock::P { text: text.clone() },
            Block::List { ordered, items } => WikiBlock::List {
                ordered: *ordered,
                items: items.clone(),
            },
            Block::Table { columns, rows } => WikiBlock::Table {
                columns: columns.clone(),
                rows: rows.clone(),
            },
            Block::Note { text } => WikiBlock::Note { text: text.clone() },
        }
    }
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct WikiSection {
    pub id: String,
    pub title: String,
    pub level: u8,
    pub blocks: Vec<WikiBlock>,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct WikiFact {
    pub key: String,
    pub label: String,
    pub value: String,
}

/// The Bonsai page an article is about, from the roblox.com links in it.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum WikiTarget {
    Item {
        id: Id,
    },
    #[serde(rename_all = "camelCase")]
    Game {
        place_id: Id,
    },
    User {
        id: Id,
    },
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct WikiArticle {
    /// What the wiki calls the page, like `Catalog:Dominus Infernus`. Identifies it.
    pub title: String,
    /// The title without its namespace, for headings.
    pub name: String,
    /// `item`, `game`, `player` or `page`.
    pub kind: String,
    /// The first paragraph.
    pub extract: String,
    /// Text before the first heading.
    pub lead: Vec<WikiBlock>,
    pub sections: Vec<WikiSection>,
    pub infobox_title: String,
    pub infobox: Vec<WikiFact>,
    pub categories: Vec<String>,
    pub target: Option<WikiTarget>,
    /// The article on the site, shown for attribution. Opened with `wiki.open_site`.
    pub url: String,
    pub revision: Id,
}

/// One article, or why there is none. `article` is `None` with nothing loading and no error
/// when the wiki has no such page.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct WikiPageView {
    /// The user turned the wiki on.
    pub enabled: bool,
    pub loading: bool,
    pub error: Option<String>,
    /// ms since the epoch; `None` before the first load.
    pub fetched_at: Option<f64>,
    pub article: Option<WikiArticle>,
    pub source: String,
    pub license: String,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct WikiHit {
    pub title: String,
    pub name: String,
    pub kind: String,
    pub snippet: String,
    /// RFC 3339; empty when the wiki didn't say.
    pub edited: String,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct WikiSearchView {
    pub enabled: bool,
    pub loading: bool,
    pub error: Option<String>,
    pub hits: Vec<WikiHit>,
}

const CATALOG: &str = "Catalog:";

/// What a page shows as its name: the title without its namespace, and an experience without
/// the group or player it is filed under (`Badimo/Jailbreak` → `Jailbreak`).
fn name_of(title: &str) -> String {
    let t = title.strip_prefix(CATALOG).unwrap_or(title);
    let t = t.strip_prefix(wiki::PLAYER).unwrap_or(t);
    match wiki::kind_of(title) {
        "game" => t.rsplit('/').next().unwrap_or(t).to_owned(),
        _ => t.to_owned(),
    }
}

fn target(a: &Article) -> Option<WikiTarget> {
    // Only an item article is about the item it links; others may mention items in passing.
    let item = a.title.starts_with(CATALOG);
    a.roblox.iter().find_map(|r| match *r {
        RobloxRef::Item(id) if item => Some(WikiTarget::Item { id: Id(id) }),
        RobloxRef::Game(id) if !item => Some(WikiTarget::Game { place_id: Id(id) }),
        RobloxRef::User(id) if !item => Some(WikiTarget::User { id: Id(id) }),
        _ => None,
    })
}

fn kind(title: &str, target: Option<&WikiTarget>) -> String {
    match target {
        _ if title.starts_with(CATALOG) => "item",
        Some(WikiTarget::Game { .. }) => "game",
        Some(WikiTarget::User { .. }) => "player",
        _ => wiki::kind_of(title),
    }
    .to_owned()
}

fn article(a: &Article) -> WikiArticle {
    let t = target(a);
    WikiArticle {
        name: name_of(&a.title),
        kind: kind(&a.title, t.as_ref()),
        extract: a.extract().to_owned(),
        lead: a.lead.iter().map(WikiBlock::from).collect(),
        sections: a
            .sections
            .iter()
            .map(|s| WikiSection {
                id: s.id.clone(),
                title: s.title.clone(),
                level: s.level,
                blocks: s.blocks.iter().map(WikiBlock::from).collect(),
            })
            .collect(),
        infobox_title: a.infobox_title.clone(),
        infobox: a
            .infobox
            .iter()
            .map(|f| WikiFact {
                key: f.key.clone(),
                label: f.label.clone(),
                value: f.value.clone(),
            })
            .collect(),
        categories: a.categories.clone(),
        target: t,
        url: wiki::site_url(&a.title),
        revision: Id(a.revision),
        title: a.title.clone(),
    }
}

fn page_view(core: &Core, r: Res<Page>) -> WikiPageView {
    WikiPageView {
        enabled: core.wiki_enabled(),
        loading: r.loading,
        error: r.error.clone(),
        fetched_at: r
            .data
            .as_ref()
            .and_then(|_| (r.fetched_at > 0).then_some(r.fetched_at as f64 * 1000.0)),
        article: r.data.as_ref().and_then(|p| p.article.as_ref()).map(article),
        source: wiki::SOURCE.to_owned(),
        license: wiki::LICENSE.to_owned(),
    }
}

/// The article called `title` (as the wiki names it, `Catalog:` included for items).
#[tauri::command]
#[specta::specta]
pub fn wiki_page(core: State<'_, Arc<Core>>, title: String) -> WikiPageView {
    page_view(&core, core.wiki_page(&title))
}

/// The article about an item (`item`, `bundle`), experience (`game`, by place id) or player
/// (`user`) called `name`, if the wiki has one about this very one. `creator` and
/// `creator_group` place an experience's article under its group or player.
#[tauri::command]
#[specta::specta]
pub fn wiki_about(
    core: State<'_, Arc<Core>>,
    kind: String,
    id: Id,
    name: String,
    creator: String,
    creator_group: bool,
) -> WikiPageView {
    let about = match kind.as_str() {
        "item" => About::Item { id: id.0, name },
        "bundle" => About::Bundle { id: id.0, name },
        "game" => About::Game {
            place: id.0,
            name,
            creator,
            group: creator_group,
        },
        "user" => About::User { id: id.0, name },
        _ => return page_view(&core, Res::default()),
    };
    page_view(&core, core.wiki_about(&about))
}

fn hit(h: &Hit) -> WikiHit {
    WikiHit {
        name: name_of(&h.title),
        kind: kind(&h.title, None),
        snippet: h.snippet.clone(),
        edited: h.edited.clone(),
        title: h.title.clone(),
    }
}

#[tauri::command]
#[specta::specta]
pub fn wiki_search(core: State<'_, Arc<Core>>, query: String) -> WikiSearchView {
    let r = core.wiki_search(&query);
    WikiSearchView {
        enabled: core.wiki_enabled(),
        loading: r.loading,
        error: r.error,
        hits: r.data.map(|h| h.iter().map(hit).collect()).unwrap_or_default(),
    }
}

pub fn run(core: &Arc<Core>, id: CommandId, inv: &Invocation) -> Option<Result<(), String>> {
    Some(match id {
        CommandId::WikiOpenSite => {
            // An argument names the article; otherwise the article page it ran from.
            let title = inv
                .arg
                .clone()
                .filter(|a| !a.trim().is_empty())
                .or_else(|| {
                    inv.context
                        .as_deref()
                        .and_then(|c| c.strip_prefix("wiki:"))
                        .map(str::to_owned)
                })
                .unwrap_or_default();
            core.open_wiki_site(&title)
        }
        CommandId::ToggleWiki => {
            let on = !core.wiki_enabled();
            core.update_settings(|s| s.wiki = on);
            core.notice(
                if on {
                    "Wiki articles from Fandom are on."
                } else {
                    "Wiki articles are off."
                },
                false,
            );
            Ok(())
        }
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn targets_and_kinds() {
        let item = Article {
            title: "Catalog:Dominus Infernus".into(),
            roblox: vec![RobloxRef::User(1), RobloxRef::Item(7)],
            ..Default::default()
        };
        let v = article(&item);
        assert_eq!((v.kind.as_str(), v.name.as_str()), ("item", "Dominus Infernus"));
        assert!(matches!(v.target, Some(WikiTarget::Item { id: Id(7) })));
        let game = Article {
            title: "Jailbreak".into(),
            roblox: vec![RobloxRef::Item(3), RobloxRef::Game(606849621)],
            ..Default::default()
        };
        assert_eq!(
            article(&game).kind,
            "game",
            "items mentioned in passing don't count"
        );
        assert_eq!(article(&Article::default()).kind, "page");
        let named = |title: &str| {
            let a = article(&Article {
                title: title.into(),
                ..Default::default()
            });
            (a.kind, a.name)
        };
        assert_eq!(named("Badimo/Jailbreak"), ("game".into(), "Jailbreak".into()));
        assert_eq!(named("Player:Builderman"), ("player".into(), "Builderman".into()));
        assert_eq!(
            named("Player:Builderman/Some Game"),
            ("game".into(), "Some Game".into())
        );
        assert_eq!(v.url, "https://roblox.fandom.com/wiki/Catalog:Dominus_Infernus");
    }
}

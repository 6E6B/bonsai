//! Roblox Wiki articles, and articles from experiences' own Fandom wikis, as the wiki screens
//! and tabs show them. Nothing is fetched until the user turns the wiki on; see
//! `bonsai_core::wiki`.

use std::sync::Arc;

use bonsai_core::Core;
use bonsai_core::commands::CommandId;
use bonsai_core::res::Res;
use bonsai_core::wiki::{self, About, Article, Block, Hit, Image, Page, ROBLOX, RobloxRef};
use serde::Serialize;
use specta::Type;
use tauri::State;

use super::{Id, Invocation};

/// Text is plain with `[[Title]]` and `[[Title|label]]` links. A title is an article (on
/// another wiki: `w:c:jailbreak:Torpedo`), `@item:<id>` (also `@bundle`, `@game` by place,
/// `@user`, `@group`) a Roblox page Bonsai opens, and `https://…` another site, opened with
/// `wiki.open_link`.
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
    Images {
        images: Vec<WikiImage>,
    },
}

/// A picture. Shown from `wikiimg://` with `src` as the path.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct WikiImage {
    pub src: String,
    /// Text with links.
    pub caption: String,
    /// What the article shows it at; 0 when unknown.
    pub width: u32,
    pub height: u32,
}

impl From<&Image> for WikiImage {
    fn from(i: &Image) -> Self {
        WikiImage {
            src: i.src.clone(),
            caption: i.caption.clone(),
            width: i.width,
            height: i.height,
        }
    }
}

/// Another Fandom wiki an article points to, like an experience's own.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct WikiLink {
    /// Its main page as a title: `w:c:jailbreak:`.
    pub title: String,
    pub name: String,
    /// Its subdomain, for searching it.
    pub wiki: String,
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
            Block::Images { images } => WikiBlock::Images {
                images: images.iter().map(WikiImage::from).collect(),
            },
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
    /// What the wiki calls the page, like `Catalog:Dominus Infernus`, or on another wiki
    /// `w:c:jailbreak:Torpedo`. Identifies it.
    pub title: String,
    /// The wiki it is on: `roblox` for the Roblox Wiki, else its subdomain.
    pub wiki: String,
    /// That wiki's name, for the byline: "Roblox Wiki", "Jailbreak Wiki".
    pub source: String,
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
    /// The infobox picture.
    pub image: Option<WikiImage>,
    pub categories: Vec<String>,
    pub target: Option<WikiTarget>,
    /// Other wikis it sends readers to, such as the experience's own.
    pub wikis: Vec<WikiLink>,
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
    /// The wiki searched, and its name when known ("Jailbreak Wiki").
    pub wiki: String,
    pub enabled: bool,
    pub loading: bool,
    pub error: Option<String>,
    pub hits: Vec<WikiHit>,
}

const CATALOG: &str = "Catalog:";

/// What a page shows as its name: the title without its namespace, and an experience without
/// the group or player it is filed under (`Badimo/Jailbreak` → `Jailbreak`).
fn name_of(title: &str) -> String {
    let (w, local) = wiki::split_title(title);
    if w != ROBLOX {
        return local.to_owned();
    }
    let t = title.strip_prefix(CATALOG).unwrap_or(title);
    let t = t.strip_prefix(wiki::PLAYER).unwrap_or(t);
    match wiki::kind_of(title) {
        "game" => t.rsplit('/').next().unwrap_or(t).to_owned(),
        _ => t.to_owned(),
    }
}

fn target(a: &Article) -> Option<WikiTarget> {
    // An experience's own wiki links the experience on every page; that says nothing.
    if wiki::split_title(&a.title).0 != ROBLOX {
        return None;
    }
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
    let (w, _) = wiki::split_title(&a.title);
    WikiArticle {
        wiki: w.to_owned(),
        source: if a.site.is_empty() {
            wiki::SOURCE.to_owned()
        } else {
            a.site.clone()
        },
        name: if a.main && !a.site.is_empty() {
            a.site.clone()
        } else {
            name_of(&a.title)
        },
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
        image: a.image.as_ref().map(WikiImage::from),
        categories: a.categories.clone(),
        target: t,
        wikis: a
            .wikis
            .iter()
            .map(|o| WikiLink {
                title: wiki::join_title(&o.wiki, ""),
                name: o.name.clone(),
                wiki: o.wiki.clone(),
            })
            .collect(),
        url: wiki::site_url(&a.title),
        revision: Id(a.revision),
        title: a.title.clone(),
    }
}

fn page_view(core: &Core, r: Res<Page>) -> WikiPageView {
    let article = r.data.as_ref().and_then(|p| p.article.as_ref()).map(article);
    WikiPageView {
        enabled: core.wiki_enabled(),
        loading: r.loading,
        error: r.error.clone(),
        fetched_at: r
            .data
            .as_ref()
            .and_then(|_| (r.fetched_at > 0).then_some(r.fetched_at as f64 * 1000.0)),
        source: article
            .as_ref()
            .map_or_else(|| wiki::SOURCE.to_owned(), |a| a.source.clone()),
        article,
        license: wiki::LICENSE.to_owned(),
    }
}

/// The article called `title` (as the wiki names it, `Catalog:` included for items;
/// `w:c:<wiki>:<title>` on another wiki, `w:c:<wiki>:` for its main page).
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

/// Search `wiki` (`roblox`, or an experience's wiki like `jailbreak`) for `query`.
#[tauri::command]
#[specta::specta]
pub fn wiki_search(core: State<'_, Arc<Core>>, wiki: String, query: String) -> WikiSearchView {
    let r = core.wiki_search(&wiki, &query);
    WikiSearchView {
        wiki: if wiki.is_empty() { ROBLOX.to_owned() } else { wiki },
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
        CommandId::WikiOpenLink => {
            // `<article title>\n<url>`, or the url alone from the article page it ran from.
            let arg = inv.arg.clone().unwrap_or_default();
            let (title, url) = match arg.split_once('\n') {
                Some((t, u)) => (t.to_owned(), u.trim().to_owned()),
                None => (
                    inv.context
                        .as_deref()
                        .and_then(|c| c.strip_prefix("wiki:"))
                        .unwrap_or_default()
                        .to_owned(),
                    arg.trim().to_owned(),
                ),
            };
            core.open_wiki_link(&title, &url)
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
        assert_eq!((v.wiki.as_str(), v.source.as_str()), ("roblox", "Roblox Wiki"));
        let other = article(&Article {
            title: "w:c:jailbreak:Jailbreak Wiki:Home".into(),
            site: "Jailbreak Wiki".into(),
            main: true,
            roblox: vec![RobloxRef::Game(606849621)],
            ..Default::default()
        });
        assert_eq!(
            (other.wiki.as_str(), other.name.as_str(), other.kind.as_str()),
            ("jailbreak", "Jailbreak Wiki", "page")
        );
        assert!(other.target.is_none(), "an experience's wiki links it everywhere");
        assert_eq!(other.url, "https://jailbreak.fandom.com/wiki/Jailbreak_Wiki:Home");
        assert_eq!(named("w:c:jailbreak:Torpedo"), ("page".into(), "Torpedo".into()));
    }
}

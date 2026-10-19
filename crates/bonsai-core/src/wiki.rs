//! Articles about items, experiences and players from the Roblox Wiki (roblox.fandom.com), a
//! community wiki on Fandom, a third-party site. Opt-in (`Settings::wiki`): nothing is fetched
//! until the user turns it on, and every article shows its source and license (CC BY-SA).
//!
//! The page's HTML is parsed here and only plain text with `[[Title|label]]` links leaves this
//! module, so no markup, scripts or styles from Fandom ever reach the webview. Images leave as
//! paths on Fandom's image host, which the app serves over its own protocol after fetching
//! them here ([`Core::wiki_image`]); the webview never loads them from Fandom.
//! The request goes out with the anonymous client, and the transport never sends Roblox
//! credentials to a non-Roblox host anyway.
//!
//! Experiences with a wiki of their own on Fandom (jailbreak.fandom.com) are read the same way.
//! Their articles are titled the way MediaWiki links between wikis, `w:c:jailbreak:Torpedo`,
//! and `w:c:jailbreak:` is that wiki's main page.

use std::collections::BTreeSet;

use bonsai_api::Error;
/// The Roblox Wiki's name on Fandom; other wikis are named by their subdomain.
pub use bonsai_api::fandom::ROBLOX;
use bonsai_api::fandom::{self as wikis, IMAGE_HOST};
use bonsai_api::generated::roblox_fandom_com::undocumented as fandom;
use dom_query::{Document, NodeRef};
use serde::{Deserialize, Serialize};

use crate::res::{Key, Res};
use crate::resolve::{Pasted, resolve};
use crate::{Core, thumbs};
use tokio::sync::oneshot;

/// Shown under every article.
pub const SOURCE: &str = "Roblox Wiki";
/// Fandom's text license. Credit and a link back are part of it.
pub const LICENSE: &str = "CC BY-SA 3.0";

/// Articles change rarely; a day-old copy is fine and the saved one shows at once.
const ARTICLE_AGE: i64 = 12 * 3600;
const SEARCH_AGE: i64 = 3600;
/// MediaWiki allows up to 500; the screen shows the first page of results.
const SEARCH_LIMIT: i32 = 50;
/// The namespace items live in on the Roblox Wiki: `Catalog:Dominus Infernus`.
const CATALOG: &str = "Catalog:";
/// Players and the experiences they made: `Player:Builderman`, `Player:Builderman/Some Game`.
pub const PLAYER: &str = "Player:";
/// Main, Catalog (112) and Player (114), from the wiki's `meta=siteinfo` on 2026-10-19. Talk,
/// user, blog and template pages are left out of search.
const SEARCH_NAMESPACES: &str = "0|112|114";
/// Other wikis: articles only.
const OTHER_NAMESPACES: &str = "0";
/// How a title names an article on another wiki: `w:c:jailbreak:Torpedo`.
pub const OTHER_WIKI: &str = "w:c:";
/// Smaller images are icons beside text, not pictures.
const MIN_IMAGE: u32 = 40;

/// One block of article text. Text is plain with `[[Title]]` or `[[Title|label]]` links to
/// other articles on the same wiki.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Block {
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
    /// A quote, hatnote or notice box.
    Note {
        text: String,
    },
    /// A picture with its caption, or a gallery of them.
    Images {
        images: Vec<Image>,
    },
}

/// A picture in an article.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Image {
    /// Path and query on [`IMAGE_HOST`], without the leading `/`.
    pub src: String,
    /// With links, like block text.
    pub caption: String,
    /// The size the article shows it at; 0 when it didn't say.
    pub width: u32,
    pub height: u32,
}

/// Another Fandom wiki a notice in the article points to as the place to read more, like the
/// Roblox Wiki's "This experience has a dedicated wiki" box.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct OtherWiki {
    /// Its subdomain: `jailbreak`.
    pub wiki: String,
    /// What the link calls it: "Jailbreak Wiki".
    pub name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Section {
    /// The heading's anchor on the site.
    pub id: String,
    pub title: String,
    /// 2 for `==`, 3 for `===`, ...
    pub level: u8,
    pub blocks: Vec<Block>,
}

/// One row of the infobox.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Fact {
    /// The template field (`price`, `creator`); empty for a classic table infobox.
    pub key: String,
    pub label: String,
    pub value: String,
}

/// A Roblox page an article links to, such as the item it is about.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "kind", content = "id", rename_all = "lowercase")]
pub enum RobloxRef {
    Item(u64),
    Bundle(u64),
    Game(u64),
    User(u64),
    Group(u64),
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Article {
    /// After redirects: what the wiki calls the page, `w:c:<wiki>:` first on another wiki.
    pub title: String,
    /// The wiki's name when it isn't the Roblox Wiki ("Jailbreak Wiki").
    pub site: String,
    /// The wiki's main page.
    pub main: bool,
    pub revision: u64,
    /// Text before the first heading.
    pub lead: Vec<Block>,
    pub sections: Vec<Section>,
    pub infobox_title: String,
    pub infobox: Vec<Fact>,
    /// Visible categories, without hidden maintenance ones.
    pub categories: Vec<String>,
    /// roblox.com pages the article links to, infobox links first.
    pub roblox: Vec<RobloxRef>,
    /// Usernames the infobox links by name (`users/profile?username=`), lowercase. Player
    /// articles link profiles this way rather than by id.
    pub usernames: Vec<String>,
    /// The infobox picture.
    pub image: Option<Image>,
    /// Other Fandom wikis the article sends readers to, such as an experience's own.
    pub wikis: Vec<OtherWiki>,
    /// Every link to another site in the text, as written there (`[[https://…|label]]`).
    /// Only these open from the article ([`Core::open_wiki_link`]).
    pub links: Vec<String>,
}

impl Article {
    /// The first paragraph, for summaries and search.
    pub fn extract(&self) -> &str {
        self.lead
            .iter()
            .chain(self.sections.iter().flat_map(|s| &s.blocks))
            .find_map(|b| match b {
                Block::P { text } => Some(text.as_str()),
                _ => None,
            })
            .unwrap_or("")
    }

    /// Whether the article is about `r`. An article that links nowhere on Roblox can't say.
    fn names(&self, r: RobloxRef) -> Option<bool> {
        if self.roblox.is_empty() {
            None
        } else {
            Some(self.roblox.contains(&r))
        }
    }
}

/// A page as fetched. `article` is `None` when the wiki has no such page.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Page {
    pub article: Option<Article>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Hit {
    pub title: String,
    /// Plain text around the match.
    pub snippet: String,
    /// When the page was last edited, RFC 3339.
    pub edited: String,
}

/// A Bonsai page that may have an article about it, with what its title is made from.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum About {
    Item {
        id: u64,
        name: String,
    },
    Bundle {
        id: u64,
        name: String,
    },
    /// By root place. `creator` is the group or user that made it; empty when unknown.
    Game {
        place: u64,
        name: String,
        creator: String,
        group: bool,
    },
    User {
        id: u64,
        name: String,
    },
}

/// A name without update tags (`[🎃 UPDATE] Name`) and leading emoji.
fn clean_name(name: &str) -> String {
    let mut n = name.trim();
    while let Some(rest) = n.strip_prefix('[').or_else(|| n.strip_prefix('(')) {
        let Some(end) = rest.find([']', ')']) else { break };
        n = rest[end + 1..].trim_start();
    }
    n.trim_start_matches(|c: char| !c.is_alphanumeric())
        .trim()
        .to_owned()
}

/// The wiki titles to try for a page, best first. Items live under `Catalog:`, players under
/// `Player:`. Experiences are `<group>/<name>` or `Player:<user>/<name>`; their bare name is
/// often a redirect to that, so it comes second.
pub fn titles_for(about: &About) -> Vec<String> {
    match about {
        About::Item { name, .. } | About::Bundle { name, .. } => {
            vec![format!("{CATALOG}{}", clean_name(name))]
        }
        About::User { name, .. } => vec![format!("{PLAYER}{}", name.trim())],
        About::Game {
            name, creator, group, ..
        } => {
            let n = clean_name(name);
            let c = creator.trim();
            match (c.is_empty(), group) {
                (true, _) => vec![n],
                (false, true) => vec![format!("{c}/{n}"), n],
                (false, false) => vec![format!("{PLAYER}{c}/{n}"), n],
            }
        }
    }
}

/// Whether `article` is about `about`. Item names are unique enough in the Catalog namespace,
/// so an item article only fails when it links a different item. An experience article must
/// link the very place, and a player article that profile (by id or by username): names
/// alone collide too often.
pub fn is_about(article: &Article, about: &About) -> bool {
    match about {
        About::Item { id, .. } => article.names(RobloxRef::Item(*id)) != Some(false),
        About::Bundle { id, .. } => article.names(RobloxRef::Bundle(*id)) != Some(false),
        About::Game { place, .. } => article.names(RobloxRef::Game(*place)) == Some(true),
        About::User { id, name } => {
            article.roblox.contains(&RobloxRef::User(*id))
                || article.usernames.contains(&name.trim().to_lowercase())
        }
    }
}

/// The wiki a title is on and the title there: `w:c:jailbreak:Torpedo` → (`jailbreak`,
/// `Torpedo`), `Catalog:Dominus Infernus` → (`roblox`, the same).
pub fn split_title(title: &str) -> (&str, &str) {
    if let Some(rest) = title.strip_prefix(OTHER_WIKI)
        && let Some((wiki, t)) = rest.split_once(':')
        && wikis::valid_wiki(wiki)
    {
        return (wiki, t.trim());
    }
    (ROBLOX, title)
}

/// The other way round: `w:c:jailbreak:Torpedo`, or the bare title on the Roblox Wiki.
pub fn join_title(wiki: &str, title: &str) -> String {
    if wiki == ROBLOX {
        title.to_owned()
    } else {
        format!("{OTHER_WIKI}{wiki}:{title}")
    }
}

/// `item`, `game`, `player` or `page`, from the title alone.
pub fn kind_of(title: &str) -> &'static str {
    if split_title(title).0 != ROBLOX {
        "page"
    } else if title.starts_with(CATALOG) {
        "item"
    } else if let Some(rest) = title.strip_prefix(PLAYER) {
        if rest.contains('/') { "game" } else { "player" }
    } else if title.contains('/') {
        "game"
    } else {
        "page"
    }
}

/// The article's address on the site.
pub fn site_url(title: &str) -> String {
    let (wiki, t) = split_title(title);
    let host = wikis::host(wiki).unwrap_or(fandom::HOST);
    format!("https://{host}/wiki/{}", path_title(t))
}

/// A title as MediaWiki puts it in a path: underscores for spaces, the rest percent-encoded.
fn path_title(title: &str) -> String {
    let mut out = String::new();
    for b in title.trim().replace(' ', "_").bytes() {
        match b {
            b'A'..=b'Z'
            | b'a'..=b'z'
            | b'0'..=b'9'
            | b'_'
            | b'-'
            | b'.'
            | b'~'
            | b':'
            | b'('
            | b')'
            | b'!'
            | b',' => out.push(b as char),
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

impl Core {
    /// Whether the user turned wiki articles on.
    pub fn wiki_enabled(&self) -> bool {
        self.settings().wiki
    }

    /// The article called `title`, or nothing (and no request) while the wiki is off.
    pub fn wiki_page(&self, title: &str) -> Res<Page> {
        let title = title.trim().replace('_', " ");
        if !self.wiki_enabled() || title.is_empty() {
            return Res::default();
        }
        self.inner.res(
            Key::shared("wiki_page", &title),
            ARTICLE_AGE,
            move |f| async move { fetch_page(&f.client, &title).await },
        )
    }

    /// The article about an item, experience or player, if the wiki has one and it is about
    /// this very one (see [`titles_for`] and [`is_about`]).
    pub fn wiki_about(&self, about: &About) -> Res<Page> {
        let mut last = Res::default();
        for title in titles_for(about) {
            let r = self.wiki_page(&title);
            match &r.data {
                Some(p) if p.article.as_ref().is_some_and(|a| is_about(a, about)) => return r,
                // Missing, or about something else: try the next title.
                Some(_) => last = r,
                // Still loading or failed: say so rather than trying more.
                None => return r,
            }
        }
        last.data = last.data.map(|_| std::sync::Arc::new(Page::default()));
        last
    }

    /// Search results for `query` on `wiki` (`roblox` for the Roblox Wiki, where items and
    /// players are searched too), best match first.
    pub fn wiki_search(&self, wiki: &str, query: &str) -> Res<Vec<Hit>> {
        let q = query.split_whitespace().collect::<Vec<_>>().join(" ");
        let wiki = if wiki.is_empty() { ROBLOX } else { wiki }.to_owned();
        if !self.wiki_enabled() || q.is_empty() || wikis::host(&wiki).is_none() {
            return Res::default();
        }
        self.inner.res(
            Key::shared("wiki_search", format!("{wiki}:{}", q.to_lowercase())),
            SEARCH_AGE,
            move |f| async move {
                let namespaces = if wiki == ROBLOX {
                    SEARCH_NAMESPACES
                } else {
                    OTHER_NAMESPACES
                };
                let r = wikis::action_api_on(
                    &f.client,
                    &wiki,
                    &fandom::ActionApiParams {
                        action: "query".into(),
                        format: "json".into(),
                        formatversion: Some("2".into()),
                        list: Some("search".into()),
                        srsearch: Some(q),
                        srnamespace: Some(namespaces.into()),
                        srlimit: Some(SEARCH_LIMIT),
                        srprop: Some("snippet|timestamp".into()),
                        ..Default::default()
                    },
                )
                .await?;
                if let Some(e) = r.error {
                    return Err(api_error(e));
                }
                Ok(hits(&wiki, r.query.and_then(|q| q.search).unwrap_or_default()))
            },
        )
    }

    /// Opens the article on the wiki's site in the browser. Built here from the wiki's host,
    /// so the opener only ever gets a link to a Fandom wiki.
    pub fn open_wiki_site(&self, title: &str) -> Result<(), String> {
        if title.trim().is_empty() {
            return Err("Which article?".into());
        }
        self.open_built_link(&site_url(title))
    }

    /// Opens `url`, a link in the text of the article called `title`, in the browser. Only
    /// links the saved article has open, so the UI can't open just any address.
    pub fn open_wiki_link(&self, title: &str, url: &str) -> Result<(), String> {
        let page = self.wiki_page(title);
        let known = page
            .data
            .as_ref()
            .and_then(|p| p.article.as_ref())
            .is_some_and(|a| a.links.iter().any(|l| l == url));
        if !known {
            return Err("That link isn't in the article.".into());
        }
        self.open_built_link(url)
    }

    /// An article picture's bytes, from disk or Fandom, for the `wikiimg://` protocol. `src` is
    /// an [`Image::src`]. `None` while the wiki is off or when Fandom won't serve it.
    pub fn wiki_image(&self, src: &str) -> oneshot::Receiver<Option<thumbs::Image>> {
        let (tx, rx) = oneshot::channel();
        let url = image_url(src).filter(|_| self.wiki_enabled());
        let inner = self.inner.clone();
        self.inner.rt.spawn(async move {
            let Some(url) = url else {
                let _ = tx.send(None);
                return;
            };
            let bytes = match inner.disk.get(&url) {
                Some(b) => Some(b),
                None => match inner.anon.fetch_wiki_image(&url).await {
                    Ok(b) => {
                        inner.disk.put(&url, &b);
                        Some(b)
                    }
                    Err(e) => {
                        tracing::debug!("wiki image: {e}");
                        None
                    }
                },
            };
            let _ = tx.send(bytes.and_then(sniff_image));
        });
        rx
    }
}

/// The address of an [`Image::src`] on Fandom's image host. `None` for anything that would
/// leave that host.
pub fn image_url(src: &str) -> Option<String> {
    if src.is_empty()
        || src.starts_with('/')
        || src.contains("..")
        || src.contains(['\\', '@', '#'])
        || src.chars().any(|c| c.is_whitespace() || c.is_control())
    {
        return None;
    }
    let url = format!("https://{IMAGE_HOST}/{src}");
    let parsed = url::Url::parse(&url).ok()?;
    (parsed.host_str() == Some(IMAGE_HOST) && parsed.port().is_none()).then_some(url)
}

/// Like [`thumbs::sniff`], with GIFs too: wikis have animated ones.
fn sniff_image(bytes: Vec<u8>) -> Option<thumbs::Image> {
    match image::guess_format(&bytes).ok()? {
        image::ImageFormat::Gif => Some(thumbs::Image {
            mime: "image/gif",
            bytes,
        }),
        _ => thumbs::sniff(bytes),
    }
}

async fn fetch_page(client: &bonsai_api::Client, title: &str) -> Result<Page, Error> {
    let (wiki, local) = split_title(title);
    // Another wiki: its name for the byline, and its main page when no title is given.
    let (site, page, mainpage) = if wiki == ROBLOX {
        (String::new(), local.to_owned(), String::new())
    } else {
        let r = wikis::action_api_on(
            client,
            wiki,
            &fandom::ActionApiParams {
                action: "query".into(),
                format: "json".into(),
                formatversion: Some("2".into()),
                meta: Some("siteinfo".into()),
                siprop: Some("general".into()),
                ..Default::default()
            },
        )
        .await;
        // Fandom answers 404 for a wiki that doesn't exist.
        let r = match r {
            Err(Error::Status { status: 404, .. }) => return Ok(Page::default()),
            r => r?,
        };
        if let Some(e) = r.error {
            return Err(api_error(e));
        }
        let g = r.query.and_then(|q| q.general).unwrap_or_default();
        let main = g.mainpage.unwrap_or_default();
        let page = if local.is_empty() {
            main.clone()
        } else {
            local.to_owned()
        };
        if page.is_empty() {
            return Ok(Page::default());
        }
        (g.sitename.unwrap_or_default(), page, main)
    };
    let r = wikis::action_api_on(
        client,
        wiki,
        &fandom::ActionApiParams {
            action: "parse".into(),
            format: "json".into(),
            formatversion: Some("2".into()),
            page: Some(page.clone()),
            prop: Some("text|categories|revid|displaytitle".into()),
            redirects: Some("1".into()),
            disableeditsection: Some("1".into()),
            disablelimitreport: Some("1".into()),
            ..Default::default()
        },
    )
    .await?;
    if let Some(e) = r.error {
        return match e.code.as_deref() {
            Some("missingtitle" | "invalidtitle") => Ok(Page::default()),
            _ => Err(api_error(e)),
        };
    }
    let Some(p) = r.parse else {
        return Ok(Page::default());
    };
    let categories = p
        .categories
        .unwrap_or_default()
        .into_iter()
        .filter(|c| c.hidden != Some(true))
        .filter_map(|c| c.category)
        .map(|c| c.replace('_', " "))
        .collect();
    let mut a = parse_html_on(wiki, &p.text.unwrap_or_default());
    let found = p.title.unwrap_or(page);
    a.main = !mainpage.is_empty() && found == mainpage;
    a.title = join_title(wiki, &found);
    a.site = site;
    a.revision = p.revid.unwrap_or(0).max(0) as u64;
    a.categories = categories;
    Ok(Page { article: Some(a) })
}

fn api_error(e: fandom::ActionError) -> Error {
    Error::Message(format!(
        "The wiki said: {}",
        e.info.or(e.code).unwrap_or_else(|| "unknown error".into())
    ))
}

/// Keeps articles (and on the Roblox Wiki items and players), in case the wiki returns other
/// pages anyway.
fn hits(wiki: &str, raw: Vec<fandom::SearchHit>) -> Vec<Hit> {
    raw.into_iter()
        .filter_map(|h| {
            let title = h.title?;
            let ok = match h.ns {
                Some(0) => true,
                Some(112 | 114) => wiki == ROBLOX,
                _ => false,
            };
            if !ok {
                return None;
            }
            Some(Hit {
                snippet: squash(
                    &Document::from(h.snippet.unwrap_or_default().as_str())
                        .root()
                        .text(),
                ),
                edited: h.timestamp.unwrap_or_default(),
                title: join_title(wiki, &title),
            })
        })
        .collect()
}

/// Elements never rendered, with everything inside them.
const SKIP_TAGS: &[&str] = &[
    "script", "style", "link", "meta", "noscript", "img", "figure", "audio", "video", "iframe", "svg", "map",
    "form", "input", "button", "hr",
];
/// Classes of things that are site furniture, not article text.
const SKIP_CLASSES: &[&str] = &[
    "navbox",
    "toc",
    "mw-editsection",
    "reference",
    "references",
    "mw-references-wrap",
    "noprint",
    "thumb",
    "gallery",
    "wikia-gallery",
    "mw-empty-elt",
    "printfooter",
    "catlinks",
    "metadata",
    "portable-infobox",
    "infobox",
    "navigation-not-searchable",
    "mw-cite-backlink",
    // Tooltip bodies: the text the site shows on hover, not part of the sentence.
    "wds-dropdown__content",
    // Tab labels; each tab's content still shows, one after another.
    "wds-tabs__wrapper",
    "interactive-maps-container",
    "video-thumbnail",
    "gallery-image-wrapper",
    "mbox__close",
    "mw-collapsible-toggle",
];
/// Elements that start a new block.
const BLOCK_TAGS: &[&str] = &[
    "p",
    "div",
    "section",
    "article",
    "main",
    "center",
    "ul",
    "ol",
    "dl",
    "dt",
    "dd",
    "table",
    "blockquote",
    "pre",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "aside",
    "details",
    "summary",
];
/// Classes that make a box a note: notices, hatnotes ("Main article: ..."), quotes.
const NOTE_CLASSES: &[&str] = &[
    "notice", "ambox", "mbox", "hatnote", "dablink", "rellink", "quote", "cquote",
];

fn tag(n: &NodeRef) -> String {
    n.node_name().map(|t| t.to_string()).unwrap_or_default()
}

fn skipped(n: &NodeRef) -> bool {
    if !n.is_element() {
        return !n.is_text();
    }
    let t = tag(n);
    SKIP_TAGS.contains(&t.as_str())
        || SKIP_CLASSES.iter().any(|c| n.has_class(c))
        || n.id_attr().is_some_and(|i| &*i == "toc")
        || n.attr("style")
            .is_some_and(|s| s.replace(' ', "").contains("display:none"))
}

/// Collapses whitespace runs and trims.
fn squash(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// `[[Title|label]]` → `label`, `[[Title]]` → `Title`. A label may end in `]` ("[VIP]"), so a
/// link ends at the last of a run of `]`.
fn strip_links(s: &str) -> String {
    let mut out = String::new();
    let mut rest = s;
    while let Some(i) = rest.find("[[") {
        out.push_str(&rest[..i]);
        let mut end = None;
        let mut from = i + 2;
        while let Some(k) = rest[from..].find("]]") {
            let k = from + k;
            if !rest[k + 2..].starts_with(']') {
                end = Some(k - i);
                break;
            }
            from = k + 1;
        }
        let Some(j) = end else {
            out.push_str(&rest[i..]);
            return out;
        };
        let inner = &rest[i + 2..i + j];
        out.push_str(inner.split_once('|').map_or(inner, |(_, l)| l));
        rest = &rest[i + j + 2..];
    }
    out.push_str(rest);
    out
}

/// Keeps stray `[[`/`]]` in article text from reading as links.
fn escape(s: &str) -> String {
    s.replace("[[", "[ [").replace("]]", "] ]")
}

/// Namespaces that aren't articles. Links to them are kept as plain text.
const NOT_ARTICLES: &[&str] = &[
    "file",
    "image",
    "category",
    "template",
    "user",
    "user talk",
    "talk",
    "special",
    "help",
    "forum",
    "message wall",
    "board",
    "thread",
    "blog",
    "user blog",
    "module",
    "mediawiki",
];

/// The article an `href` points to, as a title ([`join_title`]): on `wiki` for a link within
/// the page's wiki, or on another Fandom wiki. `None` for other sites, files, categories, user
/// and talk pages.
fn wiki_title(wiki: &str, href: &str) -> Option<String> {
    let (on, path) = match href.strip_prefix("/wiki/") {
        Some(p) => (wiki.to_owned(), p),
        None => {
            let rest = href
                .strip_prefix("https:")
                .or_else(|| href.strip_prefix("http:"))
                .unwrap_or(href)
                .strip_prefix("//")?;
            let (host, path) = rest.split_once('/')?;
            let on = wikis::wiki_of(&host.to_ascii_lowercase())?.to_owned();
            (on, path.strip_prefix("wiki/")?)
        }
    };
    let path = path.split(['#', '?']).next()?;
    let title = percent_decode(path).replace('_', " ");
    let title = title.trim();
    if let Some((ns, _)) = title.split_once(':') {
        let ns = ns.to_ascii_lowercase();
        if NOT_ARTICLES.contains(&ns.as_str()) || (on == ROBLOX && ns == "roblox wiki") {
            return None;
        }
    }
    // Another wiki's bare address is its main page; the Roblox Wiki's isn't worth a link.
    if title.is_empty() && on == ROBLOX {
        return None;
    }
    Some(join_title(&on, title))
}

fn percent_decode(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%'
            && let Some(v) = s.get(i + 1..i + 3).and_then(|h| u8::from_str_radix(h, 16).ok())
        {
            out.push(v);
            i += 3;
            continue;
        }
        out.push(b[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn roblox_ref(href: &str) -> Option<RobloxRef> {
    match resolve(href)? {
        Pasted::Asset(id) => Some(RobloxRef::Item(id)),
        Pasted::Bundle(id) => Some(RobloxRef::Bundle(id)),
        Pasted::Place(id) | Pasted::Server { place_id: id, .. } => Some(RobloxRef::Game(id)),
        Pasted::User(id) => Some(RobloxRef::User(id)),
        Pasted::Group(id) => Some(RobloxRef::Group(id)),
        _ => None,
    }
}

/// How the text links a Roblox page Bonsai can open: `@item:31101391`.
fn roblox_link(r: RobloxRef) -> String {
    match r {
        RobloxRef::Item(id) => format!("@item:{id}"),
        RobloxRef::Bundle(id) => format!("@bundle:{id}"),
        RobloxRef::Game(id) => format!("@game:{id}"),
        RobloxRef::User(id) => format!("@user:{id}"),
        RobloxRef::Group(id) => format!("@group:{id}"),
    }
}

/// A link to another site as the text keeps it: http(s) only, with no sign-in part, and with
/// the characters that would end a `[[link|label]]` escaped.
fn web_url(href: &str) -> Option<String> {
    let u = url::Url::parse(href.trim()).ok()?;
    if !matches!(u.scheme(), "http" | "https")
        || u.host_str().is_none()
        || !u.username().is_empty()
        || u.password().is_some()
    {
        return None;
    }
    Some(
        u.as_str()
            .replace('|', "%7C")
            .replace('[', "%5B")
            .replace(']', "%5D"),
    )
}

/// What a link shows when the article gives only its address: `https://www.example.com/a/` →
/// `example.com/a`, cut short past 48 characters.
fn short_url(u: &str) -> String {
    let s = u.split_once("://").map_or(u, |(_, r)| r);
    let s = s.strip_prefix("www.").unwrap_or(s).trim_end_matches('/');
    if s.chars().count() <= 48 {
        return s.to_owned();
    }
    let cut: String = s.chars().take(47).collect();
    format!("{cut}…")
}

/// What parsing collects besides the text.
#[derive(Default)]
struct Cx {
    /// The wiki the page is on.
    wiki: String,
    refs: Vec<RobloxRef>,
    links: Vec<String>,
    wikis: Vec<OtherWiki>,
    /// Inside a notice box, where an article names the wiki to read more on. Elsewhere, links
    /// to other wikis are just mentions (a main page's list of sister wikis).
    in_note: bool,
}

/// Collects inline text with links, and the Roblox pages and other sites linked on the way.
struct Inline<'c> {
    out: String,
    cx: &'c mut Cx,
}

impl Inline<'_> {
    fn node(&mut self, n: &NodeRef) {
        if n.is_text() {
            self.out.push_str(&escape(&n.text()));
            return;
        }
        if skipped(n) {
            return;
        }
        match tag(n).as_str() {
            "br" => self.out.push(' '),
            "a" => self.link(n),
            t => {
                // Blocks inside inline text (a notice's header and body) stay apart.
                let block = BLOCK_TAGS.contains(&t);
                if block {
                    self.out.push(' ');
                }
                for c in n.children_it(false) {
                    self.node(&c);
                }
                if block {
                    self.out.push(' ');
                }
            }
        }
    }

    fn link(&mut self, n: &NodeRef) {
        let href = n.attr("href").map(|h| h.to_string()).unwrap_or_default();
        let label = squash(&n.text());
        let roblox = roblox_ref(&href);
        if let Some(r) = roblox {
            self.cx.refs.push(r);
        }
        if label.is_empty() {
            return;
        }
        let shown = escape(&label).replace('|', "/");
        // Red links point at pages nobody wrote yet.
        let article = wiki_title(&self.cx.wiki, &href).filter(|_| !n.has_class("new"));
        if let Some(t) = article {
            let (wiki, local) = split_title(&t);
            let names_wiki = self.cx.in_note && local.is_empty() && wiki != self.cx.wiki;
            if names_wiki && wiki != "community" && !self.cx.wikis.iter().any(|w| w.wiki == wiki) {
                self.cx.wikis.push(OtherWiki {
                    wiki: wiki.to_owned(),
                    name: strip_links(&label),
                });
            }
            if t == label {
                self.out.push_str(&format!("[[{t}]]"));
            } else {
                self.out.push_str(&format!("[[{t}|{shown}]]"));
            }
        } else if let Some(r) = roblox {
            self.out.push_str(&format!("[[{}|{shown}]]", roblox_link(r)));
        } else if let Some(u) = web_url(&href) {
            let shown = if label.starts_with("http://") || label.starts_with("https://") {
                escape(&short_url(&u)).replace('|', "/")
            } else {
                shown
            };
            self.out.push_str(&format!("[[{u}|{shown}]]"));
            self.cx.links.push(u);
        } else {
            self.out.push_str(&escape(&label));
        }
    }
}

fn inline(n: &NodeRef, cx: &mut Cx) -> String {
    let mut i = Inline {
        out: String::new(),
        cx,
    };
    i.node(n);
    squash(&i.out)
}

/// Inline text of a list item without its nested lists, which become items of their own.
fn list_items(n: &NodeRef, depth: usize, cx: &mut Cx, items: &mut Vec<String>) {
    for li in n.element_children().into_iter().filter(|c| tag(c) == "li") {
        let mut own = Inline {
            out: String::new(),
            cx,
        };
        let mut nested = Vec::new();
        for c in li.children_it(false) {
            if matches!(tag(&c).as_str(), "ul" | "ol") {
                nested.push(c);
            } else {
                own.node(&c);
            }
        }
        let text = squash(&own.out);
        if !text.is_empty() {
            items.push(format!("{}{text}", "– ".repeat(depth)));
        }
        for l in nested {
            list_items(&l, depth + 1, cx, items);
        }
    }
}

/// A figure with a picture, a gallery, or an old-style `div.thumb`.
fn is_picture(n: &NodeRef) -> bool {
    n.is_element()
        && (tag(n) == "figure"
            || ["thumb", "gallery", "wikia-gallery"]
                .iter()
                .any(|c| n.has_class(c)))
        && !n.has_class("video-thumbnail")
        && !n
            .attr("style")
            .is_some_and(|s| s.replace(' ', "").contains("display:none"))
}

/// The first picture in `n` on Fandom's image host, with the size the article shows it at.
/// Lazy-loaded images keep their address in `data-src`.
fn image_of(n: &NodeRef) -> Option<(String, u32, u32)> {
    for img in n.descendants_it().filter(|d| tag(d) == "img") {
        let Some(src) = img
            .attr("data-src")
            .filter(|s| !s.is_empty())
            .or_else(|| img.attr("src"))
        else {
            continue;
        };
        let Ok(u) = url::Url::parse(&src) else { continue };
        if u.scheme() != "https" || u.host_str() != Some(IMAGE_HOST) || u.port().is_some() {
            continue;
        }
        let size = |a: &str| img.attr(a).and_then(|v| v.parse::<u32>().ok()).unwrap_or(0);
        let (w, h) = (size("width"), size("height"));
        if (w > 0 && w < MIN_IMAGE) || (h > 0 && h < MIN_IMAGE) {
            continue;
        }
        let path = &u[url::Position::BeforePath..];
        return Some((sharper(path.trim_start_matches('/')), w, h));
    }
    None
}

/// Asks for twice the width the article shows, so pictures stay sharp on high-density screens.
/// Fandom never scales up, so a small original comes back as it is.
fn sharper(src: &str) -> String {
    const SCALE: &str = "/scale-to-width-down/";
    let Some(i) = src.find(SCALE) else {
        return src.to_owned();
    };
    let rest = &src[i + SCALE.len()..];
    let end = rest.find(|c: char| !c.is_ascii_digit()).unwrap_or(rest.len());
    match rest[..end].parse::<u32>() {
        Ok(w) => format!("{}{SCALE}{}{}", &src[..i], (w * 2).min(1600), &rest[end..]),
        Err(_) => src.to_owned(),
    }
}

/// The picture in `n` with its caption.
fn picture(n: &NodeRef, cx: &mut Cx) -> Option<Image> {
    let (src, width, height) = image_of(n)?;
    let caption = n
        .descendants_it()
        .find(|d| {
            tag(d) == "figcaption"
                || ["thumbcaption", "lightbox-caption", "gallerytext", "pi-caption"]
                    .iter()
                    .any(|c| d.has_class(c))
        })
        .map(|c| inline(&c, cx))
        .unwrap_or_default();
    Some(Image {
        src,
        caption,
        width,
        height,
    })
}

struct Builder {
    lead: Vec<Block>,
    sections: Vec<Section>,
    cx: Cx,
    pending: String,
}

impl Builder {
    fn push(&mut self, b: Block) {
        match self.sections.last_mut() {
            Some(s) => s.blocks.push(b),
            None => self.lead.push(b),
        }
    }

    fn flush(&mut self) {
        let text = squash(&std::mem::take(&mut self.pending));
        if !text.is_empty() {
            self.push(Block::P { text });
        }
    }

    fn heading(&mut self, n: &NodeRef, level: u8) {
        let id = n
            .id_attr()
            .or_else(|| {
                n.descendants_it()
                    .find(|d| d.has_class("mw-headline"))
                    .and_then(|d| d.id_attr())
            })
            .map(|i| i.to_string())
            .unwrap_or_default();
        let title = inline(n, &mut self.cx);
        if title.is_empty() {
            return;
        }
        let title = strip_links(&title);
        let id = if id.is_empty() {
            title.replace(' ', "_")
        } else {
            id
        };
        self.sections.push(Section {
            id,
            title,
            level,
            blocks: Vec::new(),
        });
    }

    fn table(&mut self, n: &NodeRef) {
        let rows: Vec<NodeRef> = n.descendants_it().filter(|d| tag(d) == "tr").collect();
        let mut columns = Vec::new();
        let mut body = Vec::new();
        for (i, tr) in rows.iter().enumerate() {
            let cells: Vec<NodeRef> = tr
                .element_children()
                .into_iter()
                .filter(|c| matches!(tag(c).as_str(), "td" | "th"))
                .collect();
            let all_head = cells.iter().all(|c| tag(c) == "th");
            let texts: Vec<String> = cells.iter().map(|c| inline(c, &mut self.cx)).collect();
            if texts.iter().all(String::is_empty) {
                continue;
            }
            if i == 0 && all_head {
                columns = texts;
            } else {
                body.push(texts);
            }
        }
        if !body.is_empty() || !columns.is_empty() {
            self.push(Block::Table { columns, rows: body });
        }
    }

    /// A figure or a gallery: one block of pictures.
    fn pictures(&mut self, n: &NodeRef) {
        let items: Vec<NodeRef> = n
            .descendants_it()
            .filter(|d| d.has_class("wikia-gallery-item") || d.has_class("gallerybox"))
            .collect();
        let images: Vec<Image> = if items.is_empty() {
            picture(n, &mut self.cx).into_iter().collect()
        } else {
            items.iter().filter_map(|i| picture(i, &mut self.cx)).collect()
        };
        if !images.is_empty() {
            self.push(Block::Images { images });
        }
    }

    fn block(&mut self, n: &NodeRef) {
        for c in n.children_it(false) {
            if c.is_text() {
                self.pending.push_str(&escape(&c.text()));
                continue;
            }
            if is_picture(&c) {
                self.flush();
                self.pictures(&c);
                continue;
            }
            if skipped(&c) {
                continue;
            }
            let t = tag(&c);
            if !BLOCK_TAGS.contains(&t.as_str()) {
                let mut i = Inline {
                    out: std::mem::take(&mut self.pending),
                    cx: &mut self.cx,
                };
                i.node(&c);
                self.pending = i.out;
                continue;
            }
            self.flush();
            match t.as_str() {
                "h1" => {}
                "h2" | "h3" | "h4" | "h5" | "h6" => self.heading(&c, t[1..].parse().unwrap_or(2)),
                "p" | "dt" | "dd" | "pre" | "summary" => {
                    let text = inline(&c, &mut self.cx);
                    if !text.is_empty() {
                        self.push(Block::P { text });
                    }
                }
                "ul" | "ol" => {
                    let mut items = Vec::new();
                    list_items(&c, 0, &mut self.cx, &mut items);
                    if !items.is_empty() {
                        self.push(Block::List {
                            ordered: t == "ol",
                            items,
                        });
                    }
                }
                "table" if NOTE_CLASSES.iter().any(|k| c.has_class(k)) => self.note(&c),
                "table" => self.table(&c),
                "blockquote" => self.note(&c),
                _ if NOTE_CLASSES.iter().any(|k| c.has_class(k)) => self.note(&c),
                _ => self.block(&c),
            }
        }
    }

    fn note(&mut self, n: &NodeRef) {
        self.cx.in_note = true;
        let text = inline(n, &mut self.cx);
        self.cx.in_note = false;
        if !text.is_empty() {
            self.push(Block::Note { text });
        }
    }
}

/// Fandom's portable infobox (`aside.portable-infobox`), or a classic `table.infobox`: its
/// title, rows and picture.
fn infobox(doc: &Document, cx: &mut Cx) -> (String, Vec<Fact>, Option<Image>) {
    let mut facts = Vec::new();
    let aside = doc.select("aside.portable-infobox").first();
    if let Some(a) = aside.nodes().first() {
        let title = a
            .descendants_it()
            .find(|d| d.has_class("pi-title"))
            .map(|t| strip_links(&inline(&t, cx)))
            .unwrap_or_default();
        let image = a
            .descendants_it()
            .find(|d| d.has_class("pi-image") || d.has_class("pi-image-collection"))
            .and_then(|f| picture(&f, cx));
        for d in a.descendants_it() {
            if d.has_class("pi-data") {
                let label = d.descendants_it().find(|x| x.has_class("pi-data-label"));
                let value = d.descendants_it().find(|x| x.has_class("pi-data-value"));
                if let Some(v) = value {
                    facts.push(Fact {
                        key: d.attr("data-source").map(|s| s.to_string()).unwrap_or_default(),
                        label: label.map(|l| strip_links(&inline(&l, cx))).unwrap_or_default(),
                        value: inline(&v, cx),
                    });
                }
            } else if d.has_class("pi-horizontal-group") && tag(&d) == "table" {
                let labels: Vec<String> = d
                    .descendants_it()
                    .filter(|x| tag(x) == "th" && x.has_class("pi-horizontal-group-item"))
                    .map(|x| strip_links(&inline(&x, cx)))
                    .collect();
                let values: Vec<NodeRef> = d
                    .descendants_it()
                    .filter(|x| tag(x) == "td" && x.has_class("pi-horizontal-group-item"))
                    .collect();
                for (i, v) in values.iter().enumerate() {
                    facts.push(Fact {
                        key: v.attr("data-source").map(|s| s.to_string()).unwrap_or_default(),
                        label: labels.get(i).cloned().unwrap_or_default(),
                        value: inline(v, cx),
                    });
                }
            } else if d.has_class("pi-smart-data-value") {
                let key = d.attr("data-source").map(|s| s.to_string()).unwrap_or_default();
                let label = a
                    .descendants_it()
                    .find(|x| {
                        x.has_class("pi-smart-data-label")
                            && x.attr("data-source").is_some_and(|s| *s == *key)
                    })
                    .map(|l| strip_links(&inline(&l, cx)))
                    .unwrap_or_default();
                facts.push(Fact {
                    key,
                    label,
                    value: inline(&d, cx),
                });
            }
        }
        facts.retain(|f| !f.value.is_empty());
        for f in facts.iter_mut().filter(|f| f.label.is_empty()) {
            f.label = label_of(&f.key);
        }
        return (title, facts, image);
    }
    let table = doc.select("table.infobox").first();
    let mut image = None;
    if let Some(t) = table.nodes().first() {
        image = picture(t, cx);
        for tr in t.descendants_it().filter(|d| tag(d) == "tr") {
            let cells = tr.element_children();
            if let [th, td] = cells.as_slice()
                && tag(th) == "th"
            {
                facts.push(Fact {
                    key: String::new(),
                    label: squash(&th.text()),
                    value: inline(td, cx),
                });
            }
        }
        facts.retain(|f| !f.value.is_empty());
    }
    (String::new(), facts, image)
}

/// A label for an infobox row the template left unlabelled, from its field: `last_updated` →
/// "Last updated".
fn label_of(key: &str) -> String {
    let words = key.replace('_', " ");
    let mut c = words.trim().chars();
    c.next()
        .map(|f| f.to_uppercase().chain(c).collect())
        .unwrap_or_default()
}

/// Profiles the infobox links by username, like the title of a player article.
fn infobox_usernames(doc: &Document) -> Vec<String> {
    let mut out = Vec::new();
    for a in doc.select("aside.portable-infobox a[href]").nodes() {
        let Some(href) = a.attr("href") else { continue };
        let Ok(u) = url::Url::parse(&href) else { continue };
        let roblox = u
            .host_str()
            .is_some_and(|h| h == "roblox.com" || h.ends_with(".roblox.com"));
        if !roblox || !u.path().trim_end_matches('/').ends_with("/users/profile") {
            continue;
        }
        if let Some((_, name)) = u.query_pairs().find(|(k, _)| k.eq_ignore_ascii_case("username")) {
            let name = name.trim().to_lowercase();
            if !name.is_empty() && !out.contains(&name) {
                out.push(name);
            }
        }
    }
    out
}

/// Turns the Roblox Wiki's rendered HTML into blocks. Title, revision and categories come from
/// the rest of the response.
pub fn parse_html(html: &str) -> Article {
    parse_html_on(ROBLOX, html)
}

/// [`parse_html`] for a page on `wiki`, so its links stay on that wiki.
pub fn parse_html_on(wiki: &str, html: &str) -> Article {
    let doc = Document::from(html);
    let mut cx = Cx {
        wiki: wiki.to_owned(),
        ..Default::default()
    };
    // Infobox links say best what the article is about, so they go first; body links follow.
    let (infobox_title, infobox, image) = infobox(&doc, &mut cx);
    let output = doc.select(".mw-parser-output").first();
    let root = output.nodes().first().cloned().unwrap_or_else(|| doc.root());
    let mut b = Builder {
        lead: Vec::new(),
        sections: Vec::new(),
        cx,
        pending: String::new(),
    };
    b.block(&root);
    b.flush();
    // An empty section stays when one of its subsections has text ("History" over "Release
    // history"); empty ones without go.
    let n = b.sections.len();
    let mut keep = vec![false; n];
    for i in (0..n).rev() {
        let level = b.sections[i].level;
        keep[i] = !b.sections[i].blocks.is_empty()
            || (i + 1..n)
                .take_while(|&j| b.sections[j].level > level)
                .any(|j| keep[j]);
    }
    let mut i = 0;
    b.sections.retain(|_| {
        i += 1;
        keep[i - 1]
    });
    let Cx {
        mut refs,
        mut links,
        wikis,
        ..
    } = b.cx;
    let mut seen = BTreeSet::new();
    refs.retain(|r| seen.insert(*r));
    let mut seen = BTreeSet::new();
    links.retain(|l| seen.insert(l.clone()));
    Article {
        lead: b.lead,
        sections: b.sections,
        infobox_title,
        infobox,
        roblox: refs,
        usernames: infobox_usernames(&doc),
        image,
        wikis,
        links,
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PAGE: &str = r##"<div class="mw-parser-output">
<aside class="portable-infobox pi-background">
  <h2 class="pi-item pi-title" data-source="name">Dominus Infernus</h2>
  <figure class="pi-item pi-image"><a href="https://static.wikia.nocookie.net/x.png"><img src="https://static.wikia.nocookie.net/x.png"></a></figure>
  <div class="pi-item pi-data" data-source="creator"><h3 class="pi-data-label">Creator</h3><div class="pi-data-value"><a href="/wiki/Roblox_(user)" title="Roblox (user)">Roblox</a></div></div>
  <div class="pi-item pi-data" data-source="id"><h3 class="pi-data-label">ID</h3><div class="pi-data-value"><a rel="nofollow" class="external text" href="https://www.roblox.com/catalog/31101391/Dominus-Infernus">31101391</a></div></div>
  <table class="pi-horizontal-group"><thead><tr><th class="pi-horizontal-group-item" data-source="price">Price</th><th class="pi-horizontal-group-item" data-source="type">Type</th></tr></thead>
  <tbody><tr><td class="pi-horizontal-group-item" data-source="price">Offsale</td><td class="pi-horizontal-group-item" data-source="type">Hat</td></tr></tbody></table>
</aside>
<p><b>Dominus Infernus</b> is a <a href="/wiki/Catalog:Limited" title="Catalog:Limited">limited</a> hat by <a href="/wiki/Roblox_(user)">Roblox</a>.<sup class="reference"><a href="#cite_note-1">[1]</a></sup>
</p>
<div id="toc" class="toc"><ul><li>Contents</li></ul></div>
<div class="mw-heading mw-heading2"><h2 id="Appearance">Appearance</h2><span class="mw-editsection">[edit]</span></div>
<p>A [[dark]] helmet.<br>It glows.</p>
<h2><span class="mw-headline" id="Trivia">Trivia</span></h2>
<ul><li>It was sold in <a href="/wiki/2010" class="new" title="2010 (page does not exist)">2010</a>.<ul><li>For <a href="/wiki/Robux">Robux</a>.</li></ul></li><li><a href="https://www.roblox.com/users/156/profile">builderman</a> owns one.</li></ul>
<table class="wikitable"><tr><th>Date</th><th>Price</th></tr><tr><td>2010</td><td>25,000</td></tr></table>
<div class="hatnote">Main article: <a href="/wiki/Dominus">Dominus</a></div>
<p>See the <a href="/wiki/File:X.png">file</a> and <a href="/wiki/Category:Hats">Hats</a>.</p>
<h3 id="Empty"><a href="/wiki/Empty">Empty</a></h3>
<table class="navbox"><tr><td><a href="/wiki/Other">Other</a></td></tr></table>
<script>alert(1)</script><style>p{}</style>
</div>"##;

    #[test]
    fn parses_infobox_sections_and_links() {
        let a = parse_html(PAGE);
        assert_eq!(a.infobox_title, "Dominus Infernus");
        let facts: Vec<_> = a
            .infobox
            .iter()
            .map(|f| (f.key.as_str(), f.label.as_str(), f.value.as_str()))
            .collect();
        assert_eq!(
            facts,
            [
                ("creator", "Creator", "[[Roblox (user)|Roblox]]"),
                ("id", "ID", "[[@item:31101391|31101391]]"),
                ("price", "Price", "Offsale"),
                ("type", "Type", "Hat"),
            ]
        );
        assert_eq!(
            a.lead,
            [Block::P {
                text: "Dominus Infernus is a [[Catalog:Limited|limited]] hat by [[Roblox (user)|Roblox]]."
                    .into()
            }]
        );
        assert_eq!(
            a.extract(),
            "Dominus Infernus is a [[Catalog:Limited|limited]] hat by [[Roblox (user)|Roblox]]."
        );
        let titles: Vec<_> = a
            .sections
            .iter()
            .map(|s| (s.id.as_str(), s.title.as_str(), s.level))
            .collect();
        assert_eq!(
            titles,
            [("Appearance", "Appearance", 2), ("Trivia", "Trivia", 2)],
            "empty section dropped"
        );
        assert_eq!(
            a.sections[0].blocks,
            [Block::P {
                text: "A [ [dark] ] helmet. It glows.".into()
            }]
        );
        let trivia = &a.sections[1].blocks;
        assert_eq!(
            trivia[0],
            Block::List {
                ordered: false,
                items: vec![
                    "It was sold in 2010.".into(),
                    "– For [[Robux]].".into(),
                    "[[@user:156|builderman]] owns one.".into()
                ]
            },
            "red links are plain text; Roblox links open in Bonsai"
        );
        assert_eq!(
            trivia[1],
            Block::Table {
                columns: vec!["Date".into(), "Price".into()],
                rows: vec![vec!["2010".into(), "25,000".into()]]
            }
        );
        assert_eq!(
            trivia[2],
            Block::Note {
                text: "Main article: [[Dominus]]".into()
            }
        );
        assert_eq!(
            trivia[3],
            Block::P {
                text: "See the file and Hats.".into()
            }
        );
        assert_eq!(trivia.len(), 4, "navbox, script and style dropped");
        assert_eq!(a.roblox, [RobloxRef::Item(31_101_391), RobloxRef::User(156)]);
    }

    #[test]
    fn titles_and_matching() {
        let item = |id| About::Item {
            id,
            name: " Dominus Infernus ".into(),
        };
        let game = |name: &str, creator: &str, group| About::Game {
            place: 606_849_621,
            name: name.into(),
            creator: creator.into(),
            group,
        };
        assert_eq!(titles_for(&item(1)), ["Catalog:Dominus Infernus"]);
        assert_eq!(
            titles_for(&game("[🎃 UPDATE] (x2) Jailbreak", "Badimo", true)),
            ["Badimo/Jailbreak", "Jailbreak"]
        );
        assert_eq!(
            titles_for(&game("🔥 Natural Disaster Survival", "Stickmasterluke", false)),
            [
                "Player:Stickmasterluke/Natural Disaster Survival",
                "Natural Disaster Survival"
            ]
        );
        assert_eq!(titles_for(&game("Solo", "", true)), ["Solo"]);
        assert_eq!(
            titles_for(&About::User {
                id: 156,
                name: "builderman".into()
            }),
            ["Player:builderman"]
        );
        let a = parse_html(PAGE);
        assert!(is_about(&a, &item(31_101_391)));
        assert!(!is_about(&a, &item(5)), "links a different item");
        assert!(is_about(
            &a,
            &About::User {
                id: 156,
                name: "x".into()
            }
        ));
        assert!(
            !is_about(&a, &game("Jailbreak", "Badimo", true)),
            "games need their place linked"
        );
        let bare = Article::default();
        assert!(is_about(&bare, &item(5)));
        assert!(!is_about(
            &bare,
            &About::User {
                id: 5,
                name: "x".into()
            }
        ));
        assert_eq!(
            (
                kind_of("Catalog:Dominus Infernus"),
                kind_of("Player:Builderman"),
                kind_of("Player:Builderman/Some Game"),
                kind_of("Badimo/Jailbreak"),
                kind_of("Robux")
            ),
            ("item", "player", "game", "game", "page")
        );
    }

    /// Cut from the live pages (2026-10-19): Badimo/Jailbreak's and Player:Builderman's
    /// infobox heads, and a tooltip and a tab row from Catalog:Dominus Infernus and others.
    #[test]
    fn real_markup_games_players_tooltips_tabs() {
        let jb = parse_html(
            r##"<div class="mw-parser-output"><aside role="region" class="portable-infobox pi-background">
<h2 class="pi-item pi-title" data-source="name"><a class="external text" href="https://www.roblox.com/games/606849621">Jailbreak</a></h2>
<div class="pi-item pi-data" data-source="creator"><h3 class="pi-data-label">Creator</h3><div class="pi-data-value"><a href="/wiki/Badimo" title="Badimo">Badimo</a> <span class="wds-dropdown custom-tooltip"><span class="wds-dropdown&#95;&#95;toggle"><span typeof="mw:File/Frameless"><span><img src="x.svg"></span></span></span><span class="wds-dropdown&#95;&#95;content"><span><b>Badimo</b> is a notable community verified by Roblox Corporation. <a href="/wiki/Verified_Badge">Learn more</a>.</span></span></span></div></div>
<div class="pi-item pi-data" data-source="former genre"><h3 class="pi-data-label"><span class="wds-dropdown custom-tooltip"><span class="wds-dropdown&#95;&#95;toggle">Former Genre</span><span class="wds-dropdown&#95;&#95;content">This experience's genre prior to the update.</span></span></h3><div class="pi-data-value">Town and City</div></div>
<div class="pi-item pi-data" data-source="last_updated"><div class="pi-data-value pi-font"><small>Page last updated on <b>July 9, 2026</b></small></div></div>
</aside>
<b>Jailbreak</b> is an experience by <a href="/wiki/Badimo">Badimo</a>.
<div class="wds-tabber"><div class="wds-tabs__wrapper"><ul class="wds-tabs"><li class="wds-tabs__tab"><span class="wds-tabs__tab-label">Current</span></li></ul></div><div class="wds-tab__content wds-is-current"><p>Tab text.</p></div></div>
<h2><span class="mw-headline" id="History">History</span></h2>
<h3><span class="mw-headline" id="Release_history">Release history</span></h3><p>Released in 2017.</p>
<div class="mbox"><div class="mbox&#95;&#95;content"><div class="mbox&#95;&#95;content&#95;&#95;header">Jailbreak Wiki</div><div class="mbox&#95;&#95;content&#95;&#95;text">This experience has a dedicated wiki.</div></div></div>
<h2><span class="mw-headline" id="Gallery">Gallery</span></h2>
<h3><span class="mw-headline" id="Old">Old</span></h3><div class="wikia-gallery"><a href="/wiki/File:X.png">X</a></div>
</div>"##,
        );
        let facts: Vec<_> = jb
            .infobox
            .iter()
            .map(|f| (f.label.as_str(), f.value.as_str()))
            .collect();
        assert_eq!(
            facts,
            [
                ("Creator", "[[Badimo]]"),
                ("Former Genre", "Town and City"),
                ("Last updated", "Page last updated on July 9, 2026")
            ],
            "tooltip bodies dropped"
        );
        assert_eq!(jb.roblox, [RobloxRef::Game(606_849_621)]);
        assert!(is_about(
            &jb,
            &About::Game {
                place: 606_849_621,
                name: "Jailbreak".into(),
                creator: "Badimo".into(),
                group: true
            }
        ));
        assert_eq!(
            jb.lead,
            [
                Block::P {
                    text: "Jailbreak is an experience by [[Badimo]].".into()
                },
                Block::P {
                    text: "Tab text.".into()
                }
            ],
            "tab labels dropped, tab content kept"
        );
        let titles: Vec<_> = jb.sections.iter().map(|s| (s.title.as_str(), s.level)).collect();
        assert_eq!(
            titles,
            [("History", 2), ("Release history", 3)],
            "parent of a subsection with text kept, empty gallery dropped"
        );
        assert_eq!(
            jb.sections[1].blocks[1],
            Block::Note {
                text: "Jailbreak Wiki This experience has a dedicated wiki.".into()
            }
        );

        let bm = parse_html(
            r##"<aside class="portable-infobox"><h2 class="pi-item pi-title" data-source="name"><a class="external text" href="https://www.roblox.com/users/profile?username=builderman">builderman</a></h2></aside><p>A player.</p>"##,
        );
        assert_eq!(bm.usernames, ["builderman"]);
        assert!(bm.roblox.is_empty());
        assert!(is_about(
            &bm,
            &About::User {
                id: 156,
                name: "Builderman".into()
            }
        ));
        assert!(!is_about(
            &bm,
            &About::User {
                id: 1,
                name: "Roblox".into()
            }
        ));
    }

    #[test]
    fn links_and_urls() {
        let t = |href| wiki_title(ROBLOX, href);
        assert_eq!(
            t("/wiki/Catalog:Dominus_Infernus#Trivia").as_deref(),
            Some("Catalog:Dominus Infernus")
        );
        assert_eq!(
            t("https://roblox.fandom.com/wiki/Caf%C3%A9").as_deref(),
            Some("Café")
        );
        assert_eq!(t("/wiki/User:Someone"), None);
        assert_eq!(t("https://example.com/wiki/X"), None);
        assert_eq!(t("https://roblox.fandom.com/wiki/"), None);
        // Another wiki: its articles, and its bare address for the main page.
        assert_eq!(
            t("https://jailbreak.fandom.com/wiki/Torero").as_deref(),
            Some("w:c:jailbreak:Torero")
        );
        assert_eq!(
            t("https://jailbreak.fandom.com/wiki/").as_deref(),
            Some("w:c:jailbreak:")
        );
        assert_eq!(t("https://jailbreak.fandom.com/es/wiki/Torero"), None);
        assert_eq!(t("https://evil.com@jailbreak.fandom.com/wiki/X"), None);
        // On that wiki, its own links stay there and the Roblox Wiki's go back.
        assert_eq!(
            wiki_title("jailbreak", "/wiki/Jailbreak_Wiki:Home").as_deref(),
            Some("w:c:jailbreak:Jailbreak Wiki:Home")
        );
        assert_eq!(
            wiki_title("jailbreak", "https://roblox.fandom.com/wiki/Badimo").as_deref(),
            Some("Badimo")
        );
        assert_eq!(
            site_url("Catalog:Dominus Infernus"),
            "https://roblox.fandom.com/wiki/Catalog:Dominus_Infernus"
        );
        assert_eq!(
            site_url("Café & more?"),
            "https://roblox.fandom.com/wiki/Caf%C3%A9_%26_more%3F"
        );
        assert_eq!(
            site_url("w:c:jailbreak:Torero"),
            "https://jailbreak.fandom.com/wiki/Torero"
        );
        assert_eq!(site_url("w:c:jailbreak:"), "https://jailbreak.fandom.com/wiki/");
        assert_eq!(split_title("w:c:jailbreak:Torero"), ("jailbreak", "Torero"));
        assert_eq!(split_title("w:c:Bad Wiki:X"), (ROBLOX, "w:c:Bad Wiki:X"));
        assert_eq!(join_title("jailbreak", "Torero"), "w:c:jailbreak:Torero");
        assert_eq!(join_title(ROBLOX, "Badimo"), "Badimo");
        assert_eq!(kind_of("w:c:jailbreak:Torero"), "page");
        assert_eq!(
            web_url("https://example.com/a|b[c]").as_deref(),
            Some("https://example.com/a%7Cb%5Bc%5D")
        );
        for bad in [
            "javascript:alert(1)",
            "/wiki/X",
            "#cite",
            "ftp://x.com/",
            "https://u:p@x.com/",
        ] {
            assert_eq!(web_url(bad), None, "{bad}");
        }
        assert_eq!(short_url("https://www.example.com/a/"), "example.com/a");
        assert_eq!(
            short_url("https://web.archive.org/web/20190804181502/https://blog.roblox.com/x"),
            "web.archive.org/web/20190804181502/https://blog…"
        );
    }

    #[test]
    fn external_roblox_and_other_wiki_links() {
        let a = parse_html(
            r#"<div class="mw-parser-output">
<div class="mbox"><div class="mbox__content__header"><a href="https://jailbreak.fandom.com/wiki/" class="extiw" title="w:c:jailbreak">Jailbreak Wiki</a></div><div class="mbox__content__text">This experience has <a href="https://jailbreak.fandom.com/wiki/" class="extiw" title="w:c:jailbreak">a dedicated wiki project</a>.</div></div>
<p>Play <a class="external text" href="https://www.roblox.com/games/606849621">Jailbreak</a>, read <a class="external free" href="https://blog.roblox.com/2017/08/jailbreak-roblox-success-story/">https://blog.roblox.com/2017/08/jailbreak-roblox-success-story/</a> or buy <a class="external text" href="https://www.roblox.com/game-pass/2296901/VIP">Very Important Player [VIP]</a>.</p>
<p>Other wikis: <a href="https://adoptme.fandom.com/wiki/" class="extiw">Adopt Me!</a></p>
</div>"#,
        );
        assert_eq!(
            a.lead[0],
            Block::Note {
                text: "[[w:c:jailbreak:|Jailbreak Wiki]] This experience has [[w:c:jailbreak:|a dedicated wiki project]].".into()
            }
        );
        assert_eq!(
            a.lead[1],
            Block::P {
                text: "Play [[@game:606849621|Jailbreak]], read [[https://blog.roblox.com/2017/08/jailbreak-roblox-success-story/|blog.roblox.com/2017/08/jailbreak-roblox-succes…]] or buy [[https://www.roblox.com/game-pass/2296901/VIP|Very Important Player [VIP]]]."
                    .into()
            }
        );
        // Only the notice names the experience's own wiki; a list of others is a mention.
        assert_eq!(
            a.wikis,
            [OtherWiki {
                wiki: "jailbreak".into(),
                name: "Jailbreak Wiki".into()
            }]
        );
        assert_eq!(
            a.links,
            [
                "https://blog.roblox.com/2017/08/jailbreak-roblox-success-story/",
                "https://www.roblox.com/game-pass/2296901/VIP"
            ]
        );
    }

    #[test]
    fn pictures_and_galleries() {
        let a = parse_html(
            r#"<div class="mw-parser-output">
<aside class="portable-infobox"><h2 class="pi-item pi-title">Builderman</h2><div class="pi-image-collection wds-tabber"><div class="wds-tabs__wrapper"><ul class="wds-tabs"><li><span class="wds-tabs__tab-label">Current</span></li></ul></div><div class="wds-tab__content wds-is-current"><figure class="pi-item pi-image"><a href="https://static.wikia.nocookie.net/roblox/images/a/a9/B.png/revision/latest?cb=1" class="image image-thumbnail"><img src="https://static.wikia.nocookie.net/roblox/images/a/a9/B.png/revision/latest/scale-to-width-down/268?cb=1" width="268" height="268"></a><figcaption class="pi-item-spacer pi-caption">Current avatar</figcaption></figure></div></div></aside>
<p>Text.</p>
<figure class="thumb show-info-icon" typeof="mw:File/Thumb"><a href="https://static.wikia.nocookie.net/roblox/images/e/e9/Book.jpeg/revision/latest?cb=2" class="mw-file-description image"><img alt="A figure." src="data:image/gif;base64,R0lGODlhAQABAIABAAAAAP///yH5BAEAAAEALAAAAAABAAEAQAICTAEAOw%3D%3D" width="180" height="234" class="thumbimage lazyload" data-src="https://static.wikia.nocookie.net/roblox/images/e/e9/Book.jpeg/revision/latest/scale-to-width-down/180?cb=2" /></a><figcaption>A figure in the <a href="/wiki/Roblox_toys">toys</a> line.</figcaption></figure>
<div id="gallery-1" class="wikia-gallery"><div class="wikia-gallery-item"><div class="thumb"><div class="gallery-image-wrapper"><a class="image lightbox" href="/wiki/File:A.png"><img src="https://static.wikia.nocookie.net/roblox/images/1/1a/A%2C_2026.png/revision/latest/scale-to-width-down/120?cb=3" width="120" height="120"></a></div></div><div class="lightbox-caption">2026</div></div><div class="wikia-gallery-item"><div class="thumb"><img src="https://evil.example/x.png" width="120" height="120"></div><div class="lightbox-caption">Elsewhere</div></div><div class="wikia-gallery-item"><img src="https://static.wikia.nocookie.net/roblox/images/b/b2/C.png/revision/latest?cb=4" width="120" height="90"><div class="lightbox-caption">Original</div></div></div>
<p>An icon <span typeof="mw:File"><img src="https://static.wikia.nocookie.net/roblox/images/r/r1/Robux.png/revision/latest/scale-to-width-down/16?cb=5" width="16" height="16"></span> in text.</p>
</div>"#,
        );
        assert_eq!(
            a.image,
            Some(Image {
                src: "roblox/images/a/a9/B.png/revision/latest/scale-to-width-down/536?cb=1".into(),
                caption: "Current avatar".into(),
                width: 268,
                height: 268,
            })
        );
        assert_eq!(a.lead.len(), 4, "{:?}", a.lead);
        assert_eq!(
            a.lead[1],
            Block::Images {
                images: vec![Image {
                    src: "roblox/images/e/e9/Book.jpeg/revision/latest/scale-to-width-down/360?cb=2".into(),
                    caption: "A figure in the [[Roblox toys|toys]] line.".into(),
                    width: 180,
                    height: 234,
                }]
            }
        );
        let Block::Images { images } = &a.lead[2] else {
            panic!("{:?}", a.lead[2])
        };
        // Pictures from anywhere but Fandom's image host are left out.
        let srcs: Vec<_> = images
            .iter()
            .map(|i| (i.src.as_str(), i.caption.as_str()))
            .collect();
        assert_eq!(
            srcs,
            [
                (
                    "roblox/images/1/1a/A%2C_2026.png/revision/latest/scale-to-width-down/240?cb=3",
                    "2026"
                ),
                ("roblox/images/b/b2/C.png/revision/latest?cb=4", "Original")
            ]
        );
        // Icons inside text stay out.
        assert_eq!(
            a.lead[3],
            Block::P {
                text: "An icon in text.".into()
            }
        );
    }

    #[test]
    fn strips_links_with_brackets_in_labels() {
        assert_eq!(strip_links("[[A|Player [VIP]]] and [[B]]"), "Player [VIP] and B");
        assert_eq!(strip_links("[[A|x]] [[broken"), "x [[broken");
    }

    #[test]
    fn image_paths_stay_on_the_image_host() {
        assert_eq!(
            image_url("roblox/images/a/a9/B%2C_2026.png/revision/latest?cb=1").as_deref(),
            Some("https://static.wikia.nocookie.net/roblox/images/a/a9/B%2C_2026.png/revision/latest?cb=1")
        );
        for bad in [
            "",
            "/evil.com/x.png",
            "@evil.com/x.png",
            "../x.png",
            "roblox/../../x",
            "a b.png",
            "x\\y.png",
            "x.png#frag",
        ] {
            assert_eq!(image_url(bad), None, "{bad}");
        }
        assert_eq!(
            sharper("a/scale-to-width-down/120?cb=1"),
            "a/scale-to-width-down/240?cb=1"
        );
        assert_eq!(
            sharper("a/scale-to-width-down/1200"),
            "a/scale-to-width-down/1600"
        );
        assert_eq!(sharper("a/revision/latest?cb=1"), "a/revision/latest?cb=1");
    }

    #[test]
    fn search_keeps_articles_and_items() {
        let raw = vec![
            fandom::SearchHit {
                ns: Some(0),
                title: Some("Dominus".into()),
                snippet: Some(r#"The <span class="searchmatch">Dominus</span> series"#.into()),
                timestamp: Some("2026-01-02T03:04:05Z".into()),
                ..Default::default()
            },
            fandom::SearchHit {
                ns: Some(112),
                title: Some("Catalog:Dominus Infernus".into()),
                ..Default::default()
            },
            fandom::SearchHit {
                ns: Some(2),
                title: Some("User:Dominus fan".into()),
                ..Default::default()
            },
        ];
        let h = hits(ROBLOX, raw.clone());
        assert_eq!(h.len(), 2);
        assert_eq!(h[0].snippet, "The Dominus series");
        assert_eq!(h[1].title, "Catalog:Dominus Infernus");
        // Another wiki has no Catalog; its titles say which wiki they are on.
        let h = hits("jailbreak", raw);
        assert_eq!(h.len(), 1);
        assert_eq!(h[0].title, "w:c:jailbreak:Dominus");
    }
}

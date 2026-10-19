//! Articles about items, experiences and players from the Roblox Wiki (roblox.fandom.com), a
//! community wiki on Fandom, a third-party site. Opt-in (`Settings::wiki`): nothing is fetched
//! until the user turns it on, and every article shows its source and license (CC BY-SA).
//!
//! The page's HTML is parsed here and only plain text with `[[Title|label]]` links leaves this
//! module, so no markup, scripts, styles or remote images from Fandom ever reach the webview.
//! The request goes out with the anonymous client, and the transport never sends Roblox
//! credentials to a non-Roblox host anyway.

use std::collections::BTreeSet;

use bonsai_api::Error;
use bonsai_api::generated::roblox_fandom_com::undocumented as fandom;
use dom_query::{Document, NodeRef};
use serde::{Deserialize, Serialize};

use crate::Core;
use crate::res::{Key, Res};
use crate::resolve::{Pasted, resolve};

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
    /// After redirects: what the wiki calls the page.
    pub title: String,
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

/// `item`, `game`, `player` or `page`, from the title alone.
pub fn kind_of(title: &str) -> &'static str {
    if title.starts_with(CATALOG) {
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
    format!("https://{}/wiki/{}", fandom::HOST, path_title(title))
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

    /// Search results for `query` among articles and items, best match first.
    pub fn wiki_search(&self, query: &str) -> Res<Vec<Hit>> {
        let q = query.split_whitespace().collect::<Vec<_>>().join(" ");
        if !self.wiki_enabled() || q.is_empty() {
            return Res::default();
        }
        self.inner.res(
            Key::shared("wiki_search", q.to_lowercase()),
            SEARCH_AGE,
            move |f| async move {
                let r = fandom::action_api(
                    &f.client,
                    &fandom::ActionApiParams {
                        action: "query".into(),
                        format: "json".into(),
                        formatversion: Some("2".into()),
                        list: Some("search".into()),
                        srsearch: Some(q),
                        srnamespace: Some(SEARCH_NAMESPACES.into()),
                        srlimit: Some(SEARCH_LIMIT),
                        srprop: Some("snippet|timestamp".into()),
                        ..Default::default()
                    },
                )
                .await?;
                if let Some(e) = r.error {
                    return Err(api_error(e));
                }
                Ok(hits(r.query.and_then(|q| q.search).unwrap_or_default()))
            },
        )
    }

    /// Opens the article on the wiki's site in the browser. Built here from the wiki's host,
    /// so the opener only ever gets a link to that one site.
    pub fn open_wiki_site(&self, title: &str) -> Result<(), String> {
        if title.trim().is_empty() {
            return Err("Which article?".into());
        }
        self.open_built_link(&site_url(title))
    }
}

async fn fetch_page(client: &bonsai_api::Client, title: &str) -> Result<Page, Error> {
    let r = fandom::action_api(
        client,
        &fandom::ActionApiParams {
            action: "parse".into(),
            format: "json".into(),
            formatversion: Some("2".into()),
            page: Some(title.to_owned()),
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
    let mut a = parse_html(&p.text.unwrap_or_default());
    a.title = p.title.unwrap_or_else(|| title.to_owned());
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

/// Keeps articles, items and players, in case the wiki returns other pages anyway.
fn hits(raw: Vec<fandom::SearchHit>) -> Vec<Hit> {
    raw.into_iter()
        .filter_map(|h| {
            let title = h.title?;
            if !matches!(h.ns, Some(0 | 112 | 114)) {
                return None;
            }
            Some(Hit {
                snippet: squash(
                    &Document::from(h.snippet.unwrap_or_default().as_str())
                        .root()
                        .text(),
                ),
                edited: h.timestamp.unwrap_or_default(),
                title,
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

/// `[[Title|label]]` → `label`, `[[Title]]` → `Title`.
fn strip_links(s: &str) -> String {
    let mut out = String::new();
    let mut rest = s;
    while let Some(i) = rest.find("[[") {
        out.push_str(&rest[..i]);
        let Some(j) = rest[i..].find("]]") else { break };
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

/// The article an `href` points to on this wiki, as a title. `None` for other sites, red
/// links, files, categories, user and talk pages.
fn wiki_title(href: &str) -> Option<String> {
    let rest = href
        .strip_prefix(&format!("https://{}", fandom::HOST))
        .or_else(|| href.strip_prefix(&format!("//{}", fandom::HOST)))
        .unwrap_or(href);
    let path = rest.strip_prefix("/wiki/")?;
    let path = path.split(['#', '?']).next()?;
    let title = percent_decode(path).replace('_', " ");
    let title = title.trim();
    let ns = title.split_once(':').map(|(n, _)| n.to_ascii_lowercase());
    if let Some(ns) = ns
        && ns != "catalog"
        && [
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
            "roblox wiki",
        ]
        .contains(&ns.as_str())
    {
        return None;
    }
    (!title.is_empty()).then(|| title.to_owned())
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

/// Collects inline text with links, and the Roblox pages linked on the way.
struct Inline<'r> {
    out: String,
    refs: &'r mut Vec<RobloxRef>,
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
            "a" => {
                let href = n.attr("href").map(|h| h.to_string()).unwrap_or_default();
                let label = squash(&n.text());
                if let Some(r) = roblox_ref(&href) {
                    self.refs.push(r);
                }
                match wiki_title(&href).filter(|_| !n.has_class("new") && !label.is_empty()) {
                    Some(t) if t == label => self.out.push_str(&format!("[[{t}]]")),
                    Some(t) => self
                        .out
                        .push_str(&format!("[[{t}|{}]]", escape(&label).replace('|', "/"))),
                    None => self.out.push_str(&escape(&label)),
                }
            }
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
}

fn inline(n: &NodeRef, refs: &mut Vec<RobloxRef>) -> String {
    let mut i = Inline {
        out: String::new(),
        refs,
    };
    i.node(n);
    squash(&i.out)
}

/// Inline text of a list item without its nested lists, which become items of their own.
fn list_items(n: &NodeRef, depth: usize, refs: &mut Vec<RobloxRef>, items: &mut Vec<String>) {
    for li in n.element_children().into_iter().filter(|c| tag(c) == "li") {
        let mut own = Inline {
            out: String::new(),
            refs,
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
            list_items(&l, depth + 1, refs, items);
        }
    }
}

struct Builder {
    lead: Vec<Block>,
    sections: Vec<Section>,
    refs: Vec<RobloxRef>,
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
        let title = inline(n, &mut self.refs);
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
            let texts: Vec<String> = cells.iter().map(|c| inline(c, &mut self.refs)).collect();
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

    fn block(&mut self, n: &NodeRef) {
        for c in n.children_it(false) {
            if c.is_text() {
                self.pending.push_str(&escape(&c.text()));
                continue;
            }
            if skipped(&c) {
                continue;
            }
            let t = tag(&c);
            if !BLOCK_TAGS.contains(&t.as_str()) {
                let mut i = Inline {
                    out: std::mem::take(&mut self.pending),
                    refs: &mut self.refs,
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
                    let text = inline(&c, &mut self.refs);
                    if !text.is_empty() {
                        self.push(Block::P { text });
                    }
                }
                "ul" | "ol" => {
                    let mut items = Vec::new();
                    list_items(&c, 0, &mut self.refs, &mut items);
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
        let text = inline(n, &mut self.refs);
        if !text.is_empty() {
            self.push(Block::Note { text });
        }
    }
}

/// Fandom's portable infobox (`aside.portable-infobox`), or a classic `table.infobox`.
fn infobox(doc: &Document, refs: &mut Vec<RobloxRef>) -> (String, Vec<Fact>) {
    let mut facts = Vec::new();
    let aside = doc.select("aside.portable-infobox").first();
    if let Some(a) = aside.nodes().first() {
        let title = a
            .descendants_it()
            .find(|d| d.has_class("pi-title"))
            .map(|t| inline(&t, refs))
            .unwrap_or_default();
        for d in a.descendants_it() {
            if d.has_class("pi-data") {
                let label = d.descendants_it().find(|x| x.has_class("pi-data-label"));
                let value = d.descendants_it().find(|x| x.has_class("pi-data-value"));
                if let Some(v) = value {
                    facts.push(Fact {
                        key: d.attr("data-source").map(|s| s.to_string()).unwrap_or_default(),
                        label: label.map(|l| strip_links(&inline(&l, refs))).unwrap_or_default(),
                        value: inline(&v, refs),
                    });
                }
            } else if d.has_class("pi-horizontal-group") && tag(&d) == "table" {
                let labels: Vec<String> = d
                    .descendants_it()
                    .filter(|x| tag(x) == "th" && x.has_class("pi-horizontal-group-item"))
                    .map(|x| strip_links(&inline(&x, refs)))
                    .collect();
                let values: Vec<NodeRef> = d
                    .descendants_it()
                    .filter(|x| tag(x) == "td" && x.has_class("pi-horizontal-group-item"))
                    .collect();
                for (i, v) in values.iter().enumerate() {
                    facts.push(Fact {
                        key: v.attr("data-source").map(|s| s.to_string()).unwrap_or_default(),
                        label: labels.get(i).cloned().unwrap_or_default(),
                        value: inline(v, refs),
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
                    .map(|l| strip_links(&inline(&l, refs)))
                    .unwrap_or_default();
                facts.push(Fact {
                    key,
                    label,
                    value: inline(&d, refs),
                });
            }
        }
        facts.retain(|f| !f.value.is_empty());
        for f in facts.iter_mut().filter(|f| f.label.is_empty()) {
            f.label = label_of(&f.key);
        }
        return (title, facts);
    }
    let table = doc.select("table.infobox").first();
    if let Some(t) = table.nodes().first() {
        for tr in t.descendants_it().filter(|d| tag(d) == "tr") {
            let cells = tr.element_children();
            if let [th, td] = cells.as_slice()
                && tag(th) == "th"
            {
                facts.push(Fact {
                    key: String::new(),
                    label: squash(&th.text()),
                    value: inline(td, refs),
                });
            }
        }
        facts.retain(|f| !f.value.is_empty());
    }
    (String::new(), facts)
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

/// Turns MediaWiki's rendered HTML into blocks. Title, revision and categories come from the
/// rest of the response.
pub fn parse_html(html: &str) -> Article {
    let doc = Document::from(html);
    let mut refs = Vec::new();
    let (infobox_title, infobox) = infobox(&doc, &mut refs);
    let output = doc.select(".mw-parser-output").first();
    let root = output.nodes().first().cloned().unwrap_or_else(|| doc.root());
    let mut b = Builder {
        lead: Vec::new(),
        sections: Vec::new(),
        refs: Vec::new(),
        pending: String::new(),
    };
    b.block(&root);
    b.flush();
    // An empty section stays when one of its subsections has text ("History" over "Release
    // history"); empty ones without, like a gallery's, go.
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
    // Infobox links say best what the article is about; body links follow.
    refs.append(&mut b.refs);
    let mut seen = BTreeSet::new();
    refs.retain(|r| seen.insert(*r));
    Article {
        lead: b.lead,
        sections: b.sections,
        infobox_title,
        infobox,
        roblox: refs,
        usernames: infobox_usernames(&doc),
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
                ("id", "ID", "31101391"),
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
                    "builderman owns one.".into()
                ]
            },
            "red links and outside links are plain text"
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
        assert_eq!(
            wiki_title("/wiki/Catalog:Dominus_Infernus#Trivia").as_deref(),
            Some("Catalog:Dominus Infernus")
        );
        assert_eq!(
            wiki_title("https://roblox.fandom.com/wiki/Caf%C3%A9").as_deref(),
            Some("Café")
        );
        assert_eq!(wiki_title("/wiki/User:Someone"), None);
        assert_eq!(wiki_title("https://example.com/wiki/X"), None);
        assert_eq!(
            site_url("Catalog:Dominus Infernus"),
            "https://roblox.fandom.com/wiki/Catalog:Dominus_Infernus"
        );
        assert_eq!(
            site_url("Café & more?"),
            "https://roblox.fandom.com/wiki/Caf%C3%A9_%26_more%3F"
        );
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
        let h = hits(raw);
        assert_eq!(h.len(), 2);
        assert_eq!(h[0].snippet, "The Dominus series");
        assert_eq!(h[1].title, "Catalog:Dominus Infernus");
    }
}

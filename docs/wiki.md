# Wiki

Articles about items, experiences and players from the [Roblox Wiki](https://roblox.fandom.com),
a community wiki on Fandom, inside Bonsai.

**Status:** fetched and parsed for real, behind an opt-in setting. **Not yet checked against the
live site**: the spec and replay fixture were written from the MediaWiki API docs on a machine
that couldn't reach fandom.com. Before relying on it, capture real responses (see
[Checking against the site](#checking-against-the-site)).

## What's there

| Where | What | Command |
|---|---|---|
| Settings → Data → Third-party data | "Wiki articles", off by default | Turn wiki articles from Fandom on or off (`wiki.toggle`) |
| Wiki screen (`/wiki`, sidebar) | Search the wiki; results open in the article screen | Go to Wiki (`g w`), Search the wiki |
| Article (`/wiki/<Title>`) | Contents, lead and sections, infobox, categories, a button to the page in Bonsai, attribution | Open wiki article, Open wiki article on Fandom |
| Experience → About / Wiki | The start of the article under the description; the full article in a tab | Read the wiki article about this |
| Item → Wiki | The full `Catalog:` article | Read the wiki article about this |
| Profile → Overview / Wiki | A summary card and the full article | Read the wiki article about this |

Tabs and cards show only when there is an article, so pages without one look as before. While
the setting is off, nothing is fetched; the Wiki screen and article links show a "Turn on" button.

Pages are rendered as the wiki has them: every infobox field, every section, every category
except hidden maintenance ones. Nothing is filtered by topic. Prices and values in an item's
infobox are the wiki's own text.

## How it works

- **Client.** `specs/undocumented/roblox.fandom.com.yaml` describes MediaWiki's `api.php`, so
  `bonsai-api` generates the client like any other host (no extra crate). One operation,
  `action_api`, since `action` picks what api.php does. The transport never sends Roblox
  credentials to a non-Roblox host, and requests share the machine rate limiter.
  - Article: `action=parse&page=<title>&prop=text|categories|revid|displaytitle&redirects=1`.
  - Search: `action=query&list=search&srsearch=<text>&srnamespace=*`, then only main-namespace
    and `Catalog:` titles are kept.
- **Parsing** (`bonsai-core/src/wiki.rs`). The page HTML is parsed with `dom_query` (already in
  the tree through Tauri) into paragraphs, lists, tables and notes, plus the portable infobox
  (`aside.portable-infobox`) or a classic `table.infobox`. Links to other articles become
  `[[Title|label]]`; links to missing pages, files, categories, user and talk pages become plain
  text. Scripts, styles, images, navboxes, the TOC, edit links and references are dropped. Only
  this text crosses IPC; no Fandom HTML reaches the webview, and the CSP is unchanged.
- **Matching.** Items are looked up as `Catalog:<name>`, experiences and players by name, with
  update tags like `[🎃 UPDATE]` stripped from experience names. The roblox.com links in an
  article decide whether it is about this very page: an item article is rejected only if it links
  a different item; an experience or player article must link that place or profile, since names
  alone collide too often.
- **Cache.** Articles are shared public data (`Key::shared`), kept 12 hours; searches 1 hour.
  The saved copy shows at once on the next visit.
- **Open on Fandom.** `wiki.open_site` builds the address from the wiki's host in Rust and opens
  it with the system browser. It never opens a link taken from a response or the UI.
- **IPC** (`bonsai/src/ipc/wiki.rs`): `wiki_page(title)`, `wiki_about(kind, id, name)` and
  `wiki_search(query)`, read with `useLive`. The browser preview (`bun run dev`) shows made-up
  articles from `ui/src/lib/wiki-sample.ts`; the app never does.
- **UI.** `ui/src/lib/wiki.ts` (pure helpers, tested), `ui/src/lib/wiki-panels.tsx` (shared
  views and the navigating `wiki.*` handlers, in `lib/` because three features embed them) and
  `ui/src/features/wiki/` (the two screens).

## Checking against the site

1. On a machine that can reach Fandom, open
   `https://roblox.fandom.com/api.php?action=parse&page=Catalog:Dominus_Infernus&prop=text|categories|revid|displaytitle&redirects=1&format=json&formatversion=2`
   and a search like `...api.php?action=query&list=search&srsearch=dominus&srnamespace=*&format=json&formatversion=2`.
2. Replace `crates/bonsai-api/tests/fixtures/wiki.json` with those responses (`"source": "live"`)
   and update the spec's `x-captured` note.
3. Check what the parser makes of a few real pages (an item, an experience, a player, a long
   article with tables): `parse_html` in `wiki.rs` has a unit test to extend. Things to look at:
   which template classes Fandom uses for notices and quotes, tabbers (`.tabber`), galleries
   inside infoboxes, and whether experience and player articles link their roblox.com pages.

## Not done

- **Images.** The infobox picture and inline images are dropped. They should come through
  `thumb://` from a cache, like other thumbnails, never as remote URLs.
- **Experience fan wikis** (e.g. `<game>.fandom.com`). Left out: there is no lookup from a place
  to its wiki, and guessing hosts isn't reliable.
- **"Linked from".** MediaWiki has `list=backlinks` for it; not fetched yet.

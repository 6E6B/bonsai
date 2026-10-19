# Wiki

Articles about items, experiences and players from the [Roblox Wiki](https://roblox.fandom.com),
a community wiki on Fandom, inside Bonsai.

**Status:** fetched and parsed for real, behind an opt-in setting. Checked against the live site
on 2026-10-19: the fixture is a capture, and the parser has tests built from real item,
experience and player pages (see [Checked against the site](#checked-against-the-site)).

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
  - Search: `action=query&list=search&srsearch=<text>&srnamespace=0|112|114` (main, `Catalog:`,
    `Player:`; numbers from `meta=siteinfo`). Fandom returns no snippets, so results show the
    title, kind and last edit.
- **Parsing** (`bonsai-core/src/wiki.rs`). The page HTML is parsed with `dom_query` (already in
  the tree through Tauri) into paragraphs, lists, tables and notes, plus the portable infobox
  (`aside.portable-infobox`) or a classic `table.infobox`. Links to other articles become
  `[[Title|label]]`; links to missing pages, files, categories, user and talk pages become plain
  text. Scripts, styles, images, navboxes, the TOC, edit links and references are dropped. Only
  this text crosses IPC; no Fandom HTML reaches the webview, and the CSP is unchanged.
- **Matching.** The wiki's titles: items are `Catalog:<name>`, players `Player:<username>`, and
  experiences live under whoever made them, `<Group>/<Game>` or `Player:<user>/<Game>` (the bare
  name is tried next; it is often a redirect). Update tags like `[🎃 UPDATE]` and leading emoji
  are stripped from experience names. Then the article must be about this very page: an item
  article is rejected only if it links a different item; an experience article must link that
  place; a player article must link that profile or name the username in its infobox's
  `users/profile?username=` link, which is how most player infoboxes link.
- **Cache.** Articles are shared public data (`Key::shared`), kept 12 hours; searches 1 hour.
  The saved copy shows at once on the next visit.
- **Open on Fandom.** `wiki.open_site` builds the address from the wiki's host in Rust and opens
  it with the system browser. It never opens a link taken from a response or the UI.
- **IPC** (`bonsai/src/ipc/wiki.rs`): `wiki_page(title)`, `wiki_about(kind, id, name, creator, creator_group)` and
  `wiki_search(query)`, read with `useLive`. The browser preview (`bun run dev`) shows made-up
  articles from `ui/src/lib/wiki-sample.ts`; the app never does.
- **UI.** `ui/src/lib/wiki.ts` (pure helpers, tested), `ui/src/lib/wiki-panels.tsx` (shared
  views and the navigating `wiki.*` handlers, in `lib/` because three features embed them) and
  `ui/src/features/wiki/` (the two screens).

## Checked against the site

On 2026-10-19, against `Catalog:Dominus Infernus`, `Badimo/Jailbreak`, `Player:Builderman` and
`Dominus (series)`. What real pages needed, each with a test in `wiki.rs`:

- Leads are often bare inline text before the first block, not a `<p>`.
- Infobox labels carry tooltip text (`.wds-dropdown__content`), dropped. Tabbed infobox groups
  (`.wds-tabs`) keep their content and drop the tab labels.
- Headings are `<h2><span class="mw-headline" id=...>`, not `<div class="mw-heading">`.
- Sections that are empty except for subsections (History → Release history) are kept;
  sections left empty after dropping galleries and maps are not.
- Notices are `.mbox` boxes; their close buttons and collapsible toggles are dropped.
- Item prices carry a Robux icon image, dropped with other images.

To check again, fetch a page with the same parameters (see the spec) with `format=json&formatversion=2`
and compare with what `parse_html` makes of it.

## Not done

- **Images.** The infobox picture and inline images are dropped. They should come through
  `thumb://` from a cache, like other thumbnails, never as remote URLs.
- **Experience fan wikis** (e.g. `<game>.fandom.com`). Left out: there is no lookup from a place
  to its wiki, and guessing hosts isn't reliable.
- **"Linked from".** MediaWiki has `list=backlinks` for it; not fetched yet.

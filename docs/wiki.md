# Wiki

Articles about items, experiences and players from the [Roblox Wiki](https://roblox.fandom.com),
a community wiki on Fandom, inside Bonsai, with their pictures, and articles from an experience's
own Fandom wiki (like [jailbreak.fandom.com](https://jailbreak.fandom.com)) when the Roblox Wiki
points to one.

**Status:** fetched and parsed for real, behind an opt-in setting. Checked against the live site
on 2026-10-19: the fixture is a capture, and the parser has tests built from real item,
experience and player pages (see [Checked against the site](#checked-against-the-site)).

## What's there

| Where | What | Command |
|---|---|---|
| Settings → Data → Third-party data | "Wiki articles", off by default | Turn wiki articles from Fandom on or off (`wiki.toggle`) |
| Wiki screen (`/wiki`, sidebar) | Search the wiki; results open in the article screen. `?w=<wiki>` searches an experience's own wiki | Go to Wiki (`g w`), Search the wiki (`w:c:jailbreak: torero` searches the Jailbreak Wiki) |
| Article (`/wiki/<Title>`) | Contents, lead and sections with pictures and galleries, infobox with its picture, categories, a button to the page in Bonsai, the experience's own wiki, attribution | Open wiki article, Open wiki article on Fandom, Open link from wiki article |
| Experience → About / Wiki | The start of the article under the description, with a link to the experience's own wiki; the full article in a tab, with a switch to that wiki's main page | Read the wiki article about this |
| Item → Wiki | The full `Catalog:` article | Read the wiki article about this |
| Profile → Overview / Wiki | A summary card and the full article | Read the wiki article about this |

Tabs and cards show only when there is an article, so pages without one look as before. While
the setting is off, nothing is fetched; the Wiki screen and article links show a "Turn on" button.

Pages are rendered as the wiki has them: every infobox field, every section, every picture and
gallery, every category except hidden maintenance ones. Nothing is filtered by topic. Prices and
values in an item's infobox are the wiki's own text.

Links in the text open what they point to: another article in Bonsai (on the same or another
Fandom wiki), a Roblox item, experience, profile or community in Bonsai, or another site in the
browser (marked with an arrow). Links that are just an address show it short, like
`blog.roblox.com/2017/08/…`.

## How it works

- **Client.** `specs/undocumented/roblox.fandom.com.yaml` describes MediaWiki's `api.php`, so
  `bonsai-api` generates the client like any other host (no extra crate). One operation,
  `action_api`, since `action` picks what api.php does. The transport never sends Roblox
  credentials to a non-Roblox host, and requests share the machine rate limiter.
  - Article: `action=parse&page=<title>&prop=text|categories|revid|displaytitle&redirects=1`.
  - Search: `action=query&list=search&srsearch=<text>&srnamespace=0|112|114` (main, `Catalog:`,
    `Player:`; numbers from `meta=siteinfo`). Fandom returns no snippets, so results show the
    title, kind and last edit. On other wikis, main namespace only.
  - Other wikis: every Fandom wiki serves the same `api.php` on its own subdomain, so
    `bonsai_api::fandom::action_api_on` sends the generated operation to `<wiki>.fandom.com`
    (the name must be a valid subdomain). `meta=siteinfo&siprop=general` gives the wiki's name
    and main page (`Jailbreak Wiki:Home`), fetched with each of its articles.
  - Images: `Client::fetch_wiki_image`, which only fetches from `static.wikia.nocookie.net`.
- **Parsing** (`bonsai-core/src/wiki.rs`). The page HTML is parsed with `dom_query` (already in
  the tree through Tauri) into paragraphs, lists, tables and notes, plus the portable infobox
  (`aside.portable-infobox`) or a classic `table.infobox`. Links to other articles become
  `[[Title|label]]` (`[[w:c:jailbreak:Torero|label]]` on another wiki); links to Roblox pages
  Bonsai has become `[[@item:<id>|label]]` (`@bundle`, `@game` by place, `@user`, `@group`);
  other http(s) links `[[https://…|label]]`. Links to missing pages, files, categories, user and
  talk pages become plain text. Scripts, styles, navboxes, the TOC, edit links and references
  are dropped. Only this text crosses IPC; no Fandom HTML reaches the webview.
- **Pictures.** Figures (`figure`, `div.thumb`), galleries (`.wikia-gallery`, `ul.gallery`) and
  the infobox picture become image blocks: a path on Fandom's image host, a caption with links,
  and the size the article shows. Lazy-loaded images keep their address in `data-src`. Images
  from any other host, and icons under 40 px inside text, are left out. The path asks for twice
  the shown width, for high-density screens. The webview loads them from `wikiimg://<path>`
  (`bonsai/src/wikiimg.rs`): `Core::wiki_image` checks the path stays on that host, fetches it
  with the anonymous client, keeps it in the image disk cache and serves nothing while the wiki
  is off. The CSP gains only that local scheme in `img-src`.
- **Experience wikis.** The Roblox Wiki's experience articles open with a notice, "This
  experience has a dedicated wiki project", linking the wiki's main page
  (`https://jailbreak.fandom.com/wiki/`). Main-page links to other Fandom wikis inside notices
  become `Article::wikis`; elsewhere they are just mentions (a main page's list of sister wikis).
  Articles there are titled `w:c:<wiki>:<title>`, MediaWiki's own form for links between
  wikis, and `w:c:<wiki>:` is the main page. Their links stay on that wiki. They never point a
  page in Bonsai at an experience, since every page there links it.
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
  it with the system browser.
- **Links to other sites.** `wiki.open_link` takes the article's title and the link, and opens
  it only if the cached article has that very link (`Article::links`), so the UI can't open
  just any address.
- **IPC** (`bonsai/src/ipc/wiki.rs`): `wiki_page(title)`, `wiki_about(kind, id, name, creator, creator_group)` and
  `wiki_search(wiki, query)`, read with `useLive`. The browser preview (`bun run dev`) shows made-up
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
- Item prices carry a Robux icon image, dropped with other icons in text.
- Pictures are lazy-loaded: the real address is in `data-src`, `src` is a 1×1 GIF.
- Galleries are `.wikia-gallery` with `.lightbox-caption` captions; infobox pictures can be a
  tabbed `.pi-image-collection`, of which the first is shown.
- Experience articles link the experience's own wiki from an `.mbox` notice. Checked on
  jailbreak.fandom.com (`Jailbreak Wiki:Home`, `Torero`): same API and markup.

To check again, fetch a page with the same parameters (see the spec) with `format=json&formatversion=2`
and compare with what `parse_html` makes of it.

## Not done

- **Experiences without a Roblox Wiki article.** Their own wiki is found only through the Roblox
  Wiki's notice; there is no lookup from a place to a wiki, and hosts aren't guessed.
- **Other languages** of a wiki (`jailbreak.fandom.com/es/`): links to them open nothing.
- **Pictures in a viewer.** Pictures show in the article; there is no larger view yet.
- **"Linked from".** MediaWiki has `list=backlinks` for it; not fetched yet.

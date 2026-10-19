# Wiki (prototype)

Articles about items, experiences and players from the [Roblox Wiki](https://roblox.fandom.com)
and each experience's own fan wiki on Fandom, inside Bonsai.

**Status: UI prototype.** Every article comes from made-up sample data in
`crates/bonsai/ui/src/lib/wiki-sample.ts`, in the app and in a browser alike. Nothing talks to
Fandom yet.

## What's there

| Where | What | Command |
|---|---|---|
| Wiki screen (`/wiki`, sidebar) | Search across all wikis, filter by kind (items, experiences, players, other), articles by kind, recently edited | Go to Wiki (`g w`), Search the wiki |
| Article (`/wiki/<wiki>/<slug>`) | Contents, text with links between articles, infobox, "Linked from", a button to the page in Bonsai, attribution | Open wiki article, Open wiki article on Fandom |
| Experience → About | The start of the Roblox Wiki article under the description | Read the wiki article about this |
| Experience → Wiki | The experience's own wiki: pages by category, search, a front page, pages open in place. Falls back to the Roblox Wiki article | — |
| Item → Wiki | The Roblox Wiki article about the item | Read the wiki article about this |
| Profile → Overview / Wiki | A summary card for notable players, and the full article | Read the wiki article about this |

Tabs and cards only show when an article exists, so most pages look as before.

## Code

- `ui/src/lib/wiki.ts`: view types, search, link parsing, which article belongs to which Bonsai
  page. Pure, tested in `wiki.test.ts`.
- `ui/src/lib/wiki-panels.tsx`: shared pieces (`WikiArticleView`, `WikiSummary`, `WikiTeaser`,
  `GameWikiPanel`) and the `wiki.*` command handlers. They live in `lib/` because the game, item
  and profile features embed them.
- `ui/src/features/wiki/`: the Wiki screen and the article screen.
- `bonsai-core/src/commands.rs`: `nav.wiki`, `wiki.open`, `wiki.search`, `wiki.article`,
  `wiki.open_site`. All are reads; the UI handles them.

## To make it real

1. **Client.** Fandom wikis run MediaWiki, so each has an `api.php`. Capture the requests the
   site makes (search, page content, page info) and add them as an undocumented spec for
   `roblox.fandom.com` in `specs/undocumented/`, per AGENTS.md. Experience wikis are other hosts
   with the same API, so the client needs the host as a parameter rather than one spec per wiki.
2. **Parsing.** Turn page HTML or wikitext into the `WikiBlock`s the UI already renders
   (paragraphs, lists, tables, notes), infobox fields into `WikiFact`s, and internal links into
   `[[links]]`. Drop everything else (scripts, styles, ads, navboxes). Do this in Rust so no
   HTML from Fandom reaches the webview.
3. **Images.** Serve wiki images through `thumb://` from a cache, like other thumbnails. The CSP
   stays as it is.
4. **Matching.** Roblox Wiki infoboxes for items and experiences usually carry the asset or place
   ID. Use it to link articles to Bonsai pages (`WikiTarget`), and only fall back to a name search
   with the ID confirmed. Which wiki belongs to an experience needs a small curated list (place
   ID → host), since Fandom has no lookup by place ID.
5. **Cache and limits.** Articles are shared public data, cached across accounts with a long
   lifetime and refetched when the revision changes. Requests share the machine rate limiter.
6. **Opt-in.** Like Rolimons values, a third-party site: off until turned on in Settings, with a
   line saying what is fetched and when.
7. **Attribution.** Fandom text is CC BY-SA. Keep the source, license and link under every
   article and summary, as the prototype does.
8. **IPC.** Move the types in `wiki.ts` to `ipc.rs` views and read them with `useLive`. Replace
   the toast in `wiki.open_site` with opening the browser from Rust.

## Open questions

- Should "Open on Fandom" exist at all? The app otherwise opens only Roblox pages in the browser.
  The CC BY-SA link back is needed either way; it could be plain text with a copy action.
- Which experience wikis to list, and who maintains the list.
- Whether to show wiki prices and "value" fields for limiteds, or hide them in favor of live RAP
  and Rolimons data. The prototype shows them with a note.

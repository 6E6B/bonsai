# UI review, November 2026

A pass over every screen at 1440×900 and 1024×680 (mocked build, `bun run perf:build`).
Check an item off when it ships.

## Whole app

- [x] **Chrome.** Four bands sit above content (title bar, top bar, tab strip, page title). Move the
  command bar, sidebar toggle and top-bar buttons into the title bar row. In `auto` sidebar mode,
  collapse the sidebar to its icon rail on screens with their own side panel (Marketplace,
  Inventory, Settings).
- [x] **Repeated page names.** The name shows in the sidebar, the tab and the page title. On screens
  with top-level sub-tabs, drop the title and put the screen's actions on the tab row.
- [x] **Left edge jumps.** Narrow pages (Friends, Notifications, Communities, Search) are centred,
  so the title moves sideways between tabs. Left-align every page at the same inset; cap width only.
- [x] **Accent overuse.** Green marks primary buttons, tabs, counts, "Limited" tags, serials, links,
  focus and the Hub warning. Keep the accent for the primary action and the current selection.
- [x] **Count badges.** Red on the account rail, solid green in the sidebar, outlined green on tabs.
  One neutral count pill everywhere; coloured only for unread.
- [x] **Selected rows.** In Messages, Trades and Chat the selected row looks like a hovered row.
  Mark it with an accent bar.
- [x] **Skeletons.** Discover and Pass skeletons are big blocks unlike the loaded layout. Build them
  from the same shelf and tile shapes.
- [x] **Heading hierarchy.** Page titles (18px) barely differ from section titles (15px). Where page
  titles stay, make them larger.

## Screens

- [x] **Home.** The Continue row leaves a quarter of the width empty; let it fill. Add a Friends
  online row.
- [x] **Friends.** Rows are two lines with empty space beside them. Show the game a friend is in
  and a Join button. (Already shown for friends in a game; the mocked build had none.)
- [x] **Notifications.** Group by day (Today, Yesterday, earlier) and mark unread rows.
- [x] **Communities.** Show the user's role in each community. (Already in the subtitle.)
- [x] **Marketplace, Inventory.** "Limited" and serial tags on every tile are noise; use the neutral
  media-chip style. The Filters block pinned to the bottom of the side panel covers categories on
  short windows; move it to the top of the panel.
- [x] **Item page.** The details card wraps "Updated" onto its own row. Use a layout that fits.
- [x] **Transactions.** At 1024px wide the table squashes headers to "E" and "W": drop columns or
  move the detail pane below. The empty table has no message. "Syncing" and the Sync spinner say
  the same thing. Three toolbar rows can be two.
- [x] **Trades.** The `$` and copy header icons have no labels.
- [x] **Messages.** Cap the body at about 70 characters per line.
- [x] **Hub.** The "unavailable" banner is green; use the warning colour. Under it, "No releases
  found, try another search" contradicts it, and "0 themes on this page" adds nothing.
- [x] **Themes.** The pencil and ⋯ buttons sit on every preview. Show them on hover or focus.
- [x] **Values.** The header toggle and the "Turn on values" button do the same thing.
- [x] **Community.** Tabs the user can't use (Join requests, Audit log, Revenue, Settings) show
  greyed out. Hide them.
- [x] **Settings.** "Accounts on this com…" is cut off; call it "Saved accounts".
- [x] **Command palette.** An empty query says "No matches". Show recent places and suggested
  commands. (Without a query it listed every command; the mocked build has no registry.)
- [x] **`/overview`.** A static replica of an unrelated product page ships in release builds.
  Limit it to dev builds.

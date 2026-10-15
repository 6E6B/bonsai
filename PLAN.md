# Plan: finish the parity checklists

Source of truth for what is left: the unchecked `- [ ]` lines in `docs/parity/*.md`.

## Rules for every phase

Give the agent this file section plus `AGENTS.md`. Each phase is one agent, one PR. Phases in the same wave touch different areas and can run in parallel.

Every phase must:

1. First re-check each item in its list against the code. Tick it in `docs/parity/<area>.md` if it already exists. Do not rebuild it.
2. Look up each endpoint in `vendor/roblox-api-tracker/`, then `specs/undocumented/`. If neither has it, stop on that item and write it under "Needs capture" in `BLOCKERS.md`. Do not guess a URL or field.
3. Add a replay fixture for each new wrapper. Every new action is a registry command with title and keywords.
4. Finish with the Definition of done in `AGENTS.md` (fmt, clippy, test, `bun run check`, regenerated `bindings.ts`, parity file updated, `coverage.json`).
5. Only edit its own area. If it needs a shared change (a `lib/ui` component, `ipc.rs` types), keep it minimal and say so in the PR.

## Wave 0: endpoint capture (human, or an agent with a browser)

Blocks several later items. Capture from roblox.com devtools, add to `specs/undocumented/<host>.yaml` with a dated comment and a fixture. Scrub personal data.

- Avatar reset to default, emote equip/unequip.
- ~~Transaction history~~ captured 2026-10-15 from the site's Transactions bundle (`apis.roblox.com/transaction-records/v1/...`); see Phase 6.
- Group store, group funds balance, payouts (one-time and recurring), allies/enemies.
- Public servers region, voice chat and camera indicators, age rating, event banners, report flows.
- Premium tier and renewal date, payment history, age verification status, account PIN status, "who can message me".
- Sent friend requests (cancel), friend nicknames, followed experiences per user.

## Wave 1: independent, run in parallel

### Phase 1: Messages and notifications
Area: `features/messages`, `features/notifications`, `features/chat`, matching `rbx-core` modules.
- Notification categories (friend requests, trades, group shouts, experience updates, Robux and security alerts) with filter.
- Real-time updates for notifications and chat if a push channel exists. Otherwise poll via a core job that only notifies, and record that in the checklist.
- Verify Enter to send and Shift+Enter for newline in the composer. Fix or tick.

Status 2026-10-15: built, with desktop OS alerts for the kinds turned on in Settings. Message deletion and single-notification mark-read are dropped: no working endpoint (BLOCKERS.md).

### Phase 2: Auth and account safety
Area: `app/shell/AddAccount.tsx`, `rbx-core/src/account.rs`, `rbx-api` auth wrappers.
- 2FA modal: authenticator code, email code, remember device.
- Account PIN prompt for protected actions.
- Captcha and challenge modal that surfaces the challenge to the user and resumes the action after they solve it. Never auto-solve.
- Quick Login: enter a code from another device.
- Sign out of all other sessions.
- Act-as-alt account picker on join, buy and friend-request actions.

### Phase 3: Inventory
Area: `features/inventory`, `rbx-core/src/inventory.rs`.
- Categories: game passes, badges, places, development assets (audio, decals, meshes, models, plugins, videos).
- Item card details: thumbnail, name, creator, verified badge.
- Inspection drawer: description, creator, asset ID, resale price.
- Wear and remove directly from inventory.
- Search and filtering. Use the virtual list for anything over about 100 rows.

Status 2026-10-15: built. Places list only what the user made, and the passes fixture is empty (BLOCKERS.md #15).

### Phase 4: Profiles and friends
Area: `features/profile`, `features/friends`, `rbx-core/src/people.rs`.
- Followers list with follow/unfollow toggle, paginated.
- Connections tab follow toggle (do not load the whole following list).
- Cancel sent friend request, outgoing requests tab.
- About tab: bio with URL and hashtag formatting, currently wearing grid, collections and showcase.
- Creations tab: public models, decals, audio, development assets.
- Friend nicknames and notes, stored locally per account.
- Followed experiences per user.

### Phase 5: Search
Area: `features/discover` and the search entry in the palette.
- Full results page with tabs: All, Experiences, People, Groups, Marketplace.
- Recent queries, stored locally per account, with a clear command.

## Wave 2: needs Wave 0 captures

### Phase 6: Transactions
Area: new `features/transactions`, `rbx-core/src/transactions.rs`, `rbx-api` wrappers.

Endpoints (captured 2026-10-15, `specs/undocumented/apis.roblox.com.yaml`):
- `GET /transaction-records/v1/users/{id}/transaction-totals?timeFrame=Day|Week|Month|Year&transactionType=summary`: totals per category, incoming, outgoing, pending.
- `GET /transaction-records/v1/users/{id}/transaction-types`: which types the account has (`HasPurchase`, ...).
- `GET /transaction-records/v1/{users|groups}/{id}/transactions?transactionType=&limit=&cursor=`: one type per call, newest first, offset cursor. `id` is always 0; `idHash` is the stable key. The response's `transactionType` is a display string, so the type is the one we asked for.
- `GET /transaction-records/v1/groups/{id}/revenue/summary/{timeFrame}`: a community's totals.
- Rate limit is tight (429 after about 8 quick calls), so syncs are slow and shared.

Facts that shape the design:
- The site only offers Day / Week / Month / Year totals. "All time" and custom ranges come from the local ledger, labelled with how far back it goes.
- The list has no date filter, so the ledger is synced into SQLite: fetch newest pages per type until a known `idHash`, and fetch older history only on request (`transactions.load_older`), stoppable.
- Sale rows carry one amount. Whether it is net of the fee isn't verified (test account has no sales), so no gross/fee split until a real sale is recorded.
- Pending rows have `isPending`; the API gives no release date, so the screen shows "pending since" and the pending total, not a guessed timeline.

Steps (one PR each):
1. Spec, codegen, fixtures (live from the test account, scrubbed; synthetic for sales, stipends and groups), replay tests.
2. Core: ledger table, sync and backfill, totals, filters, range sums, CSV, rollups. Unit tests.
3. IPC and screen: header (balance, pending, scope), period summary tiles, virtual ledger, type filter, detail pane, commands.
4. Date range, search, amount filter, CSV save and copy.
5. Community ledgers and revenue summary for communities where the account may see funds.
6. Beyond the website: all-accounts rollup, monthly in/out chart, spend per experience, top items and buyers, pending panel, new-sale notification (watcher only notifies).

Status 2026-10-15: steps 1–6 built together. The pending panel is the Pending tile, the pending-only filter and the Pending badge on rows. Sales, stipends and community ledgers wait on a live recording (BLOCKERS.md #14).

### Phase 7: Communities admin
Area: `features/communities`.
- Group store: classic clothing, UGC catalog items, game passes.
- Group funds balance, one-time payout, recurring percentage splits.
- Allies tab (list, send request, remove) and Enemies tab.
- Writes need explicit confirmation dialogs. They are user-triggered only.

### Phase 8: Settings and discover gaps
Area: `features/settings`, `features/discover`, `features/game`.
- Settings: premium tier and renewal date, payment history, age verification, PIN status, who can message me, notification preferences (in-app, desktop OS, per activity type), language and region formats, custom theme palettes (tokens only, in `theme.css`).
- Discover: event banners, age filters, voice and camera indicators.
- Game: server region, report experience dialog if a report API exists.
- Items that stay a link to the website (change password, change username) are marked as such in the checklist and left alone.

## Wave 3: 3D, last and shared

### Phase 9: 3D preview
Area: new shared component in `src/lib/ui`, then three consumers.
- One renderer with orbit, zoom and pan. Bundle all assets, no network. Check WebKitGTK performance on Linux.
- Use it in the avatar editor viewport, the item page preview, and the profile full-body render.
- Item page extras: audio player, animation preview, "Try On" toggle.
- Emote hotbar (slots 1 to 8) and emote playback, if Wave 0 found the equip endpoints.
- Add the component to the `/components` gallery.

## Order

Wave 0 first. Wave 1 phases 1 to 5 can all start at once. Wave 2 phases start as their captures land. Phase 9 goes last, because it adds a heavy dependency and three screens should not conflict on it.

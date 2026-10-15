# Marketplace, Transactions & Trades

## Marketplace (Catalog)
- [x] **Browsing & Discovery**:
  - [x] Category navigation: Featured, Community Creations, Collectibles/Limiteds, Clothing, Accessories, Animations, Body Parts
  - [x] Subcategory filters (e.g. Hats, Hair, Face, Neck, Layered Clothing categories)
  - [x] Sort order: Relevance, Most Favorited, Bestselling, Price (Low to High, High to Low), Recently Created, Recently Updated
- [x] **Search & Filtering**:
  - [x] Text search (keyword matching, command bar `i:`)
  - [x] Price range filters: Minimum and Maximum Robux (including Free items)
  - [x] Creator filter: Search by username or group name
  - [x] Sales type: On Sale, Off Sale, Limiteds only
  - [x] Include unavailable items toggle
- [ ] **Item Details Page**:
  - [x] High-resolution 2D rendered image preview
  - [ ] Interactive 3D item preview (orbit, zoom, pan)
  - [ ] Audio asset player (for music and sound effects)
  - [ ] Animation preview player
  - [x] Creator attribution link with verified badge
  - [x] Pricing: Robux price, Free, Off Sale, or Lowest Resale Price for Limiteds
  - [x] Favorites count & Favorite toggle button
  - [x] Add to / Remove from local Wishlist
  - [ ] "Try On" avatar preview toggle
  - [x] Asset metadata: Asset ID, genres, tags, description, created and updated dates
  - [x] Download clothing template (shirts, pants, T-shirts) to the downloads folder, plain or masked to the template regions; in the item page's More menu, item right-click menus and the command bar (not run live yet)
- [x] **Limiteds & Resellers**:
  - [x] Resellers list: Seller avatar, username, serial number (#), resale price, Buy button
  - [x] Historical resale price chart: daily average price and copies sold, 7-day average, RAP, Rolimons value and best price lines, crosshair with price and date readout (hover or arrow keys), 7D/30D/90D/1Y/All, linear or log scale, chart or table (`market.history_range`, `market.history_scale`, `market.history_view`)
  - [x] Zoom and pan: scroll to zoom around the pointer, drag or sideways-scroll to pan, overview strip of the whole history with a draggable window, double-click or 0 to reset, +/- and Shift+arrows from the keyboard (`market.history_zoom`)
  - [x] Summary of the visible part: RAP vs its average, average sale, change, high/low, copies sold per day, days with sales, best price vs RAP
  - [x] Item ownership metrics: Total copies, copies remaining, original price
- [x] **Bundles**:
  - [x] "In this bundle" grid showing every included sub-asset
- [x] **Purchase Flow**: in-app, from the creator, the cheapest reseller or a picked resale copy (not run live yet, BLOCKERS.md #5)
  - [x] Buy confirmation modal (shows item name, price, current Robux balance, remaining balance)
  - [x] Insufficient Robux prompt with "Buy Robux on website" external link
  - [x] Recommended / Similar catalog items carousel

## Transactions
Source: `apis.roblox.com/transaction-records/v1` (captured 2026-10-15 from the site's Transactions bundle, `specs/undocumented/apis.roblox.com.yaml`). The site offers Day / Week / Month / Year totals; longer and custom ranges come from the local ledger.
- [x] **Transaction Types**:
  - [x] Summary by period (Day, Week, Month, Year from Roblox; All synced and custom ranges from the ledger), incoming, outgoing and net
  - [x] Purchases (item or pass, experience, seller, price paid)
  - [x] Sales of goods (item, buyer, amount; fee split once a real sale shows whether the amount is net; not run live yet, BLOCKERS.md #14)
  - [x] Premium stipends (not run live yet, BLOCKERS.md #14)
  - [x] Group payouts received (not run live yet, BLOCKERS.md #14)
  - [x] Currency purchases, trade Robux and the other types Roblox lists, shown only when the account has them
  - [x] Pending Robux: pending total, pending rows marked with when they started (Roblox gives no release date)
- [x] **Group Transactions**:
  - [x] Ledger and revenue summary for communities where the account may see funds (not run live yet, BLOCKERS.md #14)
- [x] **Filters & Export**:
  - [x] Date range selection
  - [x] Filter by transaction category
  - [x] Search by item, experience or person; minimum amount
  - [x] CSV export of the ledger (save to Downloads, or copy the rows shown)

## Transactions beyond the website
- [x] Local ledger: synced incrementally, older history on request, works offline
- [x] All your accounts: balance, pending and period totals per account and summed (Transactions → All accounts)
- [x] Monthly incoming vs outgoing chart
- [x] Spending per experience (passes and developer products grouped by experience)
- [x] Top items sold and top buyers
- [x] Notification on a new sale: checks the newest sales page every 5 minutes per account and toasts new ones; the first check only fills the ledger (Settings → Notifications → Sales, `app.notify sales`)

## Trades
- [x] **Trade Queues**:
  - [x] Inbound trades (incoming offers awaiting user action)
  - [x] Outbound trades (pending offers sent by the user)
  - [x] Completed trades (accepted trade history)
  - [x] Inactive trades (declined, expired, or voided trades)
- [x] **Trade Inspector**:
  - [x] Side-by-side inventory comparison: "Your Offer" vs "Their Offer"
  - [x] Item thumbnails, names, serial numbers, and original prices
  - [x] Recent Average Price (RAP) and Rolimons value calculations per side
  - [x] Robux addition display with marketplace fee calculation (30% fee deducted on received Robux)
  - [x] Net gain / loss calculation in Robux and percentage
- [x] **Trade Actions**:
  - [x] Accept trade (confirmation modal displaying final item exchange)
  - [x] Decline trade
  - [x] Counter trade (opens trade composer pre-populated with current items)
- [x] **Trade Composer**:
  - [x] Two-panel item selector (my tradable inventory vs partner tradable inventory)
  - [x] Up to 4 items selectable per side
  - [x] Robux addition input (validated against 50% max allowable Robux contribution rule)
  - [x] Search and filter tradable inventory by serial number, name, or RAP
- [x] **Safety & Value Alerts**:
  - [x] Lopsided trade warning (alert on trades with >25% value loss)
  - [x] Projected / manipulated item flag
  - [x] On-hold item flag
- [x] **Trade Settings**:
  - [x] Trade privacy configuration (Who can trade with me)
  - [x] Trade value threshold / Rolimons integration toggle

## Item values (Rolimons, opt-in)
Beyond the website. Source: `api.rolimons.com/items/v2/itemdetails` (`specs/undocumented/api.rolimons.com.yaml`), decoded in `rbx-core/src/values.rs`, refetched once the copy is 2 minutes old and only while a screen that shows values is open (`useValuesRefresh`).
- [x] Values screen (`nav.values`, G V): every tracked limited with value, RAP, RAP-vs-value gap, demand, trend and flags; search by name, acronym or ID; filter by Limited 1.0/2.0 and flags; sort by value, RAP, demand, gap or name
- [x] Item tiles: value next to RAP, and a chip over the picture with demand, trend and a projected warning
- [x] Item page: value panel with acronym, limited version, RAP gap, demand, trend, flags and update time
- [x] Trade rows: value, demand, trend and projected badge per item; value total per side
- [x] Inventory: sort collectibles by value or demand

# Home, Discover, Experience Page, Private Servers & Settings

## Home
- [x] Friends playing now, grouped by experience (game icon and title open it; Join per friend)
- [x] Continue playing (recent games launched from the app or website)
- [x] Recommended sorts (Roblox Home feed via `discovery-api/omni-recommendation`)
- [x] Favorites row (experiences favorited by the active account)
- [x] Today's picks and dynamic topic categories from the feed
- [ ] Promotional / seasonal event cards and banners — endpoint not captured; `/v1/events/single` and `virtual-events` exist but their response shapes aren't recorded (BLOCKERS #18)
- [x] Quick account overview (avatar, display name, username, current Robux balance)
- [x] Pull-to-refresh / manual feed reload button

## Discover and Charts
- [x] All sort categories (Popular, Top Rated, Up-and-Coming, Trending, paged with "See all")
- [x] Charts sorts (Top Grossing, Most Playing, paged)
- [x] Genre filters (Action, RPG, Simulation, Adventure, Comedy, Sports, Strategy, etc.)
- [x] Device filters (PC, Mobile, Console, VR)
- [x] Search integration with remote autocomplete (Omni-search, command bar `g:`)
- [x] Sponsored / promoted experiences toggle (hidden by default)

## Experience Page
- [x] Header: title, game icon, developer/group name (with verified badge and link)
- [x] Main Play / Join button (with account picker to "Join as alt")
- [x] Media carousel (16:9 thumbnails, YouTube gameplay trailers with titles) — trailers show their preview and title; they play on the website (the webview loads nothing remote)
- [x] Description (collapsible with markdown formatting, emoji, and link detection) — Roblox descriptions are plain text; Roblox links open in the app (experiences, profiles, communities, items) or browser, @mentions show the user's headshot and open their profile, other sites stay text
- [x] Stats grid: active players, total visits, favorites count, up/down votes, like ratio percentage
- [x] Metadata: max server size, genre, subgenre, created date, updated relative date
- [x] Age guidelines & maturity rating (descriptors: Mild Violence, Purchases, etc.)
- [x] Voice chat and camera support indicators — shown for the account in Settings > Account (`voice.roblox.com/v1/settings`), not per experience
- [x] Quick actions: vote up/down (with eligibility status), favorite toggle, follow/notify bell, report abuse
- [x] Sub-tab: About (full description, developer bio, place attributes)
- [x] Sub-tab: Store (game passes with icon, price in Robux, owned status, purchase flow) — bought in-app after a confirm; not run live yet (BLOCKERS.md #5)
- [x] Sub-tab: Badges (all badges with icon, name, description, rarity percentage, earned timestamp)
- [x] Sub-tab: Servers (public servers with ping, player counts, friend icons, sorting asc/desc, pagination)
- [x] Sub-tab: Friends' servers (filtered list of servers currently hosting friends)
- [x] Sub-tab: Private servers (list of owned servers, friends' servers you can access)
- [x] Sub-tab: Events (running and scheduled in-game events, event details, RSVP / "Notify me")
- [x] Sub-tab: Recommendations (similar experiences grid)

## Private Servers
- [x] Purchase private server modal (confirm dialog, monthly Robux fee, free servers support)
- [x] List all owned private servers across experiences
- [x] Configure private server:
  - [x] Name configuration
  - [x] Active / inactive status toggle
  - [x] Friends allowed toggle
  - [x] Player whitelist / member management (add/remove usernames)
  - [x] Generate and copy private server invite link
- [x] Subscription management: cancel or renew (renew on/off)
- [x] Direct Join private server action

## Settings
- [x] Layout: sections split into **Roblox account** (Profile, Security, Privacy, Private servers; saved on Roblox) and **This app** (General, Appearance, Background, Layout, Notifications, Keyboard, Accounts, Data and storage, Developer; saved on this computer), with icons. Old names (`account`, `advanced`) still open via `nav.settings`. Shared `SettingsGroup` / `SettingRow` / `SettingSwitch` in `lib/ui`.
- [x] One list of app settings in `rbx-core/src/prefs.rs` (key, section, title, kind, default). `settings.set <key> [value]` changes one (a switch without a value flips) and `settings.reset <key|section|all>` puts defaults back; the command bar lists on/off and each choice per setting, and resets per section. Changed rows show a dot and a reset button.
- [x] Settings search in the side pane (titles, descriptions, keywords and the Roblox account rows); results open `/settings/:section/:setting`, which scrolls to and flashes the row. `nav.settings` takes `section/setting` too.
- [ ] **Account Info**:
  - [x] Display name (edit with 7-day cooldown validation)
  - [x] Email address and verification status
  - [x] Phone number and verification status
  - [x] Birthday and age verification indicator — `age-verification-service/v1/age-verification/verified-age`
  - [x] Account PIN status (unlock with PIN) — `account.unlock_pin`, `account.lock_pin`
- [ ] **Security**:
  - [ ] Change password
  - [ ] 2-Step Verification (Authenticator App / TOTP, Email codes, Hardware Security Keys / Passkeys)
  - [x] Cross-Device Quick Login (generate code for other devices, enter code from another device) — `account.quick_sign_in`, `account.approve_code`
  - [x] Active session list (devices, locations, last active) — IPs and tokens are dropped in core
  - [x] Sign out of all other sessions action — `account.sign_out_others`
- [x] **Privacy**:
  - [x] Who can chat with me in app
  - [x] Who can chat with me in experience
  - [x] Who can join me in game
  - [x] Who can see my inventory
  - [x] Who can trade with me
- [x] **Notifications**:
  - [x] Notification preferences — switches for friend joins, trades, chats, friend requests, experience updates and sales (`app.notify`) pick which alerts are raised
  - [x] Desktop OS push notification toggles — the same switches pick which alerts show while the app is in the background
  - [x] Activity notifications (trades, friend requests, chats, experience updates; group shouts show in the stream under Communities)
- [x] **Blocked Users**:
  - [x] Paginated list of blocked accounts with avatar and username
  - [x] Unblock button with confirmation
- [ ] **Language & Region**:
  - [ ] App display language selector
  - [x] Localized number and date/time format — Settings > Appearance, `app.format_locale`; relative times ("5m ago") stay English
- [ ] **Billing & Subscriptions**:
  - [x] Premium membership tier and renewal date — `premiumfeatures/v1/users/{id}/subscriptions/details` (shape from the site bundle; fixture is synthetic)
  - [x] Active private server subscriptions
- [ ] **App Preferences (Client native)**:
  - [ ] Theme (Light, Dark, System, Custom palettes) — twelve built-in themes, a dark and a light pick that System switches between; editor and import in progress, see [theming.md](../theming.md)
    - [x] Token registry, theme document format and seed derivation (`ui/src/lib/theme/`)
    - [x] Theme storage, selection and `theme.apply` / `theme.cycle` / `theme.delete` (`rbx-core/src/themes.rs`, Settings > Appearance)
    - [x] Google Fonts in themes: searchable picker, `theme.font` / `theme.mono_font` / `theme.duplicate`, files cached on disk and served over `font://` (`rbx-core/src/fonts.rs`)
    - [x] Theme editor with live preview and contrast checks: Themes screen (`nav.themes`, G E), editor tab at `/themes/:id` with seeds, per-token pins, undo/redo, contrast checks with `theme.fix_contrast`, `theme.live` to show the draft app-wide (`ui/src/features/themes/`)
    - [ ] Import and export (`.bonsaitheme` files and share codes)
    - [x] Inspect mode (`theme.inspect`: click part of the editor preview to list the tokens that paint it)
    - [ ] Theme hub (community themes on the Themes screen; later)
    - [x] Background picture or GIF behind pages, from a file or https link, with picture strength, page opacity and effects (blur, dim, saturation, accent tint, vignette, scanlines, grain) (`view.background*`, Settings > Appearance; see [theming.md](../theming.md#background-picture))
  - [x] UI density / scaling (Compact, Comfortable)
  - [x] Sidebar layout: screens in groups split by dividers; drag rows or `view.sidebar_up`/`view.sidebar_down` to reorder within a group, `view.sidebar_toggle` to hide (right-click menu, Settings > Appearance); Messages hidden by default; Transactions also opens from Trades
  - [x] Player launch command configuration (Default OS protocol, Flatpak/Sober, Wine, custom script)
  - [x] Disk cache manager (view cache size on disk, clear thumbnail and data caches)
  - [x] Automatic cache upkeep (data and images unused for 30 days are dropped, images capped at 512 MB, idle screens and images leave memory)

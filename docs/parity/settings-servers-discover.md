# Home, Discover, Experience Page, Private Servers & Settings

## Home
- [x] Friends playing now, grouped by experience (game icon and title open it; Join per friend)
- [x] Continue playing (recent games launched from the app or website)
- [x] Recommended sorts (Roblox Home feed via `discovery-api/omni-recommendation`)
- [x] Favorites row (experiences favorited by the active account)
- [x] Today's picks and dynamic topic categories from the feed
- [ ] Promotional / seasonal event cards and banners — endpoint not captured
- [x] Quick account overview (avatar, display name, username, current Robux balance)
- [x] Pull-to-refresh / manual feed reload button

## Discover and Charts
- [x] All sort categories (Popular, Top Rated, Up-and-Coming, Trending, paged with "See all")
- [x] Charts sorts (Top Grossing, Most Playing, paged)
- [x] Genre filters (Action, RPG, Simulation, Adventure, Comedy, Sports, Strategy, etc.)
- [x] Device filters (PC, Mobile, Console, VR)
- [ ] Age guidelines / maturity filters (All ages, 9+, 13+, 17+) — no filter in the explore or charts APIs
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
- [ ] Voice chat and camera / face tracking support indicators — no known endpoint; not captured yet
- [x] Quick actions: vote up/down (with eligibility status), favorite toggle, follow/notify bell, report abuse
- [x] Sub-tab: About (full description, developer bio, place attributes)
- [x] Sub-tab: Store (game passes with icon, price in Robux, owned status, purchase flow) — bought in-app after a confirm; not run live yet (BLOCKERS.md #5)
- [x] Sub-tab: Badges (all badges with icon, name, description, rarity percentage, earned timestamp)
- [ ] Sub-tab: Servers (public servers with ping, region, player counts, friend icons, sorting asc/desc, pagination) — all but region, which the API doesn't return
- [x] Sub-tab: Friends' servers (filtered list of servers currently hosting friends)
- [x] Sub-tab: Private servers (list of owned servers, friends' servers you can access)
- [x] Sub-tab: Events (running and scheduled in-game events, event details, RSVP / "Notify me")
- [x] Sub-tab: Recommendations (similar experiences grid)
- [ ] Report experience dialog (category selection, details, submit report) — no report API; opens the website's report page

## Private Servers
- [x] Purchase private server modal (confirm dialog, monthly Robux fee, free servers support)
- [x] List all owned private servers across experiences
- [x] Configure private server:
  - [x] Name configuration
  - [x] Active / inactive status toggle
  - [x] Friends allowed toggle
  - [x] Player whitelist / member management (add/remove usernames)
  - [x] Generate and copy private server invite link
- [ ] Subscription management: cancel subscription, renew, view payment history — renew on/off done; payment history has no known endpoint
- [x] Direct Join private server action

## Settings
- [ ] **Account Info**:
  - [x] Display name (edit with 7-day cooldown validation)
  - [ ] Username change (modal with Robux fee check) — links to the website (account security area, not automated)
  - [x] Email address and verification status
  - [x] Phone number and verification status
  - [ ] Birthday and age verification indicator — birthday shown; age verification status not captured
  - [ ] Account PIN status (unlock with PIN)
- [ ] **Security**:
  - [ ] Change password
  - [ ] 2-Step Verification (Authenticator App / TOTP, Email codes, Hardware Security Keys / Passkeys)
  - [ ] Cross-Device Quick Login (generate code for other devices, enter code from another device)
  - [ ] Active session list (browsers, devices, IP locations, last active timestamp)
  - [ ] Sign out of all other sessions action
- [ ] **Privacy**:
  - [ ] Who can message me (Everyone, Friends of Friends, Friends, No One) — not in the user-settings API this account sees
  - [x] Who can chat with me in app
  - [x] Who can chat with me in experience
  - [x] Who can join me in game
  - [x] Who can see my inventory
  - [x] Who can trade with me
- [ ] **Notifications**:
  - [ ] In-app notification stream preferences — switches for friend joins, trades, chats, friend requests and experience updates are saved (`app.notify`); nothing reads them yet, as no notification stream exists
  - [ ] Desktop OS push notification toggles — same saved switches; no OS notification delivery yet
  - [ ] Activity notifications (trades, friend requests, group shouts, chats)
- [x] **Blocked Users**:
  - [x] Paginated list of blocked accounts with avatar and username
  - [x] Unblock button with confirmation
- [ ] **Language & Region**:
  - [ ] App display language selector
  - [x] Localized number and date/time format — Settings > Appearance, `app.format_locale`; relative times ("5m ago") stay English
- [ ] **Billing & Subscriptions**:
  - [ ] Premium membership tier and renewal date — Premium yes/no shown; tier and renewal date not captured
  - [x] Active private server subscriptions
- [ ] **App Preferences (Client native)**:
  - [ ] Theme (Light, Dark, System, Custom palettes) — light, dark and system done; custom palettes not yet
  - [x] UI density / scaling (Compact, Comfortable)
  - [x] Player launch command configuration (Default OS protocol, Flatpak/Sober, Wine, custom script)
  - [x] Disk cache manager (view cache size on disk, clear thumbnail and data caches)
  - [x] Automatic cache upkeep (data and images unused for 30 days are dropped, images capped at 512 MB, idle screens and images leave memory)

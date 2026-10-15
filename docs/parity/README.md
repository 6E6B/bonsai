# Website Parity Checklists

This directory contains parity checklists for every page, screen, and subsystem in the application. All items are initially unchecked. An item is ticked when it ships in the frontend: the screen works from cache first, connects to `rbx-core`, supports keyboard navigation / commands, and has replay test coverage.

## Master Page Index

| Page / Screen | Area Document | Scope & Key Features |
|---|---|---|
| **Authentication & Multi-Account** | [auth-accounts-security.md](auth-accounts-security.md) | Cookie login, Quick login, account rail, instant switching, act-as-alt, 2FA, PIN, captcha |
| **Home** | [settings-servers-discover.md](settings-servers-discover.md#home) | Friends playing now row, Continue playing, Recommended sorts, Favorites, Event banners |
| **Discover & Charts** | [settings-servers-discover.md](settings-servers-discover.md#discover-and-charts) | Charts sorts, Genre and device filters, Maturity filters, Search, Sponsored hidden |
| **Experience Page** | [settings-servers-discover.md](settings-servers-discover.md#experience-page) | Media carousel, stats, voting, favorite, notify, store/passes, badges, servers list, events |
| **Private Servers** | [settings-servers-discover.md](settings-servers-discover.md#private-servers) | Buy, list across experiences, configure (whitelist, links), renew/cancel subscription, join |
| **Avatar Editor** | [avatar-inventory.md](avatar-inventory.md#avatar-editor) | 2D/3D viewport, wear/remove, skin colors, scales, emotes, outfits, R6/R15, layered clothing |
| **Inventory** | [avatar-inventory.md](avatar-inventory.md#inventory) | Asset categories, stacked counts, serials, search, privacy-aware inspection, delete item |
| **Marketplace (Catalog)** | [marketplace-trades.md](marketplace-trades.md#marketplace-catalog) | Category browsing, search & filters, item details, 3D preview, resellers, try on, wishlist |
| **Transactions** | [marketplace-trades.md](marketplace-trades.md#transactions) | Period summaries, purchases, sales, stipends, payouts, pending Robux, CSV export |
| **Trades** | [marketplace-trades.md](marketplace-trades.md#trades) | Inbound, outbound, completed, inactive; inspector, calculator, send/counter/accept/decline |
| **User Profiles** | [profiles-communities-search.md](profiles-communities-search.md#user-profiles) | About, presence, friends/followers, creations, collections, badges, statistics |
| **Friends & Connections** | [profiles-communities-search.md](profiles-communities-search.md#friends-connections) | Drawer list with presence, requests queue, followers/following, block/unblock |
| **Communities (Groups)** | [profiles-communities-search.md](profiles-communities-search.md#communities-groups) | Browse, search, forums, shouts, member management, join requests, audit log, store, payouts |
| **Badges & Followings** | [profiles-communities-search.md](profiles-communities-search.md#badges-favorites--following) | Badges per user/experience, user favorites, followed games |
| **Search & Command Bar** | [profiles-communities-search.md](profiles-communities-search.md#search--command-palette) | Omni-search, user search (`@`), communities (`#`), games (`g:`), items (`i:`) |
| **Private Messages** | [messages-chat-notifications.md](messages-chat-notifications.md#private-messages) | Inbox, sent, archive, news, compose, archive/read triage |
| **Chat** | [messages-chat-notifications.md](messages-chat-notifications.md#chat) | Conversations, 1-on-1 and group chat, typing indicators, read receipts, cards, drafts |
| **Notifications** | [messages-chat-notifications.md](messages-chat-notifications.md#notifications) | Notification stream, unread counts, mark read |
| **Settings** | [settings-servers-discover.md](settings-servers-discover.md#settings) | Account info, security (2FA/sessions), privacy, notifications, blocked users, theme |

---

## Power Features (Beyond the Website)

For power-user features beyond the standard website (such as Rolimons integration, trade calculator with fee accounting, CSV exports, multi-account roles), see [features.md](../features.md).

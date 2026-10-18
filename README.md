<div align="center">

<img src="crates/bonsai/icons/128x128.png" width="96" alt="Bonsai icon">

# Bonsai

**A fast, keyboard-first desktop client for Roblox, built for people with more than one account.**

[![Rust](https://github.com/6E6B/bonsai/actions/workflows/rust.yml/badge.svg)](https://github.com/6E6B/bonsai/actions/workflows/rust.yml)
[![UI](https://github.com/6E6B/bonsai/actions/workflows/ui.yml/badge.svg)](https://github.com/6E6B/bonsai/actions/workflows/ui.yml)
[![License: GPL-3.0](https://img.shields.io/badge/license-GPL--3.0-blue.svg)](LICENSE)
![Platforms](https://img.shields.io/badge/platforms-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey)

[Features](#features) · [Install](#install) · [Getting started](#getting-started) · [Shortcuts](#keyboard-shortcuts) · [Plugins](#plugins) · [Building](#building-from-source) · [Architecture](#architecture)

<!-- TODO: hero screenshot. Home screen, dark theme, account rail with 3+ accounts, friends drawer open. -->
<img src="docs/images/hero.png" alt="Bonsai home screen with the account rail and friends drawer" width="900">

</div>

---

Bonsai is the Roblox website as a native app. Sign in with all your accounts at once, switch between them
with `Ctrl+1`–`Ctrl+9`, and do everything from one command bar: join games, trade, chat, manage
communities, track Robux, and change your avatar.

It's built with Rust and [Tauri 2](https://tauri.app), so it starts fast and uses little memory. Data is
cached on disk, so screens open with what you saw last instead of a spinner.

## Features

### Multiple accounts

- Sign in with as many accounts as you like: quick sign-in code, the Roblox sign-in page, or a pasted cookie.
- Switch with `Ctrl+1`–`Ctrl+9`, or `Ctrl+Tab` to go back to the last one.
- Give accounts local labels and color tags, and order them in the rail.
- See which of your accounts own an item, are in a community, or hold a role, on every page.
- Join a game, buy an item or send a friend request **as another account** without switching.
- Account security from the app: password, authenticator and email codes, recovery codes, PIN lock, sign
  out of other sessions.

<!-- TODO: screenshot of the account rail + command bar showing "Switch account". -->
<img src="docs/images/accounts.png" alt="Account rail and account switcher" width="800">

### Joining games

- Join a game, a friend, a specific server, or a server by job ID.
- Pick the **best server for you**, the smallest, or a random one.
- Copy server invite links. Buy, configure and join private servers.
- Find which server any user is in.
- Custom launch command, so it works with [Sober](https://sober.vinegarhq.org) on Linux.

### Trading and collectibles

- A trade calculator on every trade: RAP and value per side, Robux after fee, net gain and percent.
- Warnings for lopsided trades, projected items, on-hold items, and item or Robux limits.
- Go through inbound trades from the keyboard: `j`/`k` to move, `a` accept, `d` decline, `c` counter.
- Trade history filters and CSV export.
- Inventory RAP and value totals, per account and across all your accounts.
- Optional item values, demand and trend from [Rolimons](https://www.rolimons.com). Off until you turn it on.

<!-- TODO: screenshot of the Trades screen with the calculator and a warning visible. -->
<img src="docs/images/trades.png" alt="Trade screen with the calculator" width="800">

### Marketplace and avatar

- Search the Marketplace from the command bar (`i:` prefix).
- Resale price history chart with daily average and volume, on a log or linear scale.
- Wishlist that shows price drops since you added an item.
- Bundle breakdowns, clothing template downloads, and audio previews.
- 3D avatar view with animations and emotes. Try items on before you buy.
- Edit outfits, body colors, scales, layered clothing order and the emote wheel.

<!-- TODO: screenshot of an item page with price history, or the 3D avatar editor. -->
<img src="docs/images/marketplace.png" alt="Item page with price history" width="800">

### Messages, chat and friends

- Chat with live typing, unread counts and pinned chats. Group chats too.
- Drafts are saved per chat and survive restarts and account switches.
- Experience links in chats, messages and forums turn into cards with a Join button.
- Join the person you're chatting with, from the chat header.
- Private messages: search, bulk select, keyboard triage (`j`/`k`, `x`, `e`, `u`, `r`).
- Private nicknames and notes on any user, stored locally.
- Desktop notifications for friends joining games, trades, chats, friend requests, game updates and sales.

### Communities

- Shouts feed across all your communities, with new markers.
- Forums, members, ranks, payouts, allies and enemies, settings and audit log.
- Go through join requests from the keyboard (`a`, `d`, `x`) or answer them in bulk.
- Leave several communities in one confirmed action.
- Export member lists as CSV.

### Transactions

- A local ledger of your Robux history with custom date ranges, search, minimum amount and pending-only.
- Every account's balance and totals side by side.
- Insights: monthly in vs out, spending per experience, top items sold, top buyers.
- Save or copy as CSV. Get a toast when something sells.

<!-- TODO: screenshot of Transactions → Insights. -->
<img src="docs/images/transactions.png" alt="Transaction insights" width="800">

### Built for the keyboard

- One command bar (`Ctrl+K` or `/`) for **every** action in the app: navigation, joining, trading,
  settings, themes, plugins. If there's a button for it, there's a command for it.
- Vim-style list navigation (`j`/`k`/`Enter`) and `g`-prefixed jumps between screens.
- Browser-style tabs with pinning, reordering and reopen-closed-tab.
- Every shortcut can be rebound.

<!-- TODO: GIF of opening the command bar, typing "join", picking a friend. ~5 seconds. -->
<img src="docs/images/command-bar.gif" alt="Running a command from the command bar" width="800">

### Make it yours

- A built-in theme editor: pick three colors and Bonsai derives the rest. Pin single tokens if you want.
- Fix low contrast with one command.
- Share themes as a short code or a file. Themes are plain data and can't run code.
- Any Google Fonts family, downloaded once and cached.
- Background pictures with blur, dim, tint, vignette, scanlines and grain.
- Compact or comfortable density, zoom, and a sidebar you can reorder.

See [docs/theming.md](docs/theming.md).

<!-- TODO: side-by-side of 2–3 themes, or the theme editor with the live preview. -->
<img src="docs/images/themes.png" alt="Theme editor" width="800">

### More than the website

The full list of things Bonsai does that roblox.com doesn't is in [docs/features.md](docs/features.md).
What the website does that Bonsai doesn't yet is in [docs/gaps.md](docs/gaps.md).

## Safety and privacy

Bonsai holds sign-in cookies for your accounts, so it's built to keep them safe:

- **Cookies live in your OS keychain** (Windows Credential Manager, macOS Keychain, Secret Service on
  Linux). They're never written to disk in plain text, logged, or sent to the UI.
- **Only the Rust backend talks to the network**, and only to Roblox (plus Rolimons and Google Fonts if
  you turn them on). Roblox credentials are never sent to any other host. The UI can't make requests
  at all; its content security policy blocks it.
- **No automation.** Bonsai never buys, trades, messages or friends anyone on its own. Every write is the
  result of a click or keypress. Background watchers only notify you.
- **No captcha solving.** When Roblox asks for a challenge, Bonsai shows it to you.
- **Plugins run sandboxed** and never see your cookies. See [Plugins](#plugins).

Bonsai is open source under the GPL, so you can check all of this yourself.

## Install

Download the latest installer from [Releases](https://github.com/6E6B/bonsai/releases):

| Platform | Files |
|---|---|
| Windows | `.exe` (NSIS) or `.msi` |
| macOS | universal `.dmg` (Apple Silicon and Intel) |
| Linux | `.deb`, `.rpm` or `.AppImage` |

> [!NOTE]
> Builds aren't code-signed yet. On Windows, SmartScreen may warn you: click **More info → Run anyway**.
> On macOS, right-click the app and choose **Open** the first time, or run
> `xattr -dr com.apple.quarantine /Applications/Bonsai.app`.

### Linux notes

- On Wayland with the NVIDIA driver, Bonsai sets `__NV_DISABLE_EXPLICIT_SYNC=1` for you to avoid a
  WebKitGTK crash.
- To launch games through Sober, set the launch command in **Settings → General** (or run
  `app.player_command` from the command bar), for example `flatpak run org.vinegarhq.Sober {uri}`.

## Getting started

1. Open Bonsai and add an account. Quick sign-in is the easiest: Bonsai shows a code, you approve it in
   the Roblox app or on roblox.com.
2. Add more accounts the same way. They show up in the rail on the left.
3. Press `Ctrl+K` and start typing. Everything is in there.
4. Press `?` to see every keyboard shortcut.

## Keyboard shortcuts

The most used ones. Press `?` in the app for the full list, and change any of them in
**Settings → Shortcuts**.

| Shortcut | Action |
|---|---|
| `Ctrl+K` or `/` | Command bar |
| `?` | Show all shortcuts |
| `Ctrl+1` … `Ctrl+9` | Switch to account 1–9 |
| `Ctrl+Tab` | Switch to the last account |
| `g h` / `g d` / `g a` / `g m` | Home / Discover / Avatar / Marketplace |
| `g i` / `g t` / `g v` / `g r` | Inventory / Trades / Values / Transactions |
| `g n` / `g c` / `g f` / `g p` | Messages / Communities / Friends / Profile |
| `g e` | Themes |
| `Ctrl+,` | Settings |
| `Ctrl+T` / `Ctrl+W` | New tab / close tab |
| `Ctrl+Shift+T` | Reopen closed tab |
| `Alt+1` … `Alt+9` | Go to tab 1–8 / last tab |
| `Ctrl+PageDown` / `Ctrl+PageUp` | Next / previous tab |
| `j` / `k` / `Enter` | Move down / up / open |
| `[` / `]` | Previous / next sub-tab |
| `Ctrl+B` | Collapse the sidebar |
| `Ctrl+\` | Friends and chat drawer |
| `Ctrl+Shift+O` | Open the current page on roblox.com |
| `F5` | Refresh |
| `Ctrl+Shift+Backspace` | Stop all plugins |

## Plugins

Plugins add commands, alerts and small tools, written in [Luau](https://luau.org), the language Roblox
developers already know.

```toml
# plugin.toml
id = "trade-guard"
name = "Trade guard"
version = "0.1.0"
entry = "main.luau"
bonsai = 1

permissions = ["read.public", "read.account", "notify"]
actions = ["trade.decline"]
```

```lua
-- main.luau
bonsai.on("trade", function(ctx, _)
	for _, t in ipairs(ctx.trades { status = "inbound", limit = 25 }) do
		local u = ctx.user(t.partner.id)
		if u.banned then
			return { alert = { title = "Trade from a banned account", body = "@" .. u.name } }
		end
	end
end)
```

Plugins are safe to install from strangers:

- Each one runs in its own Luau sandbox with time and memory limits, no file system and no network.
- They never see cookies or tokens and can't make signed-in requests.
- They only get the permissions you grant, per account.
- Writes are graded by risk. Anything that runs without a click needs automation turned on for that
  account, and is rate-capped, budgeted (Robux and trades per day) and logged.
- Account control (passwords, security, sign-out) is never available to plugins.
- `Ctrl+Shift+Backspace` stops all of them at once.

Install one with **Install plugin** in the command bar, or try the examples in
[`examples/plugins/`](examples/plugins). The full API and security model is in
[docs/plugins.md](docs/plugins.md).

<!-- TODO: screenshot of Settings → Plugins with the permission review dialog. -->
<img src="docs/images/plugins.png" alt="Plugin permission review" width="800">

## Building from source

### Prerequisites

- [Rust](https://rustup.rs) (the version is pinned in `rust-toolchain.toml`; rustup picks it up)
- [bun](https://bun.sh)
- The [Tauri 2 prerequisites](https://tauri.app/start/prerequisites/) for your OS. On Debian or Ubuntu:

  ```sh
  sudo apt install build-essential pkg-config file xdg-utils patchelf \
    libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev \
    libxdo-dev libdbus-1-dev libssl-dev
  ```

### Run it

```sh
git clone --recursive https://github.com/6E6B/bonsai
cd bonsai/crates/bonsai/ui
bun install
bun run app        # desktop app with hot reload
```

### Other commands

From `crates/bonsai/ui/`:

| Command | What it does |
|---|---|
| `bun run app` | Desktop app with hot reload |
| `bun run dev` | Vite only, in a browser (backend calls fail there) |
| `bun run check` | Typecheck, lint, format check and tests |
| `bun run fix` | Apply lint and format fixes |
| `bun run tauri build` | Release build and installers |

From the repo root:

| Command | What it does |
|---|---|
| `cargo test --workspace` | Run the Rust tests |
| `cargo clippy --workspace --all-targets -- -D warnings` | Lint |
| `cargo fmt` | Format |
| `cargo xtask codegen` | Regenerate the Roblox API client from specs |

## Architecture

```
┌──────────────────────────────────────────────────────────────┐
│ SolidJS UI (webview)        no network, renders from stores  │
└──────────────▲──────────────────────────────┬────────────────┘
     storeChanged events,                     │ run(command, arg)
     thumb:// images                          │ typed IPC (tauri-specta)
┌──────────────┴──────────────────────────────▼────────────────┐
│ bonsai          Tauri 2 shell: IPC, tray, thumbnail protocol │
├──────────────────────────────────────────────────────────────┤
│ bonsai-core     accounts, store, cache, command registry,    │
│                 watchers, launcher, themes                   │
├─────────────────────────────┬────────────────────────────────┤
│ bonsai-plugin               │ bonsai-api                     │
│ Luau sandbox, manifests     │ the only crate on the network: │
│                             │ REST, realtime, rate limits    │
└─────────────────────────────┴────────────────────────────────┘
```

| Crate | Role |
|---|---|
| [`bonsai-api`](crates/bonsai-api) | Roblox API client. REST and realtime WebSocket, shared rate limiting, auth. Wrappers are generated from specs. |
| [`bonsai-core`](crates/bonsai-core) | Domain logic and the source of truth: accounts, store, disk cache, command registry, background watchers, launcher. |
| [`bonsai-plugin`](crates/bonsai-plugin) | Plugin manifests and the Luau sandbox. No network, no store access. |
| [`bonsai-codegen`](crates/bonsai-codegen) | Turns OpenAPI/Swagger specs into `bonsai-api` wrappers (`cargo xtask codegen`). |
| [`bonsai`](crates/bonsai) | Tauri 2 shell and the SolidJS app in [`ui/`](crates/bonsai/ui). No domain logic. |

API specs come from [Roblox-Api-Tracker](https://github.com/Paficent/Roblox-Api-Tracker) (a submodule in `vendor/`). Endpoints Roblox doesn't document are
specced by hand in [`specs/undocumented/`](specs/undocumented).

A few design choices:

- **Every action is a command.** Buttons, menus, shortcuts and plugins all dispatch through one registry
  in `bonsai-core`, which is why the command bar can reach everything.
- **Everything is per account.** Every request, cache row and piece of UI state carries an account ID.
  Private data is never shared between accounts.
- **Cache first.** The UI renders from a local mirror of the store and updates when the backend says
  something changed. It never waits on the network to show a screen.

## Contributing

Issues and pull requests are welcome. Before you start:

- Read [`AGENTS.md`](AGENTS.md). It's the rulebook for the codebase (for people and coding agents alike).
- Check [docs/gaps.md](docs/gaps.md) for what's left to build.
- Keep PRs small: one screen or one subsystem.
- Run `bun run check` and the Rust checks above before you push.

CI runs on every PR: Rust tests on Linux, Windows and macOS; `fmt`, `clippy` and a check that generated
code is up to date on Linux; and the UI checks when `crates/bonsai/ui/` changes.

Releases are built by [`.github/workflows/release.yml`](.github/workflows/release.yml). Pushing a tag
that matches the workspace version (`git tag v0.1.0 && git push origin v0.1.0`) creates a draft
GitHub release with installers for all three platforms.

## Disclaimer

Bonsai is an unofficial, community-made client. It is not affiliated with, endorsed by, or sponsored by
Roblox Corporation. "Roblox" is a trademark of Roblox Corporation. Use it in line with the
[Roblox Terms of Use](https://en.help.roblox.com/hc/en-us/articles/115004647846).

## License

[GPL-3.0-only](LICENSE).

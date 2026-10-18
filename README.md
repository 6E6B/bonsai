<div align="center">

<img src="crates/bonsai/icons/128x128.png" width="80" alt="">

# Bonsai

Desktop client for Roblox with support for multiple signed-in accounts.

[Releases](https://github.com/6E6B/bonsai/releases) · Windows, macOS, Linux

</div>

![Bonsai home screen](docs/images/hero.png)

## Features

### Accounts

- Several accounts signed in at once. `Ctrl+1`–`Ctrl+9` switches between them.
- Join a game, buy an item or send a friend request as another account without switching.
- Item, community and inventory pages show which of your accounts own the item, are members or hold a role.

### Command bar

- `Ctrl+K` opens a command bar. Every action in the app is a command there.
- Lists use `j`/`k`/`Enter`. Screens have `g` shortcuts, e.g. `g t` for Trades. `?` lists all shortcuts.
- Shortcuts can be changed in Settings → Shortcuts.

![Command bar](docs/images/command-bar.png)

### Trades

- Each trade shows RAP and value per side, Robux after the fee, and net gain.
- Warnings for lopsided trades, projected items, on-hold items, and item or Robux limits.
- Inbound trades can be accepted, declined or countered with `a`, `d` and `c`.
- Item values come from Rolimons. Off by default.

![Trade screen](docs/images/trades.png)

### Other

- Server list with best, smallest and random server joins.
- Resale price history charts and a wishlist that tracks price drops.
- Robux transaction ledger with date ranges, per-account totals and spending breakdowns.
- Chat drafts are kept per chat across restarts.
- CSV export for trades, community members and transactions.

[docs/features.md](docs/features.md) lists everything Bonsai adds over the website.
[docs/gaps.md](docs/gaps.md) lists what the website does that Bonsai doesn't yet.

### Themes and plugins

- Themes are made from a few seed colors in the theme editor and can be shared as a code or file. See [docs/theming.md](docs/theming.md).
- Plugins are written in Luau and run in a sandbox. See [docs/plugins.md](docs/plugins.md).

![Theme editor](docs/images/themes.png)

## Security

- `.ROBLOSECURITY` cookies are stored in the OS keychain. They are not saved in plain text, logged, or sent to the UI.
- Network requests are made only by the Rust backend. Cookies are sent only to Roblox. The UI can't make network requests.
- Bonsai makes no purchases, trades, messages or friend requests on its own. Every write comes from a click or keypress.
- Plugins can't read cookies and need permission per account. See [docs/plugins.md](docs/plugins.md).

## Install

Installers are on the [Releases](https://github.com/6E6B/bonsai/releases) page:

| OS | Files |
|---|---|
| Windows | `.exe`, `.msi` |
| macOS | `.dmg` |
| Linux | `.deb`, `.rpm`, `.AppImage` |

Builds are not code-signed. On Windows, SmartScreen asks for confirmation (More info → Run anyway).
On macOS, right-click the app and choose Open the first time.

To launch games through [Sober](https://sober.vinegarhq.org) on Linux, set the launch command in Settings → General.

## Building

Requires [Rust](https://rustup.rs), [bun](https://bun.sh) and the
[Tauri prerequisites](https://tauri.app/start/prerequisites/) for your OS.

```sh
git clone --recursive https://github.com/6E6B/bonsai
cd bonsai/crates/bonsai/ui
bun install
bun run app
```

See [AGENTS.md](AGENTS.md) for the code layout and contribution rules.

## License

[GPL-3.0](LICENSE). Not affiliated with Roblox Corporation.

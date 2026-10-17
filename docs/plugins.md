# Plugins

Plugins let people add commands, alerts and small tools to Bonsai without forking it, and let
users install them from strangers without risking their accounts. This is the design and the
roadmap. What's still missing is in [`gaps.md`](gaps.md).

Bonsai holds `.ROBLOSECURITY` cookies for several accounts, and cookie-stealing "tools" are
common in the Roblox world. The sandbox comes first; the API is what's safe to put inside it.

## Principles

- **Plugins never see secrets.** No cookies, CSRF tokens or auth tickets, and no way to make a
  signed-in request. A plugin only calls host functions, which go through `bonsai-core`.
- **Plugins never write.** A handler returns a list of actions, like
  `{ { id = "user.unfriend", arg = "123" } }`. The host runs them, and if any is a write it first
  shows a dialog listing every action. Plugin code has no way to perform a write itself, so the
  rule in `AGENTS.md` holds even for a malicious plugin.
- **No plugin code in the webview.** No plugin HTML or JS in the app's origin, where it could
  reach `invoke()` or the DOM.
- **No ambient authority.** A plugin gets nothing it didn't ask for in its manifest and the user
  didn't grant.
- **Background code only notifies.** Event handlers return alerts. An alert may carry a button
  bound to a command; the user still clicks it.
- **Everything is a command.** Plugin commands are registry entries and show up in the command
  bar. Installing, enabling and removing plugins are registry commands too.

## Runtime: Luau

Plugins are written in [Luau](https://luau.org), embedded with `mlua` (`luau` feature).

- Luau was built to run untrusted scripts: it has a sandbox mode and interrupts for time limits,
  and `mlua` caps memory.
- The people who'd write Bonsai plugins already write Luau for Roblox.

Each plugin gets its own VM on a plugin worker thread, with:

- a time limit per call (about 50 ms for a command handler, more for event handlers that yield);
- a memory cap (about 32 MB);
- no `io`, `os`, `debug`, file-based `require`, `getfenv`/`setfenv`, or bytecode loading.
  `require` only resolves modules inside the plugin's own folder.

Host functions that wait on data yield the Luau coroutine, so a handler reads like plain code.

## The manifest

```toml
# plugin.toml
id = "friend-cleanup"          # lowercase letters, digits, dashes; also the folder name
name = "Friend cleanup"
version = "1.0.0"
author = "nick"
entry = "main.luau"
bonsai = 1                     # plugin API version; newer than the app's is refused

permissions = [
  "read.public",
  "read.account",
  "notify",
]
actions = ["user.unfriend", "user.decline_friend"]   # built-in commands it may propose
```

Reading is tolerant like themes: unknown keys are ignored, an unknown permission is shown to the
user and refused, and a missing `id`, `entry` or a newer `bonsai` refuse the plugin.

## Permissions

| Permission | Grants |
|---|---|
| `read.public` | Users, games, items, groups. Shared across accounts. |
| `read.account` | Friends, presence, inventory, trades, messages. Granted **per account**; a plugin only sees the accounts the user ticked. |
| `actions` | The built-in commands the plugin may propose. Anything not listed is refused. |
| `notify` | Desktop alerts through `alerts.rs`. |
| `clipboard.write` | Copy text. |
| `net` | HTTPS to listed hosts. Not built yet; see [Network](#8-network). |

The user sees the permissions at install. An update that asks for more needs the user to agree
again. Never grantable: secrets, signed-in requests, files, processes, other plugins' storage.

## The API

```lua
bonsai.command {
  id = "unfriend-inactive",
  title = "Unfriend people offline more than 6 months",
  keywords = { "friends", "cleanup", "inactive" },
  arg = "none",                 -- none | text | user | game | item | account
  run = function(ctx, arg)
    local out = {}
    for _, f in ctx.friends(ctx.account) do
      if f.last_online and ctx.now - f.last_online > 180 * 86400 then
        table.insert(out, { id = "user.unfriend", arg = tostring(f.id) })
      end
    end
    return { actions = out }
  end,
}

bonsai.on("presence", function(ctx, change)
  if change.user == 123 and change.kind == "in_game" then
    return { alert = { title = "Friend joined a game", body = change.game_name,
                       command = { id = "game.join", arg = tostring(change.place_id) } } }
  end
end)
```

A command handler returns one of:

- `{ actions = { ... } }`: built-in commands to run, confirmed if any writes;
- `{ toast = "..." }`;
- `{ open = { kind = "user", id = 1 } }`: open a screen.

### Contribution points

| Point | Host side |
|---|---|
| `bonsai.command` | Registry entry `plugin.<plugin>.<id>`. No default shortcuts; the user binds them. |
| `context = "user"` etc. on a command | Shown in menus where `commandContext()` matches. |
| `bonsai.on(event, fn)` | Presence, friend request, trade received, message, game update. Returns alerts only. |
| `settings` in the manifest | Shown under Settings > Plugins > name, with `PrefKind`-style kinds. |
| `bonsai.store.get/set` | Key-value storage per plugin and **per account**, capped in size. |

### Reads

`ctx.user(id)`, `ctx.game(id)`, `ctx.item(id)`, `ctx.presence(account, ids)`,
`ctx.friends(account)`, `ctx.inventory(account, filter)`, `ctx.trades(account)`.
They read through the cache and the shared rate limiter, like the rest of the app. Private reads
need `read.account` for that account.

## Layout

- `crates/bonsai-plugin`: manifest parsing, the Luau sandbox and a `Host` trait. No network and
  no store. Tests run fixture plugins against a fake `Host`.
- `bonsai-core/src/plugins.rs`: implements `Host` over the store and cache; install, enable and
  permission state; plugin commands in the registry.
- `bonsai`: IPC views (`PluginInfo`), nothing else.
- UI: `features/plugins` for Settings > Plugins and the permission and action dialogs.

Plugins live in `Paths::plugins_dir()`, one folder each, like themes.

## Roadmap

### 0. Groundwork

- Every command declares an effect: `Read`, `Local` or `Write`, in the `commands!` macro. The
  action dialog needs it, and it can replace the hand-kept list in `lib/commands/confirm.ts`.
- `CommandId` is a fixed enum. Add `CommandRef = Builtin(CommandId) | Plugin(PluginCmd)` and
  return both from `commands()`. Ids already cross IPC as strings.

### 1. Sandbox

- `bonsai-plugin` crate: manifest, VM setup, limits, `Host` trait.
- Tests that escapes fail (`os`, `io`, `debug`, `getfenv`, `load`, string metatable, `require`
  outside the folder), that time and memory limits trip, and that undeclared actions are refused.

### 2. Install and manage

- Settings > Plugins: list, enable, disable, permissions per account, remove.
- Install from a folder. "Load unpacked plugin" for development, with reload on change (the
  theme folder watcher is the model).
- Commands: `plugin.install`, `plugin.enable`, `plugin.disable`, `plugin.remove`,
  `plugin.reload`.

### 3. Commands and actions

- Plugin commands in the command bar and context menus.
- The action dialog: lists each proposed action with its title and argument; run all or cancel.
- Errors and timeouts show a toast naming the plugin.

### 4. Reads

- The read functions above over the store and cache, with permission checks.

### 5. Events, storage and settings

- Events to alerts, through `alerts.rs` and the user's notification settings.
- Per-account storage and manifest settings.

### 6. Plugin UI (later)

Not now. When it comes back: plugins return a tree of our components (row, column, table, stat,
input, button) and get events back, so they follow the theme and the keyboard. Raw HTML in an
iframe stays out unless that proves too narrow.

### 7. Packages

- `.bonsaiplugin` files (a zip) with a content hash. Install shows the manifest and permissions
  first.

### 8. Network

Most plugins people want (value lists, trade calculators, Discord pings) need outside data.

**Decide first:** `AGENTS.md` makes `bonsai-api` the only crate that touches the network. A `net`
permission would add a `PluginHttp` client there: HTTPS only, hosts listed in the manifest and
shown at install, never any cookies, under the shared rate limiter, with a response size cap.
That needs the rule amended.

### 9. Hub

Shared with the theme hub in [theming.md](theming.md#6-hub).

## Not doing

- **Writes from plugin code.** Plugins propose; the user confirms.
- **Signed-in requests or raw Roblox endpoints.** The read API widens as needed instead.
- **Auto-trading, sniping, auto-buying, auto-accepting.** Background code only notifies.
- **Plugin JS in the app's webview.**
- **Files, processes, or reading other plugins' data.**

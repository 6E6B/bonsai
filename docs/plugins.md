# Plugins

Plugins let people add commands, alerts, automation and small tools to Bonsai without forking it,
and let users install them from strangers without risking their accounts. This is the design and
the roadmap. What's still missing is in [`gaps.md`](gaps.md).

Bonsai holds `.ROBLOSECURITY` cookies for several accounts, and cookie-stealing "tools" are
common in the Roblox world. The sandbox comes first; the API is what's safe to put inside it.
A plugin that can't read a cookie can still do harm with the actions it's allowed, so writes are
graded by risk (see [Tiers](#tiers)).

## Principles

- **Plugins never see secrets.** No cookies, CSRF tokens or auth tickets, and no way to make a
  signed-in request. A plugin only calls host functions, which go through `bonsai-core`.
- **Writes are graded by risk.** Every write command has a tier. Low-risk writes can run
  unattended with a permission. Riskier ones also need automation turned on for that account.
  Account control is never available to plugins.
- **Automation is opt-in per account.** It's off by default. Turning it on for an alt never
  exposes the main account.
- **Every plugin write is logged, capped and stoppable.** An activity log, rate caps, budgets and
  a kill switch.
- **No plugin code in the webview.** No plugin HTML or JS in the app's origin, where it could
  reach `invoke()` or the DOM.
- **No ambient authority.** A plugin gets nothing it didn't ask for in its manifest and the user
  didn't grant.
- **Everything is a command.** Plugin commands are registry entries and show up in the command
  bar. Installing, enabling, granting and stopping plugins are registry commands too.

Bonsai's own code still follows `AGENTS.md`: no automated writes, watchers only notify. Only
plugins, under the rules here, may write without a click.

## Runtime: Luau

Plugins are written in [Luau](https://luau.org), embedded with `mlua` (`luau` feature).

- Luau was built to run untrusted scripts: it has a sandbox mode and interrupts for time limits,
  and `mlua` caps memory.
- The people who'd write Bonsai plugins already write Luau for Roblox.

Each plugin gets its own VM on a plugin worker thread, with:

- a time limit per call (50 ms for a command handler, 250 ms for loading). Only time in Luau
  counts; time waiting on the host doesn't. Once tripped, the limit keeps failing until the call
  ends, so `pcall` can't swallow it;
- a memory cap (about 32 MB);
- no `io`, `os`, `debug`, file-based `require`, `getfenv`/`setfenv`, or bytecode loading.
  `require` only resolves modules inside the plugin's own folder.

Host functions that wait on data or a write yield the Luau coroutine, so a handler reads like
plain code.

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
actions = ["user.unfriend", "user.decline_friend"]   # built-in commands it may run or propose

[[settings]]                   # at most 50; shown under Settings > Plugins > name
key = "min_age"                # lowercase letters, digits and _, at most 64
title = "Decline accounts younger than"
description = "In days."
kind = "range"                 # bool | choice | range | text
min = 0                        # range: min, max (default 0..100), step (default 1)
max = 30
default = 7                    # else false, the first option, min or ""
# choice: options = ["quiet", { value = "loud", label = "Loud" }]
# text: placeholder = "..."
```

Reading is tolerant like themes: unknown keys are ignored, an unknown permission or action is
shown to the user and refused, and a missing `id`, `entry` or a newer `bonsai` refuse the plugin.
The install dialog shows each action under its tier.

## Permissions

| Permission | Grants |
|---|---|
| `read.public` | Users, games, items, groups. Shared across accounts. |
| `read.account` | Friends, presence, inventory, trades, messages. |
| `actions` | The built-in commands the plugin may run or propose, limited by [tier](#tiers). Anything not listed is refused. |
| `notify` | Desktop alerts through `alerts.rs`. |
| `clipboard.write` | Copy text. |
| `net` | HTTPS to listed hosts. Not built yet; see [Network](#9-network). |

Everything except `read.public` is granted **per account**: a plugin only sees and acts on the
accounts the user ticked. The user sees the permissions at install. An update that asks for more,
or for a higher tier, needs the user to agree again and shows what changed. Never grantable:
secrets, signed-in requests, files, processes, other plugins' storage.

## Tiers

Every write command in the registry declares a tier. The tier decides how a plugin may use it.

| Tier | Examples | Without automation | With automation on |
|---|---|---|---|
| **1. Harmless, reversible** | Wear item or outfit, favorite, follow a game, join a game, nickname, note | Runs unattended | Runs unattended |
| **2. Social, visible to others** | Friend requests, accept, decline, unfriend, block, follow users, messages, chat | User confirms each batch | Runs unattended, under a rate cap |
| **3. Value can be lost** | Buy, send or accept trades, group payouts, private servers | User confirms each batch | Runs unattended within a budget the user sets; over budget, user confirms |
| **4. Account control** | Password, two-step, sign out, cookie, PIN, email, privacy, removing the account | Never | Never |

Every built-in command declares its effect in the `commands!` macro in `commands.rs`: `Read`
(opens, shows or copies), `Local` (this app's settings, themes, layout, tabs, account labels) or
`Write(tier)`. Per-account data kept on this computer, like nicknames, notes, pinned chats and the
wishlist, counts as a write. A manifest may list only tier 1 to 3 writes in `actions`; `Read` and
`Local` commands are refused there, since plugins have reads and `open` for the first, and the
second includes launching programs and reading files. Display name is tier 2.

"User confirms" means the action dialog: it lists every proposed action with its account, title
and argument, and the user runs all or cancels.

### Automation toggle

Settings > Plugins > **Allow automation**, one switch per account, off by default. Also the
commands `plugins.allow_automation` and `plugins.block_automation`.

- Turning it on opens a dialog that names the account and says plainly that Roblox bans
  accounts for bots and automated trading. The user confirms by typing the account's name.
- While it's off, a plugin's tier 2 and 3 grants on that account are shown as "Needs automation"
  and act as tier "user confirms".
- Turning it off takes effect immediately, including for writes queued but not yet sent.

### Limits

- **Rate caps** per plugin, per account, per tier 2 action (e.g. 20 friend requests an hour). The
  plugin can't raise them; the user can lower them. They sit on top of the shared rate limiter.
- **Budgets** for tier 3, per plugin and account: Robux per day, and trades per day. Set in the
  grant dialog, which is its own screen, never a checkbox in a list.
- **Activity log**: every plugin write, with plugin, account, command, argument, time and result.
  Settings > Plugins > Activity, and `plugins.activity` in the command bar.
- **Kill switch**: a top bar indicator while any plugin has written in the last few minutes. The
  `plugins.stop_all` command (with a default shortcut) disables every plugin at once and drops
  queued writes.

## The API

```lua
bonsai.command {
  id = "unfriend-banned",
  title = "Unfriend banned accounts",
  keywords = { "friends", "cleanup", "banned" },
  arg = "none",                 -- none | text | user | game | item | account
  run = function(ctx, arg)
    local out = {}
    for _, f in ctx.friends() do
      if ctx.user(f.id).banned then
        table.insert(out, { id = "user.unfriend", arg = tostring(f.id) })
      end
    end
    return { actions = out }    -- shown to the user to confirm
  end,
}

bonsai.on("friend_request", function(ctx, e)
  for _, req in ctx.friend_requests{ limit = e.new } do
    local user = ctx.user(req.id)
    if user.created and ctx.now - user.created < ctx.settings.min_age * 86400 then
      -- Tier 2: runs unattended only if automation is on for ctx.account and the plugin is
      -- granted user.decline_friend there. Otherwise it raises an error the plugin can catch.
      ctx.run("user.decline_friend", tostring(req.id))
    end
  end
end)
```

- `ctx.run(id, arg)` performs a write now, as `ctx.account`. It fails with a clear error if the
  action isn't granted, its tier needs automation that's off, or a cap or budget is reached. It
  never falls back to asking the user.
- A handler can instead return:
  - `{ actions = { ... } }`: built-in commands to run, confirmed if any need it;
  - `{ toast = "..." }`;
  - `{ open = { kind = "user", id = 1 } }`: open a screen;
  - `{ alert = { title, body, command } }`: a desktop alert, optionally with a button bound to a
    command.

### Contribution points

| Point | Host side |
|---|---|
| `bonsai.command` | Registry entry `plugin.<plugin>.<id>`. No default shortcuts; the user binds them. |
| `context = "user"` etc. on a command | Shown in menus where `commandContext()` matches. |
| `bonsai.on(event, fn)` | See [Events](#events). Runs per granted account. |
| `bonsai.every(seconds, fn)` | A timer, run once per granted account each time. |
| `[[settings]]` in the manifest | Shown under Settings > Plugins > name; read with `ctx.settings`. |
| `ctx.store.get/set` | Key-value storage per plugin and **per account**, capped in size. |

### Events

`bonsai.on(event, fn)` and `bonsai.every(seconds, fn)` only work while `main.luau` loads. Events
need `read.account`, and each plugin listens to an event once. `fn(ctx, payload)` runs as each
granted account it happened to; a plugin with no free queue slot misses it.

| Event | When | Payload |
|---|---|---|
| `presence` | A friends' presence refresh saw statuses change | a list of `id, status, was, place_id, universe_id, location` |
| `friend_request` | The unread count went up | `new, total` |
| `trade` | The inbound trade count went up | `new, total` |
| `message` | The unread message count went up | `new, total` |
| `chat` | A chat message from someone else arrived | `from, conversation` |
| `game_update` | A new experience update in the notification stream | `title, body` |

- Timers take whole seconds from 60 to 86400, at most 10 per plugin. The first run is one
  interval after the plugin starts. Each run calls `fn(ctx)` once per granted, known account, or
  once as account 0 with none.
- From an event or timer, `alert` is a desktop alert (kind "Plugin alerts" in Settings >
  Notifications) when the plugin has `notify` and the account; otherwise a toast. `open` is
  refused. Actions run by themselves when every one may run unattended (tier 1, or tier 2 with
  automation on and under the cap); otherwise the batch waits in the action dialog.

### Storage and settings

- `ctx.store.get(key)` and `ctx.store.set(key, value)` keep JSON values (nil, booleans,
  numbers, strings, lists and string-keyed tables, 16 deep) for `ctx.account`. Keys are 1 to 128
  bytes; setting nil deletes. Account 0 is the plugin's own storage. At most 100 sets a call and
  256 KB per account; a full store raises an error. Kept in `config/plugin-data/<plugin>/<account>.json`
  and deleted with the plugin.
- `ctx.settings.<key>` is each manifest setting's value: a boolean, an option value, a number or
  text. It is read-only. The user changes it in Settings > Plugins or with `plugin.set`
  ("plugin key value"; no value resets it). Values survive updates; an invalid one falls back to
  the default.

### Reads

`ctx.user(id)`, `ctx.game(id)`, `ctx.item(id)`, `ctx.presence(ids)`, `ctx.friends()`,
`ctx.inventory(filter)`, `ctx.trades()`, `ctx.friend_requests()`. They read through the cache and the shared rate limiter,
like the rest of the app. Private reads need `read.account` for `ctx.account`.

| Read | Takes | Returns |
|---|---|---|
| `ctx.user(id)` | user id | `id, name, display_name, description, created, verified, banned, friends, followers, following` |
| `ctx.game(id)` | place id | `universe_id, place_id, name, description, ...`, or nil if there is none |
| `ctx.item(id, kind)` | asset id, `kind` `"asset"` (default) or `"bundle"` | the item's catalog details |
| `ctx.presence(ids)` | up to 100 user ids | a list of `id, status, place_id, universe_id, location` |
| `ctx.friends()` | | a list of `id, name, display_name, status` |
| `ctx.inventory{user, asset_type, bundle_type, limit}` | all optional; `user` defaults to `ctx.account`, no type means collectibles; `limit` 100, at most 1000 | a list of items |
| `ctx.trades{status, limit}` | `status` `inbound` (default), `outbound`, `completed` or `inactive`; `limit` 25, at most 100 | a list of trades |
| `ctx.friend_requests{limit}` | `limit` 50, at most 500 | a list of `id, name, display_name, sent, mutuals, origin` |

- Fields are snake_case. Dates are unix seconds, like `ctx.now`. Unknown counts are nil.
  `status` is `offline`, `online`, `in_game`, `in_studio` or `invisible`.
- `read.public` covers user, game and item; `read.account` the rest, and only for an account
  the plugin was granted. Ids are whole numbers or numeric strings.
- A call makes at most 200 reads. Each waits up to 15 s for the network, off the plugin's clock.
  A read that fails raises an error the plugin can catch with `pcall`.

## Layout

- `crates/bonsai-plugin`: manifest parsing, the Luau sandbox and a `Host` trait. No network and
  no store. Tests run fixture plugins against a fake `Host`.
- `bonsai-core/src/plugins.rs`: implements `Host` over the store and cache; install, enable,
  permission, automation and budget state; the activity log; plugin commands in the registry.
  Tier checks live here, not in the sandbox crate.
- `bonsai`: IPC views (`PluginInfo`, `PluginActivity`), nothing else.
- UI: `features/plugins` for the install/review, automation and action dialogs and the activity
  log; `features/settings/plugins.tsx` for Settings > Plugins; `lib/plugins.ts` for tier labels.

Plugins live in `Paths::plugins_dir()`, one folder each, like themes.

## Roadmap

### 0. Groundwork (done)

- Every command declares an effect in the `commands!` macro: `Read`, `Local` or `Write(tier)`.
  Tests pin account control to tier 4 and the main value and social writes to their tiers.
  `lib/commands/confirm.ts` still holds the dialog text for built-in confirms; the effect isn't
  sent to the UI until the action dialog needs it.
- `CommandRef = Builtin(CommandId) | Plugin(PluginCmd)`. `Registry` holds plugin commands next to
  the built-in ones and `commands()` returns both. Plugin commands can't be rebound yet:
  overrides are still keyed by `CommandId`.

### 1. Sandbox (done)

- `bonsai-plugin` crate: manifest, VM setup, limits, `Host` trait.
- Tests that escapes fail (`os`, `io`, `debug`, `getfenv`, `load`, string metatable, `require`
  outside the folder), that time and memory limits trip, and that undeclared actions are refused.
- Built: `bonsai.command`, the `ctx` table (`account`, `now`, `run`), returned outcomes, `print`
  to the host log, `require` inside the folder (symlinks out are refused, bytecode isn't
  loaded). Reads came in phase 4; events, timers, storage and settings in phase 5. Not yet:
  yielding host calls; host calls block the plugin's thread for now.
- Errors raised by host functions reach Luau as error objects; plugins `tostring` them.

### 2. Install and manage (done)

- Settings > Plugins: list, enable, disable, permissions per account, remove.
- Install from a folder. "Load unpacked plugin" for development, with reload on change (the
  theme folder watcher is the model).
- Commands: `plugin.install`, `plugin.enable`, `plugin.disable`, `plugin.remove`,
  `plugin.reload`.
- Built: also `plugin.load_unpacked`, `plugin.review`, `plugin.grant` and `plugin.revoke` (the
  last two act on the current account). Install copies the folder (no dot files or symlinks,
  500 files, 20 MB) through a temp folder; unpacked plugins load in place and are only
  forgotten on remove. A 2 s watcher hashes each folder and restarts a plugin whose files
  changed. Each running plugin has its own thread.
- The install dialog opens from `plugin_install` in the store, like theme import. It lists
  permissions, actions by tier and what was refused (non-writes, tier 4, unknown keys) before
  anything is copied.
- `config/plugins.json` holds, per plugin, on/off, the granted accounts and the permissions and
  actions the user agreed to. A manifest that asks for more waits as "Needs review" and doesn't
  start; turning it on opens the dialog listing only what's new.

### 3. Commands and confirmed actions (done)

- Plugin commands in the command bar and context menus.
- The action dialog. Errors and timeouts show a toast naming the plugin.
- Built: running plugins' commands are listed as "Plugin name: title", with the plugin's name in
  the keywords. Context menus for users, games, items and communities have a Plugins submenu for
  commands with that `context`; the arg is the page's id (a place id for games). A command run
  with no arg takes it from the page context.
- `ctx.account` is the acting account only if the plugin was granted it, else 0. Each plugin
  queues at most 8 calls; past that the command toasts that the plugin is busy.
- Outcomes: `toast` and `alert` show a toast naming the plugin (from events and timers, an
  alert can be a desktop alert; see [Events](#events)); `open` runs the built-in open command for that kind; `actions` are
  checked (at most 100, account granted, each action agreed to and tier 1 to 3). All tier 1
  runs at once; anything higher opens the action dialog, which lists every action with its tier.
  Confirming checks the batch again, then runs it through the same dispatch as built-in
  commands and toasts how many ran.
- `ctx.run` came in phase 6. Not yet: plugin commands can't be rebound.

### 4. Reads (done)

- The read functions above over the store and cache, with permission checks.
- Built: see [Reads](#reads). Each read checks, in bonsai-core, that the plugin is running, the
  permission was agreed to and, for private reads, the account is granted and signed in. Users,
  items, inventories and trades use the same cached resources as their screens. Games use the
  store, else load the place. Friends and friends' presence come from the store; presence for
  anyone else is fetched in chunks of 50 and not stored, so it never fires friend-join alerts.

### 5. Events, timers, storage and settings (done)

- Events and timers per granted account. Alerts through `alerts.rs` and the user's notification
  settings.
- Per-account storage and manifest settings.
- Built: see [Events](#events) and [Storage and settings](#storage-and-settings). The watchers
  that raise Bonsai's own alerts (`note_unread`, `note_chat`, `note_stream`, presence refreshes)
  also send plugin events; the notification stream is read when a plugin listens to
  `game_update` even if its alert is off. Events are sent with `try_send` and never wait.
  Timers run on the plugin's thread between calls. Storage loads lazily on the plugin's thread
  and writes through a temp file. A new `notify.plugins` kind (on by default) gates plugin
  alerts. Settings > Plugins lists what each plugin listens to, its timers and its settings
  with the same controls as Bonsai's own.

### 6. Automation (done)

- `ctx.run` with tier checks, the per-account automation toggle, rate caps, the activity log and
  the kill switch. Tier 1 and 2 only.
- Tests for every refusal: not granted, automation off, cap reached, tier 4, plugin stopped.
- Built: `ctx.run` runs tier 1, and tier 2 when automation is on and under the cap; anything
  else fails with an error saying why. One call can make at most 50 `ctx.run` calls. Actions
  returned from events and timers run by themselves under the same rules; anything that can't
  opens the action dialog. Each unattended run checks that the plugin is running, that
  automation is still on and that `plugins.stop_all` hasn't run since the call started, so
  turning either off drops queued writes. The toggle is per account in `config/plugins.json`.
  `plugins.allow_automation` opens a dialog where the user types the account's name. Caps
  count successful unattended runs in the last hour; `plugin.cap` lowers one (Settings >
  Plugins has a slider per tier 2 action). `config/plugin-activity.jsonl` logs every write,
  confirmed or not, keeping the last hour plus 500 older entries; a refusal repeated within a
  minute is logged once. The top bar shows a stop button for five minutes after a write.
  `plugins.stop_all` (Ctrl+Shift+Backspace) turns every plugin off.

### 7. Budgets

- Tier 3 unattended within budgets, after the activity log and kill switch have shipped.

### 8. Plugin UI (later)

Not now. When it comes back: plugins return a tree of our components (row, column, table, stat,
input, button) and get events back, so they follow the theme and the keyboard. Raw HTML in an
iframe stays out unless that proves too narrow.

### 9. Network

Most plugins people want (value lists, trade calculators, Discord pings) need outside data.

**Decide first:** `AGENTS.md` makes `bonsai-api` the only crate that touches the network. A `net`
permission would add a `PluginHttp` client there: HTTPS only, hosts listed in the manifest and
shown at install, never any cookies, under the shared rate limiter, with a response size cap.
That needs the rule amended.

### 10. Packages and hub

- `.bonsaiplugin` files (a zip) with a content hash. Install shows the manifest and permissions
  first.
- The hub is shared with the theme hub in [theming.md](theming.md#6-hub).

## Not doing

- **Tier 4 for plugins.** No plugin changes passwords, two-step, sessions or cookies.
- **Signed-in requests or raw Roblox endpoints.** The read API and the registry widen as needed
  instead.
- **One switch for everything.** Automation is per account; grants are per plugin and account.
- **Solving captchas or challenges.** A write that hits one fails and alerts the user.
- **Plugin JS in the app's webview.**
- **Files, processes, or reading other plugins' data.**

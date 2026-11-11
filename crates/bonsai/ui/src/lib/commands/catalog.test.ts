import { describe, expect, it } from "vitest"
import { type History, openable, seenBoost, touch } from "~/lib/history"
import type { CommandInfo, RemoteRow } from "~/lib/ipc/bindings"
import { fold } from "~/lib/search"
import catalog from "./catalog.json"
import { intentRows, remoteTail } from "./intent"
import { booster, emptyUsage, record, type Usage } from "./learn"
import {
  flatRows,
  type PalettePerson,
  type PalettePref,
  paletteEntries,
  paletteSections,
  parseScope,
  remoteEntries,
  remoteQuery,
  type Scope,
  seenEntry,
} from "./palette"

/** What people type into the command bar, and what should come first. Runs against the real
 * command and settings lists (`catalog.json`, written by `cargo test -p bonsai`). */

const commands: CommandInfo[] = catalog.commands.map((c) => ({
  ...c,
  arg: c.arg as CommandInfo["arg"],
  shortcuts: [],
  rebound: false,
  context: null,
}))
const prefs = catalog.prefs as PalettePref[]

const accounts = [
  { id: 1, name: "mainacc", displayName: "Main", label: "Main" },
  { id: 2, name: "altguy", displayName: "Alt", label: "Alt 2" },
]
const themes = [
  { id: "dark", name: "Dark", builtin: true },
  { id: "light", name: "Light", builtin: true },
  { id: "sakura", name: "Sakura", builtin: false },
]
const people: PalettePerson[] = [
  { id: 101, name: "PlayerOne", displayName: "PlayerOne", inGame: true, status: "Tower of Hell" },
  { id: 102, name: "builderman", displayName: "Builder", inGame: false, status: "Online" },
  { id: 103, name: "StickMasterLuke", displayName: "Luke", inGame: true, status: "Natural Disaster Survival" },
  { id: 104, name: "xX_Gamer_Xx", displayName: "Gamer", nickname: "Sam", inGame: false, status: "Offline" },
]

const experience = (id: number, title: string): RemoteRow => ({
  group: "Experiences",
  title,
  subtitle: "",
  command: "game.open",
  arg: String(id),
  kind: "experience",
  id,
  universeId: id + 1,
})
const user = (id: number, name: string): RemoteRow => ({
  group: "People",
  title: name,
  subtitle: `@${name}`,
  command: "user.open",
  arg: String(id),
  kind: "user",
  id,
  universeId: null,
})

/** Canned Roblox search results by query. */
const ROBLOX: Record<string, RemoteRow[]> = {
  "tower of hell": [experience(1962086868, "Tower of Hell"), experience(5, "Tower of Hell but easy")],
  tower: [experience(7, "Tower Defense Simulator"), experience(1962086868, "Tower of Hell")],
  bloxburg: [experience(185655149, "Welcome to Bloxburg")],
  doors: [experience(6516141723, "DOORS")],
  roblox: [user(1, "Roblox"), user(9, "RobloxFan")],
}

const entries = paletteEntries(commands, accounts, [], [], themes, prefs)

function results(query: string, usage: Usage = emptyUsage(), history: History = {}) {
  const parsed = parseScope(query) ?? { scope: "all" as Scope, text: query }
  const tail = parsed.scope === "all" ? remoteTail(entries, parsed.text) : null
  const q = remoteQuery(parsed.scope, tail ?? parsed.text)
  const remote = remoteEntries(ROBLOX[fold(q)] ?? [])
  const intents = intentRows({ commands: entries, text: parsed.text, people, remote, remoteFor: tail ?? undefined })
  return flatRows(
    paletteSections({
      scope: parsed.scope,
      text: parsed.text,
      commands: entries,
      friends: people,
      remote,
      intents,
      seen: openable(history).flatMap((h) => seenEntry(h) ?? []),
      boost: (key) => booster(usage, parsed.text)(key) + seenBoost(history, key),
    }),
  )
}

const top = (query: string, usage?: Usage, history?: History) => results(query, usage, history)[0]?.key

/** [query, the key that should come first, or a prefix ending in `*`]. */
const CASES: [string, string][] = [
  // Screens
  ["home", "nav.home"],
  ["settings", "nav.settings"],
  ["prefs", "nav.settings"],
  ["inv", "nav.inventory"],
  ["inventory", "nav.inventory"],
  ["trades", "nav.trades"],
  ["inbox", "nav.messages"],
  ["friends", "nav.friends"],
  ["my profile", "nav.profile"],
  ["marketplace", "nav.marketplace"],
  ["catalog", "nav.marketplace"],
  ["shop", "nav.marketplace"],
  ["chat", "nav.chat"],
  ["notifs", "nav.notifications"],
  ["values", "nav.values"],
  ["rolimons", "nav.values"],
  ["discover", "nav.discover"],
  ["themes", "nav.themes"],
  ["wiki", "nav.wiki"],
  ["playlists", "nav.playlists"],
  ["groups", "nav.communities"],
  ["transactions", "nav.transactions"],
  ["go to friends", "nav.friends"],
  // Typos
  ["setings", "nav.settings"],
  ["setttings", "nav.settings"],
  ["invnetory", "nav.inventory"],
  ["frinds", "nav.friends"],
  ["notifcations", "nav.notifications"],
  ["marketplac", "nav.marketplace"],
  ["transations", "nav.transactions"],
  ["swtich account", "account.switch:*"],
  ["sgin out", "account.sign_out"],
  ["zoom inn", "view.zoom_in"],
  // Initials
  ["gh", "nav.home"],
  ["gf", "nav.friends"],
  ["zi", "view.zoom_in"],
  ["lb", "vault.lock"],
  // Actions
  ["log out", "account.sign_out"],
  ["logout", "account.sign_out"],
  ["sign out", "account.sign_out"],
  ["quit", "app.quit"],
  ["exit", "app.quit"],
  ["lock", "vault.lock"],
  ["zoom in", "view.zoom_in"],
  ["bigger", "view.zoom_in"],
  ["new tab", "tab.new"],
  ["close tab", "tab.close"],
  ["reopen tab", "tab.reopen"],
  ["refresh", "view.refresh"],
  ["reload", "view.refresh"],
  ["keyboard shortcuts", "help.shortcuts"],
  ["hotkeys", "help.shortcuts"],
  ["add account", "account.add"],
  ["add alt", "account.add"],
  ["switch to alt", "account.switch:2"],
  ["switch main", "account.switch:1"],
  ["check for updates", "app.check_updates"],
  ["clear cache", "app.clear_cache"],
  ["2fa", "security.*"],
  ["change password", "security.change_password"],
  ["copy id", "entity.copy_id"],
  ["copy link", "entity.copy_link"],
  ["mark all read", "message.mark_all_read"],
  ["minimize", "window.minimize"],
  ["fullscreen", "window.maximize"],
  ["stop plugins", "plugins.stop_all"],
  ["install plugin", "plugin.install*"],
  ["emoji", "chat.emoji"],
  ["export settings", "settings.export"],
  ["discord", "help.discord"],
  ["best server", "game.join_best"],
  ["smallest server", "game.join_smallest"],
  ["r6", "avatar.rig"],
  ["wear outfit", "avatar.outfit"],
  ["sakura", "theme.apply:sakura"],
  ["use theme sakura", "theme.apply:sakura"],
  ["delete theme sakura", "theme.delete:sakura"],
  ["dark mode", "*:dark"],
  ["light theme", "*light"],
  ["reduce motion", "settings.set:motion*"],
  ["do not disturb", "settings.set:quiet_hours:on"],
  ["hide status bar", "settings.set:status_bar:off"],
  [">lock", "vault.lock"],
  // A command and its argument at once
  ["join playe", "intent:game.join:user:101"],
  ["join player", "intent:game.join:user:101"],
  ["join plyerone", "intent:game.join:user:101"],
  ["join luke", "intent:game.join:user:103"],
  ["play luke", "intent:game.join:user:103"],
  ["luke join", "intent:game.join:user:103"],
  ["message builder", "intent:user.message:102"],
  ["dm builder", "intent:user.message:102"],
  ["msg luke", "intent:user.message:103"],
  ["chat with sam", "intent:user.message:104"],
  ["trade builder", "intent:trade.new:102"],
  ["profile luke", "intent:user.open:103"],
  ["luke profile", "intent:user.open:103"],
  ["inventory luke", "intent:inventory.view:103"],
  ["add friend builderman", "intent:user.add_friend:102"],
  ["block builder", "intent:user.block:102"],
  ["unfriend builder", "intent:user.unfriend:102"],
  ["follow luke", "intent:user.follow:103"],
  ["friends of luke", "intent:user.friends:103"],
  ["profile roblox", "intent:user.open:1"],
  ["join tower of hell", "intent:game.join:1962086868"],
  ["join bloxburg", "intent:game.join:185655149"],
  ["open tower of hell", "intent:game.open:1962086868"],
  ["join 1962086868", "intent:game.join:1962086868"],
  ["search wiki doors", "intent:wiki.search:doors"],
  ["search marketplace sparkle", "intent:market.search:sparkle"],
  ["rename account bob", "intent:account.rename:bob"],
  // Plain searches stay plain
  ["builder", "user:102"],
  ["tower of hell", "remote:*"],
]

const fits = (key: string | undefined, want: string) => {
  if (!key) return false
  if (want.startsWith("*")) return key.endsWith(want.slice(1))
  if (want.endsWith("*")) return key.startsWith(want.slice(0, -1))
  return key === want
}

/** [query, a key that should be in the first three]: good enough when the top is a judgment
 * call. */
const NEAR: [string, string][] = [
  ["dm", "user.message"],
  ["msg", "user.message"],
  ["pm", "user.message"],
  ["join", "game.join"],
  ["theme", "nav.themes"],
  ["dark", "*:dark"],
  ["zoom", "view.zoom_in"],
  ["switch acc", "account.switch:*"],
  ["tab", "tab.new"],
  ["plugins", "nav.hub"],
  ["check upd", "app.check_updates"],
  ["sign", "account.sign_out"],
  ["tower", "*:1962086868"],
  ["roblox", "remote:People:user.open:1"],
]

const DAY = 86_400_000
const now = Date.now()
const tower = {
  ref: "game:1962086868",
  title: "Tower of Hell",
  open: { id: "game.open", arg: "1962086868" },
  entity: { kind: "experience" as const, id: 1962086868 },
}
/** Looked at Tower of Hell on three days this week, and RobloxFan's profile once a month ago. */
let looked: History = {}
for (const d of [5, 3, 1]) looked = touch(looked, tower, now - d * DAY)
looked = touch(
  looked,
  {
    ref: "user:9",
    title: "RobloxFan",
    open: { id: "user.open", arg: "9" },
    entity: { kind: "user", id: 9 },
  },
  now - 30 * DAY,
)
/** Picked Tower of Hell from Roblox's results after typing "tower", twice. */
let picked = emptyUsage()
for (const d of [4, 2]) {
  picked = record(picked, "remote:Experiences:game.open:1962086868", "tower", now - d * DAY)
  picked = record(picked, "game:1962086868", "tower", now - d * DAY)
}

/** [query, key that should come first] with that history. */
const REMEMBERED: [string, string][] = [
  // Found from the first letter, without Roblox
  ["t", "seen:game:1962086868"],
  ["to", "seen:game:1962086868"],
  ["tow", "seen:game:1962086868"],
  ["tower", "seen:game:1962086868"],
  ["hell", "seen:game:1962086868"],
  // A good command match still wins over something looked at
  ["trades", "nav.trades"],
  ["settings", "nav.settings"],
  // Friends still beat a stranger you once looked at
  ["luke", "user:103"],
]

/** Where `want` is in `got` (1 = first), or 0 when it's missing. */
const place = (got: string[], want: string) => got.findIndex((k) => fits(k, want)) + 1

describe("command bar ranking score", () => {
  it("stays good across every case", () => {
    const cases = [
      ...CASES.map(
        ([q, w]) =>
          [
            q,
            w,
            place(
              results(q).map((r) => r.key),
              w,
            ),
          ] as const,
      ),
      ...NEAR.map(
        ([q, w]) =>
          [
            q,
            w,
            place(
              results(q).map((r) => r.key),
              w,
            ),
          ] as const,
      ),
      ...REMEMBERED.map(
        ([q, w]) =>
          [
            q,
            w,
            place(
              results(q, picked, looked).map((r) => r.key),
              w,
            ),
          ] as const,
      ),
    ]
    const all = cases.map((c) => c[2])
    const first = all.filter((p) => p === 1).length / all.length
    const top3 = all.filter((p) => p >= 1 && p <= 3).length / all.length
    const mrr = all.reduce((s, p) => s + (p ? 1 / p : 0), 0) / all.length
    console.info(
      `command bar: ${all.length} cases, first ${(first * 100).toFixed(1)}%, top 3 ${(top3 * 100).toFixed(1)}%, MRR ${mrr.toFixed(3)}`,
    )
    const missed = cases.filter((c) => c[2] !== 1).map(([q, w, p]) => `${q} → ${w} at ${p || "none"}`)
    if (missed.length) console.info(`not first: ${missed.join("; ")}`)
    // Raise this when ranking gets better; a change that lowers it needs a reason.
    expect(mrr).toBeGreaterThanOrEqual(0.98)
  })
})

describe("command bar ranking", () => {
  it.each(CASES)("%s → %s", (query, want) => {
    const got = results(query)
      .slice(0, 3)
      .map((r) => `${r.key} (${r.title})`)
    expect(fits(top(query), want), `top results: ${got.join(", ")}`).toBe(true)
  })

  it.each(NEAR)("%s → %s in the first three", (query, want) => {
    const got = results(query).map((r) => r.key)
    const at = place(got, want)
    expect(at >= 1 && at <= 3, `top results: ${got.slice(0, 5).join(", ")}`).toBe(true)
  })

  it.each(REMEMBERED)("%s → %s after looking and picking", (query, want) => {
    const got = results(query, picked, looked).map((r) => r.key)
    expect(fits(got[0], want), `top results: ${got.slice(0, 5).join(", ")}`).toBe(true)
  })

  it("learns what you pick for what you type", () => {
    expect(top("j")).not.toBe("game.join_random")
    let u = emptyUsage()
    u = record(u, "game.join_random", "j")
    u = record(u, "game.join_random", "jo")
    expect(top("j", u)).toBe("game.join_random")
    expect(top("jo", u)).toBe("game.join_random")
  })

  it("learns inline rows too", () => {
    const u = record(emptyUsage(), "intent:user.message:103", "msg luke")
    expect(top("msg l", u)).toBe("intent:user.message:103")
  })
})

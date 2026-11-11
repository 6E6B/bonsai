import { entityRef, type Seen } from "~/lib/history"
import type { Arg, BookmarkView, CommandInfo, PrefView, Presence, RemoteKind, RemoteRow } from "~/lib/ipc/bindings"
import { fold } from "~/lib/search"
import { settingsSections } from "~/lib/sections"
import type { ThumbKind } from "~/lib/thumb"
import type { RecentArg } from "./learn"
import {
  matchTerms,
  type Prepared,
  prepare,
  prepared,
  type Ranked,
  rankScored,
  terms,
  tokens,
  wordScore,
} from "./match"

/** A row in the command bar. Picking it runs `id` with `arg`, or, when `ask` is set, asks for
 * the argument first. */
export interface PaletteEntry {
  key: string
  title: string
  words: string
  shortcut?: string
  id: string
  arg: string | null
  /** Asks for a typed argument of this kind before running. */
  ask?: Arg
  /** Muted text after the title (a search result's details). */
  subtitle?: string
  /** The result group a Roblox search row came from, like "People". */
  group?: string
  /** Picture shown before the title. */
  thumb?: { kind: ThumbKind; id: number }
  /** The person, experience, community or item the row opens. Its other actions are listed
   * with `entityActions`. */
  entity?: PaletteEntity
  /** What the row is about (`user:1`, `game:2`), when that isn't its `entity`: an inline row
   * that messages a friend. What you picked or looked at before is kept by this. */
  ref?: string
  /** Picking it types this into the box instead of running anything: a recent search. */
  fill?: string
  /** Shown because it was picked or looked at before, not because it matched. */
  recent?: boolean
}

/** What a row is about, for learning: the same person or game whichever way it was found. */
export const refOf = (e: PaletteEntry): string | undefined =>
  e.ref ?? (e.entity ? entityRef(e.entity.kind, e.entity.id) : undefined)

export interface PaletteEntity {
  kind: RemoteKind
  id: number
  /** Joinable: a friend in a game. */
  inGame?: boolean
}

export interface PalettePerson {
  id: number
  name: string
  displayName: string
  /** What the active account calls them in this app. */
  nickname?: string | null
  /** In a game, so `game.join` can follow them. */
  inGame?: boolean
  presence?: Presence
  /** The game's name in game, else the presence. */
  status?: string
}

export interface PaletteAccount {
  id: number
  name: string
  displayName: string
  label: string | null
}

export interface PaletteTab {
  key: string
  title: string
}

/** A screen that can sit in the sidebar, for `view.sidebar_*`. */
export interface PaletteScreen {
  key: string
  title: string
}

/** An installed theme, for `theme.*`. Built-ins can't be deleted, so `theme.delete` skips them. */
export interface PaletteTheme {
  id: string
  name: string
  builtin: boolean
}

/** An installed plugin, for `plugin.*`. */
export interface PalettePlugin {
  id: string
  name: string
  enabled: boolean
}

export interface PalettePlaylist {
  id: number
  name: string
}

/** A setting, for `settings.set` and `settings.reset`. */
export type PalettePref = Pick<PrefView, "key" | "section" | "title" | "keywords" | "kind" | "options">

const COLORS = ["none", "red", "orange", "yellow", "green", "blue", "purple", "pink"]
const MODES = ["system", "light", "dark"]

const titleCase = (s: string) => s.charAt(0).toUpperCase() + s.slice(1)

/** Rows for `settings.set`: on and off for switches, one per option for choices. Text and
 * number settings are changed on their settings page. */
function settingRows(c: CommandInfo, prefs: PalettePref[]): PaletteEntry[] {
  const out: PaletteEntry[] = []
  for (const p of prefs) {
    const section = settingsSections.find((s) => s.value === p.section)?.label ?? ""
    const words = `${c.keywords.join(" ")} ${p.keywords.join(" ")} ${section}`
    const row = (value: string, label: string) =>
      out.push({
        key: `${c.id}:${p.key}:${value}`,
        title: `${p.title}: ${label}`,
        words,
        id: c.id,
        arg: `${p.key} ${value}`,
      })
    if (p.kind === "bool") {
      row("on", "On")
      row("off", "Off")
    } else if (p.kind === "choice") for (const o of p.options) row(o.value, o.label)
  }
  return out
}

/** Typed-argument commands that also do something with no argument, and that row's title. */
const BARE: Record<string, string> = {
  "theme.import": "Import theme from a file",
  "plugin.install": "Install plugin from a folder",
  "plugin.load_unpacked": "Load unpacked plugin from a folder",
}

/** Rows for `settings.reset`: everything, then each section that has settings. */
function resetRows(c: CommandInfo, prefs: PalettePref[]): PaletteEntry[] {
  const words = c.keywords.join(" ")
  const row = (value: string, label: string) => ({
    key: `${c.id}:${value}`,
    title: `${c.title}: ${label}`,
    words,
    id: c.id,
    arg: value,
  })
  const sections = settingsSections.filter((s) => prefs.some((p) => p.section === s.value))
  return [row("all", "All"), ...sections.map((s) => row(s.value, s.label))]
}

/** Every command, with one row per choice for arguments that have a short fixed list
 * (accounts, settings sections, colors, modes, themes, tabs, sidebar screens, settings, plugins,
 * playlists) and an
 * asking row for typed arguments. */
export function paletteEntries(
  commands: CommandInfo[],
  accounts: PaletteAccount[],
  tabs: PaletteTab[] = [],
  screens: PaletteScreen[] = [],
  themes: PaletteTheme[] = [],
  prefs: PalettePref[] = [],
  plugins: PalettePlugin[] = [],
  playlists: PalettePlaylist[] = [],
  bookmarks: BookmarkView[] = [],
): PaletteEntry[] {
  const out: PaletteEntry[] = []
  for (const c of commands) {
    const base = {
      words: c.keywords.join(" "),
      shortcut: c.shortcuts[0]?.label,
      id: c.id,
    }
    const choice = (value: string, label: string, words = "") =>
      out.push({
        ...base,
        key: `${c.id}:${value}`,
        title: `${c.title}: ${label}`,
        arg: value,
        words: `${base.words} ${words}`,
      })
    switch (c.arg) {
      case "none":
        out.push({ ...base, key: c.id, title: c.title, arg: null })
        break
      case "account":
        for (const a of accounts) choice(String(a.id), a.label ?? a.displayName, a.name)
        break
      case "settingsSection":
        out.push({ ...base, key: c.id, title: c.title, arg: null })
        for (const s of settingsSections) choice(s.value, s.label)
        break
      case "color":
        for (const v of COLORS) choice(v, titleCase(v))
        break
      case "mode":
        for (const v of MODES) choice(v, titleCase(v))
        break
      case "theme":
        for (const t of themes) if (!(c.id === "theme.delete" && t.builtin)) choice(t.id, t.name)
        break
      case "plugin":
        for (const x of plugins) {
          if ((c.id === "plugin.enable" && x.enabled) || (c.id === "plugin.disable" && !x.enabled)) continue
          choice(x.id, x.name)
        }
        break
      case "bookmark":
        for (const b of bookmarks) choice(String(b.id), b.title)
        break
      case "playlist":
        for (const p of playlists) choice(String(p.id), p.name)
        break
      case "tab":
        for (const t of tabs) choice(t.key, t.title)
        break
      case "screen":
        for (const s of screens) choice(s.key, s.title)
        break
      case "setting":
        out.push(...(c.id === "settings.reset" ? resetRows(c, prefs) : settingRows(c, prefs)))
        break
      default:
        if (c.id in BARE) out.push({ ...base, key: `${c.id}:bare`, title: BARE[c.id], arg: null })
        out.push({ ...base, key: c.id, title: `${c.title}…`, arg: null, ask: c.arg })
    }
  }
  return out
}

/** What to type for an argument of this kind. */
export function argHint(kind: Arg): string {
  switch (kind) {
    case "user":
      return "Username or user ID"
    case "gameOrFriend":
      return "Place ID, game link, or a friend"
    case "outfit":
      return "Outfit ID"
    case "font":
      return "A Google Fonts name, or Inter, JetBrains Mono or System"
    default:
      return "Type, then press Enter"
  }
}

/** What the command bar searches. `all` mixes commands, friends and Roblox results. */
export type Scope = "all" | "commands" | "people" | "experiences" | "communities" | "items" | "bookmarks"

export const scopes: { value: Scope; label: string; prefix: string }[] = [
  { value: "all", label: "All", prefix: "" },
  { value: "people", label: "People", prefix: "@" },
  { value: "experiences", label: "Experiences", prefix: "g:" },
  { value: "communities", label: "Communities", prefix: "#" },
  { value: "items", label: "Items", prefix: "i:" },
  { value: "bookmarks", label: "Bookmarks", prefix: "b:" },
  { value: "commands", label: "Commands", prefix: ">" },
]

/** Splits a typed scope prefix (`@`, `g:`, `#`, `i:`, `>`) off the query. */
export function parseScope(query: string): { scope: Scope; text: string } | null {
  const q = query.trimStart()
  const s = scopes.find((x) => x.prefix && q.startsWith(x.prefix))
  return s ? { scope: s.value, text: q.slice(s.prefix.length).trimStart() } : null
}

/** A bare id or a Roblox link: searched at any length. */
const isLookup = (text: string) => /^\d+$/.test(text) || /roblox\.com|^roblox:/i.test(text)

/** The query to send to Roblox's search, with the scope's prefix, or "" when there is nothing
 * to search: commands only, or under three letters (ids and links are searched at any length). */
export function remoteQuery(scope: Scope, text: string): string {
  const t = text.trim()
  if (scope === "commands" || scope === "bookmarks" || (t.length < 3 && !isLookup(t))) return ""
  return `${scopes.find((s) => s.value === scope)?.prefix ?? ""}${t}`
}

const thumbKinds: Record<RemoteKind, ThumbKind> = {
  user: "headshot",
  experience: "game_icon",
  community: "group_icon",
  item: "asset",
  bundle: "bundle",
}

/** The picture for a search result: a game icon by universe, the rest by id. */
export function remoteThumb(r: RemoteRow): { kind: ThumbKind; id: number } | undefined {
  const id = r.kind === "experience" ? r.universeId : r.id
  return id ? { kind: thumbKinds[r.kind], id } : undefined
}

/** Roblox search results as command bar rows. */
export function remoteEntries(rows: RemoteRow[]): PaletteEntry[] {
  return rows.map((r) => {
    return {
      key: `remote:${r.group}:${r.command}:${r.arg}`,
      group: r.group,
      title: r.title,
      subtitle: r.subtitle || undefined,
      words: r.title,
      id: r.command,
      arg: r.arg,
      thumb: remoteThumb(r),
      entity: { kind: r.kind, id: r.id },
    }
  })
}

/** The words of a name: split at spaces, camelCase humps and where letters meet digits, so
 * "StickMasterLuke" is "stick master luke" and "Builder123" is "builder 123". */
export function nameWords(name: string): string[] {
  const raw = name.normalize("NFD").replace(/\p{M}/gu, "")
  return tokens(
    raw
      .replace(/(\p{Ll}|\p{N})(\p{Lu})/gu, "$1 $2")
      .replace(/(\p{Lu})(\p{Lu}\p{Ll})/gu, "$1 $2")
      .replace(/(\p{L})(\p{N})/gu, "$1 $2")
      .replace(/(\p{N})(\p{L})/gu, "$1 $2"),
  )
}

interface PreparedName {
  prep: Prepared
  /** The whole name run together, folded: "stickmasterluke". */
  whole: string
}

const names = new WeakMap<PalettePerson, PreparedName[]>()

/** A friend's names, best first: what you call them, their display name, their username. */
function preparedNames(p: PalettePerson): PreparedName[] {
  let out = names.get(p)
  if (!out) {
    out = [p.nickname, p.displayName, p.name].flatMap((n) => {
      if (!n) return []
      const ws = nameWords(n)
      if (!ws.length) return []
      // Initials of a name ("sml") find too many people to mean much.
      return [{ prep: { ...prepare(ws.join(" "), ""), initials: [] }, whole: ws.join("") }]
    })
    names.set(p, out)
  }
  return out
}

/** Part of a name, not at its start or at a word in it. */
const NAME_INSIDE = 30
/** A later name counts this much less than the first. */
const NAME_STEP = 2

/** How well a friend matches a typed name, on the same scale as commands and Roblox results
 * (`matchTerms`): -1 for not at all. Each name is matched by its words ("luke" in
 * StickMasterLuke) and as a whole ("stickmast", or a typo of it). Typing a whole name exactly
 * counts like a Roblox result named exactly that. */
export function personScore(p: PalettePerson, text: string): number {
  const ts = terms(text)
  if (!ts.length) return 0
  const joined = ts.join("")
  let best = -1
  preparedNames(p).forEach(({ prep, whole }, i) => {
    const byWords = matchTerms(prep, ts, false)?.score ?? 0
    // Usernames are often one lowercase run ("stickmasterluke"), so a part of one counts
    // from three letters, where a command's word needs four.
    const asWhole = wordScore(joined, whole) || (joined.length >= 3 && whole.includes(joined) ? NAME_INSIDE : 0)
    let s = Math.max(byWords, asWhole ? asWhole + COVERAGE_WHOLE : 0)
    if (!s) return
    if (whole === joined) s += REMOTE_EXACT
    best = Math.max(best, s - i * NAME_STEP)
  })
  return best
}

/** A match of the whole name covers all of it, like a title whose every word matched. */
const COVERAGE_WHOLE = 25

/** `people` matching `query`, best first, each with its score. `lift` adds what was picked or
 * looked at before, by ref. Ties keep the list's order. */
function searchPeople(
  people: readonly PalettePerson[],
  query: string,
  lift: (ref: string) => number = () => 0,
): { person: PalettePerson; score: number }[] {
  const out: { person: PalettePerson; score: number; index: number }[] = []
  people.forEach((person, index) => {
    const s = personScore(person, query)
    if (s >= 0) out.push({ person, score: s + lift(`user:${person.id}`), index })
  })
  return out.sort((a, b) => b.score - a.score || a.index - b.index)
}

/** A person as a row that opens their profile. */
export function personEntry(p: PalettePerson): PaletteEntry {
  return {
    key: `user:${p.id}`,
    title: p.displayName || p.name,
    subtitle: [p.name && `@${p.name}`, p.status].filter(Boolean).join(" · "),
    words: p.name,
    id: "user.open",
    arg: String(p.id),
    thumb: { kind: "headshot", id: p.id },
    entity: { kind: "user", id: p.id, inGame: p.inGame },
    ref: `user:${p.id}`,
  }
}

export interface PaletteSection {
  title: string
  rows: PaletteEntry[]
  /** The scope that lists all of a group that was cut short. */
  more?: Scope
}

export interface SectionInput {
  scope: Scope
  text: string
  /** Every command row, from `paletteEntries`. */
  commands: PaletteEntry[]
  friends: PalettePerson[]
  remote: PaletteEntry[]
  bookmarks?: BookmarkView[]
  /** Rows picked or looked at lately, newest first (`recentRows`). Shown for an empty query. */
  recent?: PaletteEntry[]
  /** What was typed lately, newest first (`recentQueries`). Shown for an empty query. */
  searches?: string[]
  /** Things looked at before (`seenEntry`), found from the first letter. */
  seen?: PaletteEntry[]
  /** Command ids of the screens to offer under "Go to" for an empty query, in sidebar order. */
  places?: string[]
  /** Commands with their argument typed inline (`intentRows`), ranked with the commands. */
  intents?: Ranked<PaletteEntry>[]
  /** Extra score for a row from what was picked or looked at before, by row key and by what a
   * row is about (`refOf`). */
  boost?: (key: string) => number
}

/** How many rows an empty query lists as recent. */
const RECENT = 6

/** Something picked from the command bar, by row key, and when. */
export interface Picked {
  key: string
  t: number
}

/** An openable thing from the history (`openable`) as a row. */
export function seenEntry(s: Seen): PaletteEntry | null {
  if (!s.open || !s.title) return null
  return {
    key: `seen:${s.ref}`,
    title: s.title,
    words: s.title,
    id: s.open.id,
    arg: s.open.arg,
    thumb: s.thumb,
    entity: s.entity,
    ref: s.ref,
    recent: true,
  }
}

/** What an empty query lists as recent: commands picked lately and things looked at lately,
 * newest first, one row each. Picks of rows that are gone (a deleted theme) or that were about
 * a thing (those come from `seen`) are skipped. */
export function recentRows(commands: readonly PaletteEntry[], picks: readonly Picked[], seen: readonly Seen[]) {
  const byKey = new Map(commands.map((e) => [e.key, e]))
  const rows: { entry: PaletteEntry; t: number }[] = []
  for (const p of picks) {
    const e = byKey.get(p.key)
    if (e && !e.ask) rows.push({ entry: e, t: p.t })
  }
  for (const s of seen) {
    const e = seenEntry(s)
    if (e) rows.push({ entry: e, t: s.t })
  }
  const keys = new Set<string>()
  return rows
    .sort((a, b) => b.t - a.t)
    .filter((r) => !keys.has(r.entry.key) && keys.add(r.entry.key))
    .slice(0, RECENT)
    .map((r) => ({ ...r.entry, recent: true }))
}

/** How many recent searches an empty query lists. */
const SEARCHES = 3

/** A search typed before, as a row that types it again. */
export const searchEntry = (q: string): PaletteEntry => ({
  key: `search:${q}`,
  title: q,
  words: q,
  id: "",
  arg: null,
  fill: q,
  recent: true,
})

/** Rows per group when searching everything; a scope shows all of its results. */
const PER_GROUP = 4

/** A screen is what a short query most often means; a single setting's value rarely is. */
const SCREEN = 30
const SETTING_VALUE = -40

/** How much more or less likely a command row is, before looking at the query. */
function prior(e: PaletteEntry): number {
  if (e.arg === null && e.id.startsWith("nav.")) return SCREEN
  if (e.id === "settings.set") return SETTING_VALUE
  return 0
}

/** Commands shown above friends and Roblox results when searching everything. */
const TOP_COMMANDS = 4

/** Order of Roblox result groups. Titles come from bonsai-core's `Group::title`. */
const REMOTE_GROUPS = ["Pasted link", "People", "Experiences", "Communities", "Marketplace"]

/** The scope that lists all of a Roblox result group. */
const REMOTE_SCOPES: Record<string, Scope | undefined> = {
  People: "people",
  Experiences: "experiences",
  Communities: "communities",
  Marketplace: "items",
}

/** Which looked-at things a scope shows. */
const SEEN_KINDS: Partial<Record<Scope, RemoteKind[]>> = {
  all: ["user", "experience", "community", "item", "bundle"],
  people: ["user"],
  experiences: ["experience"],
  communities: ["community"],
  items: ["item", "bundle"],
}

/** The scope listing every looked-at thing in `rows`, when they are all one kind. */
function scopeOfSeen(rows: Ranked<PaletteEntry>[]): Scope | undefined {
  const kinds = new Set(rows.map((r) => r.entry.entity?.kind))
  if (kinds.size !== 1) return undefined
  return (Object.keys(SEEN_KINDS) as Scope[]).find(
    (s) => s !== "all" && SEEN_KINDS[s]?.includes([...kinds][0] as RemoteKind),
  )
}

/** You saved a bookmark on purpose, so it beats a screen that matches as well. */
const BOOKMARK = 40
/** A Roblox result named exactly what was typed. */
const REMOTE_EXACT = 30
/** Each place down Roblox's own order costs this much. */
const REMOTE_STEP = 3

/** The command bar's grouped rows. Searching everything puts the best of commands, things
 * looked at before, friends and bookmarks first, each group where its best row ranks, then
 * Roblox results, then the other matching commands. A Roblox result named exactly what was typed
 * that beats everything found here comes first, as the top hit. Rows about something picked or looked at
 * before rank higher in every group. */
export function paletteSections(input: SectionInput): PaletteSection[] {
  const text = input.text.trim()
  const all = input.scope === "all"
  const limit = all ? PER_GROUP : Number.POSITIVE_INFINITY
  const out: PaletteSection[] = []
  const add = (title: string, rows: PaletteEntry[], more?: Scope) => {
    if (rows.length) out.push(more ? { title, rows, more } : { title, rows })
  }
  const boost = input.boost ?? (() => 0)
  /** What was picked or looked at before, by the row and by what it is about. */
  const lift = (e: PaletteEntry) => {
    const ref = refOf(e)
    return boost(e.key) + (ref && ref !== e.key ? boost(ref) : 0)
  }
  const bookmarkRows = (): PaletteEntry[] =>
    (input.bookmarks ?? []).map((b) => ({
      key: `bookmark:${b.id}`,
      title: b.title,
      words: b.title,
      id: "bookmark.open",
      arg: String(b.id),
      subtitle: b.status || undefined,
      thumb: b.thumbnail ?? undefined,
    }))

  if (input.scope === "bookmarks") {
    add(
      "Bookmarks",
      rankScored(bookmarkRows(), text, lift).map((r) => r.entry),
    )
    return out
  }
  if (all && !text) {
    const recent = input.recent ?? []
    const byKey = new Map(input.commands.map((e) => [e.key, e]))
    const seen = new Set(recent.map((e) => e.key))
    const places = (input.places ?? []).flatMap((id) => {
      const e = byKey.get(id)
      return e && !seen.has(e.key) ? [e] : []
    })
    for (const e of places) seen.add(e.key)
    add("Recent", recent)
    add("Recent searches", (input.searches ?? []).slice(0, SEARCHES).map(searchEntry))
    add("Go to", places)
    add(
      "Commands",
      input.commands.filter((e) => !seen.has(e.key)),
    )
    return out
  }
  const ranked = () => {
    const scored = rankScored(input.commands, text, (e) => prior(e) + lift(e))
    const intents = (input.intents ?? []).map((r) => ({ ...r, score: r.score + lift(r.entry) }))
    return [...intents, ...scored].sort((a, b) => b.score - a.score)
  }
  if (input.scope === "commands") {
    add(
      "Commands",
      ranked().map((r) => r.entry),
    )
    return out
  }

  /** Groups found on this machine, placed by their best row. */
  const local: { title: string; rows: PaletteEntry[]; best: number; more?: Scope }[] = []
  const addLocal = (title: string, rows: { entry: PaletteEntry; score: number }[], more?: Scope) => {
    if (rows.length) local.push({ title, rows: rows.map((r) => r.entry), best: rows[0].score, more })
  }
  let rest: PaletteEntry[] = []
  if (all) {
    const matched = ranked()
    const top = matched.filter((r) => r.strong).slice(0, TOP_COMMANDS)
    addLocal("Commands", top)
    rest = matched.filter((r) => !top.includes(r)).map((r) => r.entry)
  }

  /** What is already shown, so Roblox results don't repeat it. */
  const shown = new Set<string>()
  const everyone = all || input.scope === "people" ? searchPeople(input.friends, text, boost) : []
  const people = everyone.slice(0, all ? PER_GROUP : 10)
  for (const p of people) shown.add(`user:${p.person.id}`)

  const kinds = SEEN_KINDS[input.scope] ?? []
  const seenAll = rankScored(
    (input.seen ?? []).filter((e) => {
      const ref = refOf(e)
      return e.entity && kinds.includes(e.entity.kind) && !(ref && shown.has(ref))
    }),
    text,
    lift,
  ).filter((r) => r.strong)
  const seen = seenAll.slice(0, limit)
  for (const r of seen) {
    const ref = refOf(r.entry)
    if (ref) shown.add(ref)
  }
  addLocal("Recently viewed", seen, all && seenAll.length > seen.length ? scopeOfSeen(seenAll) : undefined)
  addLocal(
    "Friends",
    people.map((p) => ({ entry: personEntry(p.person), score: p.score })),
    all && everyone.length > people.length ? "people" : undefined,
  )
  if (all) {
    const marks = rankScored(bookmarkRows(), text, (e) => BOOKMARK + lift(e)).filter((r) => r.strong)
    addLocal("Bookmarks", marks.slice(0, limit), marks.length > limit ? "bookmarks" : undefined)
  }
  // Stable: on a tie, the order above.
  local.sort((a, b) => b.best - a.best)

  // Roblox results are scored like everything else, so one named exactly what was typed can
  // come first, while keeping roughly Roblox's own order among the rest.
  const ts = terms(text)
  const typed = fold(text)
  const remote = REMOTE_GROUPS.map((group) => ({
    group,
    rows: input.remote
      .filter((r) => {
        if (r.group !== group) return false
        const ref = refOf(r)
        return !(ref && shown.has(ref))
      })
      .map((entry, i) => {
        const exact = fold(entry.title) === typed || fold(entry.subtitle ?? "") === `@${typed}`
        const m = matchTerms(prepared(entry), ts)
        return {
          entry,
          exact,
          score: (m?.score ?? 0) + (exact ? REMOTE_EXACT : 0) + lift(entry) - REMOTE_STEP * i,
        }
      })
      .sort((a, b) => b.score - a.score),
  }))
  const best = remote.flatMap((g) => g.rows.filter((r) => r.exact).slice(0, 1)).sort((a, b) => b.score - a.score)[0]
  if (all && best && best.score > (local[0]?.best ?? Number.NEGATIVE_INFINITY)) {
    add("Top hit", [best.entry])
    for (const g of remote) g.rows = g.rows.filter((r) => r !== best)
  }
  for (const g of local) add(g.title, g.rows, g.more)
  // A scoped Roblox search returns more than the few shown here, so these always offer it.
  for (const g of remote)
    add(
      g.group,
      g.rows.slice(0, limit).map((r) => r.entry),
      all ? REMOTE_SCOPES[g.group] : undefined,
    )

  add("More commands", rest)
  return out
}

/** Every row of `sections`, in order, for keyboard movement. */
export const flatRows = (sections: PaletteSection[]) => sections.flatMap((s) => s.rows)

interface Action {
  id: string
  arg: (e: PaletteEntity) => string
  when?: (e: PaletteEntity) => boolean
}

/** Commands offered for a result, first one being what Enter does. Titled from the registry. */
const ACTIONS: Record<RemoteKind, Action[]> = {
  user: [
    { id: "user.open", arg: (e) => String(e.id) },
    { id: "game.join", arg: (e) => `user:${e.id}`, when: (e) => e.inGame === true },
    { id: "user.message", arg: (e) => String(e.id) },
    { id: "trade.new", arg: (e) => String(e.id) },
    { id: "inventory.view", arg: (e) => String(e.id) },
    { id: "user.add_friend", arg: (e) => String(e.id) },
    { id: "user.follow", arg: (e) => String(e.id) },
    { id: "user.block", arg: (e) => String(e.id) },
  ],
  experience: [
    { id: "game.open", arg: (e) => String(e.id) },
    { id: "game.join", arg: (e) => String(e.id) },
  ],
  community: [{ id: "group.open", arg: (e) => String(e.id) }],
  item: [{ id: "market.open", arg: (e) => `item:${e.id}` }],
  bundle: [{ id: "market.open", arg: (e) => `bundle:${e.id}` }],
}

/** The actions for a result's entity, as rows. `title` looks a command up in the registry. */
export function entityActions(entity: PaletteEntity, title: (id: string) => string | undefined): PaletteEntry[] {
  return ACTIONS[entity.kind]
    .filter((a) => a.when?.(entity) ?? true)
    .map((a) => ({
      key: `action:${a.id}`,
      title: (title(a.id) ?? a.id).replace(/…$/, ""),
      words: "",
      id: a.id,
      arg: a.arg(entity),
    }))
}

/** Whether a result has more than its default action. */
export const hasActions = (e: PaletteEntry) => e.entity !== undefined && entityActions(e.entity, () => "").length > 1

/** Suggestions while typing an argument: matching friends, then Roblox people results for user
 * arguments, then the typed text itself. Before anything is typed, what the command was run with
 * lately. */
export function argEntries(
  command: PaletteEntry,
  query: string,
  people: PalettePerson[],
  remote: PaletteEntry[] = [],
  limit = 8,
  boost: (key: string) => number = () => 0,
  recent: readonly RecentArg[] = [],
): PaletteEntry[] {
  const text = query.trim()
  const q = text.toLowerCase()
  const base = { id: command.id, words: "" }
  const out: PaletteEntry[] = []
  if (!text)
    return recent.map((a) => ({
      ...base,
      key: `${command.id}:recent:${a.arg}`,
      title: a.title,
      words: a.title,
      arg: a.arg,
      thumb: a.thumb,
      ref: a.ref,
      recent: true,
    }))
  const user = command.ask === "user"
  if ((user || command.ask === "gameOrFriend") && q) {
    const matches = searchPeople(
      people.filter((p) => user || p.inGame),
      q,
      boost,
    ).slice(0, limit)
    for (const { person: p } of matches)
      out.push({
        ...personEntry(p),
        ...base,
        key: `${command.id}:${p.id}`,
        arg: user ? String(p.id) : `user:${p.id}`,
        entity: undefined,
      })
  }
  if (user) {
    const seen = new Set(out.map((e) => e.arg))
    for (const r of remote)
      if (r.entity?.kind === "user" && !seen.has(String(r.entity.id)))
        out.push({
          ...r,
          ...base,
          key: `${command.id}:${r.key}`,
          arg: String(r.entity.id),
          entity: undefined,
          ref: refOf(r),
        })
  }
  if (text) out.push({ ...base, key: `${command.id}:text`, title: `Use “${text}”`, arg: text })
  return out
}

/** Argument kinds you type, which are worth offering again. */
const TYPED_ARGS: ReadonlySet<Arg> = new Set<Arg>(["user", "gameOrFriend", "text", "font", "outfit"])

/** What to offer again the next time a command that takes a typed argument asks for it, after
 * picking `e`: the command and the argument with its title. `asked` is the command that asked
 * for it, `about` the result whose actions were listed, `kind` looks up what a command takes.
 * Null for rows that don't run such a command. */
export function argToRemember(
  e: PaletteEntry,
  asked: PaletteEntry | null,
  about: PaletteEntry | null,
  kind: (id: string) => Arg | undefined,
): { command: string; arg: Omit<RecentArg, "t"> } | null {
  if (e.arg === null || e.arg === "" || e.fill !== undefined) return null
  const command = asked?.id ?? e.id
  const k = asked?.ask ?? kind(command)
  if (!k || !TYPED_ARGS.has(k)) return null
  let title = e.title
  let thumb = e.thumb
  if (about) {
    title = about.title
    thumb = about.thumb
  } else if (e.key === `${command}:text`) title = e.arg
  else if (e.key.startsWith("intent:")) title = e.title.slice(e.title.indexOf(": ") + 2).replace(/^“(.*)”$/, "$1")
  return { command, arg: { arg: e.arg, title, thumb, ref: refOf(e) ?? (about ? refOf(about) : undefined) } }
}

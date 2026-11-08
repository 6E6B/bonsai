import type { Arg, CommandInfo, PrefView, Presence, RemoteKind, RemoteRow } from "~/lib/ipc/bindings"
import { rank } from "~/lib/search"
import { settingsSections } from "~/lib/sections"
import type { ThumbKind } from "~/lib/thumb"
import { type Ranked, rankScored } from "./match"

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
}

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
export type Scope = "all" | "commands" | "people" | "experiences" | "communities" | "items"

export const scopes: { value: Scope; label: string; prefix: string }[] = [
  { value: "all", label: "All", prefix: "" },
  { value: "people", label: "People", prefix: "@" },
  { value: "experiences", label: "Experiences", prefix: "g:" },
  { value: "communities", label: "Communities", prefix: "#" },
  { value: "items", label: "Items", prefix: "i:" },
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
  if (scope === "commands" || (t.length < 3 && !isLookup(t))) return ""
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

/** `people` matching `query`, best match first. */
const searchPeople = (people: readonly PalettePerson[], query: string) =>
  rank(people, query, (p) => [p.displayName, p.nickname, p.name])

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
  }
}

export interface PaletteSection {
  title: string
  rows: PaletteEntry[]
}

export interface SectionInput {
  scope: Scope
  text: string
  /** Every command row, from `paletteEntries`. */
  commands: PaletteEntry[]
  friends: PalettePerson[]
  remote: PaletteEntry[]
  /** Rows picked lately, newest first (`remember`). Shown for an empty query. */
  recent?: PaletteEntry[]
  /** Command ids of the screens to offer under "Go to" for an empty query, in sidebar order. */
  places?: string[]
  /** Commands with their argument typed inline (`intentRows`), ranked with the commands. */
  intents?: Ranked<PaletteEntry>[]
  /** Extra score for a row from what was picked before (`booster`), by row key. */
  boost?: (key: string) => number
}

/** How many rows `remember` keeps. */
const RECENT = 5

/** `recent` with `entry` moved to the front. Rows that ask for an argument aren't kept: the
 * row alone doesn't say what was typed. */
export function remember(recent: PaletteEntry[], entry: PaletteEntry): PaletteEntry[] {
  if (entry.ask) return recent
  return [entry, ...recent.filter((e) => e.key !== entry.key)].slice(0, RECENT)
}

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

/** The command bar's grouped rows. Searching everything puts commands whose title matches
 * first, then friends, then Roblox results, then the other matching commands. */
export function paletteSections(input: SectionInput): PaletteSection[] {
  const text = input.text.trim()
  const q = text.toLowerCase()
  const all = input.scope === "all"
  const limit = all ? PER_GROUP : Number.POSITIVE_INFINITY
  const out: PaletteSection[] = []
  const add = (title: string, rows: PaletteEntry[]) => {
    if (rows.length) out.push({ title, rows })
  }

  if (all && !text) {
    // Recent command rows are looked up again, so renamed or removed ones (a deleted theme)
    // follow the registry. Results and friends are kept as they were.
    const byKey = new Map(input.commands.map((e) => [e.key, e]))
    const recent = (input.recent ?? []).flatMap((e) => {
      const now = byKey.get(e.key)
      return now ? [now] : e.entity ? [e] : []
    })
    const seen = new Set(recent.map((e) => e.key))
    const places = (input.places ?? []).flatMap((id) => {
      const e = byKey.get(id)
      return e && !seen.has(e.key) ? [e] : []
    })
    for (const e of places) seen.add(e.key)
    add("Recent", recent)
    add("Go to", places)
    add(
      "Commands",
      input.commands.filter((e) => !seen.has(e.key)),
    )
    return out
  }
  const ranked = () => {
    const boost = input.boost
    const scored = rankScored(input.commands, text, (e) => prior(e) + (boost?.(e.key) ?? 0))
    const intents = (input.intents ?? []).map((r) => ({ ...r, score: r.score + (boost?.(r.entry.key) ?? 0) }))
    return [...intents, ...scored].sort((a, b) => b.score - a.score)
  }
  if (input.scope === "commands") {
    add(
      "Commands",
      ranked().map((r) => r.entry),
    )
    return out
  }

  let rest: PaletteEntry[] = []
  if (all) {
    const matched = ranked()
    const top = matched.filter((r) => r.strong).slice(0, TOP_COMMANDS)
    add(
      "Commands",
      top.map((r) => r.entry),
    )
    rest = matched.filter((r) => !top.includes(r)).map((r) => r.entry)
  }

  const shownPeople = new Set<number>()
  if (all || input.scope === "people") {
    const friends = searchPeople(input.friends, q).slice(0, all ? PER_GROUP : 10)
    for (const f of friends) shownPeople.add(f.id)
    add("Friends", friends.map(personEntry))
  }

  for (const group of REMOTE_GROUPS) {
    const rows = input.remote.filter(
      (r) => r.group === group && !(r.entity?.kind === "user" && shownPeople.has(r.entity.id)),
    )
    add(group, rows.slice(0, limit))
  }

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
    { id: "user.find", arg: (e) => String(e.id) },
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
 * arguments, then the typed text itself. */
export function argEntries(
  command: PaletteEntry,
  query: string,
  people: PalettePerson[],
  remote: PaletteEntry[] = [],
  limit = 8,
): PaletteEntry[] {
  const text = query.trim()
  const q = text.toLowerCase()
  const base = { id: command.id, words: "" }
  const out: PaletteEntry[] = []
  const user = command.ask === "user"
  if ((user || command.ask === "gameOrFriend") && q) {
    const matches = searchPeople(
      people.filter((p) => user || p.inGame),
      q,
    ).slice(0, limit)
    for (const p of matches)
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
        out.push({ ...r, ...base, key: `${command.id}:${r.key}`, arg: String(r.entity.id), entity: undefined })
  }
  if (text) out.push({ ...base, key: `${command.id}:text`, title: `Use “${text}”`, arg: text })
  return out
}

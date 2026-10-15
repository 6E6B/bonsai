import type { Arg, CommandInfo, Presence, RemoteKind, RemoteRow } from "~/lib/ipc/bindings"
import { settingsSections } from "~/lib/sections"
import type { ThumbKind } from "~/lib/thumb"
import { filterEntries } from "./filter"

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

const COLORS = ["none", "red", "orange", "yellow", "green", "blue", "purple", "pink"]
const THEMES = ["system", "light", "dark"]

const titleCase = (s: string) => s.charAt(0).toUpperCase() + s.slice(1)

/** Every command, with one row per choice for arguments that have a short fixed list
 * (accounts, settings sections, colors, themes) and an asking row for typed arguments. */
export function paletteEntries(
  commands: CommandInfo[],
  accounts: PaletteAccount[],
  tabs: PaletteTab[] = [],
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
      case "theme":
        for (const v of THEMES) choice(v, titleCase(v))
        break
      case "tab":
        for (const t of tabs) choice(t.key, t.title)
        break
      default:
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

const matchesPerson = (p: PalettePerson, q: string) =>
  p.name.toLowerCase().includes(q) || p.displayName.toLowerCase().includes(q)

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
}

/** Rows per group when searching everything; a scope shows all of its results. */
const PER_GROUP = 4

/** Order of Roblox result groups. Titles come from rbx-core's `Group::title`. */
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

  if (input.scope === "commands" || (all && !text)) {
    add("Commands", filterEntries(input.commands, text))
    return out
  }

  let rest: PaletteEntry[] = []
  if (all) {
    const matched = filterEntries(input.commands, text)
    const terms = q.split(/\s+/)
    const inTitle = (e: PaletteEntry) => terms.every((t) => e.title.toLowerCase().includes(t))
    const top = matched.filter(inTitle).slice(0, 3)
    add("Commands", top)
    rest = matched.filter((e) => !top.includes(e))
  }

  const shownPeople = new Set<number>()
  if (all || input.scope === "people") {
    const friends = input.friends.filter((p) => matchesPerson(p, q)).slice(0, all ? PER_GROUP : 10)
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
    const matches = people
      .filter((p) => user || p.inGame)
      .filter((p) => matchesPerson(p, q))
      .slice(0, limit)
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

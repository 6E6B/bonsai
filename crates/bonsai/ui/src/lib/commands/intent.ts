import type { RemoteKind } from "~/lib/ipc/bindings"
import { fold, score as nameScore } from "~/lib/search"
import { distance, matchTerms, prepared, type Ranked, tokens } from "./match"
import type { PaletteEntry, PalettePerson } from "./palette"
import { personEntry } from "./palette"
import { STOP } from "./synonyms"

/** A command and its argument typed in one go: "join builder", "message luke hi", "search wiki
 * doors". The start of the query names a command that takes an argument, and the rest is
 * matched against what that argument can be (friends, Roblox results) or passed as text. For
 * people and games the name can come first too: "builder profile". */

/** Commands whose typed argument names something Roblox search finds, and how a result of
 * that kind becomes the argument. */
const REMOTE_ARGS: Record<string, Partial<Record<RemoteKind, (id: number) => string>>> = {
  "game.join": { experience: String },
  "game.join_as": { experience: String },
  "game.open": { experience: String },
  "group.open": { community: String },
  "market.open": { item: (id) => `item:${id}`, bundle: (id) => `bundle:${id}` },
  "market.preview": { item: (id) => `item:${id}`, bundle: (id) => `bundle:${id}` },
}

/** A typed id or Roblox link, which these commands take as text. */
const isIdOrLink = (s: string) => /^\d+$/.test(s) || /roblox\.com|^roblox:/i.test(s)

type Kind = "people" | "friendsInGame" | "remote" | "text"

/** What the argument of an asking row can be, or null when it can't be typed inline. */
function kindOf(e: PaletteEntry): Kind | null {
  switch (e.ask) {
    case "user":
      return "people"
    case "gameOrFriend":
      return "friendsInGame"
    case "text":
    case "font":
    case "outfit":
      return e.id in REMOTE_ARGS ? "remote" : "text"
    default:
      return null
  }
}

/** Whether results for `kind` come from Roblox search. */
const searchesRoblox = (kind: Kind) => kind !== "text"

export interface Head {
  entry: PaletteEntry
  kind: Kind
  /** The rest of the query, as typed. */
  tail: string
  score: number
  /** How many query terms named the command. */
  terms: number
  /** The command came after its argument. */
  reversed: boolean
}

/** Each head term must match at least this well: a word, a good prefix or a close typo. */
const HEAD_TERM = 55
/** At most this many words name a command. */
const HEAD_WORDS = 4
/** A command named after its argument counts this much. */
const REVERSED = 0.85

/** Commands named by the start (or, for people and games, the end) of `text`, with the rest as
 * their argument. Best first; one reading per command. */
export function heads(commands: readonly PaletteEntry[], text: string): Head[] {
  const words = text.trim().split(/\s+/).filter(Boolean)
  if (words.length < 2) return []
  const byCommand = new Map<string, Head>()
  for (const entry of commands) {
    const kind = entry.ask ? kindOf(entry) : null
    if (!kind) continue
    const p = prepared(entry)
    const consider = (headWords: string[], tail: string, reversed: boolean) => {
      const ts = headWords.flatMap(tokens)
      if (!ts.length || ts.every((t) => STOP.has(t)) || !tail) return
      const m = matchTerms(p, ts, false)
      if (!m || m.weakest < HEAD_TERM) return
      const score = m.score * (reversed ? REVERSED : 1)
      const prev = byCommand.get(entry.key)
      // On a tie a name keeps the stop words before it: "chat with sam" is about "sam". Text
      // arguments keep every word.
      const better = !prev || score > prev.score || (score === prev.score && kind !== "text" && ts.length > prev.terms)
      if (better) byCommand.set(entry.key, { entry, kind, tail, score, terms: ts.length, reversed })
    }
    for (let k = 1; k < words.length && k <= HEAD_WORDS; k++) {
      consider(words.slice(0, k), words.slice(k).join(" "), false)
      if (kind !== "text") consider(words.slice(-k), words.slice(0, -k).join(" "), true)
    }
  }
  return [...byCommand.values()].sort((a, b) => b.score - a.score)
}

/** What to search Roblox for: the argument part when the query starts with a command that
 * takes a person or a game ("join tower of hell" searches "tower of hell"), else null. */
export function remoteTail(commands: readonly PaletteEntry[], text: string): string | null {
  const all = heads(commands, text)
  const h = all.find((x) => !x.reversed && searchesRoblox(x.kind))
  // A much better reading as a text command ("search wiki doors") keeps the whole query.
  if (!h || all[0].score > h.score * 1.2) return null
  return h.tail
}

/** How well a friend matches a typed name: -1 for not at all. Allows one typo in a name of
 * four letters or more. */
export function personScore(p: PalettePerson, text: string): number {
  const s = nameScore([p.displayName, p.nickname, p.name], text)
  if (s >= 0) return s
  const t = fold(text.trim())
  if (t.length < 4 || /\s/.test(t)) return -1
  for (const n of [p.name, p.displayName, p.nickname]) {
    if (!n) continue
    const f = fold(n)
    if (distance(t, f, 1) <= 1 || (f.length > t.length && distance(t, f.slice(0, t.length), 1) <= 1)) return 30
  }
  return -1
}

/** Friend results count this much at most; Roblox results a bit less. */
const REMOTE_TOP = 55
const REMOTE_EXACT = 95
const REMOTE_STEP = 3
const TEXT = 20
/** An id or a link is almost certainly the argument. */
const ID = 45
/** A reading that uses every word of the query. */
const WHOLE = 20
/** At most this many inline rows, and this many plain-text ones among them. */
const MAX_ROWS = 8
const MAX_TEXT = 3

/** The command part of a row's title: "Open profile…" is "Open profile", "Rename playlist
 * (id, then name)" is "Rename playlist". */
export const commandLabel = (title: string) =>
  title
    .replace(/…$/, "")
    .replace(/\s*\([^)]*\)$/, "")
    .trim()

export interface IntentInput {
  commands: readonly PaletteEntry[]
  text: string
  people: readonly PalettePerson[]
  /** Roblox results, and what was searched for them. They are used for a reading whose
   * argument is that text. */
  remote: readonly PaletteEntry[]
  remoteFor?: string
}

/** Rows that run a command with its argument straight away, scored like `rankScored` rows. */
export function intentRows(input: IntentInput): Ranked<PaletteEntry>[] {
  const out: Ranked<PaletteEntry>[] = []
  let texts = 0
  for (const h of heads(input.commands, input.text).slice(0, 10)) {
    const c = h.entry
    const label = commandLabel(c.title)
    const base = h.score + WHOLE
    const row = (arg: string, title: string, extra: Partial<PaletteEntry>, s: number, strong: boolean) =>
      out.push({
        entry: { key: `intent:${c.id}:${arg}`, title: `${label}: ${title}`, words: "", id: c.id, arg, ...extra },
        score: base + s,
        strong,
      })
    const tail = h.tail.trim()
    const remote = input.remoteFor !== undefined && fold(input.remoteFor.trim()) === fold(tail) ? input.remote : []
    if (h.kind === "people" || h.kind === "friendsInGame") {
      const join = h.kind === "friendsInGame"
      const seen = new Set<number>()
      let exact = false
      for (const p of input.people) {
        if (join && !p.inGame) continue
        const s = personScore(p, tail)
        if (s < 0) continue
        seen.add(p.id)
        if (fold(p.name) === fold(tail) || fold(p.displayName) === fold(tail)) exact = true
        const r = personEntry(p)
        row(join ? `user:${p.id}` : String(p.id), r.title, { subtitle: r.subtitle, thumb: r.thumb }, s, true)
      }
      if (!join) {
        remote.forEach((r, i) => {
          if (r.entity?.kind !== "user" || seen.has(r.entity.id)) return
          const same = fold(r.title) === fold(tail) || fold(r.subtitle ?? "").includes(`@${fold(tail)}`)
          exact ||= same
          const s = same ? REMOTE_EXACT : REMOTE_TOP - REMOTE_STEP * i
          row(String(r.entity.id), r.title, { subtitle: r.subtitle, thumb: r.thumb }, s, !h.reversed)
        })
        if (!exact && !h.reversed) row(tail, `“${tail}”`, {}, TEXT, false)
      }
    }
    if (h.kind === "remote" || h.kind === "friendsInGame") {
      const kinds = REMOTE_ARGS[c.id] ?? {}
      remote.forEach((r, i) => {
        const to = r.entity && kinds[r.entity.kind]
        if (!to || !r.entity) return
        const s = fold(r.title) === fold(tail) ? REMOTE_EXACT : REMOTE_TOP - REMOTE_STEP * i
        row(to(r.entity.id), r.title, { subtitle: r.subtitle, thumb: r.thumb }, s, !h.reversed)
      })
      if (isIdOrLink(tail)) row(tail, tail, {}, ID, true)
    }
    if (h.kind === "text" && texts < MAX_TEXT) {
      texts++
      // A plain-text argument is only a confident reading when the command was named well.
      row(h.tail, `“${h.tail}”`, {}, TEXT, h.terms >= 2 && h.score >= 150)
    }
  }
  const seen = new Set<string>()
  return out
    .sort((a, b) => b.score - a.score)
    .filter((r) => !seen.has(r.entry.key) && seen.add(r.entry.key))
    .slice(0, MAX_ROWS)
}

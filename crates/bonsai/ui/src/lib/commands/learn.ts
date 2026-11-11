import { fade } from "~/lib/history"
import { fold } from "~/lib/search"
import type { ThumbKind } from "~/lib/thumb"

/** What the command bar learned from your picks: how often and how lately each row was picked,
 * and what you had typed when you picked it. Rows are ranked higher the more they fit both. */
export interface Usage {
  picks: Record<string, { n: number; t: number }>
  /** Newest last. */
  queries: { q: string; key: string; n: number; t: number }[]
  /** What each command that takes a typed argument was last run with, newest first. */
  args?: Record<string, RecentArg[]>
}

/** An argument a command was run with, to offer again. */
export interface RecentArg {
  arg: string
  title: string
  thumb?: { kind: ThumbKind; id: number }
  /** What it is about (`user:1`), for ranking. */
  ref?: string
  t: number
}

export const emptyUsage = (): Usage => ({ picks: {}, queries: [] })

/** Arguments kept per command. */
const ARGS = 6
/** Commands whose arguments are kept. */
const ARG_COMMANDS = 200

/** `u` after running `command` with `a`. Doesn't change `u`. */
export function rememberArg(u: Usage, command: string, a: Omit<RecentArg, "t">, now = Date.now()): Usage {
  const prev = u.args?.[command] ?? []
  const list = [{ ...a, t: now }, ...prev.filter((x) => x.arg !== a.arg)].slice(0, ARGS)
  let args = { ...u.args, [command]: list }
  const keys = Object.keys(args)
  if (keys.length > ARG_COMMANDS) {
    const keep = keys.sort((x, y) => (args[y][0]?.t ?? 0) - (args[x][0]?.t ?? 0)).slice(0, ARG_COMMANDS)
    args = Object.fromEntries(keep.map((k) => [k, args[k]]))
  }
  return { ...u, args }
}

/** `u` without anything learned about these row keys and refs. A recent-argument row's key
 * (`<command>:recent:<arg>`) drops that argument. */
export function forget(u: Usage, keys: readonly string[]): Usage {
  const gone = new Set(keys)
  const picks = Object.fromEntries(Object.entries(u.picks).filter(([k]) => !gone.has(k)))
  const queries = u.queries.filter((r) => !gone.has(r.key))
  const args = u.args
    ? Object.fromEntries(
        Object.entries(u.args).map(([c, list]) => [
          c,
          list.filter((a) => !(a.ref && gone.has(a.ref)) && !gone.has(`${c}:recent:${a.arg}`)),
        ]),
      )
    : undefined
  return { picks, queries, args }
}

/** What was typed before, newest first, once each. */
export function recentQueries(u: Usage, limit: number): string[] {
  const out: string[] = []
  for (let i = u.queries.length - 1; i >= 0 && out.length < limit; i--) {
    const q = u.queries[i].q
    if (!out.includes(q)) out.push(q)
  }
  return out
}

/** The rest of the query you most often typed that starts with `query`, to show after it and
 * accept with Tab or →. Null when there is none. */
export function completion(u: Usage, query: string, now = Date.now()): string | null {
  const typed = fold(query).replace(/\s+/g, " ").trimStart()
  if (!typed.trim()) return null
  let best: { rest: string; s: number } | null = null
  for (const r of u.queries) {
    if (r.q.length <= typed.length || !r.q.startsWith(typed)) continue
    const s = Math.log2(1 + r.n) * fade(now - r.t)
    if (!best || s > best.s) best = { rest: r.q.slice(typed.length), s }
  }
  return best?.rest ?? null
}

/** How many typed queries are kept. */
const QUERIES = 400
/** How many rows' pick counts are kept. */
const PICKS = 600

const norm = (q: string) => fold(q).trim().replace(/\s+/g, " ")

/** `u` after picking the row `key` with `query` typed. Doesn't change `u`. */
export function record(u: Usage, key: string, query: string, now = Date.now()): Usage {
  const prev = u.picks[key]
  let picks: Usage["picks"] = { ...u.picks, [key]: { n: (prev?.n ?? 0) + 1, t: now } }
  const keys = Object.keys(picks)
  if (keys.length > PICKS) {
    const keep = keys.sort((a, b) => picks[b].t - picks[a].t).slice(0, PICKS)
    picks = Object.fromEntries(keep.map((k) => [k, picks[k]]))
  }
  const q = norm(query)
  if (!q) return { ...u, picks }
  const old = u.queries.find((r) => r.q === q && r.key === key)
  const queries = [...u.queries.filter((r) => r !== old), { q, key, n: (old?.n ?? 0) + 1, t: now }].slice(-QUERIES)
  return { ...u, picks, queries }
}

/** Picked often and lately: enough to put a row you use above one you don't, among good
 * matches. */
const FRECENCY = 10
/** Picked before for exactly this query. */
const SAME_QUERY = 70
/** Picked before for a longer query that starts with this one: you are typing it again. Half of
 * it counts from the first letter, the rest grows as more of the query is typed. */
const TYPING = 60
/** Picked before for a shorter query that this one starts with. */
const LONGER = 25

/** A function giving each row key its boost for `query`. */
export function booster(u: Usage, query: string, now = Date.now()): (key: string) => number {
  const q = norm(query)
  const learned = new Map<string, number>()
  if (q)
    for (const r of u.queries) {
      const times = Math.log2(1 + r.n) * fade(now - r.t)
      let s = 0
      if (r.q === q) s = SAME_QUERY
      else if (r.q.startsWith(q)) s = TYPING * (0.5 + (0.5 * q.length) / r.q.length)
      else if (q.startsWith(r.q)) s = LONGER
      if (s) learned.set(r.key, Math.max(learned.get(r.key) ?? 0, s * Math.min(1.5, 0.6 + 0.4 * times)))
    }
  return (key) => {
    const p = u.picks[key]
    const f = p ? FRECENCY * Math.log2(1 + p.n) * fade(now - p.t) : 0
    return f + (learned.get(key) ?? 0)
  }
}

const storageKey = (account: string) => `bonsai:palette:${account}`

/** Usage saved on this machine for an account. Empty when there is none or it is unreadable. */
export function loadUsage(account: string): Usage {
  try {
    const raw = localStorage.getItem(storageKey(account))
    if (!raw) return emptyUsage()
    const u = JSON.parse(raw) as Partial<Usage>
    return {
      picks: u.picks && typeof u.picks === "object" ? u.picks : {},
      queries: Array.isArray(u.queries) ? u.queries : [],
      args: u.args && typeof u.args === "object" && !Array.isArray(u.args) ? u.args : undefined,
    }
  } catch {
    return emptyUsage()
  }
}

export function saveUsage(account: string, u: Usage) {
  try {
    localStorage.setItem(storageKey(account), JSON.stringify(u))
  } catch {
    // Storage full or blocked: learning just doesn't persist.
  }
}

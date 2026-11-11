import { fade } from "~/lib/history"
import { fold } from "~/lib/search"

/** What the command bar learned from your picks: how often and how lately each row was picked,
 * and what you had typed when you picked it. Rows are ranked higher the more they fit both. */
export interface Usage {
  picks: Record<string, { n: number; t: number }>
  /** Newest last. */
  queries: { q: string; key: string; n: number; t: number }[]
}

export const emptyUsage = (): Usage => ({ picks: {}, queries: [] })

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
  if (!q) return { picks, queries: u.queries }
  const old = u.queries.find((r) => r.q === q && r.key === key)
  const queries = [...u.queries.filter((r) => r !== old), { q, key, n: (old?.n ?? 0) + 1, t: now }].slice(-QUERIES)
  return { picks, queries }
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

import { createStore } from "solid-js/store"
import type { RemoteKind } from "~/lib/ipc/bindings"
import { screens } from "~/lib/screens"
import { pathname } from "~/lib/tabs"
import type { ThumbKind } from "~/lib/thumb"

/** What you looked at in the app, per account, so the command bar can rank it higher and find
 * it without searching Roblox. Profiles, experiences, items and communities are kept with their
 * name and picture; screens and settings sections only count, under their command bar row key
 * (`nav.friends`, `nav.settings:privacy`). Saved on this machine only. */

export interface Seen {
  /** `user:1`, `game:<place id>`, `group:1`, `item:1`, `bundle:1`, `pass:1`, or a row key. */
  ref: string
  /** Visits, counting a run of visits within `REVISIT` as one. */
  n: number
  /** Last visit (ms). */
  t: number
  title?: string
  thumb?: { kind: ThumbKind; id: number }
  /** The command that opens it again. */
  open?: { id: string; arg: string }
  /** What it is, for its other actions in the command bar. */
  entity?: { kind: RemoteKind; id: number }
}

export type History = Record<string, Seen>

/** How many things are kept. */
const KEEP = 500
/** Coming back sooner than this is the same visit. */
const REVISIT = 10 * 60_000

/** The ref of a person, experience, community or item. */
export function entityRef(kind: RemoteKind, id: number): string {
  switch (kind) {
    case "user":
      return `user:${id}`
    case "experience":
      return `game:${id}`
    case "community":
      return `group:${id}`
    case "item":
      return `item:${id}`
    case "bundle":
      return `bundle:${id}`
  }
}

type Described = Pick<Seen, "ref" | "thumb" | "open" | "entity">

const ENTITY_PATHS: [RegExp, (id: number) => Omit<Described, "ref">, (id: number) => string][] = [
  [
    /^\/users\/(\d+)$/,
    (id) => ({
      thumb: { kind: "headshot", id },
      open: { id: "user.open", arg: String(id) },
      entity: { kind: "user", id },
    }),
    (id) => `user:${id}`,
  ],
  [
    /^\/games\/(\d+)$/,
    (id) => ({ open: { id: "game.open", arg: String(id) }, entity: { kind: "experience", id } }),
    (id) => `game:${id}`,
  ],
  [
    /^\/communities\/(\d+)$/,
    (id) => ({
      thumb: { kind: "group_icon", id },
      open: { id: "group.open", arg: String(id) },
      entity: { kind: "community", id },
    }),
    (id) => `group:${id}`,
  ],
  [
    /^\/catalog\/(\d+)$/,
    (id) => ({
      thumb: { kind: "asset", id },
      open: { id: "market.open", arg: `item:${id}` },
      entity: { kind: "item", id },
    }),
    (id) => `item:${id}`,
  ],
  [
    /^\/bundles\/(\d+)$/,
    (id) => ({
      thumb: { kind: "bundle", id },
      open: { id: "market.open", arg: `bundle:${id}` },
      entity: { kind: "bundle", id },
    }),
    (id) => `bundle:${id}`,
  ],
  [
    /^\/passes\/(\d+)$/,
    (id) => ({ thumb: { kind: "game_pass", id }, open: { id: "market.open", arg: `pass:${id}` } }),
    (id) => `pass:${id}`,
  ],
]

/** What a path shows, or null for paths that aren't worth remembering. */
export function describeVisit(path: string): Described | null {
  const p = pathname(path).replace(/\/+$/, "") || "/"
  for (const [re, describe, ref] of ENTITY_PATHS) {
    const m = re.exec(p)
    if (m) {
      const id = Number(m[1])
      return { ref: ref(id), ...describe(id) }
    }
  }
  const section = /^\/settings\/([^/]+)/.exec(p)
  if (section) return { ref: `nav.settings:${section[1]}` }
  const screen = screens
    .filter((s) => p === s.path || (s.path !== "/" && p.startsWith(`${s.path}/`)))
    .sort((a, b) => b.path.length - a.path.length)[0]
  return screen ? { ref: screen.command } : null
}

/** `h` after looking at `seen`. Doesn't change `h`. Known details are kept unless `seen` has
 * newer ones. */
export function touch(h: History, seen: Partial<Seen> & { ref: string }, now = Date.now()): History {
  const prev = h[seen.ref]
  const again = prev && now - prev.t < REVISIT
  const next: Seen = {
    ...prev,
    ...Object.fromEntries(Object.entries(seen).filter(([, v]) => v !== undefined)),
    ref: seen.ref,
    n: (prev?.n ?? 0) + (again ? 0 : 1),
    t: now,
  }
  const out = { ...h, [seen.ref]: next }
  const keys = Object.keys(out)
  if (keys.length <= KEEP) return out
  const keep = keys.sort((a, b) => out[b].t - out[a].t).slice(0, KEEP)
  return Object.fromEntries(keep.map((k) => [k, out[k]]))
}

/** `h` with what is known about `ref` filled in, without counting a visit. */
export function describe(h: History, ref: string, details: Pick<Seen, "title" | "thumb">): History {
  const prev = h[ref]
  if (!prev) return h
  const title = details.title ?? prev.title
  const thumb = details.thumb ?? prev.thumb
  if (title === prev.title && thumb?.kind === prev.thumb?.kind && thumb?.id === prev.thumb?.id) return h
  return { ...h, [ref]: { ...prev, title, thumb } }
}

const DAY = 86_400_000

/** How much something done `age` ms ago still counts: half after ten days, never under a fifth. */
export function fade(age: number): number {
  return Math.max(0.2, 0.5 ** (Math.max(0, age) / (10 * DAY)))
}

/** Per visit, and at most. A thing looked at often and lately can beat a slightly better
 * match, but not a much better one. */
const SEEN = 12
const SEEN_MAX = 45

/** How much to lift a row about `ref` for having looked at it. */
export function seenBoost(h: History, ref: string, now = Date.now()): number {
  const s = h[ref]
  if (!s) return 0
  return Math.min(SEEN_MAX, SEEN * Math.log2(1 + s.n) * fade(now - s.t))
}

/** Things that can be opened again, newest first. */
export const openable = (h: History): Seen[] =>
  Object.values(h)
    .filter((s) => s.open && s.title)
    .sort((a, b) => b.t - a.t)

const storageKey = (account: string) => `bonsai:history:${account}`

export function loadHistory(account: string): History {
  try {
    const raw = localStorage.getItem(storageKey(account))
    const h = raw ? (JSON.parse(raw) as unknown) : null
    if (!h || typeof h !== "object" || Array.isArray(h)) return {}
    return Object.fromEntries(
      Object.entries(h as Record<string, Seen>).filter(
        ([k, v]) => v && typeof v === "object" && v.ref === k && typeof v.n === "number" && typeof v.t === "number",
      ),
    )
  } catch {
    return {}
  }
}

function saveHistory(account: string, h: History) {
  try {
    localStorage.setItem(storageKey(account), JSON.stringify(h))
  } catch {
    // Storage full or blocked: history just doesn't persist.
  }
}

// ── Store ──

const [histories, setHistories] = createStore<Record<string, History>>({})

/** The account's history, loaded from this machine on first use. */
export function historyOf(account: string): History {
  if (!histories[account]) setHistories(account, loadHistory(account))
  return histories[account]
}

function update(account: string, f: (h: History) => History) {
  const prev = historyOf(account)
  const next = f(prev)
  if (next === prev) return
  setHistories(account, next)
  saveHistory(account, next)
}

/** The account opened `path`. */
export function noteVisit(account: string, path: string, title?: string) {
  const d = describeVisit(path)
  if (d) update(account, (h) => touch(h, { ...d, title }))
}

/** A screen learned the name or picture of what `path` shows. Counts as the visit when the
 * screen got there first; a visit is counted once either way. */
export function noteDetails(account: string, path: string, details: Pick<Seen, "title" | "thumb">) {
  const d = describeVisit(path)
  if (d) update(account, (h) => (h[d.ref] ? describe(h, d.ref, details) : touch(h, { ...d, ...details })))
}

/** The account did something with `ref` from the command bar (joined a game, messaged
 * someone). Details are used when it wasn't seen before. */
export function noteUse(account: string, seen: Partial<Seen> & { ref: string }) {
  update(account, (h) => touch(h, h[seen.ref] ? { ref: seen.ref } : seen))
}

/** The account no longer wants `ref` remembered. */
export function forgetSeen(account: string, ref: string) {
  update(account, (h) => {
    if (!h[ref]) return h
    const { [ref]: _, ...rest } = h
    return rest
  })
}

import { createStore, reconcile } from "solid-js/store"
import type { IconName } from "~/lib/ui"

/**
 * The tab strip's model. A tab is a place in the app (a router path, with its query). Screen
 * paths (`/friends`) load in the current tab, like following a link; entity paths (`/games/1`,
 * `/users/1`) get a tab of their own, or focus the tab already showing them.
 *
 * Only the active tab's screen is mounted, so an open tab costs a few bytes, not a page.
 *
 * The functions below are pure so they can be tested; `tabs` and its actions wrap them in a
 * store the shell renders. Updates go through `reconcile` keyed by `key`, so a tab keeps its
 * DOM node across changes and only the tabs that changed re-render.
 */

export interface Tab {
  key: string
  path: string
  /** Pinned tabs sit first, show only an icon and keep their place when screens load. */
  pinned?: boolean
  /** The tab that was active when this one opened. Closing this tab goes back there, until
   * the user picks another tab. */
  opener?: string
}

export interface ClosedTab {
  path: string
  index: number
  pinned?: boolean
}

export interface TabsState {
  tabs: Tab[]
  active: string
  /** Most recently closed last, for Ctrl+Shift+T. */
  closed: ClosedTab[]
  /** Titles screens set for entity paths, like a game's name. Keyed by pathname. */
  titles: Record<string, string>
  /** The last tab opened in the background from the active tab, so the next one lands after
   * it and a run of Ctrl+clicks keeps its order. */
  lastOpened?: string
  next: number
}

const CLOSED_LIMIT = 25

/** The path without its query or hash. */
export function pathname(path: string): string {
  return path.split(/[?#]/, 1)[0]
}

/** Paths that open in their own tab. */
export function isEntityPath(path: string): boolean {
  return /^\/(games|users|catalog|bundles|passes|communities|themes)\/[^/?#]+/.test(path)
}

/** The first segment, which names the screen: `/settings/privacy` → `settings`. */
function screenOf(path: string): string {
  return pathname(path).split("/")[1] ?? ""
}

export function initialTabs(path = "/"): TabsState {
  return { tabs: [{ key: "t0", path }], active: "t0", closed: [], titles: {}, next: 1 }
}

/** A tab as it is saved between launches. */
export interface SavedTab {
  path: string
  pinned: boolean
}

/** The tabs to save, and the index of the active one. */
export function savedTabs(s: TabsState): { tabs: SavedTab[]; active: number } {
  return {
    tabs: s.tabs.map((t) => ({ path: t.path, pinned: !!t.pinned })),
    active: Math.max(0, indexOf(s, s.active)),
  }
}

/** Fresh tabs from saved ones, pinned first. Falls back to one tab on Home when none are left. */
export function restoredTabs(saved: SavedTab[], active: number): TabsState {
  const order = [...saved.filter((t) => t.pinned), ...saved.filter((t) => !t.pinned)]
  if (order.length === 0) return initialTabs()
  const tabs: Tab[] = order.map((t, i) => ({ key: `t${i}`, path: t.path, pinned: t.pinned || undefined }))
  const activeAt = order.indexOf(saved[active] ?? saved[0])
  return { tabs, active: tabs[Math.max(0, activeAt)].key, closed: [], titles: {}, next: tabs.length }
}

export function activeTab(s: TabsState): Tab | undefined {
  return s.tabs.find((t) => t.key === s.active)
}

function indexOf(s: TabsState, key: string) {
  return s.tabs.findIndex((t) => t.key === key)
}

/** Number of pinned tabs. They always come first. */
function pinnedCount(tabs: Tab[]) {
  let n = 0
  while (n < tabs.length && tabs[n].pinned) n++
  return n
}

function insert(tabs: Tab[], at: number, tab: Tab): Tab[] {
  return [...tabs.slice(0, at), tab, ...tabs.slice(at)]
}

/** Makes `key` active. Picking a tab forgets where tabs were opened from, like a browser. */
export function activate(s: TabsState, key: string): TabsState {
  if (s.active === key || indexOf(s, key) < 0) return s
  return {
    ...s,
    active: key,
    lastOpened: undefined,
    tabs: s.tabs.map((t) => (t.opener ? { ...t, opener: undefined } : t)),
  }
}

/** The router moved to `path`. Returns the new state. */
export function visit(s: TabsState, path: string): TabsState {
  const current = activeTab(s)
  if (current?.path === path) return s
  if (!current) return open(s, path)
  const entity = isEntityPath(path)
  if (entity) {
    const existing = s.tabs.find((t) => pathname(t.path) === pathname(path))
    if (existing) {
      const next = activate(s, existing.key)
      // Same entity, new query (a sub-tab of the page): keep it in that tab.
      return existing.path === path ? next : setPath(next, existing.key, path)
    }
    return open(s, path, { opener: current.key })
  }
  // A pinned tab keeps its screen. Moving within it (settings sections, a search) is fine;
  // going elsewhere opens a tab.
  if (current.pinned && screenOf(current.path) !== screenOf(path)) return open(s, path, { opener: current.key })
  return setPath(s, current.key, path)
}

function setPath(s: TabsState, key: string, path: string): TabsState {
  return { ...s, tabs: s.tabs.map((t) => (t.key === key ? { ...t, path } : t)) }
}

export interface OpenOptions {
  /** Open without switching to it. */
  background?: boolean
  opener?: string
}

/** Opens `path` in a new tab after the active one. A background entity tab that is already
 * open is left alone. */
export function open(s: TabsState, path: string, opts: OpenOptions = {}): TabsState {
  if (opts.background) {
    if (isEntityPath(path) && s.tabs.some((t) => pathname(t.path) === pathname(path))) return s
  }
  const key = `t${s.next}`
  const after = opts.background && s.lastOpened && indexOf(s, s.lastOpened) >= 0 ? s.lastOpened : s.active
  const at = Math.max(indexOf(s, after) + 1, pinnedCount(s.tabs))
  const tab: Tab = opts.opener ? { key, path, opener: opts.opener } : { key, path }
  const tabs = insert(s.tabs, at, tab)
  if (opts.background) return { ...s, tabs, lastOpened: key, next: s.next + 1 }
  return { ...s, tabs, active: key, lastOpened: undefined, next: s.next + 1 }
}

/** The tab to show when the active tab `i` closes: a sibling opened from the same tab, else
 * the tab that opened it, else the right neighbor, else the left one. */
function successor(s: TabsState, i: number, remaining: Tab[]): string {
  const gone = s.tabs[i]
  if (gone.opener) {
    const right = s.tabs[i + 1]
    if (right?.opener === gone.opener) return right.key
    const left = s.tabs[i - 1]
    if (left?.opener === gone.opener) return left.key
    if (remaining.some((t) => t.key === gone.opener)) return gone.opener
  }
  return remaining[Math.min(i, remaining.length - 1)].key
}

/** Closes the tabs in `keys`. Closing the last tab leaves a Home tab. */
export function closeMany(s: TabsState, keys: Iterable<string>): TabsState {
  const drop = new Set(keys)
  const gone = s.tabs.flatMap((t, index) => (drop.has(t.key) ? [{ path: t.path, index, pinned: t.pinned }] : []))
  if (gone.length === 0) return s
  const closed = [
    ...s.closed,
    ...gone.map(({ path, index, pinned }) => (pinned ? { path, index, pinned } : { path, index })),
  ].slice(-CLOSED_LIMIT)
  const tabs = s.tabs.filter((t) => !drop.has(t.key))
  if (tabs.length === 0) {
    const key = `t${s.next}`
    return tidy({ ...s, tabs: [{ key, path: "/" }], active: key, closed, lastOpened: undefined, next: s.next + 1 })
  }
  let active = s.active
  if (drop.has(s.active)) {
    const i = indexOf(s, s.active)
    active = successor(s, i, tabs)
    // A sibling or opener may itself be closing; fall back to the nearest survivor.
    if (!tabs.some((t) => t.key === active)) {
      const right = s.tabs.slice(i).find((t) => !drop.has(t.key))
      active = (right ?? tabs[tabs.length - 1]).key
    }
  }
  const forget = (k?: string) => (k && drop.has(k) ? undefined : k)
  return tidy({
    ...s,
    tabs: tabs.map((t) => (t.opener && drop.has(t.opener) ? { ...t, opener: undefined } : t)),
    active,
    closed,
    lastOpened: forget(s.lastOpened),
  })
}

export function close(s: TabsState, key: string): TabsState {
  return closeMany(s, [key])
}

/** Closes every tab but `key` and the pinned ones. */
export function closeOthers(s: TabsState, key: string): TabsState {
  const keys = s.tabs.filter((t) => t.key !== key && !t.pinned).map((t) => t.key)
  return activate(closeMany(s, keys), key)
}

/** Closes the unpinned tabs right of `key`. */
export function closeRight(s: TabsState, key: string): TabsState {
  const i = indexOf(s, key)
  if (i < 0) return s
  return closeMany(
    s,
    s.tabs
      .slice(i + 1)
      .filter((t) => !t.pinned)
      .map((t) => t.key),
  )
}

/** Drops titles no open or recently closed tab uses, so they don't pile up. */
function tidy(s: TabsState): TabsState {
  const used = new Set([...s.tabs.map((t) => pathname(t.path)), ...s.closed.map((c) => pathname(c.path))])
  const stale = Object.keys(s.titles).filter((p) => !used.has(p))
  if (stale.length === 0) return s
  const titles = { ...s.titles }
  for (const p of stale) delete titles[p]
  return { ...s, titles }
}

/** Activates the tab `step` places away, wrapping. */
export function step(s: TabsState, by: 1 | -1): TabsState {
  const i = indexOf(s, s.active)
  const n = s.tabs.length
  return activate(s, s.tabs[(i + by + n) % n].key)
}

/** Activates tab `n` (1-based); `Infinity` is the last tab. Does nothing past the end. */
export function goTo(s: TabsState, n: number): TabsState {
  const t = n === Number.POSITIVE_INFINITY ? s.tabs.at(-1) : s.tabs[n - 1]
  return t ? activate(s, t.key) : s
}

/** Moves `key` to index `to`. Pinned tabs stay among the pinned, others after them. */
export function move(s: TabsState, key: string, to: number): TabsState {
  const i = indexOf(s, key)
  if (i < 0) return s
  const tab = s.tabs[i]
  const rest = s.tabs.filter((t) => t.key !== key)
  const pins = pinnedCount(rest)
  const lo = tab.pinned ? 0 : pins
  const hi = tab.pinned ? pins : rest.length
  const at = Math.min(hi, Math.max(lo, to))
  if (at === i) return s
  return { ...s, tabs: insert(rest, at, tab) }
}

/** Pins or unpins `key`. Pinning moves it to the end of the pinned tabs; unpinning, to the
 * start of the others. */
export function togglePin(s: TabsState, key: string): TabsState {
  const i = indexOf(s, key)
  if (i < 0) return s
  const tab = s.tabs[i]
  const rest = s.tabs.filter((t) => t.key !== key)
  const changed: Tab = tab.pinned ? { key: tab.key, path: tab.path } : { key: tab.key, path: tab.path, pinned: true }
  return { ...s, tabs: insert(rest, pinnedCount(rest), changed) }
}

/** Reopens the most recently closed tab where it was. */
export function reopen(s: TabsState): TabsState {
  const last = s.closed.at(-1)
  if (last === undefined) return s
  const base = { ...s, closed: s.closed.slice(0, -1) }
  if (isEntityPath(last.path)) {
    const existing = s.tabs.find((t) => pathname(t.path) === pathname(last.path))
    if (existing) return activate(base, existing.key)
  }
  const key = `t${s.next}`
  const pins = pinnedCount(s.tabs)
  const at = last.pinned ? Math.min(last.index, pins) : Math.min(Math.max(last.index, pins), s.tabs.length)
  const tab: Tab = last.pinned ? { key, path: last.path, pinned: true } : { key, path: last.path }
  return { ...base, tabs: insert(s.tabs, at, tab), active: key, lastOpened: undefined, next: s.next + 1 }
}

/** A copy of `key` right after it. */
export function duplicate(s: TabsState, key: string): TabsState {
  const i = indexOf(s, key)
  if (i < 0) return s
  const copy: Tab = { key: `t${s.next}`, path: s.tabs[i].path }
  const at = Math.max(i + 1, pinnedCount(s.tabs))
  return { ...s, tabs: insert(s.tabs, at, copy), active: copy.key, lastOpened: undefined, next: s.next + 1 }
}

// ── Store ──

const [tabs, setTabs] = createStore<TabsState>(initialTabs())

export { tabs }

const apply = (f: (s: TabsState) => TabsState) => {
  const next = f(tabs)
  if (next !== tabs) setTabs(reconcile(next, { key: "key" }))
}

/** Each takes a tab key; without one it acts on the active tab. */
export const tabActions = {
  visit: (path: string) => apply((s) => visit(s, path)),
  open: (path: string, opts?: OpenOptions) => apply((s) => open(s, path, opts)),
  activate: (key: string) => apply((s) => activate(s, key)),
  close: (key = tabs.active) => apply((s) => close(s, key)),
  closeOthers: (key = tabs.active) => apply((s) => closeOthers(s, key)),
  closeRight: (key = tabs.active) => apply((s) => closeRight(s, key)),
  togglePin: (key = tabs.active) => apply((s) => togglePin(s, key)),
  duplicate: (key = tabs.active) => apply((s) => duplicate(s, key)),
  move: (key: string, to: number) => apply((s) => move(s, key, to)),
  /** Moves a tab one place left or right. */
  shift: (by: 1 | -1, key = tabs.active) => apply((s) => move(s, key, indexOf(s, key) + by)),
  step: (by: 1 | -1) => apply((s) => step(s, by)),
  goTo: (n: number) => apply((s) => goTo(s, n)),
  reopen: () => apply((s) => reopen(s)),
  /** Replaces every tab, for opening the saved ones at launch. */
  restore: (saved: SavedTab[], active: number) => apply(() => restoredTabs(saved, active)),
}

let describe: (path: string) => { title: string; icon?: IconName } = () => ({ title: "Tab" })

/** Sets how a path is named before its screen sets a title. The shell passes its screen list. */
export function setTabDescriber(f: typeof describe) {
  describe = f
}

/** The title and icon of a tab showing `path`. */
export function tabLabel(path: string): { title: string; icon?: IconName } {
  const d = describe(path)
  return { title: tabs.titles[pathname(path)] ?? d.title, icon: d.icon }
}

/** Sets the title of the tab showing `path`. Screens call this once they know a name. */
export function setTabTitle(path: string, title: string) {
  const p = pathname(path)
  if (tabs.titles[p] !== title) setTabs("titles", p, title)
}

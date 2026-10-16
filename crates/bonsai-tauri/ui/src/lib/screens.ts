import type { SidebarView } from "~/lib/ipc/bindings"
import { shell } from "~/lib/store"
import { pathname } from "~/lib/tabs"
import type { IconName } from "~/lib/ui"

/** Sidebar groups, top to bottom. Rows move within their group, not between groups. */
export const groups = ["start", "social", "items", "economy"] as const
export type Group = (typeof groups)[number]

/** A top-level screen: a sidebar row, a `nav.*` command and a route. */
export interface Screen {
  command: string
  label: string
  icon: IconName
  path: string
  /** Shown in the sidebar. Settings sits at the bottom, Chat and Notifications live in the
   * drawer and top bar. */
  sidebar: "top" | "bottom" | false
  /** Where a `top` row sits. */
  group?: Group
  /** Hides the sidebar row while false, e.g. Values while values are off. */
  shown?: () => boolean
}

export const screens: Screen[] = [
  { command: "nav.home", label: "Home", icon: "home", path: "/", sidebar: "top", group: "start" },
  { command: "nav.friends", label: "Friends", icon: "friends", path: "/friends", sidebar: "top", group: "social" },
  { command: "nav.discover", label: "Discover", icon: "discover", path: "/discover", sidebar: "top", group: "start" },
  { command: "nav.avatar", label: "Avatar", icon: "avatar", path: "/avatar", sidebar: "top", group: "items" },
  {
    command: "nav.marketplace",
    label: "Marketplace",
    icon: "marketplace",
    path: "/marketplace",
    sidebar: "top",
    group: "items",
  },
  {
    command: "nav.inventory",
    label: "Inventory",
    icon: "inventory",
    path: "/inventory",
    sidebar: "top",
    group: "items",
  },
  { command: "nav.trades", label: "Trades", icon: "trades", path: "/trades", sidebar: "top", group: "economy" },
  {
    command: "nav.values",
    label: "Values",
    icon: "values",
    path: "/values",
    sidebar: "top",
    group: "economy",
    shown: () => shell.itemValues,
  },
  {
    command: "nav.transactions",
    label: "Transactions",
    icon: "dollar",
    path: "/transactions",
    sidebar: "top",
    group: "economy",
  },
  { command: "nav.messages", label: "Messages", icon: "messages", path: "/messages", sidebar: "top", group: "social" },
  {
    command: "nav.communities",
    label: "Communities",
    icon: "communities",
    path: "/communities",
    sidebar: "top",
    group: "social",
  },
  { command: "nav.chat", label: "Chat", icon: "chat", path: "/chat", sidebar: false },
  { command: "nav.notifications", label: "Notifications", icon: "bell", path: "/notifications", sidebar: false },
  { command: "nav.settings", label: "Settings", icon: "settings", path: "/settings", sidebar: "bottom" },
  { command: "nav.components", label: "Components", icon: "grid", path: "/components", sidebar: false },
  { command: "nav.overview", label: "Overview", icon: "grid", path: "/overview", sidebar: false },
]

/** The sidebar name of a screen, as the `view.sidebar_*` commands take it: `nav.friends` → `friends`. */
export const screenKey = (s: Screen) => s.command.slice("nav.".length)

/** Screens that can sit in the top of the sidebar, which the user can hide and reorder. */
export const sidebarScreens = () => screens.filter((s) => s.sidebar === "top")

/** Every top-of-sidebar screen by group, in the saved order, hidden ones included. Screens the
 * saved order doesn't name keep their default place after the ones it does. */
export function arrange(all: Screen[], prefs: SidebarView): Screen[][] {
  const rank = (s: Screen) => {
    const i = prefs.order.indexOf(screenKey(s))
    return i < 0 ? prefs.order.length + all.indexOf(s) : i
  }
  return groups.map((g) =>
    all.filter((s) => s.sidebar === "top" && (s.group ?? "start") === g).sort((a, b) => rank(a) - rank(b)),
  )
}

/** Whether a screen's row shows: not hidden by the user and not switched off (Values). */
export const isShown = (s: Screen, prefs: SidebarView) => !prefs.hidden.includes(screenKey(s)) && (s.shown?.() ?? true)

/** The rows the sidebar shows, by group. Empty groups are dropped. */
export function sidebarGroups(all: Screen[], prefs: SidebarView): Screen[][] {
  return arrange(all, prefs)
    .map((g) => g.filter((s) => isShown(s, prefs)))
    .filter((g) => g.length > 0)
}

/** The saved order after dragging `key` to slot `to` among the shown rows of its group, or null
 * when nothing changes. Hidden rows keep their place relative to their neighbours. */
export function moveScreen(all: Screen[], prefs: SidebarView, key: string, to: number): string[] | null {
  const arranged = arrange(all, prefs)
  const group = arranged.find((g) => g.some((s) => screenKey(s) === key))
  if (!group) return null
  const shown = group.filter((s) => isShown(s, prefs))
  const from = shown.findIndex((s) => screenKey(s) === key)
  if (from < 0) return null
  const slot = Math.max(0, Math.min(shown.length - 1, to))
  if (slot === from) return null
  const moving = shown[from]
  const target = shown[slot]
  group.splice(group.indexOf(moving), 1)
  group.splice(group.indexOf(target) + (slot > from ? 1 : 0), 0, moving)
  return arranged.flat().map(screenKey)
}

/** The saved order after moving `key` one row up (-1) or down (1) in its group, or null at the
 * end. A shown screen steps over hidden rows to pass the next shown one (or to the end when there
 * is none), so every press changes the sidebar; a hidden one moves one row. */
export function nudgeScreen(all: Screen[], prefs: SidebarView, key: string, by: -1 | 1): string[] | null {
  const arranged = arrange(all, prefs)
  const group = arranged.find((g) => g.some((s) => screenKey(s) === key))
  if (!group) return null
  const from = group.findIndex((s) => screenKey(s) === key)
  const moving = group[from]
  let to = from + by
  if (isShown(moving, prefs)) while (to >= 0 && to < group.length && !isShown(group[to], prefs)) to += by
  to = Math.max(0, Math.min(group.length - 1, to))
  if (to === from) return null
  group.splice(from, 1)
  group.splice(to, 0, moving)
  return arranged.flat().map(screenKey)
}

const entityTitles: [RegExp, string, IconName][] = [
  [/^\/games\//, "Experience", "game"],
  [/^\/users\//, "Profile", "user"],
  [/^\/(catalog|bundles)\//, "Item", "marketplace"],
  [/^\/passes\//, "Pass", "pass"],
  [/^\/communities\/./, "Community", "communities"],
  [/^\/search$/, "Search", "search"],
]

/** Title and icon for a tab showing `path`, before the screen sets a better title. */
export function describePath(full: string): { title: string; icon: IconName } {
  const path = pathname(full)
  for (const [re, title, icon] of entityTitles) if (re.test(path)) return { title, icon }
  const s = screens
    .filter((s) => path === s.path || (s.path !== "/" && path.startsWith(`${s.path}/`)))
    .sort((a, b) => b.path.length - a.path.length)[0]
  return s ? { title: s.label, icon: s.icon } : { title: "Home", icon: "home" }
}

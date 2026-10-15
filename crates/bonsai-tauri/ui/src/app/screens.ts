import { shell } from "~/lib/store"
import { pathname } from "~/lib/tabs"
import type { IconName } from "~/lib/ui"

/** A top-level screen: a sidebar row, a `nav.*` command and a route. */
export interface Screen {
  command: string
  label: string
  icon: IconName
  path: string
  /** Shown in the sidebar. Settings sits at the bottom, Chat and Notifications live in the
   * drawer and top bar. */
  sidebar: "top" | "bottom" | false
  /** Hides the sidebar row while false, e.g. Values while values are off. */
  shown?: () => boolean
}

export const screens: Screen[] = [
  { command: "nav.home", label: "Home", icon: "home", path: "/", sidebar: "top" },
  { command: "nav.friends", label: "Friends", icon: "friends", path: "/friends", sidebar: "top" },
  { command: "nav.discover", label: "Discover", icon: "discover", path: "/discover", sidebar: "top" },
  { command: "nav.avatar", label: "Avatar", icon: "avatar", path: "/avatar", sidebar: "top" },
  { command: "nav.marketplace", label: "Marketplace", icon: "marketplace", path: "/marketplace", sidebar: "top" },
  { command: "nav.inventory", label: "Inventory", icon: "inventory", path: "/inventory", sidebar: "top" },
  { command: "nav.trades", label: "Trades", icon: "trades", path: "/trades", sidebar: "top" },
  {
    command: "nav.values",
    label: "Values",
    icon: "values",
    path: "/values",
    sidebar: "top",
    shown: () => shell.itemValues,
  },
  { command: "nav.transactions", label: "Transactions", icon: "dollar", path: "/transactions", sidebar: "top" },
  { command: "nav.messages", label: "Messages", icon: "messages", path: "/messages", sidebar: "top" },
  { command: "nav.communities", label: "Communities", icon: "communities", path: "/communities", sidebar: "top" },
  { command: "nav.chat", label: "Chat", icon: "chat", path: "/chat", sidebar: false },
  { command: "nav.notifications", label: "Notifications", icon: "bell", path: "/notifications", sidebar: false },
  { command: "nav.settings", label: "Settings", icon: "settings", path: "/settings", sidebar: "bottom" },
  { command: "nav.components", label: "Components", icon: "grid", path: "/components", sidebar: false },
  { command: "nav.overview", label: "Overview", icon: "grid", path: "/overview", sidebar: false },
]

const entityTitles: [RegExp, string, IconName][] = [
  [/^\/games\//, "Experience", "game"],
  [/^\/users\//, "Profile", "user"],
  [/^\/(catalog|bundles)\//, "Item", "marketplace"],
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

import { screens } from "~/lib/screens"
import type { IconName } from "~/lib/ui"
import type { PaletteEntry } from "./palette"

/** How a command bar row looks and what Enter says it does, from its command id. */

/** Icons by the part of a command id before the dot. */
const BY_AREA: Record<string, IconName> = {
  account: "user",
  app: "info",
  avatar: "avatar",
  bookmark: "bookmark",
  chat: "chat",
  friends: "friends",
  game: "game",
  group: "communities",
  help: "info",
  hub: "plugin",
  image: "image",
  inventory: "inventory",
  list: "list",
  market: "marketplace",
  message: "messages",
  notifications: "bell",
  playlist: "playlist",
  plugin: "plugin",
  plugins: "plugin",
  privacy: "lock",
  security: "shield",
  server: "server",
  settings: "settings",
  shortcuts: "keyboard",
  tab: "layout",
  theme: "palette",
  trade: "trades",
  transactions: "transactions",
  user: "user",
  vault: "lock",
  view: "eye",
  wiki: "wiki",
  window: "layout",
}

const screenIcons = new Map(screens.map((s) => [s.command, s.icon]))

/** The icon for a row without a picture. */
export function rowIcon(e: PaletteEntry): IconName {
  if (e.fill !== undefined) return "clock"
  const screen = screenIcons.get(e.id)
  if (screen) return screen
  if (e.id.startsWith("nav.settings")) return "settings"
  return BY_AREA[e.id.split(".")[0]] ?? "code"
}

/** What Enter does with a row, in one word. */
export function enterVerb(e: PaletteEntry | undefined): string {
  if (!e) return "Open"
  if (e.fill !== undefined) return "Search"
  if (e.ask) return "Choose"
  const id = e.id
  if (id === "game.join" || id === "game.join_as") return "Join"
  if (id.startsWith("nav.") || id === "tab.switch") return "Go"
  if (id.endsWith(".open") || id === "market.preview" || e.entity) return "Open"
  if (id === "theme.apply" || id === "settings.set") return "Apply"
  return "Run"
}

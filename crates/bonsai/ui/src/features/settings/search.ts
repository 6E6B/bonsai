import { filterEntries } from "~/lib/commands/filter"
import type { PrefView } from "~/lib/ipc/bindings"
import { type SettingsSectionId, settingsSection } from "~/lib/sections"

/** A row the settings search can find. `setting` is the row's `data-setting`, when it has one. */
export interface SettingsEntry {
  section: SettingsSectionId
  setting?: string
  title: string
  words: string
}

/** Rows that aren't in bonsai-core's settings list: Roblox account settings, actions and pages. */
export const extraEntries: SettingsEntry[] = [
  {
    section: "desktop",
    setting: "updates",
    title: "Bonsai updates",
    words: "update upgrade version release download install restart",
  },
  { section: "profile", title: "Display name", words: "name rename" },
  { section: "profile", title: "Username", words: "name rename" },
  { section: "profile", title: "Email", words: "contact verify" },
  { section: "profile", title: "Phone", words: "contact verify number" },
  { section: "profile", title: "Birthday", words: "age birthdate verified" },
  { section: "profile", title: "Premium", words: "membership subscription robux" },
  { section: "security", title: "Password", words: "sign in login" },
  { section: "security", title: "Two-step verification", words: "2fa 2sv authenticator code" },
  { section: "security", title: "Account PIN", words: "lock unlock" },
  { section: "security", title: "Quick sign-in", words: "code device" },
  { section: "security", title: "Signed-in sessions", words: "sign out devices logout" },
  { section: "privacy", title: "Who can message, chat and join", words: "privacy contact trade inventory" },
  { section: "privacy", title: "Blocked users", words: "block unblock" },
  { section: "servers", title: "Private servers", words: "vip server" },
  { section: "appearance", setting: "themes", title: "Themes", words: "colors palette dark light" },
  { section: "appearance", title: "Fonts", words: "font typeface mono" },
  { section: "background", title: "Background picture", words: "wallpaper image" },
  { section: "background", title: "Background effects", words: "blur dim tint vignette scanlines grain" },
  { section: "layout", title: "Sidebar screens", words: "sidebar screens order hide show" },
  { section: "shortcuts", title: "Keyboard shortcuts", words: "keys bindings hotkeys commands" },
  { section: "accounts", title: "Signed-in accounts", words: "add remove cookie switch" },
  { section: "data", setting: "cache", title: "Cache", words: "clear disk storage" },
  { section: "data", setting: "settings_file", title: "Settings file", words: "export import backup restore move" },
  { section: "data", setting: "reset_all", title: "Reset all settings", words: "defaults restore factory" },
  { section: "data", setting: "diagnostics", title: "Diagnostics", words: "bug report version debug support" },
  { section: "developer", title: "Component gallery", words: "components ui" },
]

/** The settings and rows matching `query`, settings from bonsai-core first. */
export function searchSettings(prefs: PrefView[], query: string): SettingsEntry[] {
  if (!query.trim()) return []
  const entries: SettingsEntry[] = [
    ...prefs.map((p) => ({
      section: settingsSection(p.section),
      setting: p.key,
      title: p.title,
      words: `${p.description} ${p.keywords.join(" ")} ${p.section}`,
    })),
    ...extraEntries.map((e) => ({ ...e, words: `${e.words} ${e.section}` })),
  ]
  return filterEntries(entries, query)
}

/** Where a search result goes. */
export const entryPath = (e: SettingsEntry) => `/settings/${e.section}${e.setting ? `/${e.setting}` : ""}`

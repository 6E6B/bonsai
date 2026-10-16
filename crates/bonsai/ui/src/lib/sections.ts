import type { IconName } from "~/lib/ui"

/** Where a settings section is saved: on Roblox for the active account, or on this computer. */
export const settingsGroups = [
  { value: "account", label: "Roblox account" },
  { value: "app", label: "This app" },
] as const

export type SettingsGroup = (typeof settingsGroups)[number]["value"]

/** Settings sections, in order. `nav.settings` takes one of these values. */
export const settingsSections = [
  { value: "profile", label: "Profile", icon: "user", group: "account" },
  { value: "security", label: "Security", icon: "shield", group: "account" },
  { value: "privacy", label: "Privacy", icon: "eye", group: "account" },
  { value: "servers", label: "Private servers", icon: "server", group: "account" },
  { value: "general", label: "General", icon: "sliders", group: "app" },
  { value: "appearance", label: "Appearance", icon: "palette", group: "app" },
  { value: "background", label: "Background", icon: "image", group: "app" },
  { value: "layout", label: "Layout", icon: "layout", group: "app" },
  { value: "notifications", label: "Notifications", icon: "bell", group: "app" },
  { value: "shortcuts", label: "Keyboard", icon: "keyboard", group: "app" },
  { value: "desktop", label: "Desktop", icon: "devices", group: "app" },
  { value: "accounts", label: "Accounts on this computer", icon: "friends", group: "app" },
  { value: "data", label: "Data and storage", icon: "database", group: "app" },
  { value: "developer", label: "Developer", icon: "code", group: "app" },
] as const satisfies readonly { value: string; label: string; icon: IconName; group: SettingsGroup }[]

export type SettingsSectionId = (typeof settingsSections)[number]["value"]

/** Names sections had before they were split up, so old links and shortcuts still land. */
const aliases: Record<string, SettingsSectionId> = {
  account: "profile",
  advanced: "general",
}

/** The section a `nav.settings` argument or `/settings/:section` names. Unknown names open the
 * first section. */
export function settingsSection(name: string | undefined): SettingsSectionId {
  const n = (name ?? "").trim().toLowerCase()
  const hit = settingsSections.find((s) => s.value === n)
  return hit?.value ?? aliases[n] ?? settingsSections[0].value
}

/** Settings sections, in order. `nav.settings` takes one of these values. */
export const settingsSections = [
  { value: "account", label: "Account" },
  { value: "accounts", label: "Accounts on this computer" },
  { value: "privacy", label: "Privacy" },
  { value: "servers", label: "Private servers" },
  { value: "notifications", label: "Notifications" },
  { value: "appearance", label: "Appearance" },
  { value: "shortcuts", label: "Shortcuts" },
  { value: "advanced", label: "Advanced" },
] as const

import { describe, expect, it } from "vitest"
import { settingsGroups, settingsSection, settingsSections } from "./sections"

describe("settingsSection", () => {
  it("finds a section by name, ignoring case and spaces", () => {
    expect(settingsSection("privacy")).toBe("privacy")
    expect(settingsSection(" Layout ")).toBe("layout")
  })

  it("maps names from before the split", () => {
    expect(settingsSection("account")).toBe("profile")
    expect(settingsSection("advanced")).toBe("general")
  })

  it("opens the first section for unknown or missing names", () => {
    expect(settingsSection(undefined)).toBe("profile")
    expect(settingsSection("nope")).toBe("profile")
  })

  it("puts every section in a known group", () => {
    const groups = settingsGroups.map((g) => g.value as string)
    for (const s of settingsSections) expect(groups).toContain(s.group)
  })
})

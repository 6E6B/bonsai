import { describe, expect, it } from "vitest"
import type { PrefView } from "~/lib/ipc/bindings"
import { entryPath, searchSettings } from "./search"

const pref = (key: string, section: string, title: string, keywords: string[] = []): PrefView => ({
  key,
  section,
  title,
  description: "",
  keywords,
  kind: "bool",
  options: [],
  min: 0,
  max: 0,
  step: 0,
  placeholder: "",
  value: "on",
  default: "on",
})

describe("searchSettings", () => {
  const prefs = [
    pref("notify.trades", "notifications", "Trade offers", ["trade"]),
    pref("density", "appearance", "Density"),
  ]

  it("finds settings by title, keyword and section", () => {
    expect(searchSettings(prefs, "trade")[0]).toMatchObject({ setting: "notify.trades", section: "notifications" })
    expect(searchSettings(prefs, "notifications").map((e) => e.setting)).toContain("notify.trades")
  })

  it("finds rows outside the settings list", () => {
    expect(searchSettings(prefs, "2fa")[0]).toMatchObject({ section: "security", title: "Two-step verification" })
  })

  it("returns nothing for an empty query", () => {
    expect(searchSettings(prefs, "  ")).toEqual([])
  })

  it("links to the row when it has an id", () => {
    expect(entryPath({ section: "appearance", setting: "density", title: "", words: "" })).toBe(
      "/settings/appearance/density",
    )
    expect(entryPath({ section: "profile", title: "", words: "" })).toBe("/settings/profile")
  })
})

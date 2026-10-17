import { describe, expect, it } from "vitest"
import { isThemeCode, isThemeFile, summarize } from "./import"

describe("summarize", () => {
  it("lists skipped values and low-contrast pairs", () => {
    const doc = JSON.stringify({
      schema: 1,
      id: "x",
      base: "dark",
      name: "X",
      seeds: { background: "#000000", foreground: "#111111" },
      tokens: { "color-nope": "#fff", "color-accent": "red" },
    })
    const s = summarize(doc)
    if (!s.ok) throw new Error(s.error)
    expect(s.theme.name).toBe("X")
    expect(s.skipped).toEqual(["color-nope: unknown token", "color-accent: bad value"])
    expect(s.failing.map((c) => c.fg)).toContain("color-fg")
    expect(s.checked).toBeGreaterThanOrEqual(s.failing.length)
  })

  it("passes a readable theme and refuses what isn't one", () => {
    const s = summarize(JSON.stringify({ schema: 1, id: "y", base: "light" }))
    expect(s.ok && s.failing).toEqual([])
    expect(summarize("{")).toEqual({ ok: false, error: "Not a theme." })
    expect(summarize("{}")).toEqual({ ok: false, error: "Not a theme." })
  })
})

describe("isThemeCode / isThemeFile", () => {
  it("matches codes and theme files only", () => {
    expect(isThemeCode("  bonsai-theme:1:abc")).toBe(true)
    expect(isThemeCode("hello")).toBe(false)
    expect(isThemeFile("/home/a/Nord.BonsaiTheme")).toBe(true)
    expect(isThemeFile("/home/a/nord.json")).toBe(false)
  })
})

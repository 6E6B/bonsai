import { describe, expect, it } from "vitest"
import { colorOf, fixValue, PAIRS, runChecks } from "./checks"
import { contrast, hex, toHex } from "./color"
import { DEFAULT_SEEDS } from "./derive"
import { resolve, type Theme } from "./theme"

const theme = (over: Partial<Theme["seeds"]> = {}, tokens: Record<string, string> = {}): Theme => ({
  schema: 1,
  id: "t",
  name: "T",
  author: "",
  version: "1.0.0",
  base: "dark",
  seeds: { ...DEFAULT_SEEDS.dark, ...over },
  tokens,
})

describe("colorOf", () => {
  it("follows references and flattens alpha over the page", () => {
    const t = { "color-canvas": "#000000", "color-a": "var(--color-b)", "color-b": "#ffffff80" }
    expect(toHex(colorOf(t, "color-a") ?? hex("#123"))).toBe("#808080")
  })
  it("is undefined for a missing token or a cycle", () => {
    expect(colorOf({}, "color-x")).toBeUndefined()
    expect(colorOf({ "color-a": "var(--color-a)" }, "color-a")).toBeUndefined()
  })
})

describe("runChecks", () => {
  it("passes the default dark theme", () => {
    expect(runChecks(resolve(theme())).every((c) => c.ok)).toBe(true)
  })
  it("fails grey text on grey", () => {
    const low = runChecks(resolve(theme({ foreground: "#555555" }))).filter((c) => !c.ok)
    expect(low.map((c) => c.fg)).toContain("color-fg")
  })
})

/** The value, failing the test when it's missing. */
function must<T>(v: T | undefined): T {
  if (v === undefined) throw new Error("missing value")
  return v
}

describe("fixValue", () => {
  it("moves the text until it passes", () => {
    const tokens = resolve(theme({}, { "color-fg-muted": "#444444" }))
    const pair = must(PAIRS.find((p) => p.fg === "color-fg-muted" && p.bg === "color-surface"))
    const v = must(fixValue(tokens, pair))
    expect(contrast(hex(v), must(colorOf(tokens, "color-surface")))).toBeGreaterThanOrEqual(pair.min)
    // Lighter on a dark surface.
    expect(hex(v).r).toBeGreaterThan(0x44)
  })
  it("darkens a fill under white text", () => {
    const tokens = resolve(theme({}, { "color-accent": "#9ab8ff", "color-on-accent": "#ffffff" }))
    const pair = must(PAIRS.find((p) => p.bg === "color-accent"))
    expect(runChecks(tokens).find((c) => c.bg === "color-accent")?.ok).toBe(false)
    const v = must(fixValue(tokens, pair))
    expect(hex(v).r).toBeLessThan(0x9a)
    expect(contrast(hex(v), must(colorOf(tokens, "color-on-accent")))).toBeGreaterThanOrEqual(3)
  })
})

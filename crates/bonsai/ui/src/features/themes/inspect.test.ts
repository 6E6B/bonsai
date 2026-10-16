import { describe, expect, it } from "vitest"
import { varsIn } from "./inspect"

describe("varsIn", () => {
  it("finds every custom property, fallbacks included", () => {
    expect(varsIn("var(--color-fg)")).toEqual(["color-fg"])
    expect(varsIn("0 1px 2px var(--color-line), inset 0 0 0 1px var( --color-focus , var(--color-accent))")).toEqual([
      "color-line",
      "color-focus",
      "color-accent",
    ])
    expect(varsIn("#fff")).toEqual([])
  })
})

import { describe, expect, it } from "vitest"
import { seeThrough } from "./backdrop"

describe("seeThrough", () => {
  it("fades the page and cards and keeps the solid page color", () => {
    const t = { "color-canvas": "#1a1a1a", "color-surface": "var(--color-raised)", "color-raised": "#282828" }
    expect(seeThrough(t, 0.75)).toEqual({
      "backdrop-base": "#1a1a1a",
      "color-canvas": "#1a1a1abf",
      "color-surface": "#282828bf",
    })
  })

  it("clamps the opacity and skips tokens that aren't colors", () => {
    expect(seeThrough({ "color-canvas": "#ffffff" }, 2)).toEqual({
      "backdrop-base": "#ffffff",
      "color-canvas": "#ffffff",
    })
    expect(seeThrough({}, 0.5)).toEqual({})
  })
})

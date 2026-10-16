import { describe, expect, it } from "vitest"
import { backdropLayers, cutoutPath, seeThrough } from "./backdrop"

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

describe("backdropLayers", () => {
  const plain = {
    image: 70,
    panels: 75,
    blur: 0,
    dim: 0,
    saturation: 100,
    tint: 0,
    vignette: 0,
    scanlines: 0,
    grain: 0,
  }

  it("leaves the picture alone with no effects", () => {
    expect(backdropLayers(plain)).toMatchObject({ filter: undefined, overhang: 0, image: 0.7, scanlines: 0 })
  })

  it("builds the filter and overlay opacities", () => {
    const l = backdropLayers({ ...plain, blur: 10, dim: 25, saturation: 0, scanlines: 40, grain: 5 })
    expect(l.filter).toBe("blur(10px) brightness(0.75) saturate(0)")
    expect(l.overhang).toBe(20)
    expect(l).toMatchObject({ scanlines: 0.4, grain: 0.05, tint: 0 })
  })
})

describe("cutoutPath", () => {
  it("covers the box below the strip", () => {
    expect(cutoutPath(800, 600, 36)).toBe('path("M0 36 H800 V600 H0 Z")')
  })

  it("reaches up into the active tab with rounded corners", () => {
    expect(cutoutPath(800, 600, 36, { left: 10, right: 210, top: 4, radius: 6 })).toBe(
      'path("M0 36 H10 V10 A6 6 0 0 1 16 4 H204 A6 6 0 0 1 210 10 V36 H800 V600 H0 Z")',
    )
  })

  it("skips a tab scrolled out of view", () => {
    expect(cutoutPath(800, 600, 36, { left: 50, right: 50, top: 4, radius: 6 })).toBe('path("M0 36 H800 V600 H0 Z")')
  })
})

import { describe, expect, it } from "vitest"
import { BLACK, contrast, flatten, hex, mix, onFill, parseHex, toHex, WHITE, withAlpha } from "./color"

describe("parseHex / toHex", () => {
  it("round-trips every length", () => {
    expect(toHex(hex("#abc"))).toBe("#aabbcc")
    expect(toHex(hex("#abcd"))).toBe("#aabbccdd")
    expect(toHex(hex("#3D7DFF"))).toBe("#3d7dff")
    expect(toHex(hex("#ffffff0f"))).toBe("#ffffff0f")
  })

  it("rejects anything else", () => {
    for (const s of ["", "red", "#12", "#12345", "#gggggg", "url(x)", "rgb(0,0,0)"]) expect(parseHex(s)).toBeUndefined()
  })
})

describe("mix", () => {
  it("goes t of the way", () => {
    expect(toHex(mix(BLACK, WHITE, 0))).toBe("#000000")
    expect(toHex(mix(BLACK, WHITE, 1))).toBe("#ffffff")
    expect(toHex(mix(hex("#1a1a1a"), BLACK, 0.3))).toBe("#121212")
  })
})

describe("contrast", () => {
  it("matches WCAG", () => {
    expect(contrast(BLACK, WHITE)).toBeCloseTo(21)
    expect(contrast(WHITE, WHITE)).toBe(1)
    expect(contrast(hex("#767676"), WHITE)).toBeCloseTo(4.54, 2)
  })

  it("flattens translucent colors first", () => {
    expect(toHex(flatten(withAlpha(WHITE, 0), BLACK))).toBe("#000000")
    expect(toHex(flatten(withAlpha(WHITE, 255), BLACK))).toBe("#ffffff")
  })
})

describe("onFill", () => {
  it("picks white on dark fills and black on pale ones", () => {
    expect(onFill(hex("#3d7dff"))).toBe(WHITE)
    expect(onFill(hex("#ffd400"))).toBe(BLACK)
  })
})

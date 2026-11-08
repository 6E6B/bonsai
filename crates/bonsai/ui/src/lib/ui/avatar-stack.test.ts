import { describe, expect, it } from "vitest"
import { facesThatFit } from "./avatar-stack"

describe("facesThatFit", () => {
  it("shows everyone when they all fit and nobody is left over", () => {
    // 20 + 3 × 13.33 = 60 px for four faces.
    expect(facesThatFit(60, 20, 4, 4)).toBe(4)
  })

  it("keeps room for the count when people are left over", () => {
    expect(facesThatFit(60, 20, 4, 30)).toBeLessThan(4)
    expect(facesThatFit(200, 20, 40, 75)).toBe(12)
  })

  it("never shows more faces than it has", () => {
    expect(facesThatFit(2000, 20, 8, 75)).toBe(8)
  })

  it("shows none when there is no room", () => {
    expect(facesThatFit(0, 20, 8, 75)).toBe(0)
    expect(facesThatFit(25, 20, 8, 75)).toBe(0)
  })
})

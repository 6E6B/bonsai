import { describe, expect, it } from "vitest"
import { stepZoom } from "./zoom"

describe("stepZoom", () => {
  it("steps between levels and stops at the ends", () => {
    expect(stepZoom(1, 1)).toBe(1.1)
    expect(stepZoom(1, -1)).toBe(0.9)
    expect(stepZoom(2, 1)).toBe(2)
    expect(stepZoom(0.67, -1)).toBe(0.67)
  })

  it("starts from the nearest level", () => {
    expect(stepZoom(1.3, 1)).toBe(1.5)
    expect(stepZoom(0.99, -1)).toBe(0.9)
  })
})

import { describe, expect, it } from "vitest"
import { gridStep } from "./hooks"

describe("gridStep", () => {
  it("moves one item in a list and stops at the ends", () => {
    expect(gridStep(0, 1, 5, 1)).toBe(1)
    expect(gridStep(4, 1, 5, 1)).toBe(4)
    expect(gridStep(0, -1, 5, 1)).toBe(0)
  })

  it("moves a row at a time in a grid", () => {
    // 3 columns, 8 items: rows 0-2, 3-5, 6-7.
    expect(gridStep(1, 1, 8, 3)).toBe(4)
    expect(gridStep(4, -1, 8, 3)).toBe(1)
  })

  it("stays put going up from the top row", () => {
    expect(gridStep(2, -1, 8, 3)).toBe(2)
  })

  it("lands on the last item going down into a short last row", () => {
    expect(gridStep(5, 1, 8, 3)).toBe(7)
  })

  it("stays put going down from the last row", () => {
    expect(gridStep(6, 1, 8, 3)).toBe(6)
  })

  it("wraps sideways across rows in reading order", () => {
    expect(gridStep(2, 1, 8, 1)).toBe(3)
  })
})

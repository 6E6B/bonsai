import { describe, expect, it } from "vitest"
import { MAX_TICKS, placeMarks } from "./slider-marks"

describe("placeMarks", () => {
  it("puts a tick on every step", () => {
    expect(placeMarks(0, 100, 25, true)).toEqual([
      { value: 0, percent: 0 },
      { value: 25, percent: 25 },
      { value: 50, percent: 50 },
      { value: 75, percent: 75 },
      { value: 100, percent: 100 },
    ])
  })

  it("keeps fractional steps free of float noise", () => {
    expect(placeMarks(0, 1, 0.1, true).map((m) => m.value)).toEqual([0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1])
  })

  it("thins out dense steps", () => {
    const marks = placeMarks(0, 3, 0.05, true)
    expect(marks.length).toBeLessThanOrEqual(MAX_TICKS)
    expect(marks[0]?.value).toBe(0)
    expect(marks[1]?.value).toBe(0.15)
  })

  it("places listed marks in order and drops ones out of range or repeated", () => {
    expect(placeMarks(0, 10, 1, [10, { value: 5, label: "Mid" }, 20, -1, 5])).toEqual([
      { value: 5, label: "Mid", percent: 50 },
      { value: 10, percent: 100 },
    ])
  })

  it("draws nothing for an empty range or no marks", () => {
    expect(placeMarks(5, 5, 1, true)).toEqual([])
    expect(placeMarks(0, 10, 0, true)).toEqual([])
    expect(placeMarks(0, 10, 1, undefined)).toEqual([])
    expect(placeMarks(0, 10, 1, false)).toEqual([])
  })
})

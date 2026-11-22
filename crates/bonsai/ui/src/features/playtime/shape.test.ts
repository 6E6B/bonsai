import { describe, expect, it } from "vitest"
import { experienceName } from "~/lib/playtime"
import { barWidths, weeklyAverage } from "./shape"

describe("barWidths", () => {
  it("scales to the largest", () => {
    expect(barWidths([50, 100, 0])).toEqual([50, 100, 0])
    expect(barWidths([0, 0])).toEqual([0, 0])
    expect(barWidths([])).toEqual([])
  })
})

describe("weeklyAverage", () => {
  it("averages every week, empty ones too", () => {
    expect(weeklyAverage([])).toBe(0)
    expect(
      weeklyAverage([
        { start: 0, secs: 3600 },
        { start: 1, secs: 0 },
      ]),
    ).toBe(1800)
  })
})

describe("experienceName", () => {
  it("falls back to the id", () => {
    expect(experienceName({ name: "", universeId: 9 })).toBe("Experience 9")
    expect(experienceName({ name: "Obby", universeId: 9 })).toBe("Obby")
  })
})

import { describe, expect, it } from "vitest"
import {
  DAY,
  localX,
  logTicks,
  matchRange,
  movingAverage,
  niceTicks,
  panWindow,
  pctOver,
  presetWindow,
  priceDomain,
  rangesFor,
  spread,
  summarize,
  toDays,
  visibleSlice,
  zoomWindow,
} from "./history"

const t0 = Date.UTC(2026, 9, 1)
const p = (day: number, v: number) => ({ t: t0 + day * DAY, v })

describe("toDays", () => {
  it("joins series by day and fills days with no sales", () => {
    const days = toDays([p(0, 100), p(3, 130)], [p(3, 2), p(0, 1)])
    expect(days.map((d) => [d.price, d.sold])).toEqual([
      [100, 1],
      [null, 0],
      [null, 0],
      [130, 2],
    ])
  })

  it("drops a price with no copies sold", () => {
    expect(toDays([p(0, 100), p(1, 120)], [p(0, 1), p(1, 0)]).map((d) => d.price)).toEqual([100, null])
  })

  it("is empty with no data", () => {
    expect(toDays([], [])).toEqual([])
  })
})

describe("windows", () => {
  it("covers the last days of a preset", () => {
    expect(presetWindow(400, "7d")).toEqual({ from: 393, to: 400 })
    expect(presetWindow(400, "1y")).toEqual({ from: 35, to: 400 })
    expect(presetWindow(400, "all")).toEqual({ from: 0, to: 400 })
    expect(presetWindow(20, "30d")).toEqual({ from: 0, to: 20 })
  })

  it("offers only ranges shorter than the history", () => {
    expect(rangesFor(60).map((r) => r.value)).toEqual(["7d", "30d", "all"])
  })

  it("matches a window to its preset", () => {
    expect(matchRange({ from: 370, to: 400 }, 400)).toBe("30d")
    expect(matchRange({ from: 0, to: 400 }, 400)).toBe("all")
    expect(matchRange({ from: 300, to: 350 }, 400)).toBeNull()
  })

  it("zooms around the anchor and stays in bounds", () => {
    expect(zoomWindow({ from: 0, to: 100 }, 0.5, 50, 100)).toEqual({ from: 25, to: 75 })
    expect(zoomWindow({ from: 0, to: 100 }, 0.5, 0, 100)).toEqual({ from: 0, to: 50 })
    expect(zoomWindow({ from: 40, to: 60 }, 10, 50, 100)).toEqual({ from: 0, to: 100 })
    expect(zoomWindow({ from: 40, to: 50 }, 0.01, 45, 100).to - 40).toBeGreaterThanOrEqual(5)
  })

  it("pans up to either end", () => {
    expect(panWindow({ from: 10, to: 20 }, 5, 100)).toEqual({ from: 15, to: 25 })
    expect(panWindow({ from: 10, to: 20 }, -50, 100)).toEqual({ from: 0, to: 10 })
    expect(panWindow({ from: 80, to: 90 }, 50, 100)).toEqual({ from: 90, to: 100 })
  })

  it("rounds to whole days", () => {
    expect(visibleSlice({ from: 2.4, to: 9.6 })).toEqual([2, 10])
  })
})

describe("movingAverage", () => {
  it("weights by copies sold and skips empty days", () => {
    const days = toDays([p(0, 100), p(2, 200)], [p(0, 1), p(2, 3)])
    expect(movingAverage(days, 7)).toEqual([100, 100, 175])
  })

  it("forgets sales older than the window", () => {
    const days = toDays([p(0, 100), p(3, 200)], [p(0, 1), p(3, 1)])
    expect(movingAverage(days, 2)).toEqual([100, 100, null, 200])
  })
})

describe("summarize", () => {
  it("finds high, low, change and sale-weighted average", () => {
    const s = summarize(toDays([p(0, 100), p(1, 300), p(3, 150)], [p(0, 2), p(1, 1), p(3, 1)]))
    expect(s.sold).toBe(4)
    expect(s.days).toBe(4)
    expect(s.activeDays).toBe(3)
    expect(s.average).toBe(162.5)
    expect(s.high?.price).toBe(300)
    expect(s.low?.price).toBe(100)
    expect(s.change).toBeCloseTo(0.5)
  })

  it("has no change from a single sale", () => {
    expect(summarize(toDays([p(0, 100)], [p(0, 1)])).change).toBeNull()
  })
})

describe("pctOver", () => {
  it("rounds and handles missing values", () => {
    expect(pctOver(110, 100)).toBe(10)
    expect(pctOver(90, 100)).toBe(-10)
    expect(pctOver(null, 100)).toBeNull()
    expect(pctOver(100, 0)).toBeNull()
  })
})

describe("niceTicks", () => {
  it("picks round steps", () => {
    expect(niceTicks(0, 100)).toEqual([0, 50, 100])
    expect(niceTicks(231_000, 262_000)).toEqual([240_000, 250_000, 260_000])
    expect(niceTicks(0, 5, 4)).toEqual([0, 2, 4])
  })

  it("returns one tick for a flat range", () => {
    expect(niceTicks(5, 5)).toEqual([5])
  })
})

describe("logTicks", () => {
  it("uses 1-2-5 steps and thins wide ranges", () => {
    expect(logTicks(15, 600)).toEqual([20, 50, 100, 200, 500])
    expect(logTicks(1, 1_000_000)).toEqual([1, 10, 100, 1000, 10_000, 100_000, 1_000_000])
  })

  it("falls back to plain ticks for narrow ranges", () => {
    expect(logTicks(231_000, 262_000)).toEqual([240_000, 250_000, 260_000])
  })
})

describe("localX", () => {
  it("scales zoomed client pixels back to layout pixels", () => {
    expect(localX(600, { left: 375, width: 625 }, 500)).toBe(180)
    expect(localX(400, { left: 300, width: 500 }, 500)).toBe(100)
  })
})

describe("priceDomain", () => {
  it("keeps references within 4x on a log scale", () => {
    const d = priceDomain(
      [100, 400],
      [
        { key: "rap", label: "RAP", value: 50 },
        { key: "value", label: "Value", value: 5000 },
      ],
      true,
    )
    expect(d.shown.map((r) => r.key)).toEqual(["rap"])
    expect(d.lo).toBeLessThan(50)
    expect(d.hi).toBeGreaterThan(400)
  })

  it("keeps nearby references and leaves far ones off", () => {
    const d = priceDomain(
      [100, 120],
      [
        { key: "rap", label: "RAP", value: 130 },
        { key: "value", label: "Value", value: 1000 },
      ],
    )
    expect(d.shown.map((r) => r.key)).toEqual(["rap"])
    expect(d.off.map((r) => r.key)).toEqual(["value"])
    expect(d.lo).toBeLessThan(100)
    expect(d.hi).toBeGreaterThan(130)
  })
})

describe("spread", () => {
  it("pushes close labels apart and keeps order", () => {
    expect(spread([50, 52, 10], 12, 0, 100)).toEqual([50, 62, 10])
  })

  it("stays inside the bounds", () => {
    expect(spread([98, 99], 12, 0, 100)).toEqual([88, 100])
  })
})

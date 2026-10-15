import { describe, expect, it } from "vitest"
import { barWidths, dayString, matchKind, matchScope, nextPeriod, parseRange, periodLabel } from "./query"

describe("periodLabel", () => {
  it("names site periods and leaves custom to its dates", () => {
    expect(periodLabel("month")).toBe("Past month")
    expect(periodLabel("all")).toBe("All saved history")
    expect(periodLabel("custom")).toBeNull()
  })
})

describe("nextPeriod", () => {
  it("cycles the periods and skips custom", () => {
    expect(nextPeriod("day")).toBe("week")
    expect(nextPeriod("year")).toBe("all")
    expect(nextPeriod("all")).toBe("day")
    expect(nextPeriod("custom")).toBe("day")
  })
})

describe("parseRange", () => {
  it("includes the end date", () => {
    const r = parseRange("2026-01-01..2026-03-31")
    expect(r).not.toBeNull()
    expect(dayString(r?.from ?? 0)).toBe("2026-01-01")
    expect(dayString(r?.to ?? 0)).toBe("2026-04-01")
  })

  it("allows an open side", () => {
    expect(parseRange("2026-01-01..")?.to).toBeNull()
    expect(parseRange("..2026-01-01")?.from).toBeNull()
  })

  it("rejects bad dates and backwards ranges", () => {
    expect(parseRange("2026-02-30..2026-03-01")).toBeNull()
    expect(parseRange("2026-03-01..2026-01-01")).toBeNull()
    expect(parseRange("..")).toBeNull()
    expect(parseRange("2026-01-01")).toBeNull()
  })
})

describe("matchScope", () => {
  const groups = [
    { id: 11, name: "Obstacle Labs" },
    { id: 22, name: "Plain Clothing" },
  ]

  it("matches me, an ID, a prefix, then a substring", () => {
    expect(matchScope("", groups)).toBe("me")
    expect(matchScope("me", groups)).toBe("me")
    expect(matchScope("22", groups)).toBe("group:22")
    expect(matchScope("obst", groups)).toBe("group:11")
    expect(matchScope("cloth", groups)).toBe("group:22")
    expect(matchScope("nothing", groups)).toBeNull()
  })
})

describe("matchKind", () => {
  const kinds = [
    { key: "sales", label: "Sales" },
    { key: "premium_payouts", label: "Premium payouts" },
  ]

  it("matches keys, labels and prefixes", () => {
    expect(matchKind("", kinds)).toBe("")
    expect(matchKind("all", kinds)).toBe("")
    expect(matchKind("Sales", kinds)).toBe("sales")
    expect(matchKind("premium payouts", kinds)).toBe("premium_payouts")
    expect(matchKind("prem", kinds)).toBe("premium_payouts")
    expect(matchKind("trades", kinds)).toBeNull()
  })
})

describe("barWidths", () => {
  it("scales to the largest magnitude", () => {
    expect(barWidths([50, -100, 0])).toEqual([50, 100, 0])
    expect(barWidths([0, 0])).toEqual([0, 0])
    expect(barWidths([])).toEqual([])
  })
})

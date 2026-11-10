import { describe, expect, it } from "vitest"
import { pageOf, shownOnOverview, wornGroups, wornKind } from "./wearing"

const item = (assetType: number, id = 0) => ({ assetType, id })

describe("wornKind", () => {
  it("groups asset types", () => {
    expect(wornKind(8)).toBe("accessories")
    expect(wornKind(2)).toBe("clothing")
    expect(wornKind(11)).toBe("clothing")
    expect(wornKind(67)).toBe("clothing")
    expect(wornKind(17)).toBe("body")
    expect(wornKind(53)).toBe("animations")
    expect(wornKind(0)).toBe("other")
  })
})

describe("shownOnOverview", () => {
  it("shows accessories, clothing, heads and faces but not other body parts or animations", () => {
    expect([8, 11, 17, 18, 79, 0].every(shownOnOverview)).toBe(true)
    expect([27, 31, 53].some(shownOnOverview)).toBe(false)
  })
})

describe("wornGroups", () => {
  it("puts clothing first and drops empty groups", () => {
    const groups = wornGroups([item(8, 1), item(2, 2), item(53, 3), item(12, 4)])
    expect(groups.map((g) => g.kind)).toEqual(["clothing", "accessories", "animations"])
    expect(groups[0].items.map((i) => i.id)).toEqual([2, 4])
  })

  it("is empty for nothing worn", () => {
    expect(wornGroups([])).toEqual([])
  })
})

describe("pageOf", () => {
  const nine = [1, 2, 3, 4, 5, 6, 7, 8, 9]

  it("slices one page and counts the pages", () => {
    expect(pageOf(nine, 0, 8)).toEqual({ items: [1, 2, 3, 4, 5, 6, 7, 8], page: 0, pages: 2 })
    expect(pageOf(nine, 1, 8)).toEqual({ items: [9], page: 1, pages: 2 })
  })

  it("clamps a page past the end or before the start", () => {
    expect(pageOf(nine, 5, 8).page).toBe(1)
    expect(pageOf(nine, -1, 8).page).toBe(0)
  })

  it("has one empty page for nothing", () => {
    expect(pageOf([], 3, 8)).toEqual({ items: [], page: 0, pages: 1 })
  })
})

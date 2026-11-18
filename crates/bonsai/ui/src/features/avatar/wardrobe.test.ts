import { describe, expect, it } from "vitest"
import { groupTypes, groupWorn, matchesWear, named, searchStatus, sectionFor } from "./wardrobe"

describe("wardrobe results", () => {
  const items = [
    { name: "Hat 10", id: 1 },
    { name: "Blue Hat 2", id: 2 },
    { name: "Blue Shirt", id: 3 },
  ]
  it("matches every search word regardless of order or case", () => {
    expect(named(items, " HAT blue ", "newest").map((x) => x.id)).toEqual([2])
    expect(named(items, "missing", "name")).toEqual([])
  })
  it("sorts naturally without mutating the source or inventing acquisition dates", () => {
    expect(named(items, "hat", "name").map((x) => x.id)).toEqual([2, 1])
    expect(named(items, "hat", "name-desc").map((x) => x.id)).toEqual([1, 2])
    expect(named(items, "", "oldest")).toEqual(items)
    expect(items[0].id).toBe(1)
  })
  it("supports equipped, unequipped and all items", () => {
    expect(matchesWear(1, [1], "worn")).toBe(true)
    expect(matchesWear(2, [1], "worn")).toBe(false)
    expect(matchesWear(1, [1], "unworn")).toBe(false)
    expect(matchesWear(2, [1], "unworn")).toBe(true)
    expect(matchesWear(1, [1], "all")).toBe(true)
  })
})

describe("wardrobe search status", () => {
  const base = { items: [1, 2], loaded: 3, total: 3, failed: 0, partial: false, truncated: false }

  it("counts matches once every type has loaded", () => {
    expect(searchStatus(base)).toBe("2 matches")
    expect(searchStatus({ ...base, items: [1] })).toBe("1 match")
  })

  it("says what is still loading, failed or left out", () => {
    expect(searchStatus({ ...base, loaded: 1, failed: 1, partial: true, truncated: true })).toBe(
      "First 2 matches · loading 1 of 3 types · 1 type failed to load · some types have more items; open their tab to load them",
    )
  })
})

describe("worn groups", () => {
  const w = (id: number, typeId: number, layer: number | null = null) => ({ id, typeId, layer })
  it("groups by type and stacks layered clothing outermost first", () => {
    const g = groupWorn([w(1, 11), w(2, 8), w(3, 64, 1), w(4, 65, 2), w(5, 41), w(6, 999), w(7, 51)])
    expect(g.map((x) => x.key)).toEqual(["head", "accessories", "clothing", "body", "animation"])
    expect(g[0].items.map((x) => x.id)).toEqual([2, 5])
    expect(g[1].items.map((x) => x.id)).toEqual([6])
    expect(g[2].items.map((x) => x.id)).toEqual([4, 3, 1])
    expect(g[3].items).toEqual([])
    expect(g[4].items.map((x) => x.id)).toEqual([7])
  })
  it("finds the section to add to", () => {
    const sections = [
      { name: "Clothing", slots: [{ value: "a11" }] },
      { name: "Accessories", slots: [{ value: "a41" }, { value: "a8" }] },
    ]
    expect(sectionFor(sections, groupTypes("head"))).toEqual({ section: "Accessories", slot: "a8" })
    expect(sectionFor(sections, groupTypes("clothing"))).toEqual({ section: "Clothing", slot: "a11" })
    expect(sectionFor(sections, groupTypes("body"))).toBeUndefined()
  })
})

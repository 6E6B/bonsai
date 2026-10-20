import { describe, expect, it } from "vitest"
import { matchesWear, named } from "./wardrobe"

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

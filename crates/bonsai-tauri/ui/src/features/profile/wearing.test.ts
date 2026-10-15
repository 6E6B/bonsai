import { describe, expect, it } from "vitest"
import { wornGroups, wornKind } from "./wearing"

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

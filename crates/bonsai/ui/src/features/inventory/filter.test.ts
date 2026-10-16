import { describe, expect, it } from "vitest"
import type { ItemCard } from "~/lib/ipc/bindings"
import { isFilter, matches, shapeInventory, slotAssetType } from "./filter"

const item = (i: Partial<ItemCard>): ItemCard => ({
  id: 1,
  bundle: false,
  name: "Item",
  creator: "Roblox",
  creatorVerified: true,
  copies: 1,
  assetType: 8,
  price: null,
  limited: false,
  serial: null,
  rap: null,
  values: null,
  ...i,
})

const opts = { query: "", filter: "all" as const, sort: "newest", worn: new Set<number>(), collectibles: false }
const ids = (l: ItemCard[]) => l.map((i) => i.id)

describe("matches", () => {
  it("finds by name, creator or exact id", () => {
    const i = item({ id: 1029025, name: "The Classic ROBLOX Fedora", creator: "Roblox" })
    expect(matches(i, "fedora")).toBe(true)
    expect(matches(i, " ROBLOX ")).toBe(true)
    expect(matches(i, "1029025")).toBe(true)
    expect(matches(i, "102902")).toBe(false)
    expect(matches(i, "hat")).toBe(false)
    expect(matches(i, "")).toBe(true)
  })
})

describe("shapeInventory", () => {
  const list = [
    item({ id: 1, price: 50, copies: 2 }),
    item({ id: 2, limited: true, rap: 900, price: null }),
    item({ id: 3, price: 10 }),
    item({ id: 4, limited: true, rap: 100, price: 2000 }),
  ]

  it("filters", () => {
    expect(ids(shapeInventory(list, { ...opts, filter: "limited" }))).toEqual([2, 4])
    expect(ids(shapeInventory(list, { ...opts, filter: "copies" }))).toEqual([1])
    expect(ids(shapeInventory(list, { ...opts, filter: "for_sale" }))).toEqual([1, 3, 4])
    expect(ids(shapeInventory(list, { ...opts, filter: "wearing", worn: new Set([3, 9]) }))).toEqual([3])
  })

  it("keeps server order for newest and does not change its input", () => {
    expect(ids(shapeInventory(list, opts))).toEqual([1, 2, 3, 4])
    shapeInventory(list, { ...opts, sort: "price_asc" })
    expect(ids(list)).toEqual([1, 2, 3, 4])
  })

  it("sorts by price with unpriced items last", () => {
    expect(ids(shapeInventory(list, { ...opts, sort: "price_asc" }))).toEqual([3, 1, 4, 2])
    expect(ids(shapeInventory(list, { ...opts, sort: "price_desc" }))).toEqual([4, 1, 3, 2])
  })

  it("uses RAP as the price of collectibles", () => {
    const c = { ...opts, collectibles: true }
    expect(ids(shapeInventory(list, { ...c, sort: "price_asc" }))).toEqual([3, 1, 4, 2])
    expect(ids(shapeInventory(list, { ...c, sort: "price_desc" }))).toEqual([2, 4, 1, 3])
    expect(ids(shapeInventory(list, { ...c, sort: "rap" }))).toEqual([2, 4, 1, 3])
  })
})

it("reads filters and slot types", () => {
  expect(isFilter("wearing")).toBe(true)
  expect(isFilter("worn")).toBe(false)
  expect(slotAssetType("a8")).toBe(8)
  expect(slotAssetType("collectibles")).toBe(0)
  expect(slotAssetType("badges")).toBe(0)
})

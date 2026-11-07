import { describe, expect, it } from "vitest"
import type { InvCategoryView } from "~/lib/ipc/bindings"
import { findSlot, inventorySections } from "./sections"

const cat = (name: string, ...slots: [string, string][]): InvCategoryView => ({
  name,
  slots: slots.map(([value, label]) => ({ value, label })),
})

const categories = [
  cat("Collectibles", ["collectibles", "Collectibles"]),
  cat("Decals", ["a13", "Decals"]),
  cat("Models & Packages", ["a10", "Models"], ["a32", "Packages"]),
  cat("Heads", ["b4", "Dynamic Heads"]),
  cat("Accessories", ["a8", "Head"], ["a42", "Face"]),
  cat("Places", ["places", "Places"]),
  cat("Badges", ["badges", "Badges"]),
  cat("Brand New", ["a999", "Brand New"]),
  cat("Empty"),
]

describe("inventorySections", () => {
  const sections = inventorySections(categories)
  const labels = (i: number) => sections[i]?.items.map((n) => n.label)

  it("orders sections and keeps the server's order inside them", () => {
    expect(sections.map((s) => s.label)).toEqual([undefined, "Avatar", "Experiences", "Creations"])
    expect(labels(0)).toEqual(["Collectibles"])
    expect(labels(1)).toEqual(["Heads", "Accessories", "Brand New"])
    expect(labels(2)).toEqual(["Places", "Badges"])
    expect(labels(3)).toEqual(["Decals", "Models & Packages"])
  })

  it("makes one slot a plain row and several a group", () => {
    expect(sections[1]?.items[0]).toEqual({ value: "b4", label: "Heads" })
    const group = sections[1]?.items[1]
    expect(group?.value).toBeUndefined()
    expect(group?.children?.map((c) => c.value)).toEqual(["a8", "a42"])
  })

  it("leaves out sections with nothing in them", () => {
    expect(inventorySections([cat("Places", ["places", "Places"])]).map((s) => s.label)).toEqual(["Experiences"])
    expect(inventorySections([])).toEqual([])
  })
})

describe("findSlot", () => {
  it("finds a slot by key", () => {
    expect(findSlot(categories, "a42")).toBe("a42")
  })

  it("finds a slot by its label or its single-slot category, ignoring case", () => {
    expect(findSlot(categories, "face")).toBe("a42")
    expect(findSlot(categories, "HEADS")).toBe("b4")
  })

  it("is null for an unknown name or a group of several slots", () => {
    expect(findSlot(categories, "nope")).toBeNull()
    expect(findSlot(categories, "Accessories")).toBeNull()
  })

  it("trusts the param until the list loads", () => {
    expect(findSlot([], "a42")).toBe("a42")
  })
})

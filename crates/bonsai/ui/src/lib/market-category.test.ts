import { describe, expect, it } from "vitest"
import type { CategoryView } from "~/lib/ipc/bindings"
import { findCategory, priceOf, WISHLIST } from "./market-category"

const categories: CategoryView[] = [
  { name: "All", taxonomy: "t-all", subcategories: [] },
  {
    name: "Body",
    taxonomy: "t-body",
    subcategories: [
      { value: "t-hair", label: "Hair" },
      { value: "t-heads", label: "Heads" },
    ],
  },
]

describe("findCategory", () => {
  it("finds a category or subcategory by taxonomy", () => {
    expect(findCategory(categories, "t-body")).toBe("t-body")
    expect(findCategory(categories, "t-heads")).toBe("t-heads")
  })

  it("finds one by name, ignoring case", () => {
    expect(findCategory(categories, "body")).toBe("t-body")
    expect(findCategory(categories, "HAIR")).toBe("t-hair")
  })

  it("knows the wishlist", () => {
    expect(findCategory(categories, "Wishlist")).toBe(WISHLIST)
    expect(findCategory([], WISHLIST)).toBe(WISHLIST)
  })

  it("is null for an unknown name", () => {
    expect(findCategory(categories, "nope")).toBeNull()
  })

  it("trusts the param until the list loads", () => {
    expect(findCategory([], "t-hair")).toBe("t-hair")
  })
})

describe("priceOf", () => {
  it("reads whole robux and treats anything else as unset", () => {
    expect(priceOf("120")).toBe(120)
    expect(priceOf("0")).toBe(0)
    expect(priceOf("")).toBeNull()
    expect(priceOf("abc")).toBeNull()
  })
})

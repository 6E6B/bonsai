import { describe, expect, it } from "vitest"
import { type CategoryNode, type CategorySection, openFor, pressNode, treeKey, visibleRows } from "./category-tree"

const body: CategoryNode = {
  value: "body",
  label: "Body",
  children: [
    { value: "hair", label: "Hair" },
    { value: "heads", label: "Heads" },
  ],
}
const accessories: CategoryNode = {
  label: "Accessories",
  children: [
    { value: "a8", label: "Head" },
    { value: "a42", label: "Face" },
  ],
}
const sections: CategorySection[] = [
  { items: [{ value: "all", label: "All" }, body] },
  { label: "More", items: [accessories, { value: "wishlist", label: "Wishlist", icon: "heart" }] },
]

describe("openFor", () => {
  it("opens the group a child is in", () => {
    expect(openFor(sections, "heads")).toBe("body")
    expect(openFor(sections, "a42")).toBe("group:Accessories")
  })

  it("opens a selected parent", () => {
    expect(openFor(sections, "body")).toBe("body")
  })

  it("opens nothing for a plain row", () => {
    expect(openFor(sections, "all")).toBeNull()
    expect(openFor(sections, "wishlist")).toBeNull()
    expect(openFor(sections, "gone")).toBeNull()
  })
})

describe("visibleRows", () => {
  it("lists only the open group's children, across sections", () => {
    expect(visibleRows(sections, "group:Accessories").map((r) => r.key)).toEqual([
      "all",
      "body",
      "group:Accessories",
      "a8",
      "a42",
      "wishlist",
    ])
  })

  it("lists top-level rows when nothing is open", () => {
    expect(visibleRows(sections, null).map((r) => r.key)).toEqual(["all", "body", "group:Accessories", "wishlist"])
  })
})

describe("treeKey", () => {
  const rows = visibleRows(sections, "body")

  it("moves through what is showing and stops at the ends", () => {
    expect(treeKey(rows, "body", "ArrowDown", "body")).toEqual({ kind: "focus", key: "hair" })
    expect(treeKey(rows, "hair", "ArrowUp", "body")).toEqual({ kind: "focus", key: "body" })
    expect(treeKey(rows, "all", "ArrowUp", "body")).toBeNull()
    expect(treeKey(rows, "wishlist", "ArrowDown", "body")).toBeNull()
    expect(treeKey(rows, "heads", "Home", "body")).toEqual({ kind: "focus", key: "all" })
    expect(treeKey(rows, "all", "End", "body")).toEqual({ kind: "focus", key: "wishlist" })
  })

  it("right opens a group, then enters it", () => {
    expect(treeKey(visibleRows(sections, null), "body", "ArrowRight", null)).toEqual({ kind: "open", key: "body" })
    expect(treeKey(rows, "body", "ArrowRight", "body")).toEqual({ kind: "focus", key: "hair" })
    expect(treeKey(rows, "all", "ArrowRight", "body")).toBeNull()
  })

  it("left leaves a child, then closes the group", () => {
    expect(treeKey(rows, "heads", "ArrowLeft", "body")).toEqual({ kind: "focus", key: "body" })
    expect(treeKey(rows, "body", "ArrowLeft", "body")).toEqual({ kind: "close" })
    expect(treeKey(rows, "all", "ArrowLeft", "body")).toBeNull()
  })

  it("leaves other keys alone", () => {
    expect(treeKey(rows, "all", "Enter", "body")).toBeNull()
    expect(treeKey(rows, "all", "g", "body")).toBeNull()
    expect(treeKey(rows, "gone", "ArrowDown", "body")).toBeNull()
  })
})

describe("pressNode", () => {
  it("picks a row that isn't showing", () => {
    expect(pressNode(body, "all")).toEqual({ select: "body" })
  })

  it("goes back to the parent from one of its children", () => {
    expect(pressNode(body, "hair")).toEqual({ select: "body" })
  })

  it("toggles a parent that is already picked", () => {
    expect(pressNode(body, "body")).toBe("toggle")
  })

  it("does nothing on a plain row that is already picked", () => {
    expect(pressNode({ value: "all", label: "All" }, "all")).toBeNull()
  })

  it("picks a group's first child, then toggles while inside it", () => {
    expect(pressNode(accessories, "all")).toEqual({ select: "a8" })
    expect(pressNode(accessories, "a42")).toBe("toggle")
  })

  it("does nothing for an empty group", () => {
    expect(pressNode({ label: "Empty", children: [] }, "all")).toBeNull()
  })
})

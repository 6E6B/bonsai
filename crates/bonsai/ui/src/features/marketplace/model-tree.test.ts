import { describe, expect, it } from "vitest"
import { allParents, nodePath, visibleRows } from "./model-tree"

const n = (parent: number | null, depth: number, cls: string, name: string, children = 0) => ({
  parent,
  depth,
  class: cls,
  name,
  children,
})

// Hat
//   Handle
//     Mesh
//     Attachment
//   Weld
const nodes = [
  n(null, 0, "Accessory", "Hat", 2),
  n(0, 1, "MeshPart", "Handle", 2),
  n(1, 2, "SpecialMesh", "Mesh"),
  n(1, 2, "Attachment", "HatAttachment"),
  n(0, 1, "Weld", "AccessoryWeld"),
]

describe("visibleRows", () => {
  it("shows everything when nothing is collapsed", () => {
    expect(visibleRows(nodes, {}, "")).toEqual([0, 1, 2, 3, 4])
  })

  it("hides a collapsed node's descendants but not its siblings", () => {
    expect(visibleRows(nodes, { 1: true }, "")).toEqual([0, 1, 4])
    expect(visibleRows(nodes, { 0: true }, "")).toEqual([0])
  })

  it("ignores collapse on a leaf", () => {
    expect(visibleRows(nodes, { 2: true }, "")).toEqual([0, 1, 2, 3, 4])
  })

  it("keeps matches and their ancestors when searching, by name or class", () => {
    expect(visibleRows(nodes, { 0: true }, "attach")).toEqual([0, 1, 3])
    expect(visibleRows(nodes, {}, "WELD")).toEqual([0, 4])
    expect(visibleRows(nodes, {}, "nothing")).toEqual([])
  })
})

describe("nodePath", () => {
  it("joins names from the root", () => {
    expect(nodePath(nodes, 3)).toBe("Hat/Handle/HatAttachment")
    expect(nodePath(nodes, 0)).toBe("Hat")
  })
})

describe("allParents", () => {
  it("marks only nodes with children", () => {
    expect(allParents(nodes)).toEqual({ 0: true, 1: true })
  })
})

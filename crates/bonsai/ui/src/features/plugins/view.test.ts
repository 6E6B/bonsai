import { describe, expect, it } from "vitest"
import type { ViewNode } from "~/lib/ipc/bindings"
import { cellText, inputValues, mergeValues } from "./view"

const input = (key: string, value: string): ViewNode => ({
  type: "input",
  key,
  value,
  label: "",
  placeholder: "",
  action: null,
})

const tree = (...children: ViewNode[]): ViewNode => ({
  type: "column",
  children: [{ type: "row", children }],
})

describe("plugin views", () => {
  it("finds inputs anywhere in the tree", () => {
    expect(inputValues(tree(input("a", "1"), input("b", "")))).toEqual({ a: "1", b: "" })
    expect(inputValues(null)).toEqual({})
  })

  it("keeps typed text unless the plugin changed the value", () => {
    const prev = tree(input("min", "0"), input("name", "x"))
    const next = tree(input("min", "0"), input("name", "y"), input("new", "n"))
    expect(mergeValues(prev, next, { min: "15", name: "typed" })).toEqual({ min: "15", name: "y", new: "n" })
  })

  it("starts from the plugin's values and drops inputs that went away", () => {
    expect(mergeValues(null, tree(input("a", "1")), { gone: "x" })).toEqual({ a: "1" })
  })

  it("formats number cells", () => {
    expect(cellText(1500, (n) => `#${n}`)).toBe("#1500")
    expect(cellText("Hat", () => "")).toBe("Hat")
    expect(cellText(null, () => "x")).toBe("")
  })
})

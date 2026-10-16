import { describe, expect, it } from "vitest"
import { gridColumns, toRows } from "./virtual"

describe("gridColumns", () => {
  it("fits as many minimum-width columns as the gaps allow", () => {
    expect(gridColumns(1000, 150, 12)).toBe(6)
    expect(gridColumns(150, 150, 12)).toBe(1)
    expect(gridColumns(312, 150, 12)).toBe(2)
    expect(gridColumns(311, 150, 12)).toBe(1)
  })

  it("never returns less than one", () => {
    expect(gridColumns(0, 150, 12)).toBe(1)
    expect(gridColumns(40, 150, 12)).toBe(1)
    expect(gridColumns(500, 0, 12)).toBe(1)
  })
})

describe("toRows", () => {
  it("chunks with a short last row", () => {
    expect(toRows([1, 2, 3, 4, 5], 2)).toEqual([[1, 2], [3, 4], [5]])
  })

  it("treats zero columns as one", () => {
    expect(toRows([1, 2], 0)).toEqual([[1], [2]])
  })

  it("returns no rows for no items", () => {
    expect(toRows([], 3)).toEqual([])
  })
})

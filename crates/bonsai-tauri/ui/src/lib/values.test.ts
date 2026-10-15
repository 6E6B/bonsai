import { describe, expect, it } from "vitest"
import type { ValueRow, ValueTag } from "~/lib/ipc/bindings"
import { demandLevel, hasTag, rapGap, shapeValues, type ValueFilter, worth } from "./values"

const tag = (t: Partial<ValueTag> = {}): ValueTag => ({
  value: null,
  demand: null,
  trend: null,
  projected: false,
  hyped: false,
  rare: false,
  ...t,
})

const row = (id: number, name: string, rap: number | null, t: Partial<ValueTag> = {}, r: Partial<ValueRow> = {}) =>
  ({ id, name, acronym: "", rap, version: "classic", tag: tag(t), ...r }) as ValueRow

const rows = [
  row(1, "Classic Fedora", 400_000, { value: 450_000, demand: "amazing", rare: true }, { acronym: "CF" }),
  row(2, "Red Cap", 1_300, { projected: true }),
  row(3, "Shiny Thing", 9_000, { value: 5_000, demand: "low" }, { version: "new" }),
  row(4, "Another Cap", 2_000),
]

const all: ValueFilter = { query: "", version: "all", show: "all", sort: "worth" }
const ids = (r: ValueRow[]) => r.map((x) => x.id)

describe("values", () => {
  it("worth is the value, else RAP", () => {
    expect(worth(tag({ value: 10 }), 5)).toBe(10)
    expect(worth(tag(), 5)).toBe(5)
    expect(worth(null, null)).toBeNull()
  })

  it("rap gap is relative to the value", () => {
    expect(rapGap(100, 112)).toBe(12)
    expect(rapGap(100, 80)).toBe(-20)
    expect(rapGap(null, 80)).toBeNull()
    expect(rapGap(0, 80)).toBeNull()
  })

  it("demand levels run 1 to 5", () => {
    expect(demandLevel("terrible")).toBe(1)
    expect(demandLevel("amazing")).toBe(5)
  })

  it("an empty tag draws nothing", () => {
    expect(hasTag(tag())).toBe(false)
    expect(hasTag(tag({ projected: true }))).toBe(true)
    expect(hasTag(null)).toBe(false)
  })

  it("sorts by worth, highest first", () => {
    expect(ids(shapeValues(rows, all))).toEqual([1, 3, 4, 2])
  })

  it("searches names, exact acronyms and ids", () => {
    expect(ids(shapeValues(rows, { ...all, query: "cap" }))).toEqual([4, 2])
    expect(ids(shapeValues(rows, { ...all, query: "CF" }))).toEqual([1])
    expect(ids(shapeValues(rows, { ...all, query: "3" }))).toEqual([3])
  })

  it("filters by version and flags", () => {
    expect(ids(shapeValues(rows, { ...all, version: "new" }))).toEqual([3])
    expect(ids(shapeValues(rows, { ...all, show: "projected" }))).toEqual([2])
    expect(ids(shapeValues(rows, { ...all, show: "unprojected" }))).toEqual([1, 3, 4])
    expect(ids(shapeValues(rows, { ...all, show: "valued" }))).toEqual([1, 3])
    expect(ids(shapeValues(rows, { ...all, show: "rare" }))).toEqual([1])
  })

  it("sorts by demand, then worth", () => {
    expect(ids(shapeValues(rows, { ...all, sort: "demand" }))).toEqual([1, 3, 4, 2])
  })

  it("sorts by the biggest gap between RAP and value", () => {
    expect(ids(shapeValues(rows, { ...all, sort: "gap" }))).toEqual([3, 1, 4, 2])
  })

  it("sorts by name", () => {
    expect(ids(shapeValues(rows, { ...all, sort: "name" }))).toEqual([4, 1, 2, 3])
  })
})

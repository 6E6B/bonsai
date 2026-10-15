import { describe, expect, it } from "vitest"
import { itemMenu, templateItems } from "./menus"

const labels = (rows: ReturnType<typeof templateItems>) => rows.map((r) => ("label" in r ? r.label : r.kind))

describe("templateItems", () => {
  it("offers both downloads for shirts and pants", () => {
    expect(labels(templateItems(1, 11))).toEqual(["market.template", "market.template_masked"])
    expect(labels(templateItems(1, 12))).toHaveLength(2)
  })

  it("offers only the plain download for T-shirts", () => {
    expect(labels(templateItems(1, 2))).toEqual(["market.template"])
  })

  it("offers nothing for other items or unknown types", () => {
    expect(templateItems(1, 8)).toEqual([])
    expect(templateItems(1, undefined)).toEqual([])
  })

  it("leaves bundles alone in the item menu", () => {
    expect(itemMenu(1, true, 11)).toHaveLength(itemMenu(1, false, 8).length)
    expect(itemMenu(1, false, 11).length).toBeGreaterThan(itemMenu(1, false, 8).length)
  })
})

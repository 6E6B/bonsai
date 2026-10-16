import { describe, expect, it } from "vitest"
import { afterPurchase } from "./purchase"

describe("afterPurchase", () => {
  it("subtracts the price", () => {
    expect(afterPurchase(100, 40)).toEqual({ after: 60, short: 0 })
    expect(afterPurchase(40, 40)).toEqual({ after: 0, short: 0 })
  })
  it("reports what's missing", () => {
    expect(afterPurchase(10, 50)).toEqual({ after: 0, short: 40 })
  })
  it("free items need nothing", () => {
    expect(afterPurchase(0, 0)).toEqual({ after: 0, short: 0 })
  })
  it("unknown balance never blocks", () => {
    expect(afterPurchase(null, 50)).toEqual({ after: null, short: 0 })
  })
})

import { describe, expect, it } from "vitest"
import { budgetText, every } from "./plugins"

describe("every", () => {
  it("names the largest whole unit", () => {
    expect(every(7200)).toBe("every 2 h")
    expect(every(300)).toBe("every 5 min")
    expect(every(90)).toBe("every 90 s")
  })
})

describe("budgetText", () => {
  const b = { account: 1, robux: 0, trades: 0, spentRobux: 0, spentTrades: 0, hasRobux: true, hasTrades: false }
  it("says what asks each time", () => {
    expect(budgetText(b)).toBe("Asks before spending Robux.")
    expect(budgetText({ ...b, hasTrades: true })).toBe("Asks before spending Robux. Asks before accepting trades.")
  })
  it("shows the budget and what was spent", () => {
    expect(budgetText({ ...b, robux: 500, spentRobux: 120 })).toBe("R$ 500 a day, R$ 120 spent.")
    expect(budgetText({ ...b, hasRobux: false, hasTrades: true, trades: 1, spentTrades: 1 })).toBe(
      "1 trade a day, 1 accepted.",
    )
  })
})

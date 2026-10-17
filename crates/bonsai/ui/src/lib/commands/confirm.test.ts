import { describe, expect, it } from "vitest"
import { confirmations } from "./confirm"

describe("buy confirmations", () => {
  const title = (id: string, arg: string | null) => confirmations[id]?.(arg)?.title ?? null
  it("ask when the argument names a price", () => {
    expect(title("market.buy", "item:2 50")).toBe("Buy for R$ 50?")
    expect(title("market.buy_resale", "bundle:7  900")).toBe("Buy for R$ 900?")
    expect(title("game.buy_pass", "1818 2001 25")).toBe("Buy for R$ 25?")
    expect(title("game.buy_private_server", "1818 100 My server")).toBe("Buy this private server for R$ 100?")
  })
  it("leave page arguments to the page's own dialog", () => {
    expect(title("market.buy", null)).toBeNull()
    expect(title("market.buy_as", "156")).toBeNull()
    expect(title("market.buy_resale", "0d1d5570-68e5-49d8-8c9e-fdb777863bd0")).toBeNull()
    expect(title("game.buy_pass", "2001")).toBeNull()
    expect(title("game.buy_private_server", "Friends 5 only")).toBeNull()
  })
})

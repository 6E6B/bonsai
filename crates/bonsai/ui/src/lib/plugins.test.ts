import { describe, expect, it } from "vitest"
import type { ActionView } from "~/lib/ipc/bindings"
import { byTier } from "./plugins"

describe("byTier", () => {
  it("groups actions by tier, lowest first, and skips empty tiers", () => {
    const a = (key: string, tier: number): ActionView => ({ key, title: key, tier })
    const groups = byTier([a("market.buy", 3), a("user.unfriend", 2), a("user.block", 2)])
    expect(groups.map((g) => g.tier)).toEqual([2, 3])
    expect(groups[0].actions.map((x) => x.key)).toEqual(["user.unfriend", "user.block"])
  })
})

import { describe, expect, it } from "vitest"
import type { ActionView, CommandInfo } from "~/lib/ipc/bindings"
import { byTier, pluginCommandsFor } from "./plugins"

describe("byTier", () => {
  it("groups actions by tier, lowest first, and skips empty tiers", () => {
    const a = (key: string, tier: number): ActionView => ({ key, title: key, tier })
    const groups = byTier([a("market.buy", 3), a("user.unfriend", 2), a("user.block", 2)])
    expect(groups.map((g) => g.tier)).toEqual([2, 3])
    expect(groups[0].actions.map((x) => x.key)).toEqual(["user.unfriend", "user.block"])
  })
})

describe("pluginCommandsFor", () => {
  it("keeps commands for that menu, by title", () => {
    const c = (id: string, title: string, context: string | null): CommandInfo => ({
      id,
      title,
      keywords: [],
      arg: "user",
      shortcuts: [],
      rebound: false,
      context,
    })
    const list = [c("plugin.b.x", "B: x", "user"), c("user.open", "Open", null), c("plugin.a.y", "A: y", "user")]
    expect(pluginCommandsFor(list, "user").map((x) => x.id)).toEqual(["plugin.a.y", "plugin.b.x"])
    expect(pluginCommandsFor(list, "game")).toEqual([])
  })
})

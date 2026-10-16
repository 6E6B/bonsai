import { describe, expect, it } from "vitest"
import { arrange, describePath, moveScreen, nudgeScreen, type Screen, screenKey, sidebarGroups } from "./screens"

describe("describePath", () => {
  it("names screens and their sub-paths", () => {
    expect(describePath("/").title).toBe("Home")
    expect(describePath("/friends").title).toBe("Friends")
    expect(describePath("/transactions").title).toBe("Transactions")
    expect(describePath("/transactions").icon).toBe("dollar")
    expect(describePath("/settings/privacy").title).toBe("Settings")
  })

  it("names entity pages generically", () => {
    expect(describePath("/games/1").title).toBe("Experience")
    expect(describePath("/communities/4").title).toBe("Community")
    expect(describePath("/communities").title).toBe("Communities")
  })
})

const fake = (key: string, group: Screen["group"], shown?: () => boolean): Screen => ({
  command: `nav.${key}`,
  label: key,
  icon: "home",
  path: `/${key}`,
  sidebar: "top",
  group,
  shown,
})

describe("sidebar arrangement", () => {
  const all = [
    fake("home", "start"),
    fake("friends", "social"),
    fake("messages", "social"),
    fake("groups", "social"),
    fake("trades", "economy"),
    fake("values", "economy", () => false),
    fake("ledger", "economy"),
    { ...fake("settings", undefined), sidebar: "bottom" as const },
  ]
  const keys = (g: Screen[][]) => g.map((x) => x.map(screenKey))
  const none = { order: [], hidden: [] }

  it("groups in default order and drops hidden, switched off and empty groups", () => {
    expect(keys(sidebarGroups(all, none))).toEqual([["home"], ["friends", "messages", "groups"], ["trades", "ledger"]])
    expect(keys(sidebarGroups(all, { order: [], hidden: ["home", "messages"] }))).toEqual([
      ["friends", "groups"],
      ["trades", "ledger"],
    ])
  })

  it("follows the saved order and puts unnamed screens after", () => {
    expect(keys(arrange(all, { order: ["groups", "friends"], hidden: [] }))[1]).toEqual([
      "groups",
      "friends",
      "messages",
    ])
  })

  it("moves among shown rows and keeps hidden rows in place", () => {
    expect(moveScreen(all, none, "groups", 0)).toEqual([
      "home",
      "groups",
      "friends",
      "messages",
      "trades",
      "values",
      "ledger",
    ])
    // Values is switched off: moving Ledger up one passes Trades, not Values.
    expect(nudgeScreen(all, none, "ledger", -1)?.slice(4)).toEqual(["ledger", "trades", "values"])
    const hidden = { order: [], hidden: ["messages"] }
    expect(nudgeScreen(all, hidden, "friends", 1)?.slice(1, 4)).toEqual(["messages", "groups", "friends"])
    // No shown row below: Trades goes to the end of its group.
    expect(nudgeScreen(all, { order: [], hidden: ["ledger"] }, "trades", 1)?.slice(4)).toEqual([
      "values",
      "ledger",
      "trades",
    ])
  })

  it("does nothing at the ends or for unknown screens", () => {
    expect(nudgeScreen(all, none, "friends", -1)).toBeNull()
    expect(nudgeScreen(all, none, "groups", 1)).toBeNull()
    expect(moveScreen(all, none, "nope", 0)).toBeNull()
    expect(nudgeScreen(all, none, "values", 1)).toEqual([
      "home",
      "friends",
      "messages",
      "groups",
      "trades",
      "ledger",
      "values",
    ])
  })
})

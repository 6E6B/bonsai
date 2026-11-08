import { describe, expect, it } from "vitest"
import { categoryArg, dayLabel, groupByDay, openCommand, shownNotifications } from "./filter"

const items = [
  { id: "a", read: false, category: "friends" },
  { id: "b", read: true, category: "friends" },
  { id: "c", read: false, category: "account" },
]
const ids = (v: { view: string; category: string }) => shownNotifications(items, v).map((n) => n.id)

describe("shownNotifications", () => {
  it("filters by read state and category together", () => {
    expect(ids({ view: "all", category: "" })).toEqual(["a", "b", "c"])
    expect(ids({ view: "unread", category: "" })).toEqual(["a", "c"])
    expect(ids({ view: "all", category: "friends" })).toEqual(["a", "b"])
    expect(ids({ view: "unread", category: "friends" })).toEqual(["a"])
    expect(ids({ view: "all", category: "trades" })).toEqual([])
  })
})

describe("categoryArg", () => {
  it("accepts known ids and falls back to all", () => {
    expect(categoryArg("Friends ", ["friends"])).toBe("friends")
    expect(categoryArg("nope", ["friends"])).toBe("")
    expect(categoryArg(undefined, ["friends"])).toBe("")
  })
})

describe("openCommand", () => {
  const n = { id: "1", link: null, userId: null, category: "trades" }
  it("prefers the link, then the person", () => {
    expect(openCommand({ ...n, link: "https://www.roblox.com/x", userId: 5 })).toEqual([
      "entity.open_website",
      "https://www.roblox.com/x",
    ])
    expect(openCommand({ ...n, userId: 5 })).toEqual(["user.open", "5"])
  })
  it("opens a screen only for the app's own entries", () => {
    expect(openCommand(n)).toBeNull()
    expect(openCommand({ ...n, id: "app:1:0" })).toEqual(["nav.trades"])
    expect(openCommand({ ...n, id: "app:1:0", category: "account" })).toEqual(["nav.transactions"])
  })
})

describe("groupByDay", () => {
  const now = new Date(2026, 10, 6, 15, 0).getTime()
  const at = (d: number, h = 12) => new Date(2026, 10, d, h).getTime()

  it("labels days relative to now", () => {
    expect(dayLabel(at(6, 0), now)).toBe("Today")
    expect(dayLabel(at(5, 23), now)).toBe("Yesterday")
    expect(dayLabel(at(1), now)).toBe("This week")
    expect(dayLabel(new Date(2026, 9, 20).getTime(), now)).toBe("Older")
  })

  it("puts a heading before each group and maps positions", () => {
    const items = [at(6), at(6, 9), at(5), at(1)]
    const { rows, positions } = groupByDay(items, (t) => t, now)
    expect(rows.map((r) => ("heading" in r ? r.heading : r.index))).toEqual([
      "Today",
      0,
      1,
      "Yesterday",
      2,
      "This week",
      3,
    ])
    expect(positions).toEqual([1, 2, 4, 6])
  })

  it("is empty for no items", () => {
    expect(groupByDay([], () => 0, now)).toEqual({ rows: [], positions: [] })
  })
})

import { describe, expect, it } from "vitest"
import { categoryArg, openCommand, shownNotifications } from "./filter"

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

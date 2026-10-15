import { describe, expect, it } from "vitest"
import { categoryArg, shownNotifications } from "./filter"

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

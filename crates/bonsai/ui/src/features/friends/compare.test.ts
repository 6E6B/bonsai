import { describe, expect, it } from "vitest"
import type { SharedFriendsView } from "~/lib/ipc/bindings"
import { compareSummary, filterPeople } from "./compare"

const view = (v: Partial<SharedFriendsView>): SharedFriendsView => ({
  loading: false,
  aCount: 0,
  bCount: 0,
  linked: false,
  friends: [],
  ...v,
})

describe("compareSummary", () => {
  it("counts shared friends and each list", () => {
    const friends = [{ id: 1, name: "a", displayName: "A" }]
    expect(compareSummary(view({ aCount: 3, bCount: 4, friends }), "Main", "Alt")).toBe(
      "1 shared friend. Main has 3, Alt has 4.",
    )
    expect(compareSummary(view({ linked: true }), "Main", "Alt")).toBe(
      "0 shared friends. Main has 0, Alt has 0. They are friends with each other.",
    )
  })
  it("waits for both lists", () => {
    expect(compareSummary(view({ loading: true }), "Main", "Alt")).toBe("Loading friend lists…")
  })
})

describe("filterPeople", () => {
  const people = [
    { id: 1, name: "builder", displayName: "Sam" },
    { id: 2, name: "racer", displayName: "Alex" },
  ]
  it("matches either name", () => {
    expect(filterPeople(people, "SAM").map((p) => p.id)).toEqual([1])
    expect(filterPeople(people, "rac").map((p) => p.id)).toEqual([2])
    expect(filterPeople(people, " ")).toHaveLength(2)
  })
})

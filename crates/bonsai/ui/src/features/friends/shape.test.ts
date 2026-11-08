import { describe, expect, it } from "vitest"
import { pick, type Row, shapePeople, showOptions, sortOptions } from "./shape"

const row = (id: number, displayName: string, more: Partial<Row> = {}): Row => ({
  id,
  name: displayName.toLowerCase(),
  displayName,
  presence: "offline",
  ...more,
})
const ids = (l: Row[]) => l.map((r) => r.id)
const opts = { query: "", sort: "", show: "" }

describe("shapePeople", () => {
  const friends = [
    row(1, "Zed", { presence: "game" }),
    row(2, "Amy", { presence: "online" }),
    row(3, "Bob", { presence: "studio" }),
    row(4, "Cal", { nickname: "Aaron" }),
  ]

  it("keeps the backend's order by default", () => {
    expect(ids(shapePeople(friends, "friends", opts))).toEqual([1, 2, 3, 4])
  })

  it("filters friends by status", () => {
    expect(ids(shapePeople(friends, "friends", { ...opts, show: "online" }))).toEqual([1, 2, 3])
    expect(ids(shapePeople(friends, "friends", { ...opts, show: "game" }))).toEqual([1])
    expect(ids(shapePeople(friends, "friends", { ...opts, show: "offline" }))).toEqual([4])
  })

  it("sorts by the name shown, nickname first", () => {
    expect(ids(shapePeople(friends, "friends", { ...opts, sort: "name" }))).toEqual([4, 2, 3, 1])
  })

  it("ranks search results under the default sort", () => {
    const list = [row(1, "Mabel"), row(2, "Abe")]
    expect(ids(shapePeople(list, "followers", { ...opts, query: "abe" }))).toEqual([2, 1])
    expect(ids(shapePeople(list, "followers", { ...opts, query: "abe", sort: "name" }))).toEqual([2, 1])
  })

  it("sorts requests by date and mutual friends", () => {
    const requests = [
      row(1, "A", { sentAt: "2026-01-02T00:00:00Z", mutuals: 1 }),
      row(2, "B", { sentAt: "2026-03-01T00:00:00Z", mutuals: 0 }),
      row(3, "C", { sentAt: "2026-02-01T00:00:00Z", mutuals: 4 }),
    ]
    expect(ids(shapePeople(requests, "requests", opts))).toEqual([2, 3, 1])
    expect(ids(shapePeople(requests, "requests", { ...opts, sort: "oldest" }))).toEqual([1, 3, 2])
    expect(ids(shapePeople(requests, "requests", { ...opts, sort: "mutuals" }))).toEqual([3, 1, 2])
  })

  it("shows followers you don't follow back", () => {
    const list = [row(1, "A", { following: true }), row(2, "B", { following: false })]
    expect(ids(shapePeople(list, "followers", { ...opts, show: "not_followed" }))).toEqual([2])
  })

  it("ignores a sort or filter the section doesn't offer", () => {
    expect(ids(shapePeople(friends, "blocked", { ...opts, sort: "mutuals", show: "online" }))).toEqual([1, 2, 3, 4])
  })
})

describe("options", () => {
  it("offers filters only where they mean something", () => {
    expect(showOptions("requests")).toEqual([])
    expect(sortOptions("friends")[0].value).toBe("status")
    expect(pick(sortOptions("blocked"), "nope")).toBe("default")
  })
})

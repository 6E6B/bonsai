import { describe, expect, it } from "vitest"
import type { GroupCard } from "~/lib/ipc/bindings"
import { shapeCommunities } from "./shape"

const group = (id: number, name: string, more: Partial<GroupCard> = {}): GroupCard => ({
  id,
  name,
  members: 0,
  verified: false,
  role: "Member",
  primary: false,
  ...more,
})
const ids = (l: GroupCard[]) => l.map((c) => c.id)
const opts = { query: "", sort: "default", show: "all" }

describe("shapeCommunities", () => {
  const list = [
    group(1, "Zeta Studios", { members: 500, role: "Owner", verified: true }),
    group(2, "Alpha Club", { members: 10_000, primary: true }),
    group(3, "Mid Group", { members: 50, role: "Admin" }),
  ]

  it("keeps the default order", () => {
    expect(ids(shapeCommunities(list, opts))).toEqual([1, 2, 3])
  })

  it("sorts", () => {
    expect(ids(shapeCommunities(list, { ...opts, sort: "name" }))).toEqual([2, 3, 1])
    expect(ids(shapeCommunities(list, { ...opts, sort: "members" }))).toEqual([2, 1, 3])
    expect(ids(shapeCommunities(list, { ...opts, sort: "fewest" }))).toEqual([3, 1, 2])
    expect(ids(shapeCommunities(list, { ...opts, sort: "role" }))).toEqual([3, 2, 1])
  })

  it("filters", () => {
    expect(ids(shapeCommunities(list, { ...opts, show: "verified" }))).toEqual([1])
    expect(ids(shapeCommunities(list, { ...opts, show: "primary" }))).toEqual([2])
  })

  it("searches name and role", () => {
    expect(ids(shapeCommunities(list, { ...opts, query: "admin" }))).toEqual([3])
    expect(ids(shapeCommunities(list, { ...opts, query: "o", sort: "members" }))).toEqual([1, 3])
  })
})

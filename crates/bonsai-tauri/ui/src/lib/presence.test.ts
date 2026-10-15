import { describe, expect, it } from "vitest"
import { groupByGame, groupByPresence } from "./presence"

describe("groupByPresence", () => {
  it("orders groups in game, studio, online, offline and skips empty ones", () => {
    const rows = groupByPresence([
      { id: 1, presence: "offline" as const },
      { id: 2, presence: "game" as const },
      { id: 3, presence: "online" as const },
      { id: 4, presence: "game" as const },
    ])
    expect(rows.map((r) => (r.kind === "header" ? `${r.label}:${r.count}` : r.user.id))).toEqual([
      "In game:2",
      2,
      4,
      "Online:1",
      3,
      "Offline:1",
      1,
    ])
  })

  it("returns nothing for no users", () => {
    expect(groupByPresence([])).toEqual([])
  })
})

describe("groupByGame", () => {
  const u = (id: number, status: string, universeId: number | null, placeId: number | null = universeId) => ({
    id,
    status,
    universeId,
    placeId,
  })

  it("groups by experience, biggest first, then by name", () => {
    const groups = groupByGame([u(1, "Zeta", 10), u(2, "Alpha", 20), u(3, "Zeta", 10), u(4, "Beta", 30)])
    expect(groups.map((g) => [g.name, g.users.map((x) => x.id)])).toEqual([
      ["Zeta", [1, 3]],
      ["Alpha", [2]],
      ["Beta", [4]],
    ])
  })

  it("falls back to place id, then status text, when the experience is hidden", () => {
    const groups = groupByGame([u(1, "Obby", null, 5), u(2, "Obby", null, 5), u(3, "In a game", null, null)])
    expect(groups.map((g) => [g.key, g.users.length])).toEqual([
      ["p5", 2],
      ["sIn a game", 1],
    ])
  })
})

import { describe, expect, it } from "vitest"
import type { TraderBadge } from "~/lib/ipc/bindings"
import { topBadges } from "./trader"

const b = (key: string, family = ""): TraderBadge => ({ key, name: key, group: "other", family, earned: null })

describe("topBadges", () => {
  it("keeps the highest tier of each family and folds the rest under it", () => {
    const shown = topBadges([
      b("value_20m", "value"),
      b("value_1m", "value"),
      b("own_1_dominus"),
      b("value_100k", "value"),
      b("own_100_items", "items"),
      b("own_10_items", "items"),
    ])
    expect(shown.map((s) => s.badge.key)).toEqual(["value_20m", "own_1_dominus", "own_100_items"])
    expect(shown[0].lower.map((l) => l.key)).toEqual(["value_1m", "value_100k"])
    expect(shown[1].lower).toEqual([])
  })

  it("never merges one-offs", () => {
    expect(topBadges([b("a"), b("b")])).toHaveLength(2)
  })
})

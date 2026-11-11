import { describe, expect, it } from "vitest"
import { describeVisit, fade, describe as fill, openable, seenBoost, touch } from "./history"

const MIN = 60_000
const DAY = 86_400_000

describe("describeVisit", () => {
  it("names entities, screens and settings sections", () => {
    expect(describeVisit("/users/12?tab=about")).toMatchObject({
      ref: "user:12",
      open: { id: "user.open", arg: "12" },
      thumb: { kind: "headshot", id: 12 },
    })
    expect(describeVisit("/games/5")).toMatchObject({ ref: "game:5", entity: { kind: "experience", id: 5 } })
    expect(describeVisit("/bundles/7")?.open).toEqual({ id: "market.open", arg: "bundle:7" })
    expect(describeVisit("/friends")?.ref).toBe("nav.friends")
    expect(describeVisit("/")?.ref).toBe("nav.home")
    expect(describeVisit("/inventory/3")?.ref).toBe("nav.inventory")
    expect(describeVisit("/settings/privacy")?.ref).toBe("nav.settings:privacy")
    expect(describeVisit("/nowhere")).toBeNull()
  })
})

describe("touch", () => {
  it("counts a run of visits once and keeps details", () => {
    let h = touch({}, { ref: "user:1", title: "Luke" }, 0)
    h = touch(h, { ref: "user:1" }, 5 * MIN)
    expect(h["user:1"]).toMatchObject({ n: 1, t: 5 * MIN, title: "Luke" })
    h = touch(h, { ref: "user:1" }, 60 * MIN)
    expect(h["user:1"].n).toBe(2)
  })

  it("fills in details without counting a visit", () => {
    const h = fill(touch({}, { ref: "game:1" }, 0), "game:1", { thumb: { kind: "game_icon", id: 9 } })
    expect(h["game:1"]).toMatchObject({ n: 1, thumb: { kind: "game_icon", id: 9 } })
  })
})

describe("seenBoost", () => {
  it("grows with visits, fades with time and is capped", () => {
    let h = {}
    for (let i = 0; i < 5; i++) h = touch(h, { ref: "a" }, i * DAY)
    h = touch(h, { ref: "b" }, 4 * DAY)
    const now = 4 * DAY
    expect(seenBoost(h, "a", now)).toBeGreaterThan(seenBoost(h, "b", now))
    expect(seenBoost(h, "a", now + 60 * DAY)).toBeLessThan(seenBoost(h, "a", now))
    expect(seenBoost(h, "c", now)).toBe(0)
    for (let i = 0; i < 2000; i++) h = touch(h, { ref: "a" }, now + i * DAY)
    expect(seenBoost(h, "a", now + 2000 * DAY)).toBeLessThanOrEqual(45)
    expect(fade(0)).toBe(1)
  })

  it("lists only what can be opened, newest first", () => {
    let h = touch({}, { ref: "nav.friends" }, 0)
    h = touch(h, { ref: "user:1", title: "A", open: { id: "user.open", arg: "1" } }, 1)
    h = touch(h, { ref: "user:2", title: "B", open: { id: "user.open", arg: "2" } }, 2)
    expect(openable(h).map((s) => s.ref)).toEqual(["user:2", "user:1"])
  })
})

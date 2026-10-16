import { describe, expect, it } from "vitest"
import { dayLabel, filterConversations, startsDay } from "./timeline"

// Local times, so the tests hold in any time zone.
const at = (y: number, mo: number, d: number, h = 12) => new Date(y, mo - 1, d, h).getTime()
const now = at(2026, 10, 14, 9)

describe("dayLabel", () => {
  it("calls the current day Today, even late at night", () => {
    expect(dayLabel(at(2026, 10, 14, 0), now)).toBe("Today")
    expect(dayLabel(at(2026, 10, 14, 23), now)).toBe("Today")
  })

  it("calls the day before Yesterday, across a month edge too", () => {
    expect(dayLabel(at(2026, 10, 13, 23), now)).toBe("Yesterday")
    expect(dayLabel(at(2026, 9, 30), at(2026, 10, 1, 8))).toBe("Yesterday")
  })

  it("gives the full date for anything older", () => {
    expect(dayLabel(at(2026, 10, 12), now)).toBe("Oct 12, 2026")
    expect(dayLabel(at(2025, 12, 31), now)).toBe("Dec 31, 2025")
  })
})

describe("startsDay", () => {
  it("opens the first line", () => {
    expect(startsDay(null, at(2026, 10, 14))).toBe(true)
  })

  it("opens a line from another day only", () => {
    expect(startsDay(at(2026, 10, 14, 1), at(2026, 10, 14, 23))).toBe(false)
    expect(startsDay(at(2026, 10, 13, 23), at(2026, 10, 14, 0))).toBe(true)
  })

  it("never opens a line without a time", () => {
    expect(startsDay(null, null)).toBe(false)
    expect(startsDay(at(2026, 10, 14), null)).toBe(false)
  })
})

describe("filterConversations", () => {
  const items = [{ title: "Pixel Fox" }, { title: "Build club" }, { title: "mossy_bricks" }]

  it("matches names anywhere in the title, ignoring case", () => {
    expect(filterConversations(items, "FOX")).toEqual([{ title: "Pixel Fox" }])
    expect(filterConversations(items, " b ")).toEqual([{ title: "Build club" }, { title: "mossy_bricks" }])
  })

  it("keeps everything for a blank query and nothing for a miss", () => {
    expect(filterConversations(items, "  ")).toHaveLength(3)
    expect(filterConversations(items, "zzz")).toEqual([])
  })
})

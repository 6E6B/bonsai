import { describe, expect, it } from "vitest"
import { duration, hours, isPlaytimeRange, utcOffsetMinutes } from "./playtime"

describe("duration", () => {
  it("reads hours and minutes", () => {
    expect(duration(0)).toBe("0m")
    expect(duration(30)).toBe("<1m")
    expect(duration(600)).toBe("10m")
    expect(duration(3600)).toBe("1h")
    expect(duration(5400)).toBe("1h 30m")
    expect(duration(3599)).toBe("1h")
  })
})

describe("hours", () => {
  it("keeps one decimal under ten hours", () => {
    expect(hours(0)).toBe("0h")
    expect(hours(5400)).toBe("1.5h")
    expect(hours(36000 + 1800)).toBe("11h")
  })
})

describe("ranges", () => {
  it("knows its names", () => {
    expect(isPlaytimeRange("quarter")).toBe(true)
    expect(isPlaytimeRange("day")).toBe(false)
  })
  it("flips the sign of the JS offset", () => {
    const d = new Date()
    expect(utcOffsetMinutes(d)).toBe(-d.getTimezoneOffset())
  })
})

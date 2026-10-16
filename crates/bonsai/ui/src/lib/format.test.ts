import { describe, expect, it } from "vitest"
import {
  ago,
  bytes,
  compact,
  countdown,
  date,
  full,
  percent,
  setClockFormat,
  setFormatLocale,
  setRelativeDates,
  time,
} from "./format"

describe("compact", () => {
  it("keeps small numbers exact", () => {
    expect(compact(1037)).toBe("1,037")
    expect(compact(9999)).toBe("9,999")
  })

  it("shortens large numbers", () => {
    expect(compact(21_933)).toBe("21.9K")
    expect(compact(121_000)).toBe("121K")
    expect(compact(7_312_004)).toBe("7.3M")
  })
})

describe("percent", () => {
  it("rounds", () => {
    expect(percent(0.904)).toBe("90%")
    expect(percent(1)).toBe("100%")
  })
})

describe("ago", () => {
  const now = Date.UTC(2026, 9, 13, 12)
  it("buckets by unit", () => {
    expect(ago(now - 10_000, now)).toBe("now")
    expect(ago(now - 60_000, now)).toBe("1m ago")
    expect(ago(now - 3 * 3_600_000, now)).toBe("3h ago")
    expect(ago(now - 2 * 86_400_000, now)).toBe("2d ago")
  })

  it("falls back to the date after a week", () => {
    expect(ago(Date.UTC(2026, 7, 4, 12), now)).toBe("Aug 4, 2026")
  })

  it("treats future times as now", () => {
    expect(ago(now + 5000, now)).toBe("now")
  })
})

describe("bytes", () => {
  it("picks a unit and keeps one decimal", () => {
    expect(bytes(0)).toBe("0 B")
    expect(bytes(1536)).toBe("1.5 KB")
    expect(bytes(50 * 1024 * 1024)).toBe("50 MB")
  })
})

describe("countdown", () => {
  it("shows minutes and padded seconds", () => {
    expect(countdown(299.6)).toBe("4:59")
    expect(countdown(65)).toBe("1:05")
    expect(countdown(0)).toBe("0:00")
  })

  it("stops at zero", () => {
    expect(countdown(-3)).toBe("0:00")
  })
})

describe("setFormatLocale", () => {
  it("switches number and date formats, and resets", () => {
    setFormatLocale("de-DE")
    expect(full(1234567)).toBe("1.234.567")
    expect(date(Date.UTC(2026, 7, 4, 12))).toContain("2026")
    setFormatLocale("en-US")
    expect(full(1234567)).toBe("1,234,567")
    expect(date(Date.UTC(2026, 7, 4, 12))).toBe("Aug 4, 2026")
  })

  it("ignores invalid tags", () => {
    setFormatLocale("not a locale!!")
    expect(compact(5)).toBe("5")
    setFormatLocale("en-US")
  })
})

describe("clock and relative times", () => {
  // A local time, so the hour doesn't depend on the test machine's time zone.
  const at = new Date(2026, 7, 4, 16, 5).getTime()

  it("follows the clock setting", () => {
    setFormatLocale("en-US")
    setClockFormat("24h")
    expect(time(at)).toBe("16:05")
    setClockFormat("12h")
    expect(time(at)).toBe("4:05 PM")
    setClockFormat("nonsense")
    expect(time(at)).toBe("4:05 PM") // en-US's own clock
  })

  it("gives the date and time when relative times are off", () => {
    setFormatLocale("en-US")
    setClockFormat("24h")
    setRelativeDates(false)
    expect(ago(at, at + 60_000)).toBe("Aug 4, 2026, 16:05")
    setRelativeDates(true)
    expect(ago(at, at + 60_000)).toBe("1m ago")
    setClockFormat("auto")
  })
})

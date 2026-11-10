import { describe, expect, it } from "vitest"
import type { ModerationView } from "~/lib/ipc/bindings"
import {
  accountProblem,
  accountUnusable,
  activeVoiceBans,
  canReactivate,
  clock,
  duration,
  moderationLabel,
  moderationNext,
  moderationTone,
  voiceBanText,
} from "./moderation"

const m = (kind: ModerationView["kind"], ends: number | null = null): ModerationView => ({
  kind,
  title: "",
  message: "",
  begins: 0,
  ends,
})

describe("moderation", () => {
  it("formats remaining time", () => {
    expect(duration(30)).toBe("under a minute")
    expect(duration(45 * 60)).toBe("45m")
    expect(duration(3 * 3600 + 12 * 60)).toBe("3h 12m")
    expect(duration(2 * 3600)).toBe("2h")
    expect(duration(6 * 86400 + 4 * 3600 + 59)).toBe("6d 4h")
    expect(duration(-5)).toBe("under a minute")
  })

  it("reactivates only warnings and ended bans", () => {
    expect(canReactivate(m("warned"), 0)).toBe(true)
    expect(canReactivate(m("banned", 100), 99)).toBe(false)
    expect(canReactivate(m("banned", 100), 100)).toBe(true)
    expect(canReactivate(m("banned"), 1e12)).toBe(false)
    expect(canReactivate(m("terminated"), 1e12)).toBe(false)
    expect(canReactivate(m("locked"), 1e12)).toBe(false)
    expect(canReactivate(null, 0)).toBe(false)
  })

  it("labels and tones follow the ban clock", () => {
    expect(moderationLabel(m("banned", 100), 50)).toBe("Banned")
    expect(moderationTone(m("banned", 100), 50)).toBe("danger")
    expect(moderationLabel(m("banned", 100), 100)).toBe("Ban ended")
    expect(moderationTone(m("banned", 100), 100)).toBe("warning")
    expect(moderationTone(m("terminated"), 0)).toBe("danger")
    expect(moderationTone(m("warned"), 0)).toBe("warning")
    expect(moderationLabel(null, 0)).toBe("Moderated")
    expect(moderationNext(m("banned", 3600), 0)).toMatch(/^Ends in 1h, /)
    expect(moderationNext(m("terminated"), 0)).toContain("can't be reactivated")
  })

  it("summarizes an account for the rail", () => {
    const ok = { session: "ok" as const, moderation: null, voiceBan: null }
    expect(accountProblem(ok, 0)).toBe("")
    expect(accountProblem({ ...ok, voiceBan: { until: 7200 } }, 0)).toBe("Voice banned for 2h")
    expect(voiceBanText({ until: null }, 0)).toBe("Voice banned")
    const banned = { session: "moderated" as const, moderation: m("banned", 100), voiceBan: null }
    expect(accountProblem(banned, 0)).toBe("Banned")
    expect(accountUnusable(banned, 0)).toBe(true)
    expect(accountUnusable(banned, 100)).toBe(false)
    expect(accountUnusable({ ...banned, session: "ok" }, 0)).toBe(false)
  })

  it("counts down a clock", () => {
    expect(clock(5)).toBe("0:05")
    expect(clock(4 * 60 + 5)).toBe("4:05")
    expect(clock(3 * 3600 + 4 * 60 + 5)).toBe("3:04:05")
    expect(clock(2 * 86400 + 3 * 3600 + 4 * 60 + 5.9)).toBe("2d 03:04:05")
    expect(clock(-1)).toBe("0:00")
  })

  it("lists active voice bans, soonest end first", () => {
    const accounts = [
      { id: 1, voiceBan: null },
      { id: 2, voiceBan: { until: null } },
      { id: 3, voiceBan: { until: 500 } },
      { id: 4, voiceBan: { until: 100 } },
      { id: 5, voiceBan: { until: 50 } },
    ]
    expect(activeVoiceBans(accounts, 60).map((a) => a.id)).toEqual([4, 3, 2])
    expect(activeVoiceBans(accounts, 1000).map((a) => a.id)).toEqual([2])
  })
})

import { describe, expect, it } from "vitest"
import { clock } from "./audio-clock"

describe("clock", () => {
  it("shows minutes and seconds, and hours when needed", () => {
    expect(clock(0)).toBe("0:00")
    expect(clock(7.9)).toBe("0:07")
    expect(clock(95)).toBe("1:35")
    expect(clock(3725)).toBe("1:02:05")
  })

  it("shows a placeholder for unknown lengths", () => {
    expect(clock(Number.NaN)).toBe("-:--")
    expect(clock(Number.POSITIVE_INFINITY)).toBe("-:--")
  })
})

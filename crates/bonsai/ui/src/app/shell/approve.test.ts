import { describe, expect, it } from "vitest"
import { quickCode } from "./approve"

describe("quickCode", () => {
  it("drops spaces and dashes and upper-cases", () => {
    expect(quickCode(" ab-c 12 3")).toBe("ABC123")
  })
  it("keeps at most six characters", () => {
    expect(quickCode("ABCDEFGH")).toBe("ABCDEF")
  })
  it("is empty for nothing usable", () => {
    expect(quickCode("--- ")).toBe("")
  })
})

import { describe, expect, it } from "vitest"
import { groupKey } from "./security"

describe("groupKey", () => {
  it("splits a key into groups of four", () => {
    expect(groupKey("ABCDEFGHIJ")).toBe("ABCD EFGH IJ")
    expect(groupKey(" AB CD ")).toBe("ABCD")
    expect(groupKey("")).toBe("")
  })
})

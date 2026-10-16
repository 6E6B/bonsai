import { describe, expect, it } from "vitest"
import { codeDigits, pinDigits } from "./challenge"

describe("codeDigits", () => {
  it("keeps up to six digits", () => {
    expect(codeDigits("123 456")).toBe("123456")
    expect(codeDigits("12-34-5678")).toBe("123456")
    expect(codeDigits("abc")).toBe("")
  })
})

describe("pinDigits", () => {
  it("keeps up to four digits", () => {
    expect(pinDigits(" 12 34 5")).toBe("1234")
    expect(pinDigits("9")).toBe("9")
  })
})

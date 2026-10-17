import { describe, expect, it } from "vitest"
import { every } from "./plugins"

describe("every", () => {
  it("names the largest whole unit", () => {
    expect(every(7200)).toBe("every 2 h")
    expect(every(300)).toBe("every 5 min")
    expect(every(90)).toBe("every 90 s")
  })
})

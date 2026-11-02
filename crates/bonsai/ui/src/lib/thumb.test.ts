import { describe, expect, it } from "vitest"
import { withRev } from "./thumb"

describe("withRev", () => {
  it("leaves the URL alone without a revision", () => {
    expect(withRev("thumb://localhost/headshot.1.s", undefined)).toBe("thumb://localhost/headshot.1.s")
    expect(withRev("thumb://localhost/headshot.1.s", 0)).toBe("thumb://localhost/headshot.1.s")
  })
  it("adds the revision as a query, after one that is already there", () => {
    expect(withRev("thumb://localhost/headshot.1.s", 2)).toBe("thumb://localhost/headshot.1.s?r=2")
    expect(withRev("thumb://localhost/outfit.1.s?r=2", 5)).toBe("thumb://localhost/outfit.1.s?r=2&r=5")
  })
})

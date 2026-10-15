import { describe, expect, it } from "vitest"
import { describePath } from "./screens"

describe("describePath", () => {
  it("names screens and their sub-paths", () => {
    expect(describePath("/").title).toBe("Home")
    expect(describePath("/friends").title).toBe("Friends")
    expect(describePath("/settings/privacy").title).toBe("Settings")
  })

  it("names entity pages generically", () => {
    expect(describePath("/games/1").title).toBe("Experience")
    expect(describePath("/communities/4").title).toBe("Community")
    expect(describePath("/communities").title).toBe("Communities")
  })
})

import { describe, expect, it } from "vitest"
import { fontChoices } from "./fonts"

describe("fontChoices", () => {
  const google = [
    { family: "Inter", category: "Sans Serif" },
    { family: "Inter Tight", category: "Sans Serif" },
    { family: "Space Grotesk", category: "Sans Serif" },
  ]

  it("lists bundled fonts first and skips Google's copies of them", () => {
    expect(fontChoices("", false, google).map((c) => c.value)).toEqual([
      "Inter",
      "System",
      "Inter Tight",
      "Space Grotesk",
    ])
    expect(fontChoices("", false, google)[0].detail).toBe("Built in")
  })

  it("filters bundled fonts by the query and uses the mono ones for code", () => {
    expect(fontChoices("sys", false, []).map((c) => c.value)).toEqual(["System"])
    expect(fontChoices("jet", true, []).map((c) => c.value)).toEqual(["JetBrains Mono"])
  })
})

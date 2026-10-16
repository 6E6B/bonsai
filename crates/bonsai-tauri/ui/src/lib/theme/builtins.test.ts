import { describe, expect, it } from "vitest"
import { builtinThemes } from "./builtins"
import { runChecks } from "./checks"
import { resolve } from "./theme"

describe("built-in themes", () => {
  const themes = builtinThemes()

  it("are all read without problems", () => {
    expect(themes.map((t) => t.id).sort()).toEqual([
      "abyss",
      "adwaita",
      "adwaita-dark",
      "dark",
      "dusk",
      "ember",
      "fjord",
      "glacier",
      "light",
      "midnight",
      "moss",
      "paper",
      "phosphor",
      "sakura",
    ])
  })

  it.each(themes.map((t) => [t.id, t] as const))("%s keeps text readable", (_, theme) => {
    const checks = runChecks(resolve(theme))
    expect(checks).toHaveLength(7)
    const low = checks.filter((c) => !c.ok).map((c) => `${c.fg} on ${c.bg}: ${c.ratio.toFixed(2)} < ${c.min}`)
    expect(low).toEqual([])
  })
})

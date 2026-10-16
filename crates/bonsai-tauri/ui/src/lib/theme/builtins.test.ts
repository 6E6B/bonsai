import { describe, expect, it } from "vitest"
import { builtinThemes } from "./builtins"
import { contrast, hex } from "./color"
import { resolve } from "./theme"

/** Pairs every theme must keep readable, with the WCAG ratio each needs. */
const PAIRS: [fg: string, bg: string, min: number][] = [
  ["color-fg", "color-canvas", 7],
  ["color-fg", "color-surface", 7],
  ["color-fg", "color-raised", 7],
  ["color-fg-muted", "color-surface", 4.5],
  ["color-fg-muted", "color-canvas", 4.5],
  ["color-on-accent", "color-accent", 3],
  ["color-on-accent", "color-danger", 3],
]

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
    const tokens = resolve(theme)
    const low = PAIRS.map(([fg, bg, min]) => [fg, bg, contrast(hex(tokens[fg]), hex(tokens[bg])), min] as const)
      .filter(([, , ratio, min]) => ratio < min)
      .map(([fg, bg, ratio, min]) => `${fg} on ${bg}: ${ratio.toFixed(2)} < ${min}`)
    expect(low).toEqual([])
  })
})

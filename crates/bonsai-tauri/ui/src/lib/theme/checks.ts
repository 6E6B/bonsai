/**
 * Contrast checks for the editor: the text and fill pairs a theme must keep readable, and a fix
 * that moves one side of a failing pair toward white or black until it passes.
 */
import { BLACK, contrast, flatten, luminance, mix, parseHex, type Rgba, toHex, WHITE } from "./color"
import type { Tokens } from "./tokens"

export type Pair = {
  fg: string
  bg: string
  /** WCAG ratio it needs. */
  min: number
  /** The token the fix changes: the text, or the fill when the text is plain white or black. */
  fix: string
  label: string
}

/** Pairs every theme must keep readable, with the WCAG ratio each needs. */
export const PAIRS: readonly Pair[] = [
  { fg: "color-fg", bg: "color-canvas", min: 7, fix: "color-fg", label: "Text on the page" },
  { fg: "color-fg", bg: "color-surface", min: 7, fix: "color-fg", label: "Text on cards" },
  { fg: "color-fg", bg: "color-raised", min: 7, fix: "color-fg", label: "Text in popovers" },
  { fg: "color-fg-muted", bg: "color-surface", min: 4.5, fix: "color-fg-muted", label: "Muted text on cards" },
  { fg: "color-fg-muted", bg: "color-canvas", min: 4.5, fix: "color-fg-muted", label: "Muted text on the page" },
  { fg: "color-on-accent", bg: "color-accent", min: 3, fix: "color-accent", label: "Text on accent buttons" },
  { fg: "color-on-accent", bg: "color-danger", min: 3, fix: "color-danger", label: "Text on danger buttons" },
]

const REF = /^var\(--([\w-]+)\)$/

/** The opaque color a color token shows, following `var(--…)` references. Translucent colors are
 * painted over `under` (the page by default). Undefined for a missing or non-color token. */
export function colorOf(tokens: Tokens, name: string, under?: Rgba): Rgba | undefined {
  let v = tokens[name]
  for (let i = 0; i < 8 && v; i++) {
    const ref = REF.exec(v)
    if (!ref) break
    v = tokens[ref[1]]
  }
  const c = v ? parseHex(v) : undefined
  if (!c) return undefined
  if (c.a === 255) return c
  const page = under ?? (name === "color-canvas" ? WHITE : colorOf(tokens, "color-canvas"))
  return flatten(c, page ?? BLACK)
}

export type Check = Pair & { ratio: number; ok: boolean }

/** Every pair with its ratio. Pairs naming a token that isn't a color are left out. */
export function runChecks(tokens: Tokens): Check[] {
  const out: Check[] = []
  for (const p of PAIRS) {
    const bg = colorOf(tokens, p.bg)
    const fg = bg && colorOf(tokens, p.fg, bg)
    if (!bg || !fg) continue
    const ratio = contrast(fg, bg)
    out.push({ ...p, ratio, ok: ratio >= p.min })
  }
  return out
}

/**
 * The value to pin on `check.fix` so the pair passes: that color moved toward white or black,
 * whichever side gets away from the other color, in small steps so it stays close to the original
 * hue. Undefined when even pure white or black would not pass.
 */
export function fixValue(tokens: Tokens, check: Pair): string | undefined {
  const moving = colorOf(tokens, check.fix)
  const otherName = check.fix === check.fg ? check.bg : check.fg
  const other = colorOf(tokens, otherName)
  if (!moving || !other) return undefined
  const target = luminance(other) > 0.18 ? BLACK : WHITE
  // Text gets darker on light fills, a fill gets darker under white text: away from `other`.
  const ratioWith = (c: Rgba) => contrast(c, other)
  // Aim a little over the line so rounding to hex can't drop it back under.
  const goal = check.min + 0.05
  for (let t = 0; t <= 1.0001; t += 0.01) {
    const c = mix(moving, target, t)
    if (ratioWith(c) >= goal) return toHex({ ...c, a: 255 })
  }
  return undefined
}

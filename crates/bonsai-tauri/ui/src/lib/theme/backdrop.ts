/**
 * The background picture (Settings → Appearance → Background). The picture sits behind the page
 * area, and the page and the cards on it turn see-through so it shows. The window chrome and
 * popovers stay opaque so menus and dialogs read the same as without one.
 */
import { colorOf } from "./checks"
import { toHex, withAlpha } from "./color"
import type { Tokens } from "./tokens"

/** Tokens that turn see-through over the picture: the page and the cards on it. */
export const SEE_THROUGH = ["color-canvas", "color-surface"] as const

/** Overrides for `tokens` with the page and cards at `opacity` (0–1). `backdrop-base` keeps the
 * opaque page color for under the picture. */
export function seeThrough(tokens: Tokens, opacity: number): Tokens {
  const a = Math.min(1, Math.max(0, opacity)) * 255
  const out: Tokens = {}
  const canvas = colorOf(tokens, "color-canvas")
  if (canvas) out["backdrop-base"] = toHex(canvas)
  for (const name of SEE_THROUGH) {
    const c = colorOf(tokens, name)
    if (c) out[name] = toHex(withAlpha(c, a))
  }
  return out
}

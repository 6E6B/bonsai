/**
 * Small sRGB color helpers for theme derivation. Channels are 0–255 floats, alpha included, so
 * `#ffffff0f` round-trips exactly; `toHex` rounds once at the end.
 */
export type Rgba = { r: number; g: number; b: number; a: number }

const HEX = /^#([0-9a-f]{3,4}|[0-9a-f]{6}|[0-9a-f]{8})$/i

export function parseHex(s: string): Rgba | undefined {
  const m = HEX.exec(s.trim())
  if (!m) return undefined
  let h = m[1]
  if (h.length <= 4) h = [...h].map((c) => c + c).join("")
  const n = (i: number) => Number.parseInt(h.slice(i, i + 2), 16)
  return { r: n(0), g: n(2), b: n(4), a: h.length === 8 ? n(6) : 255 }
}

/** Parses a literal color, throwing on bad input. For constants in code only. */
export function hex(s: string): Rgba {
  const c = parseHex(s)
  if (!c) throw new Error(`not a hex color: ${s}`)
  return c
}

const byte = (v: number) => Math.min(255, Math.max(0, Math.round(v)))
const pair = (v: number) => byte(v).toString(16).padStart(2, "0")

/** `#rrggbb`, or `#rrggbbaa` when not opaque. Lowercase, like `theme.css`. */
export function toHex(c: Rgba): string {
  const a = byte(c.a)
  return `#${pair(c.r)}${pair(c.g)}${pair(c.b)}${a === 255 ? "" : pair(a)}`
}

/** `t` of the way from `a` to `b`, per channel. */
export function mix(a: Rgba, b: Rgba, t: number): Rgba {
  const at = (x: number, y: number) => x + (y - x) * t
  return { r: at(a.r, b.r), g: at(a.g, b.g), b: at(a.b, b.b), a: at(a.a, b.a) }
}

/** The same color with alpha `a` (0–255). */
export function withAlpha(c: Rgba, a: number): Rgba {
  return { ...c, a }
}

/** WCAG relative luminance of the opaque color. */
export function luminance(c: Rgba): number {
  const lin = (v: number) => {
    const s = v / 255
    return s <= 0.04045 ? s / 12.92 : ((s + 0.055) / 1.055) ** 2.4
  }
  return 0.2126 * lin(c.r) + 0.7152 * lin(c.g) + 0.0722 * lin(c.b)
}

/** WCAG contrast ratio, 1–21. Alpha is ignored; flatten translucent colors first. */
export function contrast(a: Rgba, b: Rgba): number {
  const [hi, lo] = [luminance(a), luminance(b)].sort((x, y) => y - x)
  return (hi + 0.05) / (lo + 0.05)
}

/** `top` painted over the opaque `bottom`. */
export function flatten(top: Rgba, bottom: Rgba): Rgba {
  return { ...mix(bottom, top, top.a / 255), a: 255 }
}

export const WHITE: Rgba = { r: 255, g: 255, b: 255, a: 255 }
export const BLACK: Rgba = { r: 0, g: 0, b: 0, a: 255 }

/** White text unless it falls under 3:1 on `fill`, then black. */
export function onFill(fill: Rgba): Rgba {
  return contrast(WHITE, fill) >= 3 ? WHITE : BLACK
}

/**
 * Seeds → every token. A theme sets a dozen seeds and gets the whole look; `tokens` in the theme
 * document then pin single values. The mix amounts are fitted so the default seeds reproduce the
 * hand-tuned values in `theme.css` (see `builtin.test.ts`).
 */
import { BLACK, hex, mix, onFill, parseHex, type Rgba, toHex, WHITE, withAlpha } from "./color"
import type { Tokens } from "./tokens"

export type Base = "dark" | "light"

export type ShadowSeeds = {
  /** Drop shadows under things. */
  depth: number
  /** The 1px outline around controls and panels. */
  ring: number
  /** The line of light along top edges. */
  highlight: number
}

export type Seeds = {
  background: string
  foreground: string
  accent: string
  success: string
  warning: string
  danger: string
  /** How far surfaces step away from the page. 1 is the default look, 0 is flat. */
  contrast: number
  /** Multiplies every radius. */
  radius: number
  /** Multiplies the alpha of each shadow layer. 1 each is the default look. */
  shadow: ShadowSeeds
  font: FontSeed<FontId>
  mono: FontSeed<MonoId>
}

/** A bundled font by id, or a Google Fonts family the backend downloads and serves over `font://`. */
export type FontSeed<Id extends string> = Id | { google: string }

/** Bundled fonts, and the fallback stacks behind a Google font. The webview never loads fonts from
 * the network; Google fonts come from disk (see `fonts.ts`). */
export const FONTS = {
  inter: { label: "Inter", stack: '"Inter", system-ui, sans-serif' },
  system: { label: "System", stack: "system-ui, sans-serif" },
} as const
export const MONOS = {
  jetbrains: { label: "JetBrains Mono", stack: '"JetBrainsMono Nerd Font Mono", ui-monospace, monospace' },
  system: { label: "System", stack: "ui-monospace, monospace" },
} as const
export type FontId = keyof typeof FONTS
export type MonoId = keyof typeof MONOS

/** A Google Fonts family name: letters, digits and single spaces, as in `fonts.rs`. */
export const GOOGLE_FAMILY = /^[A-Za-z0-9]+( [A-Za-z0-9]+)*$/

/** The font-family stack for a seed. A Google family goes first, with the default bundled font
 * behind it while it downloads or if it fails. */
export function fontStack<Id extends string>(
  seed: FontSeed<Id> | undefined,
  options: Record<Id, { stack: string }>,
  fallback: Id,
): string {
  if (typeof seed === "string") return (options[seed] ?? options[fallback]).stack
  if (seed && seed.google.length <= 64 && GOOGLE_FAMILY.test(seed.google))
    return `"${seed.google}", ${options[fallback].stack}`
  return options[fallback].stack
}

/** The Google family a seed names, if any. */
export const googleFamily = (seed: FontSeed<string>): string | undefined =>
  typeof seed === "string" ? undefined : seed.google

/** How a font seed reads in the UI: the bundled font's label or the Google family. */
export function fontLabel<Id extends string>(seed: FontSeed<Id>, options: Record<Id, { label: string }>): string {
  return typeof seed === "string" ? (options[seed]?.label ?? seed) : seed.google
}

export const SEED_RANGES = {
  contrast: [0, 3],
  radius: [0, 4],
  shadow: [0, 3],
} as const

const DEFAULT_SHADOW: ShadowSeeds = { depth: 1, ring: 1, highlight: 1 }

export const DEFAULT_SEEDS: Record<Base, Seeds> = {
  dark: {
    background: "#1a1a1a",
    foreground: "#fafafa",
    accent: "#3d7dff",
    success: "#3fbf7f",
    warning: "#e8a23a",
    danger: "#ef5a5a",
    contrast: 1,
    radius: 1,
    shadow: DEFAULT_SHADOW,
    font: "inter",
    mono: "jetbrains",
  },
  light: {
    background: "#ffffff",
    foreground: "#161616",
    accent: "#2563eb",
    success: "#16803c",
    warning: "#b25e09",
    danger: "#d92d2d",
    contrast: 1,
    radius: 1,
    shadow: DEFAULT_SHADOW,
    font: "inter",
    mono: "jetbrains",
  },
}

/* ── Shadows ──
 * Each layer names the seed that scales its alpha. Rings and highlights are drawn at 1px with
 * half alpha rather than 0.5px: WebKitGTK at 1x snaps 0.5px edges but not rounded corners. */
type Layer = [knob: keyof ShadowSeeds, geometry: string, color: string, inset?: "inset"]
type Shadows = Record<string, Layer[]>

const SHARED_SHADOWS: Shadows = {
  "shadow-knob": [["depth", "0 0.5px 0.5px", "#00000066"]],
  "text-shadow-fill": [["depth", "0 0.5px 0.5px", "#0000004d"]],
}

const SHADOWS: Record<Base, Shadows> = {
  dark: {
    ...SHARED_SHADOWS,
    "shadow-control": [
      ["depth", "0 1px 2px", "#00000066"],
      ["ring", "0 0 0 1px", "#ffffff1a"],
      ["highlight", "0 -1px 0", "#ffffff0d"],
    ],
    "shadow-fill": [
      ["depth", "0 1px 2px", "#00000066"],
      ["ring", "0 0 0 1px", "#ffffff33"],
      ["highlight", "0 -1px 0", "#ffffff26"],
    ],
    "shadow-field": [
      ["depth", "0 1px 2px", "#00000066"],
      ["ring", "0 0 0 1px", "#ffffff21"],
      ["highlight", "0 -1px 0", "#ffffff0d"],
    ],
    "shadow-media": [
      ["depth", "0 1px 2px", "#00000066"],
      ["ring", "0 0 0 1px", "#ffffff14"],
      ["highlight", "0 -1px 0", "#ffffff0d"],
    ],
    "shadow-well": [
      ["highlight", "0 -1px 0", "#ffffff0d", "inset"],
      ["ring", "0 0 0 1px", "#ffffff14", "inset"],
    ],
    "shadow-well-on": [
      ["highlight", "0 -1px 0", "#ffffff0d", "inset"],
      ["ring", "0 0 0 1px", "#ffffff14", "inset"],
    ],
    "shadow-key": [
      ["depth", "0 -1px 0", "#00000059", "inset"],
      ["ring", "0 0 0 1px", "#ffffff14"],
      ["depth", "0 1px 1px", "#00000066"],
    ],
    "shadow-raised": [
      ["depth", "0 2px 4px", "#0000004d"],
      ["depth", "0 1px 2px", "#0000004d"],
      ["ring", "0 0 0 1px", "#ffffff14"],
      ["highlight", "0 -1px 0", "#ffffff08"],
    ],
    "shadow-popover": [
      ["depth", "0 8px 16px", "#0000004d"],
      ["depth", "0 4px 8px", "#0000004d"],
      ["ring", "0 0 0 1px", "#ffffff14"],
      ["highlight", "0 -1px 0", "#ffffff08"],
    ],
    "shadow-dialog": [
      ["depth", "0 16px 32px", "#0000004d"],
      ["depth", "0 8px 16px", "#0000004d"],
      ["ring", "0 0 0 1px", "#ffffff14"],
      ["highlight", "0 -1px 0", "#ffffff08"],
    ],
  },
  light: {
    ...SHARED_SHADOWS,
    "shadow-control": [
      ["depth", "0 1px 1.5px", "#0000001a"],
      ["ring", "0 0 0 1px", "#00000012"],
    ],
    "shadow-fill": [
      ["depth", "0 1px 1.5px", "#00000033"],
      ["ring", "0 0 0 1px", "#3a3a3a80"],
      ["highlight", "0 1px 2px", "#ffffff24", "inset"],
      ["depth", "0 -1px 2px", "#0000000f", "inset"],
    ],
    "shadow-field": [
      ["depth", "0 1px 1.5px", "#0000001a"],
      ["ring", "0 0 0 1px", "#00000012"],
    ],
    "shadow-media": [
      ["depth", "0 1px 1.5px", "#0000001a"],
      ["ring", "0 0 0 1px", "#00000012"],
    ],
    "shadow-well": [
      ["depth", "0 1px 1px", "#00000014", "inset"],
      ["depth", "0 0.5px 0.5px", "#00000014", "inset"],
      ["ring", "0 0 0 1px", "#0000000d", "inset"],
    ],
    "shadow-well-on": [
      ["depth", "0 2px 2px", "#0000001a", "inset"],
      ["depth", "0 1px 1px", "#0000001a", "inset"],
      ["ring", "0 0 0 1px", "#0000001a", "inset"],
    ],
    "shadow-key": [
      ["depth", "0 -1px 0", "#0000001a", "inset"],
      ["ring", "0 0 0 1px", "#00000014"],
      ["depth", "0 1px 1px", "#0000001a"],
    ],
    "shadow-raised": [
      ["depth", "0 2px 4px", "#0000000a"],
      ["depth", "0 1px 2px -1px", "#00000014"],
      ["ring", "0 0 0 1px", "#00000010"],
    ],
    "shadow-popover": [
      ["depth", "0 8px 16px", "#0000000a"],
      ["depth", "0 4px 8px", "#00000014"],
      ["ring", "0 0 0 1px", "#00000010"],
    ],
    "shadow-dialog": [
      ["depth", "0 16px 32px", "#0000000a"],
      ["depth", "0 8px 16px", "#00000014"],
      ["ring", "0 0 0 1px", "#00000010"],
    ],
  },
}

function shadow(layers: Layer[], seeds: ShadowSeeds): string {
  return layers
    .map(([knob, geometry, color, inset]) => {
      const c = hex(color)
      const css = `${geometry} ${toHex(withAlpha(c, c.a * seeds[knob]))}`
      return inset ? `inset ${css}` : css
    })
    .join(", ")
}

/* ── Constants ── Same in both bases unless a theme pins them. */
const FIXED: Tokens = {
  "color-verified": "#4d94ff", // Roblox's checkmark blue; matches assets/verified-badge.svg
  "color-media-chip": "#000000a6",
  "color-on-media": "#ffffff",
  "color-presence-online": "var(--color-accent)",
  "color-presence-game": "var(--color-success)",
  "color-presence-studio": "var(--color-warning)",
  "color-presence-offline": "var(--color-fg-faint)",
  "color-tag-red": "#ef5a5a",
  "color-tag-orange": "#ef8a3a",
  "color-tag-yellow": "#e8c53a",
  "color-tag-green": "#3fbf7f",
  "color-tag-blue": "#3d7dff",
  "color-tag-purple": "#9b6bff",
  "color-tag-pink": "#ef6bb4",
  "color-knob": "#f2f2f2",
  "color-knob-edge": "#ffffff",
}

const TEXT: [name: string, size: number, line: number][] = [
  ["xs", 11, 16],
  ["sm", 12, 16],
  ["base", 13, 20],
  ["lg", 15, 22],
  ["xl", 18, 24],
  ["2xl", 22, 28],
]
const RADII: [name: string, px: number][] = [
  ["xs", 2],
  ["sm", 4],
  ["md", 6],
  ["lg", 8],
]

const px = (n: number) => `${Math.round(n * 2) / 2}px`

/** Seeds that fail to parse fall back to the base's default. */
function color(seed: string, fallback: string): Rgba {
  return parseHex(seed) ?? hex(fallback)
}

export function derive(base: Base, seeds: Seeds): Tokens {
  const d = DEFAULT_SEEDS[base]
  const bg = color(seeds.background, d.background)
  const fg = color(seeds.foreground, d.foreground)
  const accent = color(seeds.accent, d.accent)
  const success = color(seeds.success, d.success)
  const warning = color(seeds.warning, d.warning)
  const danger = color(seeds.danger, d.danger)
  const k = seeds.contrast
  const dark = base === "dark"
  // Overlays are white on dark and black on light, so they lighten or darken any fill.
  const ink = dark ? WHITE : BLACK
  const overlay = (a: number) => toHex(withAlpha(ink, a))
  const c = toHex

  const accentHover = dark ? mix(accent, WHITE, 0.151) : mix(accent, BLACK, 0.15)
  const colors: Tokens = dark
    ? {
        "color-deep": c(mix(bg, BLACK, 0.3 * k)),
        "color-canvas": c(bg),
        "color-surface": c(mix(bg, fg, 0.036 * k)),
        "color-raised": c(mix(bg, fg, 0.0625 * k)),
        "color-hover": overlay(0x0f),
        "color-active": overlay(0x1a),
        "color-field": overlay(0x0d),
        "color-control": overlay(0x0f),
        "color-control-hover": overlay(0x17),
        "color-fg-muted": c(mix(fg, bg, 0.34)),
        "color-fg-faint": c(mix(fg, bg, 0.545)),
        "color-accent-soft": c(withAlpha(accent, 0x29)),
        "color-contrast": c(mix(bg, fg, 0.295)),
        "color-focus": c(accentHover),
        "color-success-soft": c(withAlpha(success, 0x24)),
        "color-warning-soft": c(withAlpha(warning, 0x24)),
        "color-danger-hover": c(mix(danger, WHITE, 0.15)),
        "color-danger-soft": c(withAlpha(danger, 0x24)),
        "color-scrim": "#00000099",
        "color-track": c(mix(bg, fg, 0.295)),
        "color-hover-on-fill": overlay(0x1f),
        "color-pressed-on-fill": overlay(0x3d),
      }
    : {
        "color-deep": c(mix(bg, fg, 0.0215 * k)),
        "color-canvas": c(bg),
        "color-surface": c(mix(bg, fg, 0.0215 * k)),
        "color-raised": c(bg),
        "color-hover": overlay(0x0a),
        "color-active": overlay(0x14),
        "color-field": c(bg),
        "color-control": c(bg),
        "color-control-hover": c(mix(bg, fg, 0.0558 * k)),
        "color-fg-muted": c(mix(fg, bg, 0.3)),
        "color-fg-faint": c(mix(fg, bg, 0.455)),
        "color-accent-soft": c(withAlpha(accent, 0x1f)),
        "color-contrast": c(mix(fg, bg, 0.06)),
        "color-focus": c(accent),
        "color-success-soft": c(withAlpha(success, 0x1a)),
        "color-warning-soft": c(withAlpha(warning, 0x1a)),
        "color-danger-hover": c(mix(danger, BLACK, 0.1)),
        "color-danger-soft": c(withAlpha(danger, 0x1a)),
        "color-scrim": "#00000066",
        "color-track": c(mix(bg, fg, 0.155)),
        "color-hover-on-fill": overlay(0x3d),
        "color-pressed-on-fill": overlay(0x66),
      }

  const out: Tokens = {
    ...FIXED,
    ...colors,
    "color-fg": c(fg),
    "color-line": overlay(0x1a),
    "color-line-strong": overlay(0x33),
    "color-accent": c(accent),
    "color-accent-hover": c(accentHover),
    "color-on-accent": c(onFill(accent)),
    "color-success": c(success),
    "color-warning": c(warning),
    "color-danger": c(danger),
    "font-sans": fontStack(seeds.font, FONTS, "inter"),
    "font-mono": fontStack(seeds.mono, MONOS, "jetbrains"),
    "radius-full": "9999px",
  }
  for (const [name, layers] of Object.entries(SHADOWS[base])) out[name] = shadow(layers, seeds.shadow)
  for (const [name, size, line] of TEXT) {
    out[`text-${name}`] = px(size)
    out[`text-${name}--line-height`] = px(line)
  }
  for (const [name, r] of RADII) out[`radius-${name}`] = px(r * seeds.radius)
  return out
}

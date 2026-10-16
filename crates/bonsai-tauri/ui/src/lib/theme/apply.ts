/**
 * Puts a theme on the window. Each resolved token goes on `<html>` as an inline custom property,
 * which wins over `theme.css`, and every Tailwind utility already reads `var(--…)`, so the whole
 * app changes in one frame. `data-theme` and `color-scheme` follow the theme's base, so native
 * controls and scrollbars match. With no theme for a base, `theme.css` alone styles it.
 */
import { seeThrough } from "./backdrop"
import { BLACK, mix, onFill, parseHex, toHex, WHITE, withAlpha } from "./color"
import type { Base } from "./derive"
import { resolve, type Theme } from "./theme"

export type Mode = "system" | "dark" | "light"

let mode: Mode = "system"
let themes: Partial<Record<Base, Theme>> = {}
let media: MediaQueryList | undefined
let set = new Set<string>()
/** A theme being edited, shown in place of the saved one for its base. */
let draft: Theme | undefined
/** How opaque the page stays over the background picture (0–1), or undefined with none. */
let panels: number | undefined
/** A color tag (`red`, `blue`…) to use as the accent in place of the theme's, or undefined. */
let accentTag: string | undefined

function base(): Base {
  if (mode !== "system") return mode
  return (media?.matches ?? true) ? "dark" : "light"
}

function paint() {
  const b = base()
  const root = document.documentElement
  root.dataset.theme = b
  root.style.colorScheme = b
  const theme = draft?.base === b ? draft : themes[b]
  const tokens = theme ? resolve(theme) : {}
  if (panels !== undefined) Object.assign(tokens, seeThrough(tokens, panels))
  for (const name of set) if (!(name in tokens)) root.style.removeProperty(`--${name}`)
  if (accentTag) {
    const tag = `color-tag-${accentTag}`
    const value = tokens[tag] ?? getComputedStyle(root).getPropertyValue(`--${tag}`).trim()
    Object.assign(tokens, accentTokens(value, b === "dark"))
  }
  for (const [name, value] of Object.entries(tokens)) root.style.setProperty(`--${name}`, value)
  set = new Set(Object.keys(tokens))
}

/** The base `mode` shows now: its own, or in System mode the OS's. */
export function shownBase(m: Mode): Base {
  if (m !== "system") return m
  const dark =
    typeof window.matchMedia === "function" ? window.matchMedia("(prefers-color-scheme: dark)").matches : true
  return dark ? "dark" : "light"
}

/** Shows `byBase.dark` or `byBase.light` by mode. `system` follows the OS and updates when it changes. */
export function applyAppearance(next: Mode, byBase: Partial<Record<Base, Theme>> = {}) {
  mode = next
  themes = byBase
  if (!media && typeof window.matchMedia === "function") {
    media = window.matchMedia("(prefers-color-scheme: dark)")
    media.addEventListener("change", paint)
  }
  paint()
}

/** Shows `theme` across the app in place of the saved theme for its base, or stops when
 * undefined. The theme editor's "Apply to app" switch uses it. */
export function previewTheme(theme: Theme | undefined) {
  draft = theme
  paint()
}

/** Makes the page and cards see-through at `opacity` (0–1) over the background picture, or
 * opaque again when undefined. */
export function setSeeThrough(opacity: number | undefined) {
  if (panels === opacity) return
  panels = opacity
  paint()
}

/** The accent tokens for `value`, the same way a theme derives them from its accent seed. Empty
 * when `value` is not a hex color. */
function accentTokens(value: string, dark: boolean): Record<string, string> {
  const accent = parseHex(value)
  if (!accent) return {}
  const hover = dark ? mix(accent, WHITE, 0.151) : mix(accent, BLACK, 0.15)
  return {
    "color-accent": toHex(accent),
    "color-accent-hover": toHex(hover),
    "color-accent-soft": toHex(withAlpha(accent, dark ? 0x29 : 0x1f)),
    "color-focus": toHex(dark ? hover : accent),
    "color-contrast": toHex(accent),
    "color-on-accent": toHex(onFill(accent)),
  }
}

/** Uses the color tag `tag` (`red`, `blue`…) as the accent, or the theme's again when
 * undefined. "Use the account's color as accent" sets it to the active account's tag. */
export function setAccentTag(tag: string | undefined) {
  if (accentTag === tag) return
  accentTag = tag
  paint()
}

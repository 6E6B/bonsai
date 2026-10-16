/**
 * The background picture (Settings → Appearance → Background). The picture sits behind the page
 * area, and the page and the cards on it turn see-through so it shows. The window chrome and
 * popovers stay opaque so menus and dialogs read the same as without one.
 */
import { createStore } from "solid-js/store"
import type { BackgroundView } from "~/lib/ipc/bindings"
import { colorOf } from "./checks"
import { toHex, withAlpha } from "./color"
import type { Tokens } from "./tokens"

/** Tokens that turn see-through over the picture: the page and the cards on it. */
export const SEE_THROUGH = ["color-canvas", "color-surface"] as const

/** Overrides for `tokens` with the page and cards at `opacity` (0–1). `backdrop-base` keeps the
 * opaque page color for under the picture; `backdrop-frost` is the `backdrop-filter` cards use to
 * blur what shows through them by `frost` px. */
export function seeThrough(tokens: Tokens, opacity: number, frost = 0): Tokens {
  const a = Math.min(1, Math.max(0, opacity)) * 255
  const out: Tokens = { "backdrop-frost": frost > 0 ? `blur(${frost}px)` : "none" }
  const canvas = colorOf(tokens, "color-canvas")
  if (canvas) out["backdrop-base"] = toHex(canvas)
  for (const name of SEE_THROUGH) {
    const c = colorOf(tokens, name)
    if (c) out[name] = toHex(withAlpha(c, a))
  }
  return out
}

/** The picture's adjustable levels, as `view.background_level` names them. */
export type BackdropLevels = Omit<BackgroundView, "file">
export type BackdropLevel = keyof BackdropLevels

/** The effect sliders, in the order Settings shows them. `max` matches `Background::LEVELS` in
 * rbx-core. */
export const EFFECTS: readonly { key: BackdropLevel; label: string; max: number; step: number; unit: string }[] = [
  { key: "blur", label: "Blur", max: 40, step: 1, unit: "px" },
  { key: "dim", label: "Dim", max: 100, step: 5, unit: "%" },
  { key: "saturation", label: "Saturation", max: 200, step: 5, unit: "%" },
  { key: "tint", label: "Accent tint", max: 100, step: 5, unit: "%" },
  { key: "vignette", label: "Vignette", max: 100, step: 5, unit: "%" },
  { key: "scanlines", label: "Scanlines", max: 100, step: 5, unit: "%" },
  { key: "grain", label: "Grain", max: 100, step: 5, unit: "%" },
]

/** How to draw the picture for `b`: a CSS filter on the image, how far it overhangs each edge so
 * blur doesn't fade them, and the opacity (0–1) of each overlay. */
export function backdropLayers(b: BackdropLevels) {
  const filter = [
    b.blur > 0 && `blur(${b.blur}px)`,
    b.dim > 0 && `brightness(${(100 - b.dim) / 100})`,
    b.saturation !== 100 && `saturate(${b.saturation / 100})`,
  ].filter(Boolean)
  return {
    filter: filter.length ? filter.join(" ") : undefined,
    overhang: b.blur * 2,
    image: b.image / 100,
    tint: b.tint / 100,
    vignette: b.vignette / 100,
    scanlines: b.scanlines / 100,
    grain: b.grain / 100,
  }
}

/** Slider values not saved yet, shown at once while dragging. Each one goes away when the saved
 * value catches up (`settleBackdrop`) or the save fails. */
const [draft, setDraft] = createStore<Partial<BackdropLevels>>({})

/** Shows `value` for `key` before it is saved, or drops the preview when undefined. */
export function previewBackdrop(key: BackdropLevel, value: number | undefined) {
  setDraft(key, value)
}

/** `b` with any unsaved slider values on top. Reactive. */
export function withDraft<T extends BackdropLevels>(b: T): T {
  return { ...b, ...draft }
}

/** Drops previews that the saved background `b` now matches. */
export function settleBackdrop(b: BackdropLevels | null) {
  for (const key of Object.keys(draft) as BackdropLevel[]) {
    if (!b || b[key] === draft[key]) setDraft(key, undefined)
  }
}

/** The part of a `width`×`height` box the picture covers, as a CSS `path()`: everything below
 * `top`, plus `tab` reaching up from `top` with rounded top corners, so the picture runs from the
 * page into the active tab. */
export function cutoutPath(
  width: number,
  height: number,
  top: number,
  tab?: { left: number; right: number; top: number; radius: number },
) {
  const n = (v: number) => Math.round(v * 2) / 2
  const below = `V${n(top)} H${n(width)} V${n(height)} H0 Z`
  if (!tab || tab.right - tab.left < 1 || tab.top >= top) return `path("M0 ${n(top)} H${n(width)} V${n(height)} H0 Z")`
  const r = Math.min(tab.radius, (tab.right - tab.left) / 2, top - tab.top)
  const [l, rt, t] = [n(tab.left), n(tab.right), n(tab.top)]
  return `path("M0 ${n(top)} H${l} V${n(t + r)} A${n(r)} ${n(r)} 0 0 1 ${n(l + r)} ${t} H${n(rt - r)} A${n(r)} ${n(r)} 0 0 1 ${rt} ${n(t + r)} ${below}")`
}

/**
 * The theme document: what is saved, exported, shared as a code and (later) published to a hub.
 * It is data only: hex colors, px lengths, shadow layers, bundled font ids and Google Fonts family
 * names. Background images use validated external HTTPS links; themes contain no raw CSS or code.
 */
import { parseHex } from "./color"
import {
  type Base,
  DEFAULT_SEEDS,
  derive,
  FONTS,
  type FontSeed,
  GOOGLE_FAMILY,
  MONOS,
  SEED_RANGES,
  type Seeds,
} from "./derive"
import { TOKEN, type Tokens } from "./tokens"

/** Bump when the document changes shape in a way older apps can't read. */
export const SCHEMA = 1

export type Theme = {
  schema: number
  /** Stable across edits and versions. Built-ins use their name, user themes a UUID. */
  id: string
  name: string
  author: string
  version: string
  base: Base
  background?: ThemeBackground
  seeds: Seeds
  /** Exact values that win over the seeds, keyed by token name. */
  tokens: Tokens
}

export type ThemeBackground = {
  url: string
  image: number
  panels: number
  blur: number
  dim: number
  saturation: number
  tint: number
  vignette: number
  scanlines: number
  grain: number
}

export const BACKGROUND_DEFAULTS = {
  image: 70,
  panels: 75,
  blur: 0,
  dim: 0,
  saturation: 100,
  tint: 0,
  vignette: 0,
  scanlines: 0,
  grain: 0,
}

export function validBackgroundUrl(value: string): boolean {
  try {
    const url = new URL(value)
    return (
      value.startsWith("https://") &&
      value.length <= 4096 &&
      url.protocol === "https:" &&
      !!url.hostname &&
      !url.username &&
      !url.password
    )
  } catch {
    return false
  }
}

function readBackground(raw: unknown): ThemeBackground | undefined {
  if (raw === undefined || raw === null) return undefined
  if (!isObject(raw) || typeof raw.url !== "string" || !validBackgroundUrl(raw.url))
    throw new Error("Use an external https image link.")
  const levels = { ...BACKGROUND_DEFAULTS }
  for (const key of Object.keys(levels) as (keyof typeof levels)[]) {
    const value = raw[key]
    const max = key === "blur" ? 40 : key === "saturation" ? 200 : 100
    if (value !== undefined) {
      if (typeof value !== "number" || !Number.isInteger(value) || value < 0 || value > max)
        throw new Error(`Background ${key} is out of range.`)
      levels[key] = value
    }
  }
  return { url: raw.url, ...levels }
}

export type ReadResult = { ok: true; theme: Theme; problems: string[] } | { ok: false; error: string }

const ID = /^[a-z0-9][a-z0-9-]{0,63}$/
const LENGTH = /^(-?\d{1,4}(\.\d{1,2})?px|0)$/
const MAX_LAYERS = 6
const MAX_TEXT = 64

const isObject = (v: unknown): v is Record<string, unknown> => typeof v === "object" && v !== null && !Array.isArray(v)
const text = (v: unknown, fallback: string) => (typeof v === "string" ? v.trim().slice(0, MAX_TEXT) : fallback)

function shadowLayer(s: string): boolean {
  const parts = s.trim().split(/\s+/)
  if (parts[0] === "inset") parts.shift()
  const color = parts.pop()
  if (!color || !parseHex(color)) return false
  return parts.length >= 2 && parts.length <= 4 && parts.every((p) => LENGTH.test(p))
}

/**
 * The CSS value for one token, or undefined when `raw` is not a valid value of its kind.
 * Colors may also point at another color token (`var(--color-accent)`); see `resolve`.
 */
export function tokenValue(name: string, raw: unknown): string | undefined {
  const token = TOKEN.get(name)
  if (!token || typeof raw !== "string") return undefined
  const v = raw.trim()
  switch (token.kind) {
    case "color": {
      if (parseHex(v)) return v.toLowerCase()
      const ref = /^var\(--(color-[a-z0-9-]+)\)$/.exec(v)
      return ref && ref[1] !== name && TOKEN.get(ref[1])?.kind === "color" ? v : undefined
    }
    case "length":
      return LENGTH.test(v) ? v : undefined
    case "shadow": {
      const layers = v.split(",")
      return layers.length <= MAX_LAYERS && layers.every(shadowLayer)
        ? layers.map((l) => l.trim()).join(", ")
        : undefined
    }
    case "font":
      if (name === "font-mono") return v in MONOS ? MONOS[v as keyof typeof MONOS].stack : undefined
      return v in FONTS ? FONTS[v as keyof typeof FONTS].stack : undefined
  }
}

function readSeeds(raw: unknown, base: Base, problems: string[]): Seeds {
  const d = DEFAULT_SEEDS[base]
  const r = isObject(raw) ? raw : {}
  const colorSeed = (key: "background" | "foreground" | "accent" | "success" | "warning" | "danger") => {
    const v = r[key]
    if (v === undefined) return d[key]
    if (typeof v === "string" && parseHex(v)) return v.trim().toLowerCase()
    problems.push(`seeds.${key}: not a hex color`)
    return d[key]
  }
  const num = (v: unknown, fallback: number, [lo, hi]: readonly [number, number], key: string) => {
    if (v === undefined) return fallback
    if (typeof v === "number" && Number.isFinite(v)) return Math.min(hi, Math.max(lo, v))
    problems.push(`seeds.${key}: not a number`)
    return fallback
  }
  const sh = isObject(r.shadow) ? r.shadow : {}
  const pick = <T extends string>(v: unknown, options: Record<T, unknown>, fallback: FontSeed<T>, key: string) => {
    if (v === undefined) return fallback
    if (typeof v === "string" && v in options) return v as T
    if (isObject(v) && typeof v.google === "string" && v.google.length <= 64 && GOOGLE_FAMILY.test(v.google))
      return { google: v.google }
    problems.push(`seeds.${key}: unknown font "${isObject(v) ? JSON.stringify(v) : String(v)}"`)
    return fallback
  }
  return {
    background: colorSeed("background"),
    foreground: colorSeed("foreground"),
    accent: colorSeed("accent"),
    success: colorSeed("success"),
    warning: colorSeed("warning"),
    danger: colorSeed("danger"),
    contrast: num(r.contrast, d.contrast, SEED_RANGES.contrast, "contrast"),
    radius: num(r.radius, d.radius, SEED_RANGES.radius, "radius"),
    shadow: {
      depth: num(sh.depth, d.shadow.depth, SEED_RANGES.shadow, "shadow.depth"),
      ring: num(sh.ring, d.shadow.ring, SEED_RANGES.shadow, "shadow.ring"),
      highlight: num(sh.highlight, d.shadow.highlight, SEED_RANGES.shadow, "shadow.highlight"),
    },
    font: pick(r.font, FONTS, d.font, "font"),
    mono: pick(r.mono, MONOS, d.mono, "mono"),
  }
}

/**
 * Reads a theme from untrusted JSON. Only a missing id, a bad base or a newer schema refuse the
 * whole theme; anything else (unknown tokens from a newer app, bad values) is dropped and listed
 * in `problems` so the import screen can say what was skipped.
 */
export function readTheme(input: unknown): ReadResult {
  if (!isObject(input)) return { ok: false, error: "Not a theme." }
  const schema = input.schema
  if (typeof schema !== "number" || !Number.isInteger(schema) || schema < 1) return { ok: false, error: "Not a theme." }
  if (schema > SCHEMA) return { ok: false, error: "Made with a newer version of the app." }
  if (typeof input.id !== "string" || !ID.test(input.id)) return { ok: false, error: "Theme has no valid id." }
  if (input.base !== "dark" && input.base !== "light") return { ok: false, error: "Theme base must be dark or light." }

  let background: ThemeBackground | undefined
  try {
    background = readBackground(input.background)
  } catch (error) {
    return { ok: false, error: String((error as Error).message) }
  }
  const problems: string[] = []
  const base: Base = input.base
  const tokens: Tokens = {}
  if (isObject(input.tokens)) {
    for (const [name, raw] of Object.entries(input.tokens)) {
      if (!TOKEN.has(name)) problems.push(`${name}: unknown token`)
      else if (tokenValue(name, raw) === undefined) problems.push(`${name}: bad value`)
      else tokens[name] = (raw as string).trim()
    }
  }
  return {
    ok: true,
    problems,
    theme: {
      schema: SCHEMA,
      id: input.id,
      name: text(input.name, "Untitled") || "Untitled",
      author: text(input.author, ""),
      version: text(input.version, "1.0.0") || "1.0.0",
      base,
      ...(background ? { background } : {}),
      seeds: readSeeds(input.seeds, base, problems),
      tokens,
    },
  }
}

/** A theme from JSON the backend already checked, or undefined when it can't be read. */
export function parseTheme(json: string): Theme | undefined {
  try {
    const r = readTheme(JSON.parse(json))
    return r.ok ? r.theme : undefined
  } catch {
    return undefined
  }
}

const REF = /^var\(--([\w-]+)\)$/

/**
 * Every token's CSS value: the seeds' derivation with the pinned tokens on top. References may
 * chain, but a cycle would make every token in it invalid, so pinned tokens in one fall back to
 * their derived value. Derived values never form cycles.
 */
export function resolve(theme: Theme): Tokens {
  const derived = derive(theme.base, theme.seeds)
  const out = { ...derived }
  for (const [name, raw] of Object.entries(theme.tokens)) {
    const v = tokenValue(name, raw)
    if (v !== undefined) out[name] = v
  }
  for (const start of Object.keys(out)) {
    const seen: string[] = []
    let at: string | undefined = start
    while (at && !seen.includes(at)) {
      seen.push(at)
      at = REF.exec(out[at] ?? "")?.[1]
    }
    if (at === start) for (const n of seen) out[n] = derived[n]
  }
  return out
}

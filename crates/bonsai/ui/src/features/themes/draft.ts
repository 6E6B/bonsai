/**
 * Themes being edited. Only the active tab's screen is mounted, so an editor's work lives here,
 * not in the component: switching tabs and back keeps the draft, its undo history and whether it
 * shows across the app. Drafts are memory only; quitting the app drops unsaved ones.
 */
import { createEffect, createRoot, createSignal } from "solid-js"
import { createStore, produce, unwrap } from "solid-js/store"
import { fixValue, PAIRS, previewTheme, resolve, runChecks, type Seeds, type Theme, TOKENS } from "~/lib/theme"

export type Draft = {
  theme: Theme
  /** What Revert goes back to and what "changed" compares with: the saved document, or for a
   * new copy the theme it started as. */
  base: Theme
  /** Whether a document with this id is installed. False for a new copy until its first save. */
  onDisk: boolean
  /** The theme this copy was made from, for the banner. */
  from?: string
  past: Theme[]
  future: Theme[]
  /** Edits with the same group (one slider drag, one typed field) make one undo step. */
  group?: string
  /** Pins copied from the source that let go when the seed they follow changes, so changing the
   * accent of a copy of a theme with a pinned accent does what it says. */
  inherited: string[]
}

const HISTORY = 200

/** Two themes are the same document, key order aside. */
export function same(a: unknown, b: unknown): boolean {
  if (a === b) return true
  if (typeof a !== "object" || typeof b !== "object" || a === null || b === null) return false
  if (Array.isArray(a) !== Array.isArray(b)) return false
  const ka = Object.keys(a)
  const kb = Object.keys(b)
  if (ka.length !== kb.length) return false
  return ka.every(
    (k) => Object.hasOwn(b, k) && same((a as Record<string, unknown>)[k], (b as Record<string, unknown>)[k]),
  )
}

/** Whether Save has something to write. */
export const unsaved = (d: Draft) => !d.onDisk || !same(d.theme, d.base)

/** Whether the draft differs from where it started; unchanged drafts are dropped on close. */
export const changed = (d: Draft) => !same(d.theme, d.base)

export function startDraft(theme: Theme, opts: { onDisk: boolean; from?: string; inherit?: boolean }): Draft {
  return {
    theme,
    base: theme,
    onDisk: opts.onDisk,
    from: opts.from,
    past: [],
    future: [],
    group: undefined,
    inherited: opts.inherit ? Object.keys(theme.tokens) : [],
  }
}

/** Replaces the theme as one undo step, or as part of the last one when `group` matches it. */
export function edit(d: Draft, next: Theme, group?: string): Draft {
  if (same(next, d.theme)) return { ...d, group }
  const merge = group !== undefined && group === d.group
  return {
    ...d,
    theme: next,
    past: merge ? d.past : [...d.past, d.theme].slice(-HISTORY),
    future: [],
    group,
  }
}

/** Closes the current undo group, so the next edit is a step of its own. */
export const endGroup = (d: Draft): Draft => ({ ...d, group: undefined })

export function undo(d: Draft): Draft {
  const prev = d.past.at(-1)
  if (!prev) return d
  return { ...d, theme: prev, past: d.past.slice(0, -1), future: [d.theme, ...d.future], group: undefined }
}

export function redo(d: Draft): Draft {
  const next = d.future[0]
  if (!next) return d
  return { ...d, theme: next, past: [...d.past, d.theme], future: d.future.slice(1), group: undefined }
}

const inGroup = (...groups: string[]) => TOKENS.filter((t) => groups.includes(t.group)).map((t) => t.name)

/** The tokens each seed drives, whose inherited pins let go when it changes. */
const FOLLOWS: Record<string, string[]> = {
  background: inGroup("Surfaces", "Lines", "Overlays", "Switches"),
  contrast: inGroup("Surfaces", "Lines"),
  foreground: inGroup("Text"),
  accent: inGroup("Accent"),
  success: ["color-success", "color-success-soft"],
  warning: ["color-warning", "color-warning-soft"],
  danger: ["color-danger", "color-danger-hover", "color-danger-soft"],
  radius: inGroup("Shape"),
  shadow: inGroup("Shadows"),
  font: ["font-sans"],
  mono: ["font-mono"],
}

/** Sets one seed, letting go of inherited pins that follow it. */
export function setSeed<K extends keyof Seeds>(d: Draft, key: K, value: Seeds[K], group?: string): Draft {
  const release = d.inherited.filter((n) => FOLLOWS[key]?.includes(n))
  const tokens = { ...d.theme.tokens }
  for (const n of release) delete tokens[n]
  const next = { ...d.theme, seeds: { ...d.theme.seeds, [key]: value }, tokens }
  return { ...edit(d, next, group), inherited: d.inherited.filter((n) => !release.includes(n)) }
}

/** Pins a token to a value (a hex color, a length, `var(--other)`). The user's own pin. */
export function pin(d: Draft, name: string, value: string, group?: string): Draft {
  const next = { ...d.theme, tokens: { ...d.theme.tokens, [name]: value } }
  return { ...edit(d, next, group), inherited: d.inherited.filter((n) => n !== name) }
}

/** Removes a pin so the token follows the seeds again. */
export function unpin(d: Draft, name: string): Draft {
  if (!(name in d.theme.tokens)) return d
  const tokens = { ...d.theme.tokens }
  delete tokens[name]
  return { ...edit(d, { ...d.theme, tokens }), inherited: d.inherited.filter((n) => n !== name) }
}

/** Sets the name, author or version. */
export function setInfo(d: Draft, key: "name" | "author" | "version", value: string, group?: string): Draft {
  return edit(d, { ...d.theme, [key]: value.slice(0, 64) }, group)
}

/**
 * Pins whatever it takes for every contrast check to pass, as one edit. Fixing one pair can move
 * another (the text that passes on cards may then fail on popovers), so it goes again until
 * nothing changes. `failing` counts the pairs even pure white or black can't fix.
 */
export function fixContrast(d: Draft): { draft: Draft; fixed: number; failing: number } {
  let theme = d.theme
  const fixed = new Set<string>()
  for (let i = 0; i < PAIRS.length * 3; i++) {
    const tokens = resolve(theme)
    const next = runChecks(tokens)
      .filter((c) => !c.ok)
      .map((c) => [c.fix, fixValue(tokens, c)] as const)
      .find(([, v]) => v !== undefined)
    if (!next?.[1]) break
    fixed.add(next[0])
    theme = { ...theme, tokens: { ...theme.tokens, [next[0]]: next[1] } }
  }
  const failing = runChecks(resolve(theme)).filter((c) => !c.ok).length
  const draft = { ...edit(d, theme), inherited: d.inherited.filter((n) => !fixed.has(n)) }
  return { draft, fixed: fixed.size, failing }
}

/** An id for a new theme, like bonsai-core's: the name in kebab case, then a number if taken. */
export function newId(name: string, taken: (id: string) => boolean): string {
  const stem =
    name
      .toLowerCase()
      .replace(/[^a-z0-9]+/g, "-")
      .replace(/^-+|-+$/g, "")
      .slice(0, 48)
      .replace(/-+$/, "") || "theme"
  for (let i = 1; ; i++) {
    const id = i === 1 ? stem : `${stem}-${i}`
    if (!taken(id)) return id
  }
}

/** A copy of `theme` under a new id and name, ready to start a draft from. */
export function copyOf(theme: Theme, name: string, taken: (id: string) => boolean): Theme {
  return { ...theme, id: newId(name, taken), name: name.slice(0, 64), version: "1.0.0" }
}

// ── Store ──

const root = createRoot(() => {
  const [drafts, setDrafts] = createStore<Record<string, Draft>>({})
  /** The draft whose editor is showing; the editor commands act on it. */
  const [active, setActive] = createSignal<string>()
  createEffect(() => {
    const id = active()
    const d = id ? drafts[id] : undefined
    // Reading the whole draft tracks every change to it, and hands over a plain copy.
    previewTheme(d ? (JSON.parse(JSON.stringify(d.theme)) as Theme) : undefined)
  })
  return { drafts, setDrafts, active, setActive }
})

export const drafts = root.drafts
export const activeDraft = root.active
export const setActiveDraft = root.setActive
/** Applies a pure change to a draft. */
export function update(id: string, f: (d: Draft) => Draft) {
  const d = drafts[id]
  if (!d) return
  const next = f(unwrap(d))
  if (next !== unwrap(d)) root.setDrafts(id, next)
}

export function putDraft(d: Draft) {
  root.setDrafts(d.theme.id, d)
}

export function dropDraft(id: string) {
  root.setDrafts(
    produce((s) => {
      delete s[id]
    }),
  )
}

/** A plain copy of a draft's theme, for saving or sending. */
export const plain = (id: string): Theme | undefined => {
  const d = drafts[id]
  return d ? (JSON.parse(JSON.stringify(unwrap(d.theme))) as Theme) : undefined
}

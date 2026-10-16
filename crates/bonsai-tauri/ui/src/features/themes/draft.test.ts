import { describe, expect, it } from "vitest"
import { DEFAULT_SEEDS, resolve, runChecks, type Theme } from "~/lib/theme"
import {
  changed,
  copyOf,
  edit,
  endGroup,
  fixContrast,
  newId,
  pin,
  redo,
  same,
  setSeed,
  startDraft,
  undo,
  unpin,
  unsaved,
} from "./draft"

const theme = (tokens: Record<string, string> = {}): Theme => ({
  schema: 1,
  id: "fjord",
  name: "Fjord",
  author: "",
  version: "1.0.0",
  base: "dark",
  seeds: DEFAULT_SEEDS.dark,
  tokens,
})

describe("same", () => {
  it("ignores key order", () => {
    expect(same({ a: 1, b: { c: 2, d: 3 } }, { b: { d: 3, c: 2 }, a: 1 })).toBe(true)
    expect(same({ a: 1 }, { a: 1, b: undefined })).toBe(false)
    expect(same([1, 2], [2, 1])).toBe(false)
  })
})

describe("history", () => {
  it("undoes and redoes", () => {
    let d = startDraft(theme(), { onDisk: true })
    d = setSeed(d, "accent", "#ff0000")
    d = setSeed(d, "accent", "#00ff00")
    expect(d.past).toHaveLength(2)
    d = undo(d)
    expect(d.theme.seeds.accent).toBe("#ff0000")
    d = redo(d)
    expect(d.theme.seeds.accent).toBe("#00ff00")
    d = undo(undo(d))
    expect(changed(d)).toBe(false)
    expect(undo(d)).toBe(d)
  })

  it("merges a group into one step until it ends", () => {
    let d = startDraft(theme(), { onDisk: true })
    for (const v of [0.5, 0.6, 0.7]) d = setSeed(d, "contrast", v, "contrast")
    expect(d.past).toHaveLength(1)
    d = setSeed(endGroup(d), "contrast", 0.8, "contrast")
    expect(d.past).toHaveLength(2)
    expect(undo(d).theme.seeds.contrast).toBe(0.7)
  })

  it("drops redo after a new edit", () => {
    let d = setSeed(startDraft(theme(), { onDisk: true }), "radius", 2)
    d = undo(d)
    d = setSeed(d, "radius", 3)
    expect(d.future).toHaveLength(0)
  })

  it("doesn't record an edit that changes nothing", () => {
    const d = startDraft(theme(), { onDisk: true })
    expect(edit(d, theme()).past).toHaveLength(0)
  })
})

describe("pins", () => {
  it("lets go of inherited pins when their seed changes", () => {
    const src = theme({ "color-accent": "#123456", "color-fg": "#eeeeee" })
    let d = startDraft(src, { onDisk: false, inherit: true })
    d = setSeed(d, "accent", "#ff0000")
    expect(d.theme.tokens).toEqual({ "color-fg": "#eeeeee" })
    expect(d.inherited).toEqual(["color-fg"])
  })

  it("keeps the user's own pins", () => {
    let d = startDraft(theme(), { onDisk: true, inherit: true })
    d = pin(d, "color-accent", "#123456")
    d = setSeed(d, "accent", "#ff0000")
    expect(d.theme.tokens["color-accent"]).toBe("#123456")
  })

  it("unpins", () => {
    let d = startDraft(theme({ "color-fg": "#eeeeee" }), { onDisk: true, inherit: true })
    d = unpin(d, "color-fg")
    expect(d.theme.tokens).toEqual({})
    expect(d.inherited).toEqual([])
    expect(unpin(d, "color-fg")).toBe(d)
  })
})

describe("saving state", () => {
  it("a new copy is unsaved but unchanged", () => {
    const d = startDraft(theme(), { onDisk: false })
    expect(unsaved(d)).toBe(true)
    expect(changed(d)).toBe(false)
  })
})

describe("ids", () => {
  it("kebab-cases the name and avoids taken ids", () => {
    expect(newId("Fjord copy", () => false)).toBe("fjord-copy")
    expect(newId("  ¡Hi!  There ", () => false)).toBe("hi-there")
    expect(newId("", () => false)).toBe("theme")
    const taken = new Set(["fjord-copy", "fjord-copy-2"])
    expect(newId("Fjord copy", (id) => taken.has(id))).toBe("fjord-copy-3")
    expect(newId("x".repeat(80), () => false)).toHaveLength(48)
  })

  it("copies under a new id and name", () => {
    const c = copyOf(theme(), "Fjord copy", (id) => id === "fjord-copy")
    expect(c.id).toBe("fjord-copy-2")
    expect(c.name).toBe("Fjord copy")
    expect(c.seeds).toEqual(theme().seeds)
  })
})

describe("fixContrast", () => {
  it("fixes every failing pair in one undo step", () => {
    const d = startDraft(theme({ "color-fg": "#555555", "color-fg-muted": "#444444" }), { onDisk: true })
    const r = fixContrast(d)
    expect(r.failing).toBe(0)
    expect(r.fixed).toBe(2)
    expect(r.draft.past).toHaveLength(1)
    expect(runChecks(resolve(r.draft.theme)).every((c) => c.ok)).toBe(true)
  })

  it("leaves a passing theme alone", () => {
    const d = startDraft(theme(), { onDisk: true })
    expect(fixContrast(d)).toMatchObject({ fixed: 0, failing: 0, draft: { past: [] } })
  })
})

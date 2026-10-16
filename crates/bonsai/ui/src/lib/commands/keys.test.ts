import { describe, expect, test } from "vitest"
import type { Keys } from "~/lib/ipc/bindings"
import { clashes, match, type Recorded, record, spec, toPress } from "./keys"

const chord = (key: string, ctrl = true, shift = false): Keys => ({
  kind: "chord",
  ctrl,
  shift,
  alt: false,
  key,
  label: key,
})
const seq = (first: string, second: string): Keys => ({ kind: "sequence", first, second, label: `${first} ${second}` })
const press = (key: string, mods: Partial<KeyboardEvent> = {}) => {
  const p = toPress({ key, ctrlKey: false, metaKey: false, shiftKey: false, altKey: false, ...mods })
  if (!p) throw new Error(`no press for ${key}`)
  return p
}

describe("toPress", () => {
  test("names punctuation like bonsai-core", () => {
    expect(press("/").key).toBe("Slash")
    expect(press("?", { shiftKey: true }).key).toBe("Questionmark")
    expect(press(".").key).toBe("Period")
    expect(press("k").key).toBe("K")
    expect(press("PageDown").key).toBe("PageDown")
  })

  test("treats meta as ctrl", () => {
    expect(press("k", { metaKey: true }).ctrl).toBe(true)
  })

  test("ignores lone modifiers", () => {
    expect(toPress({ key: "Shift", ctrlKey: false, metaKey: false, shiftKey: true, altKey: false })).toBeUndefined()
  })
})

describe("match", () => {
  test("runs a chord", () => {
    expect(match(press("k", { ctrlKey: true }), [chord("K")], undefined)).toEqual({ kind: "run" })
  })

  test("needs the same modifiers", () => {
    expect(match(press("k"), [chord("K")], undefined)).toEqual({ kind: "none" })
    expect(match(press("t", { ctrlKey: true }), [chord("T", true, true)], undefined)).toEqual({ kind: "none" })
  })

  test("tells Shift+R from R", () => {
    expect(match(press("R", { shiftKey: true }), [chord("R", false, true)], undefined)).toEqual({ kind: "run" })
    expect(match(press("r"), [chord("R", false, true)], undefined)).toEqual({ kind: "none" })
    expect(match(press("R", { shiftKey: true }), [chord("R", false)], undefined)).toEqual({ kind: "none" })
  })

  test("ignores shift on punctuation", () => {
    expect(match(press("?", { shiftKey: true }), [chord("Questionmark", false)], undefined)).toEqual({
      kind: "run",
    })
  })

  test("runs a sequence over two presses", () => {
    const s = [seq("G", "H")]
    expect(match(press("g"), s, undefined)).toEqual({ kind: "pending", first: "G" })
    expect(match(press("h"), s, "G")).toEqual({ kind: "run" })
    expect(match(press("x"), s, "G")).toEqual({ kind: "none" })
  })

  test("does not run a chord in the middle of a sequence", () => {
    expect(match(press("k", { ctrlKey: true }), [chord("K")], "G")).toEqual({ kind: "none" })
  })
})

describe("record", () => {
  const at = (shortcut: Recorded, t = 0) => ({ shortcut, at: t })

  test("a press with Ctrl or Alt is a chord", () => {
    const r = record(undefined, press("k", { ctrlKey: true, shiftKey: true }), 0)
    expect(spec(r)).toBe("Ctrl+Shift+K")
    expect(spec(record(undefined, press("/", { altKey: true }), 0))).toBe("Alt+Slash")
  })

  test("two plain keys in a row make a sequence", () => {
    const g = record(undefined, press("g"), 0)
    expect(spec(g)).toBe("G")
    expect(spec(record(at(g, 0), press("h"), 500))).toBe("G H")
    expect(spec(record(at(g, 0), press("h"), 1500))).toBe("H")
  })

  test("shift only counts for letters", () => {
    expect(spec(record(undefined, press("?", { shiftKey: true }), 0))).toBe("Questionmark")
    expect(spec(record(undefined, press("J", { shiftKey: true }), 0))).toBe("Shift+J")
  })
})

describe("clashes", () => {
  const rec = (key: string, ctrl = true): Recorded => ({ kind: "chord", ctrl, shift: false, alt: false, key })

  test("same keys clash", () => {
    expect(clashes(rec("K"), chord("K"))).toBe(true)
    expect(clashes(rec("K"), chord("K", true, true))).toBe(false)
    expect(clashes({ kind: "sequence", first: "G", second: "H" }, seq("G", "H"))).toBe(true)
  })

  test("a plain key clashes with sequences it starts", () => {
    expect(clashes(rec("G", false), seq("G", "H"))).toBe(true)
    expect(clashes({ kind: "sequence", first: "G", second: "X" }, chord("G", false))).toBe(true)
    expect(clashes(rec("G"), seq("G", "H"))).toBe(false)
  })
})

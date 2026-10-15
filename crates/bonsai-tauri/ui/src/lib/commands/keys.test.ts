import { describe, expect, test } from "vitest"
import type { Keys } from "~/lib/ipc/bindings"
import { match, toPress } from "./keys"

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
  test("names punctuation like rbx-core", () => {
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

import { describe, expect, it } from "vitest"
import type { OverlapView } from "~/lib/ipc/bindings"
import { overlapLine, overlapNote, personName } from "./overlap-text"

const view = (v: Partial<OverlapView>): OverlapView => ({
  loading: false,
  error: null,
  checked: 0,
  complete: false,
  accounts: [],
  ...v,
})

describe("overlapNote", () => {
  it("says how much of their list was checked", () => {
    expect(overlapNote(view({ checked: 50 }))).toBe("From the first 50 of their friends.")
    expect(overlapNote(view({ checked: 12, complete: true }))).toBe("From all 12 of their friends.")
  })
  it("shows errors and loading", () => {
    expect(overlapNote(view({ error: "Nope" }))).toBe("Nope")
    expect(overlapNote(view({ loading: true }))).toBe("Loading their friends…")
    expect(overlapNote(view({}))).toBe("")
  })
})

describe("overlapLine", () => {
  it("joins friendship and mutual count", () => {
    expect(overlapLine(true, 0)).toBe("Friends")
    expect(overlapLine(true, 3)).toBe("Friends, 3 mutual")
    expect(overlapLine(false, 2)).toBe("2 mutual")
  })
})

describe("personName", () => {
  it("falls back to username and id", () => {
    expect(personName({ id: 1, name: "a", displayName: "A" })).toBe("A")
    expect(personName({ id: 1, name: "a", displayName: "" })).toBe("a")
    expect(personName({ id: 7, name: "", displayName: "" })).toBe("User 7")
  })
})

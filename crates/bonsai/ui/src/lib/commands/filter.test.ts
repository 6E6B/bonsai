import { describe, expect, it } from "vitest"
import { filterEntries } from "./filter"

const entries = [
  { title: "Go to Home", words: "home" },
  { title: "Settings", words: "settings preferences options" },
  { title: "Switch account", words: "switch account acc" },
  { title: "Go to Settings shortcut", words: "" },
]

describe("filterEntries", () => {
  it("returns everything for an empty query", () => {
    expect(filterEntries(entries, "  ")).toEqual(entries)
  })

  it("matches keywords case-insensitively", () => {
    expect(filterEntries(entries, "PREF").map((e) => e.title)).toEqual(["Settings"])
  })

  it("requires every term to match", () => {
    expect(filterEntries(entries, "switch home")).toEqual([])
  })

  it("ranks title prefix matches first", () => {
    expect(filterEntries(entries, "set").map((e) => e.title)).toEqual(["Settings", "Go to Settings shortcut"])
  })
})

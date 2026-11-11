import { describe, expect, it } from "vitest"
import { highlightParts, highlights } from "./highlight"

const hits = (title: string, q: string) =>
  highlightParts(title, q)
    .filter((p) => p.hit)
    .map((p) => p.text)

describe("highlights", () => {
  it("marks words starting with what was typed", () => {
    expect(hits("Tower of Hell", "tow he")).toEqual(["Tow", "He"])
  })

  it("marks a longer term inside a word", () => {
    expect(hits("StickMasterLuke", "luke")).toEqual(["Luke"])
  })

  it("doesn't mark three letters inside a word", () => {
    expect(hits("Start", "art")).toEqual([])
  })

  it("marks initials", () => {
    expect(hits("Switch account", "sa")).toEqual(["S", "a"])
    expect(hits("Go to Friends", "gf")).toEqual(["G", "F"])
  })

  it("keeps positions with accents", () => {
    expect(hits("Café Olé", "ole")).toEqual(["Olé"])
  })

  it("skips stop words unless that's all there is", () => {
    expect(hits("Go to Home", "to home")).toEqual(["Home"])
    expect(hits("Tower of Hell", "to")).toEqual(["To"])
  })

  it("is empty for an empty query", () => {
    expect(highlights("Home", " ")).toEqual([])
  })
})

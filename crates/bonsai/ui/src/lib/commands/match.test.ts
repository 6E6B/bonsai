import { describe, expect, it } from "vitest"
import { distance, matchTerms, prepare, rankScored, wordScore } from "./match"

describe("distance", () => {
  it("counts edits and swaps", () => {
    expect(distance("frinds", "friends", 2)).toBe(1)
    expect(distance("invnetory", "inventory", 2)).toBe(1)
    expect(distance("abc", "xyz", 1)).toBe(2)
  })
})

describe("wordScore", () => {
  it("orders exact, prefix, typo and substring", () => {
    const exact = wordScore("settings", "settings")
    const prefix = wordScore("sett", "settings")
    const typo = wordScore("setings", "settings")
    const inside = wordScore("ting", "settings")
    expect(exact).toBeGreaterThan(typo)
    expect(typo).toBeGreaterThan(prefix - 20)
    expect(prefix).toBeGreaterThan(inside)
    expect(inside).toBeGreaterThan(0)
    expect(wordScore("zz", "settings")).toBe(0)
  })

  it("allows no typos in short terms", () => {
    expect(wordScore("tb", "tab")).toBe(0)
  })

  it("doesn't read a short word as another word", () => {
    expect(wordScore("play", "plan")).toBe(0)
    expect(wordScore("art", "start")).toBe(0)
    expect(wordScore("zoon", "zoom")).toBeGreaterThan(0)
    expect(wordScore("sgin", "sign")).toBeGreaterThan(0)
    expect(wordScore("shpo", "shop")).toBeGreaterThan(0)
  })
})

describe("matchTerms", () => {
  it("reads initials", () => {
    expect(matchTerms(prepare("Switch account", ""), ["sa"])?.strong).toBe(true)
  })

  it("moves parenthetical hints to keywords", () => {
    const p = prepare("Filter friends (all, online, not_followed)", "")
    expect(p.title).toEqual(["filter", "friends"])
    expect(p.words).toContain("online")
  })

  it("matches synonyms", () => {
    expect(matchTerms(prepare("Sign out", ""), ["logout"])).not.toBeNull()
    expect(matchTerms(prepare("Remove friend", ""), ["delete", "friend"])).not.toBeNull()
  })

  it("forgives one missing term in three", () => {
    const p = prepare("Clear cache", "")
    expect(matchTerms(p, ["clear", "the", "cache", "now"])).not.toBeNull()
    expect(matchTerms(p, ["clear", "now"])).toBeNull()
  })
})

describe("rankScored", () => {
  it("puts the closer title first", () => {
    const rows = [
      { title: "Check for updates automatically", words: "" },
      { title: "Zoom in", words: "" },
      { title: "Zoom out", words: "" },
    ]
    expect(rankScored(rows, "zoom in")[0].entry.title).toBe("Zoom in")
    expect(rankScored(rows, "zoon out")[0]?.entry.title).toBe("Zoom out")
  })
})

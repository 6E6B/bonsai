import { describe, expect, it } from "vitest"
import { EMOJI_GROUPS, searchEmoji } from "./emoji"

describe("searchEmoji", () => {
  it("returns every group for a blank query", () => {
    expect(searchEmoji("  ")).toEqual(EMOJI_GROUPS)
  })

  it("finds emoji by any word they are known by", () => {
    const [r] = searchEmoji("fire")
    expect(r?.label).toBe("Results")
    expect(r?.emoji.map(([e]) => e)).toContain("🔥")
  })

  it("matches parts of words and needs every word", () => {
    expect(searchEmoji("thumb")[0]?.emoji.map(([e]) => e)).toEqual(["👍", "👎"])
    expect(searchEmoji("thumb up")[0]?.emoji.map(([e]) => e)).toEqual(["👍"])
  })

  it("returns nothing for a miss", () => {
    expect(searchEmoji("qqqq")).toEqual([])
  })

  it("has no emoji listed twice", () => {
    const all = EMOJI_GROUPS.flatMap((g) => g.emoji.map(([e]) => e))
    expect(new Set(all).size).toBe(all.length)
  })
})

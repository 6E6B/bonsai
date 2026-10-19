import { describe, expect, it } from "vitest"
import {
  articlePath,
  knownArticle,
  listDepth,
  parseLinks,
  parseTarget,
  plain,
  rememberArticle,
  splitArg,
  titleOf,
} from "./wiki"
import { sampleAbout, samplePage, sampleSearch } from "./wiki-sample"

describe("parseLinks", () => {
  it("splits links and labels out of text", () => {
    expect(parseLinks("by [[Grill Studio]] and [[Pixel Fox|pixelfox]].")).toEqual([
      { text: "by " },
      { text: "Grill Studio", link: "Grill Studio" },
      { text: " and " },
      { text: "pixelfox", link: "Pixel Fox" },
      { text: "." },
    ])
  })

  it("leaves text without links alone", () => {
    expect(parseLinks("plain")).toEqual([{ text: "plain" }])
    expect(parseLinks("")).toEqual([])
  })

  it("plain drops the brackets", () => {
    expect(plain("a [[B|c]] [[D]]")).toBe("a c D")
  })
})

describe("titles and paths", () => {
  it("round-trips titles through paths", () => {
    expect(articlePath("Catalog:Dominus Infernus")).toBe("/wiki/Catalog:Dominus_Infernus")
    expect(articlePath("Catalog:Dominus Infernus", "Trivia")).toBe("/wiki/Catalog:Dominus_Infernus#Trivia")
    expect(articlePath("Café & more?")).toBe("/wiki/Caf%C3%A9_%26_more%3F")
    expect(titleOf("Caf%C3%A9_%26_more%3F")).toBe("Café & more?")
    expect(titleOf("100%_bad")).toBe("100% bad")
  })

  it("splits a wiki.open argument", () => {
    expect(splitArg("Catalog:Dominus_Infernus#Trivia")).toEqual({
      title: "Catalog:Dominus Infernus",
      section: "Trivia",
    })
    expect(splitArg(" Robux ")).toEqual({ title: "Robux", section: undefined })
    expect(splitArg(null)).toEqual({ title: "", section: undefined })
  })

  it("reads nested list depth", () => {
    expect(listDepth("– – deep")).toEqual({ depth: 2, text: "deep" })
    expect(listDepth("top – dash")).toEqual({ depth: 0, text: "top – dash" })
  })
})

describe("targets", () => {
  it("parses command contexts", () => {
    expect(parseTarget("game:9001")).toEqual({ kind: "game", placeId: 9001 })
    expect(parseTarget("user:7")).toEqual({ kind: "user", id: 7 })
    expect(parseTarget("item:42")).toEqual({ kind: "item", id: 42 })
    expect(parseTarget("bundle:42")).toBeNull()
    expect(parseTarget(null)).toBeNull()
  })

  it("remembers which article a page had", () => {
    rememberArticle({ kind: "game", placeId: 5 }, "Some Game")
    expect(knownArticle({ kind: "game", placeId: 5 })).toBe("Some Game")
    expect(knownArticle({ kind: "user", id: 5 })).toBeUndefined()
  })
})

describe("browser sample", () => {
  it("finds articles by title, name and target", () => {
    expect(samplePage("Catalog:Golden_Spatula").article?.name).toBe("Golden Spatula")
    expect(samplePage("golden spatula").article?.title).toBe("Catalog:Golden Spatula")
    expect(samplePage("Nowhere").article).toBeNull()
    expect(sampleAbout("game", 9001).article?.title).toBe("Burger Shift")
    expect(sampleAbout("user", 101).article?.title).toBe("Pixel Fox")
    expect(sampleAbout("item", 999_999).article).toBeNull()
  })

  it("searches names and leads, with plain snippets", () => {
    const hits = sampleSearch("grill").hits
    expect(hits.map((h) => h.name)).toContain("Grill Studio")
    expect(hits.every((h) => !h.snippet.includes("[["))).toBe(true)
    expect(sampleSearch("  ").hits).toEqual([])
  })
})

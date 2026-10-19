import { describe, expect, it } from "vitest"
import {
  articlePath,
  knownArticle,
  linkTarget,
  listDepth,
  mainPage,
  parseLinks,
  parseTarget,
  plain,
  rememberArticle,
  searchPath,
  splitArg,
  splitSearch,
  splitWiki,
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

  it("keeps a single ] in a label", () => {
    expect(parseLinks("buy [[https://x.com/vip|Player [VIP]]] now")).toEqual([
      { text: "buy " },
      { text: "Player [VIP]", link: "https://x.com/vip" },
      { text: " now" },
    ])
    expect(parseLinks("[[A|x [1]]] and [[B]]")).toEqual([
      { text: "x [1]", link: "A" },
      { text: " and " },
      { text: "B", link: "B" },
    ])
  })

  it("tells articles, Roblox pages and other sites apart", () => {
    expect(linkTarget("Catalog:Dominus Infernus")).toEqual({ kind: "article", title: "Catalog:Dominus Infernus" })
    expect(linkTarget("w:c:jailbreak:Torero")).toEqual({ kind: "article", title: "w:c:jailbreak:Torero" })
    expect(linkTarget("@item:31101391")).toEqual({ kind: "roblox", command: "market.open", arg: "31101391" })
    expect(linkTarget("@bundle:7")).toEqual({ kind: "roblox", command: "market.open", arg: "bundle:7" })
    expect(linkTarget("@game:606849621")).toEqual({ kind: "roblox", command: "game.open", arg: "606849621" })
    expect(linkTarget("https://x.com/badimo")).toEqual({ kind: "web", url: "https://x.com/badimo" })
    expect(linkTarget("@nope:1")).toEqual({ kind: "article", title: "@nope:1" })
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

describe("other wikis", () => {
  it("splits titles on other wikis", () => {
    expect(splitWiki("w:c:jailbreak:Torero")).toEqual({ wiki: "jailbreak", title: "Torero" })
    expect(splitWiki("w:c:jailbreak:")).toEqual({ wiki: "jailbreak", title: "" })
    expect(splitWiki("Catalog:Dominus Infernus")).toEqual({ wiki: "roblox", title: "Catalog:Dominus Infernus" })
    expect(mainPage("jailbreak")).toBe("w:c:jailbreak:")
  })

  it("searches a wiki", () => {
    expect(searchPath("")).toBe("/wiki")
    expect(searchPath("dominus")).toBe("/wiki?q=dominus")
    expect(searchPath("", "jailbreak")).toBe("/wiki?w=jailbreak")
    expect(searchPath("torero car", "jailbreak")).toBe("/wiki?q=torero+car&w=jailbreak")
    expect(splitSearch("w:c:jailbreak: torero ")).toEqual({ wiki: "jailbreak", query: "torero" })
    expect(splitSearch(" dominus ")).toEqual({ wiki: "roblox", query: "dominus" })
    expect(splitSearch(null)).toEqual({ wiki: "roblox", query: "" })
  })

  it("keeps other wikis' pages in the Wiki screen", () => {
    expect(articlePath("w:c:jailbreak:Torero")).toBe("/wiki/w:c:jailbreak:Torero")
    expect(titleOf("w:c:jailbreak:Jailbreak_Wiki:Home")).toBe("w:c:jailbreak:Jailbreak Wiki:Home")
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

import { describe, expect, it } from "vitest"
import {
  articleFor,
  articles,
  backlinks,
  gameWiki,
  hasWiki,
  parseLinks,
  parseTarget,
  plain,
  resolveLink,
  searchWiki,
  siteUrl,
  type WikiArticle,
} from "./wiki"

const art = (title: string, extra: Partial<WikiArticle> = {}): WikiArticle => ({
  source: "roblox",
  slug: title.replace(/ /g, "_"),
  title,
  kind: "page",
  category: "Misc",
  extract: "",
  target: null,
  revised: 0,
  editors: 1,
  infobox: [],
  sections: [],
  categories: [],
  ...extra,
})

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

describe("searchWiki", () => {
  const all = [
    art("Grill Studio"),
    art("Grill"),
    art("The Big Grill"),
    art("Patty", { extract: "Cooked on the grill." }),
    art("Bun", { kind: "item" }),
  ]

  it("ranks exact, prefix, word, then lead matches", () => {
    expect(searchWiki(all, "grill").map((a) => a.title)).toEqual(["Grill", "Grill Studio", "The Big Grill", "Patty"])
  })

  it("filters by kind and keeps order for an empty query", () => {
    expect(searchWiki(all, "  ", "item").map((a) => a.title)).toEqual(["Bun"])
    expect(searchWiki(all, "").length).toBe(all.length)
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

  it("finds the Roblox Wiki article about a page, not experience wiki pages", () => {
    expect(articleFor({ kind: "game", placeId: 9001 })?.source).toBe("roblox")
    expect(articleFor({ kind: "user", id: 101 })?.title).toBe("Pixel Fox")
    expect(articleFor({ kind: "item", id: 999_999 })).toBeUndefined()
  })

  it("knows which experiences have a wiki of their own", () => {
    expect(gameWiki(9001)?.pages.every((p) => p.source === "burgershift")).toBe(true)
    expect(gameWiki(9000)).toBeUndefined()
    expect(hasWiki({ kind: "game", placeId: 9003 })).toBe(true)
    expect(hasWiki({ kind: "game", placeId: 9000 })).toBe(false)
  })
})

describe("links between articles", () => {
  it("resolves in the same wiki first, then the Roblox Wiki", () => {
    expect(resolveLink("burgershift", "Night Shift")?.source).toBe("burgershift")
    expect(resolveLink("burgershift", "Golden Spatula")?.source).toBe("roblox")
    expect(resolveLink("roblox", "Nowhere")).toBeUndefined()
  })

  it("lists the articles that link here", () => {
    const shift = articles.find((a) => a.slug === "Burger_Shift" && a.source === "roblox")
    if (!shift) throw new Error("sample article missing")
    const from = backlinks(shift).map((a) => a.title)
    expect(from).toContain("Golden Spatula")
    expect(from).not.toContain("Burger Shift")
  })

  it("builds the article's address on its wiki", () => {
    expect(siteUrl({ source: "roblox", slug: "Golden_Spatula" })).toBe("https://roblox.fandom.com/wiki/Golden_Spatula")
  })
})

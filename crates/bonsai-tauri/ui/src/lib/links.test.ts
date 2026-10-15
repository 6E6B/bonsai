import { describe, expect, it } from "vitest"
import {
  experienceLinks,
  externalHosts,
  isOnlyExperienceLinks,
  isUsername,
  mentions,
  openIn,
  splitLinks,
  splitRich,
} from "./links"

describe("splitLinks", () => {
  it("picks out Roblox links and leaves the rest as text", () => {
    expect(splitLinks("Join https://www.roblox.com/communities/7/x. Or https://example.com")).toEqual([
      { text: "Join " },
      {
        text: "https://www.roblox.com/communities/7/x",
        url: "https://www.roblox.com/communities/7/x",
        open: { command: "group.open", arg: "7" },
      },
      { text: ". Or https://example.com" },
    ])
  })

  it("marks experience links with their place id", () => {
    expect(splitLinks("(https://roblox.com/games/1818/Crossroads)")).toEqual([
      { text: "(" },
      {
        text: "https://roblox.com/games/1818/Crossroads",
        url: "https://roblox.com/games/1818/Crossroads",
        placeId: 1818,
        open: { command: "game.open", arg: "1818" },
      },
      { text: ")" },
    ])
  })

  it("does not match lookalike hosts", () => {
    expect(splitLinks("https://roblox.com.evil.test/x")).toEqual([{ text: "https://roblox.com.evil.test/x" }])
  })
})

describe("experienceLinks", () => {
  it("lists each experience once, in order", () => {
    expect(
      experienceLinks(
        "a https://www.roblox.com/games/2/b and https://roblox.com/games/1/a, again https://www.roblox.com/games/2/b",
      ),
    ).toEqual([2, 1])
  })

  it("ignores other Roblox pages and other sites", () => {
    expect(experienceLinks("https://www.roblox.com/users/1/profile https://example.com/games/5")).toEqual([])
  })
})

describe("isOnlyExperienceLinks", () => {
  it("is true for links and whitespace only", () => {
    expect(isOnlyExperienceLinks("https://www.roblox.com/games/1818")).toBe(true)
    expect(isOnlyExperienceLinks("https://www.roblox.com/games/1\nhttps://www.roblox.com/games/2 ")).toBe(true)
  })

  it("is false with words, other links or none", () => {
    expect(isOnlyExperienceLinks("join https://www.roblox.com/games/1818")).toBe(false)
    expect(isOnlyExperienceLinks("https://www.roblox.com/games/1 https://www.roblox.com/users/1/profile")).toBe(false)
    expect(isOnlyExperienceLinks("hello")).toBe(false)
  })
})

describe("externalHosts", () => {
  it("names the other sites a text links to", () => {
    expect(externalHosts("see https://Example.com/x?y=1. and http://evil.test, https://example.com/z")).toEqual([
      "example.com",
      "evil.test",
    ])
  })

  it("treats lookalike hosts as other sites and Roblox as ours", () => {
    expect(externalHosts("https://www.roblox.com/games/1 https://roblox.com.evil.test/x")).toEqual([
      "roblox.com.evil.test",
    ])
    expect(externalHosts("https://create.roblox.com/docs")).toEqual([])
  })

  it("skips text without links and bare domains", () => {
    expect(externalHosts("no links, only example.com")).toEqual([])
  })
})

describe("splitRich", () => {
  it("picks out hashtags beside links", () => {
    expect(splitRich("#Builder at https://roblox.com/games/1/x, likes #obby2!")).toEqual([
      { text: "#Builder", tag: "Builder" },
      { text: " at " },
      {
        text: "https://roblox.com/games/1/x",
        url: "https://roblox.com/games/1/x",
        placeId: 1,
        open: { command: "game.open", arg: "1" },
      },
      { text: ", likes " },
      { text: "#obby2", tag: "obby2" },
      { text: "!" },
    ])
  })

  it("leaves filtered words, numbers and mid-word hashes alone", () => {
    for (const s of ["I am ####", "rank #1", "c#sharp", "it&#39;s"]) expect(splitRich(s)).toEqual([{ text: s }])
  })
})

describe("openIn", () => {
  it("opens profiles, communities, items and bundles in the app", () => {
    expect(openIn("https://www.roblox.com/users/156/profile")).toEqual({ command: "user.open", arg: "156" })
    expect(openIn("https://www.roblox.com/groups/7/x#!/about")).toEqual({ command: "group.open", arg: "7" })
    expect(openIn("https://www.roblox.com/catalog/1028606/Red-Hat")).toEqual({
      command: "market.open",
      arg: "item:1028606",
    })
    expect(openIn("https://www.roblox.com/bundles/192/x")).toEqual({ command: "market.open", arg: "bundle:192" })
    expect(openIn("https://www.roblox.com/es/games/1818/x")).toEqual({ command: "game.open", arg: "1818" })
  })

  it("leaves other pages and subdomains to the website", () => {
    expect(openIn("https://www.roblox.com/home")).toBeUndefined()
    expect(openIn("https://create.roblox.com/users/1")).toBeUndefined()
    expect(openIn("https://www.roblox.com/search?q=/games/1")).toBeUndefined()
  })
})

describe("mentions", () => {
  it("picks out @usernames beside links and hashtags", () => {
    expect(splitRich("by @Builderman's team, #obby (@Roblox_Dev)")).toEqual([
      { text: "by " },
      { text: "@Builderman", mention: "builderman" },
      { text: "'s team, " },
      { text: "#obby", tag: "obby" },
      { text: " (" },
      { text: "@Roblox_Dev", mention: "roblox_dev" },
      { text: ")" },
    ])
  })

  it("skips emails, handles that can't be usernames, and hashtags when asked", () => {
    for (const s of ["mail me@example.com", "@ab", "@_lead", "@a_b_c", "@@double", "@abcdefghijklmnopqrstu"]) {
      expect(splitRich(s)).toEqual([{ text: s }])
    }
    expect(splitRich("#obby", { tags: false })).toEqual([{ text: "#obby" }])
  })

  it("lists each name once, lowercased", () => {
    expect(mentions("@Alice and @bob, then @ALICE https://www.roblox.com/users/1/@carol")).toEqual(["alice", "bob"])
  })

  it("checks Roblox's username rules", () => {
    expect(isUsername("abc")).toBe(true)
    expect(isUsername("a_bc")).toBe(true)
    expect(isUsername("ab")).toBe(false)
    expect(isUsername("a__b")).toBe(false)
    expect(isUsername("abc_")).toBe(false)
  })
})

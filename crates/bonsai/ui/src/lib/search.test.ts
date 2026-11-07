import { describe, expect, it } from "vitest"
import { fold, matches, rank, score } from "./search"

const names = (list: { name: string }[]) => list.map((x) => x.name)

describe("rank", () => {
  const byName = (items: { name: string; by?: string }[], q: string) => names(rank(items, q, (i) => [i.name, i.by]))

  it("keeps everything in order for a blank query", () => {
    const items = [{ name: "b" }, { name: "a" }]
    expect(byName(items, "  ")).toEqual(["b", "a"])
  })

  it("puts exact, then prefix, then word start, then substring matches first", () => {
    const items = [{ name: "Kabob" }, { name: "Big Bob" }, { name: "Bobby" }, { name: "bob" }, { name: "Nope" }]
    expect(byName(items, "bob")).toEqual(["bob", "Bobby", "Big Bob", "Kabob"])
  })

  it("treats camelCase humps, digits and underscores as word starts", () => {
    const items = [{ name: "Ablukes" }, { name: "StickMasterLuke" }, { name: "x_luke" }, { name: "99luke" }]
    expect(byName(items, "luke")).toEqual(["StickMasterLuke", "x_luke", "99luke", "Ablukes"])
  })

  it("requires every word, in any field", () => {
    const items = [
      { name: "Tower of Hell", by: "YXCeptional" },
      { name: "Natural Disaster Survival", by: "Stickmasterluke" },
    ]
    expect(byName(items, "disaster LUKE")).toEqual(["Natural Disaster Survival"])
    expect(byName(items, "tower luke")).toEqual([])
  })

  it("ranks earlier fields above later ones", () => {
    const items = [
      { name: "Obby", by: "Tower Games" },
      { name: "Tower Defense", by: "someone" },
    ]
    expect(byName(items, "tower")).toEqual(["Tower Defense", "Obby"])
  })

  it("rewards the query read as a phrase", () => {
    const items = [{ name: "Hell Tower of Doom" }, { name: "Tower of Hell" }]
    expect(byName(items, "tower of")).toEqual(["Tower of Hell", "Hell Tower of Doom"])
  })

  it("keeps the original order for equal matches", () => {
    const items = [{ name: "Bobcat" }, { name: "Bobby" }, { name: "Boba" }]
    expect(byName(items, "bob")).toEqual(["Bobcat", "Bobby", "Boba"])
  })

  it("puts an exact id match first and ignores partial ids", () => {
    const items = [
      { id: 7, name: "Item 42" },
      { id: 42, name: "Hat" },
      { id: 420, name: "Cap" },
    ]
    expect(
      names(
        rank(
          items,
          "42",
          (i) => [i.name],
          (i) => i.id,
        ),
      ),
    ).toEqual(["Hat", "Item 42"])
  })

  it("ignores accents and skips empty fields", () => {
    const items = [{ name: "Café Simulator", by: null as unknown as string }]
    expect(byName(items, "cafe")).toEqual(["Café Simulator"])
  })
})

describe("score and matches", () => {
  it("scores no match below zero and a blank query as zero", () => {
    expect(score(["abc"], "z")).toBe(-1)
    expect(score(["abc"], "")).toBe(0)
    expect(matches(["abc"], "B")).toBe(true)
  })

  it("folds case and accents", () => {
    expect(fold("ÉLAN")).toBe("elan")
  })
})

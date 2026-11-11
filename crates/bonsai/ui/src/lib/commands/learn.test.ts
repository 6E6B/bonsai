import { describe, expect, it } from "vitest"
import { booster, completion, emptyUsage, forget, recentQueries, record, rememberArg } from "./learn"

const DAY = 86_400_000

describe("learn", () => {
  it("boosts what was picked for the same query most", () => {
    const now = 10 * DAY
    let u = emptyUsage()
    u = record(u, "a", "zoom", now)
    u = record(u, "b", "zoom in", now)
    const boost = booster(u, "zoom", now)
    expect(boost("a")).toBeGreaterThan(boost("b"))
    expect(boost("b")).toBeGreaterThan(0)
    expect(boost("c")).toBe(0)
  })

  it("fades old picks", () => {
    const u = record(emptyUsage(), "a", "x", 0)
    expect(booster(u, "x", 60 * DAY)("a")).toBeLessThan(booster(u, "x", 1)("a"))
  })

  it("doesn't change the usage it is given", () => {
    const u = emptyUsage()
    record(u, "a", "x")
    expect(u).toEqual(emptyUsage())
  })

  it("keeps a bounded history", () => {
    let u = emptyUsage()
    for (let i = 0; i < 500; i++) u = record(u, `k${i}`, `q${i}`, i)
    expect(u.queries.length).toBe(400)
    expect(u.queries.at(-1)?.q).toBe("q499")
  })
})

describe("recent arguments", () => {
  it("keeps the newest first, once each", () => {
    let u = rememberArg(emptyUsage(), "user.message", { arg: "1", title: "A" }, 1)
    u = rememberArg(u, "user.message", { arg: "2", title: "B" }, 2)
    u = rememberArg(u, "user.message", { arg: "1", title: "A" }, 3)
    expect(u.args?.["user.message"].map((a) => a.arg)).toEqual(["1", "2"])
  })

  it("forgets by row key and by ref", () => {
    let u = rememberArg(emptyUsage(), "user.message", { arg: "1", title: "A", ref: "user:1" }, 1)
    u = rememberArg(u, "user.message", { arg: "2", title: "B" }, 2)
    u = record(u, "user:1", "a", 1)
    u = forget(u, ["user:1", "user.message:recent:2"])
    expect(u.args?.["user.message"]).toEqual([])
    expect(u.picks["user:1"]).toBeUndefined()
    expect(u.queries).toEqual([])
  })
})

describe("recentQueries", () => {
  it("lists what was typed, newest first, once each", () => {
    let u = record(emptyUsage(), "a", "tower", 1)
    u = record(u, "b", "luke", 2)
    u = record(u, "c", "tower", 3)
    expect(recentQueries(u, 5)).toEqual(["tower", "luke"])
    expect(recentQueries(u, 1)).toEqual(["tower"])
  })
})

describe("completion", () => {
  it("finishes a query typed before", () => {
    const u = record(emptyUsage(), "a", "tower of hell", 1)
    expect(completion(u, "tow", 1)).toBe("er of hell")
    expect(completion(u, "Tower  of", 1)).toBe(" hell")
  })

  it("has nothing for a new or whole query", () => {
    const u = record(emptyUsage(), "a", "tower", 1)
    expect(completion(u, "tower", 1)).toBeNull()
    expect(completion(u, "luke", 1)).toBeNull()
    expect(completion(u, "", 1)).toBeNull()
  })

  it("prefers what was typed more often", () => {
    let u = record(emptyUsage(), "a", "trade list", 1)
    u = record(u, "b", "trades", 1)
    u = record(u, "b", "trades", 1)
    expect(completion(u, "tra", 1)).toBe("des")
  })
})

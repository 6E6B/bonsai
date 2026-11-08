import { describe, expect, it } from "vitest"
import { booster, emptyUsage, record } from "./learn"

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

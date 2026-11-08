import { afterEach, beforeEach, describe, expect, it, vi } from "vitest"
import { debouncer, paramOf, withParams } from "./url-params"

describe("paramOf", () => {
  it("takes one value and treats missing as empty", () => {
    expect(paramOf("a")).toBe("a")
    expect(paramOf(["a", "b"])).toBe("a")
    expect(paramOf(undefined)).toBe("")
    expect(paramOf([])).toBe("")
  })
})

describe("withParams", () => {
  const here = { pathname: "/marketplace", search: "?cat=hair&min=5" }

  it("keeps other params when already on the path", () => {
    expect(withParams(here, "/marketplace", { q: "hat" })).toBe("/marketplace?cat=hair&min=5&q=hat")
  })

  it("replaces and removes params", () => {
    expect(withParams(here, "/marketplace", { cat: "body", min: null })).toBe("/marketplace?cat=body")
    expect(withParams(here, "/marketplace", { cat: "", min: null })).toBe("/marketplace")
  })

  it("starts clean from another path", () => {
    expect(withParams({ pathname: "/home", search: "?cat=x" }, "/marketplace", { q: "hat" })).toBe("/marketplace?q=hat")
  })

  it("encodes values", () => {
    expect(withParams(here, "/marketplace", { cat: null, min: null, q: "a&b" })).toBe("/marketplace?q=a%26b")
  })
})

describe("debouncer", () => {
  beforeEach(() => {
    vi.useFakeTimers()
  })
  afterEach(() => {
    vi.useRealTimers()
  })

  it("writes the last value once typing stops", () => {
    const write = vi.fn()
    const d = debouncer(write, 100)
    d.schedule("a")
    vi.advanceTimersByTime(50)
    d.schedule("ab")
    expect(d.pending()).toBe(true)
    vi.advanceTimersByTime(99)
    expect(write).not.toHaveBeenCalled()
    vi.advanceTimersByTime(1)
    expect(write).toHaveBeenCalledExactlyOnceWith("ab")
    expect(d.pending()).toBe(false)
  })

  it("drops the write when cancelled", () => {
    const write = vi.fn()
    const d = debouncer(write, 100)
    d.schedule("a")
    d.cancel()
    vi.advanceTimersByTime(200)
    expect(write).not.toHaveBeenCalled()
    expect(d.pending()).toBe(false)
  })
})

import { describe, expect, it } from "vitest"
import { serverName, serverNameWords } from "./server-name"

describe("serverName", () => {
  it("is stable across versions", () => {
    // Pinned: if these change, every Bonsai user sees different names for the same servers.
    expect(serverName("8b1f5e2c-4a6d-4f0e-9c3b-2d7a1e5f9b80")).toMatchInlineSnapshot(`"Sturdy Panther"`)
    expect(serverName("00000000-0000-0000-0000-000000000000")).toMatchInlineSnapshot(`"Crisp Comet"`)
  })

  it("ignores case and surrounding spaces", () => {
    const id = "8b1f5e2c-4a6d-4f0e-9c3b-2d7a1e5f9b80"
    expect(serverName(` ${id.toUpperCase()} `)).toBe(serverName(id))
  })

  it("tells nearby ids apart", () => {
    expect(serverName("8b1f5e2c-4a6d-4f0e-9c3b-2d7a1e5f9b80")).not.toBe(
      serverName("8b1f5e2c-4a6d-4f0e-9c3b-2d7a1e5f9b81"),
    )
  })

  it("has no repeated words", () => {
    for (const words of [serverNameWords.adjectives, serverNameWords.nouns]) {
      expect(new Set(words).size).toBe(words.length)
    }
  })
})

import { describe, expect, it } from "vitest"
import { groupOf, groupOrder } from "./groups"

describe("groupOf", () => {
  it("maps prefixes to sections", () => {
    expect(groupOf("nav.home")).toBe("Go to")
    expect(groupOf("friends.decline_all")).toBe("People")
    expect(groupOf("weird.thing")).toBe("Other")
  })

  it("lists each section once", () => {
    expect(new Set(groupOrder).size).toBe(groupOrder.length)
  })
})

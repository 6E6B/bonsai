import { describe, expect, it } from "vitest"
import { assetPath } from "./host"

describe("assetPath", () => {
  it("puts the account and id in the path", () => {
    expect(assetPath(7, 123)).toEqual({ path: "7.123", query: "" })
  })

  it("sends representations as base64url, padding kept", () => {
    expect(assetPath(0, 5, "avatar_meshpart_head", "W3s+/0==")).toEqual({
      path: "0.5",
      query: "?f=avatar_meshpart_head&r=W3s-_0==",
    })
  })
})

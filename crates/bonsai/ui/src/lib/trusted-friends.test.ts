import { describe, expect, it } from "vitest"
import { trustedActions } from "./trusted-friends"

describe("trusted friend actions", () => {
  it("only sends to an existing friend with a known empty trusted state", () => {
    expect(trustedActions("none", true)).toEqual(["user.add_trusted_friend"])
    expect(trustedActions("none", false)).toEqual([])
    expect(trustedActions("unknown", true)).toEqual([])
  })
  it("answers received requests even while ordinary friendship is still loading", () => {
    expect(trustedActions("received", false)).toEqual(["user.accept_trusted_friend", "user.decline_trusted_friend"])
  })
  it("cancels outgoing requests and removes trust without unfriending", () => {
    expect(trustedActions("sent", true)).toEqual(["user.cancel_trusted_friend"])
    expect(trustedActions("trusted", true)).toEqual(["user.remove_trusted_friend"])
  })
})

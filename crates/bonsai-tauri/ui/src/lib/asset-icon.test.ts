import { describe, expect, it } from "vitest"
import { assetIcon } from "./asset-icon"

describe("assetIcon", () => {
  it("picks an icon per development asset type", () => {
    expect(assetIcon(13)).toBe("image")
    expect(assetIcon(3)).toBe("audio")
    expect(assetIcon(10)).toBe("model")
    expect(assetIcon(38)).toBe("plugin")
    expect(assetIcon(62)).toBe("video")
  })
  it("falls back to the avatar icon for wearables and unknown types", () => {
    expect(assetIcon(8)).toBe("avatar")
    expect(assetIcon(0)).toBe("avatar")
    expect(assetIcon(undefined)).toBe("avatar")
  })
})

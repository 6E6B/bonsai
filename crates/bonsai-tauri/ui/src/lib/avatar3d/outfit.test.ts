import { describe, expect, it } from "vitest"
import type { Avatar3dView, WornView } from "~/lib/ipc/bindings"
import { animationName, canTryOn, emoteIds, outfitKey, showcaseAnimation, toOutfitJson, tryOnItem } from "./outfit"

const worn = (id: number, typeId: number, layer: number | null = null): WornView => ({
  id,
  name: `Item ${id}`,
  typeId,
  typeName: "",
  r15Only: false,
  layer,
})

const view = (over: Partial<Avatar3dView> = {}): Avatar3dView => ({
  loading: false,
  error: null,
  rig: "R15",
  assets: [worn(1, 8), worn(2, 64, 5)],
  emotes: [worn(30, 3), worn(10, 1)],
  colors: [1, 2, 3, 4, 5, 6],
  scales: { height: 1, width: 0.9, head: 1, depth: null, proportion: 0, bodyType: 0.3 },
  rev: 1,
  ...over,
})

describe("toOutfitJson", () => {
  it("maps colors in part order and layers to meta.order", () => {
    const j = toOutfitJson(view())
    expect(j.playerAvatarType).toBe("R15")
    expect(j.bodyColors).toEqual({
      headColorId: 1,
      torsoColorId: 2,
      leftArmColorId: 3,
      rightArmColorId: 4,
      leftLegColorId: 5,
      rightLegColorId: 6,
    })
    expect(j.assets).toEqual([
      { id: 1, name: "Item 1", assetType: { id: 8 } },
      { id: 2, name: "Item 2", assetType: { id: 64 }, meta: { order: 5 } },
    ])
    expect(j.scale.depth).toBeUndefined()
    expect(j.scale.bodyType).toBe(0.3)
  })

  it("treats anything but R6 as R15", () => {
    expect(toOutfitJson(view({ rig: "R6" })).playerAvatarType).toBe("R6")
    expect(toOutfitJson(view({ rig: "" })).playerAvatarType).toBe("R15")
  })
})

describe("outfitKey", () => {
  it("changes with the outfit and the try-on item", () => {
    const j = toOutfitJson(view())
    const item = { id: 9, name: "Hat", typeId: 8 }
    expect(outfitKey(j, null)).toBe(outfitKey(toOutfitJson(view()), null))
    expect(outfitKey(j, item)).not.toBe(outfitKey(j, null))
    expect(outfitKey(toOutfitJson(view({ colors: [9, 2, 3, 4, 5, 6] })), null)).not.toBe(outfitKey(j, null))
  })
})

describe("emoteIds", () => {
  it("sorts by slot", () => {
    expect(emoteIds(view()).map((e) => e.slot)).toEqual([1, 3])
  })
})

describe("animationName", () => {
  it("defaults to idle and keeps set animations", () => {
    expect(animationName(null)).toBe("idle")
    expect(animationName("  ")).toBe("idle")
    expect(animationName("Walk")).toBe("walk")
  })

  it("reads emote ids in any form", () => {
    expect(animationName("123")).toBe("emote.123")
    expect(animationName("emote.123")).toBe("emote.123")
    expect(animationName("emote:123")).toBe("emote.123")
  })

  it("rejects anything else", () => {
    expect(animationName("dance")).toBeNull()
    expect(animationName("id.5")).toBeNull()
  })
})

describe("try on", () => {
  it("knows what can be tried on", () => {
    expect(canTryOn(8)).toBe(true)
    expect(canTryOn(61)).toBe(true)
    expect(canTryOn(19)).toBe(false)
    expect(canTryOn(13)).toBe(false)
  })

  it("plays emotes instead of wearing them", () => {
    const emote = { id: 5, name: "Wave", typeId: 61 }
    expect(tryOnItem(emote)).toBeNull()
    expect(showcaseAnimation(emote)).toBe("emote.5")
  })

  it("shows off animation packs with the animation they replace", () => {
    const walk = { id: 6, name: "Walk", typeId: 55 }
    expect(tryOnItem(walk)).toEqual(walk)
    expect(showcaseAnimation(walk)).toBe("walk")
    expect(showcaseAnimation({ id: 7, name: "Hat", typeId: 8 })).toBeUndefined()
  })
})

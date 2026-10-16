import { describe, expect, it } from "vitest"
import type { Avatar3dView, WornView } from "~/lib/ipc/bindings"
import {
  animationName,
  canTryOn,
  emoteIds,
  MANNEQUIN,
  showcaseAnimation,
  showsAlone,
  subjectKey,
  toOutfitJson,
  wearable,
} from "./outfit"

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

describe("subjectKey", () => {
  it("changes with the outfit and the worn items", () => {
    const outfit = (j = toOutfitJson(view()), wear = [] as { id: number; name: string; typeId: number }[]) =>
      subjectKey({ kind: "outfit", outfit: j, wear })
    const hat = { id: 9, name: "Hat", typeId: 8 }
    expect(outfit()).toBe(outfit(toOutfitJson(view())))
    expect(outfit(undefined, [hat])).not.toBe(outfit())
    expect(outfit(toOutfitJson(view({ colors: [9, 2, 3, 4, 5, 6] })))).not.toBe(outfit())
    expect(subjectKey({ kind: "accessory", id: 9 })).not.toBe(outfit(MANNEQUIN, [hat]))
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
  const emote = { id: 5, name: "Wave", typeId: 61 }
  const walk = { id: 6, name: "Walk", typeId: 55 }
  const run = { id: 8, name: "Run", typeId: 53 }
  const hat = { id: 7, name: "Hat", typeId: 8 }

  it("knows what can be tried on", () => {
    expect(canTryOn(8)).toBe(true)
    expect(canTryOn(61)).toBe(true)
    expect(canTryOn(19)).toBe(false)
    expect(canTryOn(13)).toBe(false)
  })

  it("draws rigid accessories alone and the rest on a mannequin", () => {
    expect(showsAlone(8)).toBe(true)
    expect(showsAlone(64)).toBe(false)
    expect(showsAlone(11)).toBe(false)
  })

  it("plays emotes instead of wearing them", () => {
    expect(wearable([emote])).toEqual([])
    expect(showcaseAnimation([emote])).toBe("emote.5")
  })

  it("wears bundle parts and skips unknown types", () => {
    expect(wearable([hat, { id: 1, name: "?", typeId: 0 }, walk])).toEqual([hat, walk])
  })

  it("shows off animation packs with the animation they replace", () => {
    expect(showcaseAnimation([walk])).toBe("walk")
    expect(showcaseAnimation([{ id: 9, name: "Climb", typeId: 48 }, run, walk])).toBe("walk")
    expect(showcaseAnimation([hat])).toBeUndefined()
  })
})

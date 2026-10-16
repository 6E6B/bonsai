import type { Avatar3dView } from "~/lib/ipc/bindings"

/** The parts of the engine's `OutfitJson` we fill in. Kept here so this file and its tests
 * don't load the engine. */
export interface OutfitJson {
  playerAvatarType: "R6" | "R15"
  scale: {
    height?: number
    width?: number
    head?: number
    depth?: number
    proportion?: number
    bodyType?: number
  }
  bodyColors: {
    headColorId?: number
    torsoColorId?: number
    leftArmColorId?: number
    rightArmColorId?: number
    leftLegColorId?: number
    rightLegColorId?: number
  }
  assets: { id: number; name: string; assetType: { id: number }; meta?: { order: number } }[]
}

/** An item to wear on top of an avatar for Try On. */
export interface TryOnItem {
  id: number
  name: string
  typeId: number
}

const num = (v: number | null) => v ?? undefined

/** The engine outfit for an avatar view. Emotes aren't worn; the viewer plays them. */
export function toOutfitJson(view: Avatar3dView): OutfitJson {
  const [head, torso, leftArm, rightArm, leftLeg, rightLeg] = view.colors
  return {
    playerAvatarType: view.rig === "R6" ? "R6" : "R15",
    scale: {
      height: num(view.scales.height),
      width: num(view.scales.width),
      head: num(view.scales.head),
      depth: num(view.scales.depth),
      proportion: num(view.scales.proportion),
      bodyType: num(view.scales.bodyType),
    },
    bodyColors: {
      headColorId: head,
      torsoColorId: torso,
      leftArmColorId: leftArm,
      rightArmColorId: rightArm,
      leftLegColorId: leftLeg,
      rightLegColorId: rightLeg,
    },
    assets: view.assets.map((a) => ({
      id: a.id,
      name: a.name,
      assetType: { id: a.typeId },
      ...(a.layer === null ? {} : { meta: { order: a.layer } }),
    })),
  }
}

/** A stable key for what an outfit looks like, so the viewer only rebuilds on real changes. */
export function outfitKey(json: OutfitJson, tryOn: TryOnItem | null) {
  return JSON.stringify([json, tryOn?.id ?? null])
}

/** Asset ids of the equipped emotes, by hotbar slot. */
export function emoteIds(view: Avatar3dView) {
  return [...view.emotes].sort((a, b) => a.typeId - b.typeId).map((e) => ({ id: e.id, name: e.name, slot: e.typeId }))
}

/** Animations the rig plays from its animation set. */
export const BASIC_ANIMATIONS = ["idle", "walk", "run", "jump", "fall", "climb", "swim"] as const

/** The engine's animation name for a `avatar.play` argument: a set animation or `emote.<id>`.
 * Null when the argument is neither. */
export function animationName(arg: string | null): string | null {
  const a = arg?.trim().toLowerCase() ?? ""
  if (!a) return "idle"
  if ((BASIC_ANIMATIONS as readonly string[]).includes(a)) return a
  const m = /^(?:emote[.:]?)?(\d+)$/.exec(a)
  return m ? `emote.${m[1]}` : null
}

/** Asset types that can be worn on an avatar, by id (the engine's wearable types, less gear). */
const WEARABLE = new Set([
  2, 8, 11, 12, 17, 18, 27, 28, 29, 30, 31, 41, 42, 43, 44, 45, 46, 47, 48, 50, 51, 52, 53, 54, 55, 56, 64, 65, 66, 67,
  68, 69, 70, 71, 72, 76, 77, 78, 79, 88, 89, 90,
])
const EMOTE = 61

/** Animation asset types and the animation they replace. */
const ANIMATION_TYPES: Record<number, string> = {
  48: "climb",
  50: "fall",
  51: "idle",
  52: "jump",
  53: "run",
  54: "swim",
  55: "walk",
  56: "pose",
}

/** Whether an asset of this type can be tried on: worn, or for emotes, played. */
export const canTryOn = (typeId: number) => WEARABLE.has(typeId) || typeId === EMOTE

/** What Try On wears for an item. Emotes aren't worn; they're played. */
export const tryOnItem = (item: TryOnItem): TryOnItem | null => (item.typeId === EMOTE ? null : item)

/** The animation that shows an item off: the emote itself, or the animation a pack replaces. */
export function showcaseAnimation(item: TryOnItem): string | undefined {
  if (item.typeId === EMOTE) return `emote.${item.id}`
  return ANIMATION_TYPES[item.typeId]
}

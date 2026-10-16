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

/** What the stage draws: an outfit with items worn on top, or one accessory by itself. */
export type Subject = { kind: "outfit"; outfit: OutfitJson; wear: TryOnItem[] } | { kind: "accessory"; id: number }

/** A stable key for what a subject looks like, so the viewer only rebuilds on real changes. */
export function subjectKey(s: Subject) {
  return s.kind === "accessory" ? `accessory:${s.id}` : JSON.stringify([s.outfit, s.wear.map((w) => w.id)])
}

const GREY = 194

/** A plain grey R15 body at default scale, for showing items without anyone's avatar. */
export const MANNEQUIN: OutfitJson = {
  playerAvatarType: "R15",
  scale: {},
  bodyColors: {
    headColorId: GREY,
    torsoColorId: GREY,
    leftArmColorId: GREY,
    rightArmColorId: GREY,
    leftLegColorId: GREY,
    rightLegColorId: GREY,
  },
  assets: [],
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

/** Rigid accessories (hats, hair, face, neck, shoulder, front, back, waist), which are drawn
 * by themselves. Other items are shown on a mannequin. */
const RIGID_ACCESSORY = new Set([8, 41, 42, 43, 44, 45, 46, 47])

/** Whether an asset of this type can be tried on: worn, or for emotes, played. */
export const canTryOn = (typeId: number) => WEARABLE.has(typeId) || typeId === EMOTE

/** Whether an item is drawn by itself rather than on a mannequin. */
export const showsAlone = (typeId: number) => RIGID_ACCESSORY.has(typeId)

/** The items Try On wears from an item or a bundle's parts. Emotes aren't worn; they're played. */
export const wearable = (items: TryOnItem[]) => items.filter((i) => WEARABLE.has(i.typeId))

const SHOWCASE_ORDER = ["walk", "run", "idle", "jump", "fall", "swim", "climb"]

/** The animation that shows items off: an emote itself, or the animation a pack replaces
 * (walk first, for a whole pack). */
export function showcaseAnimation(items: TryOnItem[]): string | undefined {
  const emote = items.find((i) => i.typeId === EMOTE)
  if (emote) return `emote.${emote.id}`
  const names = new Set(items.map((i) => ANIMATION_TYPES[i.typeId]))
  return SHOWCASE_ORDER.find((n) => names.has(n))
}

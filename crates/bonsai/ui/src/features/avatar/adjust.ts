import type { AdjustRuleView, AvatarView, PlacementView, WornView } from "~/lib/ipc/bindings"

export type Group = "position" | "rotation" | "scale"
export type Axis = "x" | "y" | "z"

/** The three adjustments, their axes as the editor names them, and the value of each axis
 * when nothing is adjusted. */
export const GROUPS: readonly {
  key: Group
  label: string
  neutral: number
  axes: readonly { axis: Axis; label: string }[]
}[] = [
  {
    key: "position",
    label: "Position",
    neutral: 0,
    axes: [
      { axis: "x", label: "Side" },
      { axis: "y", label: "Height" },
      { axis: "z", label: "Depth" },
    ],
  },
  {
    key: "rotation",
    label: "Rotation",
    neutral: 0,
    axes: [
      { axis: "x", label: "Tilt" },
      { axis: "y", label: "Turn" },
      { axis: "z", label: "Roll" },
    ],
  },
  {
    key: "scale",
    label: "Scale",
    neutral: 1,
    axes: [
      { axis: "x", label: "Width" },
      { axis: "y", label: "Height" },
      { axis: "z", label: "Depth" },
    ],
  },
]

const EPS = 1e-4

export const DEFAULT_PLACEMENT: PlacementView = {
  position: { x: 0, y: 0, z: 0 },
  rotation: { x: 0, y: 0, z: 0 },
  scale: { x: 1, y: 1, z: 1 },
}

/** The bounds for an asset type; none when Roblox does not allow adjusting it. */
export const ruleFor = (view: AvatarView, typeId: number): AdjustRuleView | undefined =>
  view.adjustRules.find((r) => r.typeId === typeId)

/** Worn items that can be adjusted, in the order they are worn. */
export const adjustable = (view: AvatarView): WornView[] => view.wearing.filter((w) => !!ruleFor(view, w.typeId))

const AXES: readonly Axis[] = ["x", "y", "z"]

/** One axis of a placement. Roblox sends no value as null; that axis is at its default. */
export const axisValue = (p: PlacementView, group: Group, axis: Axis) =>
  p[group][axis] ?? DEFAULT_PLACEMENT[group][axis] ?? 0

const groupSame = (a: PlacementView, b: PlacementView, g: Group) =>
  AXES.every((x) => Math.abs(axisValue(a, g, x) - axisValue(b, g, x)) < EPS)

/** Whether one group, or the whole placement, differs from the default. */
export function isAdjusted(p: PlacementView | null, group?: Group): boolean {
  if (!p) return false
  const groups = group ? [group] : GROUPS.map((g) => g.key)
  return groups.some((g) => !groupSame(p, DEFAULT_PLACEMENT, g))
}

/** The `avatar.adjust` argument that sets one axis. Rounded to drop float noise. */
export const setArg = (id: number, group: Group, axis: Axis, value: number) =>
  `${id} ${group}.${axis} ${Number(value.toFixed(4))}`

export const resetArg = (id: number, group?: Group) => (group ? `${id} reset ${group}` : `${id} reset`)

export const copyArg = (id: number, from: number, mirrored: boolean) => `${id} ${mirrored ? "mirror" : "match"} ${from}`

const trim = (v: number, digits: number) => {
  const s = v.toFixed(digits).replace(/\.?0+$/, "")
  return s === "-0" ? "0" : s
}

/** How a value of a group reads on a slider. */
export function formatValue(group: Group, v: number): string {
  if (group === "position") return `${trim(v, 2)} studs`
  if (group === "rotation") return `${trim(v, 1)}°`
  return `${Math.round(v * 100)}%`
}

export interface Xyz {
  x: number
  y: number
  z: number
}

/** A placement with every axis filled in, as the 3D viewer draws it. */
export interface Placement {
  position: Xyz
  rotation: Xyz
  scale: Xyz
}

const full = (p: PlacementView | null, g: Group): Xyz => ({
  x: axisValue(p ?? DEFAULT_PLACEMENT, g, "x"),
  y: axisValue(p ?? DEFAULT_PLACEMENT, g, "y"),
  z: axisValue(p ?? DEFAULT_PLACEMENT, g, "z"),
})

export const toPlacement = (p: PlacementView | null): Placement => ({
  position: full(p, "position"),
  rotation: full(p, "rotation"),
  scale: full(p, "scale"),
})

export const withAxis = (p: Placement, group: Group, axis: Axis, value: number): Placement => ({
  ...p,
  [group]: { ...p[group], [axis]: value },
})

/** The `avatar.adjust` argument that moves `current` to `next`, setting only the axes that
 * changed. Null when nothing did. */
export function placementArg(id: number, next: Placement, current: Placement): string | null {
  const parts: string[] = []
  for (const g of GROUPS) {
    for (const a of AXES) {
      const v = next[g.key][a]
      if (Math.abs(v - current[g.key][a]) >= EPS) parts.push(`${g.key}.${a} ${Number(v.toFixed(4))}`)
    }
  }
  return parts.length ? `${id} ${parts.join(" ")}` : null
}

export type Tool = "move" | "turn" | "size"

const TOOLS: Record<string, Tool> = {
  move: "move",
  position: "move",
  turn: "turn",
  rotate: "turn",
  rotation: "turn",
  size: "size",
  scale: "size",
}

/** The handle an `avatar.adjust_tool` argument names. */
export const parseTool = (arg: string | null): Tool | undefined => TOOLS[arg?.trim().toLowerCase() ?? ""]

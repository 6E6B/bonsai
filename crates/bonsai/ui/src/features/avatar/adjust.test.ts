import { describe, expect, it } from "vitest"
import type { AvatarView, WornView } from "~/lib/ipc/bindings"
import {
  adjustable,
  copyArg,
  DEFAULT_PLACEMENT,
  formatValue,
  isAdjusted,
  parseTool,
  placementArg,
  resetArg,
  ruleFor,
  setArg,
  toPlacement,
  withAxis,
} from "./adjust"

const worn = (id: number, typeId: number): WornView => ({
  id,
  name: `Item ${id}`,
  typeId,
  typeName: "",
  r15Only: false,
  layer: null,
  placement: null,
})

const rule = (typeId: number) => ({ typeId, min: DEFAULT_PLACEMENT, max: DEFAULT_PLACEMENT })

const view = (): AvatarView => ({
  loading: false,
  error: null,
  rig: "R15",
  wearing: [worn(1, 8), worn(2, 41), worn(3, 46), worn(4, 11)],
  emotes: [],
  colors: [],
  scales: [],
  adjustRules: [rule(8), rule(46)],
  adjustSteps: { position: 0.01, rotation: 0.1, scale: 0.01 },
  rev: 0,
})

describe("adjust", () => {
  it("lists only the types Roblox allows", () => {
    expect(adjustable(view()).map((w) => w.id)).toEqual([1, 3])
    expect(ruleFor(view(), 41)).toBeUndefined()
  })

  it("tells adjusted placements apart within rounding", () => {
    expect(isAdjusted(null)).toBe(false)
    expect(isAdjusted(DEFAULT_PLACEMENT)).toBe(false)
    const p = { ...DEFAULT_PLACEMENT, scale: { x: 1.000_000_01, y: 1.1, z: 1 } }
    expect(isAdjusted(p)).toBe(true)
    expect(isAdjusted(p, "scale")).toBe(true)
    expect(isAdjusted(p, "position")).toBe(false)
  })

  it("builds command arguments", () => {
    expect(setArg(5, "rotation", "y", -12.300000000000001)).toBe("5 rotation.y -12.3")
    expect(resetArg(5)).toBe("5 reset")
    expect(resetArg(5, "scale")).toBe("5 reset scale")
    expect(copyArg(5, 6, false)).toBe("5 match 6")
    expect(copyArg(5, 6, true)).toBe("5 mirror 6")
  })

  it("formats values per group", () => {
    expect(formatValue("position", 0.1)).toBe("0.1 studs")
    expect(formatValue("position", -0)).toBe("0 studs")
    expect(formatValue("rotation", 12.25)).toBe("12.3°")
    expect(formatValue("scale", 1.1)).toBe("110%")
  })

  it("fills missing axes and sets only what changed", () => {
    const now = toPlacement({ ...DEFAULT_PLACEMENT, rotation: { x: null, y: 5, z: null } })
    expect(now.rotation).toEqual({ x: 0, y: 5, z: 0 })
    expect(placementArg(7, now, now)).toBeNull()
    const next = withAxis(withAxis(now, "position", "y", 0.12000001), "scale", "z", 1.1)
    expect(placementArg(7, next, now)).toBe("7 position.y 0.12 scale.z 1.1")
  })

  it("reads tool names", () => {
    expect(parseTool(" Rotate ")).toBe("turn")
    expect(parseTool("scale")).toBe("size")
    expect(parseTool("move")).toBe("move")
    expect(parseTool("fly")).toBeUndefined()
    expect(parseTool(null)).toBeUndefined()
  })
})

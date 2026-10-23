import { describe, expect, it } from "vitest"
import type { DependencyView } from "~/lib/ipc/bindings"
import { filterDependencies, propertyLabel, usageLabels } from "./dependencies"

const mesh: DependencyView = {
  id: 100,
  name: "Accessory mesh",
  assetType: 4,
  typeName: "Mesh",
  creator: "Builder",
  creatorId: 10,
  creatorIsGroup: true,
  creatorVerified: false,
  uses: [{ class: "MeshPart", path: "Hat/Handle", property: "MeshId" }],
}
const texture: DependencyView = {
  ...mesh,
  id: 200,
  name: null,
  assetType: 0,
  typeName: "Unknown",
  creator: "",
  uses: [
    { class: "SurfaceAppearance", path: "Hat/Handle/SurfaceAppearance", property: "ColorMap" },
    { class: "SurfaceAppearance", path: "Hat/Other/SurfaceAppearance", property: "ColorMap" },
    { class: "SurfaceAppearance", path: "Hat/Handle/SurfaceAppearance", property: "NormalMap" },
  ],
}

describe("dependency browsing", () => {
  it("deduplicates presentation labels without losing source references", () => {
    expect(usageLabels(texture)).toEqual(["Base color", "Normal map"])
    expect(texture.uses).toHaveLength(3)
    expect(propertyLabel("FutureContentProperty")).toBe("FutureContentProperty")
  })
  it("matches all search terms across names, creators, IDs, properties and paths", () => {
    expect(filterDependencies([mesh, texture], " BUILDER mesh ")).toEqual([mesh])
    expect(filterDependencies([mesh, texture], "base color surfaceappearance")).toEqual([texture])
    expect(filterDependencies([mesh, texture], "200")).toEqual([texture])
    expect(filterDependencies([mesh, texture], "Hat/Handle MeshId")).toEqual([mesh])
    expect(filterDependencies([mesh, texture], "missing")).toEqual([])
    expect(filterDependencies([mesh, texture], "   ")).toEqual([mesh, texture])
  })
})

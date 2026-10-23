import type { DependencyView } from "~/lib/ipc/bindings"

const labels: Record<string, string> = {
  MeshContent: "Mesh",
  TextureContent: "Texture",
  ColorMapContent: "Base color",
  NormalMapContent: "Normal map",
  MetalnessMapContent: "Metalness",
  RoughnessMapContent: "Roughness",
  MeshId: "Mesh",
  TextureID: "Texture",
  TextureId: "Texture",
  Texture: "Texture",
  ColorMap: "Base color",
  NormalMap: "Normal map",
  MetalnessMap: "Metalness",
  RoughnessMap: "Roughness",
  CageMeshId: "Cage mesh",
  ReferenceMeshId: "Reference mesh",
  ShirtTemplate: "Shirt template",
  PantsTemplate: "Pants template",
  Graphic: "Graphic",
  SoundId: "Audio",
  AnimationId: "Animation",
}

export const propertyLabel = (property: string) => labels[property] ?? property

export function usageLabels(item: DependencyView): string[] {
  return [...new Set(item.uses.map((use) => propertyLabel(use.property)))].sort()
}

export function filterDependencies(items: readonly DependencyView[], query: string): DependencyView[] {
  const words = query.trim().toLocaleLowerCase().split(/\s+/).filter(Boolean)
  return items.filter((item) => {
    const text = [
      item.id,
      item.name ?? "",
      item.typeName,
      item.creator,
      ...item.uses.flatMap((use) => [use.class, use.path, use.property, propertyLabel(use.property)]),
    ]
      .join(" ")
      .toLocaleLowerCase()
    return words.every((word) => text.includes(word))
  })
}

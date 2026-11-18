export const sortOptions = [
  { value: "newest", label: "Newest first" },
  { value: "oldest", label: "Oldest first" },
  { value: "name", label: "Name A–Z" },
  { value: "name-desc", label: "Name Z–A" },
]

/** Keep API acquisition order. Asset ids are creation dates, not acquisition dates. */
export function named<T extends { name: string }>(items: readonly T[], query: string, sort: string): T[] {
  const words = query.trim().toLocaleLowerCase().split(/\s+/).filter(Boolean)
  const result = items.filter((item) => words.every((word) => item.name.toLocaleLowerCase().includes(word)))
  if (sort === "name" || sort === "name-desc") {
    result.sort((a, b) => a.name.localeCompare(b.name, undefined, { numeric: true, sensitivity: "base" }))
    if (sort === "name-desc") result.reverse()
  }
  return result
}

export function matchesWear(id: number, worn: readonly number[], filter: string): boolean {
  return filter === "worn" ? worn.includes(id) : filter !== "unworn" || !worn.includes(id)
}

/** The status line over wardrobe search results. */
export function searchStatus(r: {
  items: readonly unknown[]
  loaded: number
  total: number
  failed: number
  partial: boolean
  truncated: boolean
}): string {
  const parts = [`${r.truncated ? "First " : ""}${r.items.length} ${r.items.length === 1 ? "match" : "matches"}`]
  if (r.total && r.loaded < r.total) parts.push(`loading ${r.loaded} of ${r.total} types`)
  if (r.failed) parts.push(`${r.failed} ${r.failed === 1 ? "type" : "types"} failed to load`)
  if (r.partial) parts.push("some types have more items; open their tab to load them")
  return parts.join(" · ")
}

/** The groups of the worn list, in the order they show. */
export type WornGroup = "head" | "accessories" | "clothing" | "body" | "animation"

export const WORN_GROUPS: readonly { key: WornGroup; label: string; types: readonly number[] }[] = [
  // Hat, hair, face, face accessory, head, dynamic head, eyebrows, eyelashes.
  { key: "head", label: "Head", types: [8, 41, 18, 42, 17, 79, 76, 77] },
  // Neck, shoulder, front, back, waist, gear.
  { key: "accessories", label: "Accessories", types: [43, 44, 45, 46, 47, 19] },
  // T-shirt, shirt, pants, then layered clothing and shoes.
  { key: "clothing", label: "Clothing", types: [2, 11, 12, 64, 65, 66, 67, 68, 69, 70, 71, 72] },
  // Torso, arms, legs.
  { key: "body", label: "Body", types: [27, 28, 29, 30, 31] },
  // Climb, death, fall, idle, jump, run, swim, walk, pose, mood.
  { key: "animation", label: "Animation", types: [48, 49, 50, 51, 52, 53, 54, 55, 56, 78] },
]

/** The group an asset type is listed under. Types we don't know go with accessories. */
export const groupTypes = (group: WornGroup) => WORN_GROUPS.find((g) => g.key === group)?.types ?? []

export const groupOf = (typeId: number): WornGroup =>
  WORN_GROUPS.find((g) => g.types.includes(typeId))?.key ?? "accessories"

/** Worn items by group, in `WORN_GROUPS` order. Within a group, items keep the group's type
 * order, and layered clothing goes outermost first, as it stacks. */
export function groupWorn<T extends { typeId: number; layer: number | null }>(
  worn: readonly T[],
): { key: WornGroup; label: string; items: T[] }[] {
  return WORN_GROUPS.map((g) => ({
    key: g.key,
    label: g.label,
    items: worn
      .filter((w) => groupOf(w.typeId) === g.key)
      .sort((a, b) => {
        if (a.layer !== null || b.layer !== null)
          return (b.layer ?? Number.NEGATIVE_INFINITY) - (a.layer ?? Number.NEGATIVE_INFINITY)
        return rank(g.types, a.typeId) - rank(g.types, b.typeId)
      }),
  }))
}

const rank = (types: readonly number[], t: number) => {
  const i = types.indexOf(t)
  return i < 0 ? types.length : i
}

/** The wardrobe section and slot to browse for some asset types: the first section with a slot
 * of one of them, in the order given. */
export function sectionFor(
  sections: readonly { name: string; slots: readonly { value: string }[] }[],
  types: readonly number[],
): { section: string; slot: string } | undefined {
  for (const t of types)
    for (const s of sections) if (s.slots.some((x) => x.value === `a${t}`)) return { section: s.name, slot: `a${t}` }
  return undefined
}

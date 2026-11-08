import type { InvCategoryView } from "~/lib/ipc/bindings"
import type { CategoryNode, CategorySection } from "~/lib/ui"

const EXPERIENCES = new Set(["places", "badges", "passes"])
/** Decals, Models & Packages, Plugins, Animations, Audio, Meshes and Video. */
const CREATIONS = new Set(["a13", "a10", "a38", "a24", "a3", "a40", "a62"])

/** Inventory categories in the order people look for them: collectibles, then what a player
 * wears, then experiences, then creator assets. A new type Roblox adds lands under Avatar, where
 * most new types belong. The server's order holds inside each section. */
export function inventorySections(categories: readonly InvCategoryView[]): CategorySection[] {
  const top: CategoryNode[] = []
  const avatar: CategoryNode[] = []
  const experiences: CategoryNode[] = []
  const creations: CategoryNode[] = []
  for (const c of categories) {
    const first = c.slots[0]?.value
    if (first === undefined) continue
    const node: CategoryNode =
      c.slots.length === 1 ? { value: first, label: c.name } : { label: c.name, children: c.slots }
    if (first === "collectibles") top.push(node)
    else if (EXPERIENCES.has(first)) experiences.push(node)
    else if (CREATIONS.has(first)) creations.push(node)
    else avatar.push(node)
  }
  const sections: CategorySection[] = []
  if (top.length) sections.push({ items: top })
  if (avatar.length) sections.push({ label: "Avatar", items: avatar })
  if (experiences.length) sections.push({ label: "Experiences", items: experiences })
  if (creations.length) sections.push({ label: "Creations", items: creations })
  return sections
}

/** The slot a `slot` param means: a slot key, or a name typed in the command bar. Null when
 * nothing matches. Until the list loads, a param is taken as it is. */
export function findSlot(categories: readonly InvCategoryView[], param: string): string | null {
  if (categories.length === 0) return param
  const name = param.toLowerCase()
  for (const c of categories) if (c.slots.some((s) => s.value === param)) return param
  for (const c of categories) {
    const s = c.slots.find((s) => s.label.toLowerCase() === name)
    if (s) return s.value
    const only = c.slots[0]
    if (only && c.slots.length === 1 && c.name.toLowerCase() === name) return only.value
  }
  return null
}

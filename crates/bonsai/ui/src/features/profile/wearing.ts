/** Groups worn items like the avatar editor does, so a long look reads at a glance. */
export type WornKind = "accessories" | "clothing" | "body" | "animations" | "other"

const labels: Record<WornKind, string> = {
  accessories: "Accessories",
  clothing: "Clothing",
  body: "Body",
  animations: "Animations",
  other: "Other",
}
const order: WornKind[] = ["clothing", "accessories", "body", "animations", "other"]

// Roblox asset type ids.
const ACCESSORIES = new Set([8, 41, 42, 43, 44, 45, 46, 47, 76, 77])
const CLOTHING = new Set([2, 11, 12, 64, 65, 66, 67, 68, 69, 70, 71, 72])
const BODY = new Set([17, 18, 27, 28, 29, 30, 31, 79])
const ANIMATIONS = new Set([48, 50, 51, 52, 53, 54, 55, 56, 61, 78])

/** Which group an asset type falls in. */
export function wornKind(assetType: number): WornKind {
  if (ACCESSORIES.has(assetType)) return "accessories"
  if (CLOTHING.has(assetType)) return "clothing"
  if (BODY.has(assetType)) return "body"
  if (ANIMATIONS.has(assetType)) return "animations"
  return "other"
}

/** Kinds worth a thumbnail on the profile overview. Body parts and animations mostly render as
 * the same blank figure, so the overview only counts them. */
export const SHOWN: ReadonlySet<WornKind> = new Set<WornKind>(["accessories", "clothing", "other"])

/** One page of `size` items. `index` is clamped to the pages there are, so a list that shrank
 * under the pager still shows its last page. There is always at least one page. */
export function pageOf<T>(items: T[], index: number, size: number) {
  const pages = Math.max(1, Math.ceil(items.length / size))
  const page = Math.min(Math.max(0, index), pages - 1)
  return { items: items.slice(page * size, (page + 1) * size), page, pages }
}

/** Items split into groups, clothing first, keeping Roblox's order inside each. Empty groups
 * are left out. */
export function wornGroups<T extends { assetType: number }>(items: T[]) {
  return order
    .map((kind) => ({ kind, label: labels[kind], items: items.filter((i) => wornKind(i.assetType) === kind) }))
    .filter((g) => g.items.length > 0)
}

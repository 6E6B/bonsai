import type { ItemCard } from "~/lib/ipc/bindings"
import { matches as matchesText, rank } from "~/lib/search"
import { demandLevel, worth } from "~/lib/values"

export type InvFilter = "all" | "limited" | "wearing" | "copies" | "for_sale"

export const FILTERS: { value: InvFilter; label: string }[] = [
  { value: "all", label: "All items" },
  { value: "limited", label: "Limiteds" },
  { value: "wearing", label: "Wearing" },
  { value: "copies", label: "More than one copy" },
  { value: "for_sale", label: "For sale" },
]

export const isFilter = (v: string): v is InvFilter => FILTERS.some((f) => f.value === v)

type Searched = Pick<ItemCard, "id" | "name" | "creator">

/** Text search over loaded items: every word is in the name or creator, or it is the item's id. */
export function matches(i: Searched, query: string): boolean {
  const q = query.trim()
  return !q || String(i.id) === q || matchesText([i.name, i.creator], q)
}

/** Loaded items matching `query`, best match first (see `rank`). */
export const searchItems = <T extends Searched>(items: readonly T[], query: string): T[] =>
  rank(
    items,
    query,
    (i) => [i.name, i.creator],
    (i) => i.id,
  )

function keep(i: ItemCard, f: InvFilter, worn: ReadonlySet<number>): boolean {
  if (f === "limited") return i.limited
  if (f === "wearing") return worn.has(i.id)
  if (f === "copies") return i.copies > 1
  if (f === "for_sale") return i.price !== null
  return true
}

const demandOf = (i: ItemCard) => (i.values?.demand ? demandLevel(i.values.demand) : 0)

/** Loaded items after search, filter and sort. `newest` and `oldest` keep the server's order,
 * except that a search under the default `newest` puts the best matches first; any other sort
 * the user picked wins over relevance. Price sorts use RAP for collectibles; items without a
 * price go last either way. */
export function shapeInventory(
  items: readonly ItemCard[],
  o: { query: string; filter: InvFilter; sort: string; worn: ReadonlySet<number>; collectibles: boolean },
): ItemCard[] {
  const kept = items.filter((i) => keep(i, o.filter, o.worn))
  if (o.sort === "newest") return searchItems(kept, o.query)
  const list = kept.filter((i) => matches(i, o.query))
  const cost = (i: ItemCard) => (o.collectibles ? (i.rap ?? i.price) : i.price)
  const by = (f: (a: ItemCard, b: ItemCard) => number) => list.sort(f)
  if (o.sort === "price_asc")
    return by((a, b) => (cost(a) ?? Number.POSITIVE_INFINITY) - (cost(b) ?? Number.POSITIVE_INFINITY))
  if (o.sort === "price_desc") return by((a, b) => (cost(b) ?? -1) - (cost(a) ?? -1))
  if (o.sort === "rap") return by((a, b) => (b.rap ?? 0) - (a.rap ?? 0))
  if (o.sort === "value") return by((a, b) => (worth(b.values, b.rap) ?? 0) - (worth(a.values, a.rap) ?? 0))
  if (o.sort === "demand") return by((a, b) => demandOf(b) - demandOf(a))
  return list
}

/** Asset type of a wardrobe slot key (`a<type>`), or 0 for other slots. */
export const slotAssetType = (slot: string) => (/^a\d+$/.test(slot) ? Number(slot.slice(1)) : 0)

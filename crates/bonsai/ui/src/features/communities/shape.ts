import type { GroupCard } from "~/lib/ipc/bindings"
import { matches, rank } from "~/lib/search"

export const SORTS = [
  { value: "default", label: "Default order" },
  { value: "name", label: "Name" },
  { value: "members", label: "Most members" },
  { value: "fewest", label: "Fewest members" },
  { value: "role", label: "Your role" },
]

export const SHOWS = [
  { value: "all", label: "All communities" },
  { value: "verified", label: "Verified" },
  { value: "primary", label: "Primary" },
]

export const isSort = (v: string) => SORTS.some((s) => s.value === v)
export const isShow = (v: string) => SHOWS.some((s) => s.value === v)

const byName = (a: GroupCard, b: GroupCard) => a.name.localeCompare(b.name, undefined, { sensitivity: "base" })

const compare: Record<string, (a: GroupCard, b: GroupCard) => number> = {
  name: byName,
  members: (a, b) => b.members - a.members || byName(a, b),
  fewest: (a, b) => a.members - b.members || byName(a, b),
  role: (a, b) => a.role.localeCompare(b.role, undefined, { sensitivity: "base" }) || byName(a, b),
}

/** Your communities after search, filter and sort. Under the default order a search puts the
 * best matches first; any other sort wins over how well a community matches. */
export function shapeCommunities(
  items: readonly GroupCard[],
  o: { query: string; sort: string; show: string },
): GroupCard[] {
  const kept = items.filter((c) => (o.show === "verified" ? c.verified : o.show === "primary" ? c.primary : true))
  const by = compare[o.sort]
  if (!by) return rank(kept, o.query, (c) => [c.name, c.role])
  return kept.filter((c) => matches([c.name, c.role], o.query)).sort(by)
}

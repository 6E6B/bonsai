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

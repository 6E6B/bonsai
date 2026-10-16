export interface Searchable {
  title: string
  words: string
}

/** Keeps entries whose title or keywords contain every word of the query. Title matches
 * rank first, then title prefix matches; otherwise the original order is kept. */
export function filterEntries<T extends Searchable>(entries: T[], query: string): T[] {
  const terms = query.toLowerCase().split(/\s+/).filter(Boolean)
  if (terms.length === 0) return entries

  const scored: { entry: T; score: number; index: number }[] = []
  entries.forEach((entry, index) => {
    const title = entry.title.toLowerCase()
    const hay = `${title} ${entry.words.toLowerCase()}`
    if (!terms.every((t) => hay.includes(t))) return
    let score = 0
    if (terms.every((t) => title.includes(t))) score += 1
    if (title.startsWith(terms[0])) score += 1
    scored.push({ entry, score, index })
  })

  return scored.sort((a, b) => b.score - a.score || a.index - b.index).map((s) => s.entry)
}

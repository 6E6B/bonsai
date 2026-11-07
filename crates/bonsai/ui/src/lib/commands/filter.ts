import { rank } from "~/lib/search"

export interface Searchable {
  title: string
  words: string
}

/** Keeps entries whose title or keywords contain every word of the query, best match first
 * (see `rank`). Title matches count more than keyword matches. */
export function filterEntries<T extends Searchable>(entries: T[], query: string): T[] {
  return rank(entries, query, (e) => [e.title, e.words])
}

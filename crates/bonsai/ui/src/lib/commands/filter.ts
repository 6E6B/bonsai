import { rankScored, type Searchable } from "./match"

export type { Searchable }

/** Keeps entries whose title or keywords match the query, best match first (see `match.ts`).
 * Title matches count more than keyword matches; typos and synonyms are allowed. */
export function filterEntries<T extends Searchable>(entries: T[], query: string): T[] {
  return rankScored(entries, query).map((r) => r.entry)
}

/** Text search shared by every list with a filter box.
 *
 * An item matches when every word of the query is found in one of its fields. Matches are
 * ranked: a field equal to the query, then one starting with it, then a word inside a field
 * starting with it ("luke" in "StickMasterLuke"), then any substring. Earlier fields count
 * a little more than later ones. Ties keep the list's own order, so a friends list sorted by
 * presence stays sorted that way among equally good matches. Case and accents are ignored. */

export type Fields = readonly (string | null | undefined)[]

/** Lowercase without accents, for comparing. */
export const fold = (s: string) => strip(s).toLowerCase()

const strip = (s: string) => s.normalize("NFD").replace(/\p{M}/gu, "")

const words = (query: string) => fold(query).split(/\s+/).filter(Boolean)

const isLetter = (c: string) => /\p{L}/u.test(c)
const isDigit = (c: string) => c >= "0" && c <= "9"
const isUpper = (c: string) => c !== c.toLowerCase()

/** Whether a word starts at `i` of `s`: after a separator, at a camelCase hump, or where
 * letters and digits meet. */
function wordStart(s: string, i: number): boolean {
  if (i === 0) return true
  const prev = s[i - 1]
  const cur = s[i]
  if (!isLetter(prev) && !isDigit(prev)) return true
  if (isLetter(prev) && isDigit(cur)) return true
  if (isDigit(prev) && isLetter(cur)) return true
  return isUpper(cur) && !isUpper(prev)
}

interface Field {
  raw: string
  low: string
}

/** How well `term` matches one field: 0 for not at all. */
function tier(f: Field, term: string): number {
  if (f.low === term) return 100
  if (f.low.startsWith(term)) return 60
  let at = f.low.indexOf(term)
  if (at < 0) return 0
  while (at >= 0) {
    if (wordStart(f.raw, at)) return 40
    at = f.low.indexOf(term, at + 1)
  }
  return 20
}

/** Field `i` counts this much less than the first. */
const FIELD_STEP = 2

function prepare(fields: Fields): Field[] {
  return fields.flatMap((s) => {
    if (!s) return []
    const raw = strip(s)
    return [{ raw, low: raw.toLowerCase() }]
  })
}

function scoreTerms(fields: Field[], terms: string[], phrase: string): number {
  let total = 0
  for (const term of terms) {
    let best = 0
    fields.forEach((f, i) => {
      const t = tier(f, term)
      if (t) best = Math.max(best, t - i * FIELD_STEP)
    })
    if (best <= 0) return -1
    total += best
  }
  // A multi-word query read as one phrase: "tower of" is a strong match for "Tower of Hell".
  if (terms.length > 1) {
    let best = 0
    fields.forEach((f, i) => {
      const t = tier(f, phrase)
      if (t) best = Math.max(best, (t - i * FIELD_STEP) * terms.length)
    })
    total += best
  }
  return total
}

/** How well `fields` match `query`: -1 for no match, 0 for an empty query, higher is better. */
export function score(fields: Fields, query: string): number {
  const terms = words(query)
  if (terms.length === 0) return 0
  return scoreTerms(prepare(fields), terms, terms.join(" "))
}

/** Whether `fields` match `query`. Everything matches an empty query. */
export const matches = (fields: Fields, query: string) => score(fields, query) >= 0

/** `items` matching `query`, best first. A blank query keeps them all, in order. `fields`
 * lists what to search, most important first. An item whose `id` is exactly the query
 * matches too, ahead of everything else. */
export function rank<T>(
  items: readonly T[],
  query: string,
  fields: (item: T) => Fields,
  id?: (item: T) => number | string,
): T[] {
  const terms = words(query)
  if (terms.length === 0) return [...items]
  const phrase = terms.join(" ")
  const exact = query.trim()
  const scored: { item: T; score: number; index: number }[] = []
  items.forEach((item, index) => {
    const s =
      id && String(id(item)) === exact ? Number.POSITIVE_INFINITY : scoreTerms(prepare(fields(item)), terms, phrase)
    if (s >= 0) scored.push({ item, score: s, index })
  })
  return scored.sort((a, b) => (a.score === b.score ? a.index - b.index : b.score - a.score)).map((s) => s.item)
}

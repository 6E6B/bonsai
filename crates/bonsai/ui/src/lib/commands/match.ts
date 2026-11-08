import { fold } from "~/lib/search"
import { STOP, SYNONYM_WEIGHT, synonyms } from "./synonyms"

/** Fuzzy matching for the command bar.
 *
 * A query is split into terms. Each term is matched against the words of a row's title and its
 * keywords: equal words count most, then a word starting with the term, then a close typo, then
 * a synonym, then a substring. Title words count more than keywords. One unmatched term is
 * forgiven in a query of three or more, and stop words ("the", "to") are free. A short query
 * can also be a row's initials ("sa" for Switch account). Rows that cover more of their title
 * and read the query as a phrase rank higher. */

export interface Searchable {
  title: string
  words: string
}

/** A row's words, prepared once. */
export interface Prepared {
  title: string[]
  /** Adjacent title words run together, so "darkmode" finds "dark mode". */
  pairs: string[]
  words: string[]
  /** First letters of the title's words, with and without stop words. */
  initials: string[]
  /** The title, folded, for phrase matching. */
  phrase: string
}

/** Lowercase, accentless words. */
export const tokens = (s: string) =>
  fold(s)
    .split(/[^\p{L}\p{N}]+/u)
    .filter(Boolean)

/** Title words that say what a row is about, for coverage. */
const significant = (words: string[]) => words.filter((w) => !STOP.has(w))

export function prepare(title: string, words: string): Prepared {
  // A hint in parentheses ("Filter friends (all, online…)") describes the argument, not the
  // command, so it counts as keywords.
  const hints = [...title.matchAll(/\(([^)]*)\)/g)].map((m) => m[1]).join(" ")
  const t = tokens(title.replace(/\([^)]*\)/g, " "))
  const pairs: string[] = []
  for (let i = 0; i + 1 < t.length; i++) pairs.push(t[i] + t[i + 1])
  return {
    title: t,
    pairs,
    words: tokens(`${words} ${hints}`),
    initials: [
      t.map((w) => w[0]).join(""),
      significant(t)
        .map((w) => w[0])
        .join(""),
    ],
    phrase: t.join(" "),
  }
}

const cache = new WeakMap<Searchable, Prepared>()

/** `prepare` for a row, cached while the row object lives. */
export function prepared(e: Searchable): Prepared {
  let p = cache.get(e)
  if (!p) {
    p = prepare(e.title, e.words)
    cache.set(e, p)
  }
  return p
}

/** Restricted Damerau–Levenshtein distance, or `max + 1` once it is over `max`. */
export function distance(a: string, b: string, max: number): number {
  if (Math.abs(a.length - b.length) > max) return max + 1
  if (a === b) return 0
  let prev2: number[] = []
  let prev = Array.from({ length: b.length + 1 }, (_, j) => j)
  for (let i = 1; i <= a.length; i++) {
    const cur = [i]
    let low = i
    for (let j = 1; j <= b.length; j++) {
      const cost = a[i - 1] === b[j - 1] ? 0 : 1
      let d = Math.min(prev[j] + 1, cur[j - 1] + 1, prev[j - 1] + cost)
      if (i > 1 && j > 1 && a[i - 1] === b[j - 2] && a[i - 2] === b[j - 1]) d = Math.min(d, prev2[j - 2] + 1)
      cur.push(d)
      if (d < low) low = d
    }
    if (low > max) return max + 1
    prev2 = prev
    prev = cur
  }
  return prev[b.length]
}

/** How many typos a term of this length may have. */
const allowed = (n: number) => (n >= 8 ? 2 : n >= 4 ? 1 : 0)

/** Scores for one term against one word. */
export const EXACT = 100
const PREFIX = 60
const PREFIX_SPAN = 30
const TYPO = 70
const TYPO_PREFIX = 50
const TYPO_STEP = 15
const INSIDE = 30

/** How well `term` matches `word`: 0 for not at all. */
export function wordScore(term: string, word: string, typos = true): number {
  if (word === term) return EXACT
  if (word.startsWith(term)) return term.length === 1 ? 40 : PREFIX + (PREFIX_SPAN * term.length) / word.length
  // One key too many on a short word: "inn" for "in", "tabb" for "tab".
  if (typos && word.length >= 2 && term.length === word.length + 1 && term.startsWith(word)) return TYPO - 10
  const max = typos ? allowed(term.length) : 0
  let best = 0
  if (max > 0) {
    const d = distance(term, word, max)
    if (d <= max) best = TYPO - TYPO_STEP * (d - 1)
    else if (word.length > term.length) {
      // Still typing: "swtic" is a typo of the start of "switch".
      for (const n of [term.length, term.length + 1, term.length - 1]) {
        if (n < 3 || n >= word.length) continue
        const dp = distance(term, word.slice(0, n), max)
        if (dp <= max) best = Math.max(best, TYPO_PREFIX - TYPO_STEP * (dp - 1))
      }
    }
  }
  if (!best && term.length >= 3 && word.includes(term)) best = INSIDE
  return best
}

/** Best score of `term` over `words`, and which word it was. */
function best(term: string, words: string[], typos: boolean): { score: number; at: number } {
  let score = 0
  let at = -1
  for (let i = 0; i < words.length; i++) {
    const s = wordScore(term, words[i], typos)
    if (s > score) {
      score = s
      at = i
      if (s === EXACT) break
    }
  }
  return { score, at }
}

/** Keywords count this much of a title match. */
const KEYWORD_WEIGHT = 0.8
/** Run-together title words count this much of a title match. */
const PAIR_WEIGHT = 0.9

/** How well one term matches a row, and which title word it hit (-1 for none). */
export function termScore(p: Prepared, term: string): { score: number; at: number } {
  const direct = (t: string, typos: boolean) => {
    const inTitle = best(t, p.title, typos)
    let score = inTitle.score
    let at = inTitle.at
    const pair = best(t, p.pairs, false).score * PAIR_WEIGHT
    if (pair > score) {
      score = pair
      at = -1
    }
    const kw = best(t, p.words, typos).score * KEYWORD_WEIGHT
    if (kw > score) {
      score = kw
      at = -1
    }
    return { score, at }
  }
  let out = direct(term, true)
  if (out.score >= EXACT) return out
  for (const alt of synonyms(term)) {
    let total = 0
    let at = -1
    for (const w of alt) {
      const r = direct(w, false)
      if (!r.score) {
        total = 0
        break
      }
      total += r.score
      if (at < 0) at = r.at
    }
    const s = (total / alt.length) * SYNONYM_WEIGHT
    if (s > out.score) out = { score: s, at }
  }
  return out
}

export interface Match {
  score: number
  /** Every term that matched hit a title word or an exact keyword, and none were left over. */
  strong: boolean
  /** Lowest score of a single matched term. */
  weakest: number
}

/** Missing a term costs this much. */
const MISSING = 60
/** Covering all of the title's own words adds this much. */
const COVERAGE = 25
const PHRASE = 20
const PHRASE_START = 30
/** A stop word that matched. */
const STOP_MATCH = 5
const INITIALS = 95
const INITIALS_PREFIX = 72

/** Splits a query into folded terms. */
export const terms = (query: string) => tokens(query)

/** How well a prepared row matches `ts`, or null for no match. */
export function matchTerms(p: Prepared, ts: string[], forgive = true): Match | null {
  if (ts.length === 0) return { score: 0, strong: true, weakest: EXACT }
  let total = 0
  let missing = 0
  let counted = 0
  let strong = true
  let weakest = EXACT
  const hit = new Set<number>()
  for (const t of ts) {
    const r = termScore(p, t)
    if (r.score <= 0) {
      if (STOP.has(t)) continue
      missing++
      counted++
      strong = false
      continue
    }
    counted++
    // "to" and "the" say little even when they match.
    total += STOP.has(t) ? STOP_MATCH : r.score
    if (!STOP.has(t)) weakest = Math.min(weakest, r.score)
    if (r.at >= 0) hit.add(r.at)
    // An exact keyword ("inbox" for Messages) is as sure as a title word.
    else if (r.score < EXACT * KEYWORD_WEIGHT) strong = false
  }
  const forgiven = forgive && counted >= 3 ? 1 : 0
  if (missing > forgiven || counted === missing) {
    return ts.length === 1 ? initials(p, ts[0]) : null
  }
  total -= missing * MISSING
  const own = p.title.filter((w) => !STOP.has(w)).length || p.title.length
  const covered = [...hit].filter((i) => !STOP.has(p.title[i])).length
  if (own) total += (COVERAGE * Math.min(covered, own)) / own
  if (ts.filter((t) => !STOP.has(t)).length > 1) {
    const phrase = ts.join(" ")
    if (p.phrase.startsWith(phrase)) total += PHRASE_START
    else if (p.phrase.includes(phrase)) total += PHRASE
  }
  if (ts.length === 1) {
    const i = initials(p, ts[0])
    if (i && i.score > total) return i
  }
  return { score: total, strong, weakest }
}

/** A short query read as the title's initials: "sa" for Switch account. */
function initials(p: Prepared, t: string): Match | null {
  if (t.length < 2 || t.length > 6 || !/^\p{L}+$/u.test(t)) return null
  if (p.initials.some((i) => i === t)) return { score: INITIALS, strong: true, weakest: INITIALS }
  if (p.initials.some((i) => i.length > t.length && i.startsWith(t)))
    return { score: INITIALS_PREFIX, strong: false, weakest: INITIALS_PREFIX }
  return null
}

export interface Ranked<T> {
  entry: T
  score: number
  strong: boolean
}

/** `entries` matching `query`, best first, each with its score. `boost` adds to a row's score
 * (what you picked before). Ties keep the list's order. A blank query keeps every row in order. */
export function rankScored<T extends Searchable>(
  entries: readonly T[],
  query: string,
  boost?: (e: T) => number,
): Ranked<T>[] {
  const ts = terms(query)
  if (ts.length === 0) return entries.map((entry) => ({ entry, score: boost?.(entry) ?? 0, strong: true }))
  const out: (Ranked<T> & { index: number })[] = []
  entries.forEach((entry, index) => {
    const m = matchTerms(prepared(entry), ts)
    if (m) out.push({ entry, score: m.score + (boost?.(entry) ?? 0), strong: m.strong, index })
  })
  out.sort((a, b) => b.score - a.score || a.index - b.index)
  return out.map(({ entry, score, strong }) => ({ entry, score, strong }))
}

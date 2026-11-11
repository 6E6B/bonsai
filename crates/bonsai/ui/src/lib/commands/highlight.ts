import { fold } from "~/lib/search"
import { terms } from "./match"
import { STOP } from "./synonyms"

/** The parts of `title` that `query` matched, to show in bold: words starting with a typed term,
 * a typed term of four letters or more anywhere, or the first letters of words for a query
 * read as initials. Typos and synonyms aren't marked. Ranges are [start, end) in `title`. */
export function highlights(title: string, query: string): [number, number][] {
  const ts = terms(query)
  if (!ts.length || !title) return []
  // Fold each character on its own so positions stay those of the title.
  const chars = Array.from(title)
  const low = chars.map((c) => fold(c).charAt(0) || c)
  const offsets: number[] = []
  let at = 0
  for (const c of chars) {
    offsets.push(at)
    at += c.length
  }
  offsets.push(at)
  const isWord = (i: number) => /[\p{L}\p{N}]/u.test(low[i] ?? "")
  const starts = low.flatMap((_, i) => (isWord(i) && !isWord(i - 1) ? [i] : []))
  const text = low.join("")
  const out: [number, number][] = []
  const mark = (from: number, len: number) => out.push([offsets[from], offsets[from + len]])
  const only = ts.every((t) => STOP.has(t))
  let found = 0
  for (const t of ts) {
    if (STOP.has(t) && !only) continue
    const word = starts.find((i) => text.startsWith(t, i))
    if (word !== undefined) {
      mark(word, t.length)
      found++
      continue
    }
    const inside = t.length >= 4 ? text.indexOf(t) : -1
    if (inside >= 0) {
      mark(inside, t.length)
      found++
    }
  }
  if (!found && ts.length === 1 && ts[0].length >= 2) {
    // Initials: "sa" for Switch account.
    const t = ts[0]
    const words = starts.filter((i) => !STOP.has(text.slice(i).match(/^[\p{L}\p{N}]+/u)?.[0] ?? ""))
    for (const list of [starts, words]) {
      if (list.length >= t.length && list.slice(0, t.length).every((i, k) => low[i] === t[k])) {
        for (const i of list.slice(0, t.length)) mark(i, 1)
        break
      }
    }
  }
  return merge(out)
}

function merge(ranges: [number, number][]): [number, number][] {
  const sorted = [...ranges].sort((a, b) => a[0] - b[0])
  const out: [number, number][] = []
  for (const r of sorted) {
    const last = out[out.length - 1]
    if (last && r[0] <= last[1]) last[1] = Math.max(last[1], r[1])
    else out.push([r[0], r[1]])
  }
  return out
}

/** `title` split into plain and matched parts, for rendering. */
export function highlightParts(title: string, query: string): { text: string; hit: boolean }[] {
  const out: { text: string; hit: boolean }[] = []
  let at = 0
  for (const [a, b] of highlights(title, query)) {
    if (a > at) out.push({ text: title.slice(at, a), hit: false })
    out.push({ text: title.slice(a, b), hit: true })
    at = b
  }
  if (at < title.length) out.push({ text: title.slice(at), hit: false })
  return out
}

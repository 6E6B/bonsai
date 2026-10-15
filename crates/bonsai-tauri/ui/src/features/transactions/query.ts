/** The periods Roblox totals cover, then the ones only the local ledger can answer. */
export const periods = [
  { value: "day", label: "Day" },
  { value: "week", label: "Week" },
  { value: "month", label: "Month" },
  { value: "year", label: "Year" },
  { value: "all", label: "All" },
  { value: "custom", label: "Custom" },
]

/** What the totals cover, for the summary's caption. Null for custom, which names its dates. */
export function periodLabel(p: string): string | null {
  const labels: Record<string, string> = {
    day: "Past day",
    week: "Past week",
    month: "Past month",
    year: "Past year",
    all: "All saved history",
  }
  return labels[p] ?? null
}

/** Roblox has its own totals for these. */
export const sitePeriod = (p: string) => p === "day" || p === "week" || p === "month" || p === "year"

/** The next period for `transactions.period` with no argument. Skips custom, which needs dates. */
export function nextPeriod(p: string): string {
  const cycle = ["day", "week", "month", "year", "all"]
  return cycle[(cycle.indexOf(p) + 1) % cycle.length] ?? "month"
}

/** Local midnight of a `YYYY-MM-DD` date, or null. */
export function dayStart(s: string): number | null {
  const m = /^(\d{4})-(\d{2})-(\d{2})$/.exec(s.trim())
  if (!m) return null
  const d = new Date(Number(m[1]), Number(m[2]) - 1, Number(m[3]))
  if (d.getMonth() !== Number(m[2]) - 1) return null
  return d.getTime()
}

/** `YYYY-MM-DD` in local time. */
export function dayString(ms: number): string {
  const d = new Date(ms)
  const p = (n: number) => String(n).padStart(2, "0")
  return `${d.getFullYear()}-${p(d.getMonth() + 1)}-${p(d.getDate())}`
}

/** `2026-01-01..2026-03-31` (either side may be empty) as ms; `to` is the day after the end
 * date, so the range includes it. Null when it doesn't parse or ends before it starts. */
export function parseRange(s: string): { from: number | null; to: number | null } | null {
  const parts = s.split("..")
  if (parts.length !== 2) return null
  const [a, b] = parts.map((p) => p.trim())
  const from = a ? dayStart(a) : null
  const end = b ? dayStart(b) : null
  if ((a && from === null) || (b && end === null) || (!a && !b)) return null
  const to = end === null ? null : new Date(end).setDate(new Date(end).getDate() + 1)
  if (from !== null && to !== null && to <= from) return null
  return { from, to }
}

export interface ScopeOption {
  id: number
  name: string
}

/** `transactions.scope`'s argument as a scope key: `me`, a community ID, or the first
 * community whose name starts with or contains it. */
export function matchScope(arg: string, communities: readonly ScopeOption[]): string | null {
  const a = arg.trim().toLowerCase()
  if (!a || a === "me" || a === "mine" || a === "you") return "me"
  const byId = communities.find((c) => String(c.id) === a)
  if (byId) return `group:${byId.id}`
  const hit =
    communities.find((c) => c.name.toLowerCase().startsWith(a)) ??
    communities.find((c) => c.name.toLowerCase().includes(a))
  return hit ? `group:${hit.id}` : null
}

/** A type key from `transactions.type`'s argument: a key (`sales`) or label (`Sales`). Empty
 * means all types. */
export function matchKind(arg: string, kinds: readonly { key: string; label: string }[]): string | null {
  const a = arg.trim().toLowerCase().replace(/\s+/g, "_")
  if (!a || a === "all") return ""
  const hit =
    kinds.find((k) => k.key === a || k.label.toLowerCase().replace(/\s+/g, "_") === a) ??
    kinds.find((k) => k.key.startsWith(a))
  return hit ? hit.key : null
}

/** Bar widths in percent of the largest amount, so the biggest bar fills its track. */
export function barWidths(amounts: readonly number[]): number[] {
  const max = Math.max(0, ...amounts.map(Math.abs))
  return amounts.map((a) => (max === 0 ? 0 : (Math.abs(a) / max) * 100))
}

/** `2026-10` → `Oct 2026`. */
export function monthLabel(month: string): string {
  const m = /^(\d{4})-(\d{2})$/.exec(month)
  if (!m) return month
  return new Date(Number(m[1]), Number(m[2]) - 1, 1).toLocaleDateString(undefined, {
    month: "short",
    year: "numeric",
  })
}

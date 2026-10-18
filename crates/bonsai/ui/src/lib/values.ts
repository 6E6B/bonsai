import type { LimitedVersion, ValueDemand, ValueRow, ValueTag, ValueTrend } from "~/lib/ipc/bindings"
import type { IconName } from "~/lib/ui"

/** Named next to every value. Values are a third party's opinion, not Roblox data. */
export const VALUES_SOURCE = "Rolimons"

/** A failed fetch only needs a toast when no cached Rolimons data can be shown. */
export function rolimonsFailed(enabled: boolean, error: string | null, hasData: boolean): boolean {
  return enabled && error !== null && !hasData
}

/** How often a screen showing values asks for fresh ones. The backend refetches at this age. */
export const VALUES_REFRESH_MS = 120_000

const demands: ValueDemand[] = ["terrible", "low", "normal", "high", "amazing"]

const capital = (s: string) => s.charAt(0).toUpperCase() + s.slice(1)

/** Demand as 1 (terrible) to 5 (amazing). */
export function demandLevel(d: ValueDemand): number {
  return demands.indexOf(d) + 1
}

export function demandLabel(d: ValueDemand): string {
  return capital(d)
}

export function trendLabel(t: ValueTrend): string {
  return capital(t)
}

export function trendIcon(t: ValueTrend): IconName {
  switch (t) {
    case "raising":
      return "trend-up"
    case "lowering":
      return "trend-down"
    case "stable":
      return "trend-flat"
    case "unstable":
      return "trend-unstable"
    case "fluctuating":
      return "trend-mixed"
  }
}

/** The text color class for a trend: rising is good news for a holder, falling bad. */
export function trendTone(t: ValueTrend): string {
  switch (t) {
    case "raising":
      return "text-success"
    case "lowering":
      return "text-danger"
    case "unstable":
    case "fluctuating":
      return "text-warning"
    case "stable":
      return "text-fg-muted"
  }
}

export function versionLabel(v: LimitedVersion): string | null {
  return v === "classic" ? "Limited 1.0" : v === "new" ? "Limited 2.0" : null
}

/** What trade math counts: the value, else RAP. Rolimons calls this the default value. */
export function worth(tag: ValueTag | null | undefined, rap: number | null | undefined): number | null {
  return tag?.value ?? rap ?? null
}

/** How far RAP sits from the value, as a whole percent of the value (`+12` means RAP is 12%
 * above). `null` without both. */
export function rapGap(value: number | null, rap: number | null): number | null {
  if (!value || rap === null) return null
  return Math.round(((rap - value) / value) * 100)
}

/** Whether a tag has anything worth drawing. */
export function hasTag(tag: ValueTag | null | undefined): tag is ValueTag {
  return !!tag && (tag.value !== null || tag.demand !== null || tag.trend !== null || tag.projected || tag.rare)
}

// ------------------------------------------------------------------ the values table

export type ValueSort = "worth" | "rap" | "demand" | "gap" | "name"
export type ValueShow = "all" | "valued" | "rare" | "projected" | "unprojected" | "hyped"

export interface ValueFilter {
  query: string
  version: LimitedVersion | "all"
  show: ValueShow
  sort: ValueSort
}

const matches = (r: ValueRow, q: string) =>
  !q || r.name.toLowerCase().includes(q) || r.acronym.toLowerCase() === q || String(r.id) === q

const shows = (r: ValueRow, s: ValueShow) => {
  switch (s) {
    case "all":
      return true
    case "valued":
      return r.tag.value !== null
    case "rare":
      return r.tag.rare
    case "projected":
      return r.tag.projected
    case "unprojected":
      return !r.tag.projected
    case "hyped":
      return r.tag.hyped
  }
}

const sortKey: Record<ValueSort, (r: ValueRow) => number> = {
  worth: (r) => worth(r.tag, r.rap) ?? -1,
  rap: (r) => r.rap ?? -1,
  demand: (r) => (r.tag.demand ? demandLevel(r.tag.demand) : 0),
  // Biggest gap either way first: where RAP and value disagree most.
  gap: (r) => Math.abs(rapGap(r.tag.value, r.rap) ?? -1),
  name: () => 0,
}

/** The rows a values table shows for `f`, in order. Ties go by worth, then name. */
export function shapeValues(rows: readonly ValueRow[], f: ValueFilter): ValueRow[] {
  const q = f.query.trim().toLowerCase()
  const out = rows.filter((r) => (f.version === "all" || r.version === f.version) && shows(r, f.show) && matches(r, q))
  const key = sortKey[f.sort]
  const byWorth = sortKey.worth
  return out.sort((a, b) => {
    if (f.sort === "name") return a.name.localeCompare(b.name)
    return key(b) - key(a) || byWorth(b) - byWorth(a) || a.name.localeCompare(b.name)
  })
}

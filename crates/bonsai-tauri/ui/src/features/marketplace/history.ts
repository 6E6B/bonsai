/** Shaping for the price history chart: daily rows, ranges, averages and a summary. */
import type { Point } from "~/lib/ipc/bindings"

export const DAY = 86_400_000

/** One calendar day (UTC). `price` is the average sale price, `null` when nothing sold. */
export type Day = { t: number; price: number | null; sold: number }
export type PricedDay = Day & { price: number }

export type Range = "7d" | "30d" | "90d" | "1y" | "all"
export const RANGES: { value: Range; label: string; days: number }[] = [
  { value: "7d", label: "7D", days: 7 },
  { value: "30d", label: "30D", days: 30 },
  { value: "90d", label: "90D", days: 90 },
  { value: "1y", label: "1Y", days: 365 },
  { value: "all", label: "All", days: Number.POSITIVE_INFINITY },
]

/** The ranges worth offering for `len` days: those shorter than the history, then All. */
export const rangesFor = (len: number) => RANGES.filter((r) => r.days < len || r.value === "all")

const startOfDay = (t: number) => Math.floor(t / DAY) * DAY

/** Joins price and volume by day, oldest first, with a row for every day in between so days
 * with no sales show as gaps instead of being skipped. */
export function toDays(prices: readonly Point[], volume: readonly Point[]): Day[] {
  const byDay = new Map<number, Day>()
  const at = (t: number) => {
    const d = startOfDay(t)
    let row = byDay.get(d)
    if (!row) {
      row = { t: d, price: null, sold: 0 }
      byDay.set(d, row)
    }
    return row
  }
  // f64s cross IPC as nullable numbers.
  for (const p of prices) if (p.t !== null && p.v !== null && p.v > 0) at(p.t).price = p.v
  for (const p of volume) if (p.t !== null && p.v !== null) at(p.t).sold = p.v
  if (byDay.size === 0) return []
  const keys = [...byDay.keys()]
  const first = Math.min(...keys)
  const last = Math.max(...keys)
  const out: Day[] = []
  for (let t = first; t <= last; t += DAY) out.push(byDay.get(t) ?? { t, price: null, sold: 0 })
  // A day can report copies sold with no price; there's nothing to plot for it.
  for (const d of out) if (d.sold === 0) d.price = null
  return out
}

/** The visible part of the chart, in day indices: `from` inclusive, `to` exclusive. Fractional
 * while zooming and panning, so the chart moves smoothly. */
export type Window = { from: number; to: number }

/** The fewest days the chart zooms in to. */
export const MIN_SPAN = 5

const clampWindow = (from: number, span: number, len: number): Window => {
  const s = Math.min(len, Math.max(Math.min(MIN_SPAN, len), span))
  const f = Math.min(len - s, Math.max(0, from))
  return { from: f, to: f + s }
}

/** The last days of a `len`-day history that `range` covers. */
export function presetWindow(len: number, range: Range): Window {
  const days = RANGES.find((r) => r.value === range)?.days ?? len
  return clampWindow(len - days, days, len)
}

/** The preset `w` shows exactly, if any. */
export function matchRange(w: Window, len: number): Range | null {
  for (const r of rangesFor(len)) {
    const p = presetWindow(len, r.value)
    if (Math.abs(p.from - w.from) < 0.5 && Math.abs(p.to - w.to) < 0.5) return r.value
  }
  return null
}

/** Scales the span by `factor` (under 1 zooms in), keeping the day at `anchor` where it is. */
export function zoomWindow(w: Window, factor: number, anchor: number, len: number): Window {
  const span = w.to - w.from
  const next = Math.min(len, Math.max(Math.min(MIN_SPAN, len), span * factor))
  return clampWindow(anchor - ((anchor - w.from) * next) / span, next, len)
}

/** Moves the window by `by` days, stopping at either end. */
export const panWindow = (w: Window, by: number, len: number): Window => clampWindow(w.from + by, w.to - w.from, len)

/** The whole days `w` shows, as slice bounds. */
export const visibleSlice = (w: Window): [number, number] => [Math.round(w.from), Math.round(w.to)]

/** Trailing `n`-day average price weighted by copies sold, per day; `null` until a sale. Taken
 * over all of `days` so the first days of a range still average their earlier week. */
export function movingAverage(days: readonly Day[], n = 7): (number | null)[] {
  return days.map((_, i) => {
    let robux = 0
    let copies = 0
    for (let j = Math.max(0, i - n + 1); j <= i; j++) {
      const d = days[j]
      if (d.price === null) continue
      robux += d.price * d.sold
      copies += d.sold
    }
    return copies > 0 ? robux / copies : null
  })
}

export type Summary = {
  /** Copies sold. */
  sold: number
  /** Days in the range and how many had a sale. */
  days: number
  activeDays: number
  /** Average price over every sale in the range. */
  average: number | null
  high: PricedDay | null
  low: PricedDay | null
  /** First and last day with a sale. */
  first: PricedDay | null
  last: PricedDay | null
  /** Last over first price, as a fraction: 0.12 is +12%. */
  change: number | null
}

export function summarize(days: readonly Day[]): Summary {
  const priced = days.filter((d): d is PricedDay => d.price !== null)
  const sold = days.reduce((n, d) => n + d.sold, 0)
  const pricedSold = priced.reduce((n, d) => n + d.sold, 0)
  const robux = priced.reduce((n, d) => n + d.price * d.sold, 0)
  const first = priced[0] ?? null
  const last = priced[priced.length - 1] ?? null
  return {
    sold,
    days: days.length,
    activeDays: priced.length,
    average: pricedSold > 0 ? robux / pricedSold : null,
    high: priced.reduce<PricedDay | null>((m, d) => (m === null || d.price > m.price ? d : m), null),
    low: priced.reduce<PricedDay | null>((m, d) => (m === null || d.price < m.price ? d : m), null),
    first,
    last,
    change: first && last && first !== last ? last.price / first.price - 1 : null,
  }
}

/** `a` over `b` as a whole percent change, or `null` when either is missing. */
export function pctOver(a: number | null, b: number | null): number | null {
  if (a === null || !b) return null
  return Math.round((a / b - 1) * 100)
}

/** Round axis values covering lo..hi: steps of 1, 2 or 5 times a power of ten. */
export function niceTicks(lo: number, hi: number, count = 4): number[] {
  if (!(hi > lo)) return [lo]
  const raw = (hi - lo) / count
  const mag = 10 ** Math.floor(Math.log10(raw))
  const step = ([1, 2, 5, 10].find((m) => m * mag >= raw) ?? 10) * mag
  const out: number[] = []
  for (let v = Math.ceil(lo / step) * step; v <= hi + step * 1e-9; v += step) out.push(Number(v.toPrecision(12)))
  return out
}

/** Axis values for a log scale over lo..hi: 1, 2 and 5 times powers of ten, thinned to powers
 * of ten when there are too many, and plain ticks when the range is too narrow for either. */
export function logTicks(lo: number, hi: number, count = 5): number[] {
  if (!(hi > lo) || lo <= 0) return niceTicks(lo, hi, count)
  const out: number[] = []
  for (let e = Math.floor(Math.log10(lo)); e <= Math.ceil(Math.log10(hi)); e++)
    for (const m of [1, 2, 5]) {
      const v = Number((m * 10 ** e).toPrecision(12))
      if (v >= lo && v <= hi) out.push(v)
    }
  if (out.length > count + 2) return out.filter((v) => Number.isInteger(Math.log10(v)))
  if (out.length < 2) return niceTicks(lo, hi, count)
  return out
}

/** Converts a pointer's clientX to the element's own CSS pixels. Under the app's CSS zoom,
 * WebKitGTK reports clientX and the element's rect in zoomed pixels while layout stays in
 * unzoomed ones, so offsets have to be scaled back by the element's laid-out width. */
export function localX(clientX: number, rect: { left: number; width: number }, layoutWidth: number): number {
  const scale = rect.width > 0 ? layoutWidth / rect.width : 1
  return (clientX - rect.left) * scale
}

export type Reference = { key: string; label: string; value: number }

/** The y range for prices and the reference lines that fit it. A reference far outside the
 * prices would flatten the line, so it's left off the plot and listed as off-chart instead. */
export function priceDomain(
  values: readonly number[],
  refs: readonly Reference[],
  log = false,
): { lo: number; hi: number; shown: Reference[]; off: Reference[] } {
  if (values.length === 0) return { lo: 0, hi: 1, shown: [], off: [...refs] }
  if (log) return logDomain(values, refs)
  let lo = Math.min(...values)
  let hi = Math.max(...values)
  const span = Math.max(hi - lo, hi * 0.05, 1)
  const shown: Reference[] = []
  const off: Reference[] = []
  for (const r of refs) {
    if (r.value >= lo - span && r.value <= hi + span) shown.push(r)
    else off.push(r)
  }
  for (const r of shown) {
    lo = Math.min(lo, r.value)
    hi = Math.max(hi, r.value)
  }
  const pad = Math.max(hi - lo, hi * 0.05, 1) * 0.08
  return { lo: Math.max(0, lo - pad), hi: hi + pad, shown, off }
}

/** priceDomain on a log scale: references within 4x of the prices are kept, and the padding
 * is a share of the log span so the line keeps the same room at any price level. */
function logDomain(values: readonly number[], refs: readonly Reference[]) {
  const pos = values.filter((v) => v > 0)
  let lo = Math.min(...pos)
  let hi = Math.max(...pos)
  const shown: Reference[] = []
  const off: Reference[] = []
  for (const r of refs) {
    if (r.value >= lo / 4 && r.value <= hi * 4) shown.push(r)
    else off.push(r)
  }
  for (const r of shown) {
    lo = Math.min(lo, r.value)
    hi = Math.max(hi, r.value)
  }
  const pad = Math.max(Math.log10(hi / lo), 0.05) * 0.06
  return { lo: lo / 10 ** pad, hi: hi * 10 ** pad, shown, off }
}

/** Spreads labels at `ys` (pixels, any order) at least `gap` apart, keeping them near their
 * line and inside min..max. Returns the new positions in the same order. */
export function spread(ys: readonly number[], gap: number, min: number, max: number): number[] {
  const order = ys.map((y, i) => ({ y, i })).sort((a, b) => a.y - b.y)
  for (let k = 1; k < order.length; k++) order[k].y = Math.max(order[k].y, order[k - 1].y + gap)
  const over = order.length ? order[order.length - 1].y - max : 0
  if (over > 0) for (const o of order) o.y -= over
  for (let k = 0; k < order.length; k++) order[k].y = Math.max(order[k].y, k === 0 ? min : order[k - 1].y + gap)
  const out = new Array<number>(ys.length)
  for (const o of order) out[o.i] = o.y
  return out
}

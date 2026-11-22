import type { PlaytimeWeek } from "~/lib/ipc/bindings"

/** Bar lengths in percent of the largest value. */
export function barWidths(values: readonly number[]): number[] {
  const max = Math.max(0, ...values)
  return values.map((v) => (max <= 0 ? 0 : (Math.max(0, v) / max) * 100))
}

/** Seconds per week over the range. The current week counts as a whole one. */
export function weeklyAverage(weeks: readonly PlaytimeWeek[]): number {
  if (weeks.length === 0) return 0
  return weeks.reduce((n, w) => n + w.secs, 0) / weeks.length
}

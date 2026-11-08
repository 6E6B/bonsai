import { date } from "~/lib/format"
import { rank } from "~/lib/search"

const sameDay = (a: Date, b: Date) =>
  a.getFullYear() === b.getFullYear() && a.getMonth() === b.getMonth() && a.getDate() === b.getDate()

/** The heading of a day in a chat: "Today", "Yesterday", then the full date. Days are the
 * reader's local ones. `now` is for tests. */
export function dayLabel(ms: number, now = Date.now()): string {
  const day = new Date(ms)
  const today = new Date(now)
  if (sameDay(day, today)) return "Today"
  const yesterday = new Date(today)
  yesterday.setDate(today.getDate() - 1)
  return sameDay(day, yesterday) ? "Yesterday" : date(ms)
}

/** A line at `ms` opens a new day when the line before it (if any) is from another local day.
 * Lines without a usable time never do. */
export function startsDay(prev: number | null, ms: number | null): boolean {
  if (ms === null) return false
  return prev === null || !sameDay(new Date(prev), new Date(ms))
}

/** Conversations whose name contains every word of `query`, best match first. All of them,
 * in order, for a blank query. */
export function filterConversations<T extends { title: string }>(items: readonly T[], query: string): T[] {
  return rank(items, query, (c) => [c.title])
}

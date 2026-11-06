/** Which notifications the list shows: all or unread only, optionally one category. */
export interface NotificationView {
  view: string
  category: string
}

export function shownNotifications<T extends { read: boolean; category: string }>(items: T[], v: NotificationView) {
  return items.filter((n) => (v.view !== "unread" || !n.read) && (v.category === "" || n.category === v.category))
}

/** The category a `notifications.category` argument asks for: a known id, or "" for all. */
export function categoryArg(arg: string | null | undefined, known: string[]) {
  const a = (arg ?? "").trim().toLowerCase()
  return known.includes(a) ? a : ""
}

/** The command Enter runs on a notification: its web page, the person it's about, or for the
 * app's own entries (ids starting `app:`) the screen they're about. */
export function openCommand(n: {
  id: string
  link: string | null
  userId: number | null
  category: string
}): [string, string?] | null {
  if (n.link) return ["entity.open_website", n.link]
  if (n.userId) return ["user.open", String(n.userId)]
  if (!n.id.startsWith("app:")) return null
  if (n.category === "trades") return ["nav.trades"]
  if (n.category === "account") return ["nav.transactions"]
  return null
}

/** A row of the grouped list: a day heading, or notification `index` of the filtered list. */
export type DayRow<T> = { heading: string; item?: undefined } | { heading?: undefined; item: T; index: number }

/** Which group a time falls in, relative to `now` in local time. */
export function dayLabel(when: number, now: number) {
  const start = new Date(now)
  start.setHours(0, 0, 0, 0)
  const today = start.getTime()
  const day = 86_400_000
  if (when >= today) return "Today"
  if (when >= today - day) return "Yesterday"
  if (when >= today - 6 * day) return "This week"
  return "Older"
}

/** The list with a heading before each day group, newest first as given. `positions[i]` is
 * where notification `i` sits among the rows, for the keyboard cursor. */
export function groupByDay<T>(items: readonly T[], time: (item: T) => number, now: number) {
  const rows: DayRow<T>[] = []
  const positions: number[] = []
  let last = ""
  items.forEach((item, index) => {
    const label = dayLabel(time(item), now)
    if (label !== last) {
      rows.push({ heading: label })
      last = label
    }
    positions.push(rows.length)
    rows.push({ item, index })
  })
  return { rows, positions }
}

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

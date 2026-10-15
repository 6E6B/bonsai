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

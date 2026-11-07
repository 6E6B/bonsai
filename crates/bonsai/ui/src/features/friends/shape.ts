import type { Presence } from "~/lib/ipc/bindings"
import { matches, rank } from "~/lib/search"

export type Section = "friends" | "requests" | "followers" | "following" | "blocked"

/** What the Friends page lists in every section. */
export interface Row {
  id: number
  name: string
  displayName: string
  presence: Presence
  /** Experience name when in game. */
  place?: string
  placeId?: number
  /** Second line for people lists: mutual friends. */
  note?: string
  /** What the active account calls them. Shown as the title, and matched by the filter. */
  nickname?: string
  /** Whether the active account follows them, on Followers. */
  following?: boolean
  /** How many mutual friends, on Requests. */
  mutuals?: number
  /** When the request was sent (RFC 3339), on Requests. */
  sentAt?: string
}

type Option = { value: string; label: string }

/** Sorts offered in `section`. The first is the default and keeps the backend's order (status
 * then name for friends, newest first for requests). */
export function sortOptions(section: Section): Option[] {
  const name = { value: "name", label: "Name" }
  if (section === "friends") return [{ value: "status", label: "Status" }, name]
  if (section === "requests")
    return [
      { value: "newest", label: "Newest" },
      { value: "oldest", label: "Oldest" },
      { value: "mutuals", label: "Most mutual friends" },
      name,
    ]
  return [{ value: "default", label: "Default order" }, name]
}

/** Filters offered in `section`, or none. The first keeps everyone. */
export function showOptions(section: Section): Option[] {
  if (section === "friends")
    return [
      { value: "all", label: "Everyone" },
      { value: "online", label: "Online" },
      { value: "game", label: "In game" },
      { value: "studio", label: "In Studio" },
      { value: "offline", label: "Offline" },
    ]
  if (section === "followers")
    return [
      { value: "all", label: "Everyone" },
      { value: "not_followed", label: "Not followed back" },
    ]
  return []
}

/** `value` if `options` has it, else the first option's. */
export const pick = (options: Option[], value: string) =>
  options.some((o) => o.value === value) ? value : (options[0]?.value ?? "")

const shows = (r: Row, show: string) => {
  switch (show) {
    case "online":
      return r.presence !== "offline"
    case "game":
    case "studio":
    case "offline":
      return r.presence === show
    case "not_followed":
      return r.following === false
    default:
      return true
  }
}

const title = (r: Row) => r.nickname || r.displayName || r.name
const byName = (a: Row, b: Row) => title(a).localeCompare(title(b), undefined, { sensitivity: "base" })
const sent = (r: Row) => (r.sentAt ? Date.parse(r.sentAt) || 0 : 0)

const compare: Record<string, (a: Row, b: Row) => number> = {
  name: byName,
  newest: (a, b) => sent(b) - sent(a),
  oldest: (a, b) => sent(a) - sent(b),
  mutuals: (a, b) => (b.mutuals ?? 0) - (a.mutuals ?? 0) || byName(a, b),
}

const fields = (r: Row) => [r.nickname, r.displayName, r.name]

/** One section's rows after search, filter and sort. Under the section's default sort a search
 * puts the best matches first; any other sort wins over how well someone matches. */
export function shapePeople(rows: readonly Row[], section: Section, o: { query: string; sort: string; show: string }) {
  const show = pick(showOptions(section), o.show)
  const sort = pick(sortOptions(section), o.sort)
  const kept = rows.filter((r) => shows(r, show))
  const by = sort === sortOptions(section)[0].value && sort !== "newest" ? undefined : compare[sort]
  if (!by) return rank(kept, o.query, fields)
  return kept.filter((r) => matches(fields(r), o.query)).sort(by)
}

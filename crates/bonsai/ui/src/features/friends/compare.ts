import type { Account, OverlapPerson, SharedFriendsView } from "~/lib/ipc/bindings"
import { shownAccountLabel } from "~/lib/streamer"

/** An account's name as the rail shows it, masked in streamer mode. */
export function accountName(a: Account): string {
  return shownAccountLabel(a.id, a.label ?? a.displayName ?? a.name)
}

/** One line about two accounts' shared friends. */
export function compareSummary(view: SharedFriendsView, a: string, b: string): string {
  if (view.loading) return "Loading friend lists…"
  const n = view.friends.length
  const shared = `${n} shared ${n === 1 ? "friend" : "friends"}. ${a} has ${view.aCount}, ${b} has ${view.bCount}.`
  return view.linked ? `${shared} They are friends with each other.` : shared
}

/** People whose display name or username contains `query`, ignoring case. */
export function filterPeople(people: readonly OverlapPerson[], query: string): OverlapPerson[] {
  const q = query.trim().toLowerCase()
  if (!q) return [...people]
  return people.filter((p) => p.name.toLowerCase().includes(q) || p.displayName.toLowerCase().includes(q))
}

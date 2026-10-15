import type { Presence } from "~/lib/ui"

export type PresenceRow<T> = { kind: "header"; label: string; count: number } | { kind: "user"; user: T }

const groups: [Presence, string][] = [
  ["game", "In game"],
  ["studio", "In Studio"],
  ["online", "Online"],
  ["offline", "Offline"],
]

/** Flattens users into header and user rows grouped by presence, for one virtual list. Empty
 * groups are left out; order inside a group is kept. */
export function groupByPresence<T extends { presence: Presence }>(users: readonly T[]): PresenceRow<T>[] {
  const rows: PresenceRow<T>[] = []
  for (const [presence, label] of groups) {
    const members = users.filter((u) => u.presence === presence)
    if (members.length === 0) continue
    rows.push({ kind: "header", label, count: members.length })
    for (const user of members) rows.push({ kind: "user", user })
  }
  return rows
}

/** Users in one experience. `placeId` is null when their presence hides where they are. */
export interface GameGroup<T> {
  key: string
  name: string
  placeId: number | null
  universeId: number | null
  users: T[]
}

/** Groups in-game users by experience, biggest group first, then by name. Users whose
 * experience is hidden are grouped by their status text. Order inside a group is kept. */
export function groupByGame<T extends { status: string; placeId: number | null; universeId: number | null }>(
  users: readonly T[],
): GameGroup<T>[] {
  const groups = new Map<string, GameGroup<T>>()
  for (const u of users) {
    const key = u.universeId ? `u${u.universeId}` : u.placeId ? `p${u.placeId}` : `s${u.status}`
    let g = groups.get(key)
    if (!g) {
      g = { key, name: u.status, placeId: u.placeId, universeId: u.universeId, users: [] }
      groups.set(key, g)
    }
    g.users.push(u)
  }
  return [...groups.values()].sort((a, b) => b.users.length - a.users.length || a.name.localeCompare(b.name))
}

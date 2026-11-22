import type { OverlapPerson, OverlapView } from "~/lib/ipc/bindings"

/** How many of an account's mutual friends to name on the card before "+N". */
export const SHOWN = 4

/** A person's display name, falling back to the username, then the id. */
export function personName(p: OverlapPerson): string {
  return p.displayName || p.name || `User ${p.id}`
}

/** Where the overlap comes from, or why it can't show yet. Empty while nothing is known. */
export function overlapNote(view: OverlapView): string {
  if (view.error) return view.error
  if (view.complete) return `From all ${view.checked} of their friends.`
  if (view.checked) return `From the first ${view.checked} of their friends.`
  return view.loading ? "Loading their friends…" : ""
}

/** "Friends, 3 mutual", "2 mutual", "Friends". */
export function overlapLine(friend: boolean, mutuals: number): string {
  const m = mutuals ? `${mutuals} mutual` : ""
  if (friend) return m ? `Friends, ${m}` : "Friends"
  return m
}

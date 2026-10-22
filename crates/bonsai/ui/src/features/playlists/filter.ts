import type { GameCard } from "~/lib/ipc/bindings"

/** Games whose name or creator contains every word of `query`, ignoring case. */
export function searchGames(games: GameCard[], query: string): GameCard[] {
  const words = query.toLowerCase().split(/\s+/).filter(Boolean)
  if (words.length === 0) return games
  return games.filter((g) => {
    const text = `${g.name} ${g.creator}`.toLowerCase()
    return words.every((w) => text.includes(w))
  })
}

import type { GameCard } from "~/lib/ipc/bindings"
import { rank } from "~/lib/search"

/** Games whose name or creator contains every word of `query`, best match first. */
export function searchGames(games: GameCard[], query: string): GameCard[] {
  return rank(games, query, (g) => [g.name, g.creator])
}

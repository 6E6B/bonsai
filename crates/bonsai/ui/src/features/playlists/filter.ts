import type { GameCard } from "~/lib/ipc/bindings"
import { matches, rank } from "~/lib/search"

export type PlaylistSort = "added" | "name" | "playing" | "rating" | "creator"

export const SORTS: { value: PlaylistSort; label: string }[] = [
  { value: "added", label: "Playlist order" },
  { value: "name", label: "Name" },
  { value: "playing", label: "Most playing" },
  { value: "rating", label: "Highest rated" },
  { value: "creator", label: "Creator" },
]

export const isSort = (v: string): v is PlaylistSort => SORTS.some((s) => s.value === v)

/** Value of the genre filter that keeps every game. */
export const ALL_GENRES = "all"

/** The genre filter's options: every genre in `games`, by name, after "All genres". */
export function genreOptions(games: readonly GameCard[]): { value: string; label: string }[] {
  const genres = [...new Set(games.map((g) => g.genre).filter(Boolean))].sort((a, b) => a.localeCompare(b))
  return [{ value: ALL_GENRES, label: "All genres" }, ...genres.map((g) => ({ value: g, label: g }))]
}

const byName = (a: GameCard, b: GameCard) => a.name.localeCompare(b.name, undefined, { sensitivity: "base" })

const compare: Record<Exclude<PlaylistSort, "added">, (a: GameCard, b: GameCard) => number> = {
  name: byName,
  playing: (a, b) => b.playing - a.playing || byName(a, b),
  // Unrated games go last.
  rating: (a, b) => (b.rating ?? -1) - (a.rating ?? -1) || byName(a, b),
  creator: (a, b) => a.creator.localeCompare(b.creator, undefined, { sensitivity: "base" }) || byName(a, b),
}

/** A playlist's games after search, genre filter and sort. Under the default playlist order
 * a search puts the best matches first; any other sort wins over how well a game matches. */
export function shapeGames(games: readonly GameCard[], o: { query: string; sort: string; genre: string }): GameCard[] {
  const kept = o.genre === ALL_GENRES ? games : games.filter((g) => g.genre === o.genre)
  if (!isSort(o.sort) || o.sort === "added") return rank(kept, o.query, (g) => [g.name, g.creator])
  return kept.filter((g) => matches([g.name, g.creator], o.query)).sort(compare[o.sort])
}

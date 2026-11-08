import { describe, expect, it } from "vitest"
import type { GameCard } from "~/lib/ipc/bindings"
import { ALL_GENRES, genreOptions, shapeGames } from "./filter"

const game = (placeId: number, name: string, more: Partial<GameCard> = {}): GameCard => ({
  universeId: placeId,
  placeId,
  name,
  creator: "",
  playing: 0,
  rating: null,
  sponsored: false,
  genre: "",
  ...more,
})

const ids = (l: GameCard[]) => l.map((g) => g.placeId)
const opts = { query: "", sort: "added", genre: ALL_GENRES }

describe("shapeGames", () => {
  const games = [
    game(1, "Tower of Hell", { creator: "YXCeptional", playing: 20, rating: 80, genre: "Obby" }),
    game(2, "Natural Disaster Survival", { creator: "Stickmasterluke", playing: 50, genre: "Survival" }),
    game(3, "Obby Tower", { creator: "Someone", playing: 5, rating: 95, genre: "Obby" }),
  ]

  it("keeps playlist order for a blank query", () => {
    expect(ids(shapeGames(games, opts))).toEqual([1, 2, 3])
  })

  it("matches every word in the name or creator, best match first", () => {
    expect(ids(shapeGames(games, { ...opts, query: "tower" }))).toEqual([1, 3])
    expect(ids(shapeGames(games, { ...opts, query: "disaster LUKE" }))).toEqual([2])
    expect(shapeGames(games, { ...opts, query: "tower luke" })).toEqual([])
  })

  it("sorts", () => {
    expect(ids(shapeGames(games, { ...opts, sort: "name" }))).toEqual([2, 3, 1])
    expect(ids(shapeGames(games, { ...opts, sort: "playing" }))).toEqual([2, 1, 3])
    expect(ids(shapeGames(games, { ...opts, sort: "rating" }))).toEqual([3, 1, 2])
    expect(ids(shapeGames(games, { ...opts, sort: "creator" }))).toEqual([3, 2, 1])
  })

  it("keeps a picked sort while searching", () => {
    expect(ids(shapeGames(games, { ...opts, query: "tower", sort: "rating" }))).toEqual([3, 1])
  })

  it("filters by genre", () => {
    expect(ids(shapeGames(games, { ...opts, genre: "Obby" }))).toEqual([1, 3])
  })
})

describe("genreOptions", () => {
  it("lists each genre once, by name, after All genres", () => {
    const games = [
      game(1, "a", { genre: "Survival" }),
      game(2, "b", { genre: "Obby" }),
      game(3, "c", { genre: "Obby" }),
    ]
    expect(genreOptions([...games, game(4, "d")]).map((o) => o.value)).toEqual([ALL_GENRES, "Obby", "Survival"])
  })
})

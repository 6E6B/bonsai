import { describe, expect, it } from "vitest"
import type { GameCard } from "~/lib/ipc/bindings"
import { searchGames } from "./filter"

const game = (placeId: number, name: string, creator = ""): GameCard => ({
  universeId: placeId,
  placeId,
  name,
  creator,
  playing: 0,
  rating: null,
  sponsored: false,
  genre: "",
})

describe("searchGames", () => {
  const games = [game(1, "Tower of Hell", "YXCeptional"), game(2, "Natural Disaster Survival", "Stickmasterluke")]

  it("keeps everything for an empty query", () => {
    expect(searchGames(games, "  ")).toEqual(games)
  })

  it("matches every word in the name or creator, ignoring case", () => {
    expect(searchGames(games, "tower").map((g) => g.placeId)).toEqual([1])
    expect(searchGames(games, "disaster LUKE").map((g) => g.placeId)).toEqual([2])
    expect(searchGames(games, "tower luke")).toEqual([])
  })
})

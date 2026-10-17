import type { Friends, GameCard, Home, Shell } from "~/lib/ipc/bindings"
import type { Presence } from "~/lib/ui"

/**
 * Made-up data for `bun run dev` in a browser, where there is no backend: the shell, friends
 * and home feed, so the layout can be worked on. Every name here is made up. Screens in the
 * app read real data over IPC.
 */

const DAY = 86_400_000
const NOW = Date.UTC(2026, 9, 14, 12)

export interface Friend {
  id: number
  name: string
  displayName: string
  presence: Presence
  /** Experience name when in game, or last online. */
  place?: string
  placeId?: number
}

export const friends: Friend[] = [
  { id: 101, name: "pixelfox", displayName: "Pixel Fox", presence: "game", place: "Burger Shift", placeId: 9001 },
  { id: 102, name: "mossy_bricks", displayName: "Mossy", presence: "game", place: "Burger Shift", placeId: 9001 },
  { id: 103, name: "grayline", displayName: "gray", presence: "online" },
  { id: 104, name: "lumen_dev", displayName: "Lumen", presence: "studio", place: "Untitled Place" },
  ...[
    "nightcrate",
    "driftwood",
    "glosscat",
    "quietstorm",
    "hillside",
    "lagtrap",
    "eli_always",
    "punpun",
    "seraph_on",
    "sleeze",
    "cullen_x",
  ].map((name, i): Friend => ({ id: 110 + i, name, displayName: name.replace(/_/g, " "), presence: "offline" })),
]

export interface Game {
  id: number
  name: string
  creator: string
  creatorId: number
  playing: number
  visits: number
  favorites: number
  likes: number
  dislikes: number
  maxPlayers: number
  genre: string
  maturity: string
  created: number
  updated: number
  description: string
  favorited: boolean
  liked: boolean | null
  notify: boolean
}

const game = (id: number, name: string, creator: string, playing: number): Game => ({
  id,
  name,
  creator,
  creatorId: id + 50_000,
  playing,
  visits: playing * 7_100,
  favorites: playing * 117,
  likes: Math.round(playing * 21.2),
  dislikes: Math.round(playing * 2.4),
  maxPlayers: 50,
  genre: "All",
  maturity: "Minimal",
  created: NOW - 70 * DAY,
  updated: NOW - 1 * DAY,
  description:
    "A logic puzzle about placing cats on a grid. Every puzzle has one solution and can be solved without guessing.\n\nHow to play: one cat per color, one cat per row and column, and two cats can't touch.",
  favorited: false,
  liked: null,
  notify: false,
})

export const games: Game[] = [
  game(9000, "Tile Cats", "Tilework", 1037),
  game(9001, "Burger Shift", "Grill Studio", 48_211),
  game(9002, "Obby Tower 9", "Stairwell", 12_480),
  game(9003, "Night Ferry", "Harbor Lights", 3_204),
  game(9004, "Block Kart", "Pit Lane", 22_901),
  game(9005, "Slime Farm", "Goo Co", 640),
  game(9006, "Sky Islands", "Driftworks", 8_812),
  game(9007, "Ghost Mall", "Spooky Shelf", 15_002),
  game(9008, "Fishing Pier", "Quiet Tide", 2_337),
  game(9009, "Rail Tycoon", "Steamline", 5_120),
  game(9010, "Paint Party", "Brushhouse", 980),
  game(9011, "Laser Tag Arena", "Neon Grid", 31_440),
]

/** Shell state for `bun run dev` in a browser, where there is no backend. */
export const browserShell: Shell = {
  accounts: [
    { id: 1, name: "tilework", displayName: "Tilework", label: "Main", color: "blue", session: "ok", unread: 3 },
    { id: 2, name: "tilework_alt", displayName: "Alt", label: null, color: "green", session: "ok", unread: 0 },
  ],
  active: 1,
  offline: false,
  jobs: 0,
  toasts: [],
  keychainWarning: null,
  robux: 22,
  unread: { friendRequests: 2, messages: 1, trades: 1, notifications: 3, chat: 4 },
  login: null,
  challenge: null,
  themeImport: null,
  pluginInstall: null,
  pluginActions: null,
  pluginOpen: null,
  pluginRev: 0,
  security: null,
  approve: null,
  itemValues: false,
  avatar3d: false,
  uiScale: 1,
  look: {
    density: "comfortable",
    formatLocale: "",
    motion: "system",
    timeFormat: "auto",
    relativeDates: true,
    statusBar: true,
    startScreen: "home",
    sidebarMode: "auto",
    drawer: false,
    homeFriends: true,
    homeContinue: true,
    homeFavorites: true,
    homeRecommended: true,
    accountAccent: false,
  },
  sidebar: { order: [], hidden: ["messages"] },
}

/** A sample game as the tile view the backend sends. Universe 0 so no thumbnail is fetched
 * for a made-up id. */
export const card = (g: Game): GameCard => ({
  universeId: 0,
  placeId: g.id,
  name: g.name,
  creator: g.creator,
  playing: g.playing,
  rating: Math.round((g.likes * 100) / Math.max(1, g.likes + g.dislikes)),
  sponsored: false,
  genre: g.genre,
})

export const browserFriends: Friends = {
  loading: false,
  error: null,
  friends: friends.map((f) => ({
    id: f.id,
    name: f.name,
    displayName: f.displayName,
    presence: f.presence,
    status: f.place ?? (f.presence === "offline" ? "Offline" : "Online"),
    placeId: f.placeId ?? null,
    nickname: null,
    universeId: null,
  })),
}

export const browserHome: Home = {
  loading: false,
  error: null,
  recent: games.slice(0, 4).map(card),
  favorites: games.slice(2, 6).map(card),
  sorts: [{ id: "top", title: "Recommended for you", subtitle: "", games: games.map(card) }],
}

// This module replaces bindings only in the explicit performance build. No native IPC or credentials.
import type { commands as realCommands } from "../src/lib/ipc/bindings"
import {
  badges,
  chatLines,
  conversations,
  games,
  groups,
  items,
  list,
  look,
  messages,
  notifications,
  outfits,
  people,
  profile,
  themes,
  trades,
} from "./fixtures"

export * from "../src/lib/ipc/bindings"

type Fixtures = {
  [K in keyof typeof realCommands]?: (
    ...args: Parameters<(typeof realCommands)[K]>
  ) => Awaited<ReturnType<(typeof realCommands)[K]>>
}

const fixtures = {
  appearance: () => ({ mode: "dark", dark: "", light: "", fonts: [], background: null }),
  avatar3d: () => ({
    loading: false,
    error: null,
    rig: "",
    assets: [],
    emotes: [],
    colors: [],
    scales: { height: null, width: null, head: null, depth: null, proportion: null, bodyType: null },
    rev: null,
  }),
  profile: () => profile,
  profileCreations: () => list(games),
  profileFavorites: () => list(games),
  profileFollowedGames: () => list(games),
  profileOutfits: () => list(outfits),
  profileBadges: () => list(badges),
  profileGroups: () => list(groups),
  profileCollectibles: () => list(items),
  profileTrading: () => ({ enabled: false, loading: false, error: null, stats: null }),
  profilePeople: () => list(people),
  profileLook: () => look,
  outfitLook: () => look,
  people: () => list(people.map((p) => ({ ...p, mutuals: [], sentAt: null }))),
  catalogCategories: () => list([{ name: "All", taxonomy: "all", subcategories: [] }]),
  catalogOptions: () => ({
    sorts: [{ value: "relevance", label: "Relevance" }],
    sales: [{ value: "all", label: "All" }],
  }),
  catalogSearch: () => ({ list: list(items), moreKey: "" }),
  wishlist: () => list(items),
  // Disabled optional enrichment. Keep the regular profile tab availability logic.
  wikiAbout: () => ({
    enabled: false,
    loading: false,
    error: null,
    article: null,
    fetchedAt: null,
    source: "Roblox Wiki",
    license: "CC BY-SA 3.0",
  }),
  saveTabs: () => null,
  chatList: () => ({ list: list(conversations), moreKey: "" }),
  takeChatFocus: () => "chat-100",
  chatView: () => ({
    loading: false,
    error: null,
    info: conversations[0],
    lines: chatLines,
    more: false,
    people: [],
    owner: false,
    typing: [],
    blocked: "",
    maxTitle: 150,
  }),
  messages: () => ({ list: list(messages), moreKey: "" }),
  notifications: () => ({ list: list(notifications), moreKey: "" }),
  notificationCategories: () => [],
  myCommunities: () => list(groups),
  tradeList: () => ({ list: list(trades), moreKey: "" }),
  tradeDraft: () => null,
  tradeEligibility: () => ({ loading: false, canTrade: true, reason: "" }),
  tradeDetail: () => ({
    loading: false,
    error: null,
    status: "Completed",
    partnerId: 100,
    give: null,
    get: null,
    eval: null,
  }),
  themes: () => themes,
  appSettings: () => ({
    mode: "dark",
    darkTheme: "",
    lightTheme: "",
    density: "comfortable",
    playerCommand: "",
    formatLocale: "",
    notify: {
      friendJoins: false,
      trades: false,
      chats: false,
      friendRequests: false,
      experienceUpdates: false,
      sales: false,
      plugins: false,
    },
    cacheData: 0,
    cacheImages: 0,
  }),
} satisfies Fixtures

export const calls: Record<string, number> = {}
export const commands = new Proxy({} as typeof realCommands, {
  get: (_, name: string) => async () => {
    calls[name] = (calls[name] ?? 0) + 1
    const fixture = fixtures[name as keyof typeof fixtures]
    if (!fixture) throw new Error(`Performance fixture missing for ${name}`)
    // Simulate fresh IPC payloads rather than reusing object identity across store updates.
    return structuredClone(fixture())
  },
})

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
  itemPage: () => ({
    loading: false,
    error: null,
    favorited: false,
    wishlisted: false,
    ownedBy: [],
    owned: false,
    value: null,
    info: {
      id: 100,
      bundle: false,
      name: "Midnight aviator",
      creator: "Fixture Studio",
      creatorId: 101,
      creatorIsGroup: true,
      creatorVerified: true,
      creatorFake: false,
      price: 75,
      priceLabel: "75",
      fromReseller: false,
      typeName: "Hat",
      assetType: 8,
      limited: null,
      offSale: false,
      favorites: 1200,
      description: "A classic accessory with a mesh, textures and surface maps.",
      created: "2026-01-01T00:00:00Z",
      updated: "2026-10-01T00:00:00Z",
      offSaleDeadline: "",
      category: "Accessories",
      genres: [],
      tags: [],
      hasResellers: false,
      collectibleId: "",
      totalQuantity: 0,
      unitsLeft: 0,
      premiumOnly: false,
      contents: [],
      inCatalog: true,
      publicDomain: false,
      sales: 0,
    },
  }),
  itemRecommendations: () => list(items),
  itemDependencies: () => ({
    note: null,
    list: list(
      items.slice(0, 256).map((_item, index) => ({
        id: 1000100 + index,
        name: index === 2 ? null : (["Aviator mesh", "Leather base color"][index] ?? `Surface map ${index}`),
        assetType: index === 0 ? 4 : 1,
        typeName: index === 0 ? "Mesh" : "Image",
        creator: "Fixture Studio",
        creatorId: 101,
        creatorIsGroup: true,
        creatorVerified: true,
        uses: [
          {
            class: index === 0 ? "MeshPart" : "SurfaceAppearance",
            path: "Midnight aviator/Handle",
            property: index === 0 ? "MeshId" : "ColorMap",
          },
        ],
      })),
    ),
  }),
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

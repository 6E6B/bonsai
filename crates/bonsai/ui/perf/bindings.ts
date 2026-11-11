// This module replaces bindings only in the explicit performance build. No native IPC or credentials.

import catalog from "../src/lib/commands/catalog.json"
import type { CommandInfo, commands as realCommands, TrustedStatus } from "../src/lib/ipc/bindings"
import { bookmarkFixture, bookmarkInvocation } from "./bookmark-fixture"
import {
  badges,
  catalogCategories,
  chatLines,
  conversations,
  games,
  groups,
  inventoryCategories,
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

const requestedStatus = new URLSearchParams(location.search).get("trusted")
let trustedStatus: TrustedStatus = ["none", "sent", "received", "trusted", "unknown"].includes(requestedStatus ?? "")
  ? (requestedStatus as TrustedStatus)
  : "none"
let trustedPeople = people.slice(0, 6)

const fixtures = {
  bookmarks: () => bookmarkFixture,
  prefs: () => [],
  plugins: () => [],
  remoteSearch: () => ({ loading: false, rows: [] }),
  commands: (): CommandInfo[] =>
    catalog.commands.map((command) => ({
      ...command,
      arg: command.arg as CommandInfo["arg"],
      shortcuts:
        command.id === "bookmark.toggle" || command.id === "view.bookmarks"
          ? [
              {
                kind: "chord",
                ctrl: true,
                shift: command.id === "view.bookmarks",
                alt: false,
                key: command.id === "bookmark.toggle" ? "D" : "B",
                label: command.id === "bookmark.toggle" ? "Ctrl+D" : "Ctrl+Shift+B",
              },
            ]
          : [],
      rebound: false,
      context: null,
    })),
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
  profile: () => ({
    ...profile,
    relation: { friend: "friends" as const, following: false, followedBy: false, blocked: false },
  }),
  trustedFriend: () => ({
    loading: false,
    error: requestedStatus === "error" ? "Roblox could not load trusted status." : null,
    status: requestedStatus === "error" ? ("unknown" as const) : trustedStatus,
  }),
  runCommand: (inv) => {
    if (inv.id.startsWith("bookmark.")) return bookmarkInvocation(inv)
    switch (inv.id) {
      case "user.add_trusted_friend":
        trustedStatus = "sent"
        break
      case "user.accept_trusted_friend":
        trustedStatus = "trusted"
        break
      case "user.cancel_trusted_friend":
      case "user.remove_trusted_friend":
        trustedStatus = "none"
        break
    }
    if (inv.id === "user.accept_trusted_friend" || inv.id === "user.decline_trusted_friend")
      trustedPeople = trustedPeople.filter((p) => p.id !== Number(inv.arg))
    queueMicrotask(() => void window.__bonsaiPerf.refresh())
    return { status: "ok", data: null }
  },
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
  people: (which) =>
    list(
      (which === "trusted_requests" ? trustedPeople : people).map((p, i) => ({
        ...p,
        mutuals: [],
        sentAt: which === "trusted_requests" ? `2026-11-${String(10 - i).padStart(2, "0")}T14:00:00Z` : null,
      })),
    ),
  catalogCategories: () => list(catalogCategories),
  catalogOptions: () => ({
    sorts: [
      { value: "relevance", label: "Relevance" },
      { value: "price_asc", label: "Price, low to high" },
    ],
    sales: [
      { value: "all", label: "All" },
      { value: "limited", label: "Limited" },
    ],
  }),
  inventoryCategories: () => list(inventoryCategories),
  inventoryTotals: () => ({ loading: false, items: 42, rap: 128400, value: 151000, onHold: 0 }),
  inventoryItems: () => ({ list: list(items), moreKey: "" }),
  inventoryBadges: () => ({ list: list(badges), moreKey: "" }),
  inventoryPasses: () => ({ list: list([]), moreKey: "" }),
  inventoryPlaces: () => ({ list: list([]), moreKey: "" }),
  avatar: () => ({ loading: false, error: null, rig: "", wearing: [], emotes: [], colors: [], scales: [], rev: null }),
  wardrobe: () => list([]),
  findUser: () => ({ loading: false, error: null, id: 100 }),
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
  get:
    (_, name: string) =>
    async (...args: unknown[]) => {
      calls[name] = (calls[name] ?? 0) + 1
      const fixture = fixtures[name as keyof typeof fixtures]
      if (!fixture) throw new Error(`Performance fixture missing for ${name}`)
      // Simulate fresh IPC payloads rather than reusing object identity across store updates.
      return structuredClone(Reflect.apply(fixture, null, args))
    },
})

import type {
  CategoryView,
  ChatLine,
  Connection,
  ConversationRow,
  GameCard,
  GroupCard,
  InvCategoryView,
  ItemCard,
  List,
  LookView,
  MessageRow,
  NotificationRow,
  OutfitView,
  ProfileView,
  ThemeView,
  TradeRowView,
  UserBadge,
} from "../src/lib/ipc/bindings"

export const count = Math.min(5000, Math.max(1, Number(new URLSearchParams(location.search).get("rows")) || 500))
const rows = <T>(make: (id: number) => T) => Array.from({ length: count }, (_, i) => make(i + 100))
export const list = <T>(items: T[]): List<T> => ({ loading: false, error: null, items, more: false })
export const people = rows<Connection>((id) => ({
  id,
  name: `player_${id}`,
  displayName: `Player ${id}`,
  verified: false,
  premium: false,
  friend: true,
  presence: "online",
  place: null,
  following: false,
}))
export const games = rows<GameCard>((id) => ({
  universeId: id,
  placeId: id,
  name: `Experience ${id}`,
  creator: "Fixture Studio",
  playing: id,
  rating: 95,
  sponsored: false,
  genre: "Adventure",
}))
export const items = rows<ItemCard>((id) => ({
  id,
  bundle: false,
  name: `Collectible ${id}`,
  creator: "Fixture Studio",
  creatorVerified: false,
  copies: 1,
  assetType: 8,
  price: 100,
  limited: true,
  serial: id,
  rap: 500,
  values: null,
}))
export const groups = rows<GroupCard>((id) => ({
  id,
  name: `Community ${id}`,
  members: id * 10,
  verified: false,
  role: "Member",
  primary: false,
}))
export const badges = rows<UserBadge>((id) => ({
  id,
  name: `Badge ${id}`,
  description: "Synthetic badge",
  winRate: 0.2,
  placeId: id,
}))
export const outfits = rows<OutfitView>((id) => ({ id: String(id), name: `Outfit ${id}`, editable: false, rev: null }))
export const look: LookView = { loading: false, error: null, rig: "R15", items: items.slice(0, 8), colors: [] }
export const profile: ProfileView = {
  loading: false,
  error: null,
  user: {
    id: 100,
    name: "perf_player",
    displayName: "Performance Player",
    description: "Synthetic profile for repeatable performance measurements.",
    created: "2010-01-01T00:00:00Z",
    verified: false,
    premium: false,
    plus: false,
    banned: false,
    friends: count,
    followers: count * 10,
    following: count,
  },
  presence: "online",
  place: null,
  placeId: null,
  universeId: null,
  relation: null,
  mine: false,
  pastNames: [],
  lastOnline: null,
  lastOnlineBy: null,
  lastLocation: null,
  placeImage: null,
  unfriendedAt: null,
  mutualFriends: 0,
  mutualMore: false,
  visits: 10000,
  favoritesReceived: 100,
  nickname: null,
  note: null,
  inventoryPrivate: false,
}

const created = "2026-01-01T12:00:00Z"
export const conversations = rows<ConversationRow>((id) => ({
  id: `chat-${id}`,
  title: `Conversation ${id}`,
  group: false,
  partnerId: id,
  unread: 0,
  updated: created,
  last: "Synthetic conversation",
  pinned: false,
}))
const chatLine = (id: number): ChatLine => ({
  id: `line-${id}`,
  senderId: 101,
  sender: "Fixture Player",
  mine: false,
  text: `Chat line ${id}${id % 7 === 0 ? " Longer synthetic text for measuring wrapped message heights.".repeat(8) : ""}`,
  created,
  system: false,
  moderated: false,
  pending: false,
  hidden: false,
  note: "",
  failed: false,
})
export let chatLines = rows(chatLine)
let nextChatId = 9000
export function appendChat() {
  chatLines = [...chatLines, chatLine(nextChatId++)]
}
export function prependChat() {
  chatLines = [...Array.from({ length: 50 }, (_, i) => chatLine(i + 50)), ...chatLines]
}
export const messages = rows<MessageRow>((id) => ({
  id,
  senderId: id,
  sender: `Sender ${id}`,
  senderVerified: false,
  recipientId: 1,
  recipient: "Fixture Player",
  subject: `Message ${id}`,
  body: "Synthetic message body.",
  created,
  read: true,
  system: false,
}))
export const notifications = rows<NotificationRow>((id) => ({
  id: String(id),
  title: `Notification ${id}`,
  body: "Synthetic notification body.",
  when: created,
  read: true,
  link: null,
  userId: id,
  category: "updates",
}))
export const trades = rows<TradeRowView>((id) => ({
  id,
  partnerId: id,
  partnerName: `trader_${id}`,
  partnerDisplay: `Trader ${id}`,
  created,
  expires: created,
  status: "Completed",
}))
export const themes = rows<ThemeView>((id) => ({
  id: `fixture-${id}`,
  name: `Theme ${id}`,
  author: "Fixture Studio",
  base: "dark",
  builtin: false,
  doc: "",
}))

const choices = (prefix: string, labels: string[]) => labels.map((label, i) => ({ value: `${prefix}-${i}`, label }))
const slots = (entries: [string, number][]) => entries.map(([label, id]) => ({ value: `a${id}`, label }))

// Same names and order as the recorded responses in crates/bonsai-api/tests/fixtures/market.json.
export const catalogCategories: CategoryView[] = [
  { name: "All", taxonomy: "all", subcategories: [] },
  { name: "Body", taxonomy: "body", subcategories: choices("body", ["Full Bodies", "Hair", "Heads"]) },
  {
    name: "Clothing",
    taxonomy: "clothing",
    subcategories: choices("clothing", [
      "Shirts",
      "T-Shirts",
      "Sweaters",
      "Jackets",
      "Pants",
      "Dresses & Skirts",
      "Bodysuits",
      "Shorts",
      "Shoes",
      "Classic Shirts",
      "Classic T-Shirts",
      "Classic Pants",
    ]),
  },
  {
    name: "Accessories",
    taxonomy: "accessories",
    subcategories: choices("accessories", [
      "Head",
      "Face",
      "Neck Accessory",
      "Shoulder Accessory",
      "Front Accessory",
      "Back Accessory",
      "Waist Accessory",
      "Gear",
    ]),
  },
  { name: "Backgrounds", taxonomy: "backgrounds", subcategories: [] },
  { name: "Animations", taxonomy: "animations", subcategories: choices("animations", ["Bundles", "Emotes"]) },
  {
    name: "Makeup",
    taxonomy: "makeup",
    subcategories: choices("makeup", ["Eyes", "Lips", "Face", "Eyelashes", "Eyebrows"]),
  },
]

export const inventoryCategories: InvCategoryView[] = [
  { name: "Collectibles", slots: [{ value: "collectibles", label: "Collectibles" }] },
  { name: "Decals", slots: slots([["Decals", 13]]) },
  { name: "Models & Packages", slots: slots([["Models & Packages", 10]]) },
  { name: "Plugins", slots: slots([["Plugins", 38]]) },
  { name: "Animations", slots: slots([["Animations", 24]]) },
  { name: "Audio", slots: slots([["Audio", 3]]) },
  { name: "Meshes", slots: slots([["Meshes", 40]]) },
  { name: "Heads", slots: slots([["Heads", 79]]) },
  { name: "Video", slots: slots([["Video", 62]]) },
  { name: "Bundles", slots: [{ value: "b1", label: "Body Parts" }] },
  {
    name: "Accessories",
    slots: slots([
      ["Head", 8],
      ["Face", 42],
      ["Neck", 43],
      ["Shoulder", 44],
      ["Front", 45],
      ["Back", 46],
      ["Waist", 47],
      ["Gear", 19],
    ]),
  },
  {
    name: "Classic Clothing",
    slots: slots([
      ["Classic T-shirts", 2],
      ["Classic Shirts", 11],
      ["Classic Pants", 12],
    ]),
  },
  { name: "Hair", slots: slots([["Hair", 41]]) },
  {
    name: "Tops",
    slots: slots([
      ["T-shirts", 65],
      ["Shirts", 64],
      ["Sweaters", 68],
      ["Jackets", 67],
    ]),
  },
  {
    name: "Bottoms",
    slots: slots([
      ["Pants", 66],
      ["Shorts", 69],
      ["Skirts", 72],
    ]),
  },
  {
    name: "Shoes",
    slots: slots([
      ["Left Shoe", 70],
      ["Right Shoe", 71],
    ]),
  },
  { name: "Emotes", slots: slots([["Emotes", 61]]) },
  { name: "Backgrounds", slots: slots([["Backgrounds", 88]]) },
  {
    name: "Makeup",
    slots: slots([
      ["Eyes", 89],
      ["Lips", 90],
      ["Faces", 91],
      ["Eyelash Accessory", 92],
      ["Eyebrow Accessory", 93],
    ]),
  },
  { name: "Places", slots: [{ value: "places", label: "Places" }] },
  { name: "Badges", slots: [{ value: "badges", label: "Badges" }] },
  { name: "Passes", slots: [{ value: "passes", label: "Passes" }] },
  {
    name: "Avatar Animations",
    slots: slots([
      ["Run", 53],
      ["Walk", 55],
      ["Fall", 50],
      ["Jump", 52],
      ["Idle", 51],
      ["Swim", 54],
      ["Climb", 48],
    ]),
  },
]

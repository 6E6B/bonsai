import type {
  ChatLine,
  Connection,
  ConversationRow,
  GameCard,
  GroupCard,
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
  friend: true,
  presence: "online",
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
  unfriendedAt: null,
  mutualFriends: 0,
  mutualMore: false,
  visits: 10000,
  favoritesReceived: 100,
  nickname: null,
  note: null,
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

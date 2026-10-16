import { menuItem } from "~/lib/commands/registry"
import { shell } from "~/lib/store"
import type { MenuEntry } from "~/lib/ui"

/**
 * Menus shared by several screens. Every row is a registry command, so whatever is in a menu
 * is also in the command bar.
 */

const sep: MenuEntry = { kind: "separator" }

/** The `game.join` argument that follows a user into their server. A bare number is a place. */
export const joinUser = (userId: number) => `user:${userId}`

/** "Join as" rows: one per account other than the active one. */
export function joinAsItems(id: string, target: string): MenuEntry[] {
  return shell.accounts
    .filter((a) => a.id !== shell.active)
    .map((a) => menuItem(id, { arg: target, asAccount: a.id, label: a.label ?? a.displayName }))
}

/** "Buy as" rows: one per account other than the active one, with the account as the argument. */
export function accountItems(id: string): MenuEntry[] {
  return shell.accounts
    .filter((a) => a.id !== shell.active)
    .map((a) => menuItem(id, { arg: String(a.id), label: a.label ?? a.displayName }))
}

/** "Join as" submenu, or nothing when there is no other account. */
function joinAs(target: string): MenuEntry[] {
  const items = joinAsItems("game.join_as", target)
  return items.length > 0 ? [{ kind: "submenu", label: "Join as", icon: "user", items }] : []
}

/** Copy and open-in-browser rows, in one submenu so they don't crowd the actions. */
export function shareMenu(target: string): MenuEntry {
  return {
    kind: "submenu",
    label: "Share",
    icon: "share",
    items: [
      menuItem("entity.copy_link", { arg: target, icon: "link" }),
      menuItem("entity.copy_id", { arg: target, icon: "copy" }),
      menuItem("entity.open_website", { arg: target, icon: "external" }),
    ],
  }
}

export interface UserMenuOptions {
  friend?: boolean
  /** In an experience they can be joined. */
  inGame?: boolean
  /** The experience they are in, when visible. Adds Open experience. */
  placeId?: number | null
  blocked?: boolean
  /** Whether the active account follows them. Adds Follow or Unfollow when set. */
  following?: boolean
  /** They sent the active account a friend request. Adds Accept and Decline. */
  requested?: boolean
  /** The menu is on their profile, whose header already has Open, Message, Add friend and
   * Inventory, so those are left out. */
  onProfile?: boolean
}

/**
 * Everyday actions at the top level; relationship changes and the rare and destructive ones
 * under Manage; copy and browser rows under Share.
 */
export function userMenu(userId: number, opts: UserMenuOptions = {}): MenuEntry[] {
  const id = String(userId)
  const target = `user:${id}`
  const friendAs = joinAsItems("user.add_friend_as", id)
  const manage: MenuEntry[] = [
    ...(friendAs.length > 0
      ? [{ kind: "submenu", label: "Add friend as", icon: "user-plus", items: friendAs } as const]
      : []),
    ...(opts.following === undefined
      ? []
      : [menuItem(opts.following ? "user.unfollow" : "user.follow", { arg: id, icon: "bell" })]),
    menuItem("user.nickname", { arg: id, icon: "edit", label: "Set nickname…" }),
    menuItem("user.note", { arg: id, icon: "edit", label: "Edit note…" }),
    sep,
    ...(opts.friend ? [menuItem("user.unfriend", { arg: id, icon: "user-minus", danger: true })] : []),
    ...(opts.requested ? [menuItem("user.decline_friend", { arg: id, icon: "user-minus" })] : []),
    opts.blocked
      ? menuItem("user.unblock", { arg: id, icon: "block" })
      : menuItem("user.block", { arg: id, icon: "block", danger: true }),
    menuItem("entity.report", { arg: target, icon: "flag", label: "Report", danger: true }),
  ]
  return [
    ...(opts.onProfile ? [] : [menuItem("user.open", { arg: id, icon: "user" })]),
    ...(opts.onProfile ? [] : [menuItem("user.message", { arg: id, icon: "chat", label: "Message" })]),
    menuItem("trade.new", { arg: id, icon: "trades", label: "Send trade" }),
    ...(opts.friend || opts.onProfile
      ? []
      : opts.requested
        ? [menuItem("user.accept_friend", { arg: id, icon: "user-plus" })]
        : [menuItem("user.add_friend", { arg: id, icon: "user-plus" })]),
    ...(opts.inGame
      ? [
          sep,
          menuItem("game.join", { arg: joinUser(userId), icon: "play" }),
          ...joinAs(joinUser(userId)),
          ...(opts.placeId ? [menuItem("game.open", { arg: String(opts.placeId), icon: "game" })] : []),
        ]
      : []),
    sep,
    ...(opts.onProfile ? [] : [menuItem("inventory.view", { arg: id, icon: "inventory", label: "Inventory" })]),
    menuItem("user.friends", { arg: id, icon: "friends", label: "Friends" }),
    ...(opts.inGame ? [] : [menuItem("user.find", { arg: id, icon: "search", label: "Find server" })]),
    sep,
    { kind: "submenu", label: "Manage", icon: "sliders", items: manage },
    shareMenu(target),
  ]
}

export function gameMenu(placeId: number): MenuEntry[] {
  const id = String(placeId)
  return [
    menuItem("game.open", { arg: id, icon: "game" }),
    menuItem("game.join", { arg: id, icon: "play" }),
    ...joinAs(id),
    sep,
    shareMenu(`game:${id}`),
  ]
}

/** The command target of a catalog item: `item:<id>` or `bundle:<id>`. */
export const itemTarget = (id: number, bundle = false) => `${bundle ? "bundle" : "item"}:${id}`

/** Asset types with a template image: T-shirt, shirt and pants. Only the last two have the
 * template layout that masking needs. */
const TSHIRT = 2
const SHIRT = 11
const PANTS = 12

/** Template download rows for classic clothing; empty for anything else. */
export function templateItems(itemId: number, assetType: number | undefined): MenuEntry[] {
  if (assetType !== TSHIRT && assetType !== SHIRT && assetType !== PANTS) return []
  const target = itemTarget(itemId)
  return [
    menuItem("market.template", { arg: target, icon: "download" }),
    ...(assetType === TSHIRT ? [] : [menuItem("market.template_masked", { arg: target, icon: "download" })]),
  ]
}

/** Template downloads as one row (T-shirts) or a submenu (shirts and pants). */
function templateMenu(itemId: number, assetType: number | undefined): MenuEntry[] {
  const target = itemTarget(itemId)
  if (assetType === TSHIRT) return [menuItem("market.template", { arg: target, icon: "download" })]
  if (assetType !== SHIRT && assetType !== PANTS) return []
  return [
    {
      kind: "submenu",
      label: "Download template",
      icon: "download",
      items: [
        menuItem("market.template", { arg: target, label: "Template" }),
        menuItem("market.template_masked", { arg: target, label: "Masked" }),
      ],
    },
  ]
}

export interface ItemMenuExtras {
  /** Rows for this screen, like Inspect or Wear, right under Open item. */
  actions?: MenuEntry[]
  /** Destructive rows, last, after a separator. */
  danger?: MenuEntry[]
}

/** `assetType` adds template downloads for clothing. */
export function itemMenu(itemId: number, bundle = false, assetType?: number, extras: ItemMenuExtras = {}): MenuEntry[] {
  const target = itemTarget(itemId, bundle)
  return [
    menuItem("market.open", { arg: target, icon: "marketplace" }),
    ...(extras.actions ?? []),
    sep,
    menuItem("market.favorite", { arg: target, icon: "star", label: "Toggle favorite" }),
    menuItem("market.wishlist", { arg: target, icon: "heart", label: "Toggle wishlist" }),
    ...(bundle ? [] : templateMenu(itemId, assetType)),
    shareMenu(target),
    ...(extras.danger?.length ? [sep, ...extras.danger] : []),
  ]
}

/** The "more" menu on entity pages. `target` is the page's thing, like `game:1818`. `first`
 * rows go above the rest. */
export function entityMenu(target: string, first: MenuEntry[] = []): MenuEntry[] {
  return [
    ...(first.length > 0 ? [...first, sep] : []),
    menuItem("entity.copy_link", { arg: target, icon: "link" }),
    menuItem("entity.copy_id", { arg: target, icon: "copy" }),
    menuItem("entity.open_website", { arg: target, icon: "external" }),
    sep,
    menuItem("entity.report", { arg: target, icon: "flag", label: "Report", danger: true }),
  ]
}

/** Display text for a friend's presence line. */
export function presenceText(p: { presence: string; place?: string }): string {
  if (p.presence === "game") return p.place ?? "In game"
  if (p.presence === "studio") return p.place ? `Studio · ${p.place}` : "In Studio"
  return p.presence === "online" ? "Online" : "Offline"
}

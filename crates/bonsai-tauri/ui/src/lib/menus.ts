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

function joinAs(target: string): MenuEntry {
  const items = joinAsItems("game.join_as", target)
  return { kind: "submenu", label: "Join as", icon: "user", items, disabled: items.length === 0 }
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
  /** They sent the active account a friend request. Adds Decline. */
  requested?: boolean
}

export function userMenu(userId: number, opts: UserMenuOptions = {}): MenuEntry[] {
  const id = String(userId)
  return [
    menuItem("user.open", { arg: id, icon: "user" }),
    menuItem("user.message", { arg: id, icon: "chat" }),
    ...(opts.inGame
      ? [
          sep,
          ...(opts.placeId ? [menuItem("game.open", { arg: String(opts.placeId), icon: "game" })] : []),
          menuItem("game.join", { arg: joinUser(userId), icon: "play" }),
          joinAs(joinUser(userId)),
        ]
      : []),
    menuItem("user.find", { arg: id, icon: "search" }),
    menuItem("inventory.view", { arg: id, icon: "inventory" }),
    menuItem("user.friends", { arg: id, icon: "friends" }),
    menuItem("trade.new", { arg: id, icon: "trades" }),
    ...(opts.following === undefined
      ? []
      : [menuItem(opts.following ? "user.unfollow" : "user.follow", { arg: id, icon: "bell" })]),
    menuItem("user.nickname", { arg: id, icon: "edit" }),
    menuItem("user.note", { arg: id, icon: "edit" }),
    sep,
    menuItem("entity.copy_id", { arg: `user:${id}`, icon: "copy" }),
    menuItem("entity.copy_link", { arg: `user:${id}`, icon: "link" }),
    menuItem("entity.open_website", { arg: `user:${id}`, icon: "external" }),
    menuItem("entity.report", { arg: `user:${id}`, icon: "flag", danger: true }),
    sep,
    opts.friend
      ? menuItem("user.unfriend", { arg: id, icon: "user-minus", danger: true })
      : opts.requested
        ? menuItem("user.decline_friend", { arg: id, icon: "user-minus" })
        : menuItem("user.add_friend", { arg: id, icon: "user-plus" }),
    opts.blocked
      ? menuItem("user.unblock", { arg: id, icon: "block" })
      : menuItem("user.block", { arg: id, icon: "block", danger: true }),
  ]
}

export function gameMenu(placeId: number): MenuEntry[] {
  const id = String(placeId)
  return [
    menuItem("game.open", { arg: id, icon: "game" }),
    menuItem("game.join", { arg: id, icon: "play" }),
    joinAs(id),
    sep,
    menuItem("entity.copy_id", { arg: `game:${id}`, icon: "copy" }),
    menuItem("entity.copy_link", { arg: `game:${id}`, icon: "link" }),
    menuItem("entity.open_website", { arg: `game:${id}`, icon: "external" }),
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

/** `assetType` adds template downloads for clothing. */
export function itemMenu(itemId: number, bundle = false, assetType?: number): MenuEntry[] {
  const target = itemTarget(itemId, bundle)
  const templates = bundle ? [] : templateItems(itemId, assetType)
  return [
    menuItem("market.open", { arg: target, icon: "marketplace" }),
    menuItem("market.favorite", { arg: target, icon: "star" }),
    menuItem("market.wishlist", { arg: target, icon: "heart" }),
    ...(templates.length > 0 ? [sep, ...templates] : []),
    sep,
    menuItem("entity.copy_id", { arg: target, icon: "copy" }),
    menuItem("entity.copy_link", { arg: target, icon: "link" }),
    menuItem("entity.open_website", { arg: target, icon: "external" }),
  ]
}

/** The "more" menu on entity pages. `target` is the page's thing, like `game:1818`. `first`
 * rows go above the rest. */
export function entityMenu(target: string, first: MenuEntry[] = []): MenuEntry[] {
  return [
    ...(first.length > 0 ? [...first, sep] : []),
    menuItem("entity.copy_id", { arg: target, icon: "copy" }),
    menuItem("entity.copy_link", { arg: target, icon: "link" }),
    menuItem("entity.open_website", { arg: target, icon: "external" }),
    sep,
    menuItem("entity.report", { arg: target, icon: "flag", danger: true }),
  ]
}

/** Display text for a friend's presence line. */
export function presenceText(p: { presence: string; place?: string }): string {
  if (p.presence === "game") return p.place ?? "In game"
  if (p.presence === "studio") return p.place ? `Studio · ${p.place}` : "In Studio"
  return p.presence === "online" ? "Online" : "Offline"
}

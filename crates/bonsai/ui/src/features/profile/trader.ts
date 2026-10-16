import type { TraderBadge, TraderBadgeGroup } from "~/lib/ipc/bindings"
import type { BadgeVariant, IconName } from "~/lib/ui"

/** A badge to show, with the lower tiers of its family it stands for. */
export interface ShownBadge {
  badge: TraderBadge
  lower: TraderBadge[]
}

/** One chip per family: its highest tier (the first, since they come highest first), with the
 * rest folded under it. One-offs stand alone. Order is kept. */
export function topBadges(badges: readonly TraderBadge[]): ShownBadge[] {
  const out: ShownBadge[] = []
  const byFamily = new Map<string, ShownBadge>()
  for (const b of badges) {
    const top = b.family ? byFamily.get(b.family) : undefined
    if (top) {
      top.lower.push(b)
      continue
    }
    const shown = { badge: b, lower: [] }
    if (b.family) byFamily.set(b.family, shown)
    out.push(shown)
  }
  return out
}

export function badgeIcon(g: TraderBadgeGroup): IconName {
  switch (g) {
    case "value":
      return "values"
    case "item":
      return "crown"
    case "collection":
      return "inventory"
    case "trading":
      return "trades"
    case "community":
      return "communities"
    case "other":
      return "badge"
  }
}

export type Rarity = "common" | "uncommon" | "rare" | "epic" | "legendary" | "mythic"

/** Lowest to highest. */
const RARITIES: readonly Rarity[] = ["common", "uncommon", "rare", "epic", "legendary", "mythic"]

const RARITY_VARIANTS: Record<Rarity, BadgeVariant> = {
  common: "neutral",
  uncommon: "green",
  rare: "blue",
  epic: "purple",
  legendary: "orange",
  mythic: "red",
}

const BADGE_RARITY: Record<string, Rarity> = {
  // Inventory value.
  value_100k: "common",
  value_500k: "uncommon",
  value_1m: "rare",
  value_5m: "epic",
  value_10m: "legendary",
  value_20m: "mythic",
  // Items and serials.
  own_lucky_cat_uaid: "mythic",
  own_1_serial_1: "mythic",
  own_1_serial_1337: "epic",
  own_1_sequential_serial: "epic",
  own_1_serial_1_to_9: "legendary",
  own_1_big_dominus: "legendary",
  own_1_dominus: "epic",
  own_1_stf: "rare",
  own_1_valued_federation_item: "rare",
  own_1_immortal_sword: "rare",
  own_epic_katana_set: "epic",
  own_1_kotn_item: "rare",
  // Collections.
  own_1_rare: "common",
  own_3_rares: "uncommon",
  own_10_rares: "rare",
  own_5_noob: "common",
  own_15_noob: "uncommon",
  own_10_items: "common",
  own_100_items: "uncommon",
  own_1000_items: "epic",
  own_10_of_1_item: "uncommon",
  own_50_of_1_item: "rare",
  own_100_of_1_item: "epic",
  own_10_pct_of_1_item: "rare",
  own_25_pct_of_1_item: "epic",
  own_50_pct_of_1_item: "legendary",
  own_all_asset_types: "rare",
  // Trading.
  create_10_trade_ads: "common",
  create_100_trade_ads: "uncommon",
  create_1000_trade_ads: "rare",
  create_10000_trade_ads: "epic",
  // Community.
  roli_award_nominee: "epic",
  roli_award_winner: "legendary",
  verified: "common",
  contributor: "rare",
  sword_fighting_champion: "epic",
  event_winner: "epic",
  game_night_winner: "rare",
  booster: "uncommon",
  roligang: "uncommon",
}

const BADGE_DESCRIPTION: Record<string, string> = {
  // Inventory value.
  value_100k: "Own limiteds worth at least 100,000 total value.",
  value_500k: "Own limiteds worth at least 500,000 total value.",
  value_1m: "Own limiteds worth at least 1 million total value.",
  value_5m: "Own limiteds worth at least 5 million total value.",
  value_10m: "Own limiteds worth at least 10 million total value.",
  value_20m: "Own limiteds worth at least 20 million total value.",
  // Items and serials.
  own_lucky_cat_uaid: "Own the Lucky Cat.",
  own_1_serial_1: "Own a serial #1 limited.",
  own_1_serial_1337: "Own a serial #1337 limited.",
  own_1_sequential_serial: "Own a limited with serial #123, #1234 or #12345.",
  own_1_serial_1_to_9: "Own a limited with a serial below #10.",
  own_1_big_dominus: "Own one of the bigger domini: Emp, Frig, Astra, Inf, Pittacium, Aur, Messor or Rex.",
  own_1_dominus: "Own any limited dominus.",
  own_1_stf: "Own a limited Sparkle Time Fedora.",
  own_1_valued_federation_item: "Own a valued federation item.",
  own_1_immortal_sword: "Own a limited immortal sword.",
  own_epic_katana_set: "Own the epic katana set: Blue, Crimson, Golden, Iris, Jade and Ocherous.",
  own_1_kotn_item: "Own a limited King of the Night item.",
  // Collections.
  own_1_rare: "Own a rare limited.",
  own_3_rares: "Own 3 rare limiteds.",
  own_10_rares: "Own 10 rare limiteds.",
  own_5_noob: "Own 5 noob items: Noob Attacks, Noob Assists, Pocket Pals or Bag O' Noobs.",
  own_15_noob: "Own 15 noob items: Noob Attacks, Noob Assists, Pocket Pals or Bag O' Noobs.",
  own_10_items: "Own at least 10 limiteds.",
  own_100_items: "Own at least 100 limiteds.",
  own_1000_items: "Own at least 1,000 limiteds.",
  own_10_of_1_item: "Own 10 copies of one item.",
  own_50_of_1_item: "Own 50 copies of one item.",
  own_100_of_1_item: "Own 100 copies of one item.",
  own_10_pct_of_1_item: "Own 10% of an item's available copies. It needs 2 or more copies.",
  own_25_pct_of_1_item: "Own 25% of an item's available copies. It needs 2 or more copies.",
  own_50_pct_of_1_item: "Own 50% of an item's available copies. It needs 2 or more copies.",
  own_all_asset_types: "Own a limited of every asset type: hat, face, gear, hair accessory and so on.",
  // Trading.
  create_10_trade_ads: "Create 10 trade ads.",
  create_100_trade_ads: "Create 100 trade ads.",
  create_1000_trade_ads: "Create 1,000 trade ads.",
  create_10000_trade_ads: "Create 10,000 trade ads.",
  // Community.
  roli_award_nominee: "Nominated for a Roli Award at the annual ceremony in the Rolimon's Discord.",
  roli_award_winner: "Won a Roli Award at the annual ceremony in the Rolimon's Discord.",
  verified: "Verified their account on Rolimon's.",
  contributor: "Contributed content, artwork or code used by the site or the Discord server.",
  sword_fighting_champion: "Won a sword fighting tournament in the Rolimon's Discord.",
  event_winner: "Won an event in the Rolimon's Discord, such as an outfit or art competition.",
  game_night_winner: "Won a game night in the Rolimon's Discord.",
  booster: "Boosted the Rolimon's Discord server and claimed the badge there.",
  roligang: "Joined the official Rolimon's group on Roblox.",
}

export function badgeDescription(b: TraderBadge): string | undefined {
  return BADGE_DESCRIPTION[b.key]
}

export function badgeRarity(b: TraderBadge): Rarity {
  return BADGE_RARITY[b.key] ?? "common"
}

export function badgeVariant(b: TraderBadge): BadgeVariant {
  return RARITY_VARIANTS[badgeRarity(b)]
}

/** `topBadges`, rarest first. Equal rarities keep Rolimons' order. */
export function rarestFirst(shown: readonly ShownBadge[]): ShownBadge[] {
  const rank = (s: ShownBadge) => RARITIES.indexOf(badgeRarity(s.badge))
  return [...shown].sort((a, b) => rank(b) - rank(a))
}

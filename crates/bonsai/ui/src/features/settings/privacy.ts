import type { PrivacyRow } from "~/lib/ipc/bindings"
import type { Option } from "~/lib/ui"

/** The privacy settings Settings shows, in order, with their labels. Roblox has many more
 * (parties, screen time, AI features); those stay on the website. */
export const privacyLabels: [key: string, label: string][] = [
  ["whoCanChatWithMeInApp", "Who can chat with me in the app"],
  ["whoCanChatWithMeInExperiences", "Who can chat with me in experiences"],
  ["whoCanJoinMeInExperiences", "Who can join me in experiences"],
  ["whoCanSeeMyInventory", "Who can see my inventory"],
  ["whoCanTradeWithMe", "Who can trade with me"],
  ["whoCanSeeMyOnlineStatus", "Who can see when I'm online"],
  ["privateServerPrivacy", "Who can invite me to private servers"],
]

const valueLabels: Record<string, string> = {
  All: "Everyone",
  AllUsers: "Everyone",
  AllAuthenticatedUsers: "Everyone",
  AllConnections: "All connections",
  FriendsFollowingAndFollowers: "Friends, following and followers",
  FriendsAndFollowing: "Friends and following",
  Friends: "Friends",
  Following: "People I follow",
  Followers: "Followers",
  TopFriends: "Top friends",
  NoOne: "No one",
  Disabled: "No one",
}

/** A Roblox value as words: known ones from the table, others split at capitals. */
export function valueLabel(value: string): string {
  return valueLabels[value] ?? value.replace(/([a-z])([A-Z])/g, "$1 $2").replace(/^./, (c) => c.toUpperCase())
}

/** Select options for a setting. The current value is always listed, even when Roblox didn't
 * offer it; options the account can't pick are disabled. */
export function privacyOptions(row: PrivacyRow): Option[] {
  const opts: Option[] = row.options.map((o) => ({
    value: o.value,
    label: valueLabel(o.value),
    disabled: !o.allowed,
  }))
  if (!row.options.some((o) => o.value === row.value)) {
    opts.unshift({ value: row.value, label: valueLabel(row.value) })
  }
  return opts
}

/** The rows Settings shows, in `privacyLabels` order, with their labels. */
export function shownPrivacy(rows: PrivacyRow[]): { row: PrivacyRow; label: string }[] {
  return privacyLabels.flatMap(([key, label]) => {
    const row = rows.find((r) => r.key === key)
    return row ? [{ row, label }] : []
  })
}

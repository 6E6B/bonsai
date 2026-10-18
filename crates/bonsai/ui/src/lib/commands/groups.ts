/** Section names for command id prefixes, for the shortcuts list and settings. */
const groups: [string, string][] = [
  ["palette", "General"],
  ["help", "General"],
  ["app", "General"],
  ["nav", "Go to"],
  ["view", "View"],
  ["tab", "Tabs"],
  ["list", "Lists"],
  ["account", "Accounts"],
  ["game", "Experiences"],
  ["server", "Private servers"],
  ["user", "People"],
  ["friends", "People"],
  ["market", "Marketplace"],
  ["inventory", "Marketplace"],
  ["trade", "Trades"],
  ["transactions", "Transactions"],
  ["chat", "Chat and messages"],
  ["message", "Chat and messages"],
  ["notifications", "Chat and messages"],
  ["group", "Communities"],
  ["entity", "Anything"],
  ["plugin", "Plugins"],
  ["plugins", "Plugins"],
]

export const groupOrder = [...new Set(groups.map(([, g]) => g))]

export function groupOf(id: string): string {
  const prefix = id.split(".")[0]
  return groups.find(([p]) => p === prefix)?.[1] ?? "Other"
}

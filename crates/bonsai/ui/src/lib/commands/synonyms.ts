/** Words people type for the same thing. Each group is interchangeable; a term from a group
 * also matches the group's other words, a little weaker than the word itself. Multi-word
 * entries ("log out") match when every one of their words matches. */
const GROUPS: string[][] = [
  ["remove", "delete", "erase", "trash", "uninstall", "clear"],
  ["settings", "setting", "preferences", "preference", "prefs", "options", "option", "config", "configure"],
  ["message", "messages", "dm", "pm", "msg", "whisper", "chat"],
  ["join", "play", "launch"],
  ["friend", "friends", "connection", "connections", "buddy", "buddies"],
  ["community", "communities", "group", "groups", "clan", "clans"],
  ["experience", "experiences", "game", "games", "place", "places"],
  ["marketplace", "catalog", "shop", "store", "market"],
  ["item", "items", "asset", "assets"],
  ["profile", "user", "player", "person"],
  ["buy", "purchase"],
  ["sign in", "log in", "login", "signin", "logon"],
  ["sign out", "log out", "logout", "signout"],
  ["quit", "exit"],
  ["open", "show", "view", "go", "see"],
  ["new", "create", "add", "make"],
  ["change", "edit", "modify", "set", "update", "adjust"],
  ["refresh", "reload"],
  ["favorite", "favourite", "fav", "fave", "star"],
  ["like", "upvote"],
  ["dislike", "downvote"],
  ["inventory", "inv", "backpack"],
  ["notifications", "notification", "notifs", "notif", "alerts", "alert"],
  ["theme", "themes", "skin", "appearance"],
  ["dark", "night"],
  ["light", "day"],
  ["robux", "money", "rbx"],
  ["trade", "trades", "trading", "swap"],
  ["outfit", "outfits", "fit", "fits", "costume"],
  ["avatar", "character"],
  ["block", "ban"],
  ["search", "find", "lookup"],
  ["private server", "vip server", "ps", "vip"],
  ["copy", "clipboard"],
  ["shortcut", "shortcuts", "hotkey", "hotkeys", "keybind", "keybinds", "keybinding", "keybindings", "binding"],
  ["values", "value", "worth"],
  ["transactions", "transaction", "txn", "txns", "ledger"],
  ["alt", "alts", "account", "accounts", "acc", "accs"],
  ["accept", "approve"],
  ["decline", "reject", "deny"],
  ["wiki", "fandom"],
  ["plugin", "plugins", "extension", "extensions", "addon", "addons", "mod", "mods"],
  ["home", "dashboard"],
  ["pin", "passcode"],
  ["me", "my", "mine", "your", "yours"],
  ["server", "servers", "instance", "lobby"],
  ["background", "wallpaper"],
  ["unfriend", "unconnect"],
  ["picture", "image", "photo", "pic"],
  ["mutuals", "mutual"],
  ["off", "disable", "disabled", "hide", "no"],
  ["on", "enable", "enabled", "show", "yes"],
]

/** Synonyms count this much of a direct match. */
export const SYNONYM_WEIGHT = 0.85

const byWord = new Map<string, string[][]>()
for (const group of GROUPS) {
  const split = group.map((w) => w.split(" "))
  for (const w of group) {
    if (w.includes(" ")) continue
    byWord.set(w, [...(byWord.get(w) ?? []), ...split.filter((s) => s.join(" ") !== w)])
  }
}

/** Other ways to say `term`, each as its words. A plural or singular form comes first. */
export function synonyms(term: string): string[][] {
  const out: string[][] = []
  if (term.length > 3 && term.endsWith("s")) out.push([term.slice(0, -1)])
  out.push(...(byWord.get(term) ?? []))
  return out
}

/** Words that carry no meaning on their own. An unmatched one doesn't count against a result. */
export const STOP = new Set([
  "a",
  "an",
  "the",
  "to",
  "of",
  "for",
  "in",
  "on",
  "at",
  "with",
  "and",
  "i",
  "want",
  "please",
  "can",
  "how",
  "do",
  "is",
  "it",
  "this",
  "that",
  "my",
  "me",
  "from",
  "into",
])

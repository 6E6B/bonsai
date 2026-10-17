import { full } from "~/lib/format"
import { settingsSections } from "~/lib/sections"
import type { ConfirmOptions } from "~/lib/ui"

/**
 * Commands that ask before they run, wherever they run from (button, menu, shortcut or the
 * command bar). Each returns the dialog text for an argument, or null to run without asking.
 */
export const confirmations: Record<string, (arg: string | null) => ConfirmOptions | null> = {
  "user.unfriend": () => ({
    title: "Remove friend?",
    description: "You'd have to send a new request to be friends again.",
    confirm: "Remove",
    danger: true,
  }),
  "user.block": () => ({
    title: "Block this user?",
    description: "They can't message, chat with, follow or join you. You stop being friends.",
    confirm: "Block",
    danger: true,
  }),
  "account.sign_out_others": () => ({
    title: "Sign out of all other sessions?",
    description:
      "Every other browser, device and app signed in to this account is signed out. This app stays signed in.",
    confirm: "Sign out others",
    danger: true,
  }),
  "security.remove_passkey": (arg) => ({
    title: arg ? `Remove the passkey "${arg}"?` : "Remove this passkey?",
    description: "You can't sign in with it any more. Adding it back needs the website.",
    confirm: "Remove",
    danger: true,
  }),
  "friends.decline_all": () => ({
    title: "Ignore all friend requests?",
    confirm: "Ignore all",
    danger: true,
  }),
  "group.leave": (arg) =>
    arg?.includes(",")
      ? { title: `Leave ${arg.split(",").length} communities?`, confirm: "Leave", danger: true }
      : { title: "Leave this community?", confirm: "Leave", danger: true },
  "group.kick": () => ({ title: "Remove this member?", confirm: "Remove", danger: true }),
  "group.delete_post": () => ({ title: "Delete this post?", confirm: "Delete", danger: true }),
  // Resetting one setting from its row needs no question; a section or everything does.
  "settings.reset": (arg) => {
    const target = arg?.trim() || "all"
    const section = settingsSections.find((s) => s.value === target)
    if (target !== "all" && !section) return null
    return {
      title: section ? `Reset ${section.label} settings?` : "Reset all settings?",
      description:
        "They go back to their defaults. Themes, the background, the sidebar, shortcuts and accounts are kept.",
      confirm: "Reset",
      danger: true,
    }
  },
  // From a row it resets one command; with no argument, all of them.
  "shortcuts.reset": (arg) =>
    arg?.trim()
      ? null
      : {
          title: "Reset all shortcuts?",
          description: "Every command gets its default keys back.",
          confirm: "Reset",
          danger: true,
        },
  "plugin.remove": () => ({
    title: "Remove this plugin?",
    description: "Its folder is deleted. An unpacked plugin is only forgotten; its folder stays.",
    confirm: "Remove",
    danger: true,
  }),
  "theme.delete": () => ({
    title: "Delete this theme?",
    description: "Export it first if you want to keep a copy.",
    confirm: "Delete",
    danger: true,
  }),
  // Without an argument these open a form first; the form runs them again with one.
  "group.payout": (arg) =>
    arg?.trim()
      ? {
          title: `Pay out R$ ${arg.trim().split(/\s+/).pop()}?`,
          description: "The Robux leave the community's funds. This can't be undone.",
          confirm: "Pay out",
          danger: true,
        }
      : null,
  // With a price in the argument (the form plugins use) these buy at once, so ask first. Without
  // one, the item or experience page opens its own dialog with the price.
  "market.buy": (arg) => buyAt(arg, /^(\S+) (\d+)$/),
  "market.buy_as": (arg) => buyAt(arg, /^(\S+) (\d+)$/),
  "market.buy_resale": (arg) => buyAt(arg, /^(\S+) (\d+)$/),
  "game.buy_pass": (arg) => buyAt(arg, /^(\d+ \d+) (\d+)$/),
  "game.buy_private_server": (arg) => buyAt(arg, /^(\d+) (\d+) \S/, "Buy this private server"),
  "group.recurring_payouts": (arg) =>
    arg?.trim()
      ? arg.trim().toLowerCase() === "none"
        ? { title: "Stop recurring payouts?", confirm: "Stop", danger: true }
        : {
            title: "Save recurring payouts?",
            description: "Members left out of the list stop getting a share.",
            confirm: "Save",
          }
      : null,
  "group.enemy": (arg) =>
    arg?.trim() ? { title: "Declare this community an enemy?", confirm: "Declare", danger: true } : null,
  "group.remove_ally": () => ({ title: "Remove this ally?", confirm: "Remove", danger: true }),
  "group.remove_enemy": () => ({ title: "Remove this enemy?", confirm: "Remove" }),
  "group.delete_reply": () => ({ title: "Delete this reply?", confirm: "Delete", danger: true }),
  "inventory.delete": () => ({
    title: "Delete this item?",
    description: "It leaves your inventory for good. You don't get Robux back.",
    confirm: "Delete",
    danger: true,
  }),
  "avatar.delete_outfit": () => ({ title: "Delete this outfit?", confirm: "Delete", danger: true }),
  "avatar.update_outfit": () => ({
    title: "Replace this outfit?",
    description: "It gets what you wear now. Its name stays.",
    confirm: "Replace",
  }),
  "user.unblock": () => ({ title: "Unblock this user?", confirm: "Unblock" }),
  "server.remove_user": () => ({
    title: "Take them off this server?",
    description: "They can't join it unless they're your friend and friends are allowed.",
    confirm: "Remove",
    danger: true,
  }),
  "app.clear_cache": () => ({
    title: "Clear the cache?",
    description: "Screens load again from Roblox. Drafts, pinned chats and your wishlist stay.",
    confirm: "Clear",
  }),
  "chat.leave": () => ({ title: "Leave this chat?", confirm: "Leave", danger: true }),
  "trade.accept": () => ({ title: "Accept this trade?", confirm: "Accept" }),
  "trade.decline": () => ({ title: "Decline this trade?", confirm: "Decline", danger: true }),
  "account.sign_out": () => ({
    title: "Sign out?",
    description: "Ends this session on Roblox and removes the account from this app.",
    confirm: "Sign out",
    danger: true,
  }),
  "account.remove": () => ({
    title: "Remove this account from the app?",
    description: "Its saved session is deleted from this computer. It stays signed in elsewhere.",
    confirm: "Remove",
    danger: true,
  }),
}

/** "Buy for R$ 50?" when `arg` matches `form`, whose second group is the price, else null. */
export function buyAt(arg: string | null, form: RegExp, what = "Buy"): ConfirmOptions | null {
  const price = arg?.trim().replace(/\s+/g, " ").match(form)?.[2]
  if (!price) return null
  return {
    title: `${what} for R$ ${full(Number(price))}?`,
    description: "The Robux leave this account. This can't be undone.",
    confirm: "Buy",
    danger: true,
  }
}

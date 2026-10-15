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

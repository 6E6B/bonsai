import type { TrustedStatus } from "~/lib/ipc/bindings"

/** Unknown is deliberately different from no trusted relationship. */
export function trustedActions(status: TrustedStatus, friend: boolean): string[] {
  switch (status) {
    case "received":
      return ["user.accept_trusted_friend", "user.decline_trusted_friend"]
    case "sent":
      return ["user.cancel_trusted_friend"]
    case "trusted":
      return ["user.remove_trusted_friend"]
    case "none":
      return friend ? ["user.add_trusted_friend"] : []
    default:
      return []
  }
}

export const trustedCommands = [
  "user.add_trusted_friend",
  "user.accept_trusted_friend",
  "user.decline_trusted_friend",
  "user.cancel_trusted_friend",
  "user.remove_trusted_friend",
] as const

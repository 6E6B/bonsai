import type { ActionView, CommandInfo, PluginState } from "~/lib/ipc/bindings"
import type { BadgeVariant } from "~/lib/ui"

/** What each write tier means, for the install dialog and Settings > Plugins. See
 * `docs/plugins.md#tiers`. */
export const tiers: Record<number, { label: string; description: string; variant: BadgeVariant }> = {
  1: { label: "Harmless", description: "Seen only by you and easy to undo.", variant: "neutral" },
  2: { label: "Social", description: "Other people can see these.", variant: "warning" },
  3: { label: "Value", description: "Robux or items can be lost.", variant: "danger" },
}

/** What a menu passes as the thing it's about. */
export type PluginContext = "user" | "game" | "item" | "group"

/** Plugin commands that ask to be in `kind`'s menus, by title. */
export function pluginCommandsFor(list: CommandInfo[], kind: PluginContext): CommandInfo[] {
  return list.filter((c) => c.context === kind).sort((a, b) => a.title.localeCompare(b.title))
}

/** Actions grouped by tier, lowest first, leaving out empty tiers. */
export function byTier(actions: ActionView[]): { tier: number; actions: ActionView[] }[] {
  return [1, 2, 3]
    .map((tier) => ({ tier, actions: actions.filter((a) => a.tier === tier) }))
    .filter((g) => g.actions.length > 0)
}

export const pluginStates: Record<PluginState, { label: string; variant: BadgeVariant }> = {
  off: { label: "Off", variant: "neutral" },
  loading: { label: "Starting", variant: "neutral" },
  running: { label: "On", variant: "success" },
  review: { label: "Needs review", variant: "warning" },
  failed: { label: "Failed", variant: "danger" },
}

/** How long the top bar shows that a plugin wrote, in seconds. */
export const RECENT_WRITE = 300

/** Whether a plugin write at `at` (unix seconds) is recent enough at `now` (ms) to show. */
export function wroteRecently(at: number | null, now: number): boolean {
  return at !== null && now / 1000 - at < RECENT_WRITE
}

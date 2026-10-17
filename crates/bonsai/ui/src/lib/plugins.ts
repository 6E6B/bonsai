import type { ActionView, PluginState } from "~/lib/ipc/bindings"
import type { BadgeVariant } from "~/lib/ui"

/** What each write tier means, for the install dialog and Settings > Plugins. See
 * `docs/plugins.md#tiers`. */
export const tiers: Record<number, { label: string; description: string }> = {
  1: { label: "Harmless", description: "Seen only by you and easy to undo." },
  2: { label: "Social", description: "Other people can see these." },
  3: { label: "Value", description: "Robux or items can be lost." },
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

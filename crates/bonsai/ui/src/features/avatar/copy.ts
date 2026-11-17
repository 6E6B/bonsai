import type { Account, CopyPlanView } from "~/lib/ipc/bindings"

/** Accounts a look can be copied to: signed in, and not the one it comes from. */
export function copyTargets(accounts: readonly Account[], from: number | null): Account[] {
  return accounts.filter((a) => a.id !== from && a.session === "ok")
}

/** One line about what a copy will do, or why it can't yet. */
export function copySummary(plan: CopyPlanView): string {
  if (plan.error) return plan.error
  if (plan.loading) return "Loading the look…"
  const checking = plan.items.filter((i) => i.owned === "checking").length
  if (checking) return `Checking which items it owns… ${plan.items.length - checking} of ${plan.items.length}`
  const missing = plan.items.length - plan.wear.length
  const wear = `Wears ${plan.wear.length} of ${plan.items.length} items, plus body type, colors and scales.`
  return missing ? `${wear} ${missing} not owned or not checked will be left off.` : wear
}

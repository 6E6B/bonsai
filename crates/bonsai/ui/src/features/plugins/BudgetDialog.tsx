import { createEffect, createSignal, on, Show } from "solid-js"
import { full } from "~/lib/format"
import { commands } from "~/lib/ipc/bindings"
import { shell } from "~/lib/store"
import { Alert, Button, Dialog, TextField } from "~/lib/ui"

/** A whole number from 0 up, or null. Empty is 0. */
function amount(text: string): number | null {
  const t = text.trim()
  if (t === "") return 0
  const n = Number(t)
  return Number.isInteger(n) && n >= 0 && n <= 4_000_000_000 ? n : null
}

/**
 * A plugin's daily budget on one account, opened by `plugin.budget` from `shell.pluginBudget`.
 * Zero is none: the plugin asks each time.
 */
export function BudgetDialog() {
  const pending = () => shell.pluginBudget
  const [robux, setRobux] = createSignal("")
  const [trades, setTrades] = createSignal("")
  const [payouts, setPayouts] = createSignal("")
  const [error, setError] = createSignal("")
  createEffect(
    on(pending, (p) => {
      setRobux(p?.budget.robux ? String(p.budget.robux) : "")
      setTrades(p?.budget.trades ? String(p.budget.trades) : "")
      setPayouts(p?.budget.payouts ? String(p.budget.payouts) : "")
      setError("")
    }),
  )
  const valid = () => amount(robux()) !== null && amount(trades()) !== null && amount(payouts()) !== null
  const save = async () => {
    if (!valid()) return
    const r = await commands.pluginBudgetSave(amount(robux()) ?? 0, amount(trades()) ?? 0, amount(payouts()) ?? 0)
    if (r.status === "error") setError(r.error)
  }
  const bad = (text: string) => (amount(text) === null ? "A whole number, 0 or more." : undefined)
  return (
    <Dialog
      open={pending() !== null}
      onOpenChange={(open) => {
        if (!open) void commands.pluginBudgetCancel()
      }}
      title={pending() ? `${pending()?.pluginName} budget on ${pending()?.accountName}` : ""}
      size="md"
    >
      <Show when={pending()}>
        {(p) => (
          <form
            class="flex flex-col gap-4 p-4"
            onSubmit={(e) => {
              e.preventDefault()
              void save()
            }}
          >
            <Alert variant="danger" title="Robux it spends and trades it accepts can't be undone.">
              Within these, it buys, pays out and accepts trades without asking you, as long as automation is on for
              this account. Leave one at 0 to be asked each time. Each counts the last 24 hours.
            </Alert>
            <Show when={p().budget.hasRobux}>
              <TextField
                label="Robux a day"
                description={`Spent in the last 24 hours: R$ ${full(p().budget.spentRobux)}`}
                inputmode="numeric"
                placeholder="0"
                value={robux()}
                onChange={setRobux}
                error={bad(robux())}
                autofocus
              />
            </Show>
            <Show when={p().budget.hasTrades}>
              <TextField
                label="Trades accepted a day"
                description={`Accepted in the last 24 hours: ${p().budget.spentTrades}`}
                inputmode="numeric"
                placeholder="0"
                value={trades()}
                onChange={setTrades}
                error={bad(trades())}
                autofocus={!p().budget.hasRobux}
              />
            </Show>
            <Show when={p().budget.hasPayouts}>
              <TextField
                label="Robux paid out of community funds a day"
                description={`Paid out in the last 24 hours: R$ ${full(p().budget.spentPayouts)}`}
                inputmode="numeric"
                placeholder="0"
                value={payouts()}
                onChange={setPayouts}
                error={bad(payouts())}
                autofocus={!p().budget.hasRobux && !p().budget.hasTrades}
              />
            </Show>
            <Show when={error()}>
              <Alert variant="danger" title={error()} />
            </Show>
            <div class="flex justify-end gap-2">
              <Button type="button" onClick={() => void commands.pluginBudgetCancel()}>
                Cancel
              </Button>
              <Button type="submit" variant="primary" disabled={!valid()}>
                Save
              </Button>
            </div>
          </form>
        )}
      </Show>
    </Dialog>
  )
}

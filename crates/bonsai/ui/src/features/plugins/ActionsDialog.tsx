import { createEffect, createSignal, For, on, Show } from "solid-js"
import { commands } from "~/lib/ipc/bindings"
import { tiers } from "~/lib/plugins"
import { shell } from "~/lib/store"
import { Alert, Badge, Button, Dialog } from "~/lib/ui"

/**
 * Actions a plugin command proposed, for the user to run all or cancel. Opens by itself from
 * `shell.pluginActions`. bonsai-core checks them again when they run.
 */
export function ActionsDialog() {
  const pending = () => shell.pluginActions
  const [busy, setBusy] = createSignal(false)
  const [error, setError] = createSignal("")
  createEffect(
    on(pending, () => {
      setBusy(false)
      setError("")
    }),
  )
  const confirm = async () => {
    if (busy()) return
    setBusy(true)
    const r = await commands.pluginActionsRun()
    if (r.status === "error") {
      setBusy(false)
      setError(r.error)
    }
  }
  const count = (n: number) => (n === 1 ? "1 action" : `${n} actions`)
  return (
    <Dialog
      open={pending() !== null}
      onOpenChange={(open) => {
        if (!open && !busy()) void commands.pluginActionsCancel()
      }}
      title={pending() ? `${pending()?.pluginName} wants to run ${count(pending()?.actions.length ?? 0)}` : ""}
      size="md"
    >
      <Show when={pending()}>
        {(p) => (
          <div class="flex flex-col gap-4 p-4">
            <span class="text-sm text-fg-muted">{`As ${p().accountName}. Nothing runs until you confirm.`}</span>
            <ul class="flex max-h-80 flex-col gap-1 overflow-y-auto">
              <For each={p().actions}>
                {(a) => (
                  <li class="flex items-center gap-2 text-sm">
                    <Badge variant={tiers[a.tier]?.variant ?? "neutral"}>{tiers[a.tier]?.label ?? a.tier}</Badge>
                    <span class="min-w-0 flex-1 truncate">{a.title}</span>
                    <Show when={a.arg}>{(arg) => <span class="font-mono text-fg-muted">{arg()}</span>}</Show>
                    <Show when={a.cost}>{(c) => <Badge variant="warning">{c()}</Badge>}</Show>
                  </li>
                )}
              </For>
            </ul>
            <Show when={error()}>
              <Alert variant="danger" title={error()} />
            </Show>
            <div class="flex justify-end gap-2">
              <Button onClick={() => void commands.pluginActionsCancel()} disabled={busy()}>
                Cancel
              </Button>
              <Button variant="primary" onClick={() => void confirm()} loading={busy()}>
                {`Run ${count(p().actions.length)}`}
              </Button>
            </div>
          </div>
        )}
      </Show>
    </Dialog>
  )
}

import { createEffect, createSignal, on, Show } from "solid-js"
import { commands } from "~/lib/ipc/bindings"
import { shell } from "~/lib/store"
import { Alert, Button, Dialog, TextField } from "~/lib/ui"

/**
 * "Allow automation" for one account, opened by `plugins.allow_automation` from
 * `shell.pluginAutomation`. The user confirms by typing the account's name.
 */
export function AutomationDialog() {
  const pending = () => shell.pluginAutomation
  const [typed, setTyped] = createSignal("")
  const [error, setError] = createSignal("")
  createEffect(
    on(pending, () => {
      setTyped("")
      setError("")
    }),
  )
  const matches = () => typed().trim().toLowerCase() === (pending()?.name ?? "").toLowerCase()
  const confirm = async () => {
    if (!matches()) return
    const r = await commands.pluginAutomationConfirm(typed())
    if (r.status === "error") setError(r.error)
  }
  return (
    <Dialog
      open={pending() !== null}
      onOpenChange={(open) => {
        if (!open) void commands.pluginAutomationCancel()
      }}
      title={pending() ? `Allow automation on ${pending()?.displayName}?` : ""}
      size="md"
    >
      <Show when={pending()}>
        {(p) => (
          <form
            class="flex flex-col gap-4 p-4"
            onSubmit={(e) => {
              e.preventDefault()
              void confirm()
            }}
          >
            <Alert variant="danger" title="Roblox bans accounts for bots and automated trading.">
              Plugins will be able to send friend requests, follow, block and chat as @{p().name} without asking you
              each time, up to each plugin's hourly cap. They can also spend Robux and accept trades, but only within a
              daily budget you set for each plugin. Turn it off here or stop every plugin at once with Stop all plugins.
            </Alert>
            <TextField
              label={`Type ${p().name} to confirm`}
              value={typed()}
              onChange={setTyped}
              error={error() || undefined}
              autocomplete="off"
              spellcheck={false}
              autofocus
            />
            <div class="flex justify-end gap-2">
              <Button type="button" onClick={() => void commands.pluginAutomationCancel()}>
                Cancel
              </Button>
              <Button type="submit" variant="danger" disabled={!matches()}>
                Allow automation
              </Button>
            </div>
          </form>
        )}
      </Show>
    </Dialog>
  )
}

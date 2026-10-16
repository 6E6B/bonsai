import { createSignal } from "solid-js"
import { commands } from "~/lib/ipc/bindings"
import { Button, TextField } from "~/lib/ui"
import { pinDigits } from "./challenge"

/** The dialog behind `account.unlock_pin`: the 4-digit account PIN. Roblox's answer arrives as
 * a toast. The PIN is sent once and not kept. */
export function PinForm(props: { onDone: () => void }) {
  const [pin, setPin] = createSignal("")
  const [busy, setBusy] = createSignal(false)
  const [error, setError] = createSignal<string>()
  const submit = async (e: SubmitEvent) => {
    e.preventDefault()
    setBusy(true)
    const r = await commands.unlockPin(pin())
    setBusy(false)
    if (r.status === "error") return setError(r.error)
    props.onDone()
  }
  return (
    <form class="flex flex-col gap-3 p-4" onSubmit={(e) => void submit(e)}>
      <TextField
        label="Account PIN"
        description="Unlocks settings changes for a few minutes."
        type="password"
        inputMode="numeric"
        autocomplete="off"
        autofocus
        maxLength={4}
        value={pin()}
        error={error()}
        onChange={(v) => {
          setPin(pinDigits(v))
          setError(undefined)
        }}
      />
      <div class="flex justify-end gap-2">
        <Button variant="ghost" type="button" onClick={() => props.onDone()}>
          Cancel
        </Button>
        <Button variant="primary" type="submit" disabled={busy() || pin().length !== 4}>
          Unlock
        </Button>
      </div>
    </form>
  )
}

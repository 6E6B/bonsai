import { createSignal, Match, Show, Switch } from "solid-js"
import { commands } from "~/lib/ipc/bindings"
import { shell } from "~/lib/store"
import { Alert, Button, TextField } from "~/lib/ui"
import { quickCode } from "./approve"

/**
 * The dialog behind `account.approve_code`: sign in another device that shows a quick sign-in
 * code. Enter the code, check which device it is, then confirm. Progress comes back in
 * `shell.approve`; closing the dialog early cancels the code (the `onClose` of the caller calls
 * `approveClose`).
 */
export function ApproveCode(props: { initial?: string; onSeq: (seq: number) => void; onDone: () => void }) {
  const [code, setCode] = createSignal(quickCode(props.initial ?? ""))
  const [seq, setSeq] = createSignal<number>()
  const [error, setError] = createSignal<string>()
  const view = () => {
    const a = shell.approve
    return a && a.seq === seq() ? a : undefined
  }
  const account = () => {
    const a = shell.accounts.find((x) => x.id === shell.active)
    return a ? `@${a.name}` : "this account"
  }
  const submit = async (e: SubmitEvent) => {
    e.preventDefault()
    setError(undefined)
    const r = await commands.approveEnter(code())
    if (r.status === "error") return setError(r.error)
    setSeq(r.data)
    props.onSeq(r.data)
  }
  const confirm = async () => {
    const s = seq()
    if (s === undefined) return
    const r = await commands.approveConfirm(s)
    if (r.status === "error") setError(r.error)
  }
  const step = () => view()?.step
  const confirmStep = () => {
    const s = step()
    return s?.step === "confirm" ? s : undefined
  }
  return (
    <div class="flex flex-col gap-3 p-4">
      <Switch>
        <Match when={confirmStep()}>
          {(s) => (
            <>
              <div class="flex flex-col gap-1">
                <span class="text-sm">
                  Sign in as {account()} on {s().device}?
                </span>
                <Show when={s().location}>{(l) => <span class="text-sm text-fg-muted">{l()}</span>}</Show>
              </div>
              <Alert variant="warning" title="Only confirm a sign-in you started">
                That device gets full access to the account. If someone sent you this code, cancel.
              </Alert>
              <Show when={view()?.error ?? error()}>{(e) => <Alert variant="danger" title={e()} />}</Show>
              <div class="flex justify-end gap-2">
                <Button variant="ghost" onClick={() => props.onDone()}>
                  Cancel sign-in
                </Button>
                <Button variant="primary" autofocus disabled={view()?.busy} onClick={() => void confirm()}>
                  Confirm
                </Button>
              </div>
            </>
          )}
        </Match>
        <Match when={step()?.step === "done"}>
          <Alert variant="success" title="Approved">
            The other device is signing in as {account()}.
          </Alert>
          <div class="flex justify-end">
            <Button variant="primary" autofocus onClick={() => props.onDone()}>
              Close
            </Button>
          </div>
        </Match>
        <Match when={true}>
          <form class="flex flex-col gap-3" onSubmit={(e) => void submit(e)}>
            <TextField
              label="Code from the other device"
              description={`Signs that device in as ${account()}. You confirm on the next step.`}
              autocomplete="off"
              autofocus
              maxLength={7}
              value={code()}
              error={view()?.error ?? error()}
              onChange={(v) => {
                setCode(quickCode(v))
                setError(undefined)
                // A rejected code's message goes once the user edits it.
                if (view() && !view()?.busy) setSeq(undefined)
              }}
            />
            <div class="flex justify-end gap-2">
              <Button variant="ghost" type="button" onClick={() => props.onDone()}>
                Cancel
              </Button>
              <Button variant="primary" type="submit" disabled={view()?.busy || code().length !== 6}>
                Continue
              </Button>
            </div>
          </form>
        </Match>
      </Switch>
    </div>
  )
}

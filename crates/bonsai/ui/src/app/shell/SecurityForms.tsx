import { createEffect, createSignal, For, Match, on, Show, Switch } from "solid-js"
import { commands } from "~/lib/ipc/bindings"
import { shell } from "~/lib/store"
import { Alert, Button, Dialog, TextField } from "~/lib/ui"
import { groupKey } from "./security"

type Result = { status: "ok"; data: null } | { status: "error"; error: string }

/** The dialog behind `security.change_password`. The result arrives as a toast; nothing is kept. */
export function ChangePasswordForm(props: { onDone: () => void }) {
  const [current, setCurrent] = createSignal("")
  const [next, setNext] = createSignal("")
  const [again, setAgain] = createSignal("")
  const [error, setError] = createSignal<string>()
  const mismatch = () => again() !== "" && again() !== next()
  const submit = async (e: SubmitEvent) => {
    e.preventDefault()
    if (next() !== again()) return setError("The new passwords don't match.")
    const r = await commands.changePassword(current(), next())
    if (r.status === "error") return setError(r.error)
    props.onDone()
  }
  return (
    <form class="flex flex-col gap-3 p-4" onSubmit={(e) => void submit(e)}>
      <TextField
        label="Current password"
        type="password"
        autocomplete="current-password"
        autofocus
        value={current()}
        onChange={(v) => {
          setCurrent(v)
          setError(undefined)
        }}
      />
      <TextField
        label="New password"
        description="At least 8 characters. Other sessions are signed out."
        type="password"
        autocomplete="new-password"
        value={next()}
        onChange={(v) => {
          setNext(v)
          setError(undefined)
        }}
      />
      <TextField
        label="New password again"
        type="password"
        autocomplete="new-password"
        value={again()}
        error={mismatch() ? "Doesn't match." : error()}
        onChange={(v) => {
          setAgain(v)
          setError(undefined)
        }}
      />
      <div class="flex justify-end gap-2">
        <Button variant="ghost" type="button" onClick={() => props.onDone()}>
          Cancel
        </Button>
        <Button variant="primary" type="submit" disabled={!current() || !next() || next() !== again()}>
          Change password
        </Button>
      </div>
    </form>
  )
}

/** Asks for the password before a two-step change. The result arrives as a toast. */
export function PasswordForm(props: {
  description: string
  confirm: string
  danger?: boolean
  submit: (password: string) => Promise<Result>
  onDone: () => void
}) {
  const [password, setPassword] = createSignal("")
  const [error, setError] = createSignal<string>()
  const submit = async (e: SubmitEvent) => {
    e.preventDefault()
    const r = await props.submit(password())
    if (r.status === "error") return setError(r.error)
    props.onDone()
  }
  return (
    <form class="flex flex-col gap-3 p-4" onSubmit={(e) => void submit(e)}>
      <TextField
        label="Password"
        description={props.description}
        type="password"
        autocomplete="current-password"
        autofocus
        value={password()}
        error={error()}
        onChange={(v) => {
          setPassword(v)
          setError(undefined)
        }}
      />
      <div class="flex justify-end gap-2">
        <Button variant="ghost" type="button" onClick={() => props.onDone()}>
          Cancel
        </Button>
        <Button variant={props.danger ? "danger" : "primary"} type="submit" disabled={!password()}>
          {props.confirm}
        </Button>
      </div>
    </form>
  )
}

/**
 * Turning on the authenticator app (password, then the key and a code, then recovery codes) and
 * making new recovery codes (password, then the codes). Opens by itself from `shell.security`,
 * which `security.authenticator_on` and `security.recovery_codes` set. Closing it makes core
 * forget the password.
 */
export function SecurityDialog() {
  const flow = () => shell.security
  const [input, setInput] = createSignal("")
  const [error, setError] = createSignal<string>()
  createEffect(
    on(
      () => flow()?.step.step,
      () => {
        setInput("")
        setError(undefined)
      },
    ),
  )
  const close = () => {
    const f = flow()
    if (f) void commands.securityClose(f.seq)
  }
  const submit = async (e: SubmitEvent) => {
    e.preventDefault()
    const f = flow()
    if (!f || f.busy) return
    const r =
      f.step.step === "password"
        ? await commands.securityPassword(f.seq, input())
        : await commands.securityCode(f.seq, input())
    if (r.status === "error") setError(r.error)
  }
  const copy = () => {
    const f = flow()
    if (f) void commands.securityCopy(f.seq)
  }
  const keyStep = () => {
    const s = flow()?.step
    return s?.step === "key" ? s : undefined
  }
  const doneStep = () => {
    const s = flow()?.step
    return s?.step === "done" ? s : undefined
  }
  const title = () => (flow()?.task === "authenticator" ? "Turn on authenticator app" : "New recovery codes")
  return (
    <Dialog open={flow() !== null} onOpenChange={(open) => !open && close()} title={title()} size="sm">
      <Show when={flow()}>
        {(f) => (
          <div class="flex flex-col gap-3 p-4">
            <Switch>
              <Match when={doneStep()}>
                {(s) => (
                  <>
                    <Show
                      when={s().codes.length > 0}
                      fallback={<span class="text-sm">Codes from your authenticator app are now asked for.</span>}
                    >
                      <span class="text-sm">
                        <Show when={f().task === "authenticator"}>Authenticator app codes are on. </Show>
                        Write these recovery codes down. Each one signs you in once if you lose your device; the old
                        ones no longer work.
                      </span>
                      <ul class="grid grid-cols-2 gap-x-4 gap-y-1 font-mono text-sm">
                        <For each={s().codes}>{(c) => <li>{c}</li>}</For>
                      </ul>
                    </Show>
                    <div class="flex justify-end gap-2">
                      <Show when={s().codes.length > 0}>
                        <Button icon="copy" onClick={copy}>
                          Copy codes
                        </Button>
                      </Show>
                      <Button variant="primary" onClick={close}>
                        Done
                      </Button>
                    </div>
                  </>
                )}
              </Match>
              <Match when={true}>
                <form class="flex flex-col gap-3" onSubmit={(e) => void submit(e)}>
                  <Show when={keyStep()}>
                    {(s) => (
                      <div class="flex flex-col gap-2">
                        <span class="text-sm">
                          In your authenticator app, add an account by entering this key, then type the 6-digit code it
                          shows.
                        </span>
                        <div class="flex items-center gap-2">
                          <code class="min-w-0 flex-1 break-all font-mono text-sm">{groupKey(s().manualKey)}</code>
                          <Button size="sm" icon="copy" type="button" onClick={copy}>
                            Copy
                          </Button>
                        </div>
                      </div>
                    )}
                  </Show>
                  <Show
                    when={keyStep()}
                    fallback={
                      <TextField
                        label="Password"
                        description={
                          f().task === "authenticator"
                            ? "Roblox asks for it before changing two-step verification."
                            : "Making new codes stops the old ones from working."
                        }
                        type="password"
                        autocomplete="current-password"
                        autofocus
                        value={input()}
                        error={error()}
                        onChange={(v) => {
                          setInput(v)
                          setError(undefined)
                        }}
                      />
                    }
                  >
                    <TextField
                      label="Code from the app"
                      inputMode="numeric"
                      autocomplete="one-time-code"
                      autofocus
                      maxLength={7}
                      value={input()}
                      error={error()}
                      onChange={(v) => {
                        setInput(v)
                        setError(undefined)
                      }}
                    />
                  </Show>
                  <Show when={f().error}>{(e) => <Alert variant="danger" title={e()} />}</Show>
                  <div class="flex justify-end gap-2">
                    <Button variant="ghost" type="button" onClick={close}>
                      Cancel
                    </Button>
                    <Button variant="primary" type="submit" disabled={!input()} loading={f().busy}>
                      {keyStep() ? "Turn on" : "Continue"}
                    </Button>
                  </div>
                </form>
              </Match>
            </Switch>
          </div>
        )}
      </Show>
    </Dialog>
  )
}

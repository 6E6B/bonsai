import { createEffect, createSignal, Match, on, Show, Switch } from "solid-js"
import { type ChallengeKind, type ChallengeView, commands } from "~/lib/ipc/bindings"
import { shell } from "~/lib/store"
import { Alert, Button, Checkbox, Dialog, SegmentedControl, showToast, TextField } from "~/lib/ui"
import { codeDigits, METHOD_LABELS } from "./challenge"

type Code = Extract<ChallengeKind, { kind: "code" }>

/**
 * The check Roblox put in front of an action: a 6-digit code or a puzzle. It opens by itself
 * from `shell.challenge` and sits above any other dialog. When the check passes, bonsai-core sends
 * the action again; cancelling fails it.
 */
export function ChallengeDialog() {
  const c = () => shell.challenge
  const [closing, setClosing] = createSignal(false)
  createEffect(
    on(
      () => c()?.seq,
      () => setClosing(false),
    ),
  )
  const cancel = () => {
    const seq = c()?.seq
    if (seq === undefined || closing()) return
    setClosing(true)
    void commands.challengeCancel(seq)
  }
  const account = () => {
    const id = c()?.account
    const a = shell.accounts.find((x) => x.id === id)
    return a ? (a.label ?? a.displayName) : "this account"
  }
  return (
    <Dialog
      open={c() !== null && !closing()}
      onOpenChange={(open) => {
        if (!open) cancel()
      }}
      title={c()?.kind.kind === "captcha" ? "Solve the puzzle" : "Two-step verification"}
      size="sm"
    >
      <Show when={c()} keyed>
        {(ch) => (
          <div class="flex flex-col gap-3 p-4">
            <span class="text-sm text-fg-muted">Roblox wants to check it's you ({account()}) before it continues.</span>
            <Switch>
              <Match when={ch.kind.kind === "code" && (ch.kind as Code)}>
                {(k) => <CodeForm challenge={c() ?? ch} kind={k()} onCancel={cancel} />}
              </Match>
              <Match when={ch.kind.kind === "captcha"}>
                <CaptchaForm challenge={c() ?? ch} onCancel={cancel} />
              </Match>
            </Switch>
          </div>
        )}
      </Show>
    </Dialog>
  )
}

function Messages(props: { challenge: ChallengeView }) {
  return (
    <>
      <Show when={props.challenge.notice}>{(n) => <Alert variant="info" title={n()} />}</Show>
      <Show when={props.challenge.error}>{(e) => <Alert variant="danger" title={e()} />}</Show>
    </>
  )
}

function CodeForm(props: { challenge: ChallengeView; kind: Code; onCancel: () => void }) {
  const [code, setCode] = createSignal("")
  const [remember, setRemember] = createSignal(false)
  // A wrong code clears the field for the next try.
  createEffect(
    on(
      () => props.challenge.error,
      (e) => {
        if (e) setCode("")
      },
    ),
  )
  const seq = () => props.challenge.seq
  const submit = (e: SubmitEvent) => {
    e.preventDefault()
    if (codeDigits(code()).length !== 6) return showToast("Enter the 6-digit code.")
    void commands.challengeSubmitCode(seq(), codeDigits(code()), remember()).then((r) => {
      if (r.status === "error") showToast(r.error)
    })
  }
  const use = (method: string) =>
    void commands.challengeUseMethod(seq(), method).then((r) => {
      if (r.status === "error") showToast(r.error)
    })
  return (
    <form class="flex flex-col gap-3" onSubmit={submit}>
      <Show when={props.kind.methods.length > 1}>
        <SegmentedControl
          aria-label="Where the code comes from"
          options={props.kind.methods.map((m) => ({ value: m, label: METHOD_LABELS[m] ?? m }))}
          value={props.kind.method}
          onChange={(m) => {
            if (m !== props.kind.method) use(m)
          }}
        />
      </Show>
      <TextField
        label={props.kind.method === "email" ? "Code from your email" : "Code from your authenticator app"}
        autofocus
        inputMode="numeric"
        autocomplete="one-time-code"
        maxLength={7}
        placeholder="123456"
        value={code()}
        onChange={(v) => setCode(codeDigits(v))}
        disabled={props.challenge.busy}
      />
      <Show when={props.kind.rememberOption}>
        <Checkbox checked={remember()} onChange={setRemember}>
          Remember this device
        </Checkbox>
      </Show>
      <Messages challenge={props.challenge} />
      <div class="flex justify-end gap-2">
        <Show when={props.kind.method === "email"}>
          <Button
            variant="ghost"
            type="button"
            class="mr-auto"
            disabled={props.challenge.busy}
            onClick={() => use("email")}
          >
            Resend code
          </Button>
        </Show>
        <Button variant="ghost" type="button" onClick={() => props.onCancel()}>
          Cancel
        </Button>
        <Button variant="primary" type="submit" disabled={props.challenge.busy || codeDigits(code()).length !== 6}>
          Verify
        </Button>
      </div>
    </form>
  )
}

function CaptchaForm(props: { challenge: ChallengeView; onCancel: () => void }) {
  const open = () =>
    void commands.openCaptcha(props.challenge.seq).then((r) => {
      if (r.status === "error") showToast(r.error)
    })
  return (
    <div class="flex flex-col gap-3">
      <span class="text-sm">The puzzle opens in its own window. Once you solve it, the action continues here.</span>
      <Messages challenge={props.challenge} />
      <div class="flex justify-end gap-2">
        <Button variant="ghost" type="button" onClick={() => props.onCancel()}>
          Cancel
        </Button>
        <Button variant="primary" icon="external" disabled={props.challenge.busy} onClick={open}>
          Open puzzle
        </Button>
      </div>
    </div>
  )
}

import { createEffect, createSignal, Match, onCleanup, onMount, Show, Switch } from "solid-js"
import { runBackend } from "~/lib/commands/registry"
import { countdown } from "~/lib/format"
import { commands, type Login } from "~/lib/ipc/bindings"
import { shell } from "~/lib/store"
import { thumbUrl } from "~/lib/thumb"
import { Alert, Avatar, Button, Spinner, TextField, TextLink } from "~/lib/ui"

/** How the account is added: picking a method, a pasted cookie, quick sign-in with a code,
 * or Roblox's sign-in page in its own window. */
export type AddMode = "choose" | "paste" | "quick" | "browser"

const START = { quick: "account.quick_sign_in", browser: "account.sign_in_browser" } as const

/**
 * The dialog behind `account.add`, `account.replace_cookie`, `account.quick_sign_in` and
 * `account.sign_in_browser`. bonsai-core does the work and reports progress in `shell.login`;
 * every mode ends in the same confirm step. The cookie never comes back to the UI.
 */
export function AddAccount(props: { mode: AddMode; onDone: () => void }) {
  const [mode, setMode] = createSignal<AddMode>(props.mode)
  const [text, setText] = createSignal("")
  const [saving, setSaving] = createSignal(false)
  // The sign-in window has been open, so `login === null` afterwards means it was closed.
  const [windowOpened, setWindowOpened] = createSignal(false)
  const [now, setNow] = createSignal(Date.now() / 1000)
  const login = () => shell.login
  const kind = () => login()?.kind
  const when = <K extends Login["kind"]>(k: K) => {
    const l = login()
    return l?.kind === k ? (l as Extract<Login, { kind: K }>) : undefined
  }

  const start = (m: AddMode) => {
    setMode(m)
    setWindowOpened(false)
    if (m === "choose" || m === "paste") return void commands.cancelLogin()
    void runBackend(START[m]).then((ok) => {
      if (!ok) props.onDone()
    })
  }

  onMount(() => {
    if (props.mode === "quick" || props.mode === "browser") start(props.mode)
    const tick = setInterval(() => setNow(Date.now() / 1000), 1000)
    onCleanup(() => clearInterval(tick))
  })
  // Leaving without confirming stops whatever is running: a code, the window, a checked cookie.
  onCleanup(() => {
    if (!saving() && (shell.login || mode() === "quick" || mode() === "browser")) void commands.cancelLogin()
  })
  // Confirmed: bonsai-core clears `login` once the account is saved.
  createEffect(() => {
    if (saving() && login() === null) props.onDone()
  })
  createEffect(() => {
    if (kind() === "failed") setSaving(false)
    if (kind() === "browser") setWindowOpened(true)
  })

  const submit = (e: SubmitEvent) => {
    e.preventDefault()
    if (text().trim()) void commands.submitCookie(text())
    setText("")
  }

  // Back to the method list, only when the dialog opened on it.
  const back = () => (
    <Show when={props.mode === "choose"}>
      <Button variant="ghost" type="button" class="mr-auto" icon="chevron-left" onClick={() => start("choose")}>
        Back
      </Button>
    </Show>
  )

  const busy = (label: string) => (
    <div class="flex items-center gap-2 py-6 text-fg-muted">
      <Spinner size="sm" />
      {label}
    </div>
  )

  return (
    <div class="flex flex-col gap-3 p-4">
      <Switch>
        <Match when={saving() && login() !== null}>{busy("Saving")}</Match>
        <Match when={kind() === "validating"}>{busy(mode() === "paste" ? "Checking cookie" : "Signing in")}</Match>
        <Match when={when("confirm")}>
          {(l) => (
            <>
              <div class="flex items-center gap-3">
                <Avatar size="xl" name={l().user.displayName} src={thumbUrl("headshot", l().user.id, true)} />
                <div class="flex flex-col">
                  <span class="text-lg">{l().user.displayName}</span>
                  <span class="text-fg-muted">@{l().user.name}</span>
                </div>
              </div>
              <Show when={l().alreadyAdded}>
                <p class="text-fg-muted">This account is already added. Saving replaces its cookie.</p>
              </Show>
              <div class="flex justify-end gap-2">
                <Switch>
                  <Match when={mode() === "paste"}>
                    <Button variant="ghost" onClick={() => void commands.cancelLogin()}>
                      Back
                    </Button>
                  </Match>
                  <Match when={true}>
                    <Button variant="ghost" onClick={() => props.onDone()}>
                      Cancel
                    </Button>
                  </Match>
                </Switch>
                <Button
                  variant="primary"
                  autofocus
                  onClick={() => {
                    setSaving(true)
                    void commands.confirmLogin()
                  }}
                >
                  {l().alreadyAdded ? "Save" : "Add account"}
                </Button>
              </div>
            </>
          )}
        </Match>
        <Match when={mode() === "quick"}>
          <Switch fallback={busy("Getting a code")}>
            <Match when={when("quickCode")}>
              {(l) => (
                <>
                  <p>
                    On a device where you're signed in to Roblox, open Quick Log In in Settings and enter this code.
                  </p>
                  <div class="flex flex-col items-center gap-1 py-2">
                    <span class="font-mono text-2xl select-all">{l().code}</span>
                    <span class="text-fg-muted text-sm">Expires in {countdown(l().expiresAt - now())}</span>
                  </div>
                  <Show when={l().linked}>
                    {(name) => <Alert title={`@${name()} entered the code`}>Approve the sign-in on that device.</Alert>}
                  </Show>
                  <div class="flex justify-end gap-2">
                    {back()}
                    <Button variant="ghost" onClick={() => props.onDone()}>
                      Cancel
                    </Button>
                  </div>
                </>
              )}
            </Match>
            <Match when={when("failed")}>
              {(l) => (
                <>
                  <Alert variant="danger" title="Couldn't sign in">
                    {l().message}
                  </Alert>
                  <div class="flex justify-end gap-2">
                    {back()}
                    <Button variant="ghost" onClick={() => props.onDone()}>
                      Cancel
                    </Button>
                    <Button variant="primary" autofocus onClick={() => start("quick")}>
                      Get a new code
                    </Button>
                  </div>
                </>
              )}
            </Match>
          </Switch>
        </Match>
        <Match when={mode() === "browser"}>
          <Switch fallback={busy("Opening the sign-in window")}>
            <Match when={kind() === "browser"}>
              <p>Sign in on the Roblox page in the other window. This continues once you're signed in.</p>
              <div class="flex justify-end gap-2">
                {back()}
                <Button variant="ghost" onClick={() => props.onDone()}>
                  Cancel
                </Button>
                <Button icon="globe" onClick={() => start("browser")}>
                  Show window
                </Button>
              </div>
            </Match>
            <Match when={when("failed")}>
              {(l) => (
                <>
                  <Alert variant="danger" title="Couldn't add account">
                    {l().message}
                  </Alert>
                  <div class="flex justify-end gap-2">
                    {back()}
                    <Button variant="ghost" onClick={() => props.onDone()}>
                      Cancel
                    </Button>
                    <Button variant="primary" autofocus onClick={() => start("browser")}>
                      Try again
                    </Button>
                  </div>
                </>
              )}
            </Match>
            <Match when={windowOpened() && login() === null}>
              <p class="text-fg-muted">The sign-in window was closed.</p>
              <div class="flex justify-end gap-2">
                {back()}
                <Button variant="ghost" onClick={() => props.onDone()}>
                  Cancel
                </Button>
                <Button variant="primary" autofocus onClick={() => start("browser")}>
                  Open again
                </Button>
              </div>
            </Match>
          </Switch>
        </Match>
        <Match when={mode() === "choose"}>
          <div class="flex flex-col gap-2">
            <Button size="lg" icon="link" autofocus onClick={() => start("quick")}>
              Quick sign-in
            </Button>
            <Button size="lg" icon="globe" onClick={() => start("browser")}>
              Roblox sign-in page
            </Button>
          </div>
          <div class="flex items-center justify-between gap-2">
            <TextLink tone="muted" class="text-sm" onClick={() => start("paste")}>
              Use a .ROBLOSECURITY cookie
            </TextLink>
            <Button variant="ghost" onClick={() => props.onDone()}>
              Cancel
            </Button>
          </div>
        </Match>
        <Match when={true}>
          <form class="flex flex-col gap-3" onSubmit={submit}>
            <Show when={when("failed")}>
              {(l) => (
                <Alert variant="danger" title="Couldn't add account">
                  {l().message}
                </Alert>
              )}
            </Show>
            <TextField
              type="password"
              label=".ROBLOSECURITY cookie"
              description="Stored in your system keychain. Never shown again or sent anywhere but Roblox."
              autocomplete="off"
              spellcheck={false}
              autofocus
              value={text()}
              onChange={setText}
            />
            <div class="flex justify-end gap-2">
              {back()}
              <Button variant="ghost" type="button" onClick={() => props.onDone()}>
                Cancel
              </Button>
              <Button variant="primary" type="submit" disabled={text().trim() === ""}>
                Check
              </Button>
            </div>
          </form>
        </Match>
      </Switch>
    </div>
  )
}

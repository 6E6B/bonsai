import { Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import { shell } from "~/lib/store"
import { Alert, Button } from "~/lib/ui"

/** App-wide problems shown above every screen until they are fixed. */
export function ShellAlerts() {
  const active = () => shell.accounts.find((a) => a.id === shell.active)
  return (
    <>
      <Show when={active()?.session === "expired" || active()?.session === "noCookie" ? active() : undefined}>
        {(a) => (
          <Alert
            class="mx-4 mt-3"
            variant="warning"
            title={a().session === "expired" ? "Session expired" : "No saved session"}
            actions={
              <Button size="sm" onClick={() => run("account.replace_cookie")}>
                Paste new cookie
              </Button>
            }
          >
            {a().label ?? a().displayName} is signed out. Paste a new .ROBLOSECURITY cookie to keep using it.
          </Alert>
        )}
      </Show>
      <Show when={shell.offline}>
        <Alert class="mx-4 mt-3" variant="warning" title="You're offline">
          Showing saved data. Changes need a connection.
        </Alert>
      </Show>
      <Show when={shell.keychainWarning}>
        {(w) => (
          <Alert
            class="mx-4 mt-3"
            variant="danger"
            title="Can't save sessions securely"
            actions={
              <Button size="sm" onClick={() => run("nav.settings", "advanced")}>
                Settings
              </Button>
            }
          >
            {w()}
          </Alert>
        )}
      </Show>
    </>
  )
}

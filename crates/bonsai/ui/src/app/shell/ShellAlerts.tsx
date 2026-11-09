import { Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import type { Account } from "~/lib/ipc/bindings"
import { canReactivate, moderationLabel, moderationNext, moderationTone, useNowSecs } from "~/lib/moderation"
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
      <Show when={active()?.session === "moderated" ? active() : undefined}>
        {(a) => <ModerationAlert account={a()} />}
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
              <Button size="sm" onClick={() => run("nav.settings", "accounts")}>
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

/** The active account is warned, banned, terminated or locked: what Roblox said and what to do. */
function ModerationAlert(props: { account: Account }) {
  const now = useNowSecs()
  const m = () => props.account.moderation
  const name = () => props.account.label ?? props.account.displayName
  return (
    <Alert
      class="mx-4 mt-3"
      variant={moderationTone(m(), now())}
      title={`${name()}: ${moderationLabel(m(), now())}`}
      actions={
        <>
          <Show when={canReactivate(m(), now())}>
            <Button size="sm" variant="primary" onClick={() => run("account.reactivate")}>
              Reactivate
            </Button>
          </Show>
          <Button size="sm" icon="external" onClick={() => run("entity.open_website", "section:moderation")}>
            Open on website
          </Button>
        </>
      }
    >
      <div class="flex flex-col gap-1">
        <Show when={m()?.title && m()?.title !== m()?.message ? m()?.title : undefined}>
          {(t) => <span class="font-medium">{t()}</span>}
        </Show>
        <Show when={m()?.message}>{(text) => <p class="line-clamp-4 whitespace-pre-line">{text()}</p>}</Show>
        <span class="text-fg-muted">{moderationNext(m(), now())}</span>
      </div>
    </Alert>
  )
}

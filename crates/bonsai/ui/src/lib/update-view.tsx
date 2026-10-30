import { Match, Show, Switch } from "solid-js"
import { run } from "~/lib/commands/registry"
import { Alert, Button, Card, Icon, Meter, SettingRow, Spinner } from "~/lib/ui"
import { updateBusy, updateMessage, updates } from "~/lib/updates"

function UpdateAction(props: { quiet?: boolean }) {
  return (
    <Switch>
      <Match when={updates.phase === "available"}>
        <Button size="sm" variant={props.quiet ? "secondary" : "primary"} onClick={() => run("app.download_update")}>
          Download update
        </Button>
      </Match>
      <Match when={updates.phase === "ready"}>
        <Button size="sm" variant={props.quiet ? "secondary" : "primary"} onClick={() => run("app.install_update")}>
          Install and restart
        </Button>
      </Match>
      <Match when={updates.phase === "idle" || updates.phase === "checking"}>
        <Button size="sm" loading={updates.phase === "checking"} onClick={() => run("app.check_updates")}>
          Check for updates
        </Button>
      </Match>
    </Switch>
  )
}

/** Always visible above the page when an update is pending, even with the status bar hidden. */
export function UpdateNotice() {
  const pending = () => ["available", "downloading", "verifying", "ready", "installing"].includes(updates.phase)
  return (
    <Show when={pending()}>
      <Card class="mx-4 mt-3 flex shrink-0 flex-wrap items-center gap-x-3 gap-y-2 px-3 py-2">
        <span class="flex shrink-0 items-center text-fg-muted" aria-hidden="true">
          <Show when={updateBusy(updates.phase)} fallback={<Icon name="download" />}>
            <Spinner size="sm" />
          </Show>
        </span>
        <div class="min-w-0 flex-1 text-sm" role="status">
          <p class="font-medium">{updateMessage(updates)}</p>
          <Show when={updates.error}>
            <p class="text-fg-muted">{updates.error}</p>
          </Show>
        </div>
        <div class="ml-auto flex shrink-0 items-center gap-2">
          <Button size="sm" variant="ghost" onClick={() => run("nav.settings", "desktop/updates")}>
            Details
          </Button>
          <UpdateAction quiet />
        </div>
      </Card>
    </Show>
  )
}

/** Shared update status for Desktop settings. Release notes remain plain text. */
export function UpdateDetails() {
  return (
    <div data-setting="updates" class="flex flex-col gap-3 pb-3">
      <SettingRow label={`Bonsai ${updates.currentVersion}`} description={updateMessage(updates)}>
        <Show when={updateBusy(updates.phase)}>
          <Spinner size="sm" />
        </Show>
        <UpdateAction />
      </SettingRow>
      <Show when={updates.phase === "downloading"}>
        <Show
          when={updates.total && updates.total > 0}
          fallback={<p class="text-sm text-fg-muted">{Math.round(updates.downloaded / 1048576)} MB downloaded</p>}
        >
          <Meter
            label="Update download"
            value={Math.min(updates.downloaded, updates.total ?? 0)}
            max={updates.total ?? 1}
            valueLabel={`${Math.round(updates.downloaded / 1048576)} / ${Math.round((updates.total ?? 0) / 1048576)} MB`}
          />
        </Show>
      </Show>
      <Show when={updates.error}>
        <Alert title="Update failed" variant="danger">
          {updates.error}
        </Alert>
      </Show>
      <Show when={updates.checkedAt}>
        <p class="text-sm text-fg-muted">Last checked: {new Date(updates.checkedAt ?? 0).toLocaleString()}</p>
      </Show>
      <Show when={updates.notes}>
        <details class="text-sm">
          <summary class="cursor-pointer text-fg-muted">What’s new in {updates.version}</summary>
          <p class="whitespace-pre-wrap py-2">{updates.notes}</p>
        </details>
      </Show>
    </div>
  )
}

import { createEffect, Match, on, Show, Switch } from "solid-js"
import { run } from "~/lib/commands/registry"
import { Alert, Button, dismissToast, Meter, SettingRow, Spinner, showToast } from "~/lib/ui"
import { updateBusy, updateMessage, updates } from "~/lib/updates"

function UpdateAction() {
  return (
    <Switch>
      <Match when={updates.phase === "available"}>
        <Button size="sm" variant="primary" onClick={() => run("app.download_update")}>
          Download update
        </Button>
      </Match>
      <Match when={updates.phase === "ready"}>
        <Button size="sm" variant="primary" onClick={() => run("app.install_update")}>
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

/** Toasts when an update is found, when it is ready to install and when a download fails.
 * Each one is shown once; a stale one is dismissed when the phase moves on. */
export function useUpdateToasts() {
  let shown = ""
  let open: number | undefined
  const close = () => {
    if (open !== undefined) dismissToast(open)
    open = undefined
  }
  createEffect(
    on(
      () => [updates.phase, updates.version] as const,
      ([phase, version]) => {
        const key = `${phase}:${version}`
        if (key === shown) return
        if (phase !== "available" && phase !== "ready") {
          if (!updateBusy(phase)) close()
          return
        }
        shown = key
        close()
        open = showToast({
          title: updateMessage(updates),
          persistent: true,
          action:
            phase === "available"
              ? { label: "Download", onClick: () => run("app.download_update") }
              : { label: "Install and restart", onClick: () => run("app.install_update") },
        })
      },
    ),
  )
  createEffect(
    on(
      () => updates.error,
      (error, prev) => {
        if (!error || error === prev || updates.phase !== "available") return
        close()
        open = showToast({
          title: "Update failed",
          description: error,
          variant: "error",
          persistent: true,
          action: { label: "Retry", onClick: () => run("app.download_update") },
        })
      },
    ),
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

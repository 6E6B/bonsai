import { createEffect, createSignal, For, on, Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import { emptyHub, hubUi, publishProblem, setHubUi, target } from "~/lib/hub"
import { commands, type HubHistory } from "~/lib/ipc/bindings"
import { useLive } from "~/lib/store"
import { Alert, Badge, Button, Dialog, EmptyState, Select, showToast, TextField } from "~/lib/ui"

export function HubDialogs() {
  const account = useLive(
    () => (hubUi.publishing ? "publish-account" : ""),
    async (key) => {
      if (!key) return emptyHub()
      const r = await commands.hubCatalog(JSON.stringify({ kind: hubUi.kind }))
      return r.status === "ok" ? r.data : { ...emptyHub(), error: r.error }
    },
    emptyHub,
  )
  const history = useLive(
    () => (hubUi.selected ? target(hubUi.selected) : ""),
    async (selected): Promise<HubHistory> => {
      if (!selected) return { loading: false, error: null, versions: [] }
      const result = await commands.hubHistory(selected)
      return result.status === "ok" ? result.data : { loading: false, error: result.error, versions: [] }
    },
    (): HubHistory => ({ loading: false, error: null, versions: [] }),
  )
  const [error, setError] = createSignal("")
  const [sending, setSending] = createSignal(false)
  const [reason, setReason] = createSignal("")
  createEffect(
    on(
      () => hubUi.publishing,
      () => setError(""),
    ),
  )
  const publish = async () => {
    if (sending() || account.busy) return
    const problem = publishProblem(hubUi)
    if (problem) return setError(problem)
    setSending(true)
    try {
      const accepted = await run(
        hubUi.kind === "themes" ? "hub.publish_theme" : "hub.publish_plugin",
        JSON.stringify({
          local_id: hubUi.localId,
          hub_id: hubUi.hubId,
          name: hubUi.name.trim(),
          version: hubUi.version,
          description: hubUi.description,
          tags: hubUi.tags
            .split(",")
            .map((t) => t.trim())
            .filter(Boolean),
        }),
      )
      if (accepted) {
        setHubUi("publishing", false)
        showToast(
          hubUi.kind === "themes"
            ? "Uploading theme. Bonsai will notify you when it is published."
            : "Submitting plugin for review…",
        )
      } else setError("Upload could not start. Check your connection and publication details, then try again.")
    } catch {
      setError("Upload could not start. Your publication details are still here. Try again.")
    } finally {
      setSending(false)
    }
  }
  return (
    <>
      <Dialog
        open={!!hubUi.selected}
        onOpenChange={(open) => {
          if (!open) {
            setHubUi("selected", null)
            setReason("")
          }
        }}
        title={hubUi.selected?.name ?? "Release details"}
        size="md"
      >
        <Show when={hubUi.selected}>
          {(entry) => (
            <div class="flex min-h-0 flex-col gap-4 overflow-y-auto p-4">
              <div class="flex gap-2">
                <Badge>{entry().kind}</Badge>
                <Badge>{entry().version}</Badge>
              </div>
              <p class="text-sm whitespace-pre-wrap">{entry().description}</p>
              <dl class="grid grid-cols-[auto_1fr] gap-x-4 gap-y-2 text-sm">
                <dt class="text-fg-muted">Publisher</dt>
                <dd class="break-all">{entry().ownerId}</dd>
                <dt class="text-fg-muted">Size</dt>
                <dd>{Math.ceil(entry().size / 1024)} KiB</dd>
                <dt class="text-fg-muted">SHA-256</dt>
                <dd class="font-mono text-xs break-all">{entry().sha256}</dd>
              </dl>
              <Show when={entry().kind === "plugins"}>
                <div class="flex flex-col gap-2">
                  <span class="text-sm font-medium">Requested permissions and actions</span>
                  <For each={entry().permissions}>{(permission) => <Badge>{permission}</Badge>}</For>
                  <Alert title="Review before granting access">
                    The install dialog checks the actual package manifest and shows permissions added by an update.
                  </Alert>
                </div>
              </Show>
              <Button
                variant="primary"
                onClick={async () => {
                  const selected = entry()
                  if (await run("hub.install", target(selected))) setHubUi("selected", null)
                }}
              >
                Review installation
              </Button>
              <div class="flex flex-col gap-2">
                <span class="text-sm font-medium">Version history</span>
                <Show when={history.loading}>
                  <span class="text-sm text-fg-muted">Loading versions</span>
                </Show>
                <Show when={history.error}>
                  <Alert title="Could not load versions" variant="danger">
                    {history.error}
                  </Alert>
                </Show>
                <For each={history.versions}>
                  {(version) => (
                    <div class="flex items-center justify-between gap-3 text-sm">
                      <span>{version.version}</span>
                      <Button
                        size="sm"
                        onClick={async () => {
                          if (
                            await run(
                              "hub.install",
                              JSON.stringify({ kind: entry().kind, id: entry().id, version: version.version }),
                            )
                          )
                            setHubUi("selected", null)
                        }}
                      >
                        Review version
                      </Button>
                    </div>
                  )}
                </For>
              </div>
              <div class="flex flex-col gap-2 border-t border-line pt-3">
                <TextField
                  label="Report a problem"
                  placeholder="Describe the issue (at least 10 characters)"
                  value={reason()}
                  onChange={setReason}
                  multiline
                />
                <div class="flex justify-between gap-2">
                  <Button
                    size="sm"
                    disabled={reason().trim().length < 10}
                    onClick={async () => {
                      if (
                        await run(
                          "hub.report",
                          JSON.stringify({ ...JSON.parse(target(entry())), reason: reason().trim() }),
                        )
                      ) {
                        setReason("")
                        setHubUi("selected", null)
                      }
                    }}
                  >
                    Submit report
                  </Button>
                  <Show when={entry().ownerId === hubUi.userId}>
                    <Button size="sm" onClick={() => run("hub.unlist", target(entry()))}>
                      Unlist
                    </Button>
                    <Button
                      size="sm"
                      variant="danger"
                      onClick={async () => {
                        if (await run("hub.delete", target(entry()))) setHubUi("selected", null)
                      }}
                    >
                      Delete
                    </Button>
                  </Show>
                </div>
              </div>
            </div>
          )}
        </Show>
      </Dialog>
      <Dialog
        open={hubUi.publishing}
        onOpenChange={(open) => {
          if (!sending()) setHubUi("publishing", open)
        }}
        title={hubUi.kind === "themes" ? "Publish theme" : "Submit plugin for review"}
        size="md"
      >
        <div class="flex min-h-0 flex-col gap-3 overflow-y-auto p-4">
          <Show
            when={hubUi.choices.length > 0}
            fallback={
              <EmptyState
                icon="palette"
                title={hubUi.kind === "themes" ? "Save a theme first" : "No plugins to submit"}
                description={
                  hubUi.kind === "themes"
                    ? "Create a theme or edit a copy of a built-in theme, then save it before publishing."
                    : "Install a plugin package before submitting it for review."
                }
              >
                <Show when={hubUi.kind === "themes"}>
                  <Button
                    onClick={() => {
                      setHubUi("publishing", false)
                      void run("theme.new")
                    }}
                  >
                    Create theme
                  </Button>
                </Show>
              </EmptyState>
            }
          >
            <Alert title={hubUi.kind === "themes" ? "Share your theme with the community" : "Plugins require approval"}>
              {hubUi.kind === "themes"
                ? "This uploads a saved copy to the hub. Later edits stay private until you publish another version."
                : "Your plugin files are packaged and sent for review. Set a new version in plugin.toml before submitting an update."}
            </Alert>
            <Show when={!account.userId}>
              <Alert title="Sign in to publish">
                Your GitHub account identifies you as the publisher. Your Roblox account is not used.
                <Button size="sm" class="mt-2" loading={account.signingIn} onClick={() => run("hub.sign_in")}>
                  {account.signingIn ? "Waiting for GitHub…" : "Sign in with GitHub"}
                </Button>
              </Alert>
            </Show>
            <Show when={account.userName}>
              <p class="text-sm text-fg-muted">Publishing as {account.userName}</p>
            </Show>
            <Select
              label={hubUi.kind === "themes" ? "Theme to publish" : "Plugin to submit"}
              placeholder={hubUi.kind === "themes" ? "Choose one of your saved themes" : "Choose a plugin"}
              value={hubUi.localId}
              disabled={sending()}
              options={hubUi.choices.map((c) => ({ value: c.id, label: c.name }))}
              onChange={(id) => {
                const picked = hubUi.choices.find((c) => c.id === id)
                if (picked) setHubUi({ localId: id, hubId: id, name: picked.name, version: picked.version })
              }}
            />
            <TextField label="Name" value={hubUi.name} onChange={(name) => setHubUi("name", name)} maxLength={64} />
            <TextField
              label="Version"
              description="Use 1.0.0 for a first release. For an update, use a new version such as 1.0.1. Published versions cannot be replaced."
              value={hubUi.version}
              onChange={(version) => setHubUi("version", version)}
              disabled={hubUi.kind === "plugins"}
              maxLength={64}
            />
            <TextField
              label="Description"
              description={
                hubUi.kind === "themes" ? "Describe the look of your theme." : "Describe what your plugin does."
              }
              multiline
              value={hubUi.description}
              onChange={(description) => setHubUi("description", description)}
              maxLength={2000}
            />
            <TextField
              label="Tags (optional)"
              description="Up to 8 lowercase tags, separated by commas."
              placeholder="dark, minimal"
              value={hubUi.tags}
              onChange={(tags) => setHubUi("tags", tags)}
            />
            <details class="text-sm">
              <summary class="cursor-pointer text-fg-muted">Advanced publication settings</summary>
              <TextField
                class="mt-3"
                label="Publication ID"
                value={hubUi.hubId}
                description="A stable name used to identify this theme on the hub. We fill it in for you. Keep it the same when publishing updates."
                onChange={(hubId) => setHubUi("hubId", hubId)}
                disabled={hubUi.kind === "plugins"}
                maxLength={64}
              />
            </details>
          </Show>
          <Show when={error() || account.error}>
            <Alert title="Could not publish" variant="danger">
              {error() || account.error}
            </Alert>
          </Show>
          <div class="flex justify-end gap-2">
            <Button disabled={sending()} onClick={() => setHubUi("publishing", false)}>
              Cancel
            </Button>
            <Button
              variant="primary"
              loading={sending()}
              disabled={!hubUi.localId || !account.userId || account.busy}
              onClick={publish}
            >
              {sending() ? "Starting upload…" : hubUi.kind === "themes" ? "Publish theme" : "Submit for review"}
            </Button>
          </div>
        </div>
      </Dialog>
    </>
  )
}

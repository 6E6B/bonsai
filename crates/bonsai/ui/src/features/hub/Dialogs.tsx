import { createSignal, For, Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import { hubUi, publishProblem, setHubUi, target } from "~/lib/hub"
import { commands, type HubHistory } from "~/lib/ipc/bindings"
import { useLive } from "~/lib/store"
import { Alert, Badge, Button, Dialog, Select, TextField } from "~/lib/ui"

export function HubDialogs() {
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
  const publish = async () => {
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
      if (accepted) setHubUi("publishing", false)
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
          setHubUi("publishing", open)
          setError("")
        }}
        title={hubUi.kind === "themes" ? "Publish theme" : "Submit plugin for review"}
        size="md"
      >
        <div class="flex min-h-0 flex-col gap-3 overflow-y-auto p-4">
          <Alert title={hubUi.kind === "themes" ? "Publish a saved theme" : "Plugins require approval"}>
            {hubUi.kind === "themes"
              ? "Each version is permanent. Your verified Bonsai account is the publisher."
              : "The current installed files and manifest version are packaged. New versions must be set in plugin.toml before submission."}
          </Alert>
          <Select
            label="Installed content"
            placeholder="Choose saved content"
            value={hubUi.localId}
            options={hubUi.choices.map((c) => ({ value: c.id, label: c.name }))}
            onChange={(id) => {
              const picked = hubUi.choices.find((c) => c.id === id)
              if (picked) setHubUi({ localId: id, hubId: id, name: picked.name, version: picked.version })
            }}
          />
          <TextField
            label="Hub ID"
            value={hubUi.hubId}
            onChange={(hubId) => setHubUi("hubId", hubId)}
            disabled={hubUi.kind === "plugins"}
            maxLength={64}
          />
          <TextField label="Name" value={hubUi.name} onChange={(name) => setHubUi("name", name)} maxLength={64} />
          <TextField
            label="Release version"
            value={hubUi.version}
            onChange={(version) => setHubUi("version", version)}
            disabled={hubUi.kind === "plugins"}
            maxLength={64}
          />
          <TextField
            label="Description"
            multiline
            value={hubUi.description}
            onChange={(description) => setHubUi("description", description)}
            maxLength={2000}
          />
          <TextField
            label="Tags"
            placeholder="dark, minimal"
            value={hubUi.tags}
            onChange={(tags) => setHubUi("tags", tags)}
          />
          <Show when={error()}>
            <Alert title="Check publication details" variant="danger">
              {error()}
            </Alert>
          </Show>
          <div class="flex justify-end gap-2">
            <Button onClick={() => setHubUi("publishing", false)}>Cancel</Button>
            <Button variant="primary" disabled={!hubUi.localId || sending()} onClick={publish}>
              Submit release
            </Button>
          </div>
        </div>
      </Dialog>
    </>
  )
}

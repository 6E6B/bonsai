import { createEffect, createMemo, createSignal, For, onCleanup, Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import { emptyHub, hubQuery, hubUi, setHubUi, target } from "~/lib/hub"
import { commands, type HubEntry } from "~/lib/ipc/bindings"
import { useLive } from "~/lib/store"
import { parseTheme, resolve } from "~/lib/theme"
import { Alert, Badge, Button, Card, EmptyState, Page, SegmentedControl, Select, TextField, ThemeCard } from "~/lib/ui"

export default function Hub() {
  const data = useLive(
    hubQuery,
    async () => {
      const result = await commands.hubCatalog(hubQuery())
      return result.status === "ok" ? result.data : { ...emptyHub(), error: result.error }
    },
    emptyHub,
    { keep: true },
  )
  createEffect(() => setHubUi("userId", data.userId))
  const [search, setSearch] = createSignal(hubUi.query)
  createEffect(() => {
    const q = search()
    const timer = setTimeout(() => setHubUi({ query: q, offset: 0 }), 250)
    onCleanup(() => clearTimeout(timer))
  })
  const installed = useLive(
    () => "hub-installed",
    async () => ({ themes: await commands.themes(), plugins: await commands.plugins() }),
    () => ({ themes: [], plugins: [] }),
  )
  const installLabel = (entry: HubEntry) => {
    const found =
      entry.kind === "themes"
        ? installed.themes.find((t) => t.id === entry.id)
        : installed.plugins.find((p) => p.id === entry.id)
    if (!found) return "Review install"
    const version = "doc" in found ? parseTheme(found.doc)?.version : found.version
    return version === entry.version ? "Installed" : "Review update"
  }
  return (
    <Page
      title="Hub"
      actions={
        <>
          <Show
            when={data.userName}
            fallback={
              <Button
                size="sm"
                disabled={!data.configured || data.signingIn || data.busy}
                onClick={() => run("hub.sign_in")}
              >
                {data.signingIn ? "Waiting for browser" : "Sign in with GitHub"}
              </Button>
            }
          >
            <span class="text-sm text-fg-muted">{data.userName}</span>
            <Button size="sm" disabled={data.busy} onClick={() => run("hub.sign_out")}>
              Sign out
            </Button>
          </Show>
          <Button size="sm" disabled={!data.configured || data.loading} onClick={() => run("hub.refresh")}>
            Refresh
          </Button>
          <Button size="sm" variant="primary" disabled={!data.userId || data.busy} onClick={() => run("hub.publish")}>
            Publish
          </Button>
        </>
      }
    >
      <div class="flex flex-col gap-4">
        <div class="flex flex-wrap items-center gap-3">
          <SegmentedControl
            aria-label="Hub content"
            value={hubUi.kind}
            options={[
              { value: "themes", label: "Themes" },
              { value: "plugins", label: "Plugins" },
            ]}
            onChange={(kind) => setHubUi({ kind, offset: 0, base: "all" })}
          />
          <TextField
            icon="search"
            aria-label="Search hub"
            placeholder="Search names, descriptions and tags"
            class="w-72"
            value={search()}
            onChange={setSearch}
          />
          <Show when={hubUi.kind === "themes"}>
            <SegmentedControl
              size="sm"
              aria-label="Theme base"
              value={hubUi.base}
              options={[
                { value: "all", label: "All" },
                { value: "dark", label: "Dark" },
                { value: "light", label: "Light" },
              ]}
              onChange={(base) => setHubUi({ base, offset: 0 })}
            />
          </Show>
          <Select
            size="sm"
            aria-label="Sort releases"
            value={hubUi.sort}
            onChange={(sort) => setHubUi({ sort, offset: 0 })}
            options={[
              { value: "updated", label: "Recently updated" },
              { value: "name", label: "Name" },
            ]}
          />
          <Button
            size="sm"
            onClick={() =>
              run(hubUi.kind === "themes" ? "nav.themes" : "nav.settings", hubUi.kind === "themes" ? null : "plugins")
            }
          >
            Manage installed
          </Button>
        </div>
        <Show when={!data.configured}>
          <Alert title="Hub is unavailable">
            You can still manage your installed themes and plugins while the hub is unavailable.
          </Alert>
        </Show>
        <Show when={data.error}>
          <Alert title="Hub request failed" variant="danger">
            {data.error}
          </Alert>
        </Show>
        <Show when={data.signingIn}>
          <Alert
            title="Complete sign-in in your browser"
            actions={
              <Button size="sm" disabled={data.busy} onClick={() => run("hub.sign_out")}>
                Cancel
              </Button>
            }
          >
            Sign in with GitHub to publish. Your publisher account is separate from the active Roblox account.
          </Alert>
        </Show>
        <Show when={hubUi.kind === "plugins"}>
          <Alert title="Reviewed releases only">
            Installing opens a permission review. Updates require the same review before new permissions are granted.
          </Alert>
        </Show>
        <For each={data.revoked}>
          {(warning) => (
            <Alert title="Revoked plugin" variant="danger">
              {warning}
            </Alert>
          )}
        </For>
        <div class="flex items-center justify-between text-sm text-fg-muted" aria-live="polite">
          <span>
            {data.items.length} releases on this page<Show when={data.loading}> · Loading</Show>
            <Show when={data.busy}> · Working</Show>
          </span>
          <Show when={hubUi.kind === "plugins"}>
            <Button size="sm" disabled={!data.configured} onClick={() => run("hub.revocations")}>
              Check installed releases
            </Button>
          </Show>
        </div>
        <Show
          when={data.items.length > 0}
          fallback={
            <EmptyState
              icon={hubUi.kind === "themes" ? "palette" : "plugin"}
              title={data.loading ? "Loading hub" : "No releases found"}
              description="Try another search or refresh the hub."
            />
          }
        >
          <div class="grid grid-cols-[repeat(auto-fill,minmax(15rem,1fr))] gap-3">
            <For each={data.items}>
              {(entry) => {
                const theme = createMemo(() => (entry.document ? parseTheme(entry.document) : undefined))
                return (
                  <Card class="flex flex-col gap-3 p-3">
                    <Show when={theme()}>
                      {(doc) => (
                        <ThemeCard
                          class="w-full"
                          name={entry.name}
                          detail={`${entry.version} · ${entry.base}`}
                          tokens={resolve(doc())}
                          onClick={() => run("hub.details", JSON.stringify(entry))}
                        />
                      )}
                    </Show>
                    <Show when={!theme()}>
                      <div class="flex items-center justify-between gap-2">
                        <span class="font-medium">{entry.name}</span>
                        <Badge>{entry.version}</Badge>
                      </div>
                    </Show>
                    <p class="text-sm text-fg-muted line-clamp-2">{entry.description || "No description."}</p>
                    <div class="flex flex-wrap gap-1">
                      <For each={entry.tags}>{(tag) => <Badge>{tag}</Badge>}</For>
                    </div>
                    <div class="mt-auto flex items-center justify-between gap-2">
                      <Button size="sm" onClick={() => run("hub.details", JSON.stringify(entry))}>
                        Details
                      </Button>
                      <Button
                        size="sm"
                        variant="primary"
                        disabled={data.busy || installLabel(entry) === "Installed"}
                        onClick={() => run("hub.install", target(entry))}
                      >
                        {installLabel(entry)}
                      </Button>
                    </div>
                  </Card>
                )
              }}
            </For>
          </div>
        </Show>
        <div class="flex items-center justify-between">
          <Button
            size="sm"
            disabled={hubUi.offset === 0 || data.loading}
            onClick={() => setHubUi("offset", Math.max(0, hubUi.offset - 24))}
          >
            Previous
          </Button>
          <span class="text-sm text-fg-muted">Page {Math.floor(hubUi.offset / 24) + 1}</span>
          <Button
            size="sm"
            disabled={data.nextOffset === null || data.loading}
            onClick={() => setHubUi("offset", data.nextOffset ?? hubUi.offset)}
          >
            Next
          </Button>
        </div>
        <Show when={data.userId}>
          <Card class="flex flex-col gap-3 p-3">
            <div class="flex items-center justify-between">
              <span class="font-medium">Your submissions</span>
              <Button size="sm" onClick={() => run("hub.submissions")}>
                Refresh status
              </Button>
            </div>
            <Show
              when={data.submissions.length > 0}
              fallback={<p class="text-sm text-fg-muted">Refresh to see published themes and plugin reviews.</p>}
            >
              <For each={data.submissions}>
                {(submission) => (
                  <div class="flex items-center gap-3 text-sm">
                    <span>
                      {submission.entryId} {submission.version}
                    </span>
                    <Badge>{submission.status}</Badge>
                    <span class="text-fg-muted">{submission.reason}</span>
                  </div>
                )}
              </For>
            </Show>
          </Card>
        </Show>
      </div>
    </Page>
  )
}

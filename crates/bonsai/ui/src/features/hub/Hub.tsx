import { createEffect, createMemo, createSignal, For, onCleanup, Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import { emptyHub, hubQuery, hubUi, setHubUi, target } from "~/lib/hub"
import { commands, type HubEntry } from "~/lib/ipc/bindings"
import { useLive } from "~/lib/store"
import { parseTheme, resolve } from "~/lib/theme"
import {
  Alert,
  Badge,
  Button,
  Card,
  EmptyState,
  Icon,
  IconButton,
  Page,
  SegmentedControl,
  Select,
  TabPanel,
  Tabs,
  TextField,
  ThemeCard,
  VirtualGrid,
} from "~/lib/ui"

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
  const [section, setSection] = createSignal("browse")
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
  const Browse = () => (
    <div class="flex flex-col gap-3">
      <div class="flex flex-wrap items-center justify-between gap-2">
        <div class="flex flex-wrap items-center gap-2">
          <TextField
            size="sm"
            icon="search"
            aria-label="Search hub"
            placeholder={hubUi.kind === "themes" ? "Search themes…" : "Search plugins…"}
            class="w-64"
            value={search()}
            onChange={setSearch}
          />

          <div class="flex items-center gap-3">
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
          </div>
        </div>
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
      </div>
      <span class="text-sm text-fg-muted" aria-live="polite">
        {data.items.length} {hubUi.kind} on this page<Show when={data.loading}> · Loading</Show>
        <Show when={data.busy}> · Working</Show>
      </span>
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
        <VirtualGrid items={data.items} minItemWidth={288} estimateRowHeight={240} gap={12}>
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
                    <div class="flex items-center gap-3">
                      <span class="flex size-8 shrink-0 items-center justify-center rounded-md bg-control text-fg-muted">
                        <Icon name="plugin" />
                      </span>
                      <div class="flex min-w-0 flex-col gap-1">
                        <span class="font-medium">{entry.name}</span>
                        <span class="text-xs text-fg-muted">Version {entry.version}</span>
                      </div>
                    </div>
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
                    icon="download"
                    disabled={data.busy || installLabel(entry) === "Installed"}
                    onClick={() => run("hub.install", target(entry))}
                  >
                    {installLabel(entry)}
                  </Button>
                </div>
              </Card>
            )
          }}
        </VirtualGrid>
      </Show>
      <Show when={hubUi.offset > 0 || data.nextOffset !== null}>
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
      </Show>
      <Show when={hubUi.kind === "plugins"}>
        <div class="flex items-center justify-between gap-3 border-t border-line pt-3">
          <span class="text-xs text-fg-muted">Permissions are reviewed before installation.</span>
          <Button size="sm" variant="ghost" disabled={!data.configured} onClick={() => run("hub.revocations")}>
            Check installed releases
          </Button>
        </div>
      </Show>
    </div>
  )
  return (
    <Page
      title="Hub"
      actions={
        <>
          <IconButton
            size="sm"
            icon="refresh"
            label="Refresh hub"
            disabled={!data.configured || data.loading}
            onClick={() => run("hub.refresh")}
          />
          <Button
            size="sm"
            onClick={() =>
              run(hubUi.kind === "themes" ? "nav.themes" : "nav.settings", hubUi.kind === "themes" ? null : "plugins")
            }
          >
            {hubUi.kind === "themes" ? "My themes" : "Installed plugins"}
          </Button>
        </>
      }
    >
      <Tabs
        aria-label="Hub section"
        value={section() === "publisher" ? "publisher" : hubUi.kind}
        items={[
          { value: "themes", label: "Themes" },
          { value: "plugins", label: "Plugins" },
          { value: "publisher", label: "Publishing" },
        ]}
        onChange={(value) => {
          if (value === "publisher") setSection(value)
          else {
            setSection("browse")
            setHubUi({ kind: value, offset: 0, base: "all" })
          }
        }}
      >
        <Show when={!data.configured || data.error || data.signingIn || data.revoked.length > 0}>
          <div class="flex flex-col gap-3 pt-3">
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
            <For each={data.revoked}>
              {(warning) => (
                <Alert title="Revoked plugin" variant="danger">
                  {warning}
                </Alert>
              )}
            </For>
          </div>
        </Show>
        <TabPanel value="themes">
          <Browse />
        </TabPanel>
        <TabPanel value="plugins">
          <Browse />
        </TabPanel>
        <TabPanel value="publisher" class="flex flex-col gap-3">
          <div class="flex flex-wrap items-center justify-between gap-4">
            <div class="flex flex-col gap-1">
              <span class="font-medium">Publisher account</span>
              <span class="text-sm text-fg-muted">
                Sign in with GitHub to publish. Browsing and installing need no account.
              </span>
            </div>
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
              <div class="flex items-center gap-3">
                <span class="text-sm">{data.userName}</span>
                <Button size="sm" disabled={data.busy} onClick={() => run("hub.sign_out")}>
                  Sign out
                </Button>
              </div>
            </Show>
          </div>
          <Show when={data.userId}>
            <Card class="flex flex-col gap-3 p-3">
              <div class="flex items-center justify-between">
                <span class="font-medium">Your submissions</span>
                <div class="flex gap-2">
                  <Button size="sm" onClick={() => run("hub.submissions")}>
                    Refresh status
                  </Button>
                  <Button
                    size="sm"
                    disabled={data.busy}
                    onClick={() => run("hub.publish", JSON.stringify({ kind: "plugins" }))}
                  >
                    Submit plugin
                  </Button>
                  <Button
                    size="sm"
                    variant="primary"
                    disabled={data.busy}
                    onClick={() => run("hub.publish", JSON.stringify({ kind: "themes" }))}
                  >
                    Publish theme
                  </Button>
                </div>
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
        </TabPanel>
      </Tabs>
    </Page>
  )
}

import { useSearchParams } from "@solidjs/router"
import { createEffect, createMemo, createSignal, For, Show } from "solid-js"
import { useListCursor, useSubTabs } from "~/lib/commands/hooks"
import { remoteThumb } from "~/lib/commands/palette"
import { run } from "~/lib/commands/registry"
import { commands, type RemoteRow, type SearchResults } from "~/lib/ipc/bindings"
import { ListState } from "~/lib/list-state"
import { useLive } from "~/lib/store"
import { setTabTitle } from "~/lib/tabs"
import { thumbUrl } from "~/lib/thumb"
import {
  Avatar,
  Button,
  Card,
  EmptyState,
  ListItem,
  Page,
  SectionHeader,
  TabPanel,
  Tabs,
  TextField,
  Thumbnail,
} from "~/lib/ui"

const scopes = [
  { value: "all", label: "All" },
  { value: "experiences", label: "Experiences" },
  { value: "people", label: "People" },
  { value: "communities", label: "Communities" },
  { value: "items", label: "Marketplace" },
]

/** Roblox search across experiences, people, communities and the marketplace. All shows a few
 * of each; a tab shows more of one kind. With no query, recent searches. Enter in the field runs
 * `search.open`, which remembers the query. */
export default function Search() {
  const [params, setParams] = useSearchParams()
  const query = () => (typeof params.q === "string" ? params.q.trim() : "")
  const [draft, setDraft] = createSignal(query())
  createEffect(() => setDraft(query()))
  const [scope, setScope] = createSignal("all")
  const [cursor, setCursor] = createSignal(0)
  useSubTabs(
    () => scopes,
    scope,
    (v) => {
      setScope(v)
      setCursor(0)
    },
  )
  createEffect(() => setTabTitle(`/search${query() ? `?q=${encodeURIComponent(query())}` : ""}`, query() || "Search"))

  const results = useLive(
    () => `${scope()}\n${query()}`,
    (key) => {
      const [s, q] = key.split("\n")
      return q ? commands.searchResults(q, s) : Promise.resolve({ loading: false, error: null, rows: [] })
    },
    (): SearchResults => ({ loading: true, error: null, rows: [] }),
  )
  const recent = useLive(
    () => 0,
    async () => ({ items: await commands.recentSearches() }),
    () => ({ items: [] as string[] }),
  )
  /** Rows by group, each group where its first row came. `index` is the row's place in
   * the order shown, for the keyboard cursor. */
  const groups = createMemo(() => {
    const out: { title: string; rows: { row: RemoteRow; index: number }[] }[] = []
    for (const row of results.rows) {
      const g = out.find((x) => x.title === row.group)
      if (g) g.rows.push({ row, index: 0 })
      else out.push({ title: row.group, rows: [{ row, index: 0 }] })
    }
    let i = 0
    for (const g of out) for (const r of g.rows) r.index = i++
    return out
  })
  const ordered = createMemo(() => groups().flatMap((g) => g.rows.map((r) => r.row)))
  const open = (r: RemoteRow) => void run(r.command, r.arg)
  useListCursor({
    count: () => ordered().length,
    index: cursor,
    setIndex: setCursor,
    onOpen: (i) => {
      const r = ordered()[i]
      if (r) open(r)
    },
  })
  const submit = () => {
    const q = draft().trim()
    if (q) void run("search.open", q)
    else setParams({ q: undefined })
  }

  return (
    <Page title="Search" width="narrow">
      <form
        class="flex items-center gap-2"
        onSubmit={(e) => {
          e.preventDefault()
          submit()
        }}
      >
        <TextField
          class="flex-1"
          icon="search"
          aria-label="Search Roblox"
          placeholder="Experiences, people, communities, items"
          value={draft()}
          onChange={setDraft}
          autofocus
        />
        <Button type="submit" variant="primary">
          Search
        </Button>
      </form>
      <Show when={query()} fallback={<Recent items={recent.items} />}>
        <Tabs aria-label="Search scope" items={scopes} value={scope()} onChange={setScope}>
          <TabPanel value={scope()}>
            <ListState list={{ ...results, items: results.rows, more: false }} icon="search" empty="No results">
              <div class="flex flex-col gap-4" data-cursor-list>
                <For each={groups()}>
                  {(g) => (
                    <div class="flex flex-col">
                      <Show when={scope() === "all"}>
                        <SectionHeader
                          title={g.title}
                          actions={
                            <Show when={scopeOf(g.rows[0].row)}>
                              {(s) => (
                                <Button size="sm" variant="ghost" onClick={() => setScope(s())}>
                                  More
                                </Button>
                              )}
                            </Show>
                          }
                        />
                      </Show>
                      <For each={g.rows}>
                        {(r) => (
                          <ListItem
                            data-index={r.index}
                            selected={r.index === cursor()}
                            leading={<RowPicture row={r.row} />}
                            title={r.row.title}
                            subtitle={r.row.subtitle || undefined}
                            onClick={() => {
                              setCursor(r.index)
                              open(r.row)
                            }}
                          />
                        )}
                      </For>
                    </div>
                  )}
                </For>
              </div>
            </ListState>
          </TabPanel>
        </Tabs>
      </Show>
    </Page>
  )
}

/** The tab that shows more rows like `r`; none for pasted links. */
function scopeOf(r: RemoteRow): string | undefined {
  if (r.group === "Pasted link") return undefined
  const kinds: Record<string, string> = {
    user: "people",
    experience: "experiences",
    community: "communities",
    item: "items",
    bundle: "items",
  }
  return kinds[r.kind]
}

function RowPicture(props: { row: RemoteRow }) {
  return (
    <Show when={remoteThumb(props.row)} fallback={<span class="size-10" />}>
      {(t) => (
        <Show
          when={t().kind === "headshot"}
          fallback={<Thumbnail class="w-10" alt="" src={thumbUrl(t().kind, t().id)} fallback="search" />}
        >
          <Avatar name={props.row.title} src={thumbUrl("headshot", t().id)} />
        </Show>
      )}
    </Show>
  )
}

/** What the active account searched for lately. */
function Recent(props: { items: string[] }) {
  return (
    <Show when={props.items.length > 0} fallback={<EmptyState icon="search" title="Search Roblox" />}>
      <Card class="flex flex-col p-1">
        <SectionHeader
          class="px-2.5 pt-1.5"
          title="Recent searches"
          actions={
            <Button size="sm" variant="ghost" onClick={() => run("search.clear_history")}>
              Clear
            </Button>
          }
        />
        <For each={props.items}>
          {(q) => <ListItem leading={<span class="size-4" />} title={q} onClick={() => run("search.open", q)} />}
        </For>
      </Card>
    </Show>
  )
}

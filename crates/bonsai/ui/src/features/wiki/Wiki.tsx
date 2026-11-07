import { useSearchParams } from "@solidjs/router"
import { isTauri } from "@tauri-apps/api/core"
import { createEffect, createSignal, For, Match, on, onCleanup, Show, Switch } from "solid-js"
import { useListCursor } from "~/lib/commands/hooks"
import { run } from "~/lib/commands/registry"
import { ago } from "~/lib/format"
import { commands, type WikiHit } from "~/lib/ipc/bindings"
import { useLive } from "~/lib/store"
import {
  Alert,
  Badge,
  Button,
  EmptyState,
  ListItem,
  Page,
  SectionHeader,
  Skeleton,
  TextField,
  Thumbnail,
  Toolbar,
  VirtualList,
} from "~/lib/ui"
import { kindLabel, mainPage, ROBLOX_WIKI, type WikiSearchView } from "~/lib/wiki"
import { openArticle, useWikiPage, WikiOff, WikiState, WikiSummary } from "~/lib/wiki-panels"
import { sampleSearch } from "~/lib/wiki-sample"

/** Search the Roblox Wiki on Fandom, or with `?w=jailbreak` an experience's own wiki. Results
 * open in the article screen. When there is room, the result under the cursor shows beside the
 * list: a click previews it and a second click, or Enter, opens it. */
export default function Wiki() {
  const [params] = useSearchParams()
  const wiki = () => (typeof params.w === "string" && params.w ? params.w : ROBLOX_WIKI)
  const other = () => wiki() !== ROBLOX_WIKI
  // Another wiki's name comes with its main page, which its search links to anyway.
  const main = useWikiPage(() => (other() ? mainPage(wiki()) : ""))
  const source = () => (other() ? (main.article?.source ?? wiki()) : "Roblox Wiki")
  const [query, setQuery] = createSignal("")
  const [searched, setSearched] = createSignal("")
  createEffect(
    on(
      () => params.q,
      (q) => {
        const v = typeof q === "string" ? q : ""
        setQuery(v)
        setSearched(v.trim())
      },
    ),
  )
  // Search the wiki once typing pauses, not on every key.
  createEffect(() => {
    const q = query().trim()
    const t = setTimeout(() => setSearched(q), 350)
    onCleanup(() => clearTimeout(t))
  })
  const [cursor, setCursor] = createSignal(0)
  const view = useLive(
    () => JSON.stringify([wiki(), searched()]),
    (k) => {
      const [w, q] = JSON.parse(k) as [string, string]
      return commands.wikiSearch(w, q)
    },
    (): WikiSearchView =>
      isTauri()
        ? { wiki: wiki(), enabled: true, loading: Boolean(searched()), error: null, hits: [] }
        : sampleSearch(searched(), wiki()),
  )
  // Wide enough for the list and a readable preview beside it.
  const [wide, setWide] = createSignal(false)
  const measure = (el: HTMLElement) => {
    const ro = new ResizeObserver(([e]) => setWide(e.contentRect.width >= PREVIEW_AT))
    ro.observe(el)
    onCleanup(() => ro.disconnect())
  }
  // Fetch the preview once the cursor settles, not for every row J passes.
  const [previewed, setPreviewed] = createSignal("")
  createEffect(() => {
    const t = wide() ? (view.hits[cursor()]?.title ?? "") : ""
    const timer = setTimeout(() => setPreviewed(t), 200)
    onCleanup(() => clearTimeout(timer))
  })
  const preview = useWikiPage(previewed)
  // With a preview, the first click shows a result and a second one opens it.
  const pick = (i: number) => {
    const h = view.hits[i]
    if (h && (!wide() || i === cursor())) void openArticle(h.title)
    setCursor(i)
  }

  useListCursor({
    count: () => view.hits.length,
    index: cursor,
    setIndex: setCursor,
    onOpen: (i) => {
      const h = view.hits[i]
      if (h) void openArticle(h.title)
    },
  })

  return (
    <Page>
      <Switch>
        <Match when={!view.enabled}>
          <WikiOff />
        </Match>
        <Match when>
          <Toolbar
            actions={
              <Show when={other()}>
                <Button size="sm" variant="ghost" icon="wiki" onClick={() => openArticle(mainPage(wiki()))}>
                  Main page
                </Button>
                <Button size="sm" variant="ghost" onClick={() => run("wiki.search", query())}>
                  Search the Roblox Wiki
                </Button>
              </Show>
            }
          >
            <TextField
              class="flex-1"
              icon="search"
              placeholder={other() ? `Search the ${source()}` : "Search items, experiences, players and more"}
              value={query()}
              onChange={(v) => {
                setQuery(v)
                setCursor(0)
              }}
              autofocus
            />
          </Toolbar>
          <div ref={measure} class="flex flex-col">
            <Switch>
              <Match when={!searched() && other()}>
                <EmptyState
                  icon="wiki"
                  title={`Search the ${source()}`}
                  description="An experience's own wiki on Fandom, written by its players."
                />
              </Match>
              <Match when={!searched()}>
                <EmptyState
                  icon="wiki"
                  title="Search the Roblox Wiki"
                  description="Articles about items, experiences, players, events and Roblox's history, written by the community on Fandom. Items also have a Wiki tab on their page when there is an article."
                />
              </Match>
              <Match when={view.hits.length > 0}>
                <SectionHeader size="group" title={`${view.hits.length} articles`} />
                <div class="flex items-start gap-4">
                  <div
                    class={wide() ? "flex w-104 shrink-0 flex-col" : "flex min-w-0 flex-1 flex-col"}
                    data-cursor-list
                  >
                    <VirtualList items={view.hits} activeIndex={cursor()} estimateRowHeight={72}>
                      {(h, i) => <HitRow hit={h} index={i} selected={i === cursor()} onClick={() => pick(i)} />}
                    </VirtualList>
                  </div>
                  <Show when={wide()}>
                    <aside class="sticky top-0 flex min-w-0 flex-1 flex-col" aria-label="Preview">
                      <Show when={previewed()}>
                        <WikiState view={preview}>
                          {(a) => <WikiSummary view={preview} article={a} facts={8} />}
                        </WikiState>
                      </Show>
                    </aside>
                  </Show>
                </div>
              </Match>
              <Match when={view.loading}>
                <div class="flex flex-col gap-2">
                  <For each={[0, 1, 2, 3]}>{() => <Skeleton class="h-12 w-full" />}</For>
                </div>
              </Match>
              <Match when={view.error}>
                {(e) => (
                  <Alert variant="danger" title="Couldn't search the wiki">
                    {e()}
                  </Alert>
                )}
              </Match>
              <Match when>
                <EmptyState icon="wiki" title="No articles found" description="Try another name or spelling." />
              </Match>
            </Switch>
          </div>
        </Match>
      </Switch>
    </Page>
  )
}

/** Results column plus a preview at least this wide, in px. */
const PREVIEW_AT = 880

function HitRow(props: { hit: WikiHit; index: number; selected: boolean; onClick: () => void }) {
  const edited = () => Date.parse(props.hit.edited)
  return (
    <ListItem
      data-index={props.index}
      selected={props.selected}
      leading={<Thumbnail class="w-8" alt="" fallback={props.hit.kind === "item" ? "marketplace" : "wiki"} />}
      title={
        <span class="flex items-center gap-2">
          {props.hit.name}
          <Badge>{kindLabel[props.hit.kind] ?? "Article"}</Badge>
        </span>
      }
      subtitle={props.hit.snippet}
      meta={
        <Show when={!Number.isNaN(edited())}>
          <span class="text-xs">edited {ago(edited())}</span>
        </Show>
      }
      onClick={props.onClick}
    />
  )
}

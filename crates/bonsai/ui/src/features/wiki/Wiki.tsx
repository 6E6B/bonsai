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
} from "~/lib/ui"
import { kindLabel, mainPage, ROBLOX_WIKI, type WikiSearchView } from "~/lib/wiki"
import { openArticle, useWikiPage, WikiOff } from "~/lib/wiki-panels"
import { sampleSearch } from "~/lib/wiki-sample"

/** Search the Roblox Wiki on Fandom, or with `?w=jailbreak` an experience's own wiki. Results
 * open in the article screen. */
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
    <Page
      title="Wiki"
      meta={<span class="text-sm text-fg-muted">{source()} · from Fandom</span>}
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
      <Switch>
        <Match when={!view.enabled}>
          <WikiOff />
        </Match>
        <Match when>
          <TextField
            icon="search"
            placeholder={other() ? `Search the ${source()}` : "Search items, experiences, players and more"}
            value={query()}
            onChange={(v) => {
              setQuery(v)
              setCursor(0)
            }}
            autofocus
          />
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
              <div class="flex flex-col" data-cursor-list>
                <SectionHeader size="group" title={`${view.hits.length} articles`} />
                <For each={view.hits}>{(h, i) => <HitRow hit={h} index={i()} selected={i() === cursor()} />}</For>
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
        </Match>
      </Switch>
    </Page>
  )
}

function HitRow(props: { hit: WikiHit; index: number; selected: boolean }) {
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
      onClick={() => void openArticle(props.hit.title)}
    />
  )
}

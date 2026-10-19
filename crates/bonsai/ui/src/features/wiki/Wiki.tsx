import { useSearchParams } from "@solidjs/router"
import { createEffect, createMemo, createSignal, For, on, Show } from "solid-js"
import { useListCursor } from "~/lib/commands/hooks"
import { ago } from "~/lib/format"
import { thumbUrl } from "~/lib/thumb"
import {
  Badge,
  Card,
  EmptyState,
  type IconName,
  ListItem,
  Page,
  SectionHeader,
  SegmentedControl,
  TextField,
  Thumbnail,
} from "~/lib/ui"
import { articles, kindLabel, plain, searchWiki, sourceOf, sources, type WikiArticle, type WikiKind } from "~/lib/wiki"
import { kindOptions, openArticle } from "~/lib/wiki-panels"

const fallbackIcon: Record<WikiKind, IconName> = {
  item: "marketplace",
  game: "game",
  player: "user",
  page: "wiki",
}

/** A picture for an article about something Bonsai can show. */
export function articleThumb(a: WikiArticle) {
  const t = a.target
  if (!t || a.source !== "roblox") return undefined
  if (t.kind === "item") return thumbUrl("asset", t.id)
  if (t.kind === "user") return thumbUrl("headshot", t.id)
  return undefined
}

/** Search across the Roblox Wiki and experience wikis. With no search, the articles by kind and
 * what changed lately. Prototype: articles come from sample data. */
export default function Wiki() {
  const [params] = useSearchParams()
  const [query, setQuery] = createSignal("")
  createEffect(
    on(
      () => params.q,
      (q) => setQuery(typeof q === "string" ? q : ""),
    ),
  )
  const [kind, setKind] = createSignal<WikiKind | "all">("all")
  const [cursor, setCursor] = createSignal(0)
  const results = createMemo(() => searchWiki(articles, query(), kind()))
  const browsing = () => !query().trim()
  useListCursor({
    count: () => (browsing() ? 0 : results().length),
    index: cursor,
    setIndex: setCursor,
    onOpen: (i) => {
      const a = results()[i]
      if (a) void openArticle(a)
    },
  })
  const recent = createMemo(() => [...articles].sort((a, b) => b.revised - a.revised).slice(0, 6))
  const byKind = (k: WikiKind) => articles.filter((a) => a.kind === k && a.source === "roblox")

  return (
    <Page
      title="Wiki"
      meta={<span class="text-sm text-fg-muted">{sources.map((s) => s.name).join(", ")} · from Fandom</span>}
    >
      <div class="flex flex-wrap items-center gap-2">
        <TextField
          class="min-w-64 flex-1"
          icon="search"
          placeholder="Search items, experiences, players and game pages"
          value={query()}
          onChange={(v) => {
            setQuery(v)
            setCursor(0)
          }}
          autofocus
        />
        <SegmentedControl
          aria-label="Article kind"
          options={kindOptions}
          value={kind()}
          onChange={(v) => setKind(v as WikiKind | "all")}
        />
      </div>

      <Show
        when={browsing()}
        fallback={
          <Show
            when={results().length > 0}
            fallback={<EmptyState icon="wiki" title="No articles found" description="Try another name or kind." />}
          >
            <div class="flex flex-col" data-cursor-list>
              <SectionHeader size="group" title={`${results().length} articles`} />
              <For each={results()}>{(a, i) => <ArticleRow article={a} selected={i() === cursor()} index={i()} />}</For>
            </div>
          </Show>
        }
      >
        <div class="grid gap-4 lg:grid-cols-3">
          <Column title="Items" kind="item" list={byKind("item")} />
          <Column title="Experiences" kind="game" list={byKind("game")} />
          <Column title="Players" kind="player" list={byKind("player")} />
        </div>
        <div class="flex flex-col">
          <SectionHeader title="Recently edited" size="section" />
          <For each={recent()}>{(a, i) => <ArticleRow article={a} index={i()} edited />}</For>
        </div>
      </Show>
    </Page>
  )
}

function Column(props: { title: string; kind: WikiKind; list: WikiArticle[] }) {
  return (
    <Card class="flex flex-col gap-1 p-2">
      <SectionHeader
        title={props.title}
        size="group"
        class="px-2 pt-1"
        meta={<span class="text-xs text-fg-muted">{props.list.length}</span>}
      />
      <For each={props.list}>
        {(a) => (
          <ListItem
            leading={<Thumbnail class="w-9" src={articleThumb(a)} alt="" fallback={fallbackIcon[a.kind]} />}
            title={a.title}
            subtitle={a.category}
            onClick={() => void openArticle(a)}
          />
        )}
      </For>
    </Card>
  )
}

function ArticleRow(props: { article: WikiArticle; selected?: boolean; index: number; edited?: boolean }) {
  return (
    <ListItem
      data-index={props.index}
      selected={props.selected}
      leading={
        <Thumbnail class="w-10" src={articleThumb(props.article)} alt="" fallback={fallbackIcon[props.article.kind]} />
      }
      title={
        <span class="flex items-center gap-2">
          {props.article.title}
          <Badge>{kindLabel[props.article.kind]}</Badge>
        </span>
      }
      subtitle={<span class="line-clamp-1">{plain(props.article.extract)}</span>}
      meta={
        <span class="flex flex-col items-end text-xs">
          <span>{sourceOf(props.article.source)?.name}</span>
          <Show when={props.edited}>
            <span>edited {ago(props.article.revised)}</span>
          </Show>
        </span>
      }
      onClick={() => void openArticle(props.article)}
    />
  )
}

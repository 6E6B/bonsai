import { createContext, createMemo, createSignal, For, Match, Show, Switch, useContext } from "solid-js"
import { commandContext, handle, run } from "~/lib/commands/registry"
import { ago } from "~/lib/format"
import {
  Alert,
  Badge,
  Button,
  Card,
  Clamp,
  EmptyState,
  Fact,
  FactGroup,
  Facts,
  Icon,
  ListItem,
  NavItem,
  showToast,
  TextField,
  TextLink,
} from "~/lib/ui"
import {
  articleAt,
  articleFor,
  articlePath,
  backlinks,
  gameWiki,
  kindLabel,
  parseLinks,
  parseTarget,
  plain,
  ROBLOX_WIKI,
  resolveLink,
  searchWiki,
  siteUrl,
  sourceOf,
  type WikiArticle,
  type WikiBlock,
  type WikiTarget,
} from "~/lib/wiki"

/** Opens an article in the Wiki screen. */
export const openArticle = (a: Pick<WikiArticle, "source" | "slug">) => run("wiki.open", articlePath(a).slice(6))

/** What following a `[[link]]` does. The experience Wiki tab opens its own pages in place. */
const LinkHandler = createContext<(a: WikiArticle) => void>((a) => void openArticle(a))

/** Plain text with its `[[links]]` live. Links to pages neither wiki has stay plain text. */
export function WikiText(props: { text: string; source: string }) {
  const follow = useContext(LinkHandler)
  return (
    <For each={parseLinks(props.text)}>
      {(s) => (
        <Show when={"link" in s && resolveLink(props.source, (s as { link: string }).link)} fallback={s.text}>
          {(a) => <TextLink onClick={() => follow(a())}>{s.text}</TextLink>}
        </Show>
      )}
    </For>
  )
}

export function WikiBlocks(props: { blocks: WikiBlock[]; source: string }) {
  return (
    <For each={props.blocks}>
      {(b) => (
        <Switch>
          <Match when={b.type === "p" && b}>
            {(p) => (
              <p>
                <WikiText text={p().text} source={props.source} />
              </p>
            )}
          </Match>
          <Match when={b.type === "list" && b}>
            {(l) => (
              <ul class="flex list-disc flex-col gap-1 pl-5">
                <For each={l().items}>
                  {(t) => (
                    <li>
                      <WikiText text={t} source={props.source} />
                    </li>
                  )}
                </For>
              </ul>
            )}
          </Match>
          <Match when={b.type === "table" && b}>
            {(t) => (
              <div class="overflow-x-auto rounded-md border border-line">
                <table class="w-full text-sm">
                  <thead class="bg-raised text-fg-muted">
                    <tr>
                      <For each={t().columns}>{(c) => <th class="px-3 py-1.5 text-left font-medium">{c}</th>}</For>
                    </tr>
                  </thead>
                  <tbody>
                    <For each={t().rows}>
                      {(row) => (
                        <tr class="border-t border-line">
                          <For each={row}>
                            {(cell) => (
                              <td class="px-3 py-1.5">
                                <WikiText text={cell} source={props.source} />
                              </td>
                            )}
                          </For>
                        </tr>
                      )}
                    </For>
                  </tbody>
                </table>
              </div>
            )}
          </Match>
          <Match when={b.type === "note" && b}>{(n) => <Alert title={n().text} />}</Match>
        </Switch>
      )}
    </For>
  )
}

/** The infobox beside an article: its facts, by group. */
export function WikiInfobox(props: { article: WikiArticle; class?: string }) {
  return (
    <Show when={props.article.infobox.length > 0}>
      <Facts class={props.class}>
        <For each={props.article.infobox}>
          {(g) => (
            <FactGroup title={g.title}>
              <For each={g.facts}>
                {(f) => (
                  <Fact label={f.label}>
                    <WikiText text={f.value} source={props.article.source} />
                  </Fact>
                )}
              </For>
            </FactGroup>
          )}
        </For>
      </Facts>
    </Show>
  )
}

/** Where the text came from, under every article and summary. CC BY-SA needs the source,
 * license and a link back. */
export function WikiAttribution(props: { article: WikiArticle; class?: string }) {
  const source = () => sourceOf(props.article.source)
  return (
    <p class={`flex flex-wrap items-center gap-x-1.5 text-xs text-fg-muted ${props.class ?? ""}`}>
      <Icon name="wiki" size={12} />
      <span>
        From {source()?.name ?? "the wiki"} · {source()?.license}
      </span>
      <span>·</span>
      <span>
        edited {ago(props.article.revised)} · {props.article.editors} editors
      </span>
      <span>·</span>
      <TextLink tone="muted" onClick={() => run("wiki.open_site", articlePath(props.article).slice(6))}>
        {source()?.host}
      </TextLink>
    </p>
  )
}

/** The lead and a few facts in a card, with a way into the full article. On item pages and
 * profiles. */
export function WikiSummary(props: { article: WikiArticle; facts?: number; class?: string }) {
  const facts = () => props.article.infobox.flatMap((g) => g.facts).slice(0, props.facts ?? 4)
  return (
    <Card class={`flex flex-col gap-3 p-4 ${props.class ?? ""}`}>
      <div class="flex items-center gap-2">
        <Icon name="wiki" class="text-fg-muted" />
        <span class="text-sm font-medium">{sourceOf(props.article.source)?.name}</span>
        <Badge>{props.article.category}</Badge>
        <div class="flex-1" />
        <Button
          size="sm"
          variant="ghost"
          icon="external"
          onClick={() => run("wiki.open_site", articlePath(props.article).slice(6))}
        >
          Fandom
        </Button>
        <Button size="sm" onClick={() => openArticle(props.article)}>
          Read article
        </Button>
      </div>
      <div class="grid gap-4 md:grid-cols-[minmax(0,1fr)_16rem]">
        <div class="flex min-w-0 flex-col gap-3">
          <Clamp lines={4}>
            <p>
              <WikiText text={props.article.extract} source={props.article.source} />
            </p>
          </Clamp>
          <Show when={props.article.sections.length > 0}>
            <div class="flex flex-wrap gap-1.5">
              <For each={props.article.sections}>
                {(s) => (
                  <Button
                    size="sm"
                    variant="secondary"
                    onClick={() => run("wiki.open", `${articlePath(props.article).slice(6)}#${s.id}`)}
                  >
                    {s.title}
                  </Button>
                )}
              </For>
            </div>
          </Show>
          <div class="flex-1" />
          <WikiAttribution article={props.article} />
        </div>
        <Show when={facts().length > 0}>
          <Facts class="h-fit">
            <FactGroup>
              <For each={facts()}>
                {(f) => (
                  <Fact label={f.label}>
                    <WikiText text={f.value} source={props.article.source} />
                  </Fact>
                )}
              </For>
            </FactGroup>
          </Facts>
        </Show>
      </div>
    </Card>
  )
}

/** A card section with the start of an article: under an experience's description. */
export function WikiTeaser(props: { article: WikiArticle }) {
  return (
    <section class="flex flex-col gap-1.5 px-3.5 py-3">
      <h3 class="flex items-center gap-1.5 text-sm font-medium text-fg-muted">
        <Icon name="wiki" size={14} />
        {sourceOf(props.article.source)?.name}
        <div class="flex-1" />
        <TextLink onClick={() => openArticle(props.article)}>Read article</TextLink>
      </h3>
      <Clamp lines={3}>
        <p>
          <WikiText text={props.article.extract} source={props.article.source} />
        </p>
      </Clamp>
    </section>
  )
}

/** A whole article: contents on the left, text in the middle, infobox on the right. `toc` off
 * for narrow places like the experience Wiki tab. The right column goes when it has nothing. */
export function WikiArticleView(props: {
  article: WikiArticle
  toc?: boolean
  /** Off where the article sits on the page it is about. */
  target?: boolean
}) {
  const follow = useContext(LinkHandler)
  const links = createMemo(() => backlinks(props.article))
  const aside = () =>
    Boolean(
      (props.target !== false && props.article.target && props.article.source === ROBLOX_WIKI) ||
        props.article.infobox.length > 0 ||
        links().length > 0,
    )
  const toc = () => props.toc !== false && props.article.sections.length > 1
  const cols = () =>
    aside()
      ? toc()
        ? "lg:grid-cols-[minmax(0,1fr)_17rem] xl:grid-cols-[11rem_minmax(0,1fr)_17rem]"
        : "lg:grid-cols-[minmax(0,1fr)_17rem]"
      : toc()
        ? "xl:grid-cols-[11rem_minmax(0,1fr)]"
        : ""
  const jump = (id: string) =>
    document.getElementById(`wiki-${id}`)?.scrollIntoView({ block: "start", behavior: "smooth" })
  return (
    <div class={`grid items-start gap-6 ${cols()}`}>
      <Show when={toc()}>
        <nav aria-label="Contents" class="sticky top-0 hidden flex-col gap-0.5 xl:flex">
          <span class="px-2 pb-1 text-xs font-medium text-fg-muted">Contents</span>
          <NavItem label="Overview" onClick={() => jump("top")} />
          <For each={props.article.sections}>{(s) => <NavItem label={s.title} onClick={() => jump(s.id)} />}</For>
        </nav>
      </Show>
      <article class="flex min-w-0 flex-col gap-3 leading-relaxed">
        <header id="wiki-top" class="flex scroll-mt-4 flex-col gap-1">
          <div class="flex items-center gap-2 text-sm text-fg-muted">
            <span>{sourceOf(props.article.source)?.name}</span>
            <span>›</span>
            <span>{props.article.category}</span>
          </div>
          <h1 class="text-2xl font-medium">{props.article.title}</h1>
        </header>
        <p>
          <WikiText text={props.article.extract} source={props.article.source} />
        </p>
        <For each={props.article.sections}>
          {(s) => (
            <section id={`wiki-${s.id}`} class="flex scroll-mt-4 flex-col gap-2 pt-2">
              <h2 class="border-b border-line pb-1 text-lg font-medium">{s.title}</h2>
              <WikiBlocks blocks={s.blocks} source={props.article.source} />
            </section>
          )}
        </For>
        <div class="flex flex-wrap items-center gap-1.5 border-t border-line pt-3">
          <For each={props.article.categories}>{(c) => <Badge>{c}</Badge>}</For>
        </div>
        <WikiAttribution article={props.article} />
      </article>
      <Show when={aside()}>
        <aside class="flex flex-col gap-3">
          <Show when={props.target !== false}>
            <TargetCard article={props.article} />
          </Show>
          <WikiInfobox article={props.article} />
          <Show when={links().length > 0}>
            <Facts>
              <FactGroup title="Linked from">
                <For each={links()}>
                  {(a) => (
                    <Fact label={kindLabel[a.kind]}>
                      <TextLink onClick={() => follow(a)}>{a.title}</TextLink>
                    </Fact>
                  )}
                </For>
              </FactGroup>
            </Facts>
          </Show>
        </aside>
      </Show>
    </div>
  )
}

const targetCommand = (t: WikiTarget): [string, string, string] =>
  t.kind === "game"
    ? ["game.open", String(t.placeId), "Open experience"]
    : t.kind === "item"
      ? ["market.open", String(t.id), "Open item"]
      : ["user.open", String(t.id), "Open profile"]

/** "This article is about a page in Bonsai": a button to it. */
function TargetCard(props: { article: WikiArticle }) {
  return (
    <Show when={props.article.target && props.article.source === "roblox" && props.article.target}>
      {(t) => {
        const [id, arg, label] = targetCommand(t())
        return (
          <Button
            variant="primary"
            icon={t().kind === "game" ? "game" : t().kind === "item" ? "marketplace" : "user"}
            onClick={() => run(id, arg)}
          >
            {label}
          </Button>
        )
      }}
    </Show>
  )
}

/** The Wiki tab of an experience: its fan wiki's pages on the left, the open one on the right.
 * Falls back to the Roblox Wiki article when the experience has no wiki of its own. */
export function GameWikiPanel(props: { placeId: number }) {
  const wiki = createMemo(() => gameWiki(props.placeId))
  const about = createMemo(() => articleFor({ kind: "game", placeId: props.placeId }))
  return (
    <Show
      when={wiki()}
      fallback={
        <Show
          when={about()}
          fallback={
            <EmptyState
              icon="wiki"
              title="No wiki for this experience"
              description="Neither the Roblox Wiki nor Fandom has pages about it yet."
            />
          }
        >
          {(a) => <WikiArticleView article={a()} toc={false} target={false} />}
        </Show>
      }
    >
      {(w) => <GameWikiBrowser source={w().source.key} pages={w().pages} about={about()} />}
    </Show>
  )
}

function GameWikiBrowser(props: { source: string; pages: WikiArticle[]; about?: WikiArticle }) {
  const [query, setQuery] = createSignal("")
  const main = () => props.pages.find((p) => p.target) ?? props.pages[0]
  const [open, setOpen] = createSignal<string>(main()?.slug ?? "")
  const shown = createMemo(() => searchWiki(props.pages, query()))
  /** The shown pages by category, the main page first on its own. */
  const groups = createMemo(() => {
    const by = new Map<string, WikiArticle[]>()
    for (const p of shown()) by.set(p.category, [...(by.get(p.category) ?? []), p])
    return [...by].sort(([, a], [, b]) => Number(b.includes(main())) - Number(a.includes(main())))
  })
  const page = () => props.pages.find((p) => p.slug === open()) ?? main()
  const source = () => sourceOf(props.source)
  const newest = () => Math.max(...props.pages.map((p) => p.revised))
  const follow = (a: WikiArticle) => (a.source === props.source ? setOpen(a.slug) : void openArticle(a))
  return (
    <LinkHandler.Provider value={follow}>
      <div class="flex flex-col gap-3">
        <div class="flex flex-wrap items-center gap-x-3 gap-y-1">
          <Icon name="wiki" class="text-fg-muted" />
          <span class="font-medium">{source()?.name}</span>
          <span class="text-sm text-fg-muted">
            {props.pages.length} pages · updated {ago(newest())}
          </span>
          <div class="flex-1" />
          <Show when={props.about}>
            {(a) => (
              <Button size="sm" variant="ghost" icon="wiki" onClick={() => openArticle(a())}>
                Roblox Wiki article
              </Button>
            )}
          </Show>
          <Button
            size="sm"
            variant="ghost"
            icon="external"
            onClick={() => run("wiki.open_site", `${props.source}/${page()?.slug ?? ""}`)}
          >
            {source()?.host}
          </Button>
        </div>
        <div class="grid items-start gap-6 md:grid-cols-[15rem_minmax(0,1fr)]">
          <nav aria-label="Wiki pages" class="flex flex-col gap-2">
            <TextField icon="search" size="sm" placeholder="Search this wiki" value={query()} onChange={setQuery} />
            <For each={groups()} fallback={<p class="px-2 py-3 text-sm text-fg-muted">No pages match.</p>}>
              {([category, list]) => (
                <div class="flex flex-col gap-0.5">
                  <Show when={!list.includes(main())}>
                    <span class="px-2 pt-1 text-xs font-medium text-fg-muted">{category}</span>
                  </Show>
                  <For each={list}>
                    {(p) => (
                      <NavItem label={p.title} active={p.slug === page()?.slug} onClick={() => setOpen(p.slug)} />
                    )}
                  </For>
                </div>
              )}
            </For>
          </nav>
          <Show when={page()}>
            {(p) => (
              <Show when={p() !== main()} fallback={<WikiPortal main={p()} pages={props.pages} onOpen={setOpen} />}>
                <WikiArticleView article={p()} toc={false} />
              </Show>
            )}
          </Show>
        </div>
      </div>
    </LinkHandler.Provider>
  )
}

/** An experience wiki's front page: the welcome text, then every page by category and what
 * changed lately. */
function WikiPortal(props: { main: WikiArticle; pages: WikiArticle[]; onOpen: (slug: string) => void }) {
  const rest = () => props.pages.filter((p) => p !== props.main)
  const categories = createMemo(() => [...new Set(rest().map((p) => p.category))])
  const recent = createMemo(() => [...rest()].sort((a, b) => b.revised - a.revised).slice(0, 4))
  return (
    <div class="flex min-w-0 flex-col gap-4">
      <div class="flex flex-col gap-1">
        <h1 class="text-2xl font-medium">{props.main.title}</h1>
        <p class="leading-relaxed">
          <WikiText text={props.main.extract} source={props.main.source} />
        </p>
      </div>
      <div class="grid grid-cols-[repeat(auto-fill,minmax(14rem,1fr))] gap-3">
        <For each={categories()}>
          {(c) => (
            <Card class="flex flex-col gap-1 p-3">
              <span class="text-xs font-medium text-fg-muted">{c}</span>
              <For each={rest().filter((p) => p.category === c)}>
                {(p) => (
                  <div class="flex flex-col">
                    <TextLink onClick={() => props.onOpen(p.slug)}>{p.title}</TextLink>
                    <span class="line-clamp-2 text-sm text-fg-muted">{plain(p.extract)}</span>
                  </div>
                )}
              </For>
            </Card>
          )}
        </For>
      </div>
      <div class="flex flex-col">
        <span class="px-1 pb-1 text-xs font-medium text-fg-muted">Recently edited</span>
        <For each={recent()}>
          {(p) => (
            <ListItem
              title={p.title}
              subtitle={p.category}
              meta={<span class="text-xs">edited {ago(p.revised)}</span>}
              onClick={() => props.onOpen(p.slug)}
            />
          )}
        </For>
      </div>
      <WikiAttribution article={props.main} />
    </div>
  )
}

/** Which wiki and kind of article to look in, for the Wiki screen's filters. */
export const kindOptions = [
  { value: "all", label: "All" },
  { value: "item", label: "Items" },
  { value: "game", label: "Experiences" },
  { value: "player", label: "Players" },
  { value: "page", label: "Other" },
]

/** `source/slug`, with an optional `#section`, → the article, or undefined. */
function lookup(arg: string | null) {
  const [path] = (arg ?? "").trim().split("#")
  const [source, ...rest] = path.split("/")
  return rest.length ? articleAt(source, rest.join("/")) : undefined
}

/** Handlers for the `wiki.*` commands. They only navigate, so they all live in the UI. */
export function registerWikiCommands(opts: { visit: (path: string) => void }) {
  const search = (q: string) => opts.visit(q ? `/wiki?q=${encodeURIComponent(q)}` : "/wiki")
  return [
    handle("wiki.open", (arg) => {
      const a = lookup(arg)
      if (a) opts.visit(`/wiki/${(arg ?? "").trim()}`)
      else search((arg ?? "").replace(/^[^/]*\//, "").replace(/_/g, " "))
    }),
    handle("wiki.search", (q) => search(q?.trim() ?? "")),
    handle("wiki.article", () => {
      const t = parseTarget(commandContext())
      const a = t && articleFor(t)
      if (a) opts.visit(articlePath(a))
      else showToast(t ? "The Roblox Wiki has no article about this." : "Open an item, experience or profile first.")
    }),
    handle("wiki.open_site", (arg) => {
      const ctx = commandContext()
      const a = lookup(arg) ?? (ctx?.startsWith("wiki:") ? lookup(ctx.slice(5)) : undefined)
      if (!a) return showToast("Open a wiki article first.")
      // Prototype: the app would hand this to the system browser from Rust.
      showToast(`Would open ${siteUrl(a)} in your browser`)
    }),
  ]
}

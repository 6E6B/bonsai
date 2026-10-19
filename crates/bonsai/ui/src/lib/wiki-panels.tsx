import { isTauri } from "@tauri-apps/api/core"
import { createContext, createEffect, createSignal, For, type JSX, Match, Show, Switch, useContext } from "solid-js"
import { commandContext, handle, run } from "~/lib/commands/registry"
import { commands } from "~/lib/ipc/bindings"
import { useLive } from "~/lib/store"
import { wikiImageUrl } from "~/lib/thumb"
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
  Figure,
  Icon,
  NavItem,
  SegmentedControl,
  Skeleton,
  showToast,
  TextLink,
} from "~/lib/ui"
import {
  type AboutKind,
  articlePath,
  kindLabel,
  knownArticle,
  linkTarget,
  listDepth,
  mainPage,
  parseLinks,
  parseTarget,
  plain,
  ROBLOX_WIKI,
  rememberArticle,
  searchPath,
  splitArg,
  splitSearch,
  type WikiArticle,
  type WikiBlock,
  type WikiImage,
  type WikiLink,
  type WikiPageView,
  type WikiTarget,
} from "~/lib/wiki"
import { sampleAbout, samplePage } from "~/lib/wiki-sample"

/** Opens an article, by its wiki title, in the Wiki screen. */
export const openArticle = (title: string, section?: string) =>
  run("wiki.open", section ? `${title}#${section}` : title)

const waiting = (): WikiPageView => ({
  enabled: true,
  loading: true,
  error: null,
  fetchedAt: null,
  article: null,
  source: "Roblox Wiki",
  license: "CC BY-SA 3.0",
})

/** The article called `title`. In a browser preview, a made-up one. */
export function useWikiPage(title: () => string) {
  return useLive(
    title,
    (t) => commands.wikiPage(t),
    () => (isTauri() ? waiting() : samplePage(title())),
  )
}

/** The article about an item, experience or player, if the wiki has one about this very one.
 * `null`, or no name yet, asks for nothing. */
/** What a page is: its kind, id and name, and for an experience who made it (the wiki files
 * experiences under their group or player). */
export type About = { kind: AboutKind; id: number; name: string; creator?: string; group?: boolean }

export function useWikiAbout(about: () => About | null) {
  const key = () => {
    const a = about()
    return a?.name ? JSON.stringify([a.kind, a.id, a.name, a.creator ?? "", a.group ?? false]) : ""
  }
  const view = useLive(
    key,
    (k) => {
      if (!k) return Promise.resolve({ ...waiting(), loading: false })
      const [kind, id, name, creator, group] = JSON.parse(k) as [AboutKind, number, string, string, boolean]
      return commands.wikiAbout(kind, id, name, creator, group)
    },
    () => {
      const a = about()
      return isTauri() || !a ? waiting() : sampleAbout(a.kind, a.id)
    },
  )
  createEffect(() => {
    const a = about()
    const title = view.article?.title
    if (!a || !title) return
    rememberArticle(
      a.kind === "game" ? { kind: "game", placeId: a.id } : { kind: a.kind === "user" ? "user" : "item", id: a.id },
      title,
    )
  })
  return view
}

/** The article the text on screen is from. Links to other sites open only from the article
 * that has them, so `wiki.open_link` is told which one. */
const ArticleTitle = createContext("")

/** Opens what a `[[link]]` points to: an article here, a Roblox page in Bonsai, or another site
 * in the browser. */
function follow(link: string, article: string) {
  const t = linkTarget(link)
  if (t.kind === "article") void openArticle(t.title)
  else if (t.kind === "roblox") void run(t.command, t.arg)
  else void run("wiki.open_link", `${article}\n${t.url}`)
}

/** Plain text with its `[[links]]` live. Every article link points at an existing page: the
 * Rust side turns links to missing pages into plain text. Links to other sites get an arrow. */
export function WikiText(props: { text: string }) {
  const article = useContext(ArticleTitle)
  return (
    <For each={parseLinks(props.text)}>
      {(s) => (
        <Show when={"link" in s && (s as { link: string }).link} fallback={s.text}>
          {(link) => (
            <TextLink onClick={() => follow(link(), article)}>
              {s.text}
              <Show when={linkTarget(link()).kind === "web"}>
                <Icon name="external" size={11} class="ml-0.5 inline align-baseline" />
              </Show>
            </TextLink>
          )}
        </Show>
      )}
    </For>
  )
}

/** Text from `article`, so its links to other sites open. */
function InArticle(props: { article: WikiArticle; children: JSX.Element }) {
  return <ArticleTitle.Provider value={props.article.title}>{props.children}</ArticleTitle.Provider>
}

function WikiPicture(props: { image: WikiImage; class?: string }) {
  return (
    <Figure
      class={props.class}
      src={wikiImageUrl(props.image.src)}
      alt={plain(props.image.caption)}
      width={props.image.width || undefined}
      height={props.image.height || undefined}
      caption={props.image.caption ? <WikiText text={props.image.caption} /> : undefined}
    />
  )
}

/** A picture at the size the article shows it, or a gallery in a grid. */
export function WikiPictures(props: { images: WikiImage[] }) {
  return (
    <Show
      when={props.images.length > 1}
      fallback={
        <Show when={props.images[0]}>
          {(i) => (
            <div class="max-w-full" style={{ width: `${Math.min(i().width || 320, 400)}px` }}>
              <WikiPicture image={i()} />
            </div>
          )}
        </Show>
      }
    >
      <div class="grid grid-cols-[repeat(auto-fill,minmax(7.5rem,1fr))] gap-3">
        <For each={props.images}>{(i) => <WikiPicture image={i} />}</For>
      </div>
    </Show>
  )
}

function ListItems(props: { items: string[] }) {
  return (
    <For each={props.items}>
      {(raw) => {
        const item = listDepth(raw)
        return (
          <li class={item.depth === 1 ? "ml-5" : item.depth > 1 ? "ml-10" : undefined}>
            <WikiText text={item.text} />
          </li>
        )
      }}
    </For>
  )
}

export function WikiBlocks(props: { blocks: WikiBlock[] }) {
  return (
    <For each={props.blocks}>
      {(b) => (
        <Switch>
          <Match when={b.type === "p" && b}>
            {(p) => (
              <p>
                <WikiText text={p().text} />
              </p>
            )}
          </Match>
          <Match when={b.type === "list" && b}>
            {(l) => (
              <Show
                when={l().ordered}
                fallback={
                  <ul class="flex list-disc flex-col gap-1 pl-5">
                    <ListItems items={l().items} />
                  </ul>
                }
              >
                <ol class="flex list-decimal flex-col gap-1 pl-5">
                  <ListItems items={l().items} />
                </ol>
              </Show>
            )}
          </Match>
          <Match when={b.type === "table" && b}>
            {(t) => (
              <div class="overflow-x-auto rounded-md border border-line">
                <table class="w-full text-sm">
                  <Show when={t().columns.length > 0}>
                    <thead class="bg-raised text-fg-muted">
                      <tr>
                        <For each={t().columns}>
                          {(c) => (
                            <th class="px-3 py-1.5 text-left font-medium">
                              <WikiText text={c} />
                            </th>
                          )}
                        </For>
                      </tr>
                    </thead>
                  </Show>
                  <tbody>
                    <For each={t().rows}>
                      {(row, i) => (
                        <tr class={i() > 0 || t().columns.length > 0 ? "border-t border-line" : undefined}>
                          <For each={row}>
                            {(cell) => (
                              <td class="px-3 py-1.5 align-top">
                                <WikiText text={cell} />
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
          <Match when={b.type === "note" && b}>
            {(n) => (
              <p class="rounded-md border border-line bg-raised px-3 py-2 text-sm text-fg-muted">
                <WikiText text={n().text} />
              </p>
            )}
          </Match>
          <Match when={b.type === "images" && b}>{(g) => <WikiPictures images={g().images} />}</Match>
        </Switch>
      )}
    </For>
  )
}

/** The infobox beside an article: its picture and every field the wiki has, as it has it. */
export function WikiInfobox(props: { article: WikiArticle; class?: string }) {
  return (
    <Show when={props.article.infobox.length > 0 || props.article.image}>
      <div class={`flex flex-col gap-3 ${props.class ?? ""}`}>
        <Show when={props.article.image}>{(i) => <WikiPicture image={i()} />}</Show>
        <Show when={props.article.infobox.length > 0}>
          <Facts>
            <FactGroup title={props.article.infoboxTitle || undefined}>
              <For each={props.article.infobox}>
                {(f) => (
                  <Fact label={f.label || f.key}>
                    <WikiText text={f.value} />
                  </Fact>
                )}
              </For>
            </FactGroup>
          </Facts>
        </Show>
      </div>
    </Show>
  )
}

/** Wikis of their own the article points to, like an experience's, with a way into each. */
function OtherWikis(props: { wikis: WikiLink[] }) {
  return (
    <For each={props.wikis}>
      {(w) => (
        <Card class="flex flex-col gap-2 p-3">
          <span class="flex items-center gap-1.5 text-sm font-medium">
            <Icon name="wiki" size={14} class="text-fg-muted" />
            {w.name}
          </span>
          <span class="text-xs text-fg-muted">A wiki of its own on Fandom.</span>
          <div class="flex gap-1.5">
            <Button size="sm" onClick={() => openArticle(w.title)}>
              Open
            </Button>
            <Button size="sm" variant="secondary" icon="search" onClick={() => run("wiki.search", w.title)}>
              Search
            </Button>
          </div>
        </Card>
      )}
    </For>
  )
}

/** Where the text came from, under every article and summary. CC BY-SA needs the source,
 * license and a link back. */
export function WikiAttribution(props: { view: WikiPageView; article: WikiArticle; class?: string }) {
  return (
    <p class={`flex flex-wrap items-center gap-x-1.5 text-xs text-fg-muted ${props.class ?? ""}`}>
      <Icon name="wiki" size={12} />
      <span>
        From the {props.article.source} on Fandom · {props.view.license}
      </span>
      <span>·</span>
      <TextLink tone="muted" onClick={() => run("wiki.open_site", props.article.title)}>
        {props.article.url.replace(/^https:\/\//, "")}
      </TextLink>
    </p>
  )
}

/** The lead and a few facts in a card, with a way into the full article. On profiles. */
export function WikiSummary(props: { view: WikiPageView; article: WikiArticle; facts?: number; class?: string }) {
  const facts = () => props.article.infobox.slice(0, props.facts ?? 4)
  return (
    <InArticle article={props.article}>
      <Card class={`flex flex-col gap-3 p-4 ${props.class ?? ""}`}>
        <div class="flex items-center gap-2">
          <Icon name="wiki" class="text-fg-muted" />
          <span class="text-sm font-medium">{props.article.source}</span>
          <Show when={props.article.categories[0]}>{(c) => <Badge>{c()}</Badge>}</Show>
          <div class="flex-1" />
          <Button size="sm" variant="ghost" icon="external" onClick={() => run("wiki.open_site", props.article.title)}>
            Fandom
          </Button>
          <Button size="sm" onClick={() => openArticle(props.article.title)}>
            Read article
          </Button>
        </div>
        <div class="grid gap-4 md:grid-cols-[minmax(0,1fr)_16rem]">
          <div class="flex min-w-0 flex-col gap-3">
            <Clamp lines={4}>
              <p>
                <WikiText text={props.article.extract} />
              </p>
            </Clamp>
            <Show when={props.article.sections.length > 0}>
              <div class="flex flex-wrap gap-1.5">
                <For each={props.article.sections.filter((s) => s.level <= 2)}>
                  {(s) => (
                    <Button size="sm" variant="secondary" onClick={() => openArticle(props.article.title, s.id)}>
                      {s.title}
                    </Button>
                  )}
                </For>
              </div>
            </Show>
            <div class="flex-1" />
            <WikiAttribution view={props.view} article={props.article} />
          </div>
          <Show when={facts().length > 0}>
            <Facts class="h-fit">
              <FactGroup>
                <For each={facts()}>
                  {(f) => (
                    <Fact label={f.label || f.key}>
                      <WikiText text={f.value} />
                    </Fact>
                  )}
                </For>
              </FactGroup>
            </Facts>
          </Show>
        </div>
      </Card>
    </InArticle>
  )
}

/** A card section with the start of an article: under an experience's description. */
export function WikiTeaser(props: { view: WikiPageView; article: WikiArticle }) {
  return (
    <InArticle article={props.article}>
      <section class="flex flex-col gap-1.5 px-3.5 py-3">
        <h3 class="flex items-center gap-1.5 text-sm font-medium text-fg-muted">
          <Icon name="wiki" size={14} />
          {props.article.source}
          <div class="flex-1" />
          <For each={props.article.wikis}>
            {(w) => <TextLink onClick={() => openArticle(w.title)}>{w.name}</TextLink>}
          </For>
          <TextLink onClick={() => openArticle(props.article.title)}>Read article</TextLink>
        </h3>
        <Clamp lines={3}>
          <p>
            <WikiText text={props.article.extract} />
          </p>
        </Clamp>
      </section>
    </InArticle>
  )
}

/** Shown wherever the wiki is off: what it is and a way to turn it on. */
export function WikiOff() {
  return (
    <EmptyState
      icon="wiki"
      title="Wiki articles are off"
      description="Turn them on to read Roblox Wiki articles from Fandom, a third-party site. An article is fetched only when you open a page it could be about."
    >
      <Button onClick={() => run("wiki.toggle")}>Turn on</Button>
    </EmptyState>
  )
}

/**
 * What a wiki view shows before its article: a way to turn the wiki on, a loading state, the
 * error, or that there is no such article. `children` gets the article.
 */
export function WikiState(props: { view: WikiPageView; missing?: string; children: (a: WikiArticle) => JSX.Element }) {
  return (
    <Switch>
      <Match when={!props.view.enabled}>
        <WikiOff />
      </Match>
      <Match when={props.view.article}>{(a) => props.children(a())}</Match>
      <Match when={props.view.loading}>
        <div class="flex flex-col gap-3">
          <Skeleton class="h-8 w-1/3" />
          <Skeleton class="h-4 w-full" />
          <Skeleton class="h-4 w-5/6" />
          <Skeleton class="h-4 w-2/3" />
        </div>
      </Match>
      <Match when={props.view.error}>
        {(e) => (
          <Alert variant="danger" title="Couldn't load the article">
            {e()}
          </Alert>
        )}
      </Match>
      <Match when>
        <EmptyState
          icon="wiki"
          title="No such article"
          description={props.missing ?? "The wiki has no page with this name."}
        >
          <Button onClick={() => run("nav.wiki")}>Search the wiki</Button>
        </EmptyState>
      </Match>
    </Switch>
  )
}

/** A whole article: contents on the left, text in the middle, infobox on the right. `toc` off
 * for narrow places like the Wiki tabs. The right column goes when it has nothing. */
export function WikiArticleView(props: {
  view: WikiPageView
  article: WikiArticle
  toc?: boolean
  /** Off where the article sits on the page it is about. */
  target?: boolean
}) {
  const aside = () =>
    Boolean(
      (props.target !== false && props.article.target) ||
        props.article.infobox.length > 0 ||
        props.article.image ||
        props.article.wikis.length > 0,
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
    <InArticle article={props.article}>
      <div class={`grid items-start gap-6 ${cols()}`}>
        <Show when={toc()}>
          <nav aria-label="Contents" class="sticky top-0 hidden flex-col gap-0.5 xl:flex">
            <span class="px-2 pb-1 text-xs font-medium text-fg-muted">Contents</span>
            <NavItem label="Overview" onClick={() => jump("top")} />
            <For each={props.article.sections.filter((s) => s.level <= 2)}>
              {(s) => <NavItem label={s.title} onClick={() => jump(s.id)} />}
            </For>
          </nav>
        </Show>
        <article class="flex min-w-0 flex-col gap-3 leading-relaxed">
          <header id="wiki-top" class="flex scroll-mt-4 flex-col gap-1">
            <div class="flex items-center gap-2 text-sm text-fg-muted">
              <span>{props.article.source}</span>
              <span>›</span>
              <span>{props.article.categories[0] ?? kindLabel[props.article.kind]}</span>
              <div class="flex-1" />
              <Show when={props.article.wiki !== ROBLOX_WIKI}>
                <Button
                  size="sm"
                  variant="ghost"
                  icon="search"
                  onClick={() => run("wiki.search", mainPage(props.article.wiki))}
                >
                  Search {props.article.source}
                </Button>
              </Show>
              <Button
                size="sm"
                variant="ghost"
                icon="external"
                onClick={() => run("wiki.open_site", props.article.title)}
              >
                Open on Fandom
              </Button>
            </div>
            <h1 class="text-2xl font-medium">{props.article.name}</h1>
          </header>
          <WikiBlocks blocks={props.article.lead} />
          <For each={props.article.sections}>
            {(s) => (
              <section id={`wiki-${s.id}`} class="flex scroll-mt-4 flex-col gap-2 pt-2">
                <Show when={s.level <= 2} fallback={<h3 class="font-medium">{s.title}</h3>}>
                  <h2 class="border-b border-line pb-1 text-lg font-medium">{s.title}</h2>
                </Show>
                <WikiBlocks blocks={s.blocks} />
              </section>
            )}
          </For>
          <Show when={props.article.categories.length > 0}>
            <div class="flex flex-wrap items-center gap-1.5 border-t border-line pt-3">
              <For each={props.article.categories}>{(c) => <Badge>{c}</Badge>}</For>
            </div>
          </Show>
          <WikiAttribution view={props.view} article={props.article} />
        </article>
        <Show when={aside()}>
          <aside class="flex flex-col gap-3">
            <Show when={props.target !== false && props.article.target}>{(t) => <TargetButton target={t()} />}</Show>
            <WikiInfobox article={props.article} />
            <OtherWikis wikis={props.article.wikis} />
          </aside>
        </Show>
      </div>
    </InArticle>
  )
}

const targetCommand = (t: WikiTarget): [string, string, string] =>
  t.kind === "game"
    ? ["game.open", String(t.placeId), "Open experience"]
    : t.kind === "item"
      ? ["market.open", String(t.id), "Open item"]
      : ["user.open", String(t.id), "Open profile"]

/** "This article is about a page in Bonsai": a button to it, from the roblox.com link in it. */
function TargetButton(props: { target: WikiTarget }) {
  const c = () => targetCommand(props.target)
  return (
    <Button
      variant="primary"
      icon={props.target.kind === "game" ? "game" : props.target.kind === "item" ? "marketplace" : "user"}
      onClick={() => run(c()[0], c()[1])}
    >
      {c()[2]}
    </Button>
  )
}

/** The Wiki tab of an item, experience or profile: the article about it, in full. When the
 * article points to a wiki of the experience's own, a switch shows that wiki's main page in
 * its place. */
export function WikiTab(props: { view: WikiPageView; missing: string }) {
  const [shown, setShown] = createSignal(ROBLOX_WIKI)
  const others = () => props.view.article?.wikis ?? []
  const other = useWikiPage(() => (shown() === ROBLOX_WIKI ? "" : mainPage(shown())))
  return (
    <div class="flex flex-col gap-4">
      <Show when={others().length > 0}>
        <SegmentedControl
          class="self-start"
          size="sm"
          aria-label="Wiki"
          value={shown()}
          onChange={setShown}
          options={[
            { value: ROBLOX_WIKI, label: props.view.article?.source ?? props.view.source },
            ...others().map((w) => ({ value: w.wiki, label: w.name })),
          ]}
        />
      </Show>
      <Show
        when={shown() !== ROBLOX_WIKI}
        fallback={
          <WikiState view={props.view} missing={props.missing}>
            {(a) => <WikiArticleView view={props.view} article={a} toc={false} target={false} />}
          </WikiState>
        }
      >
        <WikiState view={other} missing="The wiki has no main page.">
          {(a) => <WikiArticleView view={other} article={a} toc={false} target={false} />}
        </WikiState>
      </Show>
    </div>
  )
}

/** Handlers for the navigating `wiki.*` commands. `wiki.open_site` and `wiki.toggle` go to
 * Rust, which opens the browser and changes the setting. */
export function registerWikiCommands(opts: { visit: (path: string) => void }) {
  return [
    handle("wiki.open", (arg) => {
      const { title, section } = splitArg(arg)
      if (title) opts.visit(articlePath(title, section))
      else opts.visit(searchPath(""))
    }),
    // `w:c:jailbreak: torpedo` searches the Jailbreak Wiki.
    handle("wiki.search", (arg) => {
      const { wiki, query } = splitSearch(arg)
      opts.visit(searchPath(query, wiki))
    }),
    handle("wiki.article", () => {
      const t = parseTarget(commandContext())
      const title = t && knownArticle(t)
      if (title) opts.visit(articlePath(title))
      else showToast(t ? "The Roblox Wiki has no article about this." : "Open an item, experience or profile first.")
    }),
  ]
}

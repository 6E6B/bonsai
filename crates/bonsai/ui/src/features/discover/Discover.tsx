import { createMemo, createSignal, For, Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import { type ChartPage, commands, type DiscoverView, type GameCard } from "~/lib/ipc/bindings"
import { shell, useLive } from "~/lib/store"
import { GameTile, TILE_WIDTH, tileGrid } from "~/lib/tiles"
import {
  Alert,
  Button,
  EmptyState,
  IconButton,
  Page,
  SectionHeader,
  Select,
  Skeleton,
  Switch,
  Tooltip,
  VirtualGrid,
} from "~/lib/ui"

const ALL_GENRES = "All genres"
/** Games a chart shows before "See all". */
const PREVIEW = 12

/** Roblox's Charts with its device and location filters. Genre narrows what's loaded;
 * sponsored games are hidden unless asked for. */
export default function Discover() {
  const [device, setDevice] = createSignal("")
  const [country, setCountry] = createSignal("")
  const [genre, setGenre] = createSignal(ALL_GENRES)
  const [sponsored, setSponsored] = createSignal(false)
  const [open, setOpen] = createSignal<string | null>(null)
  const view = useLive(
    () => [shell.active, device(), country()] as const,
    ([, d, c]) => commands.discover(d, c),
    (): DiscoverView => ({ loading: true, error: null, filters: [], charts: [], more: false, moreKey: "" }),
  )
  const keep = (g: GameCard) => (sponsored() || !g.sponsored) && (genre() === ALL_GENRES || g.genre === genre())
  const genres = createMemo(() => {
    const all = new Set<string>()
    for (const c of view.charts) for (const g of c.games) if (g.genre) all.add(g.genre)
    return [ALL_GENRES, ...[...all].sort()]
  })
  const filter = (kind: string) => view.filters.find((f) => f.kind === kind)

  return (
    <Page
      title="Discover"
      class="!gap-3"
      actions={
        <Tooltip content="Search experiences in the command bar with g:">
          <Button icon="search" onClick={() => run("palette.open")}>
            Search
          </Button>
        </Tooltip>
      }
    >
      <div class="flex flex-wrap items-center gap-2">
        <Show when={filter("device")}>
          {(f) => (
            <Select
              size="sm"
              aria-label={f().title}
              options={f().options}
              value={device() || f().selected}
              onChange={setDevice}
            />
          )}
        </Show>
        <Show when={filter("country")}>
          {(f) => (
            <Select
              size="sm"
              aria-label={f().title}
              options={f().options}
              value={country() || f().selected}
              onChange={setCountry}
            />
          )}
        </Show>
        <Select size="sm" aria-label="Genre" options={genres()} value={genre()} onChange={setGenre} />
        <Switch checked={sponsored()} onChange={setSponsored}>
          Show sponsored
        </Switch>
      </div>
      <Show
        when={open()}
        fallback={
          <Charts
            view={view}
            keep={keep}
            onOpen={(id) => {
              setOpen(id)
            }}
          />
        }
      >
        {(id) => <FullChart id={id()} device={device()} country={country()} keep={keep} onBack={() => setOpen(null)} />}
      </Show>
    </Page>
  )
}

function Charts(props: { view: DiscoverView; keep: (g: GameCard) => boolean; onOpen: (id: string) => void }) {
  return (
    <Show
      when={props.view.charts.length > 0}
      fallback={
        <Show
          when={!props.view.loading}
          fallback={<For each={[0, 1, 2]}>{() => <Skeleton class="h-48 w-full rounded-md" />}</For>}
        >
          <Show when={props.view.error} fallback={<EmptyState icon="discover" title="No charts" />}>
            {(e) => (
              <Alert variant="danger" title="Couldn't load charts">
                {e()}
              </Alert>
            )}
          </Show>
        </Show>
      }
    >
      <div class="flex flex-col gap-6">
        <For each={props.view.charts}>
          {(c) => {
            const games = () => c.games.filter(props.keep)
            return (
              <Show when={games().length > 0}>
                <section class="flex flex-col gap-2">
                  <SectionHeader
                    title={c.title}
                    meta={c.subtitle || undefined}
                    actions={
                      <Button size="sm" variant="ghost" onClick={() => props.onOpen(c.id)}>
                        See all
                      </Button>
                    }
                  />
                  <div class={tileGrid}>
                    <For each={games().slice(0, PREVIEW)}>{(g) => <GameTile game={g} />}</For>
                  </div>
                </section>
              </Show>
            )
          }}
        </For>
        <Show when={props.view.more}>
          <Button
            class="self-start"
            variant="ghost"
            disabled={props.view.loading}
            onClick={() => run("list.more", props.view.moreKey)}
          >
            More charts
          </Button>
        </Show>
      </div>
    </Show>
  )
}

function FullChart(props: {
  id: string
  device: string
  country: string
  keep: (g: GameCard) => boolean
  onBack: () => void
}) {
  const page = useLive(
    () => [props.id, props.device, props.country] as const,
    ([i, d, c]) => commands.discoverChart(i, d, c),
    (): ChartPage => ({ loading: true, error: null, chart: null }),
  )
  const games = () => page.chart?.games.filter(props.keep) ?? []
  return (
    <div class="flex flex-col gap-3">
      <div class="flex items-center gap-2">
        <IconButton class="-ml-1.5" icon="chevron-left" label="Back to charts" onClick={() => props.onBack()} />
        <h2 class="truncate text-lg font-semibold">{page.chart?.title ?? "…"}</h2>
      </div>
      <Show when={page.chart?.info}>{(i) => <p class="max-w-prose text-sm text-fg-muted">{i()}</p>}</Show>
      <Show when={!page.error} fallback={<span class="text-fg-muted">{page.error}</span>}>
        <VirtualGrid
          items={games()}
          minItemWidth={TILE_WIDTH}
          estimateRowHeight={170}
          gap={12}
          empty={
            <Show when={!page.loading} fallback={<Skeleton class="h-48 w-full rounded-md" />}>
              <EmptyState icon="search" title="No experiences match" description="Try another genre." />
            </Show>
          }
        >
          {(g) => <GameTile game={g} />}
        </VirtualGrid>
      </Show>
    </div>
  )
}

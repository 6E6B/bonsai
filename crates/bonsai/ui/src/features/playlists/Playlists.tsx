import { useParams } from "@solidjs/router"
import { createEffect, createMemo, createSignal, For, on, Show } from "solid-js"
import { useCommandContext, useListCursor } from "~/lib/commands/hooks"
import { menuItem, run } from "~/lib/commands/registry"
import type { GameCard, PlaylistView } from "~/lib/ipc/bindings"
import { playlists } from "~/lib/store"
import { thumbUrl } from "~/lib/thumb"
import { GameTile, TILE_WIDTH } from "~/lib/tiles"
import {
  Button,
  EmptyState,
  IconButton,
  ListItem,
  Menu,
  Page,
  TextField,
  Thumbnail,
  Tooltip,
  VirtualGrid,
} from "~/lib/ui"
import { searchGames } from "./filter"

const experiences = (n: number) => `${n} ${n === 1 ? "experience" : "experiences"}`

/** The first game's icon, or a 2×2 mosaic once there are four. */
function Cover(props: { games: GameCard[]; class: string }) {
  const icon = (g: GameCard | undefined) => (g ? thumbUrl("game_icon", g.universeId) : undefined)
  return (
    <Show
      when={props.games.length >= 4}
      fallback={<Thumbnail class={props.class} alt="" src={icon(props.games[0])} fallback="playlist" />}
    >
      <span class={`grid shrink-0 grid-cols-2 gap-px overflow-hidden rounded-lg ${props.class}`}>
        <For each={props.games.slice(0, 4)}>
          {(g) => <Thumbnail class="w-full rounded-none" alt="" src={icon(g)} />}
        </For>
      </span>
    </Show>
  )
}

/** Game playlists of the active account: the lists on the left, the open one on the right,
 * searchable by name or creator. J/K move between lists. */
export default function Playlists() {
  const params = useParams<{ id?: string }>()
  const lists = () => playlists.lists
  const index = createMemo(() => {
    const i = lists().findIndex((p) => String(p.id) === params.id)
    return i < 0 ? 0 : i
  })
  const [cursor, setCursor] = createSignal(0)
  createEffect(() => setCursor(index()))
  const open = () => lists()[Math.min(cursor(), lists().length - 1)]
  const [query, setQuery] = createSignal("")
  createEffect(
    on(
      () => open()?.id,
      () => setQuery(""),
    ),
  )
  const shown = createMemo(() => searchGames(open()?.games ?? [], query()))
  useCommandContext(() => (open() ? `playlist:${open()?.id}` : null))
  useListCursor({ count: () => lists().length, index: cursor, setIndex: setCursor })

  return (
    <Page width="wide">
      <Show
        when={open()}
        fallback={
          <EmptyState
            class="mt-24"
            icon="playlist"
            title="No playlists yet"
            description="Make one, then add experiences from their menu or page."
          >
            <Button variant="primary" icon="plus" onClick={() => run("playlist.new")}>
              New playlist
            </Button>
          </EmptyState>
        }
      >
        {(p) => (
          <div class="flex items-start gap-8">
            <aside class="sticky top-4 flex w-56 shrink-0 flex-col gap-1">
              <div class="flex h-8 items-center justify-between pl-2">
                <h1 class="font-medium text-fg-muted text-sm">Playlists</h1>
                <Tooltip content="New playlist">
                  <IconButton size="sm" icon="plus" label="New playlist" onClick={() => run("playlist.new")} />
                </Tooltip>
              </div>
              <nav aria-label="Playlists" class="flex flex-col gap-0.5">
                <For each={lists()}>
                  {(l, i) => (
                    <ListItem
                      leading={<Cover class="w-9" games={l.games} />}
                      title={l.name}
                      subtitle={experiences(l.games.length)}
                      selected={i() === cursor()}
                      onClick={() => setCursor(i())}
                    />
                  )}
                </For>
              </nav>
            </aside>
            <section class="flex min-w-0 flex-1 flex-col gap-5">
              <Header list={p()} />
              <Show
                when={p().games.length > 0}
                fallback={
                  <EmptyState
                    class="mt-12"
                    icon="game"
                    title="Nothing here yet"
                    description="Right-click an experience and choose Add to playlist."
                  />
                }
              >
                <div class="flex flex-col gap-3">
                  <TextField
                    class="w-full"
                    icon="search"
                    aria-label="Search this playlist"
                    placeholder="Search by name or creator"
                    value={query()}
                    onChange={setQuery}
                  />
                  <Show
                    when={shown().length > 0}
                    fallback={<EmptyState class="mt-12" icon="search" title="No experiences match" />}
                  >
                    <VirtualGrid items={shown()} minItemWidth={TILE_WIDTH} estimateRowHeight={190} gap={12}>
                      {(g) => (
                        <GameTile
                          game={g}
                          menu={[
                            { kind: "separator" },
                            menuItem("playlist.remove", {
                              arg: `${p().id} ${g.placeId}`,
                              icon: "x",
                              label: "Remove from playlist",
                            }),
                          ]}
                        />
                      )}
                    </VirtualGrid>
                  </Show>
                </div>
              </Show>
            </section>
          </div>
        )}
      </Show>
    </Page>
  )
}

function Header(props: { list: PlaylistView }) {
  const id = () => String(props.list.id)
  return (
    <header class="flex items-center gap-4">
      <Cover class="w-20" games={props.list.games} />
      <div class="flex min-w-0 flex-1 flex-col gap-1">
        <h2 class="truncate font-medium text-2xl">{props.list.name}</h2>
        <span class="text-fg-muted">{experiences(props.list.games.length)}</span>
      </div>
      <Button icon="edit" onClick={() => run("playlist.rename", id())}>
        Rename
      </Button>
      <Menu
        icon="more"
        label="More"
        items={[menuItem("playlist.delete", { arg: id(), icon: "trash", label: "Delete playlist", danger: true })]}
      />
    </header>
  )
}

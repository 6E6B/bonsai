import { useParams } from "@solidjs/router"
import { createEffect, createMemo, createSignal, For, on, Show } from "solid-js"
import { useCommandContext, useListCursor } from "~/lib/commands/hooks"
import { menuItem, run } from "~/lib/commands/registry"
import { playlists } from "~/lib/store"
import { GameTile, TILE_WIDTH } from "~/lib/tiles"
import { Button, EmptyState, NavItem, Page, SectionHeader, TextField, VirtualGrid } from "~/lib/ui"
import { searchGames } from "./filter"

/** Game playlists of the active account: the lists on the left, the open one's games on the right,
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
    <Page
      title="Playlists"
      actions={
        <Button icon="plus" onClick={() => run("playlist.new")}>
          New playlist
        </Button>
      }
    >
      <Show
        when={open()}
        fallback={
          <EmptyState
            class="mt-16"
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
          <div class="flex items-start gap-6">
            <nav aria-label="Playlists" class="sticky top-4 flex w-44 shrink-0 flex-col gap-0.5">
              <For each={lists()}>
                {(l, i) => (
                  <NavItem
                    label={l.name}
                    count={l.games.length}
                    active={i() === cursor()}
                    onClick={() => setCursor(i())}
                  />
                )}
              </For>
            </nav>
            <div class="flex min-w-0 flex-1 flex-col gap-3">
              <SectionHeader
                title={p().name}
                meta={p().games.length}
                actions={
                  <>
                    <Button size="sm" icon="edit" onClick={() => run("playlist.rename", String(p().id))}>
                      Rename
                    </Button>
                    <Button size="sm" icon="trash" onClick={() => run("playlist.delete", String(p().id))}>
                      Delete
                    </Button>
                  </>
                }
              />
              <Show
                when={p().games.length > 0}
                fallback={
                  <EmptyState
                    class="mt-12"
                    icon="game"
                    title="Empty playlist"
                    description="Right-click an experience and choose Add to playlist."
                  />
                }
              >
                <TextField
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
              </Show>
            </div>
          </div>
        )}
      </Show>
    </Page>
  )
}

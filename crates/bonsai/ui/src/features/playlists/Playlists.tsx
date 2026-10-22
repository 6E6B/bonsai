import { useParams } from "@solidjs/router"
import { createEffect, createMemo, createSignal, For, Show } from "solid-js"
import { useCommandContext, useListCursor } from "~/lib/commands/hooks"
import { menuItem, run } from "~/lib/commands/registry"
import { playlists } from "~/lib/store"
import { GameTile, TILE_WIDTH } from "~/lib/tiles"
import { Button, EmptyState, ListItem, Pane, SectionHeader, Split, VirtualGrid } from "~/lib/ui"

/** Game playlists of the active account: the lists on the left, the open one's games on the right.
 * J/K move between lists. */
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
  useCommandContext(() => (open() ? `playlist:${open()?.id}` : null))
  useListCursor({ count: () => lists().length, index: cursor, setIndex: setCursor })

  return (
    <Split>
      <Pane side class="w-64">
        <SectionHeader
          title="Playlists"
          size="page"
          actions={
            <Button size="sm" icon="plus" onClick={() => run("playlist.new")}>
              New
            </Button>
          }
        />
        <div data-cursor-list class="flex flex-col">
          <For each={lists()}>
            {(p, i) => (
              <ListItem
                data-index={i()}
                selected={i() === cursor()}
                title={p.name}
                meta={p.games.length}
                onClick={() => setCursor(i())}
              />
            )}
          </For>
        </div>
      </Pane>
      <Pane>
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
            <div class="flex flex-col gap-3">
              <SectionHeader
                title={p().name}
                meta={p().games.length}
                size="page"
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
                <VirtualGrid items={p().games} minItemWidth={TILE_WIDTH} estimateRowHeight={190} gap={12}>
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
          )}
        </Show>
      </Pane>
    </Split>
  )
}

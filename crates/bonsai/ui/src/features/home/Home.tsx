import { createMemo, createSignal, For, onCleanup, onMount, Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import type { Friend, GameCard } from "~/lib/ipc/bindings"
import { userMenu } from "~/lib/menus"
import { groupByGame } from "~/lib/presence"
import { friends, shell, useHome } from "~/lib/store"
import { thumbUrl } from "~/lib/thumb"
import { GameTile, ShelfSkeleton, tileGrid } from "~/lib/tiles"
import { Alert, Button, ContextMenu, EmptyState, Page, PersonTile, SectionHeader, VirtualShelf } from "~/lib/ui"

/** Landing screen: who is playing what, what you played last, and the Roblox home sorts. */
export default function Home() {
  const home = useHome()
  const playing = () => friends.friends.filter((f) => f.presence === "game")
  const online = () => friends.friends.filter((f) => f.presence === "online" || f.presence === "studio")
  const offline = () => friends.friends.filter((f) => f.presence === "offline")
  // Friends in game first, those in the same experience side by side, biggest group first,
  // then online, then offline.
  const strip = createMemo(() => [...groupByGame(playing()).flatMap((g) => g.users), ...online(), ...offline()])
  return (
    <Show
      when={shell.accounts.length > 0}
      fallback={
        <EmptyState
          class="m-auto"
          icon="user-plus"
          title="No accounts yet"
          description="Add a Roblox account to get started. You can add as many as you like."
        >
          <Button variant="primary" icon="plus" onClick={() => run("account.add")}>
            Add account
          </Button>
        </EmptyState>
      }
    >
      <Page class="gap-6">
        <Show when={shell.look.homeFriends && strip().length > 0}>
          <section class="flex flex-col gap-2">
            <SectionHeader
              title="Friends"
              meta={friends.friends.length}
              actions={
                <Button variant="ghost" size="sm" onClick={() => run("nav.friends")}>
                  See all
                </Button>
              }
            />
            <VirtualShelf items={strip()} itemWidth={PERSON_WIDTH} aria-label="Friends">
              {(f) => <FriendTile friend={f} />}
            </VirtualShelf>
          </section>
        </Show>
        <Show when={shell.look.homeContinue && home.recent.length > 0}>
          <section class="flex flex-col gap-2">
            <SectionHeader title="Continue" />
            <ContinueRow games={home.recent.slice(0, 8)} />
          </section>
        </Show>
        <Show when={shell.look.homeFavorites && home.favorites.length > 0}>
          <section class="flex flex-col gap-2">
            <SectionHeader
              title="Favorites"
              meta={home.favorites.length}
              actions={
                <Button variant="ghost" size="sm" onClick={() => run("nav.profile")}>
                  See all
                </Button>
              }
            />
            <div class={tileGrid}>
              <For each={home.favorites.slice(0, 12)}>{(g) => <GameTile game={g} />}</For>
            </div>
          </section>
        </Show>
        <Show when={shell.look.homeRecommended}>
          <Show when={home.sorts.length === 0 && home.error}>
            {(e) => (
              <Alert variant="danger" title="Couldn't load recommendations">
                {e()}
              </Alert>
            )}
          </Show>
          <Show when={home.sorts.length === 0 && home.loading}>
            <ShelfSkeleton shelves={2} />
          </Show>
          <For each={home.sorts}>
            {(sort) => (
              <section class="flex flex-col gap-2">
                <SectionHeader
                  title={sort.title}
                  meta={sort.subtitle || undefined}
                  actions={
                    <Button variant="ghost" size="sm" onClick={() => run("nav.discover")}>
                      See all
                    </Button>
                  }
                />
                <div class={tileGrid}>
                  <For each={sort.games.slice(0, 12)}>{(g) => <GameTile game={g} />}</For>
                </div>
              </section>
            )}
          </For>
        </Show>
      </Page>
    </Show>
  )
}

const WIDE_TILE = 192
const GAP = 12

/** One row of wide tiles that fills the width: as many as fit, stretched to the edge, so a few
 * recent games don't leave a hole at the end. */
function ContinueRow(props: { games: GameCard[] }) {
  let row!: HTMLDivElement
  const [room, setRoom] = createSignal(4)
  onMount(() => {
    const ro = new ResizeObserver(() => setRoom(Math.max(1, Math.floor((row.clientWidth + GAP) / (WIDE_TILE + GAP)))))
    ro.observe(row)
    onCleanup(() => ro.disconnect())
  })
  // Two or fewer would stretch into banners; keep them at the normal width.
  const cols = () => (props.games.length > 2 ? Math.min(room(), props.games.length) : room())
  return (
    <div ref={row} class="grid gap-x-3" style={{ "grid-template-columns": `repeat(${cols()}, minmax(0, 1fr))` }}>
      <For each={props.games.slice(0, cols())}>{(g) => <GameTile game={g} shape="wide" />}</For>
    </div>
  )
}

/** A little wider than a friend on a profile, for more room between pictures. */
const PERSON_WIDTH = 104

const nameOf = (f: Friend) => f.nickname || f.displayName

const menuFor = (f: Friend) => userMenu(f.id, { friend: true, inGame: f.presence === "game", placeId: f.placeId })

/** A friend as on a profile, with the game they're in under their name. */
function FriendTile(props: { friend: Friend }) {
  const f = () => props.friend
  return (
    <ContextMenu items={menuFor(f())}>
      <PersonTile
        name={nameOf(f())}
        username={f().name || undefined}
        src={thumbUrl("headshot", f().id, true)}
        presence={f().presence}
        status={f().presence === "game" ? f().status : undefined}
        onClick={() => run("user.open", String(f().id))}
      />
    </ContextMenu>
  )
}

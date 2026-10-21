import { createMemo, For, Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import type { Friend } from "~/lib/ipc/bindings"
import { gameMenu, joinUser, userMenu } from "~/lib/menus"
import { type GameGroup, groupByGame } from "~/lib/presence"
import { friends, shell, useHome } from "~/lib/store"
import { thumbUrl } from "~/lib/thumb"
import { GameIcon, GameTile, tileGrid, wideTileGrid } from "~/lib/tiles"
import {
  Alert,
  Avatar,
  Button,
  Card,
  ContextMenu,
  EmptyState,
  IconButton,
  ListItem,
  Page,
  SectionHeader,
  Skeleton,
  Tooltip,
  VirtualGrid,
  VirtualList,
} from "~/lib/ui"

/** Landing screen: who is playing what, what you played last, and the Roblox home sorts. */
export default function Home() {
  const home = useHome()
  const playing = () => friends.friends.filter((f) => f.presence === "game")
  const games = createMemo(() => groupByGame(playing()))
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
      <Page
        title="Home"
        class="gap-6"
        actions={
          <Button icon="refresh" disabled={home.loading} onClick={() => run("view.refresh")}>
            Refresh
          </Button>
        }
      >
        <Show when={shell.look.homeFriends && playing().length > 0}>
          <section class="flex flex-col gap-2">
            <SectionHeader title="Friends playing" meta={playing().length} />
            <VirtualGrid items={games()} minItemWidth={256} estimateRowHeight={180} gap={8}>
              {(g) => <PlayingGroup group={g} />}
            </VirtualGrid>
          </section>
        </Show>
        <Show when={shell.look.homeContinue && home.recent.length > 0}>
          <section class="flex flex-col gap-2">
            <SectionHeader title="Continue" />
            <div class={wideTileGrid}>
              <For each={home.recent.slice(0, 8)}>{(g) => <GameTile game={g} shape="wide" />}</For>
            </div>
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
            <div class={tileGrid}>
              <For each={Array.from({ length: 12 })}>{() => <Skeleton class="aspect-square w-full rounded-md" />}</For>
            </div>
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

/** One experience and the friends in it. The header opens the experience; each friend row
 * opens the friend, with Join on hover. */
function PlayingGroup(props: { group: GameGroup<Friend> }) {
  const header = () => (
    <ListItem
      class="rounded-sm"
      leading={<GameIcon universeId={props.group.universeId} />}
      title={props.group.name}
      subtitle={props.group.users.length === 1 ? "1 friend" : `${props.group.users.length} friends`}
      onClick={props.group.placeId ? () => run("game.open", String(props.group.placeId)) : undefined}
    />
  )
  return (
    <Card class="flex min-w-0 flex-col p-1">
      <Show when={props.group.placeId} fallback={header()}>
        {(placeId) => <ContextMenu items={gameMenu(placeId())}>{header()}</ContextMenu>}
      </Show>
      <VirtualList items={props.group.users} estimateRowHeight={40}>
        {(f) => (
          <ContextMenu items={userMenu(f.id, { friend: true, inGame: true, placeId: f.placeId })}>
            <ListItem
              class="rounded-sm"
              leading={
                // As wide as the game icon above, so the names line up with the game's.
                <span class="flex w-8 justify-center">
                  <Avatar name={f.displayName} src={thumbUrl("headshot", f.id)} />
                </span>
              }
              title={f.displayName}
              onClick={() => run("user.open", String(f.id))}
              actionsOnHover
              actions={
                <Tooltip content="Join">
                  <IconButton
                    size="sm"
                    icon="play"
                    label={`Join ${f.displayName}`}
                    onClick={() => run("game.join", joinUser(f.id))}
                  />
                </Tooltip>
              }
            />
          </ContextMenu>
        )}
      </VirtualList>
    </Card>
  )
}

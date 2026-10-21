import { For, type JSX, Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import { ago, compact, date, percent } from "~/lib/format"
import {
  type Connection,
  commands,
  type GameCard,
  type GroupCard,
  type ItemCard,
  type List,
  type LookView,
  type ProfileView,
  type TraderView,
  type UserBadge,
} from "~/lib/ipc/bindings"
import { Description } from "~/lib/linked-text"
import { emptyList } from "~/lib/list-state"
import { itemMenu, itemTarget, userMenu } from "~/lib/menus"
import { shell, useLive } from "~/lib/store"
import { thumbUrl } from "~/lib/thumb"
import { GameTile, ItemTile } from "~/lib/tiles"
import {
  Badge,
  Button,
  Card,
  Clamp,
  ContextMenu,
  Fact,
  FactGroup,
  Facts,
  Icon,
  IconButton,
  PersonTile,
  SectionHeader,
  Thumbnail,
  Tile,
  Tooltip,
  VirtualShelf,
} from "~/lib/ui"
import { VALUES_SOURCE } from "~/lib/values"
import { TradingCard } from "./Trading"

/** Entry widths for each row: tiles match `tileGrid`; people and badges are narrower. */
const TILE = 128
const SMALL = 96
/** How many a row shows; "See all" has the rest. */
const ROW_MAX = 30

const ms = (iso: string) => {
  const t = Date.parse(iso)
  return Number.isNaN(t) ? undefined : t
}

/** Which tab, and which list inside it, a "See all" opens. */
export type SeeAll = (section: string, list?: string) => void

/** A bit of everything, like the profile on roblox.com: bio and facts, what they wear, and one
 * scrolling row each of friends, experiences, favorites, communities, badges and collectibles, each with
 * a link to its full tab. Rows with nothing in them are left out. */
export function Overview(props: {
  userId: number
  view: ProfileView
  relation: string
  creations: List<GameCard>
  favorites: List<GameCard>
  badges: List<UserBadge>
  groups: List<GroupCard>
  collectibles: List<ItemCard>
  trading: TraderView
  onSeeAll: SeeAll
}) {
  const id = () => String(props.userId)
  const friends = useLive(
    () => props.userId,
    (uid) => commands.profilePeople(uid, "friends", ""),
    emptyList<Connection>,
  )
  const look = useLive(
    () => props.userId,
    (uid) => commands.profileLook(uid),
    (): LookView => ({ loading: true, error: null, rig: "", items: [], colors: [] }),
  )
  const friendCount = () => props.view.user?.friends ?? null

  return (
    <div class="@container">
      <div class="grid grid-cols-1 gap-x-6 gap-y-5 @2xl:grid-cols-[minmax(0,1fr)_minmax(16rem,20rem)]">
        <div class="flex min-w-0 flex-col gap-5">
          <Card class="flex h-fit min-w-0 flex-col gap-1.5 px-3.5 py-3">
            <h3 class="text-sm font-medium text-fg-muted">Bio</h3>
            <Show
              when={props.view.user?.description.trim()}
              fallback={<span class="text-fg-muted">{props.view.user ? "No bio." : ""}</span>}
            >
              {(d) => (
                <Clamp lines={8}>
                  <Description text={d()} tags />
                </Clamp>
              )}
            </Show>
          </Card>
          <TradingCard view={props.trading} enabled={shell.itemValues} />
          <Show when={props.view.note}>
            {(n) => (
              <Card class="flex h-fit min-w-0 flex-col gap-1.5 px-3.5 py-3">
                <h3 class="flex items-center justify-between text-sm font-medium text-fg-muted">
                  Your note
                  <Tooltip content="Edit note">
                    <IconButton size="sm" icon="edit" label="Edit note" onClick={() => run("user.note", id())} />
                  </Tooltip>
                </h3>
                <p class="whitespace-pre-line break-words">{n()}</p>
              </Card>
            )}
          </Show>

          <Row
            title="Friends"
            count={friendCount()}
            list={friends}
            width={SMALL}
            onSeeAll={() => props.onSeeAll("connections", "friends")}
          >
            {(p) => <Person person={p} />}
          </Row>
          <Row
            title="Experiences"
            list={props.creations}
            width={TILE}
            onSeeAll={() => props.onSeeAll("experiences", "creations")}
          >
            {(g) => <GameTile game={g} />}
          </Row>
          <Row
            title="Favorites"
            list={props.favorites}
            width={TILE}
            onSeeAll={() => props.onSeeAll("experiences", "favorites")}
          >
            {(g) => <GameTile game={g} />}
          </Row>
          <Row title="Communities" list={props.groups} width={TILE} onSeeAll={() => props.onSeeAll("communities")}>
            {(c) => <GroupTile group={c} />}
          </Row>
          <Row title="Badges" list={props.badges} width={SMALL} onSeeAll={() => props.onSeeAll("badges")}>
            {(b) => <BadgeTile badge={b} />}
          </Row>
          <Row title="Collectibles" list={props.collectibles} width={TILE} onSeeAll={() => props.onSeeAll("items")}>
            {(i) => <ItemTile item={i} showSerial showRap />}
          </Row>
        </div>

        <aside class="flex h-fit min-w-0 flex-col gap-4">
          <Wearing userId={props.userId} look={look} onSeeAll={() => props.onSeeAll("outfits")} />
          <Facts class="h-fit">
            <FactGroup>
              <Show when={props.view.user && ms(props.view.user.created)}>
                {(t) => <Fact label="Joined">{date(t())}</Fact>}
              </Show>
              <Fact label="User ID">
                <span class="flex items-center gap-1">
                  {id()}
                  <Tooltip content="Copy ID">
                    <IconButton
                      size="sm"
                      icon="copy"
                      label="Copy ID"
                      onClick={() => run("entity.copy_id", `user:${id()}`)}
                    />
                  </Tooltip>
                </span>
              </Fact>
              <Show when={!props.view.mine}>
                <LastOnline view={props.view} />
              </Show>
              <Show when={props.relation}>{(t) => <Fact label="Relation">{t()}</Fact>}</Show>
              <Show when={props.view.visits !== null}>
                <Fact label="Place visits">{compact(props.view.visits ?? 0)}</Fact>
              </Show>
              <Show when={props.view.favoritesReceived !== null}>
                <Fact label="Favorites received">{compact(props.view.favoritesReceived ?? 0)}</Fact>
              </Show>
              <Show when={props.view.unfriendedAt}>{(t) => <Fact label="Unfriended you">{date(t())}</Fact>}</Show>
              <Show when={props.view.pastNames.length > 0}>
                <Fact label="Past names">
                  <For each={props.view.pastNames}>{(n) => <Badge>{n}</Badge>}</For>
                </Fact>
              </Show>
            </FactGroup>
          </Facts>
        </aside>
      </div>
    </div>
  )
}

/** One sideways-scrolling row of a list under a heading with its count and "See all". Hidden until the list has
 * something in it. */
function Row<T>(props: {
  title: string
  /** The full count when Roblox gives one; otherwise how many have loaded, with "+" if more. */
  count?: number | null
  list: List<T>
  width: number
  onSeeAll: () => void
  children: (item: T) => JSX.Element
}) {
  const meta = () => {
    if (props.count !== undefined && props.count !== null) return compact(props.count)
    return `${props.list.items.length}${props.list.more ? "+" : ""}`
  }
  return (
    <Show when={props.list.items.length > 0}>
      <section class="flex min-w-0 flex-col gap-2">
        <SectionHeader
          title={props.title}
          meta={meta()}
          actions={
            <Button size="sm" variant="ghost" onClick={props.onSeeAll}>
              See all
            </Button>
          }
        />
        <VirtualShelf items={props.list.items.slice(0, ROW_MAX)} itemWidth={props.width} aria-label={props.title}>
          {(item) => props.children(item)}
        </VirtualShelf>
      </section>
    </Show>
  )
}

/** A friend of theirs. Presence is only known for your own friends, so others get no dot rather
 * than a grey one that reads as offline. */
function Person(props: { person: Connection }) {
  const p = () => props.person
  return (
    <ContextMenu items={userMenu(p().id, { friend: p().friend, following: p().following ?? undefined })}>
      <PersonTile
        name={p().displayName || p().name || `User ${p().id}`}
        username={p().name || undefined}
        src={thumbUrl("headshot", p().id)}
        presence={p().friend ? p().presence : undefined}
        verified={p().verified}
        onClick={() => run("user.open", String(p().id))}
      />
    </ContextMenu>
  )
}

function GroupTile(props: { group: GroupCard }) {
  return (
    <Tile
      title={props.group.name}
      subtitle={`${compact(props.group.members)} members`}
      src={thumbUrl("group_icon", props.group.id)}
      fallback="communities"
      badge={
        <Show when={props.group.primary}>
          <Badge variant="accent">Primary</Badge>
        </Show>
      }
      onClick={() => run("group.open", String(props.group.id))}
    />
  )
}

/** A badge's icon and name, and how many players have it. Opens the experience it is from. */
export function BadgeTile(props: { badge: UserBadge }) {
  const b = () => props.badge
  return (
    <Tile
      title={b().name}
      subtitle={b().winRate === null ? undefined : `${percent(b().winRate ?? 0)} have it`}
      src={thumbUrl("badge_icon", b().id)}
      fallback="badge"
      onClick={() => b().placeId && run("game.open", String(b().placeId))}
    />
  )
}

/** Their avatar as it is now, with the items on it. The Outfits tab has the 3D view and more. */
function Wearing(props: { userId: number; look: LookView; onSeeAll: () => void }) {
  return (
    <Card class="flex flex-col gap-3 px-3.5 py-3">
      <div class="flex items-center justify-between gap-2">
        <h3 class="text-sm font-medium text-fg-muted">
          Wearing
          <Show when={props.look.items.length > 0}>
            <span class="ml-1.5 text-fg-faint">{props.look.items.length}</span>
          </Show>
        </h3>
        <Button size="sm" variant="ghost" onClick={props.onSeeAll}>
          Outfits
        </Button>
      </div>
      <button type="button" class="self-center" onClick={props.onSeeAll}>
        <Thumbnail class="w-48" alt="Avatar" src={thumbUrl("avatar", props.userId, true)} fallback="avatar" />
      </button>
      <Show when={props.look.items.length > 0}>
        <div class="grid grid-cols-[repeat(auto-fill,minmax(3rem,1fr))] gap-1.5">
          <For each={props.look.items}>
            {(i) => (
              <ContextMenu items={itemMenu(i.id, i.bundle, i.assetType)}>
                <Tooltip content={i.name} class="grid">
                  <button type="button" onClick={() => run("market.open", itemTarget(i.id, i.bundle))}>
                    <Thumbnail class="w-full" alt={i.name} src={thumbUrl("asset", i.id)} fallback="avatar" />
                  </button>
                </Tooltip>
              </ContextMenu>
            )}
          </For>
        </div>
      </Show>
    </Card>
  )
}

/** When they were last online. Roblox doesn't say, so this is the newer of what the app saw
 * and, with values on, what Rolimons saw (marked with its icon). */
function LastOnline(props: { view: ProfileView }) {
  const by = () => (props.view.lastOnlineBy === "rolimons" ? VALUES_SOURCE : "this app")
  const where = () => (props.view.lastLocation ? ` (${props.view.lastLocation})` : "")
  return (
    <Show when={props.view.presence === "offline"} fallback={<Fact label="Last online">Now</Fact>}>
      <Show when={props.view.lastOnline}>
        {(t) => (
          <Fact label="Last online">
            <Tooltip content={`Seen by ${by()} on ${date(t())}${where()}`} class="inline-flex items-center gap-1">
              <Show when={props.view.lastOnlineBy === "rolimons"}>
                <Icon name="rolimons" size={12} class="text-fg-faint" aria-label={VALUES_SOURCE} />
              </Show>
              {ago(t())}
            </Tooltip>
          </Fact>
        )}
      </Show>
    </Show>
  )
}

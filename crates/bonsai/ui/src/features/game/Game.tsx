import { useParams } from "@solidjs/router"
import { createEffect, createMemo, createSignal, For, Match, onCleanup, Show, Switch } from "solid-js"
import { BuyForm } from "~/lib/buy-form"
import { useCommandContext, useSubTabs } from "~/lib/commands/hooks"
import { handle, run } from "~/lib/commands/registry"
import { ago, compact, date, percent } from "~/lib/format"
import {
  type BadgeRow,
  commands,
  type EventRow,
  type GameCard,
  type GameDetails,
  type Game as GameView,
  type List,
  type PassRow,
  type PrivateServerRow,
  type RecentServerRow,
  type ServerOffer,
  type ServerRow,
  type Servers,
  type WikiPageView,
} from "~/lib/ipc/bindings"
import { Description } from "~/lib/linked-text"
import { emptyList, ListState } from "~/lib/list-state"
import { entityMenu, joinAsItems, playlistItems } from "~/lib/menus"
import { ServerControls } from "~/lib/private-server"
import { serverName } from "~/lib/server-name"
import { friends, useLive } from "~/lib/store"
import { setTabTitle } from "~/lib/tabs"
import { thumbUrl } from "~/lib/thumb"
import { GameTile } from "~/lib/tiles"
import {
  Alert,
  Avatar,
  AvatarStack,
  type AvatarStackItem,
  Badge,
  Button,
  Card,
  Checkbox,
  Clamp,
  type Column,
  DataTable,
  Divider,
  EmptyState,
  Fact,
  FactGroup,
  Facts,
  IconButton,
  ListItem,
  Menu,
  Meter,
  Page,
  Robux,
  SectionHeader,
  Select,
  Showcase,
  Skeleton,
  Stat,
  showToast,
  TabPanel,
  Tabs,
  TextField,
  TextLink,
  Thumbnail,
  ToggleButton,
  Toolbar,
  Tooltip,
  useDialog,
  VerifiedBadge,
  VirtualGrid,
  VirtualList,
} from "~/lib/ui"
import { useWikiAbout, WikiTab, WikiTeaser } from "~/lib/wiki-panels"

const allSections = [
  { value: "about", label: "About" },
  { value: "servers", label: "Servers" },
  { value: "friends", label: "Friends' servers" },
  { value: "private", label: "Private servers" },
  { value: "store", label: "Store" },
  { value: "badges", label: "Badges" },
  { value: "events", label: "Events" },
  { value: "recommended", label: "Recommended" },
  { value: "wiki", label: "Wiki" },
]

/** A list that has finished loading with nothing in it. Errors don't count, so they stand out. */
const isEmpty = (l: { loading: boolean; error: string | null; items: unknown[] }) =>
  !l.loading && !l.error && l.items.length === 0

/** ISO date → ms, or undefined when missing. */
const ms = (iso: string) => {
  const t = Date.parse(iso)
  return Number.isNaN(t) ? undefined : t
}

/** An experience, by place id: media beside the title, join, votes, a few headline stats and
 * the start of the description; then about, servers, private servers, store, badges, events and
 * recommendations in sub-tabs. The rest of the details live in About. */
export default function Game() {
  const params = useParams()
  const placeId = () => Number(params.id)
  const game = useLive(
    placeId,
    (id) => commands.game(id),
    (): GameView => ({
      placeId: placeId(),
      details: null,
      me: null,
      maturity: null,
      maturityDescriptors: [],
      media: [],
    }),
  )
  const offer = useLive(
    placeId,
    async (id) => ({ offer: await commands.gameServerOffer(id) }),
    () => ({ offer: null as ServerOffer | null }),
  )
  const friendServers = useLive(placeId, (id) => commands.gameFriendServers(id), emptyList<ServerRow>)
  const privateServers = useLive(placeId, (id) => commands.gamePrivateServers(id), emptyList<PrivateServerRow>)
  const passes = useLive(placeId, (id) => commands.gamePasses(id), emptyList<PassRow>)
  const badges = useLive(placeId, (id) => commands.gameBadges(id), emptyList<BadgeRow>)
  const events = useLive(placeId, (id) => commands.gameEvents(id), emptyList<EventRow>)
  const recommended = useLive(placeId, (id) => commands.gameRecommended(id), emptyList<GameCard>)
  const wiki = useWikiAbout(() => {
    const d = game.details
    return d ? { kind: "game", id: d.rootPlaceId, name: d.name, creator: d.creatorName, group: d.creatorIsGroup } : null
  })
  const [section, setSection] = createSignal("about")
  // Every experience gets the same tabs; empty ones are muted and moved to the end, in order.
  // Wiki is the exception: most experiences have no article, so it only shows when one exists.
  const sections = createMemo(() => {
    const empty = new Set<string>()
    if (isEmpty(friendServers)) empty.add("friends")
    if (isEmpty(privateServers) && !offer.offer) empty.add("private")
    if (isEmpty(passes)) empty.add("store")
    if (isEmpty(badges)) empty.add("badges")
    if (isEmpty(events)) empty.add("events")
    if (isEmpty(recommended)) empty.add("recommended")
    const shown = allSections.filter((t) => t.value !== "wiki" || wiki.article)
    return [
      ...shown.filter((t) => !empty.has(t.value)),
      ...shown.filter((t) => empty.has(t.value)).map((t) => ({ ...t, empty: true })),
    ]
  })
  createEffect(() => {
    if (!sections().some((t) => t.value === section())) setSection("about")
  })
  useSubTabs(sections, section, setSection)
  useCommandContext(() => `game:${placeId()}`)
  const dialog = useDialog()
  onCleanup(
    handle("game.buy_private_server", (name) => {
      const o = offer.offer
      if (!o) return showToast("You can't buy a private server here.")
      dialog.show(
        () => <BuyServerForm placeId={placeId()} price={o.price} initial={name ?? ""} onDone={dialog.close} />,
        { title: "Buy a private server", size: "sm" },
      )
    }),
  )
  onCleanup(
    handle("game.buy_pass", (arg) => {
      const p = passes.items.find((x) => String(x.id) === arg?.trim())
      if (!p) return showToast("Pick a pass in the Store tab.")
      if (p.owned) return showToast("You already own this pass.")
      if (p.price === null) return showToast("This pass is off sale.")
      const price = p.price
      dialog.show(
        () => (
          <BuyForm
            name={p.name}
            seller={game.details?.creatorName}
            price={price}
            thumb={thumbUrl("game_pass", p.id)}
            buy={() => commands.buyPass(placeId(), p.id, price)}
            onDone={dialog.close}
          />
        ),
        { title: "Buy pass", size: "sm" },
      )
    }),
  )
  createEffect(() => {
    const name = game.details?.name
    if (name) setTabTitle(`/games/${params.id}`, name)
  })

  return (
    <Page>
      <Show when={game.details} fallback={<HeaderSkeleton />}>
        {(d) => <Header game={game} details={d()} />}
      </Show>

      <Tabs aria-label="Experience sections" items={sections()} value={section()} onChange={setSection}>
        <TabPanel value="about">
          <AboutPanel
            game={game}
            wiki={wiki}
            friendServers={friendServers}
            onAllServers={() => setSection("servers")}
          />
        </TabPanel>
        <TabPanel value="servers">
          <ServersPanel placeId={placeId()} max={game.details?.maxPlayers ?? 0} />
        </TabPanel>
        <TabPanel value="friends">
          <FriendServersPanel list={friendServers} />
        </TabPanel>
        <TabPanel value="private" class="flex flex-col gap-2">
          <PrivateServersPanel list={privateServers} offer={offer.offer} />
        </TabPanel>
        <TabPanel value="store">
          <StorePanel list={passes} />
        </TabPanel>
        <TabPanel value="badges" class="flex flex-col">
          <BadgesPanel list={badges} />
        </TabPanel>
        <TabPanel value="events" class="flex flex-col">
          <EventsPanel list={events} />
        </TabPanel>
        <TabPanel value="recommended">
          <RecommendedPanel list={recommended} />
        </TabPanel>
        <TabPanel value="wiki">
          <WikiTab view={wiki} missing="The Roblox Wiki has no article about this experience." />
        </TabPanel>
      </Tabs>
    </Page>
  )
}

function rating(up: number | null, down: number | null) {
  const total = (up ?? 0) + (down ?? 0)
  return total > 0 ? percent((up ?? 0) / total) : "–"
}

function when(iso: string, fmt: (ms: number) => string) {
  const t = ms(iso)
  return t === undefined ? "–" : fmt(t)
}

function HeaderSkeleton() {
  return (
    <div class="flex flex-col overflow-hidden rounded-lg shadow-raised">
      <Skeleton class="aspect-[16/5] max-h-[380px] min-h-[220px] w-full" />
      <div class="flex items-center gap-1.5 px-4 py-3">
        <Skeleton class="h-8 w-36 rounded-md" />
        <Skeleton class="h-8 w-20 rounded-md" />
        <Skeleton class="h-8 w-48 rounded-md" />
        <Skeleton class="ml-auto h-9 w-72 rounded-md" />
      </div>
    </div>
  )
}

/** The pictures as a banner with the name, tags and creator on them; join, votes and the other
 * buttons under it, with the headline figures at the far end. Trailers show their preview image
 * and title; they play on the website. */
function Header(props: { game: GameView; details: GameDetails }) {
  const d = () => props.details
  const placeId = () => props.game.placeId
  const id = () => String(placeId())
  const [shown, setShown] = createSignal(0)
  const shownMedia = () => props.game.media[shown()]
  const images = () =>
    props.game.media.map((m, i) => ({
      src: thumbUrl("game_thumbnail", m.imageId, true),
      label: m.videoTitle || m.alt || `Image ${i + 1}`,
    }))
  const tags = () => [d().genreL1 || d().genre, d().genreL2, props.game.maturity].filter((t): t is string => Boolean(t))
  const votes = () => (d().upVotes ?? 0) + (d().downVotes ?? 0)
  const me = () => props.game.me
  return (
    <Showcase
      images={images()}
      index={shown()}
      onSelect={setShown}
      caption={shownMedia()?.video ? `${shownMedia()?.videoTitle || "Trailer"} (plays on the website)` : undefined}
      captionIcon="video"
      tags={tags()}
      title={d().name}
      byline={
        <span class="flex min-w-0 items-center gap-1">
          by
          <TextLink onClick={() => run(d().creatorIsGroup ? "group.open" : "user.open", String(d().creatorId))}>
            {d().creatorName}
          </TextLink>
          <Show when={d().creatorVerified}>
            <VerifiedBadge />
          </Show>
        </span>
      }
      summary={[
        { icon: "friends", value: d().playing.toLocaleString(), label: "playing" },
        ...(votes() > 0
          ? [{ icon: "thumbs-up" as const, value: rating(d().upVotes, d().downVotes), label: "rating" }]
          : []),
        { icon: "eye", value: compact(d().visits), label: "visits" },
        { icon: "star", value: compact(d().favorites), label: "favorites" },
      ]}
      actions={
        <>
          <Button variant="primary" size="lg" icon="play" class="w-36" onClick={() => run("game.join", id())}>
            Join
          </Button>
          <Menu
            size="lg"
            items={joinAsItems("game.join_as", id())}
            disabled={joinAsItems("game.join_as", id()).length === 0}
          >
            Join as…
          </Menu>
          <Divider orientation="vertical" class="mx-1 h-5 self-center" />
          <ToggleButton
            variant="secondary"
            size="lg"
            icon="thumbs-up"
            pressedIcon="thumbs-up-fill"
            pressed={me()?.vote === true}
            disabled={!me()}
            onChange={() => run("game.like")}
          >
            {compact(d().upVotes ?? 0)}
          </ToggleButton>
          <ToggleButton
            variant="secondary"
            size="lg"
            icon="thumbs-down"
            pressedIcon="thumbs-down-fill"
            pressed={me()?.vote === false}
            disabled={!me()}
            onChange={() => run("game.dislike")}
          >
            {compact(d().downVotes ?? 0)}
          </ToggleButton>
          <ToggleButton
            variant="secondary"
            size="lg"
            icon="star"
            pressedIcon="star-fill"
            pressed={me()?.favorited ?? false}
            disabled={!me()}
            onChange={() => run("game.favorite")}
          >
            Favorite
          </ToggleButton>
          <Tooltip content={me()?.following ? "Stop update notifications" : "Notify me about updates"}>
            <ToggleButton
              variant="secondary"
              size="lg"
              icon="bell"
              pressedIcon="bell-fill"
              label="Notify"
              pressed={me()?.following ?? false}
              disabled={!me() || (!me()?.following && !me()?.canFollow)}
              onChange={() => run("game.notify")}
            />
          </Tooltip>
          <Menu size="lg" icon="playlist" label="Add to playlist" items={playlistItems(placeId())} />
          <Menu size="lg" icon="more" label="More" items={entityMenu(`game:${placeId()}`)} />
        </>
      }
      stats={
        <>
          <Stat label="Playing now">{d().playing.toLocaleString()}</Stat>
          <Stat label="Rating">
            <span class="flex items-center gap-2">
              {rating(d().upVotes, d().downVotes)}
              <Show when={votes() > 0}>
                <Meter class="w-14" label="Rating" value={d().upVotes ?? 0} max={votes()} valueLabel="" />
              </Show>
            </span>
          </Stat>
          <Stat label="Visits">{compact(d().visits)}</Stat>
          <Stat label="Favorites">{compact(d().favorites)}</Stat>
          <Stat label="Updated">{when(d().updated, ago)}</Stat>
        </>
      }
    />
  )
}

/** Loading, error and empty states shared by the tab lists. Renders `children` once there are
 * items; keeps showing them while a refresh is in flight. */

const sorts = [
  { value: "friends", label: "Friends first" },
  { value: "ping", label: "Lowest ping" },
  { value: "most", label: "Most players" },
  { value: "fewest", label: "Fewest players" },
]

const byServer: Record<string, (a: ServerRow, b: ServerRow) => number> = {
  friends: (a, b) => b.friends.length - a.friends.length || b.playing - a.playing,
  ping: (a, b) => (a.ping ?? Number.MAX_SAFE_INTEGER) - (b.ping ?? Number.MAX_SAFE_INTEGER),
  most: (a, b) => b.playing - a.playing,
  fewest: (a, b) => a.playing - b.playing,
}

const friendName = (id: number) => friends.friends.find((f) => f.id === id)?.displayName ?? String(id)

/** Faces for a server row: friends first, ringed and named, then the other players. */
function serverFaces(s: ServerRow): AvatarStackItem[] {
  return [
    ...s.friends.map((f) => ({
      name: friendName(f),
      label: friendName(f),
      src: thumbUrl("headshot", f),
      highlight: true,
    })),
    ...s.players.map((p) => ({ name: "", src: thumbUrl("player", p) })),
  ]
}

/** A server's shared name from its job id. Hover shows the job id; click copies it. */
function ServerName(props: { jobId: string }) {
  return (
    <Tooltip
      class="min-w-0"
      content={
        <span class="flex flex-col gap-0.5">
          <span class="font-mono text-xs">{props.jobId}</span>
          <span class="text-xs">Click to copy</span>
        </span>
      }
    >
      <button
        type="button"
        class="block max-w-full cursor-copy truncate text-left"
        aria-label={`${serverName(props.jobId)}, copy job id`}
        onClick={() => run("game.copy_job_id", props.jobId)}
      >
        {serverName(props.jobId)}
      </button>
    </Tooltip>
  )
}

/** A recent server's `game.join_job` argument: its job id and place, which may be a sub-place. */
const recentKey = (r: RecentServerRow) => `${r.jobId} ${r.placeId}`

/** Servers the account played in over the last day. Players and fill show when the server is
 * in the loaded list. */
const recentColumns: Column<RecentServerRow>[] = [
  { key: "name", label: "Server", width: "10rem", cell: (r) => <ServerName jobId={r.jobId} /> },
  {
    key: "players",
    label: "Players",
    width: "8.5rem",
    cell: (r) => (
      <Show when={r.server} fallback={<span class="text-fg-muted">–</span>}>
        {(s) => <AvatarStack items={serverFaces(s())} total={Math.max(s().playing, s().friends.length)} />}
      </Show>
    ),
  },
  {
    key: "fill",
    label: "",
    width: "minmax(6rem, 1fr)",
    cell: (r) => (
      <Show when={r.server}>
        {(s) => <Meter class="w-full" label="Players" value={s().playing} max={s().maxPlayers} tone="capacity" />}
      </Show>
    ),
  },
  {
    key: "played",
    label: "Played",
    width: "7.5rem",
    align: "end",
    cell: (r) => (
      <Show when={r.lastSeen} fallback="–">
        {(t) => <span class="text-fg-muted">{ago(t())}</span>}
      </Show>
    ),
  },
  {
    key: "actions",
    label: "",
    width: "6.5rem",
    align: "end",
    cell: (r) => (
      <>
        <Button size="sm" onClick={() => run("game.join_job", recentKey(r))}>
          Join
        </Button>
        <Menu
          size="sm"
          icon="chevron-down"
          label="Join as…"
          placement="bottom-end"
          items={joinAsItems("game.join_job", recentKey(r))}
        />
      </>
    ),
  },
]

function ServersPanel(props: { placeId: number; max: number }) {
  const list = useLive(
    () => props.placeId,
    (id) => commands.gameServers(id),
    (): Servers => ({ loading: true, error: null, fetchedAt: null, servers: [], more: false }),
  )
  const [sort, setSort] = createSignal("friends")
  const [hideFull, setHideFull] = createSignal(false)
  const [friendsOnly, setFriendsOnly] = createSignal(false)
  const recent = useLive(
    () => props.placeId,
    async (id) => ({ items: await commands.gameRecentServers(id) }),
    () => ({ items: [] as RecentServerRow[] }),
  )
  // The `game.join_job` argument of the row the cursor is on, in either table.
  const [selected, setSelected] = createSignal<string>()
  onCleanup(handle("game.join_server", () => selected() && run("game.join_job", selected() ?? null)))

  const shown = createMemo(() =>
    list.servers
      .filter((s) => (!hideFull() || s.playing < s.maxPlayers) && (!friendsOnly() || s.friends.length > 0))
      .sort(byServer[sort()]),
  )

  const columns: Column<ServerRow>[] = [
    {
      key: "name",
      label: "Server",
      // Fits the longest name, "Wandering Albatross".
      width: "10rem",
      cell: (s) => <ServerName jobId={s.jobId} />,
    },
    {
      key: "players",
      label: "Players",
      // Roblox sends five player tokens a server, so the faces need little room.
      width: "8.5rem",
      cell: (s) => <AvatarStack items={serverFaces(s)} total={Math.max(s.playing, s.friends.length)} />,
    },
    {
      key: "fill",
      label: "",
      width: "minmax(6rem, 1fr)",
      cell: (s) => <Meter class="w-full" label="Players" value={s.playing} max={s.maxPlayers} tone="capacity" />,
    },
    { key: "ping", label: "Ping", width: "4rem", align: "end", cell: (s) => (s.ping === null ? "–" : `${s.ping} ms`) },
    {
      key: "fps",
      label: "FPS",
      width: "3rem",
      align: "end",
      cell: (s) => (s.fps === null ? "–" : Math.round(s.fps)),
    },
    {
      key: "actions",
      label: "",
      width: "6.5rem",
      align: "end",
      cell: (s) => (
        <>
          <IconButton
            size="sm"
            icon="link"
            label="Copy invite link"
            onClick={() => run("game.copy_server_link", s.jobId)}
          />
          <Button size="sm" onClick={() => run("game.join_job", s.jobId)}>
            Join
          </Button>
          <Menu
            size="sm"
            icon="chevron-down"
            label="Join as…"
            placement="bottom-end"
            items={joinAsItems("game.join_job", s.jobId)}
          />
        </>
      ),
    },
  ]

  return (
    <div class="flex flex-col gap-3">
      <Show when={recent.items.length > 0}>
        <div class="flex flex-col gap-1.5">
          <SectionHeader title="Recent" meta={recent.items.length} size="group" />
          <DataTable
            aria-label="Recent servers"
            columns={recentColumns}
            rows={recent.items}
            rowKey={recentKey}
            selected={selected()}
            onRowClick={(r) => setSelected(recentKey(r))}
          />
        </div>
      </Show>
      <Toolbar
        actions={
          <>
            <Button size="sm" variant="primary" onClick={() => run("game.join_best")}>
              Join best
            </Button>
            <Button size="sm" variant="ghost" onClick={() => run("game.join_smallest")}>
              Smallest
            </Button>
            <Button size="sm" variant="ghost" onClick={() => run("game.join_random")}>
              Random
            </Button>
            <IconButton
              size="sm"
              icon="refresh"
              label="Refresh"
              disabled={list.loading}
              onClick={() => run("game.refresh_servers")}
            />
          </>
        }
      >
        <Select class="w-40" size="sm" aria-label="Sort servers" options={sorts} value={sort()} onChange={setSort} />
        <Checkbox checked={hideFull()} onChange={setHideFull}>
          Hide full
        </Checkbox>
        <Checkbox checked={friendsOnly()} onChange={setFriendsOnly}>
          Friends only
        </Checkbox>
        <span class="text-sm text-fg-muted">
          <Show when={shown().length !== list.servers.length}>{shown().length} of </Show>
          {list.servers.length} servers
          <Show when={list.fetchedAt}>{(t) => ` · ${ago(t())}`}</Show>
        </span>
      </Toolbar>
      <Show when={list.error}>
        {(e) => (
          <Alert variant="danger" title="Couldn't load servers">
            {e()}
          </Alert>
        )}
      </Show>
      <Show
        when={list.servers.length > 0 || !list.loading}
        fallback={
          <div class="flex flex-col gap-2">
            <For each={[1, 2, 3, 4]}>{() => <Skeleton class="h-8 w-full rounded-md" />}</For>
          </div>
        }
      >
        <DataTable
          aria-label="Servers"
          columns={columns}
          rows={shown()}
          rowKey={(s) => s.jobId}
          selected={selected()}
          onRowClick={(s) => setSelected(s.jobId)}
          empty={
            <EmptyState
              icon="server"
              title="No servers match"
              description={props.max > 0 ? `Servers hold up to ${props.max} players.` : undefined}
            />
          }
        />
        <Show when={list.more}>
          <Button
            class="self-start"
            variant="ghost"
            disabled={list.loading}
            onClick={() => run("list.more", "servers")}
          >
            Load more servers
          </Button>
        </Show>
      </Show>
    </div>
  )
}

function PrivateServersPanel(props: { list: List<PrivateServerRow>; offer: ServerOffer | null }) {
  const list = props.list
  return (
    <div class="flex flex-col gap-2">
      <Show when={props.offer}>
        {(o) => (
          <div class="flex items-center gap-2">
            <Button icon="plus" onClick={() => run("game.buy_private_server")}>
              {o().price === 0 ? "Create a free private server" : "Buy a private server"}
            </Button>
            <Show when={o().price > 0}>
              <span class="text-sm text-fg-muted">
                <Robux amount={o().price} /> a month
              </span>
            </Show>
          </div>
        )}
      </Show>
      <ListState
        list={list}
        icon="server"
        empty="No private servers"
        description="Private servers you own or were invited to show up here."
        more="private_servers"
      >
        <VirtualList items={list.items} estimateRowHeight={96}>
          {(s) => (
            <div class="flex flex-col">
              <ListItem
                leading={<Avatar name={s.ownerName || "?"} src={thumbUrl("headshot", s.ownerId)} />}
                title={s.name || "Private server"}
                subtitle={`${s.mine ? "Yours" : `Owned by ${s.ownerName || "…"}`} · ${s.playing}/${s.maxPlayers} playing`}
                actions={
                  <Button size="sm" onClick={() => run("game.join_private_server", String(s.id))}>
                    Join
                  </Button>
                }
              />
              <Show when={s.mine}>
                <ServerControls serverId={s.id} class="pb-2 pl-12" />
              </Show>
            </div>
          )}
        </VirtualList>
      </ListState>
    </div>
  )
}

/** Name, price and the buy button. The price on the button is what Roblox charges; it refuses
 * the purchase if the price changed since. */
function BuyServerForm(props: { placeId: number; price: number; initial: string; onDone: () => void }) {
  const [name, setName] = createSignal(props.initial)
  const [busy, setBusy] = createSignal(false)
  const buy = async () => {
    setBusy(true)
    const r = await commands.buyPrivateServer(props.placeId, name().trim(), props.price)
    setBusy(false)
    if (r.status === "error") return showToast(r.error)
    props.onDone()
  }
  return (
    <form
      class="flex flex-col gap-3 p-4"
      onSubmit={(e) => {
        e.preventDefault()
        void buy()
      }}
    >
      <TextField label="Name" value={name()} onChange={setName} autofocus />
      <p class="text-sm text-fg-muted">
        <Show when={props.price > 0} fallback="Free. It renews every month at no cost.">
          Costs <Robux amount={props.price} /> now and every month until you turn renewal off.
        </Show>
      </p>
      <div class="flex justify-end gap-2">
        <Button variant="ghost" type="button" onClick={() => props.onDone()}>
          Cancel
        </Button>
        <Button variant="primary" type="submit" disabled={busy() || name().trim() === ""}>
          <Show when={props.price > 0} fallback="Create">
            Buy for <Robux amount={props.price} />
          </Show>
        </Button>
      </div>
    </form>
  )
}

function StorePanel(props: { list: List<PassRow> }) {
  const list = props.list
  return (
    <ListState list={list} icon="badge" empty="Nothing in the store" more="passes">
      <VirtualGrid items={list.items} minItemWidth={160} estimateRowHeight={240} gap={12}>
        {(p) => (
          <Card class="flex flex-col gap-2 p-2">
            <Thumbnail alt={p.name} src={thumbUrl("game_pass", p.id)} fallback="badge" />
            <Show when={p.description} fallback={<span class="truncate font-medium">{p.name}</span>}>
              {(d) => (
                <Tooltip content={d()} class="min-w-0 max-w-full">
                  <span class="truncate font-medium">{p.name}</span>
                </Tooltip>
              )}
            </Show>
            <div class="flex items-center justify-between">
              <Switch fallback={<Robux amount={p.price ?? 0} />}>
                <Match when={p.price === null}>
                  <span class="text-sm text-fg-muted">Off sale</span>
                </Match>
                <Match when={p.price === 0}>
                  <span class="text-sm">Free</span>
                </Match>
              </Switch>
              <Show when={p.owned}>
                <Badge variant="success">Owned</Badge>
              </Show>
            </div>
            <Show when={!p.owned && p.price !== null}>
              <Button size="sm" onClick={() => run("game.buy_pass", String(p.id))}>
                Buy
              </Button>
            </Show>
          </Card>
        )}
      </VirtualGrid>
    </ListState>
  )
}

function BadgesPanel(props: { list: List<BadgeRow> }) {
  const list = props.list
  return (
    <ListState list={list} icon="badge" empty="No badges" more="badges">
      <VirtualList items={list.items} estimateRowHeight={64}>
        {(b) => (
          <ListItem
            leading={<Thumbnail class="w-10" alt={b.name} src={thumbUrl("badge_icon", b.id)} fallback="badge" />}
            title={b.name}
            subtitle={b.description}
            dim={b.earned === null}
            meta={
              <>
                <span>{percent(b.winRate ?? 0)} have it</span>
                <Show when={b.earned ? ms(b.earned) : undefined}>
                  {(t) => <Badge variant="success">Earned {date(t())}</Badge>}
                </Show>
              </>
            }
          />
        )}
      </VirtualList>
    </ListState>
  )
}

function EventsPanel(props: { list: List<EventRow> }) {
  const list = props.list
  const span = (e: EventRow) => {
    const [a, b] = [ms(e.start), ms(e.end)]
    if (a === undefined) return e.host
    return b === undefined ? date(a) : `${date(a)} – ${date(b)}`
  }
  const [open, setOpen] = createSignal<string>()
  return (
    <ListState list={list} icon="calendar" empty="No upcoming events">
      <VirtualList items={list.items} estimateRowHeight={80}>
        {(e) => (
          <>
            <ListItem
              title={e.title}
              subtitle={[span(e), e.subtitle].filter(Boolean).join(" · ")}
              expanded={open() === e.id}
              onClick={e.description ? () => setOpen((o) => (o === e.id ? undefined : e.id)) : undefined}
              actions={
                <ToggleButton
                  variant="secondary"
                  size="sm"
                  icon="bell"
                  pressedIcon="bell-fill"
                  pressed={e.going}
                  onChange={() => run("game.event_notify", e.id)}
                >
                  Notify me
                </ToggleButton>
              }
            />
            <Show when={open() === e.id}>
              <div class="pb-2 pl-3">
                <Description text={e.description} />
              </div>
            </Show>
          </>
        )}
      </VirtualList>
    </ListState>
  )
}

function RecommendedPanel(props: { list: List<GameCard> }) {
  const list = props.list
  return (
    <ListState list={list} icon="game" empty="No recommendations">
      <VirtualGrid items={list.items} minItemWidth={128} estimateRowHeight={190} gap={12}>
        {(g) => <GameTile game={g} />}
      </VirtualGrid>
    </ListState>
  )
}

/** Description, details and the creator on the left; who and where to join on the right: friends
 * playing and the best open servers. */
function AboutPanel(props: {
  game: GameView
  wiki: WikiPageView
  friendServers: List<ServerRow>
  onAllServers: () => void
}) {
  const d = () => props.game.details
  const bio = useLive(
    () => (d() ? `${d()?.creatorIsGroup ? "g" : "u"}${d()?.creatorId}` : ""),
    async () => {
      const g = d()
      return { text: g ? await commands.creatorBio(g.creatorId, g.creatorIsGroup) : null }
    },
    () => ({ text: null as string | null }),
  )
  const yesNo = (b: boolean) => (b ? "Yes" : "No")
  return (
    <Show when={d()} fallback={<Skeleton class="h-32 w-full rounded-md" />}>
      {(g) => {
        const openCreator = () => run(g().creatorIsGroup ? "group.open" : "user.open", String(g().creatorId))
        return (
          <div class="@container">
            <div class="grid grid-cols-1 items-start gap-4 @3xl:grid-cols-[minmax(0,1fr)_minmax(17rem,20rem)]">
              <div class="flex min-w-0 flex-col gap-4">
                <Card class="flex min-w-0 flex-col">
                  <section class="flex flex-col gap-1.5 px-4 py-3.5">
                    <h3 class="text-sm font-medium text-fg-muted">Description</h3>
                    <Show when={g().description} fallback={<span class="text-fg-muted">No description.</span>}>
                      <Clamp lines={8}>
                        <Description text={g().description} />
                      </Clamp>
                    </Show>
                  </section>
                  <Show when={props.game.maturityDescriptors.length > 0}>
                    <Divider />
                    <div class="flex flex-wrap items-center gap-1.5 px-4 py-2.5">
                      <span class="mr-1 text-sm text-fg-muted">Contains</span>
                      <For each={props.game.maturityDescriptors}>{(t) => <Badge>{t}</Badge>}</For>
                    </div>
                  </Show>
                  <Show when={props.wiki.article}>
                    {(a) => (
                      <>
                        <Divider />
                        <WikiTeaser view={props.wiki} article={a()} />
                      </>
                    )}
                  </Show>
                </Card>
                <Facts columns>
                  <FactGroup title="Experience">
                    <Fact label="Genre">{g().genreL1 || g().genre || "–"}</Fact>
                    <Show when={g().genreL2}>
                      <Fact label="Subgenre">{g().genreL2}</Fact>
                    </Show>
                    <Fact label="Maturity">{props.game.maturity ?? "–"}</Fact>
                    <Fact label="Created">{when(g().created, date)}</Fact>
                    <Fact label="Updated">{when(g().updated, ago)}</Fact>
                  </FactGroup>
                  <FactGroup title="Servers">
                    <Fact label="Server size">{g().maxPlayers}</Fact>
                    <Fact label="Private servers">{g().privateServersAllowed ? "Allowed" : "Off"}</Fact>
                    <Fact label="Avatar type">{g().avatarType || "–"}</Fact>
                  </FactGroup>
                  <FactGroup title="Building">
                    <Fact label="Copying">{yesNo(g().copyingAllowed)}</Fact>
                    <Fact label="Gear">{g().gear.length > 0 ? g().gear.join(", ") : "None"}</Fact>
                  </FactGroup>
                </Facts>
                <Card class="flex items-start gap-3 px-4 py-3.5">
                  <Show
                    when={g().creatorIsGroup}
                    fallback={<Avatar size="md" name={g().creatorName} src={thumbUrl("headshot", g().creatorId)} />}
                  >
                    <Thumbnail
                      class="w-10 shrink-0"
                      alt=""
                      src={thumbUrl("group_icon", g().creatorId)}
                      fallback="communities"
                    />
                  </Show>
                  <div class="flex min-w-0 flex-1 flex-col gap-0.5">
                    <span class="flex items-center gap-1">
                      <TextLink tone="name" onClick={openCreator}>
                        {g().creatorName}
                      </TextLink>
                      <Show when={g().creatorVerified}>
                        <VerifiedBadge />
                      </Show>
                      <span class="text-sm text-fg-muted">· {g().creatorIsGroup ? "Community" : "User"}</span>
                    </span>
                    <Show when={bio.text !== null} fallback={<Skeleton class="h-10 w-64 rounded-md" />}>
                      <Show when={bio.text} fallback={<span class="text-sm text-fg-muted">No description.</span>}>
                        {(t) => (
                          <Clamp lines={3} class="text-sm text-fg-muted">
                            <Description text={t()} />
                          </Clamp>
                        )}
                      </Show>
                    </Show>
                  </div>
                </Card>
              </div>
              <aside class="flex min-w-0 flex-col gap-4" aria-label="Where to play">
                <FriendsHere list={props.friendServers} />
                <BestServers placeId={props.game.placeId} onAll={props.onAllServers} />
              </aside>
            </div>
          </div>
        )
      }}
    </Show>
  )
}

/** Friends of the active account playing now, one row each, with Join on their server. Hidden
 * when none are. */
function FriendsHere(props: { list: List<ServerRow> }) {
  const rows = () => props.list.items.flatMap((s) => s.friends.map((f) => ({ friend: f, server: s })))
  return (
    <Show when={rows().length > 0}>
      <Card class="flex flex-col gap-1 p-2">
        <SectionHeader class="px-2 pt-1" title="Friends here" meta={rows().length} size="group" />
        <For each={rows().slice(0, 6)}>
          {(r) => (
            <ListItem
              leading={
                <Avatar size="sm" name={friendName(r.friend)} src={thumbUrl("headshot", r.friend)} presence="game" />
              }
              title={friendName(r.friend)}
              subtitle={`${serverName(r.server.jobId)} · ${r.server.playing}/${r.server.maxPlayers}`}
              actions={
                <Button size="sm" onClick={() => run("game.join_job", r.server.jobId)}>
                  Join
                </Button>
              }
            />
          )}
        </For>
      </Card>
    </Show>
  )
}

/** Open servers with friends first, then the lowest ping. */
const byBest = (a: ServerRow, b: ServerRow) => byServer.friends(a, b) || byServer.ping(a, b)

/** The few best open servers, each with how full it is, its ping and Join. */
function BestServers(props: { placeId: number; onAll: () => void }) {
  const list = useLive(
    () => props.placeId,
    (id) => commands.gameServers(id),
    (): Servers => ({ loading: true, error: null, fetchedAt: null, servers: [], more: false }),
  )
  const best = createMemo(() =>
    list.servers
      .filter((s) => s.playing < s.maxPlayers)
      .sort(byBest)
      .slice(0, 5),
  )
  return (
    <Card class="flex flex-col gap-1 p-2">
      <SectionHeader
        class="px-2 pt-1"
        title="Best servers"
        size="group"
        actions={
          <TextLink class="text-sm" onClick={() => props.onAll()}>
            All servers
          </TextLink>
        }
      />
      <Switch>
        <Match when={best().length > 0}>
          <For each={best()}>
            {(s) => (
              <div class="flex items-center gap-3 rounded-md px-2 py-1.5">
                <div class="flex min-w-0 flex-1 flex-col gap-1">
                  <div class="flex items-baseline justify-between gap-2 text-sm">
                    <span class="truncate">{serverName(s.jobId)}</span>
                    <span class="shrink-0 text-fg-muted tabular-nums">
                      {s.playing}/{s.maxPlayers}
                      <Show when={s.ping !== null}> · {s.ping} ms</Show>
                    </span>
                  </div>
                  <Meter
                    class="w-full"
                    label="Players"
                    value={s.playing}
                    max={s.maxPlayers}
                    valueLabel=""
                    tone="capacity"
                  />
                </div>
                <Button size="sm" onClick={() => run("game.join_job", s.jobId)}>
                  Join
                </Button>
              </div>
            )}
          </For>
        </Match>
        <Match when={list.loading}>
          <div class="flex flex-col gap-2 px-2 py-1.5">
            <For each={[1, 2, 3]}>{() => <Skeleton class="h-8 w-full rounded-md" />}</For>
          </div>
        </Match>
        <Match when={list.error}>
          <span class="px-2 py-1.5 text-sm text-fg-muted">Couldn't load servers.</span>
        </Match>
        <Match when={true}>
          <span class="px-2 py-1.5 text-sm text-fg-muted">No open servers right now.</span>
        </Match>
      </Switch>
    </Card>
  )
}

/** Servers with friends of the active account in them, from Roblox's friends' server list. */
function FriendServersPanel(props: { list: List<ServerRow> }) {
  const list = props.list
  return (
    <ListState list={list} icon="friends" empty="No friends are playing">
      <VirtualList items={list.items} estimateRowHeight={64}>
        {(s) => (
          <ListItem
            leading={
              <AvatarStack
                on="canvas"
                max={4}
                items={s.friends.map((f) => ({
                  name: friendName(f),
                  label: friendName(f),
                  src: thumbUrl("headshot", f),
                }))}
              />
            }
            title={s.friends.map(friendName).join(", ") || "Friends"}
            subtitle={`${serverName(s.jobId)} · ${s.playing} of ${s.maxPlayers} players${s.ping === null ? "" : ` · ${s.ping} ms`}`}
            actions={
              <>
                <Menu size="sm" items={joinAsItems("game.join_job", s.jobId)}>
                  Join as…
                </Menu>
                <Button size="sm" onClick={() => run("game.join_job", s.jobId)}>
                  Join
                </Button>
              </>
            }
          />
        )}
      </VirtualList>
    </ListState>
  )
}

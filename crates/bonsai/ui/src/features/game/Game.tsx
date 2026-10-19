import { useParams } from "@solidjs/router"
import { createEffect, createMemo, createSignal, For, onCleanup, Show } from "solid-js"
import { BuyForm } from "~/lib/buy-form"
import { useCommandContext, useSubTabs } from "~/lib/commands/hooks"
import { handle, run } from "~/lib/commands/registry"
import { ago, compact, date, percent } from "~/lib/format"
import {
  type BadgeRow,
  commands,
  type EventRow,
  type GameCard,
  type Game as GameView,
  type List,
  type PassRow,
  type PrivateServerRow,
  type ServerOffer,
  type ServerRow,
  type Servers,
  type WikiPageView,
} from "~/lib/ipc/bindings"
import { Description } from "~/lib/linked-text"
import { emptyList, ListState } from "~/lib/list-state"
import { entityMenu, joinAsItems } from "~/lib/menus"
import { ServerControls } from "~/lib/private-server"
import { friends, useLive } from "~/lib/store"
import { setTabTitle } from "~/lib/tabs"
import { thumbUrl } from "~/lib/thumb"
import { GameTile, tileGrid } from "~/lib/tiles"
import {
  Alert,
  Avatar,
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
  MediaFrame,
  Menu,
  Meter,
  Page,
  Robux,
  SegmentedControl,
  Skeleton,
  Stat,
  showToast,
  TabPanel,
  Tabs,
  TextField,
  TextLink,
  Thumbnail,
  ToggleButton,
  Tooltip,
  useDialog,
  VerifiedBadge,
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

/** A list that has finished loading with nothing in it. Errors keep their tab so they show. */
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
    return d ? { kind: "game", id: d.rootPlaceId, name: d.name } : null
  })
  const [section, setSection] = createSignal("about")
  const sections = createMemo(() => {
    const hidden = new Set<string>()
    if (isEmpty(friendServers)) hidden.add("friends")
    if (isEmpty(privateServers) && !offer.offer) hidden.add("private")
    if (isEmpty(passes)) hidden.add("store")
    if (isEmpty(badges)) hidden.add("badges")
    if (isEmpty(events)) hidden.add("events")
    if (isEmpty(recommended)) hidden.add("recommended")
    if (!wiki.article) hidden.add("wiki")
    return allSections.filter((t) => !hidden.has(t.value))
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

  const id = () => String(placeId())

  return (
    <Page>
      <Show when={game.details} fallback={<HeaderSkeleton />}>
        {(d) => (
          <>
            {/* Media beside the title and actions, so the server list starts near the top.
                Stacks when the page is narrow. */}
            <header class="@container">
              <div class="grid grid-cols-1 items-start gap-4 @xl:grid-cols-[minmax(0,5fr)_minmax(0,6fr)]">
                <MediaStrip media={game.media} universeId={d().universeId} name={d().name} />
                <div class="flex min-w-0 flex-col gap-3">
                  <div class="flex flex-col">
                    <h1 class="truncate text-2xl font-medium">{d().name}</h1>
                    <span class="text-fg-muted">
                      by{" "}
                      <TextLink
                        tone={d().creatorVerified ? "verified" : "muted"}
                        onClick={() => run(d().creatorIsGroup ? "group.open" : "user.open", String(d().creatorId))}
                      >
                        {d().creatorName}
                      </TextLink>
                      <Show when={d().creatorVerified}>
                        <VerifiedBadge class="ml-1" />
                      </Show>
                    </span>
                  </div>
                  <div class="flex gap-1.5">
                    <Button
                      variant="primary"
                      size="lg"
                      icon="play"
                      class="flex-1"
                      onClick={() => run("game.join", id())}
                    >
                      Join
                    </Button>
                    <Menu
                      size="lg"
                      items={joinAsItems("game.join_as", id())}
                      disabled={joinAsItems("game.join_as", id()).length === 0}
                    >
                      Join as…
                    </Menu>
                    <Menu size="lg" icon="more" label="More" items={entityMenu(`game:${placeId()}`)} />
                  </div>
                  <div class="grid grid-cols-4 gap-1.5">
                    <ToggleButton
                      variant="secondary"
                      icon="thumbs-up"
                      pressedIcon="thumbs-up-fill"
                      pressed={game.me?.vote === true}
                      disabled={!game.me}
                      onChange={() => run("game.like")}
                    >
                      {compact(d().upVotes ?? 0)}
                    </ToggleButton>
                    <ToggleButton
                      variant="secondary"
                      icon="thumbs-down"
                      pressedIcon="thumbs-down-fill"
                      pressed={game.me?.vote === false}
                      disabled={!game.me}
                      onChange={() => run("game.dislike")}
                    >
                      {compact(d().downVotes ?? 0)}
                    </ToggleButton>
                    <ToggleButton
                      variant="secondary"
                      icon="star"
                      pressedIcon="star-fill"
                      pressed={game.me?.favorited ?? false}
                      disabled={!game.me}
                      onChange={() => run("game.favorite")}
                    >
                      Favorite
                    </ToggleButton>
                    <ToggleButton
                      variant="secondary"
                      icon="bell"
                      pressedIcon="bell-fill"
                      pressed={game.me?.following ?? false}
                      disabled={!game.me || (!game.me.following && !game.me.canFollow)}
                      onChange={() => run("game.notify")}
                    >
                      Notify
                    </ToggleButton>
                  </div>
                  <dl class="grid grid-cols-5 gap-x-4 border-t border-line pt-3">
                    <Stat label="Playing">{compact(d().playing)}</Stat>
                    <Stat label="Visits">{compact(d().visits)}</Stat>
                    <Stat label="Rating">{rating(d().upVotes, d().downVotes)}</Stat>
                    <Stat label="Favorites">{compact(d().favorites)}</Stat>
                    <Stat label="Updated">{when(d().updated, ago)}</Stat>
                  </dl>
                </div>
              </div>
            </header>
          </>
        )}
      </Show>

      <Tabs aria-label="Experience sections" items={sections()} value={section()} onChange={setSection}>
        <TabPanel value="about">
          <AboutPanel game={game} wiki={wiki} />
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
    <div class="grid grid-cols-1 gap-4 @xl:grid-cols-[minmax(0,5fr)_minmax(0,6fr)]">
      <Skeleton class="aspect-video w-full max-w-md rounded-md" />
      <div class="flex flex-col gap-3">
        <Skeleton class="h-8 w-2/3 rounded-md" />
        <Skeleton class="h-10 w-full rounded-md" />
        <Skeleton class="h-32 w-full rounded-md" />
      </div>
    </div>
  )
}

/** One image large with previous and next, and up to four more below. Trailers show their
 * preview image and title; they play on the website. Falls back to the game icon. */
function MediaStrip(props: { media: GameView["media"]; universeId: number; name: string }) {
  const [shown, setShown] = createSignal(0)
  const main = () => props.media[shown()] ?? props.media[0]
  const step = (by: number) => setShown((i) => (i + by + props.media.length) % props.media.length)
  // The four thumbnails around the shown one, so the strip follows previous and next.
  const strip = () => {
    const start = Math.max(0, Math.min(shown() - 1, props.media.length - 4))
    return props.media.slice(start, start + 4).map((m, i) => ({ m, i: start + i }))
  }
  return (
    <div class="order-last flex max-w-md flex-col gap-1.5 @xl:order-none @xl:max-w-none">
      <Show
        when={main()}
        fallback={
          <Thumbnail
            shape="wide"
            alt={props.name}
            src={thumbUrl("game_icon", props.universeId, true)}
            fallback="game"
          />
        }
      >
        {(m) => (
          <MediaFrame
            index={shown()}
            count={props.media.length}
            onStep={step}
            caption={m().video ? `${m().videoTitle || "Trailer"} (plays on the website)` : undefined}
            captionIcon="video"
          >
            <Thumbnail
              shape="wide"
              alt={m().alt || props.name}
              src={thumbUrl("game_thumbnail", m().imageId, true)}
              fallback="game"
            />
          </MediaFrame>
        )}
      </Show>
      <Show when={props.media.length > 1}>
        <div class="grid grid-cols-4 gap-1.5">
          <For each={strip()}>
            {(t) => (
              <button
                type="button"
                class="rounded-md"
                aria-label={t.m.videoTitle || t.m.alt || `Image ${t.i + 1}`}
                aria-pressed={shown() === t.i}
                onClick={() => setShown(t.i)}
              >
                <Thumbnail shape="wide" alt="" src={thumbUrl("game_thumbnail", t.m.imageId)} fallback="game" />
              </button>
            )}
          </For>
        </div>
      </Show>
    </div>
  )
}

/** Loading, error and empty states shared by the tab lists. Renders `children` once there are
 * items; keeps showing them while a refresh is in flight. */

const sorts = [
  { value: "friends", label: "Friends" },
  { value: "ping", label: "Ping" },
  { value: "players", label: "Players" },
  { value: "free", label: "Free slots" },
]

const byServer: Record<string, (a: ServerRow, b: ServerRow) => number> = {
  friends: (a, b) => b.friends.length - a.friends.length,
  ping: (a, b) => (a.ping ?? Number.MAX_SAFE_INTEGER) - (b.ping ?? Number.MAX_SAFE_INTEGER),
  players: (a, b) => b.playing - a.playing,
  free: (a, b) => b.maxPlayers - b.playing - (a.maxPlayers - a.playing),
}

function ServersPanel(props: { placeId: number; max: number }) {
  const list = useLive(
    () => props.placeId,
    (id) => commands.gameServers(id),
    (): Servers => ({ loading: true, error: null, fetchedAt: null, servers: [], more: false }),
  )
  const [sort, setSort] = createSignal("friends")
  const [reversed, setReversed] = createSignal(false)
  const [hideFull, setHideFull] = createSignal(false)
  const [friendsOnly, setFriendsOnly] = createSignal(false)
  const [selected, setSelected] = createSignal<string>()
  onCleanup(handle("game.join_server", () => selected() && run("game.join_job", selected() ?? null)))

  const shown = createMemo(() =>
    list.servers
      .filter((s) => (!hideFull() || s.playing < s.maxPlayers) && (!friendsOnly() || s.friends.length > 0))
      .sort((a, b) => (reversed() ? -1 : 1) * byServer[sort()](a, b)),
  )

  const friendName = (id: number) => friends.friends.find((f) => f.id === id)?.displayName ?? String(id)

  const columns: Column<ServerRow>[] = [
    {
      key: "players",
      label: "Players",
      width: "9rem",
      cell: (s) => <Meter class="w-full" label="Players" value={s.playing} max={s.maxPlayers} tone="capacity" />,
    },
    { key: "ping", label: "Ping", width: "4rem", cell: (s) => (s.ping === null ? "–" : `${s.ping} ms`) },
    { key: "fps", label: "FPS", width: "3rem", cell: (s) => (s.fps === null ? "–" : Math.round(s.fps)) },
    {
      key: "friends",
      label: "Friends",
      width: "minmax(6rem, 1fr)",
      cell: (s) => (
        <For each={s.friends}>
          {(f) => (
            <Tooltip content={friendName(f)}>
              <Avatar size="xs" name={friendName(f)} src={thumbUrl("headshot", f)} />
            </Tooltip>
          )}
        </For>
      ),
    },
    { key: "id", label: "Server", width: "5.5rem", mono: true, cell: (s) => s.jobId.slice(0, 8) },
    {
      key: "actions",
      label: "",
      width: "10rem",
      align: "end",
      cell: (s) => (
        <>
          <IconButton
            size="sm"
            icon="link"
            label="Copy invite link"
            onClick={() => run("game.copy_server_link", s.jobId)}
          />
          <Menu size="sm" items={joinAsItems("game.join_job", s.jobId)}>
            Join as…
          </Menu>
          <Button size="sm" onClick={() => run("game.join_job", s.jobId)}>
            Join
          </Button>
        </>
      ),
    },
  ]

  return (
    <div class="flex flex-col gap-3">
      <div class="flex flex-wrap items-center gap-2">
        <Button onClick={() => run("game.join_best")}>Join best for me</Button>
        <Button onClick={() => run("game.join_smallest")}>Smallest</Button>
        <Button onClick={() => run("game.join_random")}>Random</Button>
        <span class="ml-auto text-sm text-fg-muted">
          {shown().length} of {list.servers.length} servers
          <Show when={list.fetchedAt}>{(t) => ` · ${ago(t())}`}</Show>
        </span>
        <Button icon="refresh" disabled={list.loading} onClick={() => run("game.refresh_servers")}>
          Refresh
        </Button>
      </div>
      <div class="flex flex-wrap items-center gap-3">
        <span class="text-sm text-fg-muted">Sort</span>
        <SegmentedControl aria-label="Sort servers" size="sm" value={sort()} onChange={setSort} options={sorts} />
        <Tooltip content={reversed() ? "Reversed order" : "Normal order"}>
          <ToggleButton
            size="sm"
            variant="ghost"
            icon="sort-desc"
            pressedIcon="sort-asc"
            pressed={reversed()}
            onChange={setReversed}
          >
            Reverse
          </ToggleButton>
        </Tooltip>
        <Divider orientation="vertical" />
        <Checkbox checked={hideFull()} onChange={setHideFull}>
          Hide full
        </Checkbox>
        <Checkbox checked={friendsOnly()} onChange={setFriendsOnly}>
          Friends only
        </Checkbox>
      </div>
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
        <For each={list.items}>
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
        </For>
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
      <div class="grid grid-cols-[repeat(auto-fill,minmax(10rem,1fr))] gap-3">
        <For each={list.items}>
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
                <Show when={p.price !== null} fallback={<span class="text-sm text-fg-muted">Off sale</span>}>
                  <Robux amount={p.price ?? 0} />
                </Show>
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
        </For>
      </div>
    </ListState>
  )
}

function BadgesPanel(props: { list: List<BadgeRow> }) {
  const list = props.list
  return (
    <ListState list={list} icon="badge" empty="No badges" more="badges">
      <For each={list.items}>
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
      </For>
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
      <For each={list.items}>
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
      </For>
    </ListState>
  )
}

function RecommendedPanel(props: { list: List<GameCard> }) {
  const list = props.list
  return (
    <ListState list={list} icon="game" empty="No recommendations">
      <div class={tileGrid}>
        <For each={list.items}>{(g) => <GameTile game={g} />}</For>
      </div>
    </ListState>
  )
}

/** Description and the creator on the left; every detail in one panel on the right. */
function AboutPanel(props: { game: GameView; wiki: WikiPageView }) {
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
            <div class="grid grid-cols-1 gap-4 @2xl:grid-cols-[minmax(0,1fr)_minmax(16rem,20rem)]">
              <Card class="flex h-fit min-w-0 flex-col">
                <section class="flex flex-col gap-1.5 px-3.5 py-3">
                  <h3 class="text-sm font-medium text-fg-muted">Description</h3>
                  <Show when={g().description} fallback={<span class="text-fg-muted">No description.</span>}>
                    <Clamp lines={8}>
                      <Description text={g().description} />
                    </Clamp>
                  </Show>
                </section>
                <Divider />
                <section class="flex flex-col gap-1.5 px-3.5 py-3">
                  <h3 class="text-sm font-medium text-fg-muted">Creator</h3>
                  <div class="flex items-start gap-3">
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
                    <div class="flex min-w-0 flex-col gap-0.5">
                      <span class="flex items-center gap-1">
                        <TextLink tone={g().creatorVerified ? "verified" : "name"} onClick={openCreator}>
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
                  </div>
                </section>
                <Show when={props.wiki.article}>
                  {(a) => (
                    <>
                      <Divider />
                      <WikiTeaser view={props.wiki} article={a()} />
                    </>
                  )}
                </Show>
              </Card>
              <Facts class="h-fit">
                <FactGroup title="Experience">
                  <Fact label="Genre">{g().genreL1 || g().genre || "–"}</Fact>
                  <Show when={g().genreL2}>
                    <Fact label="Subgenre">{g().genreL2}</Fact>
                  </Show>
                  <Fact label="Maturity">{props.game.maturity ?? "–"}</Fact>
                  <Show when={props.game.maturityDescriptors.length > 0}>
                    <Fact label="Contains">
                      <For each={props.game.maturityDescriptors}>{(t) => <Badge>{t}</Badge>}</For>
                    </Fact>
                  </Show>
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
            </div>
          </div>
        )
      }}
    </Show>
  )
}

/** Servers with friends of the active account in them, from Roblox's friends' server list. */
function FriendServersPanel(props: { list: List<ServerRow> }) {
  const list = props.list
  const friendName = (id: number) => friends.friends.find((f) => f.id === id)?.displayName ?? String(id)
  return (
    <ListState list={list} icon="friends" empty="No friends are playing">
      <For each={list.items}>
        {(s) => (
          <ListItem
            leading={
              <div class="flex -space-x-1.5">
                <For each={s.friends.slice(0, 4)}>
                  {(f) => <Avatar size="sm" name={friendName(f)} src={thumbUrl("headshot", f)} />}
                </For>
              </div>
            }
            title={s.friends.map(friendName).join(", ") || "Friends"}
            subtitle={`${s.playing} of ${s.maxPlayers} players${s.ping === null ? "" : ` · ${s.ping} ms`}`}
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
      </For>
    </ListState>
  )
}

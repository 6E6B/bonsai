import { useParams, useSearchParams } from "@solidjs/router"
import { createEffect, createMemo, createSignal, Match, on, onCleanup, Show, Switch } from "solid-js"
import { useCommandContext, useListCursor, useSubTabs } from "~/lib/commands/hooks"
import { run } from "~/lib/commands/registry"
import { compact } from "~/lib/format"
import {
  type Connection,
  commands,
  type GameCard,
  type GroupCard,
  type ItemCard,
  type List,
  type OutfitView,
  type ProfileView,
  type TraderView,
  type UserBadge,
} from "~/lib/ipc/bindings"
import { emptyList, ListState } from "~/lib/list-state"
import { userMenu } from "~/lib/menus"
import { useLive } from "~/lib/store"
import { setTabTitle } from "~/lib/tabs"
import { thumbUrl } from "~/lib/thumb"
import { GameTile, ItemTile, PlayingRow, TILE_WIDTH } from "~/lib/tiles"
import {
  Avatar,
  Badge,
  Button,
  Card,
  ContextMenu,
  IconButton,
  ListItem,
  Menu,
  Page,
  Robux,
  SegmentedControl,
  Skeleton,
  showToast,
  TabPanel,
  Tabs,
  TextField,
  TextLink,
  Thumbnail,
  Tooltip,
  VerifiedBadge,
  VirtualGrid,
  VirtualList,
} from "~/lib/ui"
import { rolimonsFailed, VALUES_SOURCE } from "~/lib/values"
import { useWikiAbout, WikiSummary, WikiTab } from "~/lib/wiki-panels"
import { Outfits } from "./Outfits"
import { BadgeTile, Overview } from "./Overview"

const allSections = [
  { value: "about", label: "Overview" },
  { value: "outfits", label: "Outfits" },
  { value: "connections", label: "Connections" },
  { value: "experiences", label: "Experiences" },
  { value: "badges", label: "Badges" },
  { value: "communities", label: "Communities" },
  { value: "items", label: "Collectibles" },
  { value: "wiki", label: "Wiki" },
]

const experienceLists = [
  { value: "creations", label: "Created" },
  { value: "favorites", label: "Favorites" },
  { value: "followed", label: "Following" },
] as const
type ExperienceList = (typeof experienceLists)[number]["value"]
const isExperienceList = (v: string): v is ExperienceList => experienceLists.some((l) => l.value === v)

/** A list that has finished loading with nothing in it. Errors keep their tab so they show. */
const isEmpty = (l: { loading: boolean; error: string | null; items: unknown[] }) =>
  !l.loading && !l.error && l.items.length === 0

/** A user: picture, names, presence and counts beside one or two actions, then sub-tabs.
 * Everything else (trade, follow, decline, copy) is in the More menu. Your own accounts'
 * profiles hide the friend actions. */
export default function Profile() {
  const params = useParams()
  const userId = () => Number(params.id)
  const view = useLive(
    userId,
    (id) => commands.profile(id),
    (): ProfileView => ({
      loading: true,
      error: null,
      user: null,
      presence: "offline",
      place: null,
      placeId: null,
      universeId: null,
      relation: null,
      mine: false,
      pastNames: [],
      lastOnline: null,
      lastOnlineBy: null,
      lastLocation: null,
      unfriendedAt: null,
      mutualFriends: null,
      mutualMore: false,
      visits: null,
      favoritesReceived: null,
      nickname: null,
      note: null,
    }),
  )
  const creations = useLive(userId, commands.profileCreations, emptyList<GameCard>)
  const favorites = useLive(userId, commands.profileFavorites, emptyList<GameCard>)
  const followed = useLive(userId, commands.profileFollowedGames, emptyList<GameCard>)
  const outfits = useLive(userId, commands.profileOutfits, emptyList<OutfitView>)
  const badges = useLive(userId, commands.profileBadges, emptyList<UserBadge>)
  const groups = useLive(userId, commands.profileGroups, emptyList<GroupCard>)
  const collectibles = useLive(userId, commands.profileCollectibles, emptyList<ItemCard>)
  const trading = useLive(
    userId,
    commands.profileTrading,
    (): TraderView => ({ enabled: false, loading: true, error: null, stats: null }),
  )
  createEffect(
    on(
      () => rolimonsFailed(trading.enabled, trading.error, trading.stats !== null),
      (failed) => {
        if (failed) showToast({ title: "Failed to get Rolimons data", variant: "error" })
      },
    ),
  )
  /** Their limiteds' worth for the header, when Rolimons has one. */
  const worth = () => {
    const s = trading.stats
    return s?.tracked && s.value ? s.value : null
  }
  const [search] = useSearchParams()
  const [section, setSection] = createSignal("about")
  const [people, setPeople] = createSignal<PeopleList>("friends")
  const [games, setGames] = createSignal<ExperienceList>("creations")
  // `user.friends` and friends link here with `?tab=connections&list=followers`.
  createEffect(
    on([() => search.tab, () => search.list], ([tab, list]) => {
      if (typeof tab === "string" && allSections.some((t) => t.value === tab)) setSection(tab)
      if (typeof list === "string" && isPeopleList(list)) setPeople(list)
    }),
  )
  const gameLists = { creations, favorites, followed }
  /** Created, favorites and following share one tab; each shows only once it has something. */
  const shownGameLists = createMemo(() => experienceLists.filter((l) => !isEmpty(gameLists[l.value])))
  createEffect(() => {
    const shown = shownGameLists()
    if (shown.length > 0 && !shown.some((l) => l.value === games())) setGames(shown[0].value)
  })
  const wiki = useWikiAbout(() => {
    const name = view.user?.name
    return name ? { kind: "user", id: userId(), name } : null
  })
  const sections = createMemo(() => {
    const hidden = new Set<string>()
    if (shownGameLists().length === 0) hidden.add("experiences")
    if (isEmpty(badges)) hidden.add("badges")
    if (isEmpty(groups)) hidden.add("communities")
    if (isEmpty(collectibles)) hidden.add("items")
    if (!wiki.article) hidden.add("wiki")
    return allSections.filter((t) => !hidden.has(t.value))
  })
  createEffect(() => {
    if (!sections().some((t) => t.value === section())) setSection("about")
  })
  useSubTabs(sections, section, setSection)
  /** "See all" on the overview: that tab, on the list it came from. */
  const seeAll = (tab: string, list?: string) => {
    if (list && isPeopleList(list)) setPeople(list)
    if (list && isExperienceList(list)) setGames(list)
    setSection(tab)
  }
  useCommandContext(() => `user:${userId()}`)
  createEffect(() => {
    const u = view.user
    if (u) setTabTitle(`/users/${params.id}`, u.displayName || u.name)
  })

  const id = () => String(userId())
  const rel = () => view.relation
  /** How they relate to the active account, e.g. "Friends · Following". */
  const relationText = () => {
    const r = rel()
    if (!r) return ""
    const parts: string[] = []
    if (r.friend === "friends") parts.push("Friends")
    else if (r.friend === "sent") parts.push("Request sent")
    else if (r.friend === "received") parts.push("Request received")
    if (r.following) parts.push("Following")
    if (r.followedBy) parts.push("Follows you")
    if (r.blocked) parts.push("Blocked")
    return parts.join(" · ") || "None"
  }
  return (
    <Page>
      <Show
        when={view.user}
        fallback={
          <Show when={view.error} fallback={<Skeleton class="h-24 w-full rounded-md" />}>
            {(e) => <span class="text-fg-muted">{e()}</span>}
          </Show>
        }
      >
        {(u) => (
          <header class="flex flex-wrap items-center gap-x-6 gap-y-3">
            <div class="flex min-w-0 flex-1 items-center gap-4">
              <Avatar
                size="xl"
                name={u().displayName}
                presence={view.presence}
                src={thumbUrl("headshot", userId(), true)}
              />
              <div class="flex min-w-0 flex-col gap-0.5">
                <h1 class="flex items-center gap-2 truncate text-2xl font-medium">
                  <span class={u().verified ? "truncate text-verified" : "truncate"}>{u().displayName}</span>
                  <Show when={u().verified}>
                    <VerifiedBadge size={18} />
                  </Show>
                  <Show when={u().banned}>
                    <Badge variant="danger">Banned</Badge>
                  </Show>
                </h1>
                <span class="truncate text-fg-muted">
                  @{u().name}
                  <Show when={view.nickname}>
                    {(n) => (
                      <Tooltip content="Your nickname for them" class="ml-2">
                        <Badge>{n()}</Badge>
                      </Tooltip>
                    )}
                  </Show>
                </span>
                <span class="mt-1 flex flex-wrap gap-x-4 text-sm text-fg-muted">
                  <CountLink n={u().friends} label="friends" command="user.friends" id={id()} />
                  <CountLink n={u().followers} label="followers" command="user.followers" id={id()} />
                  <CountLink n={u().following} label="following" command="user.following" id={id()} />
                  <Show when={worth()}>
                    {(v) => (
                      <Tooltip content={`Value of their limiteds (${VALUES_SOURCE})`} class="inline-flex items-center">
                        <Robux class="font-medium text-fg" amount={v()} compact />
                        <span class="ml-1">value</span>
                      </Tooltip>
                    )}
                  </Show>
                </span>
              </div>
            </div>
            <div class="flex items-center gap-1.5">
              <Show
                when={!view.mine}
                fallback={
                  <>
                    <Button icon="avatar" onClick={() => run("nav.avatar")}>
                      Edit avatar
                    </Button>
                    <Tooltip content="Inventory">
                      <IconButton
                        variant="secondary"
                        icon="inventory"
                        label="Inventory"
                        onClick={() => run("inventory.view", id())}
                      />
                    </Tooltip>
                  </>
                }
              >
                <FriendButton status={rel()?.friend ?? "none"} id={id()} />
                <Tooltip content="Inventory">
                  <IconButton
                    variant="secondary"
                    icon="inventory"
                    label="Inventory"
                    onClick={() => run("inventory.view", id())}
                  />
                </Tooltip>
                <Tooltip content="Message">
                  <IconButton
                    variant="secondary"
                    icon="chat"
                    label="Message"
                    onClick={() => run("user.message", id())}
                  />
                </Tooltip>
              </Show>
              <Menu
                icon="more"
                label="More"
                items={userMenu(userId(), {
                  friend: rel()?.friend === "friends",
                  blocked: rel()?.blocked,
                  inGame: view.presence === "game",
                  placeId: view.placeId,
                  following: view.mine ? undefined : (rel()?.following ?? false),
                  requested: rel()?.friend === "received",
                  onProfile: true,
                })}
              />
            </div>
          </header>
        )}
      </Show>

      <Show when={view.presence === "game" && view.placeId}>
        {(placeId) => (
          <Card class="p-1">
            <PlayingRow
              userId={userId()}
              placeId={placeId()}
              universeId={view.universeId}
              name={view.place ?? "In a game"}
              noJoin={view.mine}
              class="rounded-sm"
            />
          </Card>
        )}
      </Show>

      <Tabs aria-label="Profile sections" items={sections()} value={section()} onChange={setSection}>
        <TabPanel value="about" class="flex flex-col gap-4">
          <Show when={wiki.article}>{(a) => <WikiSummary view={wiki} article={a()} facts={3} />}</Show>
          <Overview
            userId={userId()}
            view={view}
            relation={relationText()}
            creations={creations}
            favorites={favorites}
            badges={badges}
            groups={groups}
            collectibles={collectibles}
            trading={trading}
            onSeeAll={seeAll}
          />
        </TabPanel>
        <TabPanel value="wiki">
          <WikiTab view={wiki} missing="The Roblox Wiki has no article about this player." />
        </TabPanel>
        <TabPanel value="outfits">
          <Outfits userId={userId()} mine={view.mine} outfits={outfits} />
        </TabPanel>
        <TabPanel value="connections">
          <Connections userId={userId()} list={people()} onList={setPeople} mine={view.mine} />
        </TabPanel>
        <TabPanel value="experiences" class="flex flex-col gap-3">
          <Show when={shownGameLists().length > 1}>
            <SegmentedControl
              size="sm"
              class="self-start"
              aria-label="List"
              options={shownGameLists()}
              value={games()}
              onChange={(v) => isExperienceList(v) && setGames(v)}
            />
          </Show>
          <Switch>
            <Match when={games() === "creations"}>
              <Creations list={creations} />
            </Match>
            <Match when={games() === "favorites"}>
              <Favorites list={favorites} />
            </Match>
            <Match when={games() === "followed"}>
              <Followed list={followed} />
            </Match>
          </Switch>
        </TabPanel>
        <TabPanel value="badges">
          <Badges list={badges} />
        </TabPanel>
        <TabPanel value="communities">
          <Groups list={groups} />
        </TabPanel>
        <TabPanel value="items">
          <Collectibles list={collectibles} />
        </TabPanel>
      </Tabs>
    </Page>
  )
}

/** Add friend, or what happens next with a request. Decline lives in the More menu. */
function FriendButton(props: { status: string; id: string }) {
  return (
    <Show
      when={props.status !== "none"}
      fallback={
        <Button variant="primary" icon="user-plus" onClick={() => run("user.add_friend", props.id)}>
          Add friend
        </Button>
      }
    >
      <Show when={props.status === "received"}>
        <Button variant="primary" icon="user-plus" onClick={() => run("user.accept_friend", props.id)}>
          Accept request
        </Button>
      </Show>
      <Show when={props.status === "sent"}>
        <Button disabled>Request sent</Button>
      </Show>
    </Show>
  )
}

/** A header count that opens that list in Connections. A dash, not a link, when Roblox didn't
 * send it. */
function CountLink(props: { n: number | null; label: string; command: string; id: string }) {
  return (
    <span>
      <Show when={props.n !== null} fallback={<Count n={null} />}>
        <TextLink onClick={() => run(props.command, props.id)}>
          <Count n={props.n} />
        </TextLink>
      </Show>{" "}
      {props.label}
    </span>
  )
}

/** A count, compacted, or a dash when Roblox didn't send it. */
function Count(props: { n: number | null }) {
  return <span class="font-medium text-fg tabular-nums">{props.n === null ? "–" : compact(props.n)}</span>
}

function Creations(props: { list: List<GameCard> }) {
  const list = props.list
  return (
    <ListState list={list} icon="game" empty="No experiences">
      <VirtualGrid items={list.items} minItemWidth={TILE_WIDTH} estimateRowHeight={190} gap={12}>
        {(g) => <GameTile game={g} />}
      </VirtualGrid>
    </ListState>
  )
}

function Favorites(props: { list: List<GameCard> }) {
  const list = props.list
  return (
    <ListState list={list} icon="star" empty="No favorites" more="favorites">
      <VirtualGrid items={list.items} minItemWidth={TILE_WIDTH} estimateRowHeight={190} gap={12}>
        {(g) => <GameTile game={g} />}
      </VirtualGrid>
    </ListState>
  )
}

function Followed(props: { list: List<GameCard> }) {
  const list = props.list
  return (
    <ListState list={list} icon="bell" empty="Not following any experiences">
      <VirtualGrid items={list.items} minItemWidth={TILE_WIDTH} estimateRowHeight={190} gap={12}>
        {(g) => <GameTile game={g} />}
      </VirtualGrid>
    </ListState>
  )
}

function Badges(props: { list: List<UserBadge> }) {
  const list = props.list
  return (
    <ListState list={list} icon="badge" empty="No badges" more="badges">
      <VirtualGrid items={list.items} minItemWidth={96} estimateRowHeight={150} gap={12}>
        {(b) => <BadgeTile badge={b} />}
      </VirtualGrid>
    </ListState>
  )
}

function Groups(props: { list: List<GroupCard> }) {
  const list = props.list
  return (
    <ListState list={list} icon="communities" empty="Not in any communities">
      <VirtualGrid items={list.items} estimateRowHeight={56} gap={0}>
        {(c) => (
          <ListItem
            leading={<Thumbnail class="w-10" alt="" src={thumbUrl("group_icon", c.id)} fallback="communities" />}
            title={
              <Show when={c.verified} fallback={c.name}>
                <span class="text-verified">{c.name}</span> <VerifiedBadge />
              </Show>
            }
            subtitle={`${compact(c.members)} members · ${c.role}`}
            meta={
              <Show when={c.primary}>
                <Badge variant="accent">Primary</Badge>
              </Show>
            }
            onClick={() => run("group.open", String(c.id))}
          />
        )}
      </VirtualGrid>
    </ListState>
  )
}

function Collectibles(props: { list: List<ItemCard> }) {
  const list = props.list
  return (
    <ListState list={list} icon="inventory" empty="No collectibles">
      <VirtualGrid items={list.items} minItemWidth={TILE_WIDTH} estimateRowHeight={190} gap={12}>
        {(i) => <ItemTile item={i} showSerial showRap />}
      </VirtualGrid>
    </ListState>
  )
}

const peopleLists = [
  { value: "friends", label: "Friends" },
  { value: "followers", label: "Followers" },
  { value: "following", label: "Following" },
] as const
type PeopleList = (typeof peopleLists)[number]["value"]
const isPeopleList = (v: string): v is PeopleList => peopleLists.some((l) => l.value === v)

const emptyText: Record<PeopleList, string> = {
  friends: "No friends",
  followers: "No followers",
  following: "Not following anyone",
}

/** Their friends, followers or following, J / K / Enter to move and open. Friends are
 * searched on Roblox (so it finds people on pages not loaded yet) and list mutual friends
 * first; followers and following can only be filtered by what has loaded. */
function Connections(props: { userId: number; list: PeopleList; onList: (l: PeopleList) => void; mine: boolean }) {
  const [query, setQuery] = createSignal("")
  const [searched, setSearched] = createSignal("")
  const [mutualOnly, setMutualOnly] = createSignal("all")
  const [cursor, setCursor] = createSignal(0)
  const friends = () => props.list === "friends"
  // Search Roblox once typing pauses, not on every key.
  createEffect(() => {
    const q = query().trim()
    const t = setTimeout(() => setSearched(friends() ? q : ""), 300)
    onCleanup(() => clearTimeout(t))
  })
  createEffect(on([() => props.userId, () => props.list], () => setCursor(0), { defer: true }))
  const list = useLive(
    () => `${props.userId}\n${props.list}\n${searched()}`,
    (key) => {
      const [id, which, q] = key.split("\n")
      return commands.profilePeople(Number(id), which, q)
    },
    emptyList<Connection>,
  )
  const shown = createMemo(() => {
    let rows = list.items
    if (friends() && mutualOnly() === "mutual") rows = rows.filter((p) => p.friend)
    const q = query().trim().toLowerCase()
    // Friends are already searched on Roblox; this only filters the other lists.
    if (q && !friends()) rows = rows.filter((p) => `${p.name} ${p.displayName}`.toLowerCase().includes(q))
    return rows
  })
  useListCursor({
    count: () => shown().length,
    index: cursor,
    setIndex: setCursor,
    onOpen: (i) => {
      const p = shown()[i]
      if (p) void run("user.open", String(p.id))
    },
  })
  const more = () => (friends() && searched() ? `friends:${searched()}` : props.list)
  // On their friends list your friends are mutual; elsewhere they are just your friends.
  const tag = () => (friends() ? "Mutual" : "Friend")

  return (
    <div class="flex flex-col gap-3">
      <div class="flex flex-wrap items-center gap-2">
        <SegmentedControl
          size="sm"
          aria-label="List"
          options={[...peopleLists]}
          value={props.list}
          onChange={(v) => isPeopleList(v) && props.onList(v)}
        />
        <TextField
          class="min-w-48 flex-1"
          icon="search"
          aria-label={friends() ? "Search friends" : "Filter loaded"}
          placeholder={friends() ? "Search friends" : "Filter loaded"}
          value={query()}
          onChange={setQuery}
        />
        <Show when={friends() && !props.mine}>
          <SegmentedControl
            size="sm"
            aria-label="Show"
            options={[
              { value: "all", label: "Everyone" },
              { value: "mutual", label: "Mutual" },
            ]}
            value={mutualOnly()}
            onChange={setMutualOnly}
          />
        </Show>
      </div>
      <ListState
        list={{ ...list, items: shown() }}
        icon="friends"
        empty={query().trim() || mutualOnly() === "mutual" ? "No one matches" : emptyText[props.list]}
        more={more()}
        autoMore
      >
        <div data-cursor-list>
          <VirtualList items={shown()} activeIndex={cursor()} estimateRowHeight={44} gap={0}>
            {(p, i) => (
              <ContextMenu items={userMenu(p.id, { friend: p.friend, following: p.following ?? undefined })}>
                <ListItem
                  data-index={i}
                  selected={i === cursor()}
                  leading={
                    <Avatar name={p.displayName || p.name} presence={p.presence} src={thumbUrl("headshot", p.id)} />
                  }
                  title={
                    <Show when={p.verified} fallback={p.displayName || p.name || `User ${p.id}`}>
                      <span class="text-verified">{p.displayName || p.name}</span> <VerifiedBadge />
                    </Show>
                  }
                  subtitle={p.name ? `@${p.name}` : undefined}
                  meta={
                    <Show when={p.friend && !props.mine}>
                      <Badge variant="accent">{tag()}</Badge>
                    </Show>
                  }
                  actions={
                    <Show when={p.following !== null}>
                      <Button
                        size="sm"
                        variant={p.following ? "ghost" : "secondary"}
                        onClick={(e: MouseEvent) => {
                          e.stopPropagation()
                          void run(p.following ? "user.unfollow" : "user.follow", String(p.id))
                        }}
                      >
                        {p.following ? "Following" : "Follow"}
                      </Button>
                    </Show>
                  }
                  onClick={() => {
                    setCursor(i)
                    void run("user.open", String(p.id))
                  }}
                />
              </ContextMenu>
            )}
          </VirtualList>
        </div>
      </ListState>
    </div>
  )
}

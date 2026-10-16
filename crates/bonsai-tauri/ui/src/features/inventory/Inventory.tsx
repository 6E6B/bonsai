import { useParams } from "@solidjs/router"
import { createEffect, createMemo, createSignal, For, type JSX, Match, onCleanup, Show, Switch } from "solid-js"
import { createStore, reconcile } from "solid-js/store"
import { handle, menuItem, run } from "~/lib/commands/registry"
import { percent } from "~/lib/format"
import {
  type AvatarView,
  commands,
  type InvCategoryView,
  type InvTotalsView,
  type ItemCard,
  type Paged,
  type PassCard,
  type PlaceCard,
  type UserBadge,
  type UserLookup,
  type WardrobeSection,
} from "~/lib/ipc/bindings"
import { IdFact, ItemPreview, PreviewDescription, PreviewHeader, wantsPreview } from "~/lib/item-preview"
import { emptyList, ListState } from "~/lib/list-state"
import { gameMenu, type ItemMenuExtras, itemTarget, shareMenu } from "~/lib/menus"
import { shell, useLive } from "~/lib/store"
import { thumbUrl } from "~/lib/thumb"
import { ItemTile, Price, TILE_WIDTH } from "~/lib/tiles"
import {
  Button,
  ContextMenu,
  EmptyState,
  Fact,
  FactGroup,
  Facts,
  NavItem,
  Page,
  Robux,
  Select,
  showToast,
  TextField,
  TextLink,
  Thumbnail,
  Tile,
  Tooltip,
  usePrompt,
  VirtualGrid,
} from "~/lib/ui"
import { useValuesRefresh } from "~/lib/value-tags"
import { VALUES_SOURCE } from "~/lib/values"
import { FILTERS, type InvFilter, isFilter, matches, shapeInventory, slotAssetType } from "./filter"

const COLLECTIBLES = "collectibles"
/** Slots with their own lists instead of `inventory_items`. */
const BADGES = "badges"
const PASSES = "passes"
const PLACES = "places"

/** What the drawer shows: an item or bundle (`item:5`, `bundle:5`), a pass (`pass:9`), a badge
 * (`badge:3`) or a place (`place:1818`). */
type Inspected = { kind: "item" | "bundle" | "pass" | "badge" | "place"; id: number }

/** `item:5`, `bundle:5`, `pass:9`, `badge:3`, `place:1818` or a bare id, which is an item. */
function parseTarget(arg: string | null): Inspected | null {
  const m = /^(?:(item|bundle|pass|badge|place):)?(\d+)$/.exec(arg?.trim() ?? "")
  if (!m) return null
  return { kind: (m[1] as Inspected["kind"] | undefined) ?? "item", id: Number(m[2]) }
}

const pagedEmpty = <T,>(): Paged<T> => ({ list: emptyList<T>(), moreKey: "" })

/** A tile click opens the thing's page in a tab; Ctrl+click previews it in the drawer. */
const openOrInspect = (e: MouseEvent, inspect: (t: Inspected) => void, t: Inspected, open: () => void) => {
  if (wantsPreview(e)) inspect(t)
  else open()
}

/** A user's inventory by category: yours at `/inventory`, anyone's at `/inventory/<id or name>`.
 * Collectibles show serials, RAP and totals. */
export default function Inventory() {
  const params = useParams()
  const who = () => params.user?.trim() ?? ""
  const lookup = useLive(
    who,
    (w) => (w ? commands.findUser(w) : Promise.resolve({ loading: false, error: null, id: null })),
    (): UserLookup => ({ loading: true, error: null, id: null }),
  )
  const userId = () => (who() ? lookup.id : shell.active)
  const mine = () => shell.accounts.some((a) => a.id === userId())
  const prompt = usePrompt()
  useValuesRefresh()

  return (
    <Page
      title={who() && !mine() ? `Inventory of ${decodeURIComponent(who())}` : "Inventory"}
      actions={
        <>
          <Show when={userId()}>{(id) => <Totals userId={id()} />}</Show>
          <Button
            icon="search"
            onClick={async () => {
              const user = await prompt({ title: "Open an inventory", label: "Username or ID", submit: "Open" })
              if (user) void run("inventory.view", user)
            }}
          >
            Someone else's
          </Button>
        </>
      }
    >
      <Show
        when={userId()}
        fallback={
          <Show when={lookup.error ?? (!who() && "Add an account to see your inventory.")} fallback={<span />}>
            {(e) => <EmptyState icon="inventory" title={e()} />}
          </Show>
        }
      >
        {(id) => <Browser userId={id()} mine={mine()} />}
      </Show>
    </Page>
  )
}

function Totals(props: { userId: number }) {
  const t = useLive(
    () => props.userId,
    commands.inventoryTotals,
    (): InvTotalsView => ({ loading: true, items: 0, rap: 0, value: null, onHold: 0 }),
  )
  const exact = () =>
    `${t.items} collectibles · RAP ${(t.rap ?? 0).toLocaleString()}` +
    (t.value ? ` · Value ${t.value.toLocaleString()} (${VALUES_SOURCE})` : "")
  return (
    <Show when={t.items > 0}>
      <Tooltip content={exact()} class="flex items-center gap-3 text-sm">
        <span class="flex items-center gap-1">
          <span class="text-fg-muted">RAP</span>
          <Robux amount={t.rap} compact />
        </span>
        <Show when={t.value}>
          {(v) => (
            <span class="flex items-center gap-1">
              <span class="text-fg-muted">Value</span>
              <Robux amount={v()} compact />
            </span>
          )}
        </Show>
      </Tooltip>
    </Show>
  )
}

const noAvatar = (loading: boolean): AvatarView => ({
  loading,
  error: null,
  rig: "",
  wearing: [],
  emotes: [],
  colors: [],
  scales: [],
  rev: null,
})

/** The active account's avatar and wardrobe, to wear and take off items from its own
 * inventory. Other users' inventories can't be worn from. */
function useWardrobe(own: () => boolean) {
  const avatar = useLive(
    () => (own() ? shell.active : null),
    (me) => (me !== null ? commands.avatar() : Promise.resolve(noAvatar(false))),
    () => noAvatar(true),
  )
  const sections = useLive(
    () => (own() ? shell.active : null),
    (me) => (me !== null ? commands.wardrobe() : Promise.resolve({ ...emptyList<WardrobeSection>(), loading: false })),
    emptyList<WardrobeSection>,
  )
  const worn = createMemo(() => new Set(avatar.wearing.map((w) => w.id)))
  const slots = createMemo(() => new Set(sections.items.flatMap((s) => s.slots.map((c) => c.value))))
  return {
    worn,
    /** Asset type to wear `i` as, or 0 when it can't be worn from here. */
    wearType(i: ItemCard, slot: string) {
      if (!own() || i.bundle) return 0
      const t = slotAssetType(slot) || i.assetType
      return t && slots().has(`a${t}`) ? t : 0
    },
  }
}

function Browser(props: { userId: number; mine: boolean }) {
  const categories = useLive(() => props.userId, commands.inventoryCategories, emptyList<InvCategoryView>)
  const [slot, setSlot] = createSignal(COLLECTIBLES)
  const [query, setQuery] = createSignal("")
  const [inspected, setInspected] = createSignal<Inspected | null>(null)
  createEffect(() => {
    props.userId
    setSlot(COLLECTIBLES)
    setInspected(null)
  })
  const own = () => props.userId === shell.active
  const wardrobe = useWardrobe(own)
  const [loaded, setLoaded] = createStore<Loaded>({ items: [], passes: [], badges: [], places: [] })

  onCleanup(
    handle("inventory.inspect", (a) => {
      const t = parseTarget(a)
      if (t) setInspected(t)
      else showToast("Inspect takes an item ID, item:<id>, bundle:<id>, pass:<id>, badge:<id> or place:<id>")
    }),
  )

  return (
    <div class="flex items-start gap-6">
      <nav aria-label="Categories" class="sticky top-4 flex w-44 shrink-0 flex-col gap-0.5">
        <For each={categories.items}>
          {(c) => (
            <Show
              when={c.slots.length > 1}
              fallback={
                <Show when={c.slots[0]}>
                  {(s) => <NavItem label={c.name} active={slot() === s().value} onClick={() => setSlot(s().value)} />}
                </Show>
              }
            >
              <span class="px-2.5 pt-3 text-sm font-semibold text-fg">{c.name}</span>
              <For each={c.slots}>
                {(s) => <NavItem label={s.label} active={slot() === s.value} onClick={() => setSlot(s.value)} />}
              </For>
            </Show>
          )}
        </For>
      </nav>
      <div class="flex min-w-0 flex-1 flex-col gap-3">
        <Switch
          fallback={
            <Items
              userId={props.userId}
              mine={props.mine}
              slot={slot()}
              query={query()}
              setQuery={setQuery}
              wardrobe={wardrobe}
              inspect={setInspected}
              onLoad={(l) => setLoaded("items", reconcile(l))}
            />
          }
        >
          <Match when={slot() === BADGES}>
            <Search query={query()} setQuery={setQuery} />
            <Badges
              userId={props.userId}
              query={query()}
              inspect={setInspected}
              onLoad={(l) => setLoaded("badges", reconcile(l))}
            />
          </Match>
          <Match when={slot() === PASSES}>
            <Search query={query()} setQuery={setQuery} />
            <Passes
              userId={props.userId}
              query={query()}
              inspect={setInspected}
              onLoad={(l) => setLoaded("passes", reconcile(l))}
            />
          </Match>
          <Match when={slot() === PLACES}>
            <Search query={query()} setQuery={setQuery} />
            <Places
              userId={props.userId}
              query={query()}
              inspect={setInspected}
              onLoad={(l) => setLoaded("places", reconcile(l))}
            />
          </Match>
        </Switch>
      </div>
      <Show when={inspected()}>
        {(t) => (
          <Drawer
            target={t()}
            userId={props.userId}
            mine={props.mine}
            slot={slot()}
            wardrobe={wardrobe}
            loaded={loaded}
            onClose={() => setInspected(null)}
          />
        )}
      </Show>
    </div>
  )
}

function Search(props: { query: string; setQuery: (q: string) => void; placeholder?: string; children?: JSX.Element }) {
  return (
    <div class="flex gap-2">
      <TextField
        class="flex-1"
        icon="search"
        aria-label="Filter inventory"
        placeholder={props.placeholder ?? "Search by name or ID"}
        value={props.query}
        onChange={props.setQuery}
      />
      {props.children}
    </div>
  )
}

type Wardrobe = ReturnType<typeof useWardrobe>

/** What the open list has loaded, so the drawer can show serial, RAP and copies, and a pass's,
 * badge's or place's details, without fetching again. */
type Loaded = { items: ItemCard[]; passes: PassCard[]; badges: UserBadge[]; places: PlaceCard[] }

function Items(props: {
  userId: number
  mine: boolean
  slot: string
  query: string
  setQuery: (q: string) => void
  wardrobe: Wardrobe
  inspect: (t: Inspected) => void
  onLoad: (items: ItemCard[]) => void
}) {
  const [sort, setSort] = createSignal("newest")
  const [filter, setFilter] = createSignal<InvFilter>("all")
  const res = useLive(
    () => [props.userId, props.slot, sort() === "oldest"] as const,
    ([u, s, o]) => commands.inventoryItems(u, s, o),
    pagedEmpty<ItemCard>,
  )
  createEffect(() => props.onLoad(res.list.items))
  onCleanup(() => props.onLoad([]))
  const collectibles = () => props.slot === COLLECTIBLES
  const filters = () => FILTERS.filter((f) => f.value !== "wearing" || props.userId === shell.active)
  const shown = createMemo(() =>
    shapeInventory(res.list.items, {
      query: props.query,
      filter: filter(),
      sort: sort(),
      worn: props.wardrobe.worn(),
      collectibles: collectibles(),
    }),
  )
  onCleanup(
    handle("inventory.sort", (a) => {
      if (a && sorts().some((o) => o.value === a)) setSort(a)
      else
        showToast(
          `Sort by one of: ${sorts()
            .map((o) => o.value)
            .join(", ")}`,
        )
    }),
  )
  onCleanup(
    handle("inventory.filter", (a) => {
      if (a && isFilter(a) && filters().some((f) => f.value === a)) setFilter(a)
      else
        showToast(
          `Filter by one of: ${filters()
            .map((f) => f.value)
            .join(", ")}`,
        )
    }),
  )
  const sorts = () => [
    { value: "newest", label: "Recently added" },
    { value: "oldest", label: "Oldest first" },
    { value: "price_asc", label: "Price, low to high (loaded)" },
    { value: "price_desc", label: "Price, high to low (loaded)" },
    ...(collectibles()
      ? [
          { value: "rap", label: "Highest RAP (loaded)" },
          ...(shell.itemValues
            ? [
                { value: "value", label: "Highest value (loaded)" },
                { value: "demand", label: "Best demand (loaded)" },
              ]
            : []),
        ]
      : []),
  ]
  const menu = (i: ItemCard): ItemMenuExtras => {
    const wear = props.wardrobe.wearType(i, props.slot)
    const worn = props.wardrobe.worn().has(i.id)
    return {
      actions: [
        menuItem("inventory.inspect", { arg: itemTarget(i.id, i.bundle), icon: "info", label: "Inspect" }),
        ...(wear
          ? [
              menuItem("avatar.wear", {
                arg: `${i.id}:${wear}`,
                icon: worn ? "x" : "avatar",
                label: worn ? "Take off" : "Wear",
              }),
            ]
          : []),
      ],
      danger:
        props.mine && !i.bundle && !i.limited
          ? [
              menuItem("inventory.delete", {
                arg: itemTarget(i.id),
                asAccount: props.userId,
                icon: "trash",
                label: "Delete from inventory",
                danger: true,
              }),
            ]
          : [],
    }
  }

  return (
    <>
      <Search query={props.query} setQuery={props.setQuery} placeholder="Search by name, creator or ID">
        <Select
          aria-label="Show"
          value={filters().some((f) => f.value === filter()) ? filter() : "all"}
          onChange={(v) => isFilter(v) && setFilter(v)}
          options={filters()}
        />
        <Select
          aria-label="Sort"
          value={sorts().some((s) => s.value === sort()) ? sort() : "newest"}
          onChange={setSort}
          options={sorts()}
        />
      </Search>
      <ListState
        list={res.list}
        icon="inventory"
        empty="Nothing here"
        description="No items in this category."
        more={res.moreKey}
      >
        <VirtualGrid
          items={shown()}
          minItemWidth={TILE_WIDTH}
          estimateRowHeight={170}
          gap={12}
          empty={<EmptyState icon="search" title="No items match" />}
        >
          {(i) => (
            <ItemTile
              item={i}
              showSerial
              showRap={collectibles()}
              selected={props.wardrobe.worn().has(i.id)}
              onClick={(e) =>
                openOrInspect(e, props.inspect, { kind: i.bundle ? "bundle" : "item", id: i.id }, () =>
                  run("market.open", itemTarget(i.id, i.bundle)),
                )
              }
              menu={menu(i)}
            />
          )}
        </VirtualGrid>
      </ListState>
    </>
  )
}

const byName = (name: string, id: number, query: string) => matches({ id, name, creator: "" }, query)

function Badges(props: {
  userId: number
  query: string
  inspect: (t: Inspected) => void
  onLoad: (badges: UserBadge[]) => void
}) {
  const res = useLive(() => props.userId, commands.inventoryBadges, pagedEmpty<UserBadge>)
  createEffect(() => props.onLoad(res.list.items))
  onCleanup(() => props.onLoad([]))
  const shown = createMemo(() => res.list.items.filter((b) => byName(b.name, b.id, props.query)))
  return (
    <ListState list={res.list} icon="badge" empty="No badges" more={res.moreKey} autoMore>
      <VirtualGrid
        items={shown()}
        minItemWidth={TILE_WIDTH}
        estimateRowHeight={190}
        gap={12}
        empty={<EmptyState icon="search" title="No badges match" />}
      >
        {(b) => (
          <ContextMenu
            items={[
              menuItem("inventory.inspect", { arg: `badge:${b.id}`, icon: "info", label: "Inspect" }),
              ...(b.placeId ? [{ kind: "separator" } as const, ...gameMenu(b.placeId)] : []),
            ]}
          >
            <Tile
              title={b.name}
              subtitle={b.winRate === null ? undefined : `${percent(b.winRate)} have it`}
              src={thumbUrl("badge_icon", b.id)}
              fallback="badge"
              fixed
              onClick={(e) =>
                openOrInspect(e, props.inspect, { kind: "badge", id: b.id }, () =>
                  b.placeId ? run("game.open", String(b.placeId)) : props.inspect({ kind: "badge", id: b.id }),
                )
              }
            />
          </ContextMenu>
        )}
      </VirtualGrid>
    </ListState>
  )
}

function Passes(props: {
  userId: number
  query: string
  inspect: (t: Inspected) => void
  onLoad: (passes: PassCard[]) => void
}) {
  const res = useLive(() => props.userId, commands.inventoryPasses, pagedEmpty<PassCard>)
  createEffect(() => props.onLoad(res.list.items))
  onCleanup(() => props.onLoad([]))
  const shown = createMemo(() => res.list.items.filter((p) => matches(p, props.query)))
  return (
    <ListState list={res.list} icon="game" empty="No passes" more={res.moreKey}>
      <VirtualGrid
        items={shown()}
        minItemWidth={TILE_WIDTH}
        estimateRowHeight={190}
        gap={12}
        empty={<EmptyState icon="search" title="No passes match" />}
      >
        {(p) => (
          <ContextMenu
            items={[
              menuItem("market.open", { arg: `pass:${p.id}`, icon: "pass", label: "Open pass" }),
              menuItem("inventory.inspect", { arg: `pass:${p.id}`, icon: "info", label: "Inspect" }),
              { kind: "separator" },
              shareMenu(`pass:${p.id}`),
            ]}
          >
            <Tile
              title={p.name}
              subtitle={<Price amount={p.price} />}
              src={thumbUrl("game_pass", p.id)}
              fallback="pass"
              fixed
              onClick={(e) =>
                openOrInspect(e, props.inspect, { kind: "pass", id: p.id }, () => run("market.open", `pass:${p.id}`))
              }
            />
          </ContextMenu>
        )}
      </VirtualGrid>
    </ListState>
  )
}

function Places(props: {
  userId: number
  query: string
  inspect: (t: Inspected) => void
  onLoad: (places: PlaceCard[]) => void
}) {
  const res = useLive(() => props.userId, commands.inventoryPlaces, pagedEmpty<PlaceCard>)
  createEffect(() => props.onLoad(res.list.items))
  onCleanup(() => props.onLoad([]))
  const shown = createMemo(() => res.list.items.filter((p) => byName(p.name, p.placeId, props.query)))
  return (
    <ListState
      list={res.list}
      icon="game"
      empty="No places"
      description="Only places this user made are listed."
      more={res.moreKey}
    >
      <VirtualGrid
        items={shown()}
        minItemWidth={TILE_WIDTH}
        estimateRowHeight={190}
        gap={12}
        empty={<EmptyState icon="search" title="No places match" />}
      >
        {(p) => (
          <ContextMenu
            items={[
              menuItem("inventory.inspect", { arg: `place:${p.placeId}`, icon: "info", label: "Inspect" }),
              { kind: "separator" },
              ...gameMenu(p.placeId),
            ]}
          >
            <Tile
              title={p.name}
              subtitle={
                <Show when={p.price} fallback={p.creator}>
                  {(price) => <Robux amount={price()} />}
                </Show>
              }
              src={p.universeId ? thumbUrl("game_icon", p.universeId) : undefined}
              fallback="game"
              fixed
              onClick={(e) =>
                openOrInspect(e, props.inspect, { kind: "place", id: p.placeId }, () =>
                  run("game.open", String(p.placeId)),
                )
              }
            />
          </ContextMenu>
        )}
      </VirtualGrid>
    </ListState>
  )
}

/** Details of one item or pass beside the grid, without leaving the inventory. */
function Drawer(props: {
  target: Inspected
  userId: number
  mine: boolean
  slot: string
  wardrobe: Wardrobe
  loaded: Loaded
  onClose: () => void
}) {
  const target = () => `${props.target.kind}:${props.target.id}`
  return (
    <aside aria-label="Details" class="sticky top-4 flex w-72 shrink-0 flex-col gap-3">
      <Switch
        fallback={
          <ItemDetails
            id={props.target.id}
            bundle={props.target.kind === "bundle"}
            userId={props.userId}
            mine={props.mine}
            slot={props.slot}
            wardrobe={props.wardrobe}
            items={props.loaded.items}
            onClose={props.onClose}
          />
        }
      >
        <Match when={props.target.kind === "pass" && props.loaded.passes.find((p) => p.id === props.target.id)}>
          {(p) => (
            <>
              <PreviewHeader title={p().name} onClose={props.onClose} />
              <Thumbnail class="w-full" alt={p().name} src={thumbUrl("game_pass", p().id)} fallback="pass" />
              <Facts>
                <FactGroup>
                  <Fact label="Creator">
                    <TextLink
                      tone="name"
                      onClick={() => run(p().creatorIsGroup ? "group.open" : "user.open", String(p().creatorId))}
                    >
                      {p().creator}
                    </TextLink>
                  </Fact>
                  <Fact label="Type">Game pass</Fact>
                  <IdFact id={p().id} target={target()} />
                  <Fact label="Price">
                    <Price amount={p().price} />
                  </Fact>
                </FactGroup>
              </Facts>
              <PreviewDescription text={p().description} />
              <div class="flex flex-wrap gap-2">
                <Button variant="primary" icon="pass" onClick={() => run("market.open", target())}>
                  Open page
                </Button>
                <Button icon="external" onClick={() => run("entity.open_website", target())}>
                  Open on website
                </Button>
              </div>
            </>
          )}
        </Match>
        <Match when={props.target.kind === "badge" && props.loaded.badges.find((b) => b.id === props.target.id)}>
          {(b) => (
            <>
              <PreviewHeader title={b().name} onClose={props.onClose} />
              <Thumbnail class="w-full" alt={b().name} src={thumbUrl("badge_icon", b().id)} fallback="badge" />
              <Facts>
                <FactGroup>
                  <Fact label="Type">Badge</Fact>
                  <IdFact id={b().id} target={target()} />
                  <Show when={b().winRate !== null}>
                    <Fact label="Won by">{percent(b().winRate ?? 0)} of players</Fact>
                  </Show>
                </FactGroup>
              </Facts>
              <PreviewDescription text={b().description} />
              <Show when={b().placeId}>
                {(place) => (
                  <div class="flex flex-wrap gap-2">
                    <Button variant="primary" icon="game" onClick={() => run("game.open", String(place()))}>
                      Open experience
                    </Button>
                  </div>
                )}
              </Show>
            </>
          )}
        </Match>
        <Match when={props.target.kind === "place" && props.loaded.places.find((p) => p.placeId === props.target.id)}>
          {(p) => (
            <>
              <PreviewHeader title={p().name} onClose={props.onClose} />
              <Thumbnail
                class="w-full"
                alt={p().name}
                src={p().universeId ? thumbUrl("game_icon", p().universeId) : undefined}
                fallback="game"
              />
              <Facts>
                <FactGroup>
                  <Show when={p().creator}>
                    <Fact label="Creator">{p().creator}</Fact>
                  </Show>
                  <Fact label="Type">Place</Fact>
                  <IdFact id={p().placeId} target={`game:${p().placeId}`} />
                  <Show when={p().universeId}>{(u) => <Fact label="Universe ID">{u()}</Fact>}</Show>
                  <Fact label="Access">
                    <Show when={p().price} fallback="Free">
                      {(price) => <Robux amount={price()} />}
                    </Show>
                  </Fact>
                </FactGroup>
              </Facts>
              <div class="flex flex-wrap gap-2">
                <Button variant="primary" icon="game" onClick={() => run("game.open", String(p().placeId))}>
                  Open experience
                </Button>
              </div>
            </>
          )}
        </Match>
        <Match when={props.target.kind !== "item" && props.target.kind !== "bundle"}>
          <PreviewHeader title="Details" onClose={props.onClose} />
          <span class="text-sm text-fg-muted">Not in the loaded list.</span>
        </Match>
      </Switch>
    </aside>
  )
}

function ItemDetails(props: {
  id: number
  bundle: boolean
  userId: number
  mine: boolean
  slot: string
  wardrobe: Wardrobe
  items: readonly ItemCard[]
  onClose: () => void
}) {
  const card = () => props.items.find((i) => i.id === props.id && i.bundle === props.bundle)
  const target = () => itemTarget(props.id, props.bundle)
  const wear = () => {
    const c = card()
    return c ? props.wardrobe.wearType(c, props.slot) : 0
  }
  const worn = () => props.wardrobe.worn().has(props.id)
  return (
    <ItemPreview
      id={props.id}
      bundle={props.bundle}
      assetType={card()?.assetType}
      name={card()?.name}
      onClose={props.onClose}
      facts={
        <>
          <Show when={card()?.rap}>
            {(r) => (
              <Fact label="RAP">
                <Robux amount={r()} />
              </Fact>
            )}
          </Show>
          <Show when={card()?.serial}>{(s) => <Fact label="Serial">#{s()}</Fact>}</Show>
          <Show when={(card()?.copies ?? 1) > 1}>
            <Fact label="Copies">{card()?.copies}</Fact>
          </Show>
        </>
      }
      actions={
        <Show when={wear()}>
          {(t) => (
            <Button
              variant="primary"
              icon={worn() ? "x" : "avatar"}
              onClick={() => run("avatar.wear", `${props.id}:${t()}`)}
            >
              {worn() ? "Take off" : "Wear"}
            </Button>
          )}
        </Show>
      }
      trailing={
        <Show when={props.mine && !props.bundle && !card()?.limited}>
          <Button variant="danger" icon="trash" onClick={() => run("inventory.delete", target(), props.userId)}>
            Delete
          </Button>
        </Show>
      }
    />
  )
}

import { useLocation, useParams } from "@solidjs/router"
import { createEffect, createMemo, createSignal, For, Match, on, onCleanup, Show, Switch } from "solid-js"
import { assetIcon } from "~/lib/asset-icon"
import { canTryOn, showcaseAnimation, showsAlone, type TryOnItem, wearable } from "~/lib/avatar3d/outfit"
import { AvatarViewer, show3d } from "~/lib/avatar3d/viewer"
import { BuyForm } from "~/lib/buy-form"
import { useCommandContext, useSubTabs } from "~/lib/commands/hooks"
import { handle, menuItem, run } from "~/lib/commands/registry"
import { ago, compact, date } from "~/lib/format"
import {
  commands,
  type ItemCard,
  type ItemView,
  type OwnerRow,
  type Paged,
  type ResellerRow,
  type ValueView,
} from "~/lib/ipc/bindings"
import { Description } from "~/lib/linked-text"
import { emptyList, ListState } from "~/lib/list-state"
import { accountItems, entityMenu, itemTarget, templateItems } from "~/lib/menus"
import { shell, useLive } from "~/lib/store"
import { setTabTitle } from "~/lib/tabs"
import { assetFileUrl, thumbUrl } from "~/lib/thumb"
import { ItemTile, tileGrid } from "~/lib/tiles"
import {
  type AudioControl,
  AudioPlayer,
  Badge,
  Button,
  Card,
  type Column,
  DataTable,
  Icon,
  Menu,
  Page,
  Robux,
  Skeleton,
  Stat,
  showToast,
  TabPanel,
  Tabs,
  TextLink,
  Thumbnail,
  ToggleButton,
  Tooltip,
  useDialog,
  VerifiedBadge,
} from "~/lib/ui"
import { DemandPips, TrendMark, useValuesRefresh, ValueAmount, ValueFlags } from "~/lib/value-tags"
import { rapGap, VALUES_SOURCE, versionLabel } from "~/lib/values"
import { PriceHistory } from "./PriceHistory"

const ms = (iso: string) => {
  const t = Date.parse(iso)
  return Number.isNaN(t) ? null : t
}

const emptyPaged = <T,>(): Paged<T> => ({ list: emptyList<T>(), moreKey: "" })

/** One catalog item: picture, price, buy, favorite, wishlist, details and, for limiteds,
 * resellers (each copy buyable), price history and owners. Assets the catalog doesn't list
 * (models, decals, audio, plugins, meshes, videos...) show their economy details instead. */
export default function ItemPage() {
  const params = useParams()
  const location = useLocation()
  const id = () => Number(params.id)
  const bundle = () => location.pathname.startsWith("/bundles/")
  const target = () => itemTarget(id(), bundle())
  useValuesRefresh()
  const view = useLive(
    () => [id(), bundle()] as const,
    ([i, b]) => commands.itemPage(i, b),
    (): ItemView => ({
      loading: true,
      error: null,
      info: null,
      favorited: null,
      wishlisted: false,
      ownedBy: [],
      owned: false,
      value: null,
    }),
  )
  useCommandContext(target)
  createEffect(() => {
    const name = view.info?.name
    if (name) setTabTitle(location.pathname, name)
  })

  const info = () => view.info
  const inCatalog = () => info()?.inCatalog ?? true
  const thumb = () => thumbUrl(bundle() ? "bundle" : "asset", id())
  const dialog = useDialog()
  const buy = (account?: number) => {
    const i = info()
    if (!i) return showToast("Still loading. Try again in a moment.")
    if (!i.inCatalog) return showToast("This isn't sold in the Marketplace. Open it on the website.")
    if (i.price === null) return showToast("It's not for sale.")
    if (account === undefined && view.owned) return showToast("You already own this.")
    const price = i.price
    const [itemId, isBundle] = [id(), bundle()]
    dialog.show(
      () => (
        <BuyForm
          name={i.name}
          seller={i.fromReseller ? "the cheapest reseller" : i.creator}
          price={price}
          thumb={thumb()}
          buy={() => commands.buyItem(itemId, isBundle, price, account ?? null)}
          onDone={dialog.close}
          account={account}
        />
      ),
      { title: "Buy item", size: "sm" },
    )
  }
  onCleanup(handle("market.buy", () => buy()))
  onCleanup(
    handle("market.buy_as", (arg) => {
      const account = Number(arg)
      if (!arg || !shell.accounts.some((a) => a.id === account)) return showToast("Pick an account to buy as.")
      buy(account)
    }),
  )
  // The 3D view shows the item by itself (accessories alone, the rest on a mannequin), or with
  // Try On, your avatar wearing it or doing the emote. Bundles use their parts.
  const [trying, setTrying] = createSignal(false)
  createEffect(on(id, () => setTrying(false), { defer: true }))
  const parts = (): TryOnItem[] => {
    const i = info()
    if (!i) return []
    if (bundle()) return i.contents.filter((c) => c.typeId > 0)
    return [{ id: i.id, name: i.name, typeId: i.assetType }]
  }
  const previewable = () => parts().some((p) => canTryOn(p.typeId))
  const alone = () => !bundle() && !trying() && showsAlone(info()?.assetType ?? 0)
  onCleanup(
    handle("market.try_on", () => {
      if (!previewable()) return showToast("This can't be shown in 3D.")
      if (shell.active === null && !trying()) return showToast("Sign in to try items on.")
      show3d()
      setTrying((t) => !t)
    }),
  )
  // Audio plays from the asset file, fetched by core as the active account.
  const AUDIO = 3
  const sound = () => (!bundle() && info()?.assetType === AUDIO ? assetFileUrl(shell.active ?? 0, id()) : undefined)
  let player: AudioControl | undefined
  onCleanup(
    handle("market.play_audio", () => {
      if (!sound() || !player) return showToast("This isn't a sound.")
      player.toggle()
    }),
  )
  // Non-limited UGC can also carry a collectibleId; only limiteds and collectibles have copies.
  const resells = () => Boolean(info()?.collectibleId && info()?.limited)
  const [section, setSection] = createSignal("similar")
  const sections = createMemo(() => [
    { value: "similar", label: "Similar items", disabled: !inCatalog() },
    { value: "resellers", label: "Resellers", disabled: !resells() },
    { value: "history", label: "Price history", disabled: !resells() },
    { value: "owners", label: "Owners", disabled: !resells() || bundle() },
  ])
  useSubTabs(sections, section, setSection)
  createEffect(() => {
    if (resells() && section() === "similar" && info()?.hasResellers) setSection("resellers")
  })

  return (
    <Page class="!gap-6">
      <Show
        when={info()}
        fallback={
          <Show when={view.error} fallback={<Skeleton class="h-72 w-full rounded-md" />}>
            {(e) => <span class="text-fg-muted">{e()}</span>}
          </Show>
        }
      >
        {(item) => (
          <div class="flex flex-wrap gap-6">
            <Show
              when={previewable()}
              fallback={
                <Thumbnail
                  class="w-72 self-start"
                  alt={item().name}
                  src={thumbUrl(bundle() ? "bundle" : "asset", id(), true)}
                  fallback={bundle() ? "avatar" : assetIcon(item().assetType)}
                />
              }
            >
              <AvatarViewer
                class="w-72 self-start"
                source={trying() && shell.active !== null ? { kind: "me" } : undefined}
                image={
                  trying()
                    ? thumbUrl("avatar", shell.active ?? 0, true)
                    : thumbUrl(bundle() ? "bundle" : "asset", id(), true)
                }
                alt={trying() ? `You wearing ${item().name}` : item().name}
                accessory={alone() ? item().id : undefined}
                wear={wearable(parts())}
                animation={showcaseAnimation(parts())}
                controls
                menu={[
                  menuItem("market.try_on", {
                    label: trying() ? "View alone" : "Try on",
                    icon: trying() ? "eye" : "avatar",
                  }),
                ]}
              />
            </Show>
            <div class="flex min-w-64 flex-1 flex-col gap-3">
              <div class="flex flex-col gap-1">
                <div class="flex flex-wrap items-center gap-2">
                  <h1 class="truncate text-2xl font-medium">{item().name}</h1>
                  <Show when={item().limited}>{(l) => <Badge variant="success">{l()}</Badge>}</Show>
                  <Show when={item().premiumOnly}>
                    <Badge>Premium</Badge>
                  </Show>
                  <Show when={item().publicDomain}>
                    <Badge>Free to take</Badge>
                  </Show>
                </div>
                <span class="flex items-center gap-1 text-fg-muted">
                  {item().typeName} by{" "}
                  <TextLink
                    tone={item().creatorFake ? "danger" : item().creatorVerified ? "verified" : "name"}
                    onClick={() => run(item().creatorIsGroup ? "group.open" : "user.open", String(item().creatorId))}
                  >
                    {item().creator}
                  </TextLink>
                  <Show when={item().creatorVerified}>
                    <VerifiedBadge />
                  </Show>
                  <Show when={item().creatorFake}>
                    <Tooltip content="Not Roblox. This creator copies Roblox's name.">
                      <Icon name="alert" size={14} class="text-danger" aria-label="Not Roblox" />
                    </Tooltip>
                  </Show>
                </span>
              </div>
              <Show
                when={item().price !== null}
                fallback={<span class="text-xl font-medium">{item().priceLabel}</span>}
              >
                <Show when={item().price !== 0} fallback={<span class="text-xl font-medium">Free</span>}>
                  <Robux class="text-xl font-medium" amount={item().price} />
                </Show>
              </Show>
              <div class="flex flex-wrap gap-1.5">
                <Show
                  when={item().inCatalog}
                  fallback={
                    <Button icon="external" onClick={() => run("entity.open_website", target())}>
                      Open on website
                    </Button>
                  }
                >
                  <Button
                    variant="primary"
                    disabled={item().price === null || view.owned}
                    onClick={() => run("market.buy", target())}
                  >
                    <Switch>
                      <Match when={view.owned}>Owned</Match>
                      <Match when={item().price === 0}>Get</Match>
                      <Match when>Buy</Match>
                    </Switch>
                  </Button>
                  <Menu
                    items={accountItems("market.buy_as")}
                    disabled={item().price === null || accountItems("market.buy_as").length === 0}
                  >
                    Buy as…
                  </Menu>
                </Show>
                <ToggleButton
                  icon="star"
                  pressedIcon="star-fill"
                  pressed={view.favorited ?? false}
                  disabled={view.favorited === null}
                  onChange={() => run("market.favorite", target())}
                >
                  Favorite
                </ToggleButton>
                <Show when={previewable()}>
                  <ToggleButton icon="avatar" pressed={trying()} onChange={() => run("market.try_on")}>
                    Try on
                  </ToggleButton>
                </Show>
                <Show when={item().inCatalog}>
                  <ToggleButton
                    icon="heart"
                    pressedIcon="heart-fill"
                    pressed={view.wishlisted}
                    onChange={() => run("market.wishlist", target())}
                  >
                    Wishlist
                  </ToggleButton>
                </Show>
                <Menu
                  icon="more"
                  label="More"
                  items={entityMenu(target(), item().bundle ? [] : templateItems(item().id, item().assetType))}
                />
              </div>
              <Show when={sound()} keyed>
                {(src) => (
                  <AudioPlayer
                    class="max-w-md"
                    src={src}
                    label={item().name}
                    ref={(c) => {
                      player = c
                    }}
                  />
                )}
              </Show>
              <Show when={view.ownedBy.length > 0}>
                <span class="flex items-center gap-1.5 text-sm text-fg-muted">
                  <Icon name="check" size={14} /> Owned by {view.ownedBy.join(", ")}
                </span>
              </Show>
              <Show when={item().description}>
                {(d) => <Description text={d()} class="max-w-prose text-fg-muted" />}
              </Show>
              <Card class="grid grid-cols-[repeat(auto-fill,minmax(8rem,1fr))] gap-4 p-3">
                <Stat label={item().bundle ? "Bundle ID" : "Asset ID"}>{item().id}</Stat>
                <Stat label="Type">{item().typeName}</Stat>
                <Show when={item().category}>{(c) => <Stat label="Category">{c()}</Stat>}</Show>
                <Show when={item().genres.length > 0}>
                  <Stat label="Genres">{item().genres.join(", ")}</Stat>
                </Show>
                <Show when={item().inCatalog}>
                  <Stat label="Favorites">{compact(item().favorites)}</Stat>
                </Show>
                <Show when={item().sales > 0}>
                  <Stat label="Sales">{compact(item().sales)}</Stat>
                </Show>
                <Show when={ms(item().created)}>{(t) => <Stat label="Created">{date(t())}</Stat>}</Show>
                <Show when={ms(item().updated)}>{(t) => <Stat label="Updated">{date(t())}</Stat>}</Show>
                <Show when={item().totalQuantity > 0}>
                  <Stat label="Stock">
                    {compact(item().unitsLeft)} / {compact(item().totalQuantity)}
                  </Stat>
                </Show>
                <Show when={ms(item().offSaleDeadline)}>{(t) => <Stat label="Off sale on">{date(t())}</Stat>}</Show>
              </Card>
              <Show when={view.value}>{(v) => <ValuePanel value={v()} />}</Show>
              <Show when={item().tags.length > 0}>
                <div class="flex flex-wrap items-center gap-1.5">
                  <span class="text-xs text-fg-muted">Tags</span>
                  <For each={item().tags}>
                    {(tag) => (
                      <TextLink onClick={() => run("market.search", tag)}>
                        <Badge>{tag}</Badge>
                      </TextLink>
                    )}
                  </For>
                </div>
              </Show>
              <Show when={item().contents.length > 0}>
                <div class="flex flex-col gap-1">
                  <span class="text-sm text-fg-muted">Includes</span>
                  <div class="flex flex-wrap gap-x-3 gap-y-1">
                    <For each={item().contents}>
                      {(p) => <TextLink onClick={() => run("market.open", itemTarget(p.id))}>{p.name}</TextLink>}
                    </For>
                  </div>
                </div>
              </Show>
            </div>
          </div>
        )}
      </Show>

      <Show when={inCatalog()}>
        <Tabs aria-label="Item sections" items={sections()} value={section()} onChange={setSection}>
          <TabPanel value="similar">
            <Show when={bundle() || info()?.assetType}>
              <Similar id={id()} bundle={bundle()} assetType={info()?.assetType ?? 0} />
            </Show>
          </TabPanel>
          <TabPanel value="resellers">
            <Show when={info()}>
              {(i) => (
                <Show when={i().collectibleId}>
                  {(c) => <Resellers collectibleId={c()} id={id()} bundle={bundle()} name={i().name} thumb={thumb()} />}
                </Show>
              )}
            </Show>
          </TabPanel>
          <TabPanel value="history">
            <Show when={info()}>
              {(i) => (
                <Show when={i().collectibleId}>
                  {(c) => (
                    <PriceHistory
                      collectibleId={c()}
                      value={view.value?.tag.value ?? null}
                      best={i().fromReseller ? i().price : null}
                    />
                  )}
                </Show>
              )}
            </Show>
          </TabPanel>
          <TabPanel value="owners">
            <Owners assetId={id()} />
          </TabPanel>
        </Tabs>
      </Show>
    </Page>
  )
}

/** What Rolimons says about this limited: value beside RAP and how far apart they are, demand,
 * trend and flags, with the source and age always shown. */
function ValuePanel(props: { value: ValueView }) {
  const v = () => props.value
  const gap = () => rapGap(v().tag.value, v().rap)
  return (
    <Card class="flex flex-col gap-3 p-3">
      <div class="flex flex-wrap items-center gap-1.5 text-sm text-fg-muted">
        <Icon name="rolimons" size={14} />
        <span>{VALUES_SOURCE}</span>
        <Show when={v().acronym}>
          <Badge>{v().acronym}</Badge>
        </Show>
        <Show when={versionLabel(v().version)}>{(l) => <Badge>{l()}</Badge>}</Show>
        <ValueFlags tag={v().tag} />
        <Show when={v().fetchedAt}>{(t) => <span class="ml-auto text-xs text-fg-faint">Updated {ago(t())}</span>}</Show>
      </div>
      <dl class="grid grid-cols-[repeat(auto-fill,minmax(8rem,1fr))] gap-4">
        <Stat label="Value">
          <Show when={v().tag.value} fallback={<span class="text-fg-muted">Not valued</span>}>
            {(n) => <ValueAmount class="text-lg font-medium" amount={n()} />}
          </Show>
        </Stat>
        <Stat label="RAP (Rolimons)">
          <span class="flex flex-col">
            <Robux amount={v().rap} />
            <Show when={gap()}>
              {(g) => (
                <span
                  class="text-xs"
                  classList={{
                    "text-warning": g() > 10,
                    "text-success": g() < -10,
                    "text-fg-muted": Math.abs(g()) <= 10,
                  }}
                >
                  {Math.abs(g())}% {g() > 0 ? "above" : "below"} value
                </span>
              )}
            </Show>
          </span>
        </Stat>
        <Show when={v().tag.demand}>
          {(d) => (
            <Stat label="Demand">
              <DemandPips demand={d()} label />
            </Stat>
          )}
        </Show>
        <Show when={v().tag.trend}>
          {(t) => (
            <Stat label="Trend">
              <TrendMark trend={t()} label />
            </Stat>
          )}
        </Show>
      </dl>
    </Card>
  )
}

function Similar(props: { id: number; bundle: boolean; assetType: number }) {
  const list = useLive(
    () => [props.id, props.bundle, props.assetType] as const,
    ([i, b, t]) => commands.itemRecommendations(i, b, t),
    emptyList<ItemCard>,
  )
  return (
    <ListState list={list} icon="marketplace" empty="No similar items">
      <div class={tileGrid}>
        <For each={list.items}>{(i) => <ItemTile item={i} />}</For>
      </div>
    </ListState>
  )
}

function Resellers(props: { collectibleId: string; id: number; bundle: boolean; name: string; thumb?: string }) {
  const res = useLive(() => props.collectibleId, commands.itemResellers, emptyPaged<ResellerRow>)
  const dialog = useDialog()
  onCleanup(
    handle("market.buy_resale", (arg) => {
      const r = res.list.items.find((x) => x.instanceId === arg?.trim())
      if (!r) return showToast("Pick a copy in the Resellers tab.")
      const [itemId, isBundle] = [props.id, props.bundle]
      dialog.show(
        () => (
          <BuyForm
            name={props.name}
            seller={r.seller}
            price={r.price}
            thumb={props.thumb}
            note={r.serial ? `Serial #${r.serial}` : undefined}
            buy={() => commands.buyResale(itemId, isBundle, r.instanceId, r.price)}
            onDone={dialog.close}
          />
        ),
        { title: "Buy from reseller", size: "sm" },
      )
    }),
  )
  const columns: Column<ResellerRow>[] = [
    {
      key: "seller",
      label: "Seller",
      cell: (r) => (
        <>
          <TextLink tone={r.verified ? "verified" : "name"} onClick={() => run("user.open", String(r.sellerId))}>
            {r.seller}
          </TextLink>
          <Show when={r.verified}>
            <VerifiedBadge />
          </Show>
        </>
      ),
    },
    { key: "serial", label: "Serial", width: "6rem", mono: true, cell: (r) => (r.serial ? `#${r.serial}` : "–") },
    { key: "price", label: "Price", width: "7rem", align: "end", cell: (r) => <Robux amount={r.price} /> },
    {
      key: "buy",
      label: "",
      width: "5rem",
      align: "end",
      cell: (r) => (
        <Button size="sm" onClick={() => run("market.buy_resale", r.instanceId)}>
          Buy
        </Button>
      ),
    },
  ]
  return (
    <ListState list={res.list} icon="robux" empty="Nobody is selling this" more={res.moreKey}>
      <DataTable
        aria-label="Resellers"
        columns={columns}
        rows={res.list.items}
        rowKey={(r) => r.instanceId || `${r.sellerId}:${r.serial}:${r.price}`}
      />
    </ListState>
  )
}

function Owners(props: { assetId: number }) {
  const res = useLive(() => props.assetId, commands.itemOwners, emptyPaged<OwnerRow>)
  const columns: Column<OwnerRow>[] = [
    {
      key: "owner",
      label: "Owner",
      cell: (o) => (
        <Show when={o.userId} fallback={<span class="text-fg-muted">Hidden</span>}>
          {(u) => (
            <TextLink tone="name" onClick={() => run("user.open", String(u()))}>
              {o.name}
            </TextLink>
          )}
        </Show>
      ),
    },
    { key: "serial", label: "Serial", width: "6rem", mono: true, cell: (o) => (o.serial ? `#${o.serial}` : "–") },
    {
      key: "since",
      label: "Since",
      width: "8rem",
      align: "end",
      cell: (o) => {
        const t = ms(o.since)
        return t === null ? "–" : date(t)
      },
    },
  ]
  return (
    <ListState list={res.list} icon="friends" empty="No owners listed" more={res.moreKey}>
      <DataTable
        aria-label="Owners"
        columns={columns}
        rows={res.list.items}
        rowKey={(o) => `${o.userId}:${o.serial}:${o.since}`}
      />
    </ListState>
  )
}

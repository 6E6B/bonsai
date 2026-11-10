import { useLocation, useParams } from "@solidjs/router"
import { createEffect, createMemo, createSignal, For, Match, on, onCleanup, Show, Switch } from "solid-js"
import { assetIcon } from "~/lib/asset-icon"
import { canTryOn, showcaseAnimation, showsAlone, type TryOnItem, wearable } from "~/lib/avatar3d/outfit"
import { AvatarViewer, show3d } from "~/lib/avatar3d/viewer"
import { BuyForm } from "~/lib/buy-form"
import { useCommandContext, useSubTabs } from "~/lib/commands/hooks"
import { handle, isRunning, menuItem, run } from "~/lib/commands/registry"
import { ago, compact, date, full } from "~/lib/format"
import { commands, type ItemCard, type ItemView, type OwnerRow, type Paged, type ResellerRow } from "~/lib/ipc/bindings"
import { IdFact } from "~/lib/item-preview"
import { Description } from "~/lib/linked-text"
import { emptyList, ListState } from "~/lib/list-state"
import { accountItems, entityMenu, itemTarget, templateItems } from "~/lib/menus"
import { isSoundHat, runSoundHat, setSoundHatsOn, soundHat, soundHatsOn } from "~/lib/sound-hats"
import { shell, useLive } from "~/lib/store"
import { setTabTitle } from "~/lib/tabs"
import { assetFileUrl, thumbUrl } from "~/lib/thumb"
import { ItemTile, SoundHatBadge } from "~/lib/tiles"
import {
  type AudioControl,
  AudioPlayer,
  Avatar,
  Badge,
  Button,
  Card,
  Clamp,
  type Column,
  DataTable,
  Divider,
  Fact,
  FactGroup,
  Facts,
  Icon,
  Menu,
  Meter,
  Page,
  PremiumBadge,
  Robux,
  SegmentedControl,
  Stage,
  showToast,
  TabPanel,
  Tabs,
  TextLink,
  Thumbnail,
  ToggleButton,
  Tooltip,
  useDialog,
  VerifiedBadge,
  VirtualGrid,
} from "~/lib/ui"
import { useValuesRefresh } from "~/lib/value-tags"
import { useWikiAbout, WikiSummary, WikiTab } from "~/lib/wiki-panels"
import { ItemSkeleton } from "./DetailSkeleton"
import { pctOver } from "./history"
import { Dependencies } from "./ItemDependencies"
import { ItemModel } from "./ItemModel"
import { MarketCard } from "./MarketCard"
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
  // Sound hats run their in-game script while the page is open: a random sound every few seconds.
  const hat = () => soundHat(id(), bundle())
  const loaded = createMemo(() => info() !== null)
  createEffect(() => {
    const h = hat()
    if (h && loaded() && soundHatsOn()) onCleanup(runSoundHat(h))
  })
  onCleanup(handle("market.sound_hat", () => setSoundHatsOn(!soundHatsOn())))
  // Non-limited UGC can also carry a collectibleId; only limiteds and collectibles have copies.
  const resells = () => Boolean(info()?.collectibleId && info()?.limited)
  // Copies for sale, cheapest first. Read here as well as in the tab: the buy box names the
  // copy that sets the price.
  const resellers = useLive(
    () => (resells() ? (info()?.collectibleId ?? "") : ""),
    (c) => (c ? commands.itemResellers(c) : Promise.resolve({ ...emptyPaged<ResellerRow>(), list: idle() })),
    emptyPaged<ResellerRow>,
  )
  const cheapest = () => {
    const i = info()
    if (!i?.fromReseller || i.price === null) return null
    return resellers.list.items.find((r) => r.price === i.price) ?? null
  }
  const value = () => (resells() ? (view.value?.tag.value ?? null) : null)
  const resellerCount = () =>
    resellers.list.loading || resellers.moreKey ? undefined : resellers.list.items.length || undefined

  const [section, setSection] = createSignal("similar")
  const wiki = useWikiAbout(() => {
    const name = info()?.name
    return name ? { kind: bundle() ? "bundle" : "item", id: id(), name } : null
  })
  const sections = createMemo(() => [
    // Only limiteds and collectibles have copies to resell, track or own; they lead.
    ...(inCatalog() && resells()
      ? [
          { value: "resellers", label: "Resellers", count: resellerCount() },
          { value: "history", label: "Price history" },
          ...(!bundle() ? [{ value: "owners", label: "Owners" }] : []),
        ]
      : []),
    ...(inCatalog() ? [{ value: "similar", label: "Similar items" }] : []),
    ...(!bundle() ? [{ value: "dependencies", label: "Dependencies" }] : []),
    ...(!bundle() ? [{ value: "model", label: "Model" }] : []),
    ...(wiki.article ? [{ value: "wiki", label: "Wiki" }] : []),
  ])
  onCleanup(
    handle("market.dependencies", () => {
      if (bundle()) return showToast("Open an item in this bundle to view its dependencies.")
      setSection("dependencies")
    }),
  )
  onCleanup(
    handle("market.model", () => {
      if (bundle()) return showToast("Open an item in this bundle to view its model.")
      setSection("model")
    }),
  )
  createEffect(
    on(
      () => [id(), bundle(), inCatalog()] as const,
      () => {
        setSection(inCatalog() ? "similar" : "dependencies")
      },
    ),
  )
  useSubTabs(sections, section, setSection)
  createEffect(() => {
    if (resells() && section() === "similar" && info()?.hasResellers) setSection("resellers")
  })

  return (
    <Page class="!gap-6">
      <Show
        when={info()}
        fallback={
          <Show when={view.error} fallback={<ItemSkeleton />}>
            {(e) => <span class="text-fg-muted">{e()}</span>}
          </Show>
        }
      >
        {(item) => {
          const picture = () => thumbUrl(bundle() ? "bundle" : "asset", id(), true)
          const deadline = () => {
            const t = ms(item().offSaleDeadline)
            return t !== null && t > Date.now() ? t : null
          }
          const hasAbout = () =>
            !!sound() || !!hat() || !!item().description.trim() || item().contents.length > 0 || item().tags.length > 0
          return (
            <div class="@container">
              <div class="flex flex-col gap-4 @3xl:grid @3xl:grid-cols-[minmax(0,1fr)_21rem] @3xl:items-start">
                <div class="contents @3xl:flex @3xl:min-w-0 @3xl:flex-col @3xl:gap-4">
                  <Show
                    when={previewable()}
                    fallback={
                      <Stage class={`order-1 ${STAGE_SIZE}`}>
                        <Thumbnail
                          alt={item().name}
                          src={picture()}
                          fallback={bundle() ? "avatar" : assetIcon(item().assetType)}
                        />
                      </Stage>
                    }
                  >
                    <AvatarViewer
                      stage
                      class={`order-1 ${STAGE_SIZE}`}
                      corner={
                        <SegmentedControl
                          size="sm"
                          aria-label="Show"
                          options={[
                            { value: "alone", label: "Alone" },
                            { value: "me", label: "On me", disabled: shell.active === null },
                          ]}
                          value={trying() ? "me" : "alone"}
                          onChange={(v) => {
                            if ((v === "me") !== trying()) run("market.try_on")
                          }}
                        />
                      }
                      source={trying() && shell.active !== null ? { kind: "me" } : undefined}
                      image={trying() ? thumbUrl("avatar", shell.active ?? 0, true) : picture()}
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

                  <Show when={hasAbout()}>
                    <Card class="order-3 flex min-w-0 flex-col">
                      <Show when={sound()} keyed>
                        {(src) => (
                          <section class="px-4 py-3.5">
                            <AudioPlayer
                              src={src}
                              label={item().name}
                              ref={(c) => {
                                player = c
                              }}
                            />
                          </section>
                        )}
                      </Show>
                      <Show when={hat()}>
                        {(h) => (
                          <section class="flex items-center gap-3 px-4 py-3">
                            <div class="flex min-w-0 flex-1 flex-col gap-0.5">
                              <h3 class="text-sm font-medium text-fg-muted">Sound hat</h3>
                              <span class="text-sm">
                                Plays a random sound every {h().wait[0]}–{h().wait[1]} s while worn.
                              </span>
                            </div>
                            <ToggleButton
                              size="sm"
                              icon="audio"
                              pressedIcon="audio-fill"
                              pressed={soundHatsOn()}
                              onChange={() => run("market.sound_hat")}
                            >
                              {soundHatsOn() ? "On" : "Off"}
                            </ToggleButton>
                          </section>
                        )}
                      </Show>
                      <Show when={item().description.trim()}>
                        <Show when={sound() || hat()}>
                          <Divider />
                        </Show>
                        <section class="flex flex-col gap-1.5 px-4 py-3.5">
                          <h3 class="text-sm font-medium text-fg-muted">Description</h3>
                          <Clamp lines={6}>
                            <Description text={item().description} />
                          </Clamp>
                        </section>
                      </Show>
                      <Show when={item().contents.length > 0}>
                        <Show when={sound() || hat() || item().description.trim()}>
                          <Divider />
                        </Show>
                        <section class="flex flex-col gap-1.5 px-4 py-3.5">
                          <h3 class="text-sm font-medium text-fg-muted">Includes</h3>
                          <div class="flex flex-wrap gap-x-3 gap-y-1">
                            <For each={item().contents}>
                              {(p) => (
                                <TextLink onClick={() => run("market.open", itemTarget(p.id))}>{p.name}</TextLink>
                              )}
                            </For>
                          </div>
                        </section>
                      </Show>
                      <Show when={item().tags.length > 0}>
                        <Show when={sound() || hat() || item().description.trim() || item().contents.length > 0}>
                          <Divider />
                        </Show>
                        <div class="flex flex-wrap items-center gap-1.5 px-4 py-2.5">
                          <span class="mr-1 text-sm text-fg-muted">Tags</span>
                          <For each={item().tags}>
                            {(tag) => (
                              <TextLink onClick={() => run("market.search", tag)}>
                                <Badge>{tag}</Badge>
                              </TextLink>
                            )}
                          </For>
                        </div>
                      </Show>
                    </Card>
                  </Show>

                  <Show when={wiki.article}>
                    {(a) => <WikiSummary view={wiki} article={a()} facts={3} class="order-3" />}
                  </Show>
                </div>

                <aside class="contents @3xl:sticky @3xl:top-4 @3xl:flex @3xl:min-w-0 @3xl:flex-col @3xl:gap-4">
                  <div class="order-2 flex min-w-0 flex-col gap-4">
                    <Card class="flex flex-col">
                      <div class="grid grid-cols-[minmax(0,1fr)_auto] items-start gap-x-2 px-4 pt-3.5 pb-3.5">
                        <div class="flex min-w-0 flex-col gap-1.5">
                          <Show
                            when={
                              item().limited || isSoundHat(id(), bundle()) || item().premiumOnly || item().publicDomain
                            }
                          >
                            <div class="flex flex-wrap items-center gap-1.5">
                              <Show when={item().limited}>{(l) => <Badge variant="success">{l()}</Badge>}</Show>
                              <Show when={isSoundHat(id(), bundle())}>
                                <SoundHatBadge />
                              </Show>
                              <Show when={item().premiumOnly}>
                                <Badge>
                                  <PremiumBadge bare size={12} class="-ml-0.5 mr-1" />
                                  Premium
                                </Badge>
                              </Show>
                              <Show when={item().publicDomain}>
                                <Badge>Free to take</Badge>
                              </Show>
                            </div>
                          </Show>
                          <h1 class="text-2xl font-semibold leading-tight [overflow-wrap:anywhere]">{item().name}</h1>
                          <span class="flex flex-wrap items-center gap-1 text-fg-muted">
                            {item().typeName} by{" "}
                            <TextLink
                              tone={item().creatorFake ? "danger" : "name"}
                              onClick={() =>
                                run(item().creatorIsGroup ? "group.open" : "user.open", String(item().creatorId))
                              }
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
                        <Menu
                          variant="ghost"
                          icon="more"
                          label="More"
                          class="-mt-1 -mr-2"
                          items={entityMenu(
                            target(),
                            item().bundle
                              ? []
                              : [
                                  menuItem("market.dependencies", { label: "View dependencies", icon: "inventory" }),
                                  ...templateItems(item().id, item().assetType),
                                ],
                          )}
                        />
                      </div>
                      <Divider />
                      <div class="flex flex-col gap-3 px-4 py-3.5">
                        <div class="flex flex-col gap-1">
                          <span class="flex min-w-0 items-center gap-1 text-xs text-fg-muted">
                            <Show when={cheapest()} fallback={item().fromReseller ? "Cheapest copy" : "Price"}>
                              {(r) => (
                                <>
                                  <span class="shrink-0">
                                    Cheapest copy
                                    <Show when={r().serial}> · #{full(r().serial ?? 0)}</Show> from
                                  </span>
                                  <TextLink
                                    tone="name"
                                    class="truncate"
                                    onClick={() => run("user.open", String(r().sellerId))}
                                  >
                                    {r().seller}
                                  </TextLink>
                                </>
                              )}
                            </Show>
                          </span>
                          <div class="flex flex-wrap items-center gap-x-2.5 gap-y-1">
                            <Switch>
                              <Match when={item().price === null}>
                                <span class="text-xl font-semibold text-fg-muted">{item().priceLabel}</span>
                              </Match>
                              <Match when={item().price === 0}>
                                <span class="text-3xl font-semibold">Free</span>
                              </Match>
                              <Match when={item().price}>
                                {(p) => <Robux size="lg" class="text-3xl font-semibold" amount={p()} />}
                              </Match>
                            </Switch>
                            <Show when={pctOver(item().price, value())}>{(n) => <VsValue pct={n()} />}</Show>
                            <Show when={item().price !== null && pctOver(item().price, value()) === 0}>
                              <Badge>At value</Badge>
                            </Show>
                          </div>
                        </div>
                        <Show when={!item().fromReseller && item().totalQuantity > 0}>
                          <Meter
                            label="Stock"
                            value={item().unitsLeft}
                            max={item().totalQuantity}
                            valueLabel={`${full(item().unitsLeft)} of ${full(item().totalQuantity)} left`}
                          />
                        </Show>
                        <Show when={deadline()}>
                          {(t) => (
                            <span class="flex items-center gap-1.5 text-sm text-warning">
                              <Icon name="clock" size={14} /> Off sale {date(t())}
                            </span>
                          )}
                        </Show>
                        <div class="flex flex-col gap-1.5">
                          <Show
                            when={item().inCatalog}
                            fallback={
                              <Button size="lg" icon="external" onClick={() => run("entity.open_website", target())}>
                                Open on website
                              </Button>
                            }
                          >
                            <div class="flex gap-1.5">
                              <Button
                                variant="primary"
                                size="lg"
                                class="flex-1"
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
                                size="lg"
                                items={accountItems("market.buy_as")}
                                disabled={item().price === null || accountItems("market.buy_as").length === 0}
                              >
                                Buy as…
                              </Menu>
                            </div>
                          </Show>
                          <div class="flex gap-1.5">
                            <ToggleButton
                              variant="secondary"
                              class="flex-1"
                              icon="star"
                              pressedIcon="star-fill"
                              pressed={view.favorited ?? false}
                              disabled={view.favorited === null}
                              loading={isRunning("market.favorite", target())}
                              onChange={() => run("market.favorite", target())}
                            >
                              Favorite
                              <Show when={item().inCatalog && item().favorites > 0}>
                                <span class="font-normal text-fg-muted tabular-nums">{compact(item().favorites)}</span>
                              </Show>
                            </ToggleButton>
                            <Show when={item().inCatalog}>
                              <ToggleButton
                                variant="secondary"
                                class="flex-1"
                                icon="heart"
                                pressedIcon="heart-fill"
                                pressed={view.wishlisted}
                                loading={isRunning("market.wishlist", target())}
                                onChange={() => run("market.wishlist", target())}
                              >
                                Wishlist
                              </ToggleButton>
                            </Show>
                          </div>
                        </div>
                      </div>
                      <Show when={view.ownedBy.length > 0}>
                        <Divider />
                        <span class="flex items-center gap-1.5 px-4 py-2.5 text-sm text-fg-muted">
                          <Icon name="check" size={14} class="text-success" />
                          <span class="min-w-0">
                            Owned by <span class="text-fg">{view.ownedBy.join(", ")}</span>
                          </span>
                        </span>
                      </Show>
                    </Card>
                    <Show when={item().inCatalog && (resells() || view.value)}>
                      <MarketCard collectibleId={resells() ? item().collectibleId : null} value={view.value} />
                    </Show>
                  </div>
                  <Facts columns class="order-4">
                    <FactGroup title={bundle() ? "Bundle" : "Item"}>
                      <IdFact id={item().id} target={target()} />
                      <Fact label="Type">{item().typeName}</Fact>
                      <Show when={item().category}>{(c) => <Fact label="Category">{c()}</Fact>}</Show>
                      <Show when={item().genres.length > 0}>
                        <Fact label="Genres">{item().genres.join(", ")}</Fact>
                      </Show>
                    </FactGroup>
                    <Show when={item().totalQuantity > 0 || item().inCatalog || item().sales > 0}>
                      <FactGroup title={item().totalQuantity > 0 ? "Supply" : "Popularity"}>
                        <Show when={item().totalQuantity > 0}>
                          <Fact label="Copies">{full(item().totalQuantity)}</Fact>
                          <Show when={!item().fromReseller}>
                            <Fact label="Left">{item().unitsLeft > 0 ? full(item().unitsLeft) : "Sold out"}</Fact>
                          </Show>
                        </Show>
                        <Show when={item().inCatalog}>
                          <Fact label="Favorites">{full(item().favorites)}</Fact>
                        </Show>
                        <Show when={item().sales > 0}>
                          <Fact label="Sales">{full(item().sales)}</Fact>
                        </Show>
                      </FactGroup>
                    </Show>
                    <FactGroup title="Dates">
                      <Fact label="Created">{when(item().created, date)}</Fact>
                      <Fact label="Updated">{when(item().updated, ago)}</Fact>
                      <Show when={ms(item().offSaleDeadline)}>
                        {(t) => <Fact label="Off sale on">{date(t())}</Fact>}
                      </Show>
                    </FactGroup>
                  </Facts>
                </aside>
              </div>
            </div>
          )
        }}
      </Show>

      <Show when={info()}>
        <Tabs aria-label="Item sections" items={sections()} value={section()} onChange={setSection}>
          <TabPanel value="dependencies">
            <Show when={!bundle() && info()}>
              {(item) => (
                <Show when={`${shell.active ?? 0}:${id()}:${item().assetType}`} keyed>
                  {(_key) => <Dependencies id={id()} assetType={item().assetType} />}
                </Show>
              )}
            </Show>
          </TabPanel>
          <TabPanel value="model">
            <Show when={!bundle() && info()}>
              {(item) => (
                <Show when={`${shell.active ?? 0}:${id()}:${item().assetType}`} keyed>
                  {(_key) => <ItemModel id={id()} assetType={item().assetType} />}
                </Show>
              )}
            </Show>
          </TabPanel>
          <TabPanel value="similar">
            <Show when={bundle() || info()?.assetType}>
              <Similar id={id()} bundle={bundle()} assetType={info()?.assetType ?? 0} />
            </Show>
          </TabPanel>
          <TabPanel value="resellers">
            <Show when={info()}>
              {(i) => (
                <Resellers
                  res={resellers}
                  value={value()}
                  id={id()}
                  bundle={bundle()}
                  name={i().name}
                  thumb={thumb()}
                />
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
          <TabPanel value="wiki">
            <WikiTab view={wiki} missing="The Roblox Wiki has no article about this item." />
          </TabPanel>
        </Tabs>
      </Show>
    </Page>
  )
}

/** The stage's size: wide, and never taller than a comfortable look at the item. */
const STAGE_SIZE = "aspect-[4/3] max-h-[30rem] w-full"

const when = (iso: string, f: (t: number) => string) => {
  const t = ms(iso)
  return t === null ? "–" : f(t)
}

const idle = <T,>() => ({ ...emptyList<T>(), loading: false })

/** How a price compares to value: under is good, a little over is normal, well over is a
 * warning. */
function VsValue(props: { pct: number }) {
  return (
    <Badge variant={props.pct < 0 ? "success" : props.pct > 10 ? "warning" : "neutral"}>
      {Math.abs(props.pct)}% {props.pct < 0 ? "under" : "over"} value
    </Badge>
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
      <VirtualGrid items={list.items} minItemWidth={128} estimateRowHeight={190} gap={12}>
        {(i) => <ItemTile item={i} />}
      </VirtualGrid>
    </ListState>
  )
}

function Resellers(props: {
  res: Paged<ResellerRow>
  value: number | null
  id: number
  bundle: boolean
  name: string
  thumb?: string
}) {
  const dialog = useDialog()
  onCleanup(
    handle("market.buy_resale", (arg) => {
      const r = props.res.list.items.find((x) => x.instanceId === arg?.trim())
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
  const columns = (): Column<ResellerRow>[] => [
    {
      key: "seller",
      label: "Seller",
      cell: (r) => (
        <span class="flex min-w-0 items-center gap-2">
          <Avatar size="xs" name={r.seller} src={thumbUrl("headshot", r.sellerId)} />
          <TextLink tone="name" class="truncate" onClick={() => run("user.open", String(r.sellerId))}>
            {r.seller}
          </TextLink>
          <Show when={r.verified}>
            <VerifiedBadge />
          </Show>
        </span>
      ),
    },
    { key: "serial", label: "Serial", width: "6rem", mono: true, cell: (r) => (r.serial ? `#${r.serial}` : "–") },
    { key: "price", label: "Price", width: "7rem", align: "end", cell: (r) => <Robux amount={r.price} /> },
    ...(props.value
      ? [
          {
            key: "vs",
            label: "vs value",
            width: "6rem",
            align: "end" as const,
            cell: (r: ResellerRow) => (
              <Show when={pctOver(r.price, props.value)} fallback={<span class="text-fg-muted">At value</span>}>
                {(n) => (
                  <span
                    class="tabular-nums"
                    classList={{
                      "text-success": n() < 0,
                      "text-warning": n() > 10,
                      "text-fg-muted": n() >= 0 && n() <= 10,
                    }}
                  >
                    {n() > 0 ? "+" : "−"}
                    {Math.abs(n())}%
                  </span>
                )}
              </Show>
            ),
          },
        ]
      : []),
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
    <ListState list={props.res.list} icon="robux" empty="Nobody is selling this" more={props.res.moreKey}>
      <DataTable
        aria-label="Resellers"
        columns={columns()}
        rows={props.res.list.items}
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

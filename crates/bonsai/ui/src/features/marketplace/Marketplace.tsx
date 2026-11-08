import { useSearchParams } from "@solidjs/router"
import { createEffect, createMemo, createSignal, onCleanup, Show } from "solid-js"
import { handle, run } from "~/lib/commands/registry"
import {
  type CatalogFilter,
  type CatalogOptions,
  type CategoryView,
  commands,
  type ItemCard,
  type Paged,
} from "~/lib/ipc/bindings"
import { emptyList, ListState } from "~/lib/list-state"
import { findCategory, priceOf, WISHLIST } from "~/lib/market-category"
import { shell, useLive } from "~/lib/store"
import { ItemTile, TILE_WIDTH } from "~/lib/tiles"
import {
  Button,
  CategoryNav,
  type CategorySection,
  SectionHeader,
  Select,
  SideMain,
  SidePage,
  SidePanel,
  Switch,
  showToast,
  TextField,
  VirtualGrid,
} from "~/lib/ui"
import { paramOf, useParamDraft } from "~/lib/url-params"
import { useValuesRefresh } from "~/lib/value-tags"

const digits = (v: string) => v.replace(/\D/g, "")

/** The catalog: categories and filters in a side panel, search and results beside them. All of
 * it lives in the URL (`?cat=&q=&sort=&sales=&min=&max=&creator=&offsale=`), so a tab keeps its
 * place while another is showing. */
export default function Marketplace() {
  const [params, setParams] = useSearchParams()
  useValuesRefresh()
  const categories = useLive(() => 0, commands.catalogCategories, emptyList<CategoryView>)
  const options = useLive(
    () => 0,
    commands.catalogOptions,
    (): CatalogOptions => ({ sorts: [], sales: [] }),
  )
  const cat = () => paramOf(params.cat)
  const found = createMemo(() => (cat() ? findCategory(categories.items, cat()) : null))
  // The first category ("All") until one is picked.
  const selected = () => found() ?? categories.items[0]?.taxonomy ?? ""
  const wishlistOpen = () => selected() === WISHLIST
  createEffect(() => {
    if (cat() && !found()) {
      showToast(`No category named ${cat()}`)
      setParams({ cat: null }, { replace: true })
    }
  })

  const keyword = () => paramOf(params.q)
  const [draft, setDraft] = createSignal("")
  createEffect(() => setDraft(keyword()))
  const sort = () => paramOf(params.sort) || "relevance"
  const sales = () => paramOf(params.sales) || "all"
  const offSale = () => paramOf(params.offsale) === "1"
  const min = useParamDraft("min")
  const max = useParamDraft("max")
  const creator = useParamDraft("creator")

  const filter = createMemo(
    (): CatalogFilter => ({
      taxonomy: selected(),
      keyword: keyword(),
      sort: sort(),
      sales: sales(),
      minPrice: priceOf(paramOf(params.min)),
      maxPrice: priceOf(paramOf(params.max)),
      creator: paramOf(params.creator).trim(),
      includeOffSale: offSale(),
    }),
  )
  const filters = () =>
    Number(sales() !== "all") +
    Number(priceOf(min.value()) !== null || priceOf(max.value()) !== null) +
    Number(!!creator.value().trim()) +
    Number(offSale())

  onCleanup(
    handle("market.clear_filters", () => {
      min.reset()
      max.reset()
      creator.reset()
      setParams({ sales: null, min: null, max: null, creator: null, offsale: null }, { replace: true })
    }),
  )

  const sections = createMemo((): CategorySection[] => [
    { items: categories.items.map((c) => ({ value: c.taxonomy, label: c.name, children: c.subcategories })) },
    { items: [{ value: WISHLIST, label: "Wishlist", icon: "heart" }] },
  ])

  return (
    <SidePage>
      <SidePanel class="w-56">
        <Show when={!wishlistOpen()}>
          <div class="flex flex-col gap-2 border-b border-line px-2 pb-3">
            <SectionHeader
              title="Filters"
              size="group"
              meta={filters() || undefined}
              actions={
                <Show when={filters() > 0}>
                  <Button size="sm" variant="ghost" onClick={() => run("market.clear_filters")}>
                    Clear
                  </Button>
                </Show>
              }
            />
            <Select
              size="sm"
              aria-label="Sales type"
              options={options.sales}
              value={sales()}
              onChange={(v) => setParams({ sales: v === "all" ? null : v }, { replace: true })}
            />
            <div class="flex items-center gap-2">
              <TextField
                size="sm"
                class="min-w-0 flex-1"
                inputMode="numeric"
                aria-label="Minimum price"
                placeholder="Min R$"
                value={min.value()}
                onChange={(v) => min.set(digits(v))}
              />
              <TextField
                size="sm"
                class="min-w-0 flex-1"
                inputMode="numeric"
                aria-label="Maximum price"
                placeholder="Max R$"
                value={max.value()}
                onChange={(v) => max.set(digits(v))}
              />
            </div>
            <TextField
              size="sm"
              aria-label="Creator"
              placeholder="Creator"
              value={creator.value()}
              onChange={creator.set}
            />
            <Switch checked={offSale()} onChange={(on) => setParams({ offsale: on ? "1" : null }, { replace: true })}>
              Include off sale
            </Switch>
          </div>
        </Show>
        <CategoryNav
          aria-label="Categories"
          sections={sections()}
          selected={selected()}
          onSelect={(v) => void run("market.category", v)}
          loading={categories.loading && categories.items.length === 0}
          error={categories.items.length === 0 && categories.error ? "Couldn't load categories" : null}
        />
      </SidePanel>
      <SideMain>
        <Show when={!wishlistOpen()} fallback={<Wishlist />}>
          <form
            class="flex items-center gap-2"
            onSubmit={(e) => {
              e.preventDefault()
              void run("market.search", draft())
            }}
          >
            <TextField
              class="min-w-48 flex-1"
              icon="search"
              aria-label="Search the Marketplace"
              placeholder="Search items, then Enter"
              value={draft()}
              onChange={setDraft}
            />
            <Select
              aria-label="Sort"
              options={options.sorts}
              value={sort()}
              onChange={(v) => setParams({ sort: v === "relevance" ? null : v }, { replace: true })}
            />
          </form>
          <Results filter={filter()} />
        </Show>
      </SideMain>
    </SidePage>
  )
}

function Results(props: { filter: CatalogFilter }) {
  const res = useLive(
    () => props.filter,
    commands.catalogSearch,
    (): Paged<ItemCard> => ({ list: emptyList<ItemCard>(), moreKey: "" }),
  )
  return (
    <ListState list={res.list} icon="search" empty="No items match" more={res.moreKey}>
      <VirtualGrid items={res.list.items} minItemWidth={TILE_WIDTH} estimateRowHeight={170} gap={12}>
        {(i) => <ItemTile item={i} />}
      </VirtualGrid>
    </ListState>
  )
}

function Wishlist() {
  const list = useLive(() => shell.active, commands.wishlist, emptyList<ItemCard>)
  return (
    <ListState
      list={list}
      icon="heart"
      empty="Your wishlist is empty"
      description="Add items from their page. The wishlist stays on this computer."
    >
      <VirtualGrid items={list.items} minItemWidth={TILE_WIDTH} estimateRowHeight={170} gap={12}>
        {(i) => <ItemTile item={i} />}
      </VirtualGrid>
    </ListState>
  )
}

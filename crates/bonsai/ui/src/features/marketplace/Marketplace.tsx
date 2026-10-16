import { useSearchParams } from "@solidjs/router"
import { createEffect, createMemo, createSignal, For, Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import {
  type CatalogFilter,
  type CatalogOptions,
  type CategoryView,
  commands,
  type ItemCard,
  type Paged,
} from "~/lib/ipc/bindings"
import { emptyList, ListState } from "~/lib/list-state"
import { shell, useLive } from "~/lib/store"
import { ItemTile, TILE_WIDTH } from "~/lib/tiles"
import { Badge, NavItem, Page, Select, Switch, TextField, VirtualGrid } from "~/lib/ui"
import { useValuesRefresh } from "~/lib/value-tags"

const WISHLIST = "wishlist"

const price = (s: string) => {
  const n = Number.parseInt(s, 10)
  return Number.isNaN(n) || n < 0 ? null : n
}

/** The catalog: categories on the left, search and filters on top, results in a virtual grid.
 * `?q=` (from `market.search`) sets the keyword. */
export default function Marketplace() {
  const [params] = useSearchParams()
  useValuesRefresh()
  const categories = useLive(() => 0, commands.catalogCategories, emptyList<CategoryView>)
  const options = useLive(
    () => 0,
    commands.catalogOptions,
    (): CatalogOptions => ({ sorts: [], sales: [] }),
  )
  const [category, setCategory] = createSignal("")
  const [sub, setSub] = createSignal("")
  const [draft, setDraft] = createSignal("")
  const [keyword, setKeyword] = createSignal("")
  const [sort, setSort] = createSignal("relevance")
  const [sales, setSales] = createSignal("all")
  const [min, setMin] = createSignal("")
  const [max, setMax] = createSignal("")
  const [creator, setCreator] = createSignal("")
  const [offSale, setOffSale] = createSignal(false)

  createEffect(() => {
    const q = typeof params.q === "string" ? params.q : ""
    setDraft(q)
    setKeyword(q)
  })
  // The first category ("All") until one is picked.
  const taxonomy = () => sub() || category() || categories.items[0]?.taxonomy || ""
  const filter = createMemo(
    (): CatalogFilter => ({
      taxonomy: taxonomy(),
      keyword: keyword(),
      sort: sort(),
      sales: sales(),
      minPrice: price(min()),
      maxPrice: price(max()),
      creator: creator().trim(),
      includeOffSale: offSale(),
    }),
  )
  const wishlistOpen = () => category() === WISHLIST
  const filters = () =>
    Number(sales() !== "all") + Number(price(min()) !== null || price(max()) !== null) + Number(!!creator().trim())

  return (
    <Page
      title="Marketplace"
      meta={
        <Show when={filters() > 0}>
          <Badge>{filters()} filters</Badge>
        </Show>
      }
    >
      <div class="flex items-start gap-6">
        <nav aria-label="Categories" class="sticky top-4 flex w-44 shrink-0 flex-col gap-0.5">
          <For each={categories.items}>
            {(c, i) => {
              const active = () => (category() || (i() === 0 ? c.taxonomy : "")) === c.taxonomy
              return (
                <>
                  <NavItem
                    label={c.name}
                    active={active() && !sub()}
                    onClick={() => {
                      setCategory(c.taxonomy)
                      setSub("")
                    }}
                  />
                  <Show when={active() && c.subcategories.length > 0}>
                    <div class="flex flex-col gap-0.5 pl-3">
                      <For each={c.subcategories}>
                        {(s) => <NavItem label={s.label} active={sub() === s.value} onClick={() => setSub(s.value)} />}
                      </For>
                    </div>
                  </Show>
                </>
              )
            }}
          </For>
          <NavItem
            label="Wishlist"
            icon="heart"
            active={wishlistOpen()}
            onClick={() => {
              setCategory(WISHLIST)
              setSub("")
            }}
          />
        </nav>
        <div class="flex min-w-0 flex-1 flex-col gap-3">
          <Show when={!wishlistOpen()} fallback={<Wishlist />}>
            <form
              class="flex flex-wrap items-center gap-2"
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
              <Select aria-label="Sort" options={options.sorts} value={sort()} onChange={setSort} />
              <Select aria-label="Sales type" options={options.sales} value={sales()} onChange={setSales} />
            </form>
            <div class="flex flex-wrap items-center gap-2">
              <TextField
                class="w-24"
                inputMode="numeric"
                aria-label="Minimum price"
                placeholder="Min R$"
                value={min()}
                onChange={setMin}
              />
              <TextField
                class="w-24"
                inputMode="numeric"
                aria-label="Maximum price"
                placeholder="Max R$"
                value={max()}
                onChange={setMax}
              />
              <TextField
                class="min-w-40 flex-1"
                aria-label="Creator"
                placeholder="Creator"
                value={creator()}
                onChange={setCreator}
              />
              <Switch checked={offSale()} onChange={setOffSale}>
                Include off sale
              </Switch>
            </div>
            <Results filter={filter()} />
          </Show>
        </div>
      </div>
    </Page>
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

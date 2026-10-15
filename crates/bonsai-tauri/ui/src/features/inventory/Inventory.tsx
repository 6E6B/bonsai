import { useParams } from "@solidjs/router"
import { createEffect, createMemo, createSignal, For, onCleanup, Show } from "solid-js"
import { handle, menuItem, run } from "~/lib/commands/registry"
import {
  commands,
  type InvCategoryView,
  type InvTotalsView,
  type ItemCard,
  type Paged,
  type UserLookup,
} from "~/lib/ipc/bindings"
import { emptyList, ListState } from "~/lib/list-state"
import { itemTarget } from "~/lib/menus"
import { shell, useLive } from "~/lib/store"
import { ItemTile, TILE_WIDTH } from "~/lib/tiles"
import {
  Button,
  EmptyState,
  NavItem,
  Page,
  Robux,
  Select,
  showToast,
  TextField,
  usePrompt,
  VirtualGrid,
} from "~/lib/ui"
import { useValuesRefresh } from "~/lib/value-tags"
import { demandLevel, worth } from "~/lib/values"

const COLLECTIBLES = "collectibles"

const demandOf = (i: ItemCard) => (i.values?.demand ? demandLevel(i.values.demand) : 0)

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
  return (
    <Show when={t.items > 0}>
      <span class="text-sm text-fg-muted">{t.items} collectibles · RAP</span>
      <Robux amount={t.rap} />
      <Show when={t.value}>
        {(v) => (
          <>
            <span class="text-sm text-fg-muted">· Value</span>
            <Robux amount={v()} />
          </>
        )}
      </Show>
    </Show>
  )
}

function Browser(props: { userId: number; mine: boolean }) {
  const categories = useLive(() => props.userId, commands.inventoryCategories, emptyList<InvCategoryView>)
  const [slot, setSlot] = createSignal(COLLECTIBLES)
  const [sort, setSort] = createSignal("newest")
  const [query, setQuery] = createSignal("")
  createEffect(() => {
    props.userId
    setSlot(COLLECTIBLES)
  })
  const res = useLive(
    () => [props.userId, slot(), sort() === "oldest"] as const,
    ([u, s, o]) => commands.inventoryItems(u, s, o),
    (): Paged<ItemCard> => ({ list: emptyList<ItemCard>(), moreKey: "" }),
  )
  const collectibles = () => slot() === COLLECTIBLES
  const shown = createMemo(() => {
    const q = query().trim().toLowerCase()
    const list = res.list.items.filter((i) => !q || i.name.toLowerCase().includes(q))
    const cost = (i: ItemCard) => (collectibles() ? (i.rap ?? i.price) : i.price) ?? Number.POSITIVE_INFINITY
    if (sort() === "price_asc") return [...list].sort((a, b) => cost(a) - cost(b))
    if (sort() === "price_desc")
      return [...list].sort(
        (a, b) =>
          (cost(b) === Number.POSITIVE_INFINITY ? -1 : cost(b)) - (cost(a) === Number.POSITIVE_INFINITY ? -1 : cost(a)),
      )
    if (sort() === "rap") return [...list].sort((a, b) => (b.rap ?? 0) - (a.rap ?? 0))
    if (sort() === "value")
      return [...list].sort((a, b) => (worth(b.values, b.rap) ?? 0) - (worth(a.values, a.rap) ?? 0))
    if (sort() === "demand") return [...list].sort((a, b) => demandOf(b) - demandOf(a))
    return list
  })
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
              <span class="px-2.5 pt-2 text-xs text-fg-muted">{c.name}</span>
              <For each={c.slots}>
                {(s) => <NavItem label={s.label} active={slot() === s.value} onClick={() => setSlot(s.value)} />}
              </For>
            </Show>
          )}
        </For>
      </nav>
      <div class="flex min-w-0 flex-1 flex-col gap-3">
        <div class="flex gap-2">
          <TextField
            class="flex-1"
            icon="search"
            aria-label="Filter inventory"
            placeholder="Filter loaded items"
            value={query()}
            onChange={setQuery}
          />
          <Select
            aria-label="Sort"
            value={sorts().some((s) => s.value === sort()) ? sort() : "newest"}
            onChange={setSort}
            options={sorts()}
          />
        </div>
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
                menu={
                  props.mine && !i.bundle && !i.limited
                    ? [
                        { kind: "separator" },
                        menuItem("inventory.delete", {
                          arg: itemTarget(i.id),
                          asAccount: props.userId,
                          icon: "trash",
                          danger: true,
                        }),
                      ]
                    : []
                }
              />
            )}
          </VirtualGrid>
        </ListState>
      </div>
    </div>
  )
}

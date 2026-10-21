import { createMemo, createSignal, Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import { type CommunityPass, commands, type ItemCard } from "~/lib/ipc/bindings"
import { emptyList, ListState } from "~/lib/list-state"
import { gameMenu } from "~/lib/menus"
import { useLive } from "~/lib/store"
import { thumbUrl } from "~/lib/thumb"
import { ItemTile, Price } from "~/lib/tiles"
import { Badge, ContextMenu, SegmentedControl, Tile, VirtualGrid } from "~/lib/ui"

const kinds = [
  { value: "all", label: "All items" },
  { value: "clothing", label: "Clothing" },
  { value: "ugc", label: "UGC" },
  { value: "passes", label: "Passes" },
]

/** T-shirts, shirts and pants: the classic clothing a community sells. */
const CLASSIC_CLOTHING = [2, 11, 12]
const isClothing = (i: ItemCard) => !i.bundle && CLASSIC_CLOTHING.includes(i.assetType)

/** What a community sells: avatar items (classic clothing and UGC) and its experiences' passes. */
export function Store(props: { groupId: number }) {
  const [kind, setKind] = createSignal("all")
  const items = useLive(() => props.groupId, commands.communityStore, emptyList<ItemCard>)
  // Passes load each experience's store, so only once someone asks for them.
  const passes = useLive(
    () => (kind() === "passes" ? props.groupId : 0),
    (g) => (g ? commands.communityPasses(g) : Promise.resolve(emptyList<CommunityPass>())),
    emptyList<CommunityPass>,
  )
  const shown = createMemo(() => {
    if (kind() === "clothing") return items.items.filter(isClothing)
    if (kind() === "ugc") return items.items.filter((i) => !isClothing(i))
    return items.items
  })
  return (
    <div class="flex flex-col gap-3">
      <SegmentedControl size="sm" aria-label="What to show" options={kinds} value={kind()} onChange={setKind} />
      <Show
        when={kind() === "passes"}
        fallback={
          <ListState
            list={{ ...items, items: shown() }}
            icon="marketplace"
            empty={items.items.length > 0 ? "None loaded yet" : "Nothing for sale"}
            more="store"
          >
            <VirtualGrid items={shown()} minItemWidth={128} estimateRowHeight={240} gap={12}>
              {(i) => <ItemTile item={i} />}
            </VirtualGrid>
          </ListState>
        }
      >
        <ListState list={passes} icon="game" empty="No passes for sale">
          <VirtualGrid items={passes.items} minItemWidth={128} estimateRowHeight={240} gap={12}>
            {(p) => <PassTile pass={p} />}
          </VirtualGrid>
        </ListState>
      </Show>
    </div>
  )
}

/** A pass in a grid. It is bought from its experience's store, so click opens the experience. */
function PassTile(props: { pass: CommunityPass }) {
  return (
    <ContextMenu items={gameMenu(props.pass.placeId)}>
      <Tile
        title={props.pass.name}
        subtitle={
          <span class="flex min-w-0 items-center gap-1.5">
            <Price amount={props.pass.price} />
            <span class="truncate">· {props.pass.game}</span>
          </span>
        }
        src={thumbUrl("game_pass", props.pass.id)}
        fallback="game"
        badge={props.pass.owned ? <Badge variant="success">Owned</Badge> : undefined}
        onClick={() => run("game.open", String(props.pass.placeId))}
      />
    </ContextMenu>
  )
}

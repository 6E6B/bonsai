import { createEffect, createMemo, createSignal, For, on, Show } from "solid-js"
import { useListCursor } from "~/lib/commands/hooks"
import { menuItem, run } from "~/lib/commands/registry"
import { commands, type List, type LookView, type OutfitView } from "~/lib/ipc/bindings"
import { ListState } from "~/lib/list-state"
import { itemMenu, itemTarget } from "~/lib/menus"
import { useLive } from "~/lib/store"
import { thumbUrl } from "~/lib/thumb"
import { Price, TILE_WIDTH } from "~/lib/tiles"
import {
  Badge,
  Button,
  ContextMenu,
  ListItem,
  type MenuEntry,
  SectionHeader,
  Skeleton,
  Swatch,
  TextField,
  Thumbnail,
  Tile,
  Tooltip,
  VirtualGrid,
} from "~/lib/ui"
import { wornGroups } from "./wearing"

/** The look they have on now, as opposed to a saved outfit's id. */
const NOW = "now"
const WEAR = "avatar.outfit"

const emptyLook = (): LookView => ({ loading: true, error: null, rig: "", items: [], colors: [] })

/** Show a filter once there are this many outfits. */
const FILTER_FROM = 12

/** What they wear now and the outfits they saved. The picked look shows on the left with its
 * items; the gallery on the right picks it, J / K included. Starts on what they wear now. */
export function Outfits(props: { userId: number; mine: boolean; outfits: List<OutfitView> }) {
  const [picked, setPicked] = createSignal(NOW)
  const [query, setQuery] = createSignal("")
  createEffect(
    on(
      () => props.userId,
      () => {
        setPicked(NOW)
        setQuery("")
      },
      { defer: true },
    ),
  )
  const look = useLive(
    () => `${props.userId}\n${picked()}`,
    (key) => {
      const [user, which] = key.split("\n")
      return which === NOW ? commands.profileLook(Number(user)) : commands.outfitLook(which)
    },
    emptyLook,
  )
  const outfit = () => props.outfits.items.find((o) => o.id === picked())
  /** "Wearing now" first, then outfits matching the filter. */
  const entries = createMemo(() => {
    const q = query().trim().toLowerCase()
    const rows = props.outfits.items.filter((o) => !q || o.name.toLowerCase().includes(q))
    return [NOW, ...rows.map((o) => o.id)]
  })
  const byId = createMemo(() => new Map(props.outfits.items.map((o) => [o.id, o])))
  const cursor = () => Math.max(0, entries().indexOf(picked()))
  useListCursor({
    count: () => entries().length,
    index: cursor,
    setIndex: (i) => setPicked(entries()[i] ?? NOW),
  })

  const image = (id: string, large: boolean) => {
    if (id === NOW) return thumbUrl("avatar", props.userId, large)
    const url = thumbUrl("outfit", id, large)
    const rev = byId().get(id)?.rev
    return url && rev ? `${url}?r=${rev}` : url
  }
  /** Your own outfits can be worn from here, as the account they belong to. */
  const outfitMenu = (id: string): MenuEntry[] =>
    props.mine ? [menuItem(WEAR, { arg: id, asAccount: props.userId, label: "Wear", icon: "avatar" })] : []
  const title = () => (picked() === NOW ? "Wearing now" : (outfit()?.name ?? "Outfit"))
  const groups = createMemo(() => wornGroups(look.items))
  const count = () => props.outfits.items.length

  return (
    <div class="@container">
      <div class="flex flex-col gap-6 @3xl:flex-row @3xl:items-start">
        <aside class="flex w-full flex-col gap-3 @3xl:sticky @3xl:top-4 @3xl:w-72 @3xl:shrink-0">
          <Thumbnail alt={title()} fallback="avatar" src={image(picked(), true)} class="w-full max-w-72" />
          <div class="flex items-start justify-between gap-2">
            <div class="flex min-w-0 flex-col">
              <h3 class="truncate font-medium">{title()}</h3>
              <span class="flex items-center gap-1.5 text-sm text-fg-muted">
                <Show when={look.rig}>
                  <Badge>{look.rig}</Badge>
                </Show>
                <Show when={!look.loading || look.items.length > 0}>
                  {look.items.length} {look.items.length === 1 ? "item" : "items"}
                </Show>
              </span>
            </div>
            <Show when={props.mine}>
              <Show
                when={picked() !== NOW}
                fallback={
                  <Button size="sm" icon="avatar" onClick={() => run("nav.avatar")}>
                    Edit
                  </Button>
                }
              >
                <Button size="sm" variant="primary" onClick={() => run(WEAR, picked(), props.userId)}>
                  Wear
                </Button>
              </Show>
            </Show>
          </div>
          <Show when={look.colors.some((c) => c.hex)}>
            <div class="flex items-center gap-1.5" role="img" aria-label="Body colors">
              <For each={look.colors}>
                {(c) => (
                  <Tooltip content={c.label}>
                    <Swatch display color={c.hex} label={c.label} />
                  </Tooltip>
                )}
              </For>
            </div>
          </Show>
          <Show
            when={look.items.length > 0}
            fallback={
              <Show
                when={look.loading}
                fallback={<span class="text-sm text-fg-muted">{look.error ?? "Nothing worn."}</span>}
              >
                <div class="flex flex-col gap-2">
                  <For each={[1, 2, 3, 4]}>{() => <Skeleton class="h-10 w-full rounded-md" />}</For>
                </div>
              </Show>
            }
          >
            <For each={groups()}>
              {(g) => (
                <div class="flex flex-col">
                  <SectionHeader title={g.label} meta={g.items.length} size="group" />
                  <div class="-mx-2 flex flex-col">
                    <For each={g.items}>
                      {(i) => (
                        <ContextMenu items={itemMenu(i.id, i.bundle, i.assetType)}>
                          <ListItem
                            leading={<Thumbnail class="w-9" alt="" fallback="avatar" src={thumbUrl("asset", i.id)} />}
                            title={i.name}
                            subtitle={
                              <span class="flex items-center gap-1.5">
                                <Price amount={i.price} />
                                <Show when={i.creator}>
                                  <span class="truncate">· {i.creator}</span>
                                </Show>
                              </span>
                            }
                            meta={
                              <Show when={i.limited}>
                                <Badge variant="success">Limited</Badge>
                              </Show>
                            }
                            onClick={() => run("market.open", itemTarget(i.id, i.bundle))}
                          />
                        </ContextMenu>
                      )}
                    </For>
                  </div>
                </div>
              )}
            </For>
          </Show>
        </aside>

        <section class="flex min-w-0 flex-1 flex-col gap-3">
          <SectionHeader
            title="Outfits"
            meta={props.outfits.loading && count() === 0 ? undefined : count()}
            actions={
              <Show when={count() >= FILTER_FROM}>
                <TextField
                  class="w-56"
                  icon="search"
                  aria-label="Filter outfits"
                  placeholder="Filter"
                  value={query()}
                  onChange={setQuery}
                />
              </Show>
            }
          />
          <ListState list={{ ...props.outfits, items: entries() }} icon="avatar" empty="" more="outfits" autoMore>
            <div data-cursor-list>
              <VirtualGrid items={entries()} minItemWidth={TILE_WIDTH} estimateRowHeight={170} gap={12}>
                {(id, i) => (
                  <div data-index={i} class="grid min-w-0">
                    <Show
                      when={id !== NOW}
                      fallback={
                        <Tile
                          title="Wearing now"
                          subtitle="Current look"
                          src={image(NOW, false)}
                          fallback="avatar"
                          selected={picked() === NOW}
                          fixed
                          onClick={() => setPicked(NOW)}
                        />
                      }
                    >
                      <ContextMenu items={outfitMenu(id)}>
                        <Tile
                          title={byId().get(id)?.name ?? "Outfit"}
                          src={image(id, false)}
                          fallback="avatar"
                          selected={picked() === id}
                          fixed
                          onClick={() => setPicked(id)}
                        />
                      </ContextMenu>
                    </Show>
                  </div>
                )}
              </VirtualGrid>
            </div>
          </ListState>
          <Show when={!props.outfits.loading && !props.outfits.error && count() === 0}>
            <span class="text-sm text-fg-muted">No saved outfits.</span>
          </Show>
          <Show when={query().trim() && entries().length === 1}>
            <span class="text-sm text-fg-muted">No outfits match.</span>
          </Show>
          <Show when={props.outfits.error}>{(e) => <span class="text-sm text-fg-muted">{e()}</span>}</Show>
        </section>
      </div>
    </div>
  )
}

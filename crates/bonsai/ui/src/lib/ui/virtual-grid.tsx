import {
  createEffect,
  createMemo,
  createRenderEffect,
  createRoot,
  createSignal,
  For,
  getOwner,
  type JSX,
  on,
  onCleanup,
  onMount,
  Show,
} from "solid-js"
import { createInPageVirtualizer, gridColumns, observeVirtualResize, toRows } from "./virtual"
import { virtualFocus } from "./virtual-focus"

export interface VirtualGridProps<T> {
  items: readonly T[]
  children: (item: T, index: number) => JSX.Element
  /** Each column is at least this wide. Leave out for a one-column list. */
  minItemWidth?: number
  /** Expected row height in px. Rows are measured once rendered. */
  estimateRowHeight: number
  /** Gap between rows and columns in px. */
  gap?: number
  /** Keep a keyboard cursor visible, even when its row has not mounted yet. */
  activeIndex?: number
  itemKey?: (item: T) => string | number
  followEnd?: boolean
  /** Shown when `items` is empty. */
  empty?: JSX.Element
  class?: string
}

/** A list or tile grid that renders only the rows in view. Use it for anything that can pass
 * about 100 entries: inventories, catalog results, members, friends. It scrolls with the page. */
export function VirtualGrid<T>(props: VirtualGridProps<T>) {
  let el: HTMLDivElement | undefined
  const [width, setWidth] = createSignal(0)
  const [focused, setFocused] = createSignal<number | null>(null)
  const gap = () => props.gap ?? 8
  const columns = createMemo(() => (props.minItemWidth ? gridColumns(width(), props.minItemWidth, gap()) : 1))
  const rows = createMemo(() => toRows(props.items, columns()))

  const keys = createMemo(() => (props.itemKey ? rows().map((row) => props.itemKey?.(row[0]) ?? 0) : undefined))

  // A change in column count moves every item to another row. Rows are separate `<For>`s, so
  // without this each tile would be destroyed and rebuilt, and its images would load again.
  // Keep each tile by its index and move the same node into its new row instead.
  const owner = getOwner()
  type Tile = { item: T; node: HTMLElement; uses: number; dispose: () => void }
  const tiles = new Map<number, Tile>()
  const tile = (item: T, index: number) => {
    let t = tiles.get(index)
    if (!t || t.item !== item) {
      t = createRoot(
        (dispose) => ({
          item,
          node: (
            <div data-slot="item" data-virtual-item={index}>
              {props.children(item, index)}
            </div>
          ) as HTMLElement,
          uses: 0,
          dispose,
        }),
        owner,
      )
      tiles.set(index, t)
    }
    const held = t
    held.uses++
    onCleanup(() => {
      held.uses--
      // The new row may claim the tile after the old row lets it go, so wait a tick.
      queueMicrotask(() => {
        if (held.uses > 0) return
        held.dispose()
        if (tiles.get(index) === held) tiles.delete(index)
      })
    })
    return held.node
  }
  onCleanup(() => {
    for (const t of tiles.values()) t.dispose()
    tiles.clear()
  })

  onMount(() => {
    if (!el) return
    // Read the width now so the first paint has the right columns, not one full-width column.
    setWidth(el.clientWidth)
    observeVirtualResize([el], ([e]) => setWidth(e.contentRect.width))
  })

  const { virtualizer, margin, indexes, start } = createInPageVirtualizer({
    el: () => el,
    count: () => rows().length,
    estimate: props.estimateRowHeight,
    gap: props.gap ?? 8,
    // Tile rows are much taller and more expensive than text rows.
    overscan: props.minItemWidth ? 2 : 6,
    keys,
    followEnd: props.followEnd,
    pinned: () => (focused() === null ? null : Math.floor((focused() ?? 0) / columns())),
  })

  const scrollToItem = (index: number) => virtualizer.scrollToIndex(Math.floor(index / columns()), { align: "auto" })
  createEffect(
    on(
      () => props.activeIndex,
      (index) => {
        if (index !== undefined && index >= 0 && index < props.items.length) scrollToItem(index)
      },
      { defer: true },
    ),
  )
  virtualFocus({ root: () => el, count: () => props.items.length, scroll: scrollToItem, pin: setFocused })

  return (
    <div ref={el} data-component="virtual-grid" data-virtual-root class={props.class}>
      <Show when={props.items.length > 0} fallback={props.empty}>
        <div data-slot="sizer" style={{ height: `${virtualizer.getTotalSize()}px` }}>
          <For each={indexes()}>
            {(index) => {
              const row = (
                <div
                  data-slot="row"
                  data-index={index}
                  ref={(r) => queueMicrotask(() => virtualizer.measureElement(r))}
                  style={{
                    transform: `translateY(${start(index) - margin()}px)`,
                    "grid-template-columns": `repeat(${columns()}, minmax(0, 1fr))`,
                    gap: `${gap()}px`,
                  }}
                />
              ) as HTMLDivElement
              // Tiles move between rows, so a row places its tiles itself. A <For> assumes it alone
              // owns its nodes and breaks when another row takes one.
              createRenderEffect(() => {
                const cols = columns()
                place(
                  row,
                  (rows()[index] ?? []).map((item, i) => tile(item, index * cols + i)),
                )
              })
              return row
            }}
          </For>
        </div>
      </Show>
    </div>
  )
}

/** Make `parent`'s children exactly `nodes`, leaving nodes already in place untouched so a
 * focused tile keeps focus. A node in another row moves here. */
function place(parent: HTMLElement, nodes: readonly Node[]) {
  nodes.forEach((node, i) => {
    const at = parent.childNodes[i] ?? null
    if (at !== node) parent.insertBefore(node, at)
  })
  while (parent.childNodes.length > nodes.length) parent.lastChild?.remove()
}

import { createEffect, createMemo, createSignal, For, type JSX, on, onMount, Show } from "solid-js"
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
  const columns = () => (props.minItemWidth ? gridColumns(width(), props.minItemWidth, gap()) : 1)
  const rows = createMemo(() => toRows(props.items, columns()))

  const keys = createMemo(() => (props.itemKey ? rows().map((row) => props.itemKey?.(row[0]) ?? 0) : undefined))

  onMount(() => {
    if (!el) return
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
            {(index) => (
              <div
                data-slot="row"
                data-index={index}
                ref={(r) => queueMicrotask(() => virtualizer.measureElement(r))}
                style={{
                  transform: `translateY(${start(index) - margin()}px)`,
                  "grid-template-columns": `repeat(${columns()}, minmax(0, 1fr))`,
                  gap: `${gap()}px`,
                }}
              >
                <For each={rows()[index]}>
                  {(item, i) => (
                    <div data-slot="item" data-virtual-item={index * columns() + i()}>
                      {props.children(item, index * columns() + i())}
                    </div>
                  )}
                </For>
              </div>
            )}
          </For>
        </div>
      </Show>
    </div>
  )
}

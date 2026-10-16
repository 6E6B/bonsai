import { createMemo, createSignal, For, type JSX, onCleanup, onMount, Show } from "solid-js"
import { createInPageVirtualizer, gridColumns, toRows } from "./virtual"

export interface VirtualGridProps<T> {
  items: readonly T[]
  children: (item: T, index: number) => JSX.Element
  /** Each column is at least this wide. Leave out for a one-column list. */
  minItemWidth?: number
  /** Expected row height in px. Rows are measured once rendered. */
  estimateRowHeight: number
  /** Gap between rows and columns in px. */
  gap?: number
  /** Shown when `items` is empty. */
  empty?: JSX.Element
  class?: string
}

/** A list or tile grid that renders only the rows in view. Use it for anything that can pass
 * about 100 entries: inventories, catalog results, members, friends. It scrolls with the page. */
export function VirtualGrid<T>(props: VirtualGridProps<T>) {
  let el: HTMLDivElement | undefined
  const [width, setWidth] = createSignal(0)
  const gap = () => props.gap ?? 8
  const columns = () => (props.minItemWidth ? gridColumns(width(), props.minItemWidth, gap()) : 1)
  const rows = createMemo(() => toRows(props.items, columns()))

  onMount(() => {
    if (!el) return
    const ro = new ResizeObserver(([e]) => setWidth(e.contentRect.width))
    ro.observe(el)
    onCleanup(() => ro.disconnect())
  })

  const { virtualizer, margin, indexes, start } = createInPageVirtualizer({
    el: () => el,
    count: () => rows().length,
    estimate: props.estimateRowHeight,
    gap: props.gap ?? 8,
  })

  return (
    <div ref={el} data-component="virtual-grid" class={props.class}>
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
                <For each={rows()[index]}>{(item, i) => props.children(item, index * columns() + i())}</For>
              </div>
            )}
          </For>
        </div>
      </Show>
    </div>
  )
}

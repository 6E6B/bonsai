import { createVirtualizer, defaultRangeExtractor } from "@tanstack/solid-virtual"
import { createMemo, createSignal, For, type JSX, onCleanup, onMount, Show } from "solid-js"
import { Icon } from "./icon"
import { virtualFocus } from "./virtual-focus"

export interface VirtualShelfProps<T> {
  items: readonly T[]
  children: (item: T, index: number) => JSX.Element
  itemWidth: number
  estimateHeight?: number
  "aria-label": string
  class?: string
}

/** Horizontal windowing with Shelf's fades and arrows. Widths are CSS pixels, as in VirtualGrid. */
export function VirtualShelf<T>(props: VirtualShelfProps<T>) {
  let root!: HTMLDivElement
  let track!: HTMLElement
  const [scroller, setScroller] = createSignal<HTMLElement>()
  const [height, setHeight] = createSignal(0)
  const [previous, setPrevious] = createSignal(false)
  const [next, setNext] = createSignal(false)
  const [focused, setFocused] = createSignal<number | null>(null)
  const virtualizer = createVirtualizer({
    horizontal: true,
    get count() {
      return props.items.length
    },
    getScrollElement: () => scroller() ?? null,
    estimateSize: () => props.itemWidth,
    gap: 12,
    overscan: 2,
    rangeExtractor: (range) => {
      const indexes = defaultRangeExtractor(range)
      const index = focused()
      if (index !== null && index < props.items.length && !indexes.includes(index)) indexes.push(index)
      return indexes.sort((a, b) => a - b)
    },
  })
  const visible = createMemo(() => virtualizer.getVirtualItems())
  const indexes = createMemo(() => visible().map((item) => item.index))
  const positions = createMemo(() => new Map(visible().map((item) => [item.index, item.start])))
  const measure = () => {
    setPrevious(track.scrollLeft > 1)
    setNext(track.scrollLeft + track.clientWidth < track.scrollWidth - 1)
  }
  onMount(() => {
    setScroller(track)
    const observer = new ResizeObserver(measure)
    observer.observe(track)
    onCleanup(() => observer.disconnect())
  })
  const observe = (element: HTMLDivElement) => {
    const observer = new ResizeObserver(([entry]) =>
      setHeight((height) => Math.max(height, Math.ceil(entry.contentRect.height))),
    )
    observer.observe(element)
    onCleanup(() => observer.disconnect())
  }
  virtualFocus({
    root: () => root,
    count: () => props.items.length,
    scroll: (index) => virtualizer.scrollToIndex(index, { align: "auto" }),
    pin: setFocused,
  })
  const step = (direction: number) => track.scrollBy({ left: direction * track.clientWidth * 0.8, behavior: "smooth" })
  return (
    <div
      ref={root}
      data-component="virtual-shelf"
      data-virtual-root
      class={props.class}
      data-prev={previous() ? "" : undefined}
      data-next={next() ? "" : undefined}
      style={{ "--shelf-item": `${props.itemWidth}px` }}
    >
      <section
        ref={track}
        data-slot="track"
        aria-label={props["aria-label"]}
        onScroll={measure}
        onKeyDown={(event) => {
          if (event.target !== event.currentTarget || !["ArrowLeft", "ArrowRight", "Home", "End"].includes(event.key))
            return
          event.preventDefault()
          if (event.key === "Home" || event.key === "End")
            virtualizer.scrollToIndex(event.key === "Home" ? 0 : props.items.length - 1)
          else step(event.key === "ArrowLeft" ? -1 : 1)
        }}
      >
        <div
          data-slot="sizer"
          style={{ width: `${virtualizer.getTotalSize()}px`, height: `${height() || props.estimateHeight || 190}px` }}
        >
          <For each={indexes()}>
            {(index) => (
              <div
                ref={observe}
                data-slot="item"
                data-virtual-item={index}
                style={{ width: `${props.itemWidth}px`, transform: `translateX(${positions().get(index) ?? 0}px)` }}
              >
                <Show when={index < props.items.length}>{props.children(props.items[index], index)}</Show>
              </div>
            )}
          </For>
        </div>
      </section>
      <Show when={previous()}>
        <button type="button" data-slot="prev" aria-label="Scroll left" tabIndex={-1} onClick={() => step(-1)}>
          <Icon name="chevron-left" size={16} />
        </button>
      </Show>
      <Show when={next()}>
        <button type="button" data-slot="next" aria-label="Scroll right" tabIndex={-1} onClick={() => step(1)}>
          <Icon name="chevron-right" size={16} />
        </button>
      </Show>
    </div>
  )
}

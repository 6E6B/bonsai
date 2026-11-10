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
    useAnimationFrameWithResizeObserver: true,
    horizontal: true,
    // The track only exists after mount. Reading it here re-runs the virtualizer once it does,
    // so the first paint already has its items instead of an empty shelf until the next update.
    get enabled() {
      return scroller() !== undefined
    },
    get count() {
      return props.items.length
    },
    getScrollElement: () => scroller() ?? null,
    estimateSize: () => props.itemWidth,
    gap: 12,
    overscan: 2,
    // Arrows from the virtualizer's own numbers. Reading the track's scroll size here would make
    // WebKit lay out the whole page for every shelf on every frame of a window resize.
    onChange: (instance) => {
      const offset = instance.scrollOffset ?? 0
      const width = instance.scrollRect?.width ?? 0
      setPrevious(offset > 1)
      // The track's 2px padding on each side adds 4px to what scrolls.
      setNext(width > 0 && offset + width < instance.getTotalSize() + 3)
    },
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
  onMount(() => setScroller(track))
  // Items are absolutely placed, so the row takes the height of the tallest item seen, which
  // keeps it from shrinking when scrolling past a shorter run. Not a grid with every item in one
  // cell: WebKit lays out every item of a grid again on each change, and with an aspect-ratio
  // thumbnail in each one, a window resize on a page of shelves dropped to a few frames a second.
  // The height is written a frame later, after ResizeObserver delivery.
  let tallest = 0
  let frame = 0
  const items = new ResizeObserver((entries) => {
    for (const entry of entries) tallest = Math.max(tallest, Math.ceil(entry.contentRect.height))
    cancelAnimationFrame(frame)
    frame = requestAnimationFrame(() => setHeight(tallest))
  })
  onCleanup(() => {
    items.disconnect()
    cancelAnimationFrame(frame)
  })
  const observe = (el: HTMLElement) => {
    items.observe(el)
    onCleanup(() => items.unobserve(el))
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
        // biome-ignore lint/a11y/noNoninteractiveTabindex: WebKit requires an explicit tab stop for keyboard scrolling.
        tabIndex={0}
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
          style={{
            width: `${virtualizer.getTotalSize()}px`,
            height: `${height() || (props.estimateHeight ?? 0)}px`,
          }}
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

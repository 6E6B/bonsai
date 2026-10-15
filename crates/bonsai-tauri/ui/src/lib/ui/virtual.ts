import { createVirtualizer } from "@tanstack/solid-virtual"
import { createMemo, createSignal, onCleanup, onMount } from "solid-js"

/** How many columns of at least `min` px fit in `width`, with `gap` px between them. */
export function gridColumns(width: number, min: number, gap: number): number {
  if (width <= 0 || min <= 0) return 1
  return Math.max(1, Math.floor((width + gap) / (min + gap)))
}

/** Splits `items` into rows of `columns`. The last row may be short. */
export function toRows<T>(items: readonly T[], columns: number): T[][] {
  const n = Math.max(1, columns)
  const rows: T[][] = []
  for (let i = 0; i < items.length; i += n) rows.push(items.slice(i, i + n))
  return rows
}

/** The nearest ancestor that scrolls vertically. */
function scrollParent(el: HTMLElement): HTMLElement | null {
  for (let p = el.parentElement; p; p = p.parentElement) {
    const o = getComputedStyle(p).overflowY
    if (o === "auto" || o === "scroll") return p
  }
  return null
}

export interface InPageOptions {
  /** The list element. Read after mount. */
  el: () => HTMLElement | undefined
  count: () => number
  estimate: number
  gap?: number
  overscan?: number
}

/**
 * A virtualizer for a list that scrolls with the page around it, so a long list inside a
 * screen doesn't get its own scrollbar. It finds the nearest scrolling ancestor and keeps
 * `scrollMargin` equal to the list's offset in it as the content above changes size.
 * Position rows with `translateY(item.start - margin())`.
 */
export function createInPageVirtualizer(opts: InPageOptions) {
  const [scroller, setScroller] = createSignal<HTMLElement | null>(null)
  const [margin, setMargin] = createSignal(0)

  onMount(() => {
    const el = opts.el()
    const s = el && scrollParent(el)
    if (!el || !s) return
    const measure = () =>
      setMargin(Math.round(el.getBoundingClientRect().top - s.getBoundingClientRect().top + s.scrollTop))
    const ro = new ResizeObserver(measure)
    ro.observe(s)
    if (s.firstElementChild) ro.observe(s.firstElementChild)
    measure()
    setScroller(s)
    onCleanup(() => ro.disconnect())
  })

  const virtualizer = createVirtualizer({
    get count() {
      return opts.count()
    },
    get getScrollElement() {
      const s = scroller()
      return () => s
    },
    get scrollMargin() {
      return margin()
    },
    estimateSize: () => opts.estimate,
    gap: opts.gap ?? 0,
    overscan: opts.overscan ?? 6,
  })

  // Tanstack hands out fresh item objects on every scroll or measure. Keying `<For>` by them
  // rebuilds every row, so a click whose press lands on the old node never fires. Key rows by
  // index instead and look up each row's position.
  const items = createMemo(() => virtualizer.getVirtualItems())
  const indexes = createMemo(() => items().map((v) => v.index))
  const starts = createMemo(() => new Map(items().map((v) => [v.index, v.start])))
  const start = (index: number) => starts().get(index) ?? 0

  return { virtualizer, margin, indexes, start }
}

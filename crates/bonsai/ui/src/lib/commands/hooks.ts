import { createEffect, onCleanup } from "solid-js"
import { cycle, type TabItem } from "~/lib/ui"
import { handle, setCommandContext } from "./registry"

/** Tells commands about "this" page while the screen is mounted, like `game:<placeId>`, so
 * "Like this experience" from the command bar knows which one. */
export function useCommandContext(context: () => string | null) {
  createEffect(() => setCommandContext(context()))
  onCleanup(() => setCommandContext(null))
}

/** Wires `[` and `]` (view.prev_subtab / view.next_subtab) to a screen's sub-tabs while it is
 * mounted. */
export function useSubTabs(items: () => TabItem[], value: () => string, set: (v: string) => void) {
  const off = [
    handle("view.next_subtab", () => set(cycle(items(), value(), 1))),
    handle("view.prev_subtab", () => set(cycle(items(), value(), -1))),
  ]
  onCleanup(() => {
    for (const f of off) f()
  })
}

export interface CursorOptions {
  count: () => number
  index: () => number
  setIndex: (i: number) => void
  /** Items per row in a grid, from `VirtualGrid`'s `onColumns`. J / K then move a row and
   * H / L move sideways. Leave out for a one-column list. */
  columns?: () => number
  /** Enter (list.open). */
  onOpen?: (i: number) => void
  /** X (list.select), for bulk actions. */
  onSelect?: (i: number) => void
}

/** Wires J / K / H / L / Enter / X (list.down, list.up, list.left, list.right, list.open,
 * list.select) to a list's cursor while the screen is mounted. Rows should carry `data-index`
 * so the cursor scrolls into view. */
export function useListCursor(opts: CursorOptions) {
  const move = (dir: number, sideways: boolean) => {
    const n = opts.count()
    const columns = Math.max(1, opts.columns?.() ?? 1)
    if (n === 0 || (sideways && columns === 1)) return
    const i = gridStep(opts.index(), dir, n, sideways ? 1 : columns)
    opts.setIndex(i)
    const mounted = [...document.querySelectorAll<HTMLElement>(`[data-cursor-list] [data-index="${i}"]`)].find(
      (element) => element.dataset.slot !== "row" || element.tagName === "TR",
    )
    mounted?.scrollIntoView({ block: "nearest" })
  }
  const off = [
    handle("list.down", () => move(1, false)),
    handle("list.up", () => move(-1, false)),
    handle("list.left", () => move(-1, true)),
    handle("list.right", () => move(1, true)),
    handle("list.open", () => opts.onOpen?.(opts.index())),
    handle("list.select", () => opts.onSelect?.(opts.index())),
  ]
  onCleanup(() => {
    for (const f of off) f()
  })
}

/** Where a cursor at `index` lands after one step in direction `dir` (1 or -1) of `step` items:
 * 1 for sideways, the column count for up and down. Sideways and one-column lists stop at the
 * ends. In a grid, up from the top row stays put and down into a short last row lands on its
 * last item. */
export function gridStep(index: number, dir: number, count: number, step: number) {
  const next = index + dir * step
  if (next >= 0 && next < count) return next
  if (step === 1) return Math.min(count - 1, Math.max(0, next))
  if (next < 0) return index
  return Math.floor(index / step) < Math.floor((count - 1) / step) ? count - 1 : index
}

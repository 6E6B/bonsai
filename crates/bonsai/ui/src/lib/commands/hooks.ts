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
  /** Enter (list.open). */
  onOpen?: (i: number) => void
  /** X (list.select), for bulk actions. */
  onSelect?: (i: number) => void
}

/** Wires J / K / Enter / X (list.down, list.up, list.open, list.select) to a list's cursor
 * while the screen is mounted. Rows should carry `data-index` so the cursor scrolls into view. */
export function useListCursor(opts: CursorOptions) {
  const move = (by: number) => {
    const n = opts.count()
    if (n === 0) return
    const i = Math.min(n - 1, Math.max(0, opts.index() + by))
    opts.setIndex(i)
    const mounted = [...document.querySelectorAll<HTMLElement>(`[data-cursor-list] [data-index="${i}"]`)].find(
      (element) => element.dataset.slot !== "row" || element.tagName === "TR",
    )
    mounted?.scrollIntoView({ block: "nearest" })
  }
  const off = [
    handle("list.down", () => move(1)),
    handle("list.up", () => move(-1)),
    handle("list.open", () => opts.onOpen?.(opts.index())),
    handle("list.select", () => opts.onSelect?.(opts.index())),
  ]
  onCleanup(() => {
    for (const f of off) f()
  })
}

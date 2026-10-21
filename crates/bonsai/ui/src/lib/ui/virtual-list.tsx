import type { JSX } from "solid-js"
import { VirtualGrid } from "./virtual-grid"

export interface VirtualListProps<T> {
  items: readonly T[]
  children: (item: T, index: number) => JSX.Element
  estimateRowHeight?: number
  gap?: number
  activeIndex?: number
  /** Stable identity, required when using followEnd with prepended history. */
  itemKey?: (item: T) => string | number
  /** Follow appends only near the end; preserve the reader's keyed row on prepend. */
  followEnd?: boolean
  empty?: JSX.Element
  class?: string
}

/** Measured, variable-height rows sharing the surrounding page or pane's scrollbar. */
export function VirtualList<T>(props: VirtualListProps<T>) {
  return (
    <VirtualGrid
      items={props.items}
      estimateRowHeight={props.estimateRowHeight ?? 48}
      gap={props.gap ?? 0}
      activeIndex={props.activeIndex}
      itemKey={props.itemKey}
      followEnd={props.followEnd}
      empty={props.empty}
      class={props.class}
    >
      {props.children}
    </VirtualGrid>
  )
}

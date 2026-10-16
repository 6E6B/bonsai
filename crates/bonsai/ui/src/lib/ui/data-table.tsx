import { For, type JSX, Show } from "solid-js"
import { createInPageVirtualizer } from "./virtual"

export interface Column<T> {
  key: string
  label: string
  /** A CSS grid track: `80px`, `1fr`, `minmax(120px, 2fr)`. Default `1fr`. Not `auto`: the
   * header and each row are separate grids, so `auto` columns would not line up. */
  width?: string
  align?: "start" | "end"
  /** Monospace, for server ids and serials. */
  mono?: boolean
  cell: (row: T) => JSX.Element
}

export interface DataTableProps<T> {
  columns: Column<T>[]
  rows: readonly T[]
  rowKey: (row: T) => string | number
  /** The row the keyboard cursor or detail pane is on. */
  selected?: string | number
  onRowClick?: (row: T) => void
  rowHeight?: number
  empty?: JSX.Element
  "aria-label": string
  class?: string
}

/** A table of rows with fixed columns: servers, members, transactions, trades. Renders only the
 * rows in view and scrolls with the page; the header sticks to the top. Put row buttons in a
 * cell; clicks on them don't trigger `onRowClick`. */
export function DataTable<T>(props: DataTableProps<T>) {
  let body: HTMLTableSectionElement | undefined
  const height = () => props.rowHeight ?? 34
  const template = () => props.columns.map((c) => c.width ?? "1fr").join(" ")
  const { virtualizer, margin, indexes, start } = createInPageVirtualizer({
    el: () => body,
    count: () => props.rows.length,
    estimate: height(),
  })

  const pick = (r: T, target: EventTarget | null) => {
    if ((target as HTMLElement | null)?.closest("button, a, input, [role=combobox]")) return
    props.onRowClick?.(r)
  }

  return (
    <div
      data-component="data-table"
      class={props.class}
      style={{ "--table-columns": template(), "--table-row": `${height()}px` }}
    >
      <table aria-label={props["aria-label"]} aria-rowcount={props.rows.length + 1}>
        <thead data-slot="header">
          <tr>
            <For each={props.columns}>
              {(c) => (
                <th data-slot="th" scope="col" data-align={c.align ?? "start"}>
                  {c.label}
                </th>
              )}
            </For>
          </tr>
        </thead>
        <tbody ref={body} data-slot="body" style={{ height: `${virtualizer.getTotalSize()}px` }}>
          <For each={indexes()}>
            {(index) => {
              const row = () => props.rows[index]
              return (
                <Show when={row()}>
                  {(r) => (
                    <tr
                      data-slot="row"
                      aria-rowindex={index + 2}
                      data-index={index}
                      data-selected={
                        props.selected !== undefined && props.rowKey(r()) === props.selected ? "" : undefined
                      }
                      aria-selected={props.onRowClick ? props.rowKey(r()) === props.selected : undefined}
                      tabIndex={props.onRowClick ? 0 : undefined}
                      style={{ transform: `translateY(${start(index) - margin()}px)` }}
                      onClick={(e) => pick(r(), e.target)}
                      onKeyDown={(e) => {
                        if (e.key === "Enter" && e.target === e.currentTarget) pick(r(), e.target)
                      }}
                    >
                      <For each={props.columns}>
                        {(c) => (
                          <td data-slot="td" data-align={c.align ?? "start"} data-mono={c.mono ? "" : undefined}>
                            {c.cell(r())}
                          </td>
                        )}
                      </For>
                    </tr>
                  )}
                </Show>
              )
            }}
          </For>
        </tbody>
      </table>
      <Show when={props.rows.length === 0}>{props.empty}</Show>
    </div>
  )
}

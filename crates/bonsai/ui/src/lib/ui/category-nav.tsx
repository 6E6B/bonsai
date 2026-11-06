import { createEffect, createMemo, createSignal, For, Index, on, Show } from "solid-js"
import {
  type CategoryNode,
  type CategorySection,
  containsSelected,
  expandable,
  nodeKey,
  openFor,
  pressNode,
  treeKey,
  visibleRows,
} from "./category-tree"
import { Icon } from "./icon"
import { Skeleton } from "./skeleton"
import { scrollParent } from "./virtual"

export interface CategoryNavProps {
  sections: readonly CategorySection[]
  /** The value of the row that is showing. */
  selected: string
  /** Run the screen's category command here. */
  onSelect: (value: string) => void
  /** Show placeholder rows above the sections while the list loads. */
  loading?: boolean
  error?: string | null
  "aria-label": string
}

const LOADING_ROWS = [0, 1, 2, 3, 4, 5, 6, 7]

/** Scrolls only the panel holding `el`, so the page behind a sticky panel stays put. */
function reveal(el: Element, within: HTMLElement) {
  const a = el.getBoundingClientRect()
  const b = within.getBoundingClientRect()
  if (a.bottom > b.bottom) within.scrollTop += a.bottom - b.bottom
  // The start of the row wins when it doesn't all fit.
  const c = el.getBoundingClientRect()
  if (c.top < b.top) within.scrollTop -= b.top - c.top
}

/** A category tree for a screen's side panel. One group is open at a time and the selected row
 * opens its own. Arrow keys only move focus, so nothing loads until Enter. */
export function CategoryNav(props: CategoryNavProps) {
  let tree: HTMLDivElement | undefined
  const auto = createMemo(() => openFor(props.sections, props.selected))
  const [open, setOpen] = createSignal<string | null>(null)
  createEffect(() => {
    props.selected
    setOpen(auto())
  })
  const rows = createMemo(() => visibleRows(props.sections, open()))
  const [focused, setFocused] = createSignal<string>()
  // The row that holds the tab stop.
  const stop = createMemo(() => {
    const all = rows()
    const has = (k: string | null | undefined) => (k && all.some((r) => r.key === k) ? k : undefined)
    return has(focused()) ?? has(props.selected) ?? has(auto()) ?? all[0]?.key
  })

  const rowEl = (key: string) => tree?.querySelector(`[data-key="${CSS.escape(key)}"]`)
  const scroller = () => (tree ? scrollParent(tree) : null)

  createEffect(
    on(
      () => [props.selected, open(), rows().length > 0] as const,
      ([selected, group, ready], prev) => {
        const area = scroller()
        if (!area) return
        // Show as much of a newly open group as fits.
        const groupRow = group ? rowEl(group) : null
        const children = groupRow?.nextElementSibling
        if (children) reveal(children, area)
        if (groupRow) reveal(groupRow, area)
        // The selected row, only when it changed: a group opened by hand stays in view.
        if (selected === prev?.[0] && ready === prev?.[2]) return
        const row = rowEl(selected)
        if (row) reveal(row, area)
      },
    ),
  )

  const focusRow = (key: string) => {
    setFocused(key)
    const row = rowEl(key)
    if (!(row instanceof HTMLElement)) return
    row.focus({ preventScroll: true })
    const area = scroller()
    if (area) reveal(row, area)
  }

  const onKey = (e: KeyboardEvent) => {
    if (e.altKey || e.ctrlKey || e.metaKey || e.shiftKey) return
    const at = (e.target as HTMLElement).closest<HTMLElement>("[data-key]")?.dataset.key
    const action = at ? treeKey(rows(), at, e.key, open()) : null
    if (!action) return
    e.preventDefault()
    e.stopPropagation()
    if (action.kind === "focus") focusRow(action.key)
    else if (action.kind === "open") setOpen(action.key)
    else setOpen(null)
  }

  const toggle = (key: string) => setOpen(open() === key ? null : key)

  const Node = (node: { get: () => CategoryNode }) => {
    const n = node.get
    const key = () => nodeKey(n())
    const isOpen = () => open() === key()
    return (
      <>
        <button
          type="button"
          role="treeitem"
          data-slot="row"
          data-key={key()}
          data-active={n().value !== undefined && n().value === props.selected ? "" : undefined}
          data-contains={containsSelected(n(), props.selected) && !isOpen() ? "" : undefined}
          aria-level={1}
          aria-selected={n().value === props.selected}
          aria-expanded={expandable(n()) ? isOpen() : undefined}
          tabindex={stop() === key() ? 0 : -1}
          onFocus={() => setFocused(key())}
          onClick={() => {
            const press = pressNode(n(), props.selected)
            if (press === "toggle") toggle(key())
            else if (press) props.onSelect(press.select)
          }}
        >
          <Show when={n().icon}>{(i) => <Icon name={i()} />}</Show>
          <span data-slot="label">{n().label}</span>
          <Show when={expandable(n())}>
            {/* Mouse only: the keyboard opens and closes with the arrows. */}
            <span
              data-slot="chevron"
              data-open={isOpen() ? "" : undefined}
              aria-hidden="true"
              onClick={(e) => {
                e.stopPropagation()
                toggle(key())
              }}
            >
              <Icon name="chevron-right" />
            </span>
          </Show>
        </button>
        <Show when={isOpen() && n().children}>
          {(children) => (
            // biome-ignore lint/a11y/useSemanticElements: a tree's nested rows, not a fieldset.
            <div role="group" data-slot="children">
              <For each={children()}>
                {(c) => (
                  <button
                    type="button"
                    role="treeitem"
                    data-slot="row"
                    data-child=""
                    data-key={c.value}
                    data-active={c.value === props.selected ? "" : undefined}
                    aria-level={2}
                    aria-selected={c.value === props.selected}
                    tabindex={stop() === c.value ? 0 : -1}
                    onFocus={() => setFocused(c.value)}
                    onClick={() => {
                      if (c.value !== props.selected) props.onSelect(c.value)
                    }}
                  >
                    <span data-slot="label">{c.label}</span>
                  </button>
                )}
              </For>
            </div>
          )}
        </Show>
      </>
    )
  }

  return (
    <div ref={tree} data-component="category-nav" role="tree" aria-label={props["aria-label"]} on:keydown={onKey}>
      <Show when={props.error}>
        <span data-slot="error">{props.error}</span>
      </Show>
      <Show when={props.loading}>
        <For each={LOADING_ROWS}>{() => <Skeleton class="h-7 w-full shrink-0" />}</For>
      </Show>
      <Index each={props.sections}>
        {(section, i) => (
          // biome-ignore lint/a11y/useSemanticElements: a tree's section of rows, not a fieldset.
          <div role="group" aria-label={section().label} data-slot="section" data-rule={i > 0 ? "" : undefined}>
            <Show when={section().label}>
              <span data-slot="section-label" aria-hidden="true">
                {section().label}
              </span>
            </Show>
            <Index each={section().items}>{(n) => <Node get={n} />}</Index>
          </div>
        )}
      </Index>
    </div>
  )
}

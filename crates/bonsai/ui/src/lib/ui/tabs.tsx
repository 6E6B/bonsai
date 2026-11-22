import { Tabs as KTabs } from "@kobalte/core/tabs"
import { createEffect, createSignal, For, type JSX, onCleanup, onMount, type ParentProps, Show } from "solid-js"

export interface TabItem {
  value: string
  label: string
  /** Shown after the label, like a pending request count. Hidden when 0 or missing. */
  count?: number
  /** The tab's content loaded empty. The label is muted but the tab stays and opens, so the
   * tab row keeps the same shape from one page to the next. */
  empty?: boolean
  disabled?: boolean
}

export interface TabsProps extends ParentProps {
  items: TabItem[]
  value: string
  onChange: (value: string) => void
  /** Accessible name for the tab list. */
  "aria-label"?: string
  /** The screen's buttons, at the right end of the tab row. Screens with top-level sub-tabs
   * put them here instead of under a page title. */
  actions?: JSX.Element
  class?: string
}

/** Sub-tabs inside a screen (Servers, Store, Badges). Put one `TabPanel` per item in
 * `children`; only the selected one renders. Wire `[` and `]` to `view.prev_subtab` /
 * `view.next_subtab` with `cycle()`. */
export function Tabs(props: TabsProps) {
  let list: HTMLDivElement | undefined
  // Which ends have tabs scrolled out of view, for the fade on that side.
  const [start, setStart] = createSignal(false)
  const [end, setEnd] = createSignal(false)
  const measure = () => {
    if (!list) return
    setStart(list.scrollLeft > 1)
    setEnd(list.scrollLeft + list.clientWidth < list.scrollWidth - 1)
  }
  // Tabs added or relabelled change the scroll width without resizing the list.
  createEffect(() => {
    props.items.map((t) => t.label)
    queueMicrotask(measure)
  })
  onMount(() => {
    if (!list) return
    measure()
    if (typeof ResizeObserver === "undefined") return
    const ro = new ResizeObserver(measure)
    ro.observe(list)
    onCleanup(() => ro.disconnect())
  })
  return (
    <KTabs data-component="tabs" class={props.class} value={props.value} onChange={props.onChange}>
      <div data-slot="bar">
        <KTabs.List
          ref={list}
          data-slot="list"
          aria-label={props["aria-label"]}
          data-fade-start={start() ? "" : undefined}
          data-fade-end={end() ? "" : undefined}
          onScroll={measure}
        >
          <For each={props.items}>
            {(t) => (
              <KTabs.Trigger
                data-slot="trigger"
                value={t.value}
                disabled={t.disabled}
                data-empty={t.empty ? "" : undefined}
              >
                {t.label}
                <Show when={t.count}>
                  <span data-slot="count">{t.count}</span>
                </Show>
              </KTabs.Trigger>
            )}
          </For>
        </KTabs.List>
        <Show when={props.actions}>
          <div data-slot="actions">{props.actions}</div>
        </Show>
      </div>
      {props.children}
    </KTabs>
  )
}

export interface TabPanelProps extends ParentProps {
  value: string
  class?: string
}

export function TabPanel(props: TabPanelProps) {
  return (
    <KTabs.Content data-slot="panel" class={props.class} value={props.value}>
      {props.children}
    </KTabs.Content>
  )
}

/** The value after (or before, with `step` -1) `value` in `items`, wrapping, skipping disabled
 * tabs. For the sub-tab shortcuts. */
export function cycle(items: TabItem[], value: string, step: 1 | -1): string {
  const enabled = items.filter((t) => !t.disabled)
  if (enabled.length === 0) return value
  const i = enabled.findIndex((t) => t.value === value)
  return enabled[(i + step + enabled.length) % enabled.length].value
}

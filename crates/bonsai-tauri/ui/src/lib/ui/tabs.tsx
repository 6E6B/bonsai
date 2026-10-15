import { Tabs as KTabs } from "@kobalte/core/tabs"
import { For, type ParentProps, Show } from "solid-js"

export interface TabItem {
  value: string
  label: string
  /** Shown after the label, like a pending request count. Hidden when 0 or missing. */
  count?: number
  disabled?: boolean
}

export interface TabsProps extends ParentProps {
  items: TabItem[]
  value: string
  onChange: (value: string) => void
  /** Accessible name for the tab list. */
  "aria-label"?: string
  class?: string
}

/** Sub-tabs inside a screen (Servers, Store, Badges). Put one `TabPanel` per item in
 * `children`; only the selected one renders. Wire `[` and `]` to `view.prev_subtab` /
 * `view.next_subtab` with `cycle()`. */
export function Tabs(props: TabsProps) {
  return (
    <KTabs data-component="tabs" class={props.class} value={props.value} onChange={props.onChange}>
      <KTabs.List data-slot="list" aria-label={props["aria-label"]}>
        <For each={props.items}>
          {(t) => (
            <KTabs.Trigger data-slot="trigger" value={t.value} disabled={t.disabled}>
              {t.label}
              <Show when={t.count}>
                <span data-slot="count">{t.count}</span>
              </Show>
            </KTabs.Trigger>
          )}
        </For>
      </KTabs.List>
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

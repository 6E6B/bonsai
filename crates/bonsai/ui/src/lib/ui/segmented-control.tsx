import { ToggleGroup as KToggleGroup } from "@kobalte/core/toggle-group"
import { createMemo, For } from "solid-js"
import { normalize, type Option } from "./options"

export interface SegmentedControlProps {
  options: Option[]
  value: string
  onChange: (value: string) => void
  "aria-label": string
  size?: "sm" | "md"
  class?: string
}

/** Picks one of a few options in place, like a sort order or R6/R15. Clicking the selected
 * option keeps it selected. Use Select for more than about five options. */
export function SegmentedControl(props: SegmentedControlProps) {
  const options = createMemo(() => normalize(props.options))
  return (
    <KToggleGroup
      data-component="segmented-control"
      data-size={props.size ?? "md"}
      class={props.class}
      aria-label={props["aria-label"]}
      value={props.value}
      onChange={(v) => v && props.onChange(v)}
    >
      <For each={options()}>
        {(o) => (
          <KToggleGroup.Item data-slot="item" value={o.value} disabled={o.disabled}>
            {o.label}
          </KToggleGroup.Item>
        )}
      </For>
    </KToggleGroup>
  )
}

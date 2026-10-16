import { RadioGroup as KRadioGroup } from "@kobalte/core/radio-group"
import { createMemo, For, Show } from "solid-js"
import { normalize, type Option } from "./options"

export interface RadioGroupProps {
  options: Option[]
  label?: string
  /** Accessible name when there is no visible label. */
  "aria-label"?: string
  description?: string
  value?: string
  defaultValue?: string
  onChange?: (value: string) => void
  orientation?: "horizontal" | "vertical"
  disabled?: boolean
  class?: string
}

export function RadioGroup(props: RadioGroupProps) {
  const options = createMemo(() => normalize(props.options))
  return (
    <KRadioGroup
      data-component="radio-group"
      data-orientation={props.orientation ?? "vertical"}
      class={props.class}
      aria-label={props["aria-label"]}
      value={props.value}
      defaultValue={props.defaultValue}
      onChange={props.onChange}
      orientation={props.orientation ?? "vertical"}
      disabled={props.disabled}
    >
      <Show when={props.label}>
        <KRadioGroup.Label data-slot="field-label">{props.label}</KRadioGroup.Label>
      </Show>
      <div data-slot="items" role="presentation">
        <For each={options()}>
          {(o) => (
            <KRadioGroup.Item data-slot="item" value={o.value} disabled={o.disabled}>
              <KRadioGroup.ItemInput data-slot="input" />
              <KRadioGroup.ItemControl data-slot="circle">
                <KRadioGroup.ItemIndicator data-slot="dot" />
              </KRadioGroup.ItemControl>
              <KRadioGroup.ItemLabel data-slot="label">{o.label}</KRadioGroup.ItemLabel>
            </KRadioGroup.Item>
          )}
        </For>
      </div>
      <Show when={props.description}>
        <KRadioGroup.Description data-slot="field-description">{props.description}</KRadioGroup.Description>
      </Show>
    </KRadioGroup>
  )
}

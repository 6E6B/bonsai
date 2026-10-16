import { Checkbox as KCheckbox } from "@kobalte/core/checkbox"
import { type JSX, Show } from "solid-js"
import { Icon } from "./icon"

export interface CheckboxProps {
  /** The label. Always pass one; set `hideLabel` to keep it for screen readers only. */
  children: JSX.Element
  hideLabel?: boolean
  description?: string
  checked?: boolean
  defaultChecked?: boolean
  onChange?: (checked: boolean) => void
  disabled?: boolean
  class?: string
}

export function Checkbox(props: CheckboxProps) {
  return (
    <KCheckbox
      data-component="checkbox"
      class={props.class}
      checked={props.checked}
      defaultChecked={props.defaultChecked}
      onChange={props.onChange}
      disabled={props.disabled}
    >
      <KCheckbox.Input data-slot="input" />
      <KCheckbox.Control data-slot="box">
        <KCheckbox.Indicator data-slot="indicator">
          <Icon name="check" size={12} />
        </KCheckbox.Indicator>
      </KCheckbox.Control>
      <span data-slot="text" classList={{ "sr-only": props.hideLabel }}>
        <KCheckbox.Label data-slot="label">{props.children}</KCheckbox.Label>
        <Show when={props.description}>
          <KCheckbox.Description data-slot="field-description">{props.description}</KCheckbox.Description>
        </Show>
      </span>
    </KCheckbox>
  )
}

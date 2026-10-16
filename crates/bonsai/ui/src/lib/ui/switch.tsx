import { Switch as KSwitch } from "@kobalte/core/switch"
import { type JSX, Show } from "solid-js"

export interface SwitchProps {
  /** The label. Always pass one; set `hideLabel` to keep it for screen readers only. */
  children: JSX.Element
  hideLabel?: boolean
  description?: string
  checked?: boolean
  defaultChecked?: boolean
  onChange?: (checked: boolean) => void
  disabled?: boolean
  /** `inline` puts the switch before its label. `row` puts the label first and the switch at
   * the far end, for settings lists. */
  layout?: "inline" | "row"
  class?: string
}

export function Switch(props: SwitchProps) {
  return (
    <KSwitch
      data-component="switch"
      data-layout={props.layout ?? "inline"}
      class={props.class}
      checked={props.checked}
      defaultChecked={props.defaultChecked}
      onChange={props.onChange}
      disabled={props.disabled}
    >
      <KSwitch.Input data-slot="input" />
      <KSwitch.Control data-slot="track">
        <KSwitch.Thumb data-slot="thumb" />
      </KSwitch.Control>
      <span data-slot="text" classList={{ "sr-only": props.hideLabel }}>
        <KSwitch.Label data-slot="label">{props.children}</KSwitch.Label>
        <Show when={props.description}>
          <KSwitch.Description data-slot="field-description">{props.description}</KSwitch.Description>
        </Show>
      </span>
    </KSwitch>
  )
}

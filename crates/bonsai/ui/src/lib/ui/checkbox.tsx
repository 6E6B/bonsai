import { Checkbox as KCheckbox } from "@kobalte/core/checkbox"
import { createSignal, type JSX, Show } from "solid-js"

export interface CheckboxProps {
  /** The label. Always pass one; set `hideLabel` to keep it for screen readers only. */
  children: JSX.Element
  hideLabel?: boolean
  description?: string
  checked?: boolean
  defaultChecked?: boolean
  /** Some but not all of a group are checked: "Select all" over a partly picked list. Shows a dash. */
  indeterminate?: boolean
  onChange?: (checked: boolean) => void
  disabled?: boolean
  class?: string
}

/** The check and dash are drawn as strokes so they can draw in and erase (see checkbox.css). */
export function Checkbox(props: CheckboxProps) {
  // A click focuses the hidden input, which browsers count as :focus-visible. Hide the focus ring
  // until the next key press.
  const [pointer, setPointer] = createSignal(false)
  return (
    <KCheckbox
      data-component="checkbox"
      data-pointer={pointer() ? "" : undefined}
      onPointerDown={() => setPointer(true)}
      onKeyDown={() => setPointer(false)}
      class={props.class}
      checked={props.checked}
      defaultChecked={props.defaultChecked}
      indeterminate={props.indeterminate}
      onChange={props.onChange}
      disabled={props.disabled}
    >
      <KCheckbox.Input data-slot="input" />
      <KCheckbox.Control data-slot="box">
        <KCheckbox.Indicator data-slot="indicator" forceMount>
          <svg viewBox="0 0 16 16" width="16" height="16" aria-hidden="true">
            <path data-slot="check" d="M4 8.25 6.75 11 12 5.25" pathLength="1" />
            <path data-slot="dash" d="M4.5 8h7" pathLength="1" />
          </svg>
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

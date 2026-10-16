import { ToggleButton as KToggleButton } from "@kobalte/core/toggle-button"
import { type JSX, Show } from "solid-js"
import type { ButtonSize, ButtonVariant } from "./button"
import { Icon, type IconName } from "./icon"

export interface ToggleButtonProps {
  /** Comes from the store, never from local state: the button shows what the server says. */
  pressed: boolean
  /** Run the command here. The store update flips `pressed`. */
  onChange: (pressed: boolean) => void
  icon: IconName
  /** Icon while pressed, usually the `-fill` version of `icon`. */
  pressedIcon?: IconName
  children?: JSX.Element
  /** Accessible name when there are no children. */
  label?: string
  variant?: ButtonVariant
  size?: ButtonSize
  disabled?: boolean
  class?: string
}

/** A button that is on or off: favorite, like, notify, pin. Styled like `Button`. */
export function ToggleButton(props: ToggleButtonProps) {
  return (
    <KToggleButton
      data-component="button"
      data-variant={props.variant ?? "ghost"}
      data-size={props.size ?? "md"}
      data-icon-only={props.children ? undefined : ""}
      class={props.class}
      aria-label={props.label}
      pressed={props.pressed}
      onChange={props.onChange}
      disabled={props.disabled}
    >
      <Icon name={props.pressed ? (props.pressedIcon ?? props.icon) : props.icon} />
      <Show when={props.children}>{props.children}</Show>
    </KToggleButton>
  )
}

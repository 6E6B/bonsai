import { Button as KButton } from "@kobalte/core/button"
import { type ComponentProps, Show, splitProps } from "solid-js"
import { Icon, type IconName } from "./icon"
import { Spinner } from "./spinner"

export type ButtonVariant = "primary" | "secondary" | "ghost" | "danger"
export type ButtonSize = "sm" | "md" | "lg"

export interface ButtonProps extends ComponentProps<"button"> {
  variant?: ButtonVariant
  size?: ButtonSize
  /** Icon before the label. */
  icon?: IconName
  /** Shows a spinner in place of the icon and disables the button. */
  loading?: boolean
}

export function Button(props: ButtonProps) {
  const [local, rest] = splitProps(props, ["variant", "size", "icon", "loading", "disabled", "children"])
  return (
    <KButton
      data-component="button"
      data-variant={local.variant ?? "secondary"}
      data-size={local.size ?? "md"}
      disabled={local.disabled || local.loading}
      {...rest}
    >
      <Show when={!local.loading} fallback={<Spinner size="sm" />}>
        <Show when={local.icon}>{(name) => <Icon name={name()} />}</Show>
      </Show>
      {local.children}
    </KButton>
  )
}

export interface IconButtonProps extends Omit<ButtonProps, "icon" | "children"> {
  icon: IconName
  /** Accessible name. Also used as the tooltip if you wrap it in one. */
  label: string
}

/** A square button with only an icon. */
export function IconButton(props: IconButtonProps) {
  const [local, rest] = splitProps(props, ["label", "variant"])
  return <Button data-icon-only="" aria-label={local.label} variant={local.variant ?? "ghost"} {...rest} />
}

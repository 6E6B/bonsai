import { Alert as KAlert } from "@kobalte/core/alert"
import { type ParentProps, Show } from "solid-js"
import { Icon, type IconName } from "./icon"

export type AlertVariant = "info" | "success" | "warning" | "danger"

export interface AlertProps extends ParentProps {
  title: string
  variant?: AlertVariant
  /** Buttons on the right, like "Paste new cookie". */
  actions?: ParentProps["children"]
  class?: string
}

const icons: Record<AlertVariant, IconName> = {
  info: "info",
  success: "circle-check",
  warning: "alert",
  danger: "circle-x",
}

/** A persistent notice inside a screen or above the page: offline, expired cookie, captcha
 * needed, keychain unavailable. For short-lived feedback use a toast. */
export function Alert(props: AlertProps) {
  const variant = () => props.variant ?? "info"
  return (
    <KAlert data-component="alert" data-variant={variant()} class={props.class}>
      <span data-slot="icon">
        <Icon name={icons[variant()]} />
      </span>
      <div data-slot="body">
        <p data-slot="title">{props.title}</p>
        <Show when={props.children}>
          <div data-slot="description">{props.children}</div>
        </Show>
      </div>
      <Show when={props.actions}>
        <div data-slot="actions">{props.actions}</div>
      </Show>
    </KAlert>
  )
}

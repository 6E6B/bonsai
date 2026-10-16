import { Badge as KBadge } from "@kobalte/core/badge"
import { type ComponentProps, splitProps } from "solid-js"

export type BadgeVariant =
  | "neutral"
  | "accent"
  | "success"
  | "warning"
  | "danger"
  | "red"
  | "orange"
  | "yellow"
  | "green"
  | "blue"
  | "purple"
  | "pink"

export interface BadgeProps extends ComponentProps<"span"> {
  variant?: BadgeVariant
}

export function Badge(props: BadgeProps) {
  const [local, rest] = splitProps(props, ["variant"])
  return <KBadge data-component="badge" data-variant={local.variant ?? "neutral"} {...rest} />
}

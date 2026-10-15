import { Button as KButton } from "@kobalte/core/button"
import { type JSX, Show } from "solid-js"
import { Dynamic } from "solid-js/web"

export interface ListItemProps {
  title: JSX.Element
  subtitle?: JSX.Element
  /** Left side: an Avatar or Thumbnail. */
  leading?: JSX.Element
  /** Right side, inside the row: a time, a count, a badge. Not clickable. */
  meta?: JSX.Element
  /** Buttons on the right, outside the clickable area. */
  actions?: JSX.Element
  /** Show `actions` only on hover and focus, for dense lists. */
  actionsOnHover?: boolean
  /** The row the keyboard cursor or the detail pane is on. */
  selected?: boolean
  /** Bold title and a dot, for unread messages and notifications. */
  unread?: boolean
  /** Show the full title and subtitle instead of truncating them. */
  expanded?: boolean
  /** Muted, for offline friends and archived things. */
  dim?: boolean
  /** Without it the row is not clickable. */
  onClick?: () => void
  /** For lists that track a keyboard cursor: `list.down` / `list.up` scroll it into view. */
  "data-index"?: number
  class?: string
}

/** A row in a list: friends, conversations, messages, trades, notifications. */
export function ListItem(props: ListItemProps) {
  return (
    <div
      data-component="list-item"
      data-selected={props.selected ? "" : undefined}
      data-unread={props.unread ? "" : undefined}
      data-dim={props.dim ? "" : undefined}
      data-expanded={props.expanded ? "" : undefined}
      data-hover-actions={props.actionsOnHover ? "" : undefined}
      data-index={props["data-index"]}
      class={props.class}
    >
      <Dynamic
        component={props.onClick ? KButton : "div"}
        data-slot="main"
        onClick={props.onClick}
        aria-current={props.selected ? "true" : undefined}
      >
        <Show when={props.leading}>
          <span data-slot="leading">{props.leading}</span>
        </Show>
        <span data-slot="text">
          <span data-slot="title">{props.title}</span>
          <Show when={props.subtitle}>
            <span data-slot="subtitle">{props.subtitle}</span>
          </Show>
        </span>
        <Show when={props.meta}>
          <span data-slot="meta">{props.meta}</span>
        </Show>
      </Dynamic>
      <Show when={props.actions}>
        <div data-slot="actions">{props.actions}</div>
      </Show>
    </div>
  )
}

import { Button as KButton } from "@kobalte/core/button"
import { Show } from "solid-js"
import { filledIcon, Icon, type IconName } from "./icon"
import { Tooltip } from "./tooltip"

export interface NavItemProps {
  label: string
  icon?: IconName
  /** True for the screen that is showing. */
  active?: boolean
  /** Unread or pending count, shown on the right. Hidden when 0. */
  count?: number
  /** Shortcut label shown on hover, like `G H`. */
  shortcut?: string
  /** Run the screen's `nav.*` command here. */
  onClick: () => void
  /** Icon only, for the collapsed sidebar. The label moves to a tooltip. */
  compact?: boolean
  class?: string
}

/** One row of the sidebar. */
export function NavItem(props: NavItemProps) {
  const button = () => (
    <KButton
      data-component="nav-item"
      data-active={props.active ? "" : undefined}
      data-compact={props.compact ? "" : undefined}
      aria-current={props.active ? "page" : undefined}
      aria-label={props.compact ? props.label : undefined}
      class={props.class}
      onClick={props.onClick}
    >
      <Show when={props.icon}>{(n) => <Icon name={props.active ? filledIcon(n()) : n()} />}</Show>
      <span data-slot="label">{props.label}</span>
      <Show when={props.shortcut}>
        <span data-slot="shortcut">{props.shortcut}</span>
      </Show>
      <Show when={props.count}>
        <span data-slot="count">{props.count}</span>
      </Show>
    </KButton>
  )
  return (
    <Show when={props.compact} fallback={button()}>
      <Tooltip content={props.label} keys={props.shortcut ? [props.shortcut] : undefined} placement="right">
        {button()}
      </Tooltip>
    </Show>
  )
}

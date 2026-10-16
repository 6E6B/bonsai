import { Button as KButton } from "@kobalte/core/button"
import type { ParentProps } from "solid-js"
import { Tooltip } from "./tooltip"

/** Matches `Color` in the IPC bindings. */
export type TagColor = "none" | "red" | "orange" | "yellow" | "green" | "blue" | "purple" | "pink"

export interface RailItemProps extends ParentProps {
  /** Tooltip and accessible name. */
  label: string
  /** Shortcut in the tooltip, like `Ctrl+1`. */
  shortcut?: string
  active?: boolean
  /** The account's color tag, drawn as the active marker and a ring. */
  color?: TagColor
  onClick: () => void
}

/** A square slot in the account rail. Put an `Avatar` (or an icon) in `children`. */
export function RailItem(props: RailItemProps) {
  return (
    <Tooltip content={props.label} keys={props.shortcut ? [props.shortcut] : undefined} placement="right">
      <KButton
        data-component="rail-item"
        data-active={props.active ? "" : undefined}
        data-color={props.color ?? "none"}
        aria-label={props.label}
        aria-current={props.active ? "true" : undefined}
        onClick={props.onClick}
      >
        {props.children}
      </KButton>
    </Tooltip>
  )
}

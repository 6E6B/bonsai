import { Tooltip as KTooltip } from "@kobalte/core/tooltip"
import { type JSX, Show } from "solid-js"
import { Kbd } from "./kbd"

export interface TooltipProps {
  content: JSX.Element
  /** Shortcut keys shown after the text, like `["Ctrl", "K"]`. */
  keys?: string[]
  placement?: "top" | "bottom" | "left" | "right"
  /** On the trigger wrapper, e.g. `flex` or `min-w-0` when it wraps a block. */
  class?: string
  children: JSX.Element
}

export function Tooltip(props: TooltipProps) {
  return (
    <KTooltip placement={props.placement ?? "top"} openDelay={400} closeDelay={0} gutter={6}>
      <KTooltip.Trigger as="span" data-slot="tooltip-trigger" class={props.class}>
        {props.children}
      </KTooltip.Trigger>
      <KTooltip.Portal>
        <KTooltip.Content data-component="tooltip">
          {props.content}
          <Show when={props.keys}>{(keys) => <Kbd keys={keys()} />}</Show>
        </KTooltip.Content>
      </KTooltip.Portal>
    </KTooltip>
  )
}

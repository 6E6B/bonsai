import { Collapsible as KCollapsible } from "@kobalte/core/collapsible"
import type { JSX, ParentProps } from "solid-js"
import { Icon } from "./icon"

export interface CollapsibleProps extends ParentProps {
  title: JSX.Element
  open?: boolean
  defaultOpen?: boolean
  onOpenChange?: (open: boolean) => void
  class?: string
}

/** A titled section that folds away, like a game's About text. */
export function Collapsible(props: CollapsibleProps) {
  return (
    <KCollapsible
      data-component="collapsible"
      class={props.class}
      open={props.open}
      defaultOpen={props.defaultOpen}
      onOpenChange={props.onOpenChange}
    >
      <KCollapsible.Trigger data-slot="trigger">
        <Icon data-slot="chevron" name="chevron-right" size={12} />
        {props.title}
      </KCollapsible.Trigger>
      <KCollapsible.Content data-slot="content">{props.children}</KCollapsible.Content>
    </KCollapsible>
  )
}

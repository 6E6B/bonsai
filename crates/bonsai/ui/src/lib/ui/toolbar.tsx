import { type JSX, Show } from "solid-js"

export interface ToolbarProps {
  /** Filters, search and counts, on the left. */
  children?: JSX.Element
  /** Buttons on the right. */
  actions?: JSX.Element
  class?: string
}

/** The row of controls at the top of a page or pane. Screens have no title (the tab names
 * them), so this is what sits at the top when a screen has controls. */
export function Toolbar(props: ToolbarProps) {
  return (
    <div data-component="toolbar" class={props.class}>
      <div data-slot="start">{props.children}</div>
      <Show when={props.actions}>
        <div data-slot="actions">{props.actions}</div>
      </Show>
    </div>
  )
}

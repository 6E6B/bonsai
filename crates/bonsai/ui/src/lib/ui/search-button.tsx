import { Button as KButton } from "@kobalte/core/button"
import { Show } from "solid-js"
import { Icon } from "./icon"
import { Kbd } from "./kbd"

export interface SearchButtonProps {
  placeholder: string
  /** Shortcut caps, like `["Ctrl", "K"]`. */
  keys?: string[]
  onClick: () => void
  class?: string
}

/** Looks like a search field but opens the command bar, so search has one home. */
export function SearchButton(props: SearchButtonProps) {
  return (
    <KButton data-component="search-button" class={props.class} onClick={props.onClick}>
      <Icon name="search" size={14} />
      <span data-slot="placeholder">{props.placeholder}</span>
      <Show when={props.keys}>{(k) => <Kbd keys={k()} />}</Show>
    </KButton>
  )
}

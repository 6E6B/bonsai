import { type JSX, Show } from "solid-js"

export interface SectionHeaderProps {
  title: string
  /** After the title in muted text, like a count ("2") or "3 online of 14". */
  meta?: JSX.Element
  /** Buttons on the right. */
  actions?: JSX.Element
  /** `group` is the small label above a run of list rows ("In game · 2"). */
  size?: "page" | "section" | "group"
  class?: string
}

/** A heading with optional meta text and actions. One component so every screen's headings
 * share sizes and spacing. */
export function SectionHeader(props: SectionHeaderProps) {
  return (
    <div data-component="section-header" data-size={props.size ?? "section"} class={props.class}>
      <h2 data-slot="title">
        {props.title}
        <Show when={props.meta !== undefined}>
          <span data-slot="meta">{props.meta}</span>
        </Show>
      </h2>
      <Show when={props.actions}>
        <div data-slot="actions">{props.actions}</div>
      </Show>
    </div>
  )
}

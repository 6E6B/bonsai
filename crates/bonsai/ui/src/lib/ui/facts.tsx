import type { JSX, ParentProps } from "solid-js"
import { Show } from "solid-js"

export interface FactsProps extends ParentProps {
  /** Inside a `Card`: the control fill, so it stands out from the card around it. */
  nested?: boolean
  class?: string
}

/** A panel of label/value rows, like an entity's details beside its description. Fill it with
 * `FactGroup`s, or `Fact`s directly when there is only one group. */
export function Facts(props: FactsProps) {
  return (
    <div data-component="facts" data-nested={props.nested ? "" : undefined} class={props.class}>
      {props.children}
    </div>
  )
}

export interface FactGroupProps extends ParentProps {
  /** Small heading above the rows ("Servers"). */
  title?: string
}

/** A titled run of rows inside `Facts`. Groups are split by a line. */
export function FactGroup(props: FactGroupProps) {
  return (
    <section data-slot="group">
      <Show when={props.title}>
        <h3 data-slot="group-title">{props.title}</h3>
      </Show>
      <dl data-slot="rows">{props.children}</dl>
    </section>
  )
}

export interface FactProps {
  label: string
  children: JSX.Element
}

/** One row: muted label on the left, value on the right. */
export function Fact(props: FactProps) {
  return (
    <div data-slot="fact">
      <dt data-slot="label">{props.label}</dt>
      <dd data-slot="value">{props.children}</dd>
    </div>
  )
}

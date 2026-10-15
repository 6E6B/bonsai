import type { JSX } from "solid-js"

export interface StatProps {
  label: string
  children: JSX.Element
  class?: string
}

/** A label over a value, like "Visits / 7.3M". Lay several out in a grid inside a Card. */
export function Stat(props: StatProps) {
  return (
    <div data-component="stat" class={props.class}>
      <dt data-slot="label">{props.label}</dt>
      <dd data-slot="value">{props.children}</dd>
    </div>
  )
}

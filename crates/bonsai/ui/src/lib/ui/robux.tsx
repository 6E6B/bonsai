import { Show } from "solid-js"

export interface RobuxProps {
  /** Amount in Robux. `null` shows a dash (unknown, or not for sale). */
  amount: number | null
  /** `compact` writes 12.4K instead of 12,400. */
  compact?: boolean
  class?: string
}

const full = new Intl.NumberFormat("en-US")
const short = new Intl.NumberFormat("en-US", { notation: "compact", maximumFractionDigits: 1 })

/** A Robux amount with the currency mark. */
export function Robux(props: RobuxProps) {
  return (
    <span data-component="robux" class={props.class}>
      <span data-slot="mark" role="img" aria-label="Robux" />
      <Show when={props.amount !== null} fallback="—">
        {(props.compact ? short : full).format(props.amount ?? 0)}
      </Show>
    </span>
  )
}

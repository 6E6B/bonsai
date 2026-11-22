import { Show } from "solid-js"
import { Redacted } from "./redacted"

export interface RobuxProps {
  /** Amount in Robux. `null` shows a dash (unknown, or not for sale). */
  amount: number | null
  /** `compact` writes 12.4K instead of 12,400. */
  compact?: boolean
  /** `lg` for a headline price: the mark sized to the numerals instead of the full line. */
  size?: "md" | "lg"
  class?: string
  /** A balance or transaction of yours: hidden while streamer mode hides private content. */
  private?: boolean
}

const full = new Intl.NumberFormat("en-US")
const short = new Intl.NumberFormat("en-US", { notation: "compact", maximumFractionDigits: 1 })

/** A Robux amount with the currency mark. */
export function Robux(props: RobuxProps) {
  const amount = () => (
    <Show when={props.amount !== null} fallback="—">
      {(props.compact ? short : full).format(props.amount ?? 0)}
    </Show>
  )
  return (
    <span data-component="robux" data-size={props.size ?? "md"} class={props.class}>
      <span data-slot="mark" role="img" aria-label="Robux" />
      <Show when={props.private} fallback={amount()}>
        <Redacted>{amount()}</Redacted>
      </Show>
    </span>
  )
}

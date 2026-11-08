import { Show } from "solid-js"
import { Tooltip } from "./tooltip"

export interface PremiumBadgeProps {
  /** Height in px; the hard hat is a little wider. Defaults to 14. */
  size?: number
  /** Tooltip text, e.g. the tier name. Defaults to "Premium". */
  label?: string
  /** No tooltip, hidden from screen readers. For use inside a `Badge` that already says Premium. */
  bare?: boolean
  class?: string
}

/** The Builders Club hard hat, after the name of a user with Roblox Premium. */
export function PremiumBadge(props: PremiumBadgeProps) {
  const style = () => ({ "--badge-size": `${props.size ?? 14}px` })
  return (
    <Show
      when={!props.bare}
      fallback={<span data-component="premium-badge" aria-hidden="true" class={props.class} style={style()} />}
    >
      <Tooltip content={props.label ?? "Premium"}>
        <span
          data-component="premium-badge"
          role="img"
          aria-label={props.label ?? "Premium"}
          class={props.class}
          style={style()}
        />
      </Tooltip>
    </Show>
  )
}

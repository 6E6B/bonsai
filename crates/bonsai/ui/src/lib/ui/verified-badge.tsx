import { Tooltip } from "./tooltip"

export interface VerifiedBadgeProps {
  /** Width and height in px. Defaults to 14. */
  size?: number
  class?: string
}

/** Roblox's verified checkmark, after a verified user's or community's name. The name itself
 * keeps its normal colour. */
export function VerifiedBadge(props: VerifiedBadgeProps) {
  return (
    <Tooltip content="Verified">
      <span
        data-component="verified-badge"
        role="img"
        aria-label="Verified"
        class={props.class}
        style={{ "--badge-size": `${props.size ?? 14}px` }}
      />
    </Tooltip>
  )
}

import { Show } from "solid-js"
import plus from "./assets/plus-badge.svg?raw"
import { Tooltip } from "./tooltip"

/** The logo's outline, drawn in the text color. Inline rather than a CSS mask: a mask makes
 * WebKitGTK repaint the whole shelf row around it on every scroll frame. */
const outline = plus.replace(/^<svg[^>]*>|<\/svg>\s*$/g, "").replace('stroke="#000"', 'stroke="currentColor"')

export interface PremiumBadgeProps {
  /** Height in px, matching `VerifiedBadge` at the same size. Defaults to 14. */
  size?: number
  /** Tooltip text, e.g. the tier name. Defaults to "Premium". */
  label?: string
  /** No tooltip, hidden from screen readers. For use inside a `Badge` that already says Premium. */
  bare?: boolean
  class?: string
}

/** The Roblox Plus logo, after the name of a user with Premium or Roblox Plus. */
export function PremiumBadge(props: PremiumBadgeProps) {
  const style = () => ({ "--badge-size": `${props.size ?? 14}px` })
  return (
    <Show when={!props.bare} fallback={<Logo class={props.class} style={style()} />}>
      <Tooltip content={props.label ?? "Premium"}>
        <Logo class={props.class} style={style()} label={props.label ?? "Premium"} />
      </Tooltip>
    </Show>
  )
}

function Logo(props: { class?: string; style: Record<string, string>; label?: string }) {
  return (
    <svg
      data-component="premium-badge"
      viewBox="0 0 349.78009 390.33646"
      fill="none"
      role={props.label ? "img" : undefined}
      aria-label={props.label}
      aria-hidden={props.label ? undefined : "true"}
      class={props.class}
      style={props.style}
      innerHTML={outline}
    />
  )
}

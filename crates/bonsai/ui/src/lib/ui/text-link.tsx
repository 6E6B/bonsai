import { Button as KButton } from "@kobalte/core/button"
import type { ParentProps } from "solid-js"

export interface TextLinkProps extends ParentProps {
  /** Run a command here (usually a `nav.*` or `*.open` one). */
  onClick: () => void
  /** `accent` for links that are not names. User and community names use `name` (or
   * `muted` for secondary text like "by Chiidoku"), or `verified` when Roblox has verified
   * them: only verified names are blue, to match the `VerifiedBadge`. `danger` marks a name
   * that is pretending to be someone it isn't. `inherit` keeps the surrounding text colour and
   * underlines, for links inside a coloured bubble. */
  tone?: "accent" | "name" | "muted" | "verified" | "danger" | "inherit"
  class?: string
}

/** Inline clickable text that opens something in the app. It is a button, not an anchor:
 * nothing in the app navigates by URL. */
export function TextLink(props: TextLinkProps) {
  return (
    <KButton data-component="text-link" data-tone={props.tone ?? "accent"} class={props.class} onClick={props.onClick}>
      {props.children}
    </KButton>
  )
}

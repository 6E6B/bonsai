import { Button as KButton } from "@kobalte/core/button"
import { Show } from "solid-js"
import { Avatar } from "./avatar"

export interface MentionProps {
  /** The text as written, with its `@`. */
  text: string
  /** Username, for the picture's initials and alt text. */
  name: string
  /** Headshot. Shown before the name. */
  src?: string
  /** Shown as a tooltip: usually the display name. */
  title?: string
  /** Verified users are blue, like `TextLink tone="verified"`. */
  verified?: boolean
  /** Opens the profile. Without it the mention is highlighted but not clickable, e.g. while the
   * name is being looked up. */
  onClick?: () => void
}

/** An @mention in a description or message: a small headshot and the name, highlighted. */
export function Mention(props: MentionProps) {
  return (
    <Show
      when={props.onClick}
      fallback={
        <span data-component="mention" data-pending="">
          {props.text}
        </span>
      }
    >
      {(onClick) => (
        <KButton
          data-component="mention"
          data-verified={props.verified ? "" : undefined}
          title={props.title}
          onClick={onClick()}
        >
          <Avatar data-slot="avatar" size="xs" name={props.name} src={props.src} />
          {props.text}
        </KButton>
      )}
    </Show>
  )
}

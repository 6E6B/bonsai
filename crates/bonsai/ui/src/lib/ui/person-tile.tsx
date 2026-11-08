import { Button as KButton } from "@kobalte/core/button"
import { Show } from "solid-js"
import { Avatar, type Presence } from "./avatar"
import { VerifiedBadge } from "./verified-badge"

export interface PersonTileProps {
  /** Display name. */
  name: string
  /** Username, shown under the name with an @. */
  username?: string
  /** A `thumbUrl("headshot", id, true)` for the large avatar. */
  src?: string
  presence?: Presence
  verified?: boolean
  /** A third line under the username, such as the game they're in. */
  status?: string
  onClick: () => void
  class?: string
}

/** A person as a tile: a large round headshot with their display name and @username under it,
 * like the friends row on a roblox.com profile, and an optional status line (the game they're
 * in) under that. Put it in a `Shelf` or a grid. */
export function PersonTile(props: PersonTileProps) {
  return (
    <KButton data-component="person-tile" class={props.class} onClick={props.onClick}>
      <Avatar size="xl" name={props.name} src={props.src} presence={props.presence} />
      <span data-slot="name">
        <span data-slot="name-text" title={props.name}>
          {props.name}
        </span>
        <Show when={props.verified}>
          <VerifiedBadge />
        </Show>
      </span>
      <Show when={props.username}>
        <span data-slot="username" title={`@${props.username}`}>
          @{props.username}
        </span>
      </Show>
      <Show when={props.status}>
        <span data-slot="status" title={props.status}>
          {props.status}
        </span>
      </Show>
    </KButton>
  )
}

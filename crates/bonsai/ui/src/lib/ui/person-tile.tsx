import { Button as KButton } from "@kobalte/core/button"
import { Show } from "solid-js"
import { Avatar, type Presence } from "./avatar"
import { VerifiedBadge } from "./verified-badge"

export interface PersonTileProps {
  /** Display name. */
  name: string
  /** Username, shown under the name with an @. */
  username?: string
  /** A `thumbUrl("headshot", id)`. */
  src?: string
  presence?: Presence
  verified?: boolean
  onClick: () => void
  class?: string
}

/** A person as a tile: a large round headshot with their display name and @username under it,
 * like the friends row on a roblox.com profile. Put it in a `Shelf` or a grid. */
export function PersonTile(props: PersonTileProps) {
  return (
    <KButton data-component="person-tile" class={props.class} onClick={props.onClick}>
      <Avatar size="xl" name={props.name} src={props.src} presence={props.presence} />
      <span data-slot="name" data-verified={props.verified ? "" : undefined}>
        <span data-slot="name-text">{props.name}</span>
        <Show when={props.verified}>
          <VerifiedBadge />
        </Show>
      </span>
      <Show when={props.username}>
        <span data-slot="username">@{props.username}</span>
      </Show>
    </KButton>
  )
}

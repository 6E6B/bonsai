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
  /** Other people with them, like friends in the same game. The first peeks out from behind
   * the picture, so the tile reads as a stack. */
  stack?: { name: string; src?: string }[]
  onClick: () => void
  class?: string
}

/** A person as a tile: a large round headshot with their display name and @username under it,
 * like the friends row on a roblox.com profile, and an optional status line (the game they're
 * in) under that. Put it in a `Shelf` or a grid. */
export function PersonTile(props: PersonTileProps) {
  return (
    <KButton data-component="person-tile" class={props.class} onClick={props.onClick}>
      <span data-slot="picture" data-stacked={props.stack?.length ? "" : undefined}>
        <Show when={props.stack?.[0]}>
          {(p) => <Avatar data-slot="behind" size="xl" name={p().name} src={p().src} />}
        </Show>
        <Avatar size="xl" name={props.name} src={props.src} presence={props.presence} />
      </span>
      <span data-slot="name" data-verified={props.verified ? "" : undefined}>
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

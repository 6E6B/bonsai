import { Button as KButton } from "@kobalte/core/button"
import { Show } from "solid-js"
import { Avatar, type Presence } from "./avatar"
import { AvatarStack } from "./avatar-stack"
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
  /** Other people with them, like friends in the same game. They show as a row of small faces
   * under the status line, named on hover. */
  stack?: { name: string; src?: string }[]
  onClick: () => void
  class?: string
}

/** A person as a tile: a large round headshot with their display name and @username under it,
 * like the friends row on a roblox.com profile, and an optional status line (the game they're
 * in) under that. Put it in a `Shelf` or a grid. */
export function PersonTile(props: PersonTileProps) {
  const stack = () => props.stack ?? []
  return (
    <KButton data-component="person-tile" class={props.class} onClick={props.onClick}>
      <Avatar size="xl" name={props.name} src={props.src} presence={props.presence} />
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
      <Show when={stack().length > 0}>
        <span data-slot="stack">
          <AvatarStack
            size="sm"
            on="canvas"
            max={4}
            items={stack().map((p) => ({ name: p.name, src: p.src, label: p.name }))}
          />
        </span>
      </Show>
    </KButton>
  )
}

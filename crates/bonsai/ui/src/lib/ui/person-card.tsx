import { Button as KButton } from "@kobalte/core/button"
import { type JSX, Show } from "solid-js"
import { Avatar, type Presence } from "./avatar"
import { Card } from "./card"

export interface PersonCardProps {
  name: string
  /** A `thumbUrl("headshot", id)`. */
  src?: string
  presence?: Presence
  /** One muted line: the game they are in, or their status. */
  subtitle?: string
  /** Opens the person. */
  onClick: () => void
  /** A small trailing action, like a ghost play `IconButton`. */
  action?: JSX.Element
  class?: string
}

/** A compact card for a person, laid out in a grid: friends playing, people in a server.
 * Built on `Card`, so it shares its surface, radius and elevation. */
export function PersonCard(props: PersonCardProps) {
  return (
    <Card class={props.class}>
      <div data-component="person-card">
        <KButton data-slot="main" onClick={props.onClick}>
          <Avatar size="lg" name={props.name} src={props.src} presence={props.presence} />
          <span data-slot="text">
            <span data-slot="title">{props.name}</span>
            <Show when={props.subtitle}>
              <span data-slot="subtitle">{props.subtitle}</span>
            </Show>
          </span>
        </KButton>
        <Show when={props.action}>
          <span data-slot="action">{props.action}</span>
        </Show>
      </div>
    </Card>
  )
}

import { createMemo, For, Show } from "solid-js"
import { Avatar } from "./avatar"
import { Tooltip } from "./tooltip"

export interface AvatarStackItem {
  /** Initials fallback and alt text. */
  name: string
  src?: string
  /** Hover text, like a friend's name. None for people we only have a picture of. */
  label?: string
  /** Ringed in the accent color, like friends among strangers. */
  highlight?: boolean
}

export interface AvatarStackProps {
  items: readonly AvatarStackItem[]
  /** At most this many faces show. Default 8. */
  max?: number
  /** How many people there are in all, when more than `items`. The rest show as "+n". */
  total?: number
  size?: "xs" | "sm" | "md"
  /** The background it sits on, which rings each face. Default `surface` (cards, tables). */
  on?: "surface" | "canvas"
  class?: string
}

/** Overlapping faces for a group of people, like the players in a server, with a count of
 * the ones not shown. */
export function AvatarStack(props: AvatarStackProps) {
  const shown = createMemo(() => props.items.slice(0, props.max ?? 8))
  const rest = () => Math.max(props.total ?? props.items.length, props.items.length) - shown().length
  return (
    <span
      data-component="avatar-stack"
      data-size={props.size ?? "sm"}
      data-on={props.on ?? "surface"}
      class={props.class}
    >
      <For each={shown()}>
        {(p) => (
          <Show
            when={p.label}
            fallback={
              <Avatar
                data-slot="face"
                data-highlight={p.highlight || undefined}
                size={props.size ?? "sm"}
                name={p.name}
                src={p.src}
              />
            }
          >
            {(label) => (
              <Tooltip content={label()}>
                <Avatar
                  data-slot="face"
                  data-highlight={p.highlight || undefined}
                  size={props.size ?? "sm"}
                  name={p.name}
                  src={p.src}
                />
              </Tooltip>
            )}
          </Show>
        )}
      </For>
      <Show when={rest() > 0}>
        <span data-slot="more">+{rest()}</span>
      </Show>
    </span>
  )
}

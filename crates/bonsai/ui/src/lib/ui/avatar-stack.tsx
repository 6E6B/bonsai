import { createMemo, createSignal, Index, onCleanup, onMount, Show } from "solid-js"
import { Avatar } from "./avatar"
import { Tooltip } from "./tooltip"

export interface AvatarStackItem {
  /** Initials fallback and alt text. Empty for a plain circle while the picture loads. */
  name: string
  src?: string
  /** Hover text, like a friend's name. None for people we only have a picture of. */
  label?: string
  /** Ringed in the accent color, like friends among strangers. */
  highlight?: boolean
}

export interface AvatarStackProps {
  items: readonly AvatarStackItem[]
  /** At most this many faces show. Without it, the stack fills its width and shows as many
   * faces as fit. */
  max?: number
  /** How many people there are in all, when more than `items`. The rest show as "+n". */
  total?: number
  size?: "xs" | "sm" | "md"
  /** The background it sits on, which rings each face. Default `surface` (cards, tables). */
  on?: "surface" | "canvas"
  class?: string
}

/** Room kept for the "+n" chip, in face sizes. */
const CHIP = 1.5

/** How many faces of `size` px fit in `width` px when each overlaps the last by a third,
 * keeping room for the "+n" chip unless all `count` faces fit and nobody is left over. */
export function facesThatFit(width: number, size: number, count: number, total: number) {
  const step = (size * 2) / 3
  const facesIn = (room: number) => (room < size ? 0 : Math.floor((room - size) / step) + 1)
  if (total <= count && facesIn(width) >= count) return count
  return Math.min(count, facesIn(width - (CHIP * size - size / 3)))
}

/** Overlapping faces for a group of people, like the players in a server, with a count of
 * the ones not shown. */
export function AvatarStack(props: AvatarStackProps) {
  let el: HTMLSpanElement | undefined
  const [width, setWidth] = createSignal<number>()
  const fits = () => props.max === undefined
  onMount(() => {
    if (!fits() || !el) return
    const stack = el
    const measure = () => setWidth(stack.clientWidth)
    measure()
    const observer = new ResizeObserver(measure)
    observer.observe(stack)
    onCleanup(() => observer.disconnect())
  })
  const total = () => Math.max(props.total ?? props.items.length, props.items.length)
  const shown = createMemo(() => {
    if (props.max !== undefined) return props.items.slice(0, props.max)
    const w = width()
    if (w === undefined || !el) return []
    const size = Number.parseFloat(getComputedStyle(el).getPropertyValue("--avatar-stack-size")) || 20
    return props.items.slice(0, facesThatFit(w, size, props.items.length, total()))
  })
  const rest = () => total() - shown().length
  return (
    <span
      ref={el}
      data-component="avatar-stack"
      data-size={props.size ?? "sm"}
      data-on={props.on ?? "surface"}
      data-fit={fits() || undefined}
      class={props.class}
    >
      {/* By position, not by object: callers often build the items afresh, and recreating a face
          makes its picture load again. */}
      <Index each={shown()}>
        {(p) => (
          <Show
            when={p().label}
            fallback={
              <Avatar
                data-slot="face"
                data-highlight={p().highlight || undefined}
                size={props.size ?? "sm"}
                name={p().name}
                src={p().src}
              />
            }
          >
            {(label) => (
              <Tooltip content={label()}>
                <Avatar
                  data-slot="face"
                  data-highlight={p().highlight || undefined}
                  size={props.size ?? "sm"}
                  name={p().name}
                  src={p().src}
                />
              </Tooltip>
            )}
          </Show>
        )}
      </Index>
      <Show when={rest() > 0}>
        <span data-slot="more">+{rest()}</span>
      </Show>
    </span>
  )
}

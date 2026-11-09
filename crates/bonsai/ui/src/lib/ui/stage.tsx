import type { JSX, ParentProps } from "solid-js"
import { Show } from "solid-js"

export interface StageProps extends ParentProps {
  /** Controls in the top-left corner, like a view switch. */
  topStart?: JSX.Element
  /** Controls in the top-right corner, like reset and the viewer's menu. */
  topEnd?: JSX.Element
  /** A short tip in the bottom-left corner, shown while the pointer or focus is on the stage. */
  hint?: string
  /** Give it a size: a height, or a width and an aspect ratio. */
  class?: string
}

/**
 * A wide picture frame for the subject of a page: an item in 3D or its picture. It has the same
 * look as `Viewport` and `Thumbnail`. Put one of those inside; it fills the frame and the
 * picture is never cropped. Controls go in the top corners, with a hint at the bottom left.
 */
export function Stage(props: StageProps) {
  return (
    <div data-component="stage" class={props.class}>
      {props.children}
      <Show when={props.topStart}>
        <div data-slot="overlay" data-edge="top-start">
          {props.topStart}
        </div>
      </Show>
      <Show when={props.topEnd}>
        <div data-slot="overlay" data-edge="top-end">
          {props.topEnd}
        </div>
      </Show>
      <Show when={props.hint}>
        <span data-slot="hint" aria-hidden="true">
          {props.hint}
        </span>
      </Show>
    </div>
  )
}

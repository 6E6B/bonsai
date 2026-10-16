import { type JSX, Show } from "solid-js"
import { Icon, type IconName } from "./icon"

export interface MediaFrameProps {
  /** The image, usually a wide `Thumbnail`. */
  children: JSX.Element
  /** Zero-based index of the shown item. */
  index: number
  count: number
  /** Called with -1 or 1. Arrows and the counter only show when `count > 1`. */
  onStep: (by: number) => void
  /** Short text in the bottom-left corner, like a video title. */
  caption?: string
  captionIcon?: IconName
  class?: string
}

/** An image with previous and next arrows and a counter laid over it, so a gallery needs no
 * extra row. Arrows show on hover or keyboard focus. */
export function MediaFrame(props: MediaFrameProps) {
  return (
    <div data-component="media-frame" class={props.class}>
      {props.children}
      <Show when={props.caption}>
        <span data-slot="caption">
          <Show when={props.captionIcon}>{(icon) => <Icon name={icon()} size={12} />}</Show>
          <span data-slot="caption-text">{props.caption}</span>
        </span>
      </Show>
      <Show when={props.count > 1}>
        <span data-slot="count">
          {props.index + 1} / {props.count}
        </span>
        <button type="button" data-slot="prev" aria-label="Previous image" onClick={() => props.onStep(-1)}>
          <Icon name="chevron-left" size={16} />
        </button>
        <button type="button" data-slot="next" aria-label="Next image" onClick={() => props.onStep(1)}>
          <Icon name="chevron-right" size={16} />
        </button>
      </Show>
    </div>
  )
}

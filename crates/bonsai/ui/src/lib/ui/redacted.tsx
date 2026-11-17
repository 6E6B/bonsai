import { createSignal, type JSX, Show } from "solid-js"

const [redacting, setRedactingSignal] = createSignal(false)

/** Whether private content is hidden right now (streamer mode). */
export { redacting }

/** Turns hiding of private content on or off for every `Redacted` and private `Robux`. */
export function setRedacting(on: boolean) {
  setRedactingSignal(on)
}

/** `text` while nothing is hidden, otherwise `undefined`, for subtitles and other optional text. */
export function redactText<T>(text: T): T | undefined {
  return redacting() ? undefined : text
}

export interface RedactedProps {
  /** What to hide. */
  children: JSX.Element
  /** `false` leaves it shown even in streamer mode. Defaults to `true`. */
  when?: boolean
  class?: string
}

/** Private content, blurred while streamer mode hides it and shown again while hovered. Use
 * `redactText` instead for text that should disappear entirely. */
export function Redacted(props: RedactedProps) {
  return (
    <Show when={redacting() && props.when !== false} fallback={props.children}>
      <span data-component="redacted" class={props.class}>
        {props.children}
      </span>
    </Show>
  )
}

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
  /** Shown in its place. Defaults to a short row of dots. */
  fallback?: JSX.Element
}

/** Private content, replaced by a placeholder while streamer mode hides it. */
export function Redacted(props: RedactedProps) {
  return (
    <Show
      when={!redacting()}
      fallback={
        <span data-component="redacted" role="img" aria-label="Hidden">
          {props.fallback ?? "•••"}
        </span>
      }
    >
      {props.children}
    </Show>
  )
}

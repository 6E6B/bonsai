import { Show } from "solid-js"
import type { BackgroundView } from "~/lib/ipc/bindings"
import { backgroundUrl } from "~/lib/thumb"

/** The background picture behind the page area. The page and its cards are see-through while
 * one is set (`setSeeThrough` in `~/lib/theme`), so it shows; GIFs play. Sits under its
 * siblings in a parent with `isolate`. */
export function Backdrop(props: { background: BackgroundView | null }) {
  return (
    <Show when={props.background}>
      {(b) => (
        <div
          aria-hidden="true"
          class="pointer-events-none absolute inset-0 -z-10 overflow-hidden"
          style={{ "background-color": "var(--backdrop-base)" }}
        >
          <Show when={backgroundUrl(b().file)}>
            {(src) => (
              <img
                src={src()}
                alt=""
                draggable={false}
                class="size-full object-cover"
                style={{ opacity: b().image / 100 }}
              />
            )}
          </Show>
        </div>
      )}
    </Show>
  )
}

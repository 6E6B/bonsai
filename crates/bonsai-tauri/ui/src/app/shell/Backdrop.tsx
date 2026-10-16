import { createMemo, Show } from "solid-js"
import type { BackgroundView } from "~/lib/ipc/bindings"
import { backdropLayers, withDraft } from "~/lib/theme"
import { backgroundUrl } from "~/lib/thumb"

/** The background picture behind the page area, with its effects. The page and its cards are
 * see-through while one is set (`setSeeThrough` in `~/lib/theme`), so it shows; GIFs play. Sits
 * under its siblings in a parent with `isolate`. Overlays fade toward the theme's page color, so
 * they suit light and dark themes alike. */
export function Backdrop(props: { background: BackgroundView | null }) {
  return (
    <Show when={props.background}>
      {(b) => {
        const l = createMemo(() => backdropLayers(withDraft(b())))
        return (
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
                  class="absolute max-w-none object-cover"
                  style={{
                    top: `-${l().overhang}px`,
                    left: `-${l().overhang}px`,
                    width: `calc(100% + ${l().overhang * 2}px)`,
                    height: `calc(100% + ${l().overhang * 2}px)`,
                    opacity: l().image,
                    filter: l().filter,
                  }}
                />
              )}
            </Show>
            <Show when={l().tint > 0}>
              <div
                class="absolute inset-0"
                style={{ "background-color": "var(--color-accent)", "mix-blend-mode": "color", opacity: l().tint }}
              />
            </Show>
            <Show when={l().grain > 0}>
              <svg
                aria-hidden="true"
                class="absolute inset-0 size-full"
                style={{ "mix-blend-mode": "overlay", opacity: l().grain }}
              >
                <filter id="backdrop-grain">
                  <feTurbulence type="fractalNoise" baseFrequency="0.85" numOctaves="2" stitchTiles="stitch" />
                  <feColorMatrix type="saturate" values="0" />
                </filter>
                <rect width="100%" height="100%" filter="url(#backdrop-grain)" />
              </svg>
            </Show>
            <Show when={l().scanlines > 0}>
              <div
                class="absolute inset-0"
                style={{
                  "background-image":
                    "repeating-linear-gradient(to bottom, transparent 0 2px, var(--backdrop-base) 2px 3px)",
                  opacity: l().scanlines,
                }}
              />
            </Show>
            <Show when={l().vignette > 0}>
              <div
                class="absolute inset-0"
                style={{
                  "background-image": "radial-gradient(ellipse at center, transparent 35%, var(--backdrop-base) 100%)",
                  opacity: l().vignette,
                }}
              />
            </Show>
          </div>
        )
      }}
    </Show>
  )
}

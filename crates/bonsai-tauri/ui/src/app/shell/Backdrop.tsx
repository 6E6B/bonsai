import { createEffect, createMemo, createSignal, onCleanup, Show } from "solid-js"
import type { BackgroundView } from "~/lib/ipc/bindings"
import { backdropLayers, cutoutPath, withDraft } from "~/lib/theme"
import { backgroundUrl } from "~/lib/thumb"

/** The background picture behind the page area, with its effects. The page and its cards are
 * see-through while one is set (`setSeeThrough` in `~/lib/theme`), so it shows; GIFs play. Sits
 * under its siblings in a parent with `isolate`. Overlays fade toward the theme's page color, so
 * they suit light and dark themes alike. With `strip`, the tab strip at the top of the parent, the
 * picture is cut to the page and the active tab, so the tab reads as part of the page. */
export function Backdrop(props: { background: BackgroundView | null; strip?: HTMLElement }) {
  return (
    <Show when={props.background}>
      {(b) => {
        const l = createMemo(() => backdropLayers(withDraft(b())))
        const [box, setBox] = createSignal<HTMLDivElement>()
        const clip = stripCutout(box, () => props.strip)
        return (
          <div
            ref={setBox}
            aria-hidden="true"
            class="pointer-events-none absolute inset-0 -z-10 overflow-hidden"
            style={{ "background-color": "var(--backdrop-base)", "clip-path": clip() }}
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

/** The `clip-path` that cuts `box` to below `strip` plus its active tab. Remeasures when either
 * resizes, the strip scrolls, or tabs open, close, switch or get dragged. */
function stripCutout(box: () => HTMLElement | undefined, strip: () => HTMLElement | undefined) {
  const [clip, setClip] = createSignal<string>()
  createEffect(() => {
    const b = box()
    const s = strip()
    if (!b || !s) return setClip(undefined)
    let frame = 0
    const measure = () => {
      frame = 0
      const r = b.getBoundingClientRect()
      // Rects are zoomed with the root's CSS zoom; the path is in the box's own pixels.
      const scale = r.width / b.offsetWidth || 1
      const y = (v: number) => (v - r.top) / scale
      const x = (v: number) => (v - r.left) / scale
      const tab = s.querySelector<HTMLElement>('[data-slot="tab"][data-active]')
      const list = s.querySelector<HTMLElement>('[data-slot="list"]')
      const top = y(s.getBoundingClientRect().bottom)
      if (!tab || !list) return setClip(cutoutPath(b.offsetWidth, b.offsetHeight, top))
      const t = tab.getBoundingClientRect()
      const v = list.getBoundingClientRect()
      setClip(
        cutoutPath(b.offsetWidth, b.offsetHeight, top, {
          left: x(Math.max(t.left, v.left)),
          right: x(Math.min(t.right, v.right)),
          top: y(t.top),
          radius: Number.parseFloat(getComputedStyle(tab).borderTopLeftRadius) || 0,
        }),
      )
    }
    const later = () => {
      if (!frame) frame = requestAnimationFrame(measure)
    }
    const resize = new ResizeObserver(later)
    resize.observe(b)
    resize.observe(s)
    const changes = new MutationObserver(later)
    changes.observe(s, { subtree: true, childList: true, attributeFilter: ["data-active", "style"] })
    s.addEventListener("scroll", later, { capture: true, passive: true })
    measure()
    onCleanup(() => {
      cancelAnimationFrame(frame)
      resize.disconnect()
      changes.disconnect()
      s.removeEventListener("scroll", later, { capture: true })
    })
  })
  return clip
}

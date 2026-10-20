import { createEffect, createMemo, createSignal, onCleanup, onMount, Show } from "solid-js"
import type { BackgroundView } from "~/lib/ipc/bindings"
import { backdropLayers, cutoutPath, withDraft } from "~/lib/theme"
import { backgroundUrl } from "~/lib/thumb"

/** The background picture behind the page area, with its effects. The page and its cards are
 * see-through while one is set (`setSeeThrough` in `~/lib/theme`), so it shows; GIFs play. Sits
 * under its siblings in a parent with `isolate`. Overlays fade toward the theme's page color, so
 * they suit light and dark themes alike. With `strip`, the tab strip at the top of the parent, the
 * picture is cut to the page and the active tab, so the tab reads as part of the page. */
export function Backdrop(props: { background: BackgroundView | null; strip?: HTMLElement }) {
  // Keep the image resource separate from effect changes and store refreshes. A stable
  // image node and src let the webview keep one animation timeline while editing.
  const src = createMemo(() => backgroundUrl(props.background?.file ?? ""))
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
            data-window-corner="bottom-right"
            class="pointer-events-none absolute inset-0 -z-10 overflow-hidden"
            style={{ "background-color": "var(--backdrop-base)", "clip-path": clip() }}
          >
            <Show when={src()}>
              {(imageSrc) => (
                <img
                  src={imageSrc()}
                  alt=""
                  draggable={false}
                  decoding="async"
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
              <Scanlines opacity={l().scanlines} />
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

/** Alternating dark and clear rows, each one CSS pixel tall. Drawn on a canvas at device
 * resolution: a CSS gradient this fine is resampled under fractional zoom or display scale and
 * turns into wide bands. Dark rows use `--backdrop-base`, so light themes get light rows. */
function Scanlines(props: { opacity: number }) {
  let canvas!: HTMLCanvasElement
  onMount(() => {
    let frame = 0
    const draw = () => {
      frame = 0
      const r = canvas.getBoundingClientRect()
      // Device pixels per CSS pixel, including the root's CSS zoom.
      const scale = (r.height / canvas.offsetHeight || 1) * window.devicePixelRatio
      const w = Math.round(canvas.offsetWidth * scale)
      const h = Math.round(canvas.offsetHeight * scale)
      if (!w || !h) return
      canvas.width = w
      canvas.height = h
      const ctx = canvas.getContext("2d")
      if (!ctx) return
      ctx.fillStyle = getComputedStyle(canvas).getPropertyValue("--backdrop-base").trim() || "#000"
      const row = Math.max(1, Math.round(scale))
      for (let y = 0; y < h; y += row * 2) ctx.fillRect(0, y, w, row)
    }
    const later = () => {
      if (!frame) frame = requestAnimationFrame(draw)
    }
    const resize = new ResizeObserver(later)
    resize.observe(canvas)
    // Display scale and root zoom changes don't resize the box in CSS pixels.
    const zoom = new MutationObserver(later)
    zoom.observe(document.documentElement, { attributeFilter: ["style"] })
    window.addEventListener("resize", later)
    draw()
    onCleanup(() => {
      cancelAnimationFrame(frame)
      resize.disconnect()
      zoom.disconnect()
      window.removeEventListener("resize", later)
    })
  })
  return <canvas ref={canvas} class="absolute inset-0 size-full" style={{ opacity: props.opacity }} />
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

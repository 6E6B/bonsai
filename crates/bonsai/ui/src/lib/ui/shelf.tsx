import { createSignal, type JSX, onCleanup, onMount, Show } from "solid-js"
import { Icon } from "./icon"

export interface ShelfProps {
  /** Each child is one entry, kept at `itemWidth`. */
  children: JSX.Element
  /** Width of every entry, as CSS. Square tiles' arrows line up with the middle of the image. */
  itemWidth: string
  class?: string
}

/** One row of tiles or people that scrolls sideways, like the rows on a roblox.com profile.
 * An edge fades out while there is more that way, and an arrow shows there on hover. Shift+wheel
 * and the keyboard scroll it too. */
export function Shelf(props: ShelfProps) {
  let track!: HTMLDivElement
  const [prev, setPrev] = createSignal(false)
  const [next, setNext] = createSignal(false)
  const measure = () => {
    setPrev(track.scrollLeft > 1)
    setNext(track.scrollLeft + track.clientWidth < track.scrollWidth - 1)
  }
  onMount(() => {
    const ro = new ResizeObserver(measure)
    ro.observe(track)
    const mo = new MutationObserver(measure)
    mo.observe(track, { childList: true })
    onCleanup(() => {
      ro.disconnect()
      mo.disconnect()
    })
  })
  const step = (by: number) => track.scrollBy({ left: by * track.clientWidth * 0.8, behavior: "smooth" })
  return (
    <div
      data-component="shelf"
      data-prev={prev() ? "" : undefined}
      data-next={next() ? "" : undefined}
      class={props.class}
      style={{ "--shelf-item": props.itemWidth }}
    >
      <div data-slot="track" ref={track} onScroll={measure}>
        {props.children}
      </div>
      <Show when={prev()}>
        <button type="button" data-slot="prev" aria-label="Scroll left" tabIndex={-1} onClick={() => step(-1)}>
          <Icon name="chevron-left" size={16} />
        </button>
      </Show>
      <Show when={next()}>
        <button type="button" data-slot="next" aria-label="Scroll right" tabIndex={-1} onClick={() => step(1)}>
          <Icon name="chevron-right" size={16} />
        </button>
      </Show>
    </div>
  )
}

import { createSignal, onCleanup, onMount, type ParentProps, Show } from "solid-js"

export interface ClampProps extends ParentProps {
  /** Lines shown before "Show more". */
  lines?: number
  class?: string
}

/** Long text cut to a few lines, with a "Show more" toggle that only appears when something
 * was cut. */
export function Clamp(props: ClampProps) {
  let body!: HTMLDivElement
  const [open, setOpen] = createSignal(false)
  const [cut, setCut] = createSignal(false)
  onMount(() => {
    const check = () => setCut(open() || body.scrollHeight > body.clientHeight + 1)
    const ro = new ResizeObserver(check)
    ro.observe(body)
    onCleanup(() => ro.disconnect())
  })
  return (
    <div data-component="clamp" class={props.class}>
      <div
        ref={body}
        data-slot="body"
        data-open={open() ? "" : undefined}
        data-cut={cut() ? "" : undefined}
        style={{ "--clamp-lines": String(props.lines ?? 6) }}
      >
        {props.children}
      </div>
      <Show when={cut()}>
        <button type="button" data-slot="toggle" onClick={() => setOpen((o) => !o)}>
          {open() ? "Show less" : "Show more"}
        </button>
      </Show>
    </div>
  )
}

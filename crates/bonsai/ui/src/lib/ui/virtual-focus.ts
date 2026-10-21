import { onCleanup, onMount } from "solid-js"

/** Keep focused controls mounted and let Tab cross a virtual window's boundary. */
export function virtualFocus(options: {
  root: () => HTMLElement | undefined
  count: () => number
  scroll: (index: number) => void
  pin: (index: number | null) => void
}) {
  let frame = 0
  const selector =
    'button:not([disabled]), a[href], input:not([disabled]), textarea:not([disabled]), select:not([disabled]), [tabindex="0"]'
  const controls = (item: Element) =>
    [...(item.matches(selector) ? [item as HTMLElement] : []), ...item.querySelectorAll<HTMLElement>(selector)].filter(
      (el) => el.getClientRects().length > 0,
    )
  const itemAt = (target: EventTarget | null) => {
    let item = (target as HTMLElement | null)?.closest<HTMLElement>("[data-virtual-item]")
    while (item && item.closest("[data-virtual-root]") !== options.root()) {
      item = item.parentElement?.closest<HTMLElement>("[data-virtual-item]")
    }
    return item
  }
  const handlers = {
    focusIn(event: FocusEvent) {
      const item = itemAt(event.target)
      options.pin(item ? Number(item.dataset.virtualItem) : null)
    },
    focusOut() {
      queueMicrotask(() => {
        if (!options.root()?.contains(document.activeElement)) options.pin(null)
      })
    },
    keyDown(event: KeyboardEvent) {
      if (event.defaultPrevented || event.key !== "Tab" || event.altKey || event.ctrlKey || event.metaKey) return
      const item = itemAt(event.target)
      if (!item) return
      const entries = controls(item)
      if (event.target !== entries[event.shiftKey ? 0 : entries.length - 1]) return
      const next = Number(item.dataset.virtualItem) + (event.shiftKey ? -1 : 1)
      if (next < 0 || next >= options.count()) return
      const root = options.root()
      const mountedNext = root?.querySelector(`[data-virtual-item="${next}"]`)
      if (mountedNext && controls(mountedNext).length === 0) return
      event.preventDefault()
      options.scroll(next)
      cancelAnimationFrame(frame)
      // Most neighbors are already mounted by overscan. Focus them synchronously so
      // another Tab before the next frame advances from the new control.
      const target = root?.querySelector(`[data-virtual-item="${next}"]`)
      const nextEntries = target ? controls(target) : []
      const nextControl = nextEntries[event.shiftKey ? nextEntries.length - 1 : 0]
      if (nextControl) {
        nextControl.focus({ preventScroll: true })
        return
      }
      frame = requestAnimationFrame(() => {
        if (!options.root()?.isConnected) return
        const target = options.root()?.querySelector(`[data-virtual-item="${next}"]`)
        const entries = target ? controls(target) : []
        const nextControl = entries[event.shiftKey ? entries.length - 1 : 0]
        if (nextControl) nextControl.focus({ preventScroll: true })
        else {
          // A newly mounted read-only row must not trap Tab. Leave through the next control.
          const root = options.root()
          if (!root) return
          const outside = [...document.querySelectorAll<HTMLElement>(selector)].filter(
            (element) => !root.contains(element) && element.getClientRects().length > 0,
          )
          const direction = event.shiftKey ? Node.DOCUMENT_POSITION_PRECEDING : Node.DOCUMENT_POSITION_FOLLOWING
          const candidates = outside.filter((element) => root.compareDocumentPosition(element) & direction)
          const next = candidates[event.shiftKey ? candidates.length - 1 : 0]
          next?.focus()
        }
      })
    },
  }
  onMount(() => {
    const root = options.root()
    if (!root) return
    root.addEventListener("focusin", handlers.focusIn)
    root.addEventListener("focusout", handlers.focusOut)
    root.addEventListener("keydown", handlers.keyDown)
    onCleanup(() => {
      cancelAnimationFrame(frame)
      root.removeEventListener("focusin", handlers.focusIn)
      root.removeEventListener("focusout", handlers.focusOut)
      root.removeEventListener("keydown", handlers.keyDown)
    })
  })
}

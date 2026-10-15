import { ContextMenu as KContextMenu } from "@kobalte/core/context-menu"
import { Tabs as KTabs } from "@kobalte/core/tabs"
import { createEffect, createSignal, For, type JSX, on, onCleanup, Show } from "solid-js"
import { Icon, type IconName } from "./icon"
import { Entries, type MenuEntry } from "./menu"
import { Tooltip } from "./tooltip"

export interface StripTab {
  key: string
  title: string
  icon?: IconName
  /** Pinned tabs show only their icon and have no close button. */
  pinned?: boolean
}

export interface TabStripProps {
  /** Keep these objects stable (`mapArray` over a store): `<For>` keys rows by identity, so a
   * fresh array of fresh objects rebuilds every tab. */
  tabs: StripTab[]
  active: string | undefined
  onSelect: (key: string) => void
  onClose: (key: string) => void
  /** A tab was dragged to index `to`. Without it tabs can't be dragged. */
  onMove?: (key: string, to: number) => void
  /** The right-click menu for a tab. */
  menu?: (key: string) => MenuEntry[]
  /** Shown after the tabs, outside the scrolling part (new tab, tab list). */
  actions?: JSX.Element
  class?: string
}

/** How far the pointer moves before a press becomes a drag. */
const DRAG_SLOP = 4
/** Within this distance of the strip's ends, a drag scrolls it. */
const EDGE = 32

/** The browser-style tabs above the page. Middle-click closes a tab, dragging reorders, the
 * wheel scrolls when there are more tabs than fit. Tabs shrink to their icon before the strip
 * scrolls. The close buttons are skipped by Tab focus; Ctrl+W closes the active tab. */
export function TabStrip(props: TabStripProps) {
  let list: HTMLDivElement | undefined
  const [menuKey, setMenuKey] = createSignal<string>()
  const [dragging, setDragging] = createSignal<string>()

  const tabAt = (e: Event) => (e.target as Element).closest<HTMLElement>('[data-slot="tab"]')
  const tabEl = (key: string) => list?.querySelector<HTMLElement>(`[data-slot="tab"][data-key="${CSS.escape(key)}"]`)

  // Keep the active tab in view, also when it was picked from the keyboard or a menu.
  createEffect(
    on(
      () => props.active,
      (key) => {
        if (!key || dragging()) return
        requestAnimationFrame(() => tabEl(key)?.scrollIntoView({ block: "nearest", inline: "nearest" }))
      },
    ),
  )

  const onWheel = (e: WheelEvent) => {
    if (!list || list.scrollWidth <= list.clientWidth) return
    if (Math.abs(e.deltaY) <= Math.abs(e.deltaX)) return
    e.preventDefault()
    list.scrollLeft += e.deltaY
  }

  // ── Dragging ──
  // Pointer events on the window, not pointer capture: reordering moves the tab's node, which
  // would drop the capture.

  let stopDrag: (() => void) | undefined
  onCleanup(() => stopDrag?.())

  const onPointerDown = (e: PointerEvent, el: HTMLElement, key: string) => {
    if (e.button !== 0 || !props.onMove || !list || props.tabs.length < 2) return
    if ((e.target as Element).closest('[data-slot="close"]')) return
    const strip = list
    const startX = e.clientX
    const toList = (clientX: number) => clientX - strip.getBoundingClientRect().left + strip.scrollLeft
    const grab = toList(e.clientX) - el.offsetLeft
    let pointerX = e.clientX
    let active = false
    let frame = 0

    const place = () => {
      const node = tabEl(key)
      if (!node) return
      const x = Math.min(Math.max(toList(pointerX) - grab, 0), strip.scrollWidth - node.offsetWidth)
      node.style.transform = `translateX(${x - node.offsetLeft}px)`
      // The slot is the number of other tabs whose middle is left of the dragged tab's middle.
      const mid = x + node.offsetWidth / 2
      let to = 0
      for (const other of strip.querySelectorAll<HTMLElement>('[data-slot="tab"]')) {
        if (other !== node && other.offsetLeft + other.offsetWidth / 2 < mid) to++
      }
      const from = props.tabs.findIndex((t) => t.key === key)
      if (to !== from) {
        props.onMove?.(key, to)
        // The node moved (or the move was refused); line the transform up with where it is now.
        const again = tabEl(key)
        if (again) again.style.transform = `translateX(${x - again.offsetLeft}px)`
      }
    }

    // Scrolls the strip while the pointer rests near either end.
    const tick = () => {
      const r = strip.getBoundingClientRect()
      const by = pointerX < r.left + EDGE ? -1 : pointerX > r.right - EDGE ? 1 : 0
      if (by !== 0) {
        const before = strip.scrollLeft
        strip.scrollLeft += by * 8
        if (strip.scrollLeft !== before) place()
      }
      frame = requestAnimationFrame(tick)
    }

    const move = (ev: PointerEvent) => {
      pointerX = ev.clientX
      if (!active) {
        if (Math.abs(ev.clientX - startX) < DRAG_SLOP) return
        active = true
        setDragging(key)
        frame = requestAnimationFrame(tick)
      }
      place()
    }
    const end = () => {
      cancelAnimationFrame(frame)
      window.removeEventListener("pointermove", move)
      window.removeEventListener("pointerup", end)
      window.removeEventListener("pointercancel", end)
      window.removeEventListener("blur", end)
      const node = tabEl(key)
      if (node) node.style.transform = ""
      setDragging(undefined)
      stopDrag = undefined
    }
    stopDrag = end
    window.addEventListener("pointermove", move)
    window.addEventListener("pointerup", end)
    window.addEventListener("pointercancel", end)
    window.addEventListener("blur", end)
  }

  // Kobalte also changes the selection by itself: when `active` names a tab it hasn't
  // registered yet (one that just opened), it falls back to the first tab. Passing that on
  // would switch away from the new tab, so only pass on changes made by input in the strip.
  let input = false
  const onInput = () => {
    input = true
    setTimeout(() => (input = false))
  }
  const select = (key: string) => {
    if (input && key !== props.active) props.onSelect(key)
  }

  const strip = (
    <KTabs data-slot="root" value={props.active} onChange={select} activationMode="manual">
      <KTabs.List
        ref={list}
        data-slot="list"
        aria-label="Open tabs"
        data-dragging={dragging() ? "" : undefined}
        on:wheel={onWheel}
        on:pointerdown={{ handleEvent: onInput, capture: true }}
        on:pointerup={{ handleEvent: onInput, capture: true }}
        on:click={{ handleEvent: onInput, capture: true }}
        on:keydown={{ handleEvent: onInput, capture: true }}
        // One set of handlers for every tab. This one runs before the context menu opens;
        // off a tab, the menu is for the active one.
        onContextMenu={(e: MouseEvent) => setMenuKey(tabAt(e)?.dataset.key ?? props.active)}
        onPointerDown={(e: PointerEvent) => {
          const el = tabAt(e)
          if (el?.dataset.key) onPointerDown(e, el, el.dataset.key)
        }}
        // Middle-click on Linux pastes or autoscrolls without this.
        onMouseDown={(e: MouseEvent) => {
          if (e.button === 1) e.preventDefault()
        }}
        onAuxClick={(e: MouseEvent) => {
          const key = tabAt(e)?.dataset.key
          const t = key && props.tabs.find((t) => t.key === key)
          if (e.button === 1 && t && !t.pinned) props.onClose(t.key)
        }}
      >
        <For each={props.tabs}>
          {(t) => (
            <div
              data-slot="tab"
              data-key={t.key}
              data-active={props.active === t.key ? "" : undefined}
              data-pinned={t.pinned ? "" : undefined}
              data-dragging={dragging() === t.key ? "" : undefined}
            >
              <Tooltip content={t.title} placement="bottom" class="h-full min-w-0 flex-1">
                <KTabs.Trigger data-slot="trigger" value={t.key}>
                  <Show when={t.icon}>{(name) => <Icon data-slot="icon" name={name()} size={14} />}</Show>
                  <span data-slot="title">{t.title}</span>
                </KTabs.Trigger>
              </Tooltip>
              <Show when={!t.pinned}>
                <button
                  type="button"
                  data-slot="close"
                  tabIndex={-1}
                  aria-label={`Close ${t.title}`}
                  onClick={() => props.onClose(t.key)}
                >
                  <Icon name="x" size={12} />
                </button>
              </Show>
            </div>
          )}
        </For>
      </KTabs.List>
    </KTabs>
  )

  return (
    <div data-component="tab-strip" class={props.class}>
      <Show when={props.menu} fallback={strip}>
        {(menu) => (
          // One menu for the whole strip; the tab under the pointer picks its entries.
          <KContextMenu>
            <KContextMenu.Trigger as="div" data-slot="context-trigger">
              {strip}
            </KContextMenu.Trigger>
            <KContextMenu.Portal>
              <KContextMenu.Content data-component="menu">
                <Show when={menuKey()}>{(key) => <Entries parts={KContextMenu} items={menu()(key())} />}</Show>
              </KContextMenu.Content>
            </KContextMenu.Portal>
          </KContextMenu>
        )}
      </Show>
      <Show when={props.actions}>
        <div data-slot="actions">{props.actions}</div>
      </Show>
    </div>
  )
}

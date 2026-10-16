import { useLocation } from "@solidjs/router"
import { createEffect, createMemo, createSignal, For, on, onCleanup, Show } from "solid-js"
import { menuItem, run, shortcutOf } from "~/lib/commands/registry"
import type { SidebarView } from "~/lib/ipc/bindings"
import { groups, moveScreen, type Screen, screenKey, screens, sidebarGroups } from "~/lib/screens"
import { shell } from "~/lib/store"
import { ContextMenu, NavItem } from "~/lib/ui"

const badge: Record<string, () => number> = {
  "nav.friends": () => shell.unread.friendRequests,
  "nav.trades": () => shell.unread.trades,
  "nav.messages": () => shell.unread.messages,
  "nav.chat": () => shell.unread.chat,
}

/** How far the pointer moves before a press becomes a drag. */
const DRAG_SLOP = 4

/** Screen navigation, in groups the user can reorder (drag, or `view.sidebar_up`/`_down`) and
 * thin out (`view.sidebar_toggle`). Ctrl+B collapses it to icons. */
export function Sidebar(props: { compact: boolean }) {
  const location = useLocation()
  const isActive = (s: Screen) =>
    s.path === "/"
      ? location.pathname === "/"
      : location.pathname === s.path || location.pathname.startsWith(`${s.path}/`)

  // While dragging, and until the saved order comes back, rows follow this order.
  const [draft, setDraft] = createSignal<string[] | null>(null)
  createEffect(
    on(
      () => shell.sidebar.order,
      () => setDraft(null),
      { defer: true },
    ),
  )
  const prefs = (): SidebarView => {
    const order = draft()
    return order ? { order, hidden: shell.sidebar.hidden } : shell.sidebar
  }
  const rows = createMemo(() => sidebarGroups(screens, prefs()))

  const [dragging, setDragging] = createSignal<string>()
  let nav: HTMLElement | undefined
  const rowEl = (key: string) => nav?.querySelector<HTMLElement>(`[data-key="${CSS.escape(key)}"]`)
  let stopDrag: (() => void) | undefined
  onCleanup(() => stopDrag?.())

  // Pointer events on the window: reordering moves the row's node, which would drop a capture.
  // Positions are `offsetTop`s inside the group, which the drag's `translate` doesn't change.
  const onPointerDown = (e: PointerEvent, s: Screen) => {
    if (e.button !== 0) return
    const key = screenKey(s)
    if ((rows().find((g) => g.includes(s))?.length ?? 0) < 2) return
    const startY = e.clientY
    const startTop = rowEl(key)?.offsetTop ?? 0
    let pointerY = e.clientY
    let active = false

    const place = () => {
      const node = rowEl(key)
      const siblings = [...(node?.closest("[data-group]")?.querySelectorAll<HTMLElement>("[data-key]") ?? [])]
      if (!node || siblings.length < 2) return
      const last = siblings[siblings.length - 1]
      const max = last.offsetTop + last.offsetHeight - node.offsetHeight
      const y = Math.min(Math.max(startTop + pointerY - startY, siblings[0].offsetTop), max)
      // The slot is the number of other rows whose middle is above the dragged row's middle.
      const mid = y + node.offsetHeight / 2
      const to = siblings.filter((o) => o !== node && o.offsetTop + o.offsetHeight / 2 < mid).length
      const order = moveScreen(screens, prefs(), key, to)
      if (order) setDraft(order)
      const now = rowEl(key)
      if (now) now.style.translate = `0 ${y - now.offsetTop}px`
    }

    const move = (ev: PointerEvent) => {
      pointerY = ev.clientY
      if (!active) {
        if (Math.abs(ev.clientY - startY) < DRAG_SLOP) return
        active = true
        setDragging(key)
      }
      place()
    }
    const end = () => {
      window.removeEventListener("pointermove", move)
      window.removeEventListener("pointerup", end)
      window.removeEventListener("pointercancel", end)
      window.removeEventListener("blur", end)
      const node = rowEl(key)
      if (node) node.style.translate = ""
      setDragging(undefined)
      stopDrag = undefined
      if (!active) return
      // The release would also click the row; a drag doesn't navigate.
      const swallow = (ev: MouseEvent) => ev.stopPropagation()
      window.addEventListener("click", swallow, { capture: true, once: true })
      setTimeout(() => window.removeEventListener("click", swallow, { capture: true }))
      const order = draft()
      if (order && order.join(" ") !== shell.sidebar.order.join(" ")) {
        void run("view.sidebar_order", order.join(" ")).then(() => {
          // Refused, or the saved order was already this: show what is saved.
          if (draft() === order) setDraft(null)
        })
      } else setDraft(null)
    }
    stopDrag = end
    window.addEventListener("pointermove", move)
    window.addEventListener("pointerup", end)
    window.addEventListener("pointercancel", end)
    window.addEventListener("blur", end)
  }

  const item = (s: Screen) => (
    <NavItem
      label={s.label}
      icon={s.icon}
      active={isActive(s)}
      count={badge[s.command]?.()}
      shortcut={shortcutOf(s.command)}
      compact={props.compact}
      onClick={() => run(s.command)}
    />
  )

  const menu = (s: Screen) => {
    const key = screenKey(s)
    return [
      menuItem("view.sidebar_up", { arg: key, label: "Move up", icon: "chevron-up" }),
      menuItem("view.sidebar_down", { arg: key, label: "Move down", icon: "chevron-down" }),
      menuItem("view.sidebar_toggle", { arg: `${key} off`, label: "Hide from sidebar", icon: "eye" }),
      { kind: "separator" as const },
      menuItem("nav.settings", { arg: "appearance", label: "Customize sidebar", icon: "sliders" }),
    ]
  }

  return (
    <nav
      ref={nav}
      aria-label="Screens"
      data-dragging={dragging() ? "" : undefined}
      class="flex shrink-0 flex-col overflow-y-auto border-r border-line bg-deep p-2"
      classList={{ "w-52": !props.compact, "w-12 items-center px-1.5": props.compact }}
    >
      <For each={groups}>
        {(g) => (
          <Show when={rows().find((r) => (r[0].group ?? "start") === g)}>
            {(group) => (
              <div
                data-group
                class="relative flex flex-col gap-0.5"
                classList={{ "mt-2 border-t border-line pt-2": group() !== rows()[0], "w-full": props.compact }}
              >
                <For each={group()}>
                  {(s) => (
                    <ContextMenu items={menu(s)}>
                      <div
                        data-key={screenKey(s)}
                        class="relative"
                        classList={{ "z-10 opacity-80": dragging() === screenKey(s) }}
                        onPointerDown={(e) => onPointerDown(e, s)}
                      >
                        {item(s)}
                      </div>
                    </ContextMenu>
                  )}
                </For>
              </div>
            )}
          </Show>
        )}
      </For>
      <div class="mt-auto pt-2">
        <For each={screens.filter((s) => s.sidebar === "bottom")}>{item}</For>
      </div>
    </nav>
  )
}

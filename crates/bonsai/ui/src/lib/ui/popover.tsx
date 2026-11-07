import { Popover as KPopover } from "@kobalte/core/popover"
import { type JSX, Show } from "solid-js"

export interface PopoverProps {
  /** Controlled, so a command can open it as well as the trigger. */
  open: boolean
  onOpenChange: (open: boolean) => void
  /** The button that opens it. Spread the props it gets onto a `Button` or `IconButton`. */
  trigger: (props: Record<string, unknown>) => JSX.Element
  /** Heading at the top. */
  title?: string
  /** Buttons on the right of the heading. */
  actions?: JSX.Element
  /** Below the scrolling body, such as a "See all" button. */
  footer?: JSX.Element
  placement?: "bottom-start" | "bottom-end" | "top-start" | "top-end"
  /** On the panel: its width. */
  class?: string
  children: JSX.Element
}

const FOCUSABLE = "button:not([disabled]), [href], input, [tabindex]:not([tabindex='-1'])"

/** A panel anchored to a button: heading, scrolling body, footer. Arrow keys move between the
 * body's buttons; Esc or a click outside closes it. Plain keys stay inside, so screen
 * shortcuts behind it don't fire. */
export function Popover(props: PopoverProps) {
  const onKeyDown = (e: KeyboardEvent) => {
    const plain = !e.ctrlKey && !e.metaKey && !e.altKey
    if (!plain || e.key === "Escape" || e.key === "Tab") return
    e.stopPropagation()
    if (e.key !== "ArrowDown" && e.key !== "ArrowUp") return
    const panel = e.currentTarget as HTMLElement
    const items = [...panel.querySelectorAll<HTMLElement>(`[data-slot="body"] :is(${FOCUSABLE})`)]
    if (items.length === 0) return
    e.preventDefault()
    const at = items.indexOf(document.activeElement as HTMLElement)
    const next = at < 0 ? (e.key === "ArrowDown" ? 0 : items.length - 1) : at + (e.key === "ArrowDown" ? 1 : -1)
    items[Math.max(0, Math.min(items.length - 1, next))]?.focus()
  }
  return (
    <KPopover
      open={props.open}
      onOpenChange={props.onOpenChange}
      placement={props.placement ?? "bottom-end"}
      gutter={6}
    >
      <KPopover.Trigger as={(p: Record<string, unknown>) => props.trigger(p)} />
      <KPopover.Portal>
        <KPopover.Content data-component="popover" class={props.class} onKeyDown={onKeyDown}>
          <Show when={props.title || props.actions}>
            <div data-slot="header">
              <KPopover.Title data-slot="title">{props.title}</KPopover.Title>
              <Show when={props.actions}>
                <div data-slot="actions">{props.actions}</div>
              </Show>
            </div>
          </Show>
          <div data-slot="body">{props.children}</div>
          <Show when={props.footer}>
            <div data-slot="footer">{props.footer}</div>
          </Show>
        </KPopover.Content>
      </KPopover.Portal>
    </KPopover>
  )
}

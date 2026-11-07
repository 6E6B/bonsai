import { type JSX, type ParentProps, Show } from "solid-js"
import { Dynamic } from "solid-js/web"
import { Toolbar } from "./toolbar"

export interface PageProps extends ParentProps {
  /** Filters, search and counts in the toolbar, on the left. */
  toolbar?: JSX.Element
  /** Buttons on the right of the toolbar. */
  actions?: JSX.Element
  class?: string
}

/** Two or more `Pane`s side by side, filling the screen. */
export function Split(props: ParentProps<{ class?: string }>) {
  return (
    <div data-component="split" class={props.class}>
      {props.children}
    </div>
  )
}

/** One column of a `Split`. `side` is the fixed-width list on the left: give it a width in
 * `class`. The other pane takes the rest. Both scroll on their own. `as="nav"` for a pane that
 * is navigation (Settings sections); give it an `aria-label`. */
export function Pane(
  props: ParentProps<{ side?: boolean; as?: "div" | "nav"; class?: string; "aria-label"?: string }>,
) {
  return (
    <Dynamic
      component={props.as ?? "div"}
      data-component="pane"
      data-side={props.side ? "" : undefined}
      class={props.class}
      aria-label={props["aria-label"]}
    >
      {props.children}
    </Dynamic>
  )
}

/** A browsing screen: `SidePanel`s beside a `SideMain`. Unlike `Split`, the page itself scrolls
 * and the panels stick to the view, so long grids keep the tab's scroll position. */
export function SidePage(props: ParentProps<{ class?: string }>) {
  return (
    <div data-component="side-page" class={props.class}>
      {props.children}
    </div>
  )
}

/** A full-height panel of a `SidePage`: give it a width in `class`. `header` and `footer` stay put
 * while the children scroll. `edge="end"` for details on the right. */
export function SidePanel(
  props: ParentProps<{
    edge?: "start" | "end"
    as?: "div" | "aside"
    header?: JSX.Element
    footer?: JSX.Element
    class?: string
    "aria-label"?: string
  }>,
) {
  return (
    <Dynamic
      component={props.as ?? "div"}
      data-component="side-panel"
      data-edge={props.edge ?? "start"}
      class={props.class}
      aria-label={props["aria-label"]}
    >
      <Show when={props.header}>
        <div data-slot="header">{props.header}</div>
      </Show>
      <div data-slot="body">{props.children}</div>
      <Show when={props.footer}>
        <div data-slot="footer">{props.footer}</div>
      </Show>
    </Dynamic>
  )
}

/** The scrolling content of a `SidePage`, with the same insets as a `Page`. */
export function SideMain(props: ParentProps<{ class?: string }>) {
  return (
    <div data-component="side-main" class={props.class}>
      {props.children}
    </div>
  )
}

/** The frame of every screen that is not a `Split` or `SidePage`: fills the window with the same
 * insets everywhere. There is no title, since the tab names the screen. A `Toolbar` comes first
 * when the screen has controls. Lay out the body with Tailwind in `class`. */
export function Page(props: PageProps) {
  return (
    <div data-component="page" class={props.class}>
      <Show when={props.toolbar || props.actions}>
        <Toolbar actions={props.actions}>{props.toolbar}</Toolbar>
      </Show>
      {props.children}
    </div>
  )
}

import { type JSX, type ParentProps, Show } from "solid-js"
import { Dynamic } from "solid-js/web"
import { SectionHeader } from "./section-header"

export interface PageProps extends ParentProps {
  /** Page heading. Entity pages with their own header leave it out. */
  title?: string
  meta?: JSX.Element
  /** Buttons on the right of the heading. */
  actions?: JSX.Element
  /** `narrow` for reading (settings, messages), `wide` for grids, `full` for split views. */
  width?: "narrow" | "wide" | "full"
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

/** The frame of a screen: padding, max width and the heading. Lay out the body with Tailwind
 * in `class`. */
export function Page(props: PageProps) {
  return (
    <div data-component="page" data-width={props.width ?? "wide"} class={props.class}>
      <Show when={props.title}>
        {(t) => <SectionHeader title={t()} meta={props.meta} actions={props.actions} size="page" />}
      </Show>
      {props.children}
    </div>
  )
}

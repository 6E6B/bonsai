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

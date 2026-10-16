import { Show } from "solid-js"

export interface SwatchProps {
  /** A color from data (a body color's hex), not a theme color. */
  color: string
  /** The color's name, for the tooltip and screen readers. */
  label: string
  selected?: boolean
  onClick?: () => void
  /** Just show the color, small, with no button. For inside other controls. */
  display?: boolean
  class?: string
}

/** A pickable color chip: body colors. */
export function Swatch(props: SwatchProps) {
  return (
    <Show
      when={!props.display}
      fallback={
        <span
          data-component="swatch"
          data-display=""
          class={props.class}
          aria-hidden="true"
          style={{ "--swatch-color": props.color }}
        />
      }
    >
      <button
        type="button"
        data-component="swatch"
        class={props.class}
        title={props.label}
        aria-label={props.label}
        aria-pressed={props.selected ?? false}
        data-selected={props.selected ? "" : undefined}
        style={{ "--swatch-color": props.color }}
        onClick={() => props.onClick?.()}
      />
    </Show>
  )
}

import { type JSX, Show } from "solid-js"

export interface ThemeCardProps {
  name: string
  /** A second line: who made it, or where it is in use. */
  detail?: string
  /** The theme's resolved tokens, by name without `--`. The preview is drawn with them. */
  tokens: Record<string, string>
  selected?: boolean
  onClick?: () => void
  class?: string
}

/**
 * A pickable theme: a small window drawn in that theme, then its name. The tokens go on the
 * preview as custom properties, so the preview uses the same token names as the app.
 */
export function ThemeCard(props: ThemeCardProps) {
  const vars = (): JSX.CSSProperties => {
    const out: JSX.CSSProperties = {}
    for (const [name, value] of Object.entries(props.tokens)) out[`--${name}`] = value
    return out
  }
  return (
    <button
      type="button"
      data-component="theme-card"
      class={props.class}
      aria-pressed={props.selected ?? false}
      data-selected={props.selected ? "" : undefined}
      onClick={() => props.onClick?.()}
    >
      <span data-slot="preview" style={vars()} aria-hidden="true">
        <span data-slot="side">
          <span data-slot="dot" data-tone="accent" />
          <span data-slot="dot" />
          <span data-slot="dot" />
        </span>
        <span data-slot="canvas">
          <span data-slot="panel">
            <span data-slot="line" />
            <span data-slot="line" data-tone="muted" />
            <span data-slot="row">
              <span data-slot="fill" />
              <span data-slot="control" />
              <span data-slot="dot" data-tone="success" />
              <span data-slot="dot" data-tone="warning" />
              <span data-slot="dot" data-tone="danger" />
            </span>
          </span>
        </span>
      </span>
      <span data-slot="name">{props.name}</span>
      <Show when={props.detail}>
        <span data-slot="detail">{props.detail}</span>
      </Show>
    </button>
  )
}

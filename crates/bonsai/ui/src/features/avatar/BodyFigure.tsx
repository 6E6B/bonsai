import { For } from "solid-js"
import type { PartColor } from "~/lib/ipc/bindings"

/** Front view, so the avatar's left side is on the viewer's right. Units are SVG px. */
const SHAPES: Record<string, { x: number; y: number; w: number; h: number; r: number }> = {
  head: { x: 43, y: 2, w: 34, h: 32, r: 9 },
  torso: { x: 34, y: 37, w: 52, h: 52, r: 3 },
  right_arm: { x: 7, y: 37, w: 24, h: 52, r: 3 },
  left_arm: { x: 89, y: 37, w: 24, h: 52, r: 3 },
  right_leg: { x: 34, y: 92, w: 24.5, h: 52, r: 3 },
  left_leg: { x: 61.5, y: 92, w: 24.5, h: 52, r: 3 },
}

export interface BodyFigureProps {
  colors: PartColor[]
  selected: string[]
  onToggle: (part: string) => void
  class?: string
}

/** A blocky 2D avatar painted with the current body colors. Each part is a toggle button. */
export function BodyFigure(props: BodyFigureProps) {
  const key = (e: KeyboardEvent, part: string) => {
    if (e.key !== "Enter" && e.key !== " ") return
    e.preventDefault()
    props.onToggle(part)
  }
  return (
    // biome-ignore lint/a11y/noSvgWithoutTitle: the parts inside carry the labels.
    <svg viewBox="0 0 120 146" class={props.class}>
      <For each={props.colors.filter((c) => SHAPES[c.part])}>
        {(c) => {
          const s = SHAPES[c.part]
          const on = () => props.selected.includes(c.part)
          return (
            // biome-ignore lint/a11y/useSemanticElements: SVG shapes cannot be <button>s.
            <g
              role="button"
              tabIndex={0}
              aria-label={c.label}
              aria-pressed={on()}
              class="group cursor-pointer outline-none"
              onClick={() => props.onToggle(c.part)}
              onKeyDown={(e) => key(e, c.part)}
            >
              <title>{c.label}</title>
              <rect
                x={s.x}
                y={s.y}
                width={s.w}
                height={s.h}
                rx={s.r}
                style={{ fill: c.hex }}
                class={
                  on()
                    ? "stroke-accent [stroke-width:2.5]"
                    : "stroke-line-strong [stroke-width:1] group-hover:stroke-fg-muted group-focus-visible:stroke-focus group-focus-visible:[stroke-width:2]"
                }
              />
            </g>
          )
        }}
      </For>
    </svg>
  )
}

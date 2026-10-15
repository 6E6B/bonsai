import { Meter as KMeter } from "@kobalte/core/meter"
import { For } from "solid-js"

export interface PipsProps {
  /** Filled steps, `0` to `max`. */
  value: number
  max: number
  /** Accessible name, like "Demand". */
  label: string
  /** Read out instead of `value/max`, like "High". */
  valueLabel?: string
  /** `scale` colors the fill by level: low is danger, middle is neutral, high is success. */
  tone?: "neutral" | "scale"
  class?: string
}

/** A rating in a few steps, drawn as rising bars like a signal meter: demand. Small enough for
 * a tile or a table cell. Use `Meter` for an amount out of a maximum. */
export function Pips(props: PipsProps) {
  const level = () => {
    if (props.tone !== "scale" || props.max <= 0) return "neutral"
    const r = props.value / props.max
    return r <= 0.2 ? "low" : r <= 0.4 ? "fair" : r < 0.8 ? "neutral" : "high"
  }
  return (
    <KMeter
      data-component="pips"
      data-level={level()}
      class={props.class}
      value={props.value}
      maxValue={props.max}
      aria-label={props.label}
      getValueLabel={({ value, max }) => props.valueLabel ?? `${value}/${max}`}
    >
      <KMeter.Track data-slot="track">
        <For each={Array.from({ length: props.max }, (_, i) => i)}>
          {(i) => (
            <span
              data-slot="pip"
              data-on={i < props.value ? "" : undefined}
              style={{ "--pip-step": `${(i + 1) / props.max}` }}
            />
          )}
        </For>
      </KMeter.Track>
    </KMeter>
  )
}

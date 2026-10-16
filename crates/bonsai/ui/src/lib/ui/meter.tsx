import { Meter as KMeter } from "@kobalte/core/meter"

export interface MeterProps {
  value: number
  max: number
  /** Accessible name, like "Players". */
  label: string
  /** Text next to the bar. Defaults to `value/max`. */
  valueLabel?: string
  /** Fill turns warning at 90% and danger at 100%, for things like full servers. */
  tone?: "neutral" | "capacity"
  class?: string
}

/** A small horizontal bar for a known amount out of a maximum: server fill, rating, storage. */
export function Meter(props: MeterProps) {
  const ratio = () => (props.max > 0 ? props.value / props.max : 0)
  const level = () =>
    props.tone !== "capacity" ? "normal" : ratio() >= 1 ? "full" : ratio() >= 0.9 ? "high" : "normal"
  return (
    <KMeter
      data-component="meter"
      data-level={level()}
      class={props.class}
      value={props.value}
      maxValue={props.max}
      aria-label={props.label}
      getValueLabel={({ value, max }) => props.valueLabel ?? `${value}/${max}`}
    >
      <KMeter.Track data-slot="track">
        <KMeter.Fill data-slot="fill" />
      </KMeter.Track>
      <KMeter.ValueLabel data-slot="value" />
    </KMeter>
  )
}

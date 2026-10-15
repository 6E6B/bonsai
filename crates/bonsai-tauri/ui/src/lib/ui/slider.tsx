import { Slider as KSlider } from "@kobalte/core/slider"

export interface SliderProps {
  /** Shown above the track and used as the accessible name. */
  label: string
  value: number
  min: number
  max: number
  step?: number
  /** While dragging. */
  onChange?: (value: number) => void
  /** When the drag or key press ends. Commit writes here. */
  onChangeEnd?: (value: number) => void
  /** Text for the value. Defaults to the number. */
  format?: (value: number) => string
  disabled?: boolean
  class?: string
}

/** A number in a range: avatar scales, volume. Arrow keys step, Page Up/Down jump, Home/End
 * go to the ends. */
export function Slider(props: SliderProps) {
  return (
    <KSlider
      data-component="slider"
      class={props.class}
      value={[props.value]}
      minValue={props.min}
      maxValue={props.max}
      step={props.step ?? 1}
      disabled={props.disabled}
      onChange={(v) => props.onChange?.(v[0] ?? props.value)}
      onChangeEnd={(v) => props.onChangeEnd?.(v[0] ?? props.value)}
      getValueLabel={({ values }) => (props.format ? props.format(values[0] ?? 0) : String(values[0] ?? 0))}
    >
      <div data-slot="header">
        <KSlider.Label data-slot="label">{props.label}</KSlider.Label>
        <KSlider.ValueLabel data-slot="value" />
      </div>
      <KSlider.Track data-slot="track">
        <KSlider.Fill data-slot="fill" />
        <KSlider.Thumb data-slot="thumb">
          <KSlider.Input data-slot="input" />
        </KSlider.Thumb>
      </KSlider.Track>
    </KSlider>
  )
}

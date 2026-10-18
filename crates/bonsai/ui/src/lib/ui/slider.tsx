import { Slider as KSlider } from "@kobalte/core/slider"
import { createMemo, createSignal, For, Show } from "solid-js"
import { placeMarks, type SliderMark } from "./slider-marks"

export type { SliderMark } from "./slider-marks"

export interface SliderProps {
  /** Shown above the track and used as the accessible name. */
  label: string
  value: number
  min: number
  max: number
  step?: number
  /** Ticks on the track. `true` for one per step, or a list of values. Marks with a `label` show it
   * under the track; clicking the label jumps there. */
  marks?: boolean | readonly (number | SliderMark)[]
  /** Double-clicking the thumb or value, or pressing Delete or Backspace on the thumb, goes back here. */
  resetValue?: number
  /** While dragging. */
  onChange?: (value: number) => void
  /** When the drag or key press ends. Commit writes here. */
  onChangeEnd?: (value: number) => void
  /** Text for the value. Defaults to the number. */
  format?: (value: number) => string
  disabled?: boolean
  class?: string
}

/** Pointer travel before a press counts as a drag. Below it, the thumb glides to a clicked spot. */
const DRAG_SLOP = 3

/** A number in a range: avatar scales, volume. Arrow keys step, Shift+arrows and Page Up/Down jump,
 * Home/End go to the ends. Clicking the track glides the thumb there; dragging follows the pointer. */
export function Slider(props: SliderProps) {
  const [pressAt, setPressAt] = createSignal<number>()
  const [dragging, setDragging] = createSignal(false)
  // Kobalte focuses the thumb on press, which browsers count as :focus-visible. Hide the focus
  // ring until the next key press.
  const [pointer, setPointer] = createSignal(false)
  const marks = createMemo(() => placeMarks(props.min, props.max, props.step ?? 1, props.marks))
  const labeled = () => marks().filter((m) => m.label !== undefined)
  const text = (v: number) => (props.format ? props.format(v) : String(v))

  const set = (v: number) => {
    if (props.disabled || v === props.value) return
    props.onChange?.(v)
    props.onChangeEnd?.(v)
  }
  const reset = () => {
    if (props.resetValue !== undefined) set(props.resetValue)
  }

  const down = (e: PointerEvent) => {
    setPressAt(e.clientX)
    setPointer(true)
  }
  const move = (e: PointerEvent) => {
    const at = pressAt()
    if (at !== undefined && !dragging() && Math.abs(e.clientX - at) > DRAG_SLOP) setDragging(true)
  }
  const up = () => {
    setPressAt(undefined)
    setDragging(false)
  }

  return (
    <KSlider
      data-component="slider"
      data-dragging={dragging() ? "" : undefined}
      data-pointer={pointer() ? "" : undefined}
      data-marked={labeled().length > 0 ? "" : undefined}
      class={props.class}
      value={[props.value]}
      minValue={props.min}
      maxValue={props.max}
      step={props.step ?? 1}
      disabled={props.disabled}
      onChange={(v) => props.onChange?.(v[0] ?? props.value)}
      onChangeEnd={(v) => props.onChangeEnd?.(v[0] ?? props.value)}
      getValueLabel={({ values }) => text(values[0] ?? 0)}
    >
      <div data-slot="header">
        <KSlider.Label data-slot="label">{props.label}</KSlider.Label>
        <KSlider.ValueLabel data-slot="value" onDblClick={reset} />
      </div>
      <KSlider.Track
        data-slot="track"
        onPointerDown={down}
        onPointerMove={move}
        onPointerUp={up}
        onPointerCancel={up}
        onLostPointerCapture={up}
      >
        <KSlider.Fill data-slot="fill" />
        <For each={marks()}>
          {(m) => (
            <span
              data-slot="tick"
              data-active={m.value <= props.value ? "" : undefined}
              style={{ left: `${m.percent}%` }}
            />
          )}
        </For>
        <KSlider.Thumb
          data-slot="thumb"
          onPointerDown={down}
          onPointerMove={move}
          onPointerUp={up}
          onPointerCancel={up}
          onLostPointerCapture={up}
          onDblClick={reset}
          onKeyDown={(e: KeyboardEvent) => {
            setPointer(false)
            if ((e.key === "Delete" || e.key === "Backspace") && props.resetValue !== undefined) {
              e.preventDefault()
              reset()
            }
          }}
        >
          <KSlider.Input data-slot="input" />
        </KSlider.Thumb>
      </KSlider.Track>
      <Show when={labeled().length > 0}>
        <div data-slot="marks" aria-hidden="true">
          <For each={labeled()}>
            {(m) => (
              <span
                data-slot="mark-label"
                data-active={m.value === props.value ? "" : undefined}
                style={{ left: `${m.percent}%`, translate: `-${m.percent}% 0` }}
                onPointerDown={(e) => {
                  e.preventDefault()
                  set(m.value)
                }}
              >
                {m.label}
              </span>
            )}
          </For>
        </div>
      </Show>
    </KSlider>
  )
}

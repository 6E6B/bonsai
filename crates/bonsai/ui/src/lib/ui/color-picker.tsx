import { ColorArea } from "@kobalte/core/color-area"
import { ColorSlider } from "@kobalte/core/color-slider"
import { type Color, parseColor } from "@kobalte/core/colors"
import { createEffect, createSignal, on, Show } from "solid-js"
import { parseHex, toHex } from "~/lib/theme/color"

export interface ColorPickerProps {
  /** The accessible name, like "Accent". */
  label: string
  /** `#rrggbb`, or `#rrggbbaa` with `alpha`. */
  value: string
  /** While dragging or typing. */
  onChange: (hex: string) => void
  /** When a drag or a key press ends. Defaults to nothing; undo steps can group on it. */
  onChangeEnd?: (hex: string) => void
  /** Show an opacity slider. */
  alpha?: boolean
  class?: string
}

const toKobalte = (hex: string): Color | undefined => {
  const c = parseHex(hex)
  if (!c) return undefined
  return parseColor(`rgba(${c.r}, ${c.g}, ${c.b}, ${c.a / 255})`).toFormat("hsb")
}

const fromKobalte = (c: Color, alpha: boolean): string => {
  const rgb = c.toFormat("rgb")
  return toHex({
    r: rgb.getChannelValue("red"),
    g: rgb.getChannelValue("green"),
    b: rgb.getChannelValue("blue"),
    a: alpha ? rgb.getChannelValue("alpha") * 255 : 255,
  })
}

/**
 * A color editor: saturation and brightness area, hue and opacity sliders, and a hex field.
 * The hex field takes any hex form; ↑/↓ in it nudge lightness (Shift for ten steps). Hue is kept
 * while the color passes through grey, so dragging to the edge and back doesn't lose it.
 */
export function ColorPicker(props: ColorPickerProps) {
  const [color, setColor] = createSignal<Color>(toKobalte(props.value) ?? parseColor("#000000").toFormat("hsb"))
  const [text, setText] = createSignal(props.value)
  // Outside changes (undo, another control) replace the color, unless it is the same hex.
  createEffect(
    on(
      () => props.value,
      (v) => {
        if (fromKobalte(color(), !!props.alpha) !== v.toLowerCase()) {
          const c = toKobalte(v)
          if (c) setColor(c)
        }
        if (document.activeElement?.getAttribute("data-slot") !== "hex") setText(v)
      },
    ),
  )
  const change = (c: Color) => {
    setColor(c.toFormat("hsb"))
    const hex = fromKobalte(c, !!props.alpha)
    setText(hex)
    props.onChange(hex)
  }
  const end = (c: Color) => props.onChangeEnd?.(fromKobalte(c, !!props.alpha))
  const nudge = (by: number) => {
    const hsl = color().toFormat("hsl")
    const l = Math.min(100, Math.max(0, hsl.getChannelValue("lightness") + by))
    const next = hsl.withChannelValue("lightness", l)
    change(next)
    end(next)
  }
  return (
    <fieldset data-component="color-picker" class={props.class} aria-label={props.label}>
      <ColorArea
        data-slot="area"
        value={color()}
        colorSpace="hsb"
        xChannel="saturation"
        yChannel="brightness"
        onChange={change}
        onChangeEnd={end}
      >
        <ColorArea.Background data-slot="area-bg">
          <ColorArea.Thumb data-slot="thumb">
            <ColorArea.HiddenInputX aria-label={`${props.label} saturation`} />
            <ColorArea.HiddenInputY aria-label={`${props.label} brightness`} />
          </ColorArea.Thumb>
        </ColorArea.Background>
      </ColorArea>
      <ColorSlider
        data-slot="slider"
        value={color()}
        channel="hue"
        colorSpace="hsb"
        onChange={change}
        onChangeEnd={end}
      >
        <ColorSlider.Track data-slot="track">
          <ColorSlider.Thumb data-slot="thumb">
            <ColorSlider.Input aria-label={`${props.label} hue`} />
          </ColorSlider.Thumb>
        </ColorSlider.Track>
      </ColorSlider>
      <Show when={props.alpha}>
        <ColorSlider
          data-slot="slider"
          data-channel="alpha"
          value={color()}
          channel="alpha"
          colorSpace="hsb"
          onChange={change}
          onChangeEnd={end}
        >
          <ColorSlider.Track data-slot="track">
            <ColorSlider.Thumb data-slot="thumb">
              <ColorSlider.Input aria-label={`${props.label} opacity`} />
            </ColorSlider.Thumb>
          </ColorSlider.Track>
        </ColorSlider>
      </Show>
      <div data-slot="row">
        <span data-slot="chip" style={{ "--picker-color": props.value }} aria-hidden="true" />
        <input
          data-slot="hex"
          aria-label={`${props.label} hex`}
          spellcheck={false}
          autocomplete="off"
          value={text()}
          onInput={(e) => {
            const v = e.currentTarget.value.trim()
            setText(v)
            const c = toKobalte(v.startsWith("#") ? v : `#${v}`)
            if (c) change(c)
          }}
          onBlur={() => {
            setText(props.value)
            end(color())
          }}
          onKeyDown={(e) => {
            if (e.key === "ArrowUp" || e.key === "ArrowDown") {
              e.preventDefault()
              nudge((e.key === "ArrowUp" ? 1 : -1) * (e.shiftKey ? 10 : 1))
            } else if (e.key === "Enter") end(color())
          }}
        />
      </div>
    </fieldset>
  )
}

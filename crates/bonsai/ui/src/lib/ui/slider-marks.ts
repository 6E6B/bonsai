/** A point on a slider's track. Marks with a label also get a clickable label under the track. */
export interface SliderMark {
  value: number
  label?: string
}

export interface PlacedMark extends SliderMark {
  /** Distance from the start of the track, 0 to 100. */
  percent: number
}

/** More ticks than this and they blur into a line, so `marks: true` thins them out. */
export const MAX_TICKS = 25

const decimals = (n: number) => {
  const s = String(n)
  const e = s.indexOf("e-")
  if (e >= 0) return Number(s.slice(e + 2))
  return (s.split(".")[1] ?? "").length
}

/** Where to draw marks. `true` puts a tick on every step (every few steps if there are more than
 * `MAX_TICKS`); a list places one mark per entry. Marks outside the range are dropped. */
export function placeMarks(
  min: number,
  max: number,
  step: number,
  marks: boolean | readonly (number | SliderMark)[] | undefined,
): PlacedMark[] {
  if (!marks || !(max > min)) return []
  const percent = (v: number) => ((v - min) / (max - min)) * 100
  if (marks === true) {
    if (!(step > 0)) return []
    const steps = Math.floor((max - min) / step + 1e-9)
    const stride = Math.ceil(steps / (MAX_TICKS - 1))
    const places = Math.max(decimals(min), decimals(step))
    const out: PlacedMark[] = []
    for (let i = 0; i <= steps; i += stride) {
      const value = Number((min + i * step).toFixed(places))
      out.push({ value, percent: percent(value) })
    }
    return out
  }
  const seen = new Set<number>()
  return marks
    .map((m) => (typeof m === "number" ? { value: m } : m))
    .filter((m) => m.value >= min && m.value <= max && !seen.has(m.value) && seen.add(m.value))
    .sort((a, b) => a.value - b.value)
    .map((m) => ({ ...m, percent: percent(m.value) }))
}

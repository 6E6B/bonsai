import { createStore } from "solid-js/store"
import type { PlaytimeExperience } from "~/lib/ipc/bindings"

/** A span of the playtime log, as `playtime.range` takes it. */
export type PlaytimeRange = "week" | "month" | "quarter" | "year" | "all"

export const playtimeRanges: { value: PlaytimeRange; label: string }[] = [
  { value: "week", label: "This week" },
  { value: "month", label: "4 weeks" },
  { value: "quarter", label: "12 weeks" },
  { value: "year", label: "52 weeks" },
  { value: "all", label: "All time" },
]

export const isPlaytimeRange = (v: string): v is PlaytimeRange => playtimeRanges.some((r) => r.value === v)

/** The range the Playtime screen shows, set by `playtime.range`. */
export const [playtimeView, setPlaytimeView] = createStore<{ range: PlaytimeRange }>({ range: "week" })

/** Local time's offset from UTC in minutes, east positive, as the playtime IPC takes it. */
export const utcOffsetMinutes = (at = new Date()) => -at.getTimezoneOffset()

/** `5400` → `1h 30m`, `600` → `10m`, under a minute → `<1m`, none → `0m`. */
export function duration(secs: number): string {
  if (secs <= 0) return "0m"
  if (secs < 60) return "<1m"
  const mins = Math.round(secs / 60)
  const h = Math.floor(mins / 60)
  const m = mins % 60
  if (h === 0) return `${m}m`
  return m === 0 ? `${h}h` : `${h}h ${m}m`
}

/** Hours with one decimal under 10 and whole above, for chart labels: `5400` → `1.5h`. */
export function hours(secs: number): string {
  const h = secs / 3600
  if (h <= 0) return "0h"
  if (h < 10) return `${Math.round(h * 10) / 10}h`
  return `${Math.round(h)}h`
}

/** The experience's name, or a placeholder until it loads. */
export function experienceName(e: Pick<PlaytimeExperience, "name" | "universeId">): string {
  return e.name || `Experience ${e.universeId}`
}

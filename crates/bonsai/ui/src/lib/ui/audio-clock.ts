/** Seconds as `m:ss`, or `h:mm:ss` past an hour. Unknown lengths show as `-:--`. */
export function clock(seconds: number): string {
  if (!Number.isFinite(seconds) || seconds < 0) return "-:--"
  const s = Math.floor(seconds)
  const ss = String(s % 60).padStart(2, "0")
  const m = Math.floor(s / 60)
  if (m < 60) return `${m}:${ss}`
  return `${Math.floor(m / 60)}:${String(m % 60).padStart(2, "0")}:${ss}`
}

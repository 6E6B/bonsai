/** Number and date formatting shared by every screen. Follows the locale set in Settings. */
import { createSignal } from "solid-js"

const [locale, setLocale] = createSignal<string | undefined>(undefined)
const [clock, setClock] = createSignal<"auto" | "12h" | "24h">("auto")
const [relative, setRelative] = createSignal(true)

/** Empty or invalid tags fall back to the system locale. */
export function setFormatLocale(tag: string) {
  const t = tag.trim()
  try {
    setLocale(t ? Intl.getCanonicalLocales(t)[0] : undefined)
  } catch {
    setLocale(undefined)
  }
}

/** The clock (Settings → General): `auto` follows the locale. Unknown values count as `auto`. */
export function setClockFormat(format: string) {
  setClock(format === "12h" || format === "24h" ? format : "auto")
}

/** Whether `ago` says "5m ago" or gives the date and time. */
export function setRelativeDates(on: boolean) {
  setRelative(on)
}

let cache: {
  key: string
  compact: Intl.NumberFormat
  full: Intl.NumberFormat
  date: Intl.DateTimeFormat
  time: Intl.DateTimeFormat
  dateTime: Intl.DateTimeFormat
} | null = null

/** Reads the locale and clock signals, so callers in JSX update when they change. */
function fmts() {
  const tag = locale()
  const c = clock()
  const key = `${tag ?? ""} ${c}`
  if (!cache || cache.key !== key) {
    const hour12 = c === "auto" ? undefined : c === "12h"
    const day = { month: "short", day: "numeric", year: "numeric" } as const
    const clockOpts = { hour: "numeric", minute: "2-digit", hour12 } as const
    cache = {
      key,
      compact: new Intl.NumberFormat(tag, { notation: "compact", maximumFractionDigits: 1 }),
      full: new Intl.NumberFormat(tag),
      date: new Intl.DateTimeFormat(tag, day),
      time: new Intl.DateTimeFormat(tag, clockOpts),
      dateTime: new Intl.DateTimeFormat(tag, { ...day, ...clockOpts }),
    }
  }
  return cache
}

/** Locales offered in Settings. */
export const formatLocales = [
  { value: "en-US", label: "English (US)" },
  { value: "en-GB", label: "English (UK)" },
  { value: "de-DE", label: "Deutsch" },
  { value: "fr-FR", label: "Français" },
  { value: "es-ES", label: "Español" },
  { value: "pt-BR", label: "Português (Brasil)" },
  { value: "ja-JP", label: "日本語" },
  { value: "ko-KR", label: "한국어" },
  { value: "zh-CN", label: "简体中文" },
]

/** 1,037 → "1,037"; 7,312,004 → "7.3M" (en-US). Keeps exact numbers below 10,000. */
export function compact(n: number): string {
  return Math.abs(n) < 10_000 ? fmts().full.format(n) : fmts().compact.format(n)
}

export function full(n: number): string {
  return fmts().full.format(n)
}

/** 0.904 → "90%". */
export function percent(ratio: number): string {
  return `${Math.round(ratio * 100)}%`
}

/** "Aug 4, 2026". */
export function date(ms: number): string {
  return fmts().date.format(ms)
}

/** "4:05 PM" or "16:05", per the clock setting. */
export function time(ms: number): string {
  return fmts().time.format(ms)
}

/** "Aug 4, 2026, 4:05 PM". */
export function dateTime(ms: number): string {
  return fmts().dateTime.format(ms)
}

/** "now", "5m ago", "3h ago", "2d ago", then the date. With relative times off in Settings,
 * always the date and time. `now` is for tests. */
export function ago(ms: number, now = Date.now()): string {
  if (!relative()) return dateTime(ms)
  const s = Math.max(0, Math.round((now - ms) / 1000))
  if (s < 45) return "now"
  const m = Math.round(s / 60)
  if (m < 60) return `${m}m ago`
  const h = Math.round(m / 60)
  if (h < 24) return `${h}h ago`
  const d = Math.round(h / 24)
  if (d < 7) return `${d}d ago`
  return date(ms)
}

/** 0 → "0 B"; 1536 → "1.5 KB"; 52428800 → "50 MB". */
export function bytes(n: number): string {
  const units = ["B", "KB", "MB", "GB"]
  let v = n
  let i = 0
  while (v >= 1024 && i < units.length - 1) {
    v /= 1024
    i++
  }
  return `${i === 0 ? v : Number(v.toFixed(1))} ${units[i]}`
}

/** Seconds left → "4:05"; never negative. */
export function countdown(secs: number): string {
  const s = Math.max(0, Math.floor(secs))
  return `${Math.floor(s / 60)}:${String(s % 60).padStart(2, "0")}`
}

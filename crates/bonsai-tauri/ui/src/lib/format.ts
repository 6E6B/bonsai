/** Number and date formatting shared by every screen. Follows the locale set in Settings. */
import { createSignal } from "solid-js"

const [locale, setLocale] = createSignal<string | undefined>(undefined)

/** Empty or invalid tags fall back to the system locale. */
export function setFormatLocale(tag: string) {
  const t = tag.trim()
  try {
    setLocale(t ? Intl.getCanonicalLocales(t)[0] : undefined)
  } catch {
    setLocale(undefined)
  }
}

let cache: {
  tag: string | undefined
  compact: Intl.NumberFormat
  full: Intl.NumberFormat
  date: Intl.DateTimeFormat
} | null = null

/** Reads the locale signal, so callers in JSX update when it changes. */
function fmts() {
  const tag = locale()
  if (!cache || cache.tag !== tag) {
    cache = {
      tag,
      compact: new Intl.NumberFormat(tag, { notation: "compact", maximumFractionDigits: 1 }),
      full: new Intl.NumberFormat(tag),
      date: new Intl.DateTimeFormat(tag, { month: "short", day: "numeric", year: "numeric" }),
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

/** "now", "5m ago", "3h ago", "2d ago", then the date. `now` is for tests. */
export function ago(ms: number, now = Date.now()): string {
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

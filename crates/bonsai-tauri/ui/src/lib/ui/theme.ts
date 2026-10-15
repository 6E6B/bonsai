/**
 * Sets `data-theme` on `<html>`, which picks the token block in `styles/theme.css`.
 * `system` follows the OS setting and updates when it changes.
 */
export type ThemeMode = "system" | "light" | "dark"

let mode: ThemeMode = "system"
let media: MediaQueryList | undefined

function apply() {
  const dark = media?.matches ?? true
  document.documentElement.dataset.theme = mode === "system" ? (dark ? "dark" : "light") : mode
}

export function setTheme(next: ThemeMode) {
  mode = next
  if (!media && typeof window.matchMedia === "function") {
    media = window.matchMedia("(prefers-color-scheme: dark)")
    media.addEventListener("change", apply)
  }
  apply()
}

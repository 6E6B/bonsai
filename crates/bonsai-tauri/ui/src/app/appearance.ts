import { isTauri } from "@tauri-apps/api/core"
import { createEffect, on, onCleanup, onMount } from "solid-js"
import { type AppearanceView, commands } from "~/lib/ipc/bindings"
import { useLive } from "~/lib/store"
import { applyAppearance, type Mode, parseTheme, registerFonts } from "~/lib/theme"

const initial = (): AppearanceView => ({ mode: "system", dark: "", light: "", fonts: [] })

/** Keeps the window on the saved mode and themes, live, and loads their Google fonts as they
 * download. Call once from the layout. */
export function useAppearance() {
  const a = useLive(
    () => "appearance",
    () => commands.appearance(),
    initial,
  )
  createEffect(
    on(
      () => [a.mode, a.dark, a.light] as const,
      ([mode, dark, light]) => applyAppearance(mode as Mode, { dark: parseTheme(dark), light: parseTheme(light) }),
    ),
  )
  createEffect(() => registerFonts(a.fonts))
  // The backend needs the OS preference to know which theme System mode shows.
  onMount(() => {
    if (!isTauri() || typeof window.matchMedia !== "function") return
    const media = window.matchMedia("(prefers-color-scheme: dark)")
    const report = () => void commands.setSystemDark(media.matches).catch(() => {})
    report()
    media.addEventListener("change", report)
    onCleanup(() => media.removeEventListener("change", report))
  })
}

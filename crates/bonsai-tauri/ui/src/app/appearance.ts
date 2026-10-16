import { isTauri } from "@tauri-apps/api/core"
import { createEffect, on, onCleanup, onMount } from "solid-js"
import { type AppearanceView, commands } from "~/lib/ipc/bindings"
import { shell, useLive } from "~/lib/store"
import {
  applyAppearance,
  type Mode,
  parseTheme,
  registerFonts,
  setAccentTag,
  setSeeThrough,
  settleBackdrop,
  withDraft,
} from "~/lib/theme"

const initial = (): AppearanceView => ({ mode: "system", dark: "", light: "", fonts: [], background: null })

/** Keeps the window on the saved mode and themes, live, and loads their Google fonts as they
 * download. Call once from the layout. Returns the live view, for the background picture. */
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
  createEffect(() => settleBackdrop(a.background))
  createEffect(() => setSeeThrough(a.background ? withDraft(a.background).panels / 100 : undefined))
  // "Account color as accent": the active account's tag, unless it has none.
  createEffect(() => {
    const color = shell.look.accountAccent ? shell.accounts.find((x) => x.id === shell.active)?.color : undefined
    setAccentTag(color && color !== "none" ? color : undefined)
  })
  // The backend needs the OS preference to know which theme System mode shows.
  onMount(() => {
    if (!isTauri() || typeof window.matchMedia !== "function") return
    const media = window.matchMedia("(prefers-color-scheme: dark)")
    const report = () => void commands.setSystemDark(media.matches).catch(() => {})
    report()
    media.addEventListener("change", report)
    onCleanup(() => media.removeEventListener("change", report))
  })
  return a
}

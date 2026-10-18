import { isTauri } from "@tauri-apps/api/core"
import { createEffect, createSignal, onCleanup, onMount } from "solid-js"
import { commands, events } from "~/lib/ipc/bindings"

/** Rounds the window's corners when the backend asks (Linux, see `frame.rs`): GTK can't clip
 * the webview, so the page clips itself and leaves the corners transparent. Square while
 * maximized or tiled. Call once from the layout with the UI zoom, which scales the radius. */
export function useWindowFrame(zoom: () => number) {
  const [radius, setRadius] = createSignal(0)
  onMount(() => {
    if (!isTauri()) return
    // Registered before any await, so it stays owned by the layout.
    const off = events.windowFrame.listen((e) => setRadius(e.payload.radius))
    onCleanup(() => void off.then((f) => f()))
    void commands.windowFrame().then((f) => setRadius(f.radius))
  })
  createEffect(() => {
    const root = document.documentElement
    if (radius() > 0) {
      root.dataset.frame = "rounded"
      root.style.setProperty("--window-radius", `${radius() / zoom()}px`)
    } else {
      delete root.dataset.frame
      root.style.removeProperty("--window-radius")
    }
  })
}

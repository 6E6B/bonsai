import { getVersion } from "@tauri-apps/api/app"
import { isTauri } from "@tauri-apps/api/core"
import { createResource, createSignal, onCleanup, onMount, Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import { commands, events, type WindowButton, type WindowButtons } from "~/lib/ipc/bindings"
import { windowIcons } from "~/lib/thumb"
import { WindowControls } from "~/lib/ui"

/** Outside the app (`bun run dev`), the bundled buttons on the right. */
const BROWSER: WindowButtons = { style: "builtin", start: [], end: ["minimize", "maximize", "close"] }

/** The window's title bar (native decorations are off): the app name, version and window
 * controls. Empty space drags the window; double-click maximizes. Only elements that carry
 * `data-tauri-drag-region` themselves drag, so the label and its parts carry it too.
 *
 * On Linux the buttons follow the desktop: its symbolic icons, styled like Adwaita or Breeze,
 * on the side and in the order of its button layout (see `titlebar.rs`). */
export function TitleBar() {
  // The workspace version from Cargo.toml. Not available under `bun run dev` in a browser.
  const [version] = createResource(() => (isTauri() ? getVersion() : Promise.resolve("")))
  const [buttons] = createResource(() => (isTauri() ? commands.windowButtons() : Promise.resolve(BROWSER)))
  const [maximized, setMaximized] = createSignal(false)
  onMount(() => {
    if (!isTauri()) return
    const off = events.windowMaximized.listen((e) => setMaximized(e.payload.maximized))
    onCleanup(() => void off.then((f) => f()))
    void commands.windowMaximized().then(setMaximized)
  })

  const variant = () => {
    const style = buttons()?.style
    return style === "adwaita" || style === "breeze" ? style : "default"
  }
  const controls = (list: WindowButton[]) => (
    <Show when={list.length > 0}>
      <WindowControls
        buttons={list}
        variant={variant()}
        icons={windowIcons()}
        maximized={maximized()}
        onMinimize={() => run("window.minimize")}
        onMaximize={() => run("window.maximize")}
        onClose={() => run("app.quit")}
      />
    </Show>
  )

  return (
    <header
      data-tauri-drag-region
      data-window-corner="top"
      class="flex h-8 shrink-0 items-center border-b border-line bg-deep"
    >
      {controls(buttons()?.start ?? [])}
      <span data-tauri-drag-region class="flex flex-1 select-none items-baseline gap-1.5 px-3 text-sm">
        <span data-tauri-drag-region class="font-semibold text-fg">
          Bonsai
        </span>
        <span data-tauri-drag-region class="text-fg-muted">
          {version()}
        </span>
      </span>
      {controls(buttons()?.end ?? [])}
    </header>
  )
}

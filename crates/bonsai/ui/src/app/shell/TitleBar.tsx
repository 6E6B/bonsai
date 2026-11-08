import { getVersion } from "@tauri-apps/api/app"
import { isTauri } from "@tauri-apps/api/core"
import { createResource, createSignal, type JSX, onCleanup, onMount, type ParentProps, Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import { commands, events, type WindowButton, type WindowButtons } from "~/lib/ipc/bindings"
import { windowIcons } from "~/lib/thumb"
import { WindowControls } from "~/lib/ui"

/** Outside the app (`bun run dev`), the bundled buttons on the right. */
const BROWSER: WindowButtons = { style: "builtin", start: [], end: ["minimize", "maximize", "close"] }

/** The window's title bar (native decorations are off): the app name, version, `center` (the
 * command bar trigger, centered on the window), `children` (buttons on the right) and window
 * controls. The side columns share the leftover width equally so `center` stays centered. Empty space drags the window; double-click
 * maximizes. Only elements that carry `data-tauri-drag-region` themselves drag, so the label and
 * its parts carry it too.
 *
 * On Linux the buttons follow the desktop: its symbolic icons, styled like Adwaita or Breeze,
 * on the side and in the order of its button layout (see `titlebar.rs`). The bar is
 * `--titlebar-h` tall on every desktop. */
export function TitleBar(props: ParentProps<{ center?: JSX.Element }>) {
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
      class="grid h-(--titlebar-h) shrink-0 grid-cols-[1fr_minmax(0,36rem)_1fr] items-stretch border-b border-line bg-deep"
    >
      <div data-tauri-drag-region class="flex min-w-0 items-center">
        {controls(buttons()?.start ?? [])}
        {/* The left inset matches the space above and below the text (bar minus border and line). */}
        <span
          data-tauri-drag-region
          class="flex shrink-0 select-none items-baseline gap-1.5 pr-3 pl-[calc((var(--titlebar-h)_-_1px_-_var(--text-sm--line-height))_/_2)] text-sm"
        >
          <span data-tauri-drag-region class="font-semibold text-fg">
            Bonsai
          </span>
          <span data-tauri-drag-region class="text-fg-muted">
            {version()}
          </span>
        </span>
      </div>
      <div data-tauri-drag-region class="flex min-w-0 items-center">
        {props.center}
      </div>
      <div data-tauri-drag-region class="flex min-w-0 items-center justify-end">
        {props.children}
        {controls(buttons()?.end ?? [])}
      </div>
    </header>
  )
}

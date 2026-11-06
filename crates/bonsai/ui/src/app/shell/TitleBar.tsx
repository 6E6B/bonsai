import { getVersion } from "@tauri-apps/api/app"
import { isTauri } from "@tauri-apps/api/core"
import { createResource, createSignal, onCleanup, onMount, type ParentProps, Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import { commands, events, type WindowButton, type WindowButtons } from "~/lib/ipc/bindings"
import { windowIcons } from "~/lib/thumb"
import { WindowControls } from "~/lib/ui"

/** Outside the app (`bun run dev`), the bundled buttons on the right. */
const BROWSER: WindowButtons = { style: "builtin", start: [], end: ["minimize", "maximize", "close"], height: 0 }

/** The window's title bar (native decorations are off): the app name, version, `children` (the
 * layout passes `TopBar`) and window controls. Empty space drags the window; double-click
 * maximizes. Only elements that carry `data-tauri-drag-region` themselves drag, so the label and
 * its parts carry it too.
 *
 * On Linux the buttons follow the desktop: its symbolic icons, styled like Adwaita or Breeze,
 * on the side and in the order of its button layout, and the bar's height follows its title
 * bars (see `titlebar.rs`). */
export function TitleBar(props: ParentProps) {
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
      class="flex h-(--titlebar-h) shrink-0 items-center border-b border-line bg-deep"
      // At least tall enough for the command bar with even space around it.
      style={{ "--titlebar-h": `${Math.max(buttons()?.height ?? 0, 40)}px` }}
    >
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
      <Show when={props.children} fallback={<span data-tauri-drag-region class="flex-1 self-stretch" />}>
        {props.children}
      </Show>
      {controls(buttons()?.end ?? [])}
    </header>
  )
}

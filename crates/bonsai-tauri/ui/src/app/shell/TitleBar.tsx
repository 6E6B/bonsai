import { getVersion } from "@tauri-apps/api/app"
import { isTauri } from "@tauri-apps/api/core"
import { createResource } from "solid-js"
import { run } from "~/lib/commands/registry"
import { WindowControls } from "~/lib/ui"

/** The window's title bar (native decorations are off): the app name, version and window
 * controls. Empty space drags the window; double-click maximizes. Only elements that carry
 * `data-tauri-drag-region` themselves drag, so the label and its parts carry it too. */
export function TitleBar() {
  // The workspace version from Cargo.toml. Not available under `bun run dev` in a browser.
  const [version] = createResource(() => (isTauri() ? getVersion() : Promise.resolve("")))
  return (
    <header data-tauri-drag-region class="flex h-8 shrink-0 items-center border-b border-line bg-deep">
      <span data-tauri-drag-region class="flex flex-1 select-none items-baseline gap-1.5 px-3 text-sm">
        <span data-tauri-drag-region class="font-semibold text-fg">
          Bonsai
        </span>
        <span data-tauri-drag-region class="text-fg-muted">
          {version()}
        </span>
      </span>
      <WindowControls
        onMinimize={() => run("window.minimize")}
        onMaximize={() => run("window.maximize")}
        onClose={() => run("app.quit")}
      />
    </header>
  )
}

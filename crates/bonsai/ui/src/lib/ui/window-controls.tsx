import { Button as KButton } from "@kobalte/core/button"
import { For, Show } from "solid-js"
import { Icon, type IconName } from "./icon"

export type WindowButtonKind = "minimize" | "maximize" | "close"

/** Glyph URLs for the system-themed variants, used as CSS masks so they take the text color. */
export interface WindowControlIcons {
  minimize: string
  maximize: string
  restore: string
  close: string
}

export interface WindowControlsProps {
  onMinimize: () => void
  onMaximize: () => void
  onClose: () => void
  /** Buttons to show, in order. All three by default. */
  buttons?: WindowButtonKind[]
  /** `default`: wide flat buttons with bundled icons, flush with the bar's edge. `adwaita` and
   * `breeze`: small round buttons like GNOME's and KDE's, drawn with `icons`. */
  variant?: "default" | "adwaita" | "breeze"
  icons?: WindowControlIcons
  /** Shows the restore glyph instead of maximize. */
  maximized?: boolean
}

const ALL: WindowButtonKind[] = ["minimize", "maximize", "close"]

/** Minimize, maximize and close for the custom title bar. */
export function WindowControls(props: WindowControlsProps) {
  const button = (kind: WindowButtonKind) => {
    switch (kind) {
      case "minimize":
        return { label: "Minimize", run: props.onMinimize, icon: "minimize" as IconName, size: 14 }
      case "maximize":
        return {
          label: props.maximized ? "Restore" : "Maximize",
          run: props.onMaximize,
          icon: "maximize" as IconName,
          size: 12,
        }
      case "close":
        return { label: "Close", run: props.onClose, icon: "x" as IconName, size: 14 }
    }
  }
  const glyph = (kind: WindowButtonKind) => {
    if (!props.icons) return undefined
    if (kind === "maximize" && props.maximized) return props.icons.restore
    return props.icons[kind]
  }
  return (
    <div data-component="window-controls" data-variant={props.variant ?? "default"}>
      <For each={props.buttons ?? ALL}>
        {(kind) => (
          <KButton
            data-slot="button"
            data-close={kind === "close" ? "" : undefined}
            aria-label={button(kind).label}
            onClick={() => button(kind).run()}
          >
            <Show
              when={props.variant && props.variant !== "default" && glyph(kind)}
              fallback={<Icon name={button(kind).icon} size={button(kind).size} />}
            >
              {(url) => (
                <span
                  data-slot="glyph"
                  style={{ "mask-image": `url(${url()})`, "-webkit-mask-image": `url(${url()})` }}
                />
              )}
            </Show>
          </KButton>
        )}
      </For>
    </div>
  )
}

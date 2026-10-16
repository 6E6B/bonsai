import { Show } from "solid-js"
import { SettingsGroup } from "~/lib/ui"
import { PrefRow, usePref } from "./prefs"

/** The tray icon and starting with the computer. */
export function DesktopSection() {
  const tray = usePref("tray")
  return (
    <>
      <SettingsGroup title="Tray">
        <PrefRow name="tray" />
        <Show when={tray()?.value === "on"}>
          <PrefRow name="close_to_tray" />
        </Show>
      </SettingsGroup>
      <SettingsGroup title="Startup">
        <PrefRow name="start_on_login" />
        <Show when={tray()?.value === "on"}>
          <PrefRow name="start_hidden" />
        </Show>
      </SettingsGroup>
    </>
  )
}

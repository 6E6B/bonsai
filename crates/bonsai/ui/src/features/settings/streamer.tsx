import { Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import { shell } from "~/lib/store"
import { Button, SettingRow, SettingSwitch, SettingsGroup } from "~/lib/ui"
import { PrefRow } from "./prefs"

/** Streamer mode: hiding private content, and noticing recording apps. */
export function StreamerSection() {
  return (
    <>
      <SettingsGroup
        title="Streamer mode"
        description="Blurs Robux amounts and your own account until hovered. Hides other accounts, nicknames, notes, chat previews and notification text."
      >
        <SettingSwitch
          label="Streamer mode"
          checked={shell.streamer.on}
          onChange={(on) => run("streamer.toggle", on ? "on" : "off")}
        />
        <PrefRow name="streamer_hide_messages" />
        <Show
          when={shell.streamer.captureSupported}
          fallback={
            <SettingRow
              label="Hide from screen capture"
              setting="streamer_hide_capture"
              description="Not supported on this system."
            />
          }
        >
          <PrefRow name="streamer_hide_capture" />
        </Show>
      </SettingsGroup>
      <SettingsGroup
        title="Recording apps"
        description="Checked every few seconds. Bonsai can tell an app is open, not whether it is live. Discord is never counted."
      >
        <PrefRow name="streamer_detect" />
        <PrefRow name="streamer_apps" />
        <SettingRow label="Open now">
          <span class="text-sm text-fg-muted">
            <Show when={shell.streamer.running.length > 0} fallback="None">
              {shell.streamer.running.join(", ")}
            </Show>
          </span>
        </SettingRow>
        <Show when={shell.streamer.ignored.length > 0}>
          <SettingRow label="Not asked about" description={shell.streamer.ignored.join(", ")}>
            <Button size="sm" onClick={() => run("streamer.ask_again")}>
              Ask again
            </Button>
          </SettingRow>
        </Show>
      </SettingsGroup>
    </>
  )
}

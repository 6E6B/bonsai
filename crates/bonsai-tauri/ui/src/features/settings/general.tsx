import { createSignal, Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import { formatLocales } from "~/lib/format"
import { shell } from "~/lib/store"
import { Button, RadioGroup, Select, SettingRow, SettingsGroup, TextField } from "~/lib/ui"
import { useAppSettings } from "./shared"

/** Launch command presets. `{uri}` is the `roblox-player:` link; without it the link goes last. */
const SOBER = "flatpak run org.vinegarhq.Sober {uri}"
const launchModes = [
  { value: "default", label: "System default" },
  { value: "sober", label: "Sober (Flatpak)" },
  { value: "custom", label: "Custom command" },
]

/** How the app starts games and opens things, and how it formats numbers and dates. */
export function GeneralSection() {
  const app = useAppSettings()
  const [custom, setCustom] = createSignal<string>()
  const mode = () => {
    if (custom() !== undefined) return "custom"
    if (!app.playerCommand) return "default"
    return app.playerCommand === SOBER ? "sober" : "custom"
  }
  const pick = (m: string) => {
    if (m === "custom") return setCustom(app.playerCommand === SOBER ? "" : app.playerCommand)
    setCustom(undefined)
    void run("app.player_command", m === "sober" ? SOBER : "")
  }
  return (
    <>
      <SettingsGroup title="Playing" description="Saved on this computer for every account.">
        <SettingRow label="Launch Roblox with" description="How Join opens the game.">
          <Select class="w-48" aria-label="Launch Roblox with" options={launchModes} value={mode()} onChange={pick} />
        </SettingRow>
        <Show when={mode() === "custom"}>
          <form
            class="flex items-end gap-2 py-2.5"
            onSubmit={(e) => {
              e.preventDefault()
              void run("app.player_command", custom() ?? app.playerCommand)
              setCustom(undefined)
            }}
          >
            <TextField
              class="flex-1"
              label="Command"
              placeholder="wine /path/to/RobloxPlayerLauncher.exe {uri}"
              value={custom() ?? app.playerCommand}
              onChange={setCustom}
            />
            <Button type="submit" variant="primary">
              Save
            </Button>
          </form>
        </Show>
        <SettingRow label="Avatars" description="How avatars and items open. 3D shows a spinner until the model loads.">
          <RadioGroup
            aria-label="Avatars"
            orientation="horizontal"
            value={shell.avatar3d ? "3d" : "2d"}
            options={[
              { value: "2d", label: "Image" },
              { value: "3d", label: "3D" },
            ]}
            onChange={(v) => run("avatar.default_view", v)}
          />
        </SettingRow>
      </SettingsGroup>
      <SettingsGroup title="Region">
        <SettingRow label="Number and date format" description="Applies to counts, prices and dates.">
          <Select
            size="sm"
            aria-label="Number and date format"
            value={app.formatLocale || "system"}
            options={[{ value: "system", label: "System" }, ...formatLocales]}
            onChange={(v) => run("app.format_locale", v === "system" ? "" : v)}
          />
        </SettingRow>
      </SettingsGroup>
    </>
  )
}

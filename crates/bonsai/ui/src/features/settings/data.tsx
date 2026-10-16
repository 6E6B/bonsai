import { run } from "~/lib/commands/registry"
import { bytes } from "~/lib/format"
import { Button, SettingRow, SettingsGroup } from "~/lib/ui"
import { PrefRow } from "./prefs"
import { useAppSettings } from "./shared"

/** What the app keeps on disk, settings files, and data it fetches from outside Roblox. */
export function DataSection() {
  const app = useAppSettings()
  const usage = () =>
    app.cacheData === null || app.cacheImages === null
      ? "Measuring…"
      : `${bytes(app.cacheData)} of data, ${bytes(app.cacheImages)} of images`
  return (
    <>
      <SettingsGroup title="Cache">
        <SettingRow setting="cache" label="Saved data and images" description={usage()}>
          <Button size="sm" icon="trash" onClick={() => run("app.clear_cache")}>
            Clear
          </Button>
        </SettingRow>
        <PrefRow name="cache_days" />
        <PrefRow name="image_cache_mb" />
      </SettingsGroup>
      <SettingsGroup title="Settings">
        <SettingRow
          setting="settings_file"
          label="Settings file"
          description="Move settings, shortcuts and the sidebar to another computer. Themes have their own export, and the launch command stays here."
        >
          <span class="flex gap-2">
            <Button size="sm" icon="download" onClick={() => run("settings.export")}>
              Export
            </Button>
            <Button size="sm" icon="upload" onClick={() => run("settings.import")}>
              Import
            </Button>
          </span>
        </SettingRow>
        <SettingRow
          setting="reset_all"
          label="Reset all settings"
          description="Themes, the background, the sidebar, shortcuts and accounts are kept."
        >
          <Button size="sm" variant="danger" onClick={() => run("settings.reset", "all")}>
            Reset
          </Button>
        </SettingRow>
      </SettingsGroup>
      <SettingsGroup title="Third-party data">
        <PrefRow name="item_values" />
      </SettingsGroup>
      <SettingsGroup title="Bug reports">
        <SettingRow
          setting="diagnostics"
          label="Diagnostics"
          description="Version, system, cache size and changed settings. No names, ids or cookies."
        >
          <Button size="sm" icon="copy" onClick={() => run("app.copy_diagnostics")}>
            Copy
          </Button>
        </SettingRow>
      </SettingsGroup>
    </>
  )
}

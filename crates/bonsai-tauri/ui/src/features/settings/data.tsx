import { run } from "~/lib/commands/registry"
import { bytes } from "~/lib/format"
import { Button, SettingRow, SettingsGroup } from "~/lib/ui"
import { PrefRow } from "./prefs"
import { useAppSettings } from "./shared"

/** What the app keeps on disk, and data it fetches from outside Roblox. */
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
      </SettingsGroup>
      <SettingsGroup title="Third-party data">
        <PrefRow name="item_values" />
      </SettingsGroup>
    </>
  )
}

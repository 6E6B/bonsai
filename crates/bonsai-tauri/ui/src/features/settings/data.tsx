import { run } from "~/lib/commands/registry"
import { bytes } from "~/lib/format"
import { shell } from "~/lib/store"
import { Button, SettingRow, SettingSwitch, SettingsGroup } from "~/lib/ui"
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
        <SettingRow label="Saved data and images" description={usage()}>
          <Button size="sm" icon="trash" onClick={() => run("app.clear_cache")}>
            Clear
          </Button>
        </SettingRow>
      </SettingsGroup>
      <SettingsGroup title="Third-party data">
        <SettingSwitch
          label="Item values"
          description="Show Rolimons values, demand and trends for limiteds, the Values screen, and trading stats and last online on profiles. Refreshed every two minutes while a screen shows them."
          checked={shell.itemValues}
          onChange={() => run("market.values")}
        />
      </SettingsGroup>
    </>
  )
}

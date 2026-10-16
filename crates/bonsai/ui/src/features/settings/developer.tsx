import { run } from "~/lib/commands/registry"
import { Button, SettingRow, SettingsGroup } from "~/lib/ui"

/** Tools for working on the app itself. */
export function DeveloperSection() {
  return (
    <SettingsGroup title="Tools">
      <SettingRow label="Component gallery" description="Every shared component in each of its states.">
        <Button size="sm" onClick={() => run("nav.components")}>
          Open
        </Button>
      </SettingRow>
    </SettingsGroup>
  )
}

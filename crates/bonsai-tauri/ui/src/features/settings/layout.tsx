import { For, Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import { arrange, nudgeScreen, screenKey, screens } from "~/lib/screens"
import { shell } from "~/lib/store"
import { Button, Icon, IconButton, SettingsGroup, Switch } from "~/lib/ui"

/** Which screens the sidebar shows and in what order. Dragging rows in the sidebar does the
 * same as the arrows here. */
export function LayoutSection() {
  const groups = () => arrange(screens, shell.sidebar).filter((g) => g.length > 0)
  return (
    <SettingsGroup
      title="Sidebar"
      description="Drag rows in the sidebar or use the arrows. Each divider starts a group."
      actions={
        <Button size="sm" variant="ghost" onClick={() => run("view.sidebar_reset")}>
          Reset
        </Button>
      }
    >
      <For each={groups()}>
        {(group) => (
          <div class="flex flex-col py-1.5">
            <For each={group}>
              {(s) => {
                const key = screenKey(s)
                const off = () => s.shown?.() === false
                return (
                  <div class="flex h-10 items-center gap-2.5">
                    <Icon name={s.icon} />
                    <span class="flex-1">{s.label}</span>
                    <Show when={off()}>
                      <span class="text-sm text-fg-muted">Off while item values are off</span>
                    </Show>
                    <IconButton
                      icon="chevron-up"
                      label={`Move ${s.label} up`}
                      size="sm"
                      disabled={!nudgeScreen(screens, shell.sidebar, key, -1)}
                      onClick={() => run("view.sidebar_up", key)}
                    />
                    <IconButton
                      icon="chevron-down"
                      label={`Move ${s.label} down`}
                      size="sm"
                      disabled={!nudgeScreen(screens, shell.sidebar, key, 1)}
                      onClick={() => run("view.sidebar_down", key)}
                    />
                    <Switch
                      hideLabel
                      checked={!shell.sidebar.hidden.includes(key)}
                      onChange={(on) => run("view.sidebar_toggle", `${key} ${on ? "on" : "off"}`)}
                    >
                      {`Show ${s.label} in the sidebar`}
                    </Switch>
                  </div>
                )
              }}
            </For>
          </div>
        )}
      </For>
    </SettingsGroup>
  )
}

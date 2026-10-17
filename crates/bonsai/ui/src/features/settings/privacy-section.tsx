import { createMemo, For, Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import { commands, type Person, type PrivacyRow } from "~/lib/ipc/bindings"
import { emptyList, ListState } from "~/lib/list-state"
import { privacyOptions, shownPrivacy, valueLabel } from "~/lib/privacy"
import { shell, useLive } from "~/lib/store"
import { thumbUrl } from "~/lib/thumb"
import { Avatar, Button, ListItem, Select, SettingRow, SettingsGroup } from "~/lib/ui"
import { WebButton } from "./shared"

export function PrivacySection() {
  const privacy = useLive(
    () => shell.active,
    () => commands.privacySettings(null),
    emptyList<PrivacyRow>,
  )
  const blocked = useLive(
    () => shell.active,
    () => commands.people("blocked"),
    emptyList<Person>,
  )
  const shown = createMemo(() => shownPrivacy(privacy.items))
  return (
    <>
      <SettingsGroup
        title="Who can"
        description="Saved on Roblox for this account."
        actions={<WebButton label="More on website" />}
      >
        <ListState list={privacy} icon="lock" empty="No privacy settings">
          <For each={shown()}>
            {(s) => (
              <SettingRow
                label={s.label}
                description={s.row.options.some((o) => !o.allowed) ? "Some choices need age verification." : undefined}
              >
                <Show
                  when={s.row.options.length > 0}
                  fallback={<span class="text-fg-muted">{valueLabel(s.row.value)}</span>}
                >
                  <Select
                    class="w-56"
                    aria-label={s.label}
                    options={privacyOptions(s.row)}
                    value={s.row.value}
                    onChange={(v) => v !== s.row.value && run("privacy.set", `${s.row.key} ${v}`)}
                  />
                </Show>
              </SettingRow>
            )}
          </For>
        </ListState>
      </SettingsGroup>
      <SettingsGroup title="Blocked users" meta={blocked.items.length || undefined} list>
        <ListState list={blocked} icon="block" empty="Nobody blocked">
          <For each={blocked.items}>
            {(p) => (
              <ListItem
                leading={<Avatar name={p.displayName || "?"} src={thumbUrl("headshot", p.id)} />}
                title={p.displayName || String(p.id)}
                subtitle={p.name ? `@${p.name}` : undefined}
                onClick={() => run("user.open", String(p.id))}
                actions={
                  <Button size="sm" onClick={() => run("user.unblock", String(p.id))}>
                    Unblock
                  </Button>
                }
              />
            )}
          </For>
        </ListState>
      </SettingsGroup>
    </>
  )
}

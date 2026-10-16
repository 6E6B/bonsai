import { For } from "solid-js"
import { run } from "~/lib/commands/registry"
import { shell } from "~/lib/store"
import { thumbUrl } from "~/lib/thumb"
import { Avatar, Badge, Button, ListItem, SettingsGroup } from "~/lib/ui"
import { PrefRow } from "./prefs"

/** The Roblox accounts signed in on this computer. */
export function AccountsSection() {
  return (
    <>
      <SettingsGroup
        title="Signed in"
        meta={shell.accounts.length}
        description="Each account keeps its own tabs and screen state."
        list
        actions={
          <Button size="sm" icon="plus" onClick={() => run("account.add")}>
            Add account
          </Button>
        }
      >
        <For each={shell.accounts}>
          {(a) => (
            <ListItem
              leading={<Avatar name={a.displayName} src={thumbUrl("headshot", a.id)} />}
              title={a.label ?? a.displayName}
              subtitle={`@${a.name}`}
              meta={a.id === shell.active ? <Badge variant="accent">Active</Badge> : undefined}
              actions={
                <>
                  <Button size="sm" onClick={() => run("account.replace_cookie", null, a.id)}>
                    New cookie
                  </Button>
                  <Button size="sm" variant="danger" onClick={() => run("account.remove", String(a.id))}>
                    Remove
                  </Button>
                </>
              }
            />
          )}
        </For>
      </SettingsGroup>
      <SettingsGroup title="Background">
        <PrefRow name="background_accounts" />
      </SettingsGroup>
    </>
  )
}

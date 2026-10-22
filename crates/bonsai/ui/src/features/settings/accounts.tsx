import { For, Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import type { Account, Color } from "~/lib/ipc/bindings"
import { shell } from "~/lib/store"
import { thumbUrl } from "~/lib/thumb"
import { Avatar, Badge, Button, IconButton, ListItem, Select, SettingsGroup, usePrompt } from "~/lib/ui"
import { vault } from "~/lib/vault"
import { PrefRow } from "./prefs"

const colors: { value: Color; label: string }[] = [
  { value: "none", label: "No color" },
  { value: "red", label: "Red" },
  { value: "orange", label: "Orange" },
  { value: "yellow", label: "Yellow" },
  { value: "green", label: "Green" },
  { value: "blue", label: "Blue" },
  { value: "purple", label: "Purple" },
  { value: "pink", label: "Pink" },
]

/** The Roblox accounts signed in on this computer. */
export function AccountsSection() {
  const prompt = usePrompt()
  const rename = async (a: Account) => {
    const name = await prompt({ title: "Rename account", label: "Label", initial: a.label ?? a.displayName })
    if (name !== undefined && name !== null) void run("account.rename", name, a.id)
  }
  return (
    <>
      <SettingsGroup
        title="Bonsai PIN"
        description="One PIN protects every saved account on this computer. This is separate from Roblox account security."
      >
        <Show when={vault.systemLockWarning}>
          <p role="status" class="px-3 pt-3 text-sm text-warning">
            {vault.systemLockWarning}
          </p>
        </Show>
        <div class="flex flex-wrap items-center gap-2 p-3">
          <Show
            when={vault.status === "unlocked"}
            fallback={<Button onClick={() => run("vault.enable")}>Enable protection</Button>}
          >
            <Button onClick={() => run("vault.lock")}>Lock now</Button>
            <Button onClick={() => run("vault.change")}>Change PIN</Button>
            <Button variant="ghost" onClick={() => run("vault.disable")}>
              Turn off protection
            </Button>
          </Show>
        </div>
        <PrefRow name="vault_idle_minutes" />
      </SettingsGroup>
      <SettingsGroup
        title="Signed in"
        meta={shell.accounts.length}
        description="Each account keeps its own tabs and screen state. The label and color only show in this app."
        list
        actions={
          <Button size="sm" icon="plus" onClick={() => run("account.add")}>
            Add account
          </Button>
        }
      >
        <For each={shell.accounts}>
          {(a, i) => (
            <ListItem
              leading={<Avatar name={a.displayName} src={thumbUrl("headshot", a.id)} />}
              title={a.label ?? a.displayName}
              subtitle={`@${a.name}`}
              meta={a.id === shell.active ? <Badge variant="accent">Active</Badge> : undefined}
              actions={
                <>
                  <IconButton
                    size="sm"
                    icon="chevron-up"
                    label="Move up"
                    disabled={i() === 0}
                    onClick={() => void run("account.move_up", null, a.id)}
                  />
                  <IconButton
                    size="sm"
                    icon="chevron-down"
                    label="Move down"
                    disabled={i() === shell.accounts.length - 1}
                    onClick={() => void run("account.move_down", null, a.id)}
                  />
                  <Select
                    size="sm"
                    class="w-28"
                    aria-label="Color tag"
                    options={colors}
                    value={a.color}
                    onChange={(c) => void run("account.color", c, a.id)}
                  />
                  <Button size="sm" onClick={() => void rename(a)}>
                    Rename
                  </Button>
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
      <SettingsGroup title="Look">
        <PrefRow name="account_accent" />
      </SettingsGroup>
      <SettingsGroup title="Background">
        <PrefRow name="background_accounts" />
      </SettingsGroup>
    </>
  )
}

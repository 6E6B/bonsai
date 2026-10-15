import { For } from "solid-js"
import { menuItem, run, shortcutOf } from "~/lib/commands/registry"
import type { Account, Color } from "~/lib/ipc/bindings"
import { shell } from "~/lib/store"
import { thumbUrl } from "~/lib/thumb"
import { Avatar, ContextMenu, Icon, type MenuEntry, RailItem, useConfirm, usePrompt } from "~/lib/ui"

const colors: Color[] = ["none", "red", "orange", "yellow", "green", "blue", "purple", "pink"]
const colorName = (c: Color) => (c === "none" ? "No color" : c[0].toUpperCase() + c.slice(1))

/** The far-left column: one slot per signed-in account, then "Add account". Right-click an
 * account for its menu; every entry there is also a command. */
export function AccountRail() {
  const prompt = usePrompt()
  const confirm = useConfirm()

  const menu = (a: Account, index: number): MenuEntry[] => {
    const as = { asAccount: a.id }
    return [
      menuItem("account.switch", { arg: String(a.id), label: "Switch to this account", icon: "user" }),
      { kind: "separator" },
      {
        label: "Rename…",
        icon: "edit",
        onSelect: async () => {
          const name = await prompt({ title: "Rename account", label: "Label", initial: a.label ?? a.displayName })
          if (name) void run("account.rename", name, a.id)
        },
      },
      {
        kind: "submenu",
        label: "Color tag",
        items: colors.map((c) => menuItem("account.color", { ...as, arg: c, label: colorName(c) })),
      },
      menuItem("account.move_up", { ...as, disabled: index === 0 }),
      menuItem("account.move_down", { ...as, disabled: index === shell.accounts.length - 1 }),
      { kind: "separator" },
      menuItem("account.replace_cookie", as),
      menuItem("account.sign_out", { ...as, icon: "sign-out" }),
      {
        label: "Remove from app…",
        icon: "trash",
        danger: true,
        onSelect: async () => {
          const ok = await confirm({
            title: `Remove ${a.label ?? a.displayName}?`,
            description: "Its session and cached data are deleted from this computer.",
            confirm: "Remove",
            danger: true,
          })
          if (ok) void run("account.remove", String(a.id))
        },
      },
    ]
  }

  return (
    <nav
      aria-label="Accounts"
      class="flex w-16 shrink-0 flex-col items-stretch gap-1 overflow-y-auto border-r border-line bg-deep py-2"
    >
      <For each={shell.accounts}>
        {(a, i) => (
          <ContextMenu items={menu(a, i())}>
            <RailItem
              label={a.label ?? a.displayName}
              shortcut={i() < 9 ? shortcutOf(`account.switch_${i() + 1}`) : undefined}
              active={shell.active === a.id}
              color={a.color}
              onClick={() => run("account.switch", String(a.id))}
            >
              <Avatar size="lg" name={a.displayName} src={thumbUrl("headshot", a.id)} />
            </RailItem>
          </ContextMenu>
        )}
      </For>
      <RailItem label="Add account" onClick={() => run("account.add")}>
        <span class="flex size-10 items-center justify-center rounded-full bg-control text-fg-muted">
          <Icon name="plus" size={18} />
        </span>
      </RailItem>
    </nav>
  )
}

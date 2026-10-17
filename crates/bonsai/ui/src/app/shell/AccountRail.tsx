import { createSignal, For } from "solid-js"
import { menuItem, run, shortcutOf } from "~/lib/commands/registry"
import { type Account, type Color, commands, type PrivacyRow } from "~/lib/ipc/bindings"
import { emptyList } from "~/lib/list-state"
import { privacyLabels, valueLabel } from "~/lib/privacy"
import { shell, useLive } from "~/lib/store"
import { thumbUrl } from "~/lib/thumb"
import { Avatar, ContextMenu, Icon, type MenuEntry, RailItem, useConfirm, usePrompt } from "~/lib/ui"

const colors: Color[] = ["none", "red", "orange", "yellow", "green", "blue", "purple", "pink"]
const colorName = (c: Color) => (c === "none" ? "No color" : c[0].toUpperCase() + c.slice(1))
// The privacy settings the menu offers; the rest are in Settings.
const menuPrivacy = ["whoCanJoinMeInExperiences", "whoCanSeeMyOnlineStatus"]

/** The far-left column: one slot per signed-in account, then "Add account". Right-click an
 * account for its menu; every entry there is also a command. */
export function AccountRail() {
  const prompt = usePrompt()
  const confirm = useConfirm()
  // Privacy settings of the account whose menu was opened last, loaded when it opens.
  const [menuFor, setMenuFor] = createSignal<number | null>(null)
  const privacy = useLive(
    menuFor,
    (id) => (id === null ? Promise.resolve(emptyList<PrivacyRow>()) : commands.privacySettings(id)),
    emptyList<PrivacyRow>,
  )

  const privacyMenu = (a: Account, key: string): MenuEntry => {
    const label = privacyLabels.find(([k]) => k === key)?.[1] ?? key
    const row = menuFor() === a.id ? privacy.items.find((r) => r.key === key) : undefined
    if (!row) {
      return { kind: "submenu", label, items: [{ label: "Loading…", disabled: true, onSelect: () => {} }] }
    }
    return {
      kind: "submenu",
      label,
      items: row.options.map((o) =>
        menuItem("privacy.set", {
          asAccount: a.id,
          arg: `${key} ${o.value}`,
          label: valueLabel(o.value),
          icon: o.value === row.value ? "check" : undefined,
          disabled: !o.allowed,
        }),
      ),
    }
  }

  const menu = (a: Account, index: number): MenuEntry[] => {
    const as = { asAccount: a.id }
    return [
      menuItem("account.switch", {
        arg: String(a.id),
        label: "Switch to this account",
        icon: "user",
        disabled: shell.active === a.id,
      }),
      { kind: "separator" },
      menuItem("entity.copy_id", { arg: `user:${a.id}`, label: "Copy user ID", icon: "copy" }),
      menuItem("account.copy_name", { ...as, label: "Copy username", icon: "copy" }),
      {
        kind: "submenu",
        label: "Privacy",
        icon: "eye",
        items: menuPrivacy.map((key) => privacyMenu(a, key)),
      },
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
        icon: "palette",
        items: colors.map((c) => menuItem("account.color", { ...as, arg: c, label: colorName(c) })),
      },
      {
        kind: "submenu",
        label: "Move",
        icon: "sort-asc",
        disabled: shell.accounts.length < 2,
        items: [
          menuItem("account.move_up", { ...as, label: "Up", icon: "chevron-up", disabled: index === 0 }),
          menuItem("account.move_down", {
            ...as,
            label: "Down",
            icon: "chevron-down",
            disabled: index === shell.accounts.length - 1,
          }),
        ],
      },
      { kind: "separator" },
      {
        kind: "submenu",
        label: "Session",
        icon: "lock",
        items: [
          menuItem("account.approve_code", { ...as, label: "Sign in another device…", icon: "devices" }),
          menuItem("account.replace_cookie", { ...as, label: "Paste new cookie…" }),
          menuItem("account.sign_out_others", { ...as, label: "Sign out other sessions" }),
        ],
      },
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
          <ContextMenu items={menu(a, i())} onOpenChange={(open) => open && setMenuFor(a.id)}>
            <RailItem
              label={a.unread > 0 ? `${a.label ?? a.displayName} (${a.unread} unread)` : (a.label ?? a.displayName)}
              shortcut={i() < 9 ? shortcutOf(`account.switch_${i() + 1}`) : undefined}
              active={shell.active === a.id}
              color={a.color}
              onClick={() => run("account.switch", String(a.id))}
            >
              <Avatar size="lg" name={a.displayName} src={thumbUrl("headshot", a.id)} count={a.unread} />
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

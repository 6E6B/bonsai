import { Show } from "solid-js"
import { menuItem, run } from "~/lib/commands/registry"
import type { Account } from "~/lib/ipc/bindings"
import { canReactivate, moderationLabel, moderationNext, moderationTone, useNowSecs } from "~/lib/moderation"
import { shell } from "~/lib/store"
import { thumbUrl } from "~/lib/thumb"
import { Avatar, Button, Dialog, Icon, Menu } from "~/lib/ui"

/** The active account, when Roblox has moderated it. Nothing else in the app works for it then. */
export const moderatedAccount = () => {
  const a = shell.accounts.find((x) => x.id === shell.active)
  return a?.session === "moderated" ? a : undefined
}

/** Shortcuts that still work while the moderation dialog is up: switching to another account. */
export const allowedWhileModerated = (id: string) => !moderatedAccount() || id.startsWith("account.switch")

/**
 * Blocks the app while the active account is warned, banned, terminated or locked. It can't be
 * dismissed: the way out is to reactivate, sign out, or switch to another account.
 */
export function ModerationDialog() {
  return (
    <Dialog open={moderatedAccount() !== undefined} onOpenChange={() => {}} label="Account moderated" size="sm">
      <Show when={moderatedAccount()}>{(a) => <ModerationBody account={a()} />}</Show>
    </Dialog>
  )
}

function ModerationBody(props: { account: Account }) {
  const now = useNowSecs()
  const m = () => props.account.moderation
  const name = () => props.account.label ?? props.account.displayName
  const danger = () => moderationTone(m(), now()) === "danger"
  const others = () => shell.accounts.filter((a) => a.id !== props.account.id)
  return (
    <div class="flex flex-col">
      <div class="flex items-center gap-3 px-5 pt-5">
        <Avatar size="lg" name={props.account.displayName} src={thumbUrl("headshot", props.account.id)} />
        <div class="flex min-w-0 flex-1 flex-col">
          <span class="truncate font-semibold">{name()}</span>
          <span class="truncate text-sm text-fg-muted">@{props.account.name}</span>
        </div>
        <span
          class="flex shrink-0 items-center gap-1.5 font-semibold"
          classList={{ "text-danger": danger(), "text-warning": !danger() }}
        >
          <Icon name={danger() ? "circle-x" : "alert"} size={16} />
          {moderationLabel(m(), now())}
        </span>
      </div>
      <div class="flex flex-col gap-4 p-5">
        <div class="flex flex-col gap-1.5">
          <Show when={m()?.title && m()?.title !== m()?.message ? m()?.title : undefined}>
            {(t) => <span class="font-medium">{t()}</span>}
          </Show>
          <Show when={m()?.message}>{(text) => <p class="line-clamp-6 whitespace-pre-line text-sm">{text()}</p>}</Show>
          <span class="text-sm text-fg-muted">{moderationNext(m(), now())}</span>
        </div>
        <div class="flex flex-col gap-2">
          <Show when={canReactivate(m(), now())}>
            <Button variant="primary" onClick={() => run("account.reactivate")}>
              Reactivate
            </Button>
          </Show>
          <Button icon="external" onClick={() => run("entity.open_website", "section:moderation")}>
            Open on website
          </Button>
          <Button variant="danger" icon="sign-out" onClick={() => run("account.sign_out")}>
            Sign out
          </Button>
        </div>
        <Show when={others().length > 0}>
          <Menu
            variant="ghost"
            size="sm"
            class="self-center"
            placement="top-start"
            items={others().map((a) =>
              menuItem("account.switch", { arg: String(a.id), label: a.label ?? a.displayName, icon: "user" }),
            )}
          >
            Switch account
          </Menu>
        </Show>
      </div>
    </div>
  )
}

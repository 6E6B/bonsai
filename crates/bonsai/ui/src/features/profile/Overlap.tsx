import { For, Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import { commands, type OverlapAccount, type OverlapView } from "~/lib/ipc/bindings"
import { useLive } from "~/lib/store"
import { thumbUrl } from "~/lib/thumb"
import { Button, Card, Mention } from "~/lib/ui"
import { overlapLine, overlapNote, personName, SHOWN } from "./overlap"

const empty = (): OverlapView => ({ loading: true, error: null, checked: 0, complete: false, accounts: [] })

/** Which of your accounts know this person, directly or through a friend. Hidden when none do. */
export function OverlapCard(props: { userId: number }) {
  const view = useLive(
    () => props.userId,
    (uid) => commands.socialOverlap(uid),
    empty,
    { keep: true },
  )
  return (
    <Show when={view.accounts.length > 0}>
      <Card class="flex h-fit min-w-0 flex-col gap-2 px-3.5 py-3">
        <h3 class="text-sm font-medium text-fg-muted">Your accounts</h3>
        <For each={view.accounts}>{(a) => <AccountRow account={a} />}</For>
        <div class="flex items-center justify-between gap-2 text-xs text-fg-muted">
          <span>{overlapNote(view)}</span>
          <Show when={!view.complete && view.checked > 0}>
            <Button size="sm" variant="ghost" loading={view.loading} onClick={() => run("list.more", "friends")}>
              Check more
            </Button>
          </Show>
        </div>
      </Card>
    </Show>
  )
}

function AccountRow(props: { account: OverlapAccount }) {
  const a = () => props.account
  const rest = () => a().mutuals.length - SHOWN
  return (
    <div class="flex min-w-0 flex-col gap-0.5">
      <div class="flex items-baseline justify-between gap-2">
        <span class="truncate font-medium">{a().label}</span>
        <span class="shrink-0 text-xs text-fg-muted">{overlapLine(a().friend, a().mutuals.length)}</span>
      </div>
      <Show when={a().mutuals.length > 0}>
        <div class="flex flex-wrap items-center gap-1 text-sm">
          <For each={a().mutuals.slice(0, SHOWN)}>
            {(p) => (
              <Mention
                text={personName(p)}
                name={p.name || personName(p)}
                title={p.name ? `@${p.name}` : undefined}
                src={thumbUrl("headshot", p.id, true)}
                onClick={() => run("user.open", String(p.id))}
              />
            )}
          </For>
          <Show when={rest() > 0}>
            <span class="text-xs text-fg-muted">+{rest()}</span>
          </Show>
        </div>
      </Show>
    </div>
  )
}

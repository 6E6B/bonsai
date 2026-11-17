import { createMemo, createSignal, For, Show } from "solid-js"
import { type CopyItemView, commands } from "~/lib/ipc/bindings"
import { shell, useLive } from "~/lib/store"
import { accountName } from "~/lib/streamer"
import { Badge, Button, EmptyState, ListItem, Select, showToast } from "~/lib/ui"
import { copySummary, copyTargets } from "./copy"

const empty = () => ({ loading: true, error: null, rig: "", items: [], ready: false, wear: [] })

const ownedBadge: Record<string, { variant: "success" | "danger" | "neutral"; text: string }> = {
  yes: { variant: "success", text: "Owned" },
  no: { variant: "danger", text: "Not owned" },
  checking: { variant: "neutral", text: "Checking" },
  unknown: { variant: "neutral", text: "Couldn't check" },
}

/**
 * Copies a look (`outfit:<id>`, `avatar:<user id>`, or empty for the active account's avatar)
 * to another account. Nothing changes until the confirm button.
 */
export function CopyLookDialog(props: { source: string; from: number | null; onDone: () => void }) {
  const targets = createMemo(() => copyTargets(shell.accounts, props.from))
  const [picked, setPicked] = createSignal<number | null>(null)
  const target = () => picked() ?? targets()[0]?.id ?? null
  const [busy, setBusy] = createSignal(false)
  const plan = useLive(
    () => [props.source, target()] as const,
    ([source, t]) => (t === null ? Promise.resolve(empty()) : commands.copyLookPlan(source, t)),
    empty,
  )
  const name = () => {
    const a = shell.accounts.find((x) => x.id === target())
    return a ? accountName(a) : ""
  }
  const missing = () => plan.items.filter((i) => i.owned !== "yes")
  const confirm = async () => {
    const t = target()
    if (t === null || !plan.ready || busy()) return
    setBusy(true)
    const r = await commands.copyLook(props.source, t, plan.wear)
    setBusy(false)
    if (r.status === "error") showToast({ title: r.error, variant: "error" })
    else props.onDone()
  }
  return (
    <div class="flex flex-col gap-3 p-4">
      <Show
        when={targets().length}
        fallback={<EmptyState icon="user" title="No other account" description="Add another account to copy to." />}
      >
        <Select
          label="Copy to"
          options={targets().map((a) => ({ value: String(a.id), label: accountName(a) }))}
          value={target() === null ? undefined : String(target())}
          onChange={(v) => setPicked(Number(v))}
        />
        <p class="text-sm text-fg-muted" aria-live="polite">
          {copySummary(plan)}
        </p>
        <Show when={missing().length}>
          <div class="flex flex-col">
            <span class="text-xs text-fg-muted">Left off</span>
            <For each={missing()}>{(i: CopyItemView) => <MissingRow item={i} />}</For>
          </div>
        </Show>
        <div class="flex justify-end gap-2">
          <Button variant="ghost" onClick={() => props.onDone()}>
            Cancel
          </Button>
          <Button
            variant="primary"
            icon="avatar"
            disabled={!plan.ready || !!plan.error}
            loading={busy()}
            onClick={() => void confirm()}
          >
            Wear on {name()}
          </Button>
        </div>
      </Show>
    </div>
  )
}

function MissingRow(props: { item: CopyItemView }) {
  const badge = () => ownedBadge[props.item.owned] ?? ownedBadge.unknown
  return (
    <ListItem
      title={props.item.name || `Item ${props.item.id}`}
      subtitle={props.item.typeName || undefined}
      meta={<Badge variant={badge().variant}>{badge().text}</Badge>}
    />
  )
}

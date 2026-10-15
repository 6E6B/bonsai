import { createEffect, createMemo, createSignal, For, Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import { commands, type PayoutRow, type RevenueView, type UserLookup } from "~/lib/ipc/bindings"
import { emptyList, ListState } from "~/lib/list-state"
import { useLive } from "~/lib/store"
import { thumbUrl } from "~/lib/thumb"
import { Avatar, Button, Card, IconButton, ListItem, Robux, SectionHeader, Stat, TextField } from "~/lib/ui"

/** The community's funds and its recurring payout split. Pay out and Edit open forms that run
 * `group.payout` and `group.recurring_payouts`. */
export function Revenue(props: { groupId: number; canSpend: boolean }) {
  const view = useLive(
    () => props.groupId,
    commands.communityRevenue,
    (): RevenueView => ({ funds: null, fundsError: null, split: emptyList<PayoutRow>() }),
  )
  const total = () => view.split.items.reduce((n, p) => n + p.percent, 0)
  return (
    <div class="flex max-w-2xl flex-col gap-4">
      <Card class="flex flex-wrap items-center justify-between gap-4 p-3">
        <Stat label="Funds">
          <Show
            when={view.funds !== null}
            fallback={<span class="text-fg-muted">{view.fundsError ?? "Loading…"}</span>}
          >
            <Robux amount={view.funds} />
          </Show>
        </Stat>
        <Show when={props.canSpend}>
          <Button icon="robux" disabled={!view.funds} onClick={() => run("group.payout")}>
            Pay out
          </Button>
        </Show>
      </Card>
      <div class="flex flex-col gap-2">
        <SectionHeader
          title="Recurring payouts"
          meta={view.split.items.length > 0 ? `${total()}% of income` : undefined}
          actions={
            <Show when={props.canSpend}>
              <Button size="sm" icon="edit" onClick={() => run("group.recurring_payouts")}>
                Edit
              </Button>
            </Show>
          }
        />
        <ListState list={view.split} icon="robux" empty="No recurring payouts">
          <div class="flex flex-col">
            <For each={view.split.items}>
              {(p) => (
                <ListItem
                  leading={<Avatar size="sm" name={p.name} src={thumbUrl("headshot", p.id)} />}
                  title={p.name || `User ${p.id}`}
                  meta={<span class="tabular-nums">{p.percent}%</span>}
                  onClick={() => run("user.open", String(p.id))}
                />
              )}
            </For>
          </div>
        </ListState>
      </div>
    </div>
  )
}

/** Who and how much for a one-time payout. Submitting runs `group.payout`, which confirms. */
export function PayoutForm(props: { onDone: () => void }) {
  const [who, setWho] = createSignal("")
  const [amount, setAmount] = createSignal("")
  const n = () => Number(amount().trim())
  const valid = () => who().trim() !== "" && Number.isInteger(n()) && n() > 0
  return (
    <form
      class="flex flex-col gap-3 p-4"
      onSubmit={(e) => {
        e.preventDefault()
        if (!valid()) return
        props.onDone()
        void run("group.payout", `${who().trim()} ${n()}`)
      }}
    >
      <TextField label="Member" placeholder="Username or ID" value={who()} onChange={setWho} autofocus />
      <TextField
        label="Robux"
        inputmode="numeric"
        value={amount()}
        onChange={setAmount}
        error={amount().trim() && !(Number.isInteger(n()) && n() > 0) ? "A whole number above 0." : undefined}
      />
      <div class="flex justify-end gap-2">
        <Button variant="ghost" type="button" onClick={() => props.onDone()}>
          Cancel
        </Button>
        <Button variant="primary" type="submit" disabled={!valid()}>
          Pay out
        </Button>
      </div>
    </form>
  )
}

interface Share {
  id: number
  name: string
  percent: string
}

/** Edits the recurring split. Saving sends the whole list; members left out stop getting a
 * share. Submitting runs `group.recurring_payouts`, which confirms. */
export function SplitForm(props: { current: PayoutRow[]; onDone: () => void }) {
  const [rows, setRows] = createSignal<Share[]>(
    props.current.map((p) => ({ id: p.id, name: p.name, percent: String(p.percent) })),
  )
  const [draft, setDraft] = createSignal("")
  // A name typed into Add is looked up, then added once Roblox answers.
  const [adding, setAdding] = createSignal("")
  const lookup = useLive(
    adding,
    (w) => (w ? commands.findUser(w) : Promise.resolve({ loading: false, error: null, id: null })),
    (): UserLookup => ({ loading: true, error: null, id: null }),
  )
  createEffect(() => {
    const id = lookup.id
    const name = adding()
    if (!name || lookup.loading || id === null) return
    if (!rows().some((r) => r.id === id)) setRows([...rows(), { id, name: name.replace(/^@/, ""), percent: "" }])
    setAdding("")
    setDraft("")
  })
  const set = (id: number, percent: string) => setRows(rows().map((r) => (r.id === id ? { ...r, percent } : r)))
  const pct = (r: Share) => Number(r.percent.trim())
  const total = createMemo(() => rows().reduce((n, r) => n + (Number.isInteger(pct(r)) ? pct(r) : 0), 0))
  const bad = (r: Share) => !Number.isInteger(pct(r)) || pct(r) < 1 || pct(r) > 100
  const error = () => {
    if (rows().some(bad)) return "Each share is 1 to 100 percent."
    if (total() > 100) return `The shares add up to ${total()}%. Keep it at 100% or less.`
    return undefined
  }
  const save = () => {
    if (error()) return
    props.onDone()
    const arg =
      rows().length === 0
        ? "none"
        : rows()
            .map((r) => `${r.id}:${pct(r)}`)
            .join(",")
    void run("group.recurring_payouts", arg)
  }
  return (
    <form
      class="flex flex-col gap-3 p-4"
      onSubmit={(e) => {
        e.preventDefault()
        save()
      }}
    >
      <div class="flex flex-col">
        <For
          each={rows()}
          fallback={<span class="py-2 text-fg-muted">No one gets a share. Saving stops payouts.</span>}
        >
          {(r) => (
            <ListItem
              leading={<Avatar size="sm" name={r.name} src={thumbUrl("headshot", r.id)} />}
              title={r.name || `User ${r.id}`}
              actions={
                <>
                  <TextField
                    class="w-20"
                    size="sm"
                    inputmode="numeric"
                    aria-label={`Percent for ${r.name || r.id}`}
                    value={r.percent}
                    onChange={(v) => set(r.id, v)}
                  />
                  <span class="text-fg-muted">%</span>
                  <IconButton
                    size="sm"
                    icon="x"
                    label={`Remove ${r.name || r.id}`}
                    onClick={() => setRows(rows().filter((x) => x.id !== r.id))}
                  />
                </>
              }
            />
          )}
        </For>
      </div>
      <div class="flex items-end gap-2">
        <TextField
          class="flex-1"
          label="Add member"
          placeholder="Username or ID"
          value={draft()}
          onChange={setDraft}
          error={adding() && lookup.error ? lookup.error : undefined}
          onKeyDown={(e: KeyboardEvent) => {
            if (e.key !== "Enter") return
            e.preventDefault()
            setAdding(draft().trim())
          }}
        />
        <Button type="button" icon="plus" disabled={!draft().trim()} onClick={() => setAdding(draft().trim())}>
          Add
        </Button>
      </div>
      <span class={error() ? "text-sm text-danger" : "text-sm text-fg-muted"}>
        {error() ?? `${total()}% of income goes to these members.`}
      </span>
      <div class="flex justify-end gap-2">
        <Button variant="ghost" type="button" onClick={() => props.onDone()}>
          Cancel
        </Button>
        <Button variant="primary" type="submit" disabled={Boolean(error())}>
          Save
        </Button>
      </div>
    </form>
  )
}

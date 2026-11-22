import { createMemo, createSignal, Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import { commands, type SharedFriendsView } from "~/lib/ipc/bindings"
import { userMenu } from "~/lib/menus"
import { shell, useLive } from "~/lib/store"
import { thumbUrl } from "~/lib/thumb"
import { ContextMenu, EmptyState, ListItem, Select, TextField, Thumbnail, VirtualList } from "~/lib/ui"
import { accountName, compareSummary, filterPeople } from "./compare"

const empty = (): SharedFriendsView => ({ loading: true, aCount: 0, bCount: 0, linked: false, friends: [] })

/** Friends two of your accounts share, from their cached friend lists. */
export function CompareFriendsDialog(props: { a: number | null; b: number | null; onDone: () => void }) {
  const accounts = createMemo(() => shell.accounts.filter((x) => x.id !== 0))
  const [pickedA, setA] = createSignal(props.a)
  const [pickedB, setB] = createSignal(props.b)
  const a = () => pickedA() ?? accounts()[0]?.id ?? null
  const b = () => {
    const chosen = pickedB()
    if (chosen !== null && chosen !== a()) return chosen
    return accounts().find((x) => x.id !== a())?.id ?? null
  }
  const [query, setQuery] = createSignal("")
  const view = useLive(
    () => [a(), b()] as const,
    ([x, y]) => (x === null || y === null ? Promise.resolve(empty()) : commands.sharedFriends(x, y)),
    empty,
  )
  const name = (id: number | null) => {
    const acc = accounts().find((x) => x.id === id)
    return acc ? accountName(acc) : ""
  }
  const options = () => accounts().map((x) => ({ value: String(x.id), label: accountName(x) }))
  const shown = createMemo(() => filterPeople(view.friends, query()))
  const open = (id: number) => {
    props.onDone()
    void run("user.open", String(id))
  }
  return (
    <div class="flex flex-col gap-3 p-4">
      <Show
        when={accounts().length > 1}
        fallback={<EmptyState icon="user" title="Only one account" description="Add another account to compare." />}
      >
        <div class="grid grid-cols-2 gap-2">
          <Select
            label="Account"
            options={options()}
            value={a() === null ? undefined : String(a())}
            onChange={(v) => setA(Number(v))}
          />
          <Select
            label="Compare with"
            options={options().filter((o) => o.value !== String(a()))}
            value={b() === null ? undefined : String(b())}
            onChange={(v) => setB(Number(v))}
          />
        </div>
        <p class="text-sm text-fg-muted" aria-live="polite">
          {compareSummary(view, name(a()), name(b()))}
        </p>
        <TextField
          aria-label="Filter shared friends"
          placeholder="Filter shared friends"
          icon="search"
          size="sm"
          value={query()}
          onChange={setQuery}
        />
        <VirtualList
          class="-mx-2 h-72"
          items={shown()}
          itemKey={(p) => p.id}
          estimateRowHeight={48}
          empty={
            <EmptyState
              icon="user"
              title={view.loading ? "Loading…" : query().trim() ? "No matching friends" : "No shared friends"}
            />
          }
        >
          {(p) => (
            <ContextMenu items={userMenu(p.id, { friend: true })}>
              <ListItem
                leading={<Thumbnail class="w-8" alt={p.displayName || p.name} src={thumbUrl("headshot", p.id, true)} />}
                title={p.displayName || p.name || `User ${p.id}`}
                subtitle={p.name ? `@${p.name}` : undefined}
                onClick={() => open(p.id)}
              />
            </ContextMenu>
          )}
        </VirtualList>
      </Show>
    </div>
  )
}

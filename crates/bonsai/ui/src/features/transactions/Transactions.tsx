import { createMemo, createResource, createSignal, For, type JSX, onCleanup, Show } from "solid-js"
import { createStore, unwrap } from "solid-js/store"
import { useCommandContext, useListCursor, useSubTabs } from "~/lib/commands/hooks"
import { handle, run } from "~/lib/commands/registry"
import { ago, date, dateTime, full } from "~/lib/format"
import {
  type AccountTotalsView,
  commands,
  type GroupView,
  type TotalsView,
  type TxInsights,
  type TxQuery,
  type TxRowView,
  type TxView,
} from "~/lib/ipc/bindings"
import { gameMenu, itemMenu, itemTarget, userMenu } from "~/lib/menus"
import { useLive } from "~/lib/store"
import { thumbUrl } from "~/lib/thumb"
import {
  Alert,
  Avatar,
  Badge,
  Button,
  Card,
  type Column,
  ContextMenu,
  DataTable,
  EmptyState,
  Page,
  Robux,
  SectionHeader,
  SegmentedControl,
  Select,
  Skeleton,
  Spinner,
  Stat,
  Switch,
  showToast,
  TabPanel,
  Tabs,
  TextField,
  TextLink,
  Thumbnail,
} from "~/lib/ui"
import {
  barWidths,
  dayStart,
  dayString,
  matchKind,
  matchScope,
  monthLabel,
  nextPeriod,
  parseRange,
  periodLabel,
  periods,
  sitePeriod,
} from "./query"

const PAGE = 300

const sections = [
  { value: "ledger", label: "Ledger" },
  { value: "insights", label: "Insights" },
  { value: "accounts", label: "All accounts" },
]

function blankTotals(): TotalsView {
  return { loading: false, error: null, incoming: 0, outgoing: 0, pending: 0, byKind: [] }
}

function blankView(scope: string): TxView {
  return {
    signedIn: true,
    scope,
    balance: null,
    syncing: true,
    loadingOlder: false,
    hasOlder: true,
    syncedAt: null,
    oldest: null,
    error: null,
    ledgerCount: 0,
    kinds: [],
    roblox: null,
    sums: blankTotals(),
    matched: 0,
    rows: [],
  }
}

/** Robux in and out for you or a community: Roblox's period totals, a local ledger that keeps
 * every row it has seen (so All and custom ranges work), filters, CSV, and insights the site
 * doesn't have. */
export default function Transactions() {
  const [q, setQ] = createStore<Required<TxQuery>>({
    scope: "me",
    period: "month",
    from: null,
    to: null,
    kind: "",
    text: "",
    minAmount: null,
    pendingOnly: false,
    limit: PAGE,
  })
  const key = () => JSON.stringify(q)
  // Keep the last view while a filter changes so the table doesn't blank on every keystroke.
  let last: TxView | undefined
  const view = useLive(
    key,
    () => commands.transactionsView({ ...q }),
    () => (last ? structuredClone(unwrap(last)) : blankView(q.scope)),
  )
  last = view
  const [communities] = createResource(async () => (await commands.myCommunities()).items)
  const [section, setSection] = createSignal("ledger")
  useSubTabs(() => sections, section, setSection)
  useCommandContext(() => `transactions:${key()}`)

  const set = (patch: Partial<TxQuery>) => setQ({ ...patch, limit: PAGE })
  const setPeriod = (p: string) => {
    if (p !== "custom") return set({ period: p, from: null, to: null })
    const to = Date.now()
    set({ period: "custom", from: q.from ?? dayStart(dayString(to - 30 * 86_400_000)), to: q.to })
  }
  const scopes = createMemo(() => [
    { value: "me", label: "You" },
    ...(communities() ?? []).map((c) => ({ value: `group:${c.id}`, label: c.name })),
  ])

  const off = [
    handle("transactions.period", (a) => {
      const p = a?.trim().toLowerCase()
      if (!p) return setPeriod(nextPeriod(q.period))
      if (periods.some((x) => x.value === p)) return setPeriod(p)
      showToast("Period is one of: day, week, month, year, all, custom")
    }),
    handle("transactions.type", (a) => {
      const k = matchKind(a ?? "", view.kinds)
      if (k === null) showToast(`Type is one of: ${view.kinds.map((x) => x.key).join(", ")}`)
      else set({ kind: k })
    }),
    handle("transactions.scope", (a) => {
      const s = matchScope(a ?? "", communities() ?? [])
      if (s === null) showToast(`No community of yours matches "${a}"`)
      else set({ scope: s, kind: "" })
    }),
    handle("transactions.range", (a) => {
      if (!a?.trim()) return set({ period: "month", from: null, to: null })
      const r = parseRange(a)
      if (!r) showToast("Write a range like 2026-01-01..2026-03-31")
      else set({ period: "custom", ...r })
    }),
    handle("transactions.pending", (a) => {
      const v = a?.trim().toLowerCase()
      set({ pendingOnly: v ? v === "on" || v === "yes" || v === "true" : !q.pendingOnly })
    }),
  ]
  onCleanup(() => {
    for (const f of off) f()
  })

  const [now, setNow] = createSignal(Date.now())
  const clock = setInterval(() => setNow(Date.now()), 15_000)
  onCleanup(() => clearInterval(clock))

  return (
    <Page
      title="Transactions"
      width="wide"
      meta={<SyncLine view={view} now={now()} />}
      actions={
        <div class="flex items-center gap-1">
          <Button
            size="sm"
            variant="ghost"
            icon="refresh"
            loading={view.syncing}
            onClick={() => run("transactions.sync")}
          >
            Sync
          </Button>
          <Button size="sm" variant="ghost" icon="download" onClick={() => run("transactions.export")}>
            Save CSV
          </Button>
          <Button size="sm" variant="ghost" icon="copy" onClick={() => run("transactions.copy_csv")}>
            Copy CSV
          </Button>
        </div>
      }
    >
      <Show
        when={view.signedIn}
        fallback={<EmptyState icon="dollar" title="No account" description={view.error ?? undefined} />}
      >
        {/* What the page shows: whose money, over which period. Every tab below follows it. */}
        <div class="flex flex-wrap items-center gap-2">
          <Select
            class="w-44"
            aria-label="Whose transactions"
            options={scopes()}
            value={q.scope}
            onChange={(s) => set({ scope: s, kind: "" })}
          />
          <SegmentedControl aria-label="Period" options={periods} value={q.period} onChange={setPeriod} />
          <Show when={q.period === "custom"}>
            <div class="flex items-center gap-2">
              <TextField
                type="date"
                aria-label="From"
                value={q.from === null ? "" : dayString(q.from)}
                onChange={(v) => set({ from: dayStart(v) })}
              />
              <span class="text-sm text-fg-muted">to</span>
              <TextField
                type="date"
                aria-label="To"
                value={q.to === null ? "" : dayString(q.to - 86_400_000)}
                onChange={(v) => {
                  const d = dayStart(v)
                  set({ to: d === null ? null : d + 86_400_000 })
                }}
              />
            </div>
          </Show>
        </div>
        <Tabs aria-label="Transactions view" items={sections} value={section()} onChange={setSection}>
          <TabPanel value="ledger">
            <div class="flex flex-col gap-4">
              <Summary view={view} query={q} onKind={(k) => set({ kind: k === q.kind ? "" : k })} />
              <Coverage view={view} query={q} />
              <Ledger view={view} query={q} set={set} more={() => setQ("limit", (l) => l + PAGE * 2)} />
            </div>
          </TabPanel>
          <TabPanel value="insights">
            <div class="flex flex-col gap-4">
              <Summary view={view} query={q} onKind={(k) => set({ kind: k === q.kind ? "" : k })} />
              <Coverage view={view} query={q} />
              <Insights query={q} />
            </div>
          </TabPanel>
          <TabPanel value="accounts">
            <Accounts period={sitePeriod(q.period) ? q.period : "month"} />
          </TabPanel>
        </Tabs>
      </Show>
    </Page>
  )
}

function SyncLine(props: { view: TxView; now: number }) {
  return (
    <span class="flex items-center gap-2 text-sm text-fg-muted">
      <Show when={props.view.syncing}>
        <Spinner size="sm" />
      </Show>
      <Show
        when={props.view.syncedAt}
        fallback={
          <Show when={props.view.syncing} fallback={<span>Not synced yet</span>}>
            <span>Syncing</span>
          </Show>
        }
      >
        {(t) => <span>Synced {ago(t(), props.now)}</span>}
      </Show>
      <span>· {full(props.view.ledgerCount)} saved</span>
      <Show when={props.view.oldest}>{(o) => <span>· back to {date(o())}</span>}</Show>
    </span>
  )
}

/** Roblox's totals for site periods, else sums of the ledger rows in range. A type tile
 * filters the table to that type. */
function Summary(props: { view: TxView; query: Required<TxQuery>; onKind: (k: string) => void }) {
  const filtered = () => !!(props.query.kind || props.query.text || props.query.minAmount || props.query.pendingOnly)
  // Filters only apply to the ledger, so show its sums when one is on.
  const t = () => (filtered() ? props.view.sums : (props.view.roblox ?? props.view.sums))
  const fromRoblox = () => !filtered() && props.view.roblox !== null
  const net = () => (t().incoming ?? 0) + (t().outgoing ?? 0)
  const covers = () =>
    periodLabel(props.query.period) ??
    `${props.query.from === null ? "Start" : date(props.query.from)} – ${props.query.to === null ? "today" : date(props.query.to - 86_400_000)}`
  const total = (label: string, amount: () => number | null, cls: () => string) => (
    <Stat size="lg" label={label}>
      <Show when={!t().loading} fallback={<Skeleton class="mt-1 h-5 w-20" />}>
        <Robux class={(amount() ?? 0) === 0 ? "text-fg-muted" : cls()} amount={amount()} />
      </Show>
    </Stat>
  )
  return (
    <Card class="flex flex-col">
      <Show when={t().error}>
        {(e) => (
          <div class="p-3 pb-0">
            <Alert variant="danger" title="Couldn't load Roblox's totals">
              {e()}
            </Alert>
          </div>
        )}
      </Show>
      <div class="grid grid-cols-[minmax(8rem,auto)_1px_repeat(4,minmax(0,1fr))] gap-x-6 p-4">
        <Stat size="lg" label={props.query.scope === "me" ? "Balance" : "Community funds"}>
          <Robux amount={props.view.balance} />
        </Stat>
        <div class="bg-line" />
        {total(
          "Incoming",
          () => t().incoming,
          () => "text-success",
        )}
        {total(
          "Outgoing",
          () => t().outgoing,
          () => "text-danger",
        )}
        {total("Net", net, () => (net() < 0 ? "text-danger" : "text-fg"))}
        {total(
          "Pending",
          () => t().pending,
          () => "text-warning",
        )}
      </div>
      <div class="flex flex-col gap-2 border-t border-line px-4 py-3">
        <div class="flex items-baseline justify-between gap-3 text-xs">
          <span class="font-medium text-fg-muted">By type · {covers()}</span>
          <span class="text-fg-faint">
            {fromRoblox() ? "Totals from Roblox" : "Sums of saved transactions that match"}
          </span>
        </div>
        <Show
          when={t().byKind.length > 0}
          fallback={<span class="text-sm text-fg-faint">Nothing in this period.</span>}
        >
          <div class="-mx-2 flex flex-wrap gap-1">
            <For each={t().byKind}>
              {(k) => (
                <Button
                  size="sm"
                  variant={props.query.kind === k.key ? "secondary" : "ghost"}
                  aria-pressed={props.query.kind === k.key}
                  title={props.query.kind === k.key ? "Show all types" : `Show only ${k.label.toLowerCase()}`}
                  onClick={() => props.onKind(k.key)}
                >
                  <span class="text-fg-muted">{k.label}</span>
                  <Robux amount={k.amount} compact />
                  <Show when={k.count !== null}>
                    <span class="text-fg-faint">×{full(k.count ?? 0)}</span>
                  </Show>
                </Button>
              )}
            </For>
          </div>
        </Show>
      </div>
    </Card>
  )
}

/** Ranges past what's saved need older pages first. */
function Coverage(props: { view: TxView; query: Required<TxQuery> }) {
  const needs = () =>
    props.view.hasOlder &&
    !sitePeriod(props.query.period) &&
    (props.query.from === null || props.view.oldest === null || props.query.from < props.view.oldest)
  return (
    <Show when={props.view.error || needs()}>
      <Alert
        variant={props.view.error ? "danger" : "info"}
        title={props.view.error ? "Sync stopped" : "Older history isn't saved yet"}
      >
        <div class="flex flex-wrap items-center gap-3">
          <span>
            {props.view.error ??
              `Saved history goes back to ${props.view.oldest ? date(props.view.oldest) : "nothing yet"}. Fetching older pages is slow: Roblox limits how fast they load.`}
          </span>
          <Show
            when={props.view.loadingOlder}
            fallback={
              <Button size="sm" icon="clock" onClick={() => run("transactions.load_older")}>
                Load older
              </Button>
            }
          >
            <Button size="sm" variant="ghost" icon="x" onClick={() => run("transactions.stop")}>
              Stop
            </Button>
          </Show>
        </div>
      </Alert>
    </Show>
  )
}

function itemOpen(r: TxRowView) {
  if (r.itemKind === "Asset" && r.itemId) return () => run("market.open", itemTarget(r.itemId ?? 0))
  if (r.itemKind === "Bundle" && r.itemId) return () => run("market.open", itemTarget(r.itemId ?? 0, true))
  if (r.placeId) return () => run("game.open", String(r.placeId))
  return null
}

function agentOpen(r: TxRowView) {
  if (!r.agentId) return null
  return r.agentGroup ? () => run("group.open", String(r.agentId)) : () => run("user.open", String(r.agentId))
}

function experienceOpen(r: TxRowView) {
  return r.placeId ? () => run("game.open", String(r.placeId)) : null
}

/** A name that opens what it names, or plain text when there's nothing to open. Clicks on it
 * don't move the table cursor. */
function Linked(props: { open: (() => void) | null; tone?: "name" | "muted"; children: JSX.Element }) {
  return (
    <Show when={props.open} fallback={<span class="truncate">{props.children}</span>}>
      {(open) => (
        <TextLink class="min-w-0 truncate" tone={props.tone ?? "name"} onClick={open()}>
          {props.children}
        </TextLink>
      )}
    </Show>
  )
}

function Amount(props: { amount: number | null; pending?: boolean; class?: string }) {
  return (
    <span class={`inline-flex items-center gap-1.5 tabular-nums ${props.class ?? ""}`}>
      <Show when={props.pending}>
        <Badge variant="warning">Pending</Badge>
      </Show>
      <Robux
        class={(props.amount ?? 0) < 0 ? "text-danger" : (props.amount ?? 0) > 0 ? "text-success" : ""}
        amount={props.amount}
      />
    </span>
  )
}

function Ledger(props: {
  view: TxView
  query: Required<TxQuery>
  set: (p: Partial<TxQuery>) => void
  more: () => void
}) {
  const [cursor, setCursor] = createSignal(0)
  const current = () => props.view.rows[cursor()]
  useListCursor({
    count: () => props.view.rows.length,
    index: cursor,
    setIndex: setCursor,
    onOpen: (i) => {
      const r = props.view.rows[i]
      if (r) itemOpen(r)?.()
    },
  })
  // Kobalte treats an empty key as no selection, so "All types" needs a real value.
  const ALL = "*"
  const kinds = () => [
    { value: ALL, label: "All types" },
    ...props.view.kinds.map((k) => ({ value: k.key, label: k.label })),
  ]

  const columns: Column<TxRowView>[] = [
    {
      key: "date",
      label: "Date",
      width: "6.5rem",
      cell: (r) => <span class="text-fg-muted tabular-nums">{date(r.at ?? 0)}</span>,
    },
    {
      key: "type",
      label: "Type",
      width: "minmax(7.5rem, 1fr)",
      cell: (r) => <span class="truncate">{r.kindLabel}</span>,
    },
    {
      key: "item",
      label: "Item",
      width: "minmax(8rem, 3fr)",
      cell: (r) => (
        <Show when={r.itemName} fallback={<span class="text-fg-faint">—</span>}>
          <ContextMenu
            items={r.itemKind === "Asset" && r.itemId ? itemMenu(r.itemId) : r.placeId ? gameMenu(r.placeId) : []}
          >
            <span class="flex min-w-0 items-center gap-2">
              <Show when={r.itemKind === "Asset" && r.itemId}>
                <Thumbnail
                  class="w-6 shrink-0 rounded-sm"
                  alt=""
                  src={thumbUrl("asset", r.itemId ?? 0)}
                  fallback="avatar"
                />
              </Show>
              <Linked open={itemOpen(r)}>{r.itemName}</Linked>
            </span>
          </ContextMenu>
        </Show>
      ),
    },
    {
      key: "experience",
      label: "Experience",
      width: "minmax(0, 2fr)",
      cell: (r) => (
        <Show when={r.experience} fallback={<span class="text-fg-faint">—</span>}>
          <ContextMenu items={r.placeId ? gameMenu(r.placeId) : []}>
            <Linked open={experienceOpen(r)} tone="muted">
              {r.experience}
            </Linked>
          </ContextMenu>
        </Show>
      ),
    },
    {
      key: "agent",
      label: "With",
      width: "minmax(0, 2fr)",
      cell: (r) => (
        <Show when={r.agentName} fallback={<span class="text-fg-faint">—</span>}>
          <ContextMenu items={r.agentId && !r.agentGroup ? userMenu(r.agentId) : []}>
            <Linked open={agentOpen(r)}>{r.agentName}</Linked>
          </ContextMenu>
        </Show>
      ),
    },
    {
      key: "amount",
      label: "Amount",
      width: "7rem",
      align: "end",
      cell: (r) => <Amount amount={r.amount} pending={r.pending} />,
    },
  ]

  return (
    <div class="flex flex-col gap-3">
      <div class="flex flex-wrap items-center gap-2">
        <Select
          aria-label="Type"
          options={kinds()}
          value={props.query.kind || ALL}
          onChange={(k) => props.set({ kind: k === ALL ? "" : k })}
        />
        <TextField
          class="min-w-48 flex-1"
          icon="search"
          aria-label="Search transactions"
          placeholder="Item, experience or person"
          value={props.query.text}
          onChange={(v) => {
            props.set({ text: v })
            setCursor(0)
          }}
        />
        <TextField
          class="w-36"
          type="number"
          min="0"
          aria-label="Minimum amount"
          placeholder="Min Robux"
          value={props.query.minAmount === null ? "" : String(props.query.minAmount)}
          onChange={(v) => props.set({ minAmount: v.trim() ? Math.max(0, Number(v) || 0) : null })}
        />
        <Switch checked={props.query.pendingOnly} onChange={(c) => props.set({ pendingOnly: c })}>
          Pending only
        </Switch>
        <span class="ml-auto text-sm text-fg-muted tabular-nums">
          {full(props.view.matched)} of {full(props.view.ledgerCount)}
        </span>
      </div>
      <div class="grid grid-cols-[minmax(0,1fr)_17rem] items-start gap-4">
        <div data-cursor-list class="flex min-w-0 flex-col gap-2">
          <DataTable
            aria-label="Transactions"
            columns={columns}
            rows={props.view.rows}
            rowKey={(r) => r.id}
            rowHeight={36}
            selected={current()?.id}
            onRowClick={(r) => setCursor(props.view.rows.indexOf(r))}
            empty={
              <Show when={!props.view.syncing} fallback={<Skeleton class="h-64 w-full rounded-md" />}>
                <EmptyState icon="dollar" title="No transactions match" />
              </Show>
            }
          />
          <Show when={props.view.matched > props.view.rows.length}>
            <Button variant="ghost" onClick={props.more}>
              Show more ({full(props.view.matched - props.view.rows.length)} left)
            </Button>
          </Show>
        </div>
        <Detail row={current()} />
      </div>
    </div>
  )
}

function Detail(props: { row: TxRowView | undefined }) {
  return (
    <Card class="sticky top-2 flex flex-col overflow-hidden">
      <Show
        when={props.row}
        fallback={<span class="p-4 text-sm text-fg-muted">Pick a transaction to see its details.</span>}
      >
        {(r) => (
          <>
            <div class="flex items-start gap-3 p-4 pb-3">
              <Show when={r().itemKind === "Asset" && r().itemId}>
                <Thumbnail
                  class="size-16 shrink-0 rounded-md"
                  alt=""
                  src={thumbUrl("asset", r().itemId ?? 0, true)}
                  fallback="marketplace"
                />
              </Show>
              <div class="flex min-w-0 flex-col gap-1">
                <span class="text-xs text-fg-muted">{r().kindLabel}</span>
                <span class="line-clamp-2 font-medium">
                  {r().itemName || r().experience || r().agentName || r().kindLabel}
                </span>
                <Amount class="text-xl font-semibold" amount={r().amount} pending={r().pending} />
              </div>
            </div>
            <dl class="grid grid-cols-[auto_minmax(0,1fr)] gap-x-3 gap-y-1.5 border-t border-line px-4 py-3 text-sm">
              <DetailRow label="When">{dateTime(r().at ?? 0)}</DetailRow>
              <Show when={r().itemName}>
                <DetailRow label={kindLabel(r().itemKind) || "Item"}>
                  <Linked open={itemOpen(r())}>{r().itemName}</Linked>
                </DetailRow>
              </Show>
              <Show when={r().experience}>
                <DetailRow label="Experience">
                  <Linked open={experienceOpen(r())}>{r().experience}</Linked>
                </DetailRow>
              </Show>
              <Show when={r().agentName}>
                <DetailRow label={r().agentGroup ? "Community" : "Person"}>
                  <Linked open={agentOpen(r())}>{r().agentName}</Linked>
                </DetailRow>
              </Show>
            </dl>
            <Show when={itemOpen(r()) || agentOpen(r())}>
              <div class="flex flex-col gap-1.5 border-t border-line p-3">
                <Show when={itemOpen(r())}>
                  {(open) => (
                    <Button
                      size="sm"
                      variant="secondary"
                      icon={r().placeId && r().itemKind !== "Asset" ? "game" : "marketplace"}
                      onClick={open()}
                    >
                      Open {r().itemKind === "Asset" || r().itemKind === "Bundle" ? "item" : "experience"}
                    </Button>
                  )}
                </Show>
                <Show when={agentOpen(r())}>
                  {(open) => (
                    <Button
                      size="sm"
                      variant="secondary"
                      icon={r().agentGroup ? "communities" : "user"}
                      onClick={open()}
                    >
                      {r().agentGroup ? "Open community" : "Open profile"}
                    </Button>
                  )}
                </Show>
              </div>
            </Show>
            <Show when={r().pending}>
              <span class="border-t border-line px-4 py-3 text-xs text-fg-muted">
                Pending Robux are held by Roblox before they reach the balance. Roblox doesn't say when.
              </span>
            </Show>
          </>
        )}
      </Show>
    </Card>
  )
}

function DetailRow(props: { label: string; children: JSX.Element }) {
  return (
    <>
      <dt class="text-fg-muted">{props.label}</dt>
      <dd class="min-w-0 text-end break-words">{props.children}</dd>
    </>
  )
}

/** "DeveloperProduct" → "Developer product". */
function kindLabel(kind: string) {
  return kind.replace(/([a-z])([A-Z])/g, (_, a: string, b: string) => `${a} ${b.toLowerCase()}`)
}

function Insights(props: { query: Required<TxQuery> }) {
  const insights = useLive(
    () => JSON.stringify({ ...props.query, limit: 0 }),
    () => commands.transactionInsights({ ...props.query }),
    (): TxInsights => ({ months: [], experiences: [], items: [], buyers: [] }),
  )
  const widths = createMemo(() => barWidths(insights.months.flatMap((m) => [m.incoming ?? 0, m.outgoing ?? 0])))
  return (
    <div class="flex flex-col gap-4">
      <Card class="flex flex-col gap-3 p-4">
        <SectionHeader
          size="group"
          title="By month"
          actions={
            <span class="flex items-center gap-3 text-xs text-fg-muted">
              <span class="flex items-center gap-1.5">
                <span class="size-2 rounded-full bg-success" />
                Incoming
              </span>
              <span class="flex items-center gap-1.5">
                <span class="size-2 rounded-full bg-danger" />
                Outgoing
              </span>
            </span>
          }
        />
        <Show
          when={insights.months.length > 0}
          fallback={<span class="text-sm text-fg-muted">Nothing saved in this range.</span>}
        >
          <div class="grid grid-cols-[5rem_minmax(0,1fr)_repeat(3,5.5rem)] items-center gap-x-4 gap-y-2.5 text-sm">
            <span />
            <span />
            <span class="text-end text-xs text-fg-faint">In</span>
            <span class="text-end text-xs text-fg-faint">Out</span>
            <span class="text-end text-xs text-fg-faint">Net</span>
            <For each={insights.months}>
              {(m, i) => {
                const net = () => (m.incoming ?? 0) + (m.outgoing ?? 0)
                return (
                  <>
                    <span class="text-fg-muted">{monthLabel(m.month)}</span>
                    <div class="flex flex-col gap-1">
                      <Bar width={widths()[i() * 2] ?? 0} tone="in" />
                      <Bar width={widths()[i() * 2 + 1] ?? 0} tone="out" />
                    </div>
                    <Robux class="justify-end text-success" amount={m.incoming} compact />
                    <Robux class="justify-end text-danger" amount={m.outgoing} compact />
                    <Robux class={`justify-end ${net() < 0 ? "text-danger" : "text-fg"}`} amount={net()} compact />
                  </>
                )
              }}
            </For>
          </div>
        </Show>
      </Card>
      <div class="grid grid-cols-[repeat(auto-fill,minmax(18rem,1fr))] gap-4">
        <Ranking
          title="Spending by experience"
          rows={insights.experiences}
          empty="No purchases in experiences."
          open={(g) => run("game.open", String(g.id))}
        />
        <Ranking
          title="Top items sold"
          rows={insights.items}
          empty="No sales."
          open={(g) => run("market.open", itemTarget(g.id))}
        />
        <Ranking
          title="Top buyers"
          rows={insights.buyers}
          empty="No sales."
          open={(g) => run(g.group ? "group.open" : "user.open", String(g.id))}
        />
      </div>
    </div>
  )
}

function Bar(props: { width: number; tone: "in" | "out" }) {
  return (
    <div class="h-1.5 rounded-full bg-control">
      <div
        class="h-1.5 rounded-full"
        classList={{ "bg-success": props.tone === "in", "bg-danger": props.tone === "out" }}
        style={{ width: `${props.width}%` }}
      />
    </div>
  )
}

function Ranking(props: { title: string; rows: GroupView[]; empty: string; open: (g: GroupView) => void }) {
  const widths = createMemo(() => barWidths(props.rows.map((r) => r.amount ?? 0)))
  return (
    <Card class="flex flex-col gap-1 p-3">
      <SectionHeader class="px-1 pb-1" size="group" title={props.title} />
      <Show when={props.rows.length > 0} fallback={<span class="text-sm text-fg-muted">{props.empty}</span>}>
        <For each={props.rows}>
          {(g, i) => (
            <button
              type="button"
              class="flex flex-col gap-1.5 rounded-md px-1 py-1.5 text-start text-sm hover:bg-hover"
              onClick={() => props.open(g)}
            >
              <span class="flex items-center justify-between gap-2">
                <span class="truncate">{g.name}</span>
                <span class="flex shrink-0 items-center gap-2">
                  <span class="text-fg-faint">×{full(g.count)}</span>
                  <Robux amount={Math.abs(g.amount ?? 0)} compact />
                </span>
              </span>
              <Bar width={widths()[i()] ?? 0} tone={(g.amount ?? 0) < 0 ? "out" : "in"} />
            </button>
          )}
        </For>
      </Show>
    </Card>
  )
}

/** Every account's balance and Roblox's totals for one period, side by side. */
function Accounts(props: { period: string }) {
  const rows = useLive(
    () => props.period,
    async (p) => ({ list: await commands.transactionAccounts(p) }),
    () => ({ list: [] as AccountTotalsView[] }),
  )
  const sum = (f: (a: AccountTotalsView) => number | null) => rows.list.reduce((s, a) => s + (f(a) ?? 0), 0)
  const cell =
    (pick: (t: TotalsView) => number | null, cls: string) =>
    (a: AccountTotalsView): JSX.Element => (
      <Show when={!a.totals.loading} fallback={<Skeleton class="h-4 w-16" />}>
        <Show when={!a.totals.error} fallback={<span class="text-danger text-sm">Failed</span>}>
          <Robux class={(pick(a.totals) ?? 0) === 0 ? "text-fg-muted" : cls} amount={pick(a.totals)} />
        </Show>
      </Show>
    )
  const columns: Column<AccountTotalsView>[] = [
    {
      key: "name",
      label: "Account",
      width: "minmax(10rem, 2fr)",
      cell: (a) => (
        <span class="flex min-w-0 items-center gap-2">
          <Avatar size="sm" name={a.name} src={thumbUrl("headshot", a.id)} />
          <span class="truncate">{a.name}</span>
        </span>
      ),
    },
    { key: "balance", label: "Balance", width: "8rem", align: "end", cell: (a) => <Robux amount={a.robux} /> },
    { key: "in", label: "Incoming", width: "8rem", align: "end", cell: cell((t) => t.incoming, "text-success") },
    { key: "out", label: "Outgoing", width: "8rem", align: "end", cell: cell((t) => t.outgoing, "text-danger") },
    { key: "pending", label: "Pending", width: "8rem", align: "end", cell: cell((t) => t.pending, "text-warning") },
  ]
  return (
    <div class="flex flex-col gap-3">
      <Card class="grid grid-cols-[minmax(8rem,auto)_1px_repeat(4,minmax(0,1fr))] gap-x-6 p-4">
        <Stat size="lg" label="Total balance">
          <Robux amount={sum((a) => a.robux)} />
        </Stat>
        <div class="bg-line" />
        <Stat size="lg" label="Accounts">
          {full(rows.list.length)}
        </Stat>
        <Stat size="lg" label="Incoming">
          <Robux class="text-success" amount={sum((a) => a.totals.incoming)} />
        </Stat>
        <Stat size="lg" label="Outgoing">
          <Robux class="text-danger" amount={sum((a) => a.totals.outgoing)} />
        </Stat>
        <Stat size="lg" label="Pending">
          <Robux
            class={sum((a) => a.totals.pending) === 0 ? "text-fg-muted" : "text-warning"}
            amount={sum((a) => a.totals.pending)}
          />
        </Stat>
      </Card>
      <DataTable
        aria-label="Accounts"
        columns={columns}
        rows={rows.list}
        rowKey={(a) => a.id}
        rowHeight={36}
        empty={<EmptyState icon="user" title="No accounts" />}
      />
      <span class="text-xs text-fg-faint">
        {periodLabel(props.period)} for each account, from Roblox. Custom and All show the past month here.
      </span>
    </div>
  )
}

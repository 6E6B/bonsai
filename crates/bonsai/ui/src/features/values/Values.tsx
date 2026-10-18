import { createEffect, createMemo, createResource, createSignal, on, onCleanup, Show } from "solid-js"
import { useListCursor } from "~/lib/commands/hooks"
import { run } from "~/lib/commands/registry"
import { ago, full } from "~/lib/format"
import { commands, type LimitedVersion, type ValueRow, type ValuesStatus } from "~/lib/ipc/bindings"
import { itemMenu, itemTarget } from "~/lib/menus"
import { shell, useLive } from "~/lib/store"
import { thumbUrl } from "~/lib/thumb"
import {
  Alert,
  Badge,
  Button,
  Card,
  type Column,
  ContextMenu,
  DataTable,
  EmptyState,
  Page,
  Robux,
  SegmentedControl,
  Select,
  showToast,
  Skeleton,
  Stat,
  Switch,
  TextField,
  Thumbnail,
} from "~/lib/ui"
import { DemandPips, TrendMark, useValuesRefresh, ValueAmount, ValueFlags } from "~/lib/value-tags"
import { rapGap, rolimonsFailed, shapeValues, VALUES_SOURCE, type ValueShow, type ValueSort, versionLabel } from "~/lib/values"

const versions = [
  { value: "all", label: "All" },
  { value: "classic", label: "Limited 1.0" },
  { value: "new", label: "Limited 2.0" },
]

const sorts: { value: ValueSort; label: string }[] = [
  { value: "worth", label: "Highest value" },
  { value: "rap", label: "Highest RAP" },
  { value: "demand", label: "Best demand" },
  { value: "gap", label: "RAP furthest from value" },
  { value: "name", label: "Name" },
]

const shows: { value: ValueShow; label: string }[] = [
  { value: "all", label: "Everything" },
  { value: "valued", label: "Valued only" },
  { value: "unprojected", label: "Hide projected" },
  { value: "projected", label: "Projected only" },
  { value: "rare", label: "Rare only" },
  { value: "hyped", label: "Hyped only" },
]

/** Every limited Rolimons tracks: value, RAP, demand and trend, searchable and sortable. Kept
 * fresh every two minutes while open. */
export default function Values() {
  useValuesRefresh()
  const status = useLive(
    () => shell.itemValues,
    commands.valuesStatus,
    (): ValuesStatus => ({ enabled: false, loading: true, error: null, fetchedAt: null, count: 0 }),
  )
  // The table is large, so load it only when a new copy arrives, not on every store change.
  const [table] = createResource(
    () => (status.enabled ? status.fetchedAt : null),
    () => commands.valuesTable(),
  )
  createEffect(
    on(
      () => rolimonsFailed(status.enabled, status.error, status.count > 0),
      (failed) => {
        if (failed) showToast({ title: "Failed to get Rolimons data", variant: "error" })
      },
    ),
  )
  const [now, setNow] = createSignal(Date.now())
  const clock = setInterval(() => setNow(Date.now()), 15_000)
  onCleanup(() => clearInterval(clock))

  return (
    <Page
      title="Values"
      meta={
        <Show when={status.fetchedAt}>
          {(t) => (
            <span class="text-sm text-fg-muted">
              {VALUES_SOURCE} · updated {ago(t(), now())}
            </span>
          )}
        </Show>
      }
      actions={
        <Switch checked={shell.itemValues} onChange={() => run("market.values")}>
          Show values
        </Switch>
      }
    >
      <Show
        when={shell.itemValues}
        fallback={
          <EmptyState
            icon="values"
            title="Item values are off"
            description={`Values, demand and trends come from ${VALUES_SOURCE}, a third-party site. With values on, the app asks it for fresh ones every two minutes while a screen that shows them is open.`}
          >
            <Button variant="primary" onClick={() => run("market.values")}>
              Turn on values
            </Button>
          </EmptyState>
        }
      >
        <Show when={rolimonsFailed(status.enabled, status.error, status.count > 0)}>
          <Alert variant="danger" title={`Couldn't load values from ${VALUES_SOURCE}`} />
        </Show>
        <Show
          when={table()?.length}
          fallback={
            <Show when={!status.error}>
              <Skeleton class="h-20 w-full rounded-md" />
              <Skeleton class="h-96 w-full rounded-md" />
            </Show>
          }
        >
          <Browser rows={table() ?? []} />
        </Show>
      </Show>
    </Page>
  )
}

function Browser(props: { rows: ValueRow[] }) {
  const [query, setQuery] = createSignal("")
  const [version, setVersion] = createSignal<LimitedVersion | "all">("all")
  const [show, setShow] = createSignal<ValueShow>("all")
  const [sort, setSort] = createSignal<ValueSort>("worth")
  const [cursor, setCursor] = createSignal(0)
  const shown = createMemo(() =>
    shapeValues(props.rows, { query: query(), version: version(), show: show(), sort: sort() }),
  )
  const totals = createMemo(() => {
    let valued = 0
    let projected = 0
    let rare = 0
    let next = 0
    for (const r of props.rows) {
      if (r.tag.value !== null) valued++
      if (r.tag.projected) projected++
      if (r.tag.rare) rare++
      if (r.version === "new") next++
    }
    return { valued, projected, rare, next }
  })
  const open = (r: ValueRow | undefined) => r && run("market.open", itemTarget(r.id))
  useListCursor({
    count: () => shown().length,
    index: cursor,
    setIndex: setCursor,
    onOpen: (i) => open(shown()[i]),
  })

  const columns: Column<ValueRow>[] = [
    {
      key: "item",
      label: "Item",
      width: "minmax(10rem, 3fr)",
      cell: (r) => (
        <ContextMenu items={itemMenu(r.id)}>
          <span class="flex min-w-0 items-center gap-2.5">
            <Thumbnail class="w-7 shrink-0 rounded-sm" alt="" src={thumbUrl("asset", r.id)} fallback="avatar" />
            <span class="truncate">{r.name}</span>
            <Show when={r.acronym}>
              <Badge class="shrink-0">{r.acronym}</Badge>
            </Show>
          </span>
        </ContextMenu>
      ),
    },
    {
      key: "value",
      label: "Value",
      width: "7.5rem",
      align: "end",
      cell: (r) => (
        <Show when={r.tag.value} fallback={<span class="text-fg-faint">—</span>}>
          {(v) => <ValueAmount class="font-medium" amount={v()} />}
        </Show>
      ),
    },
    {
      key: "rap",
      label: "RAP",
      width: "7.5rem",
      align: "end",
      cell: (r) => <Robux class="text-fg-muted" amount={r.rap} />,
    },
    {
      key: "gap",
      label: "RAP vs value",
      width: "6.5rem",
      align: "end",
      cell: (r) => <Gap pct={rapGap(r.tag.value, r.rap)} />,
    },
    {
      key: "demand",
      label: "Demand",
      width: "7.5rem",
      cell: (r) => (
        <Show when={r.tag.demand} fallback={<span class="text-fg-faint">—</span>}>
          {(d) => <DemandPips demand={d()} label />}
        </Show>
      ),
    },
    {
      key: "trend",
      label: "Trend",
      width: "7.5rem",
      cell: (r) => (
        <Show when={r.tag.trend} fallback={<span class="text-fg-faint">—</span>}>
          {(t) => <TrendMark trend={t()} label />}
        </Show>
      ),
    },
    {
      key: "flags",
      label: "",
      width: "minmax(0, 1fr)",
      cell: (r) => (
        <span class="flex gap-1">
          <ValueFlags tag={r.tag} />
          <Show when={r.version === "new"}>
            <Badge>2.0</Badge>
          </Show>
        </span>
      ),
    },
  ]

  return (
    <div class="flex flex-col gap-3">
      <Card class="grid grid-cols-[repeat(auto-fill,minmax(8rem,1fr))] gap-4 p-3">
        <Stat label="Tracked">{full(props.rows.length)}</Stat>
        <Stat label="Valued">{full(totals().valued)}</Stat>
        <Stat label={versionLabel("new") ?? ""}>{full(totals().next)}</Stat>
        <Stat label="Rare">{full(totals().rare)}</Stat>
        <Stat label="Projected">
          <span classList={{ "text-warning": totals().projected > 0 }}>{full(totals().projected)}</span>
        </Stat>
      </Card>
      <div class="flex flex-wrap items-center gap-2">
        <TextField
          class="min-w-56 flex-1"
          icon="search"
          aria-label="Search values"
          placeholder="Name, acronym or ID"
          value={query()}
          onChange={(v) => {
            setQuery(v)
            setCursor(0)
          }}
        />
        <SegmentedControl
          aria-label="Limited version"
          options={versions}
          value={version()}
          onChange={(v) => setVersion(v as LimitedVersion | "all")}
        />
        <Select aria-label="Show" options={shows} value={show()} onChange={(v) => setShow(v as ValueShow)} />
        <Select aria-label="Sort" options={sorts} value={sort()} onChange={(v) => setSort(v as ValueSort)} />
      </div>
      <span class="text-sm text-fg-muted">
        {full(shown().length)} of {full(props.rows.length)}
      </span>
      <div data-cursor-list>
        <DataTable
          aria-label="Item values"
          columns={columns}
          rows={shown()}
          rowKey={(r) => r.id}
          rowHeight={40}
          selected={shown()[cursor()]?.id}
          onRowClick={open}
          empty={<EmptyState icon="search" title="No items match" />}
        />
      </div>
    </div>
  )
}

/** How far RAP sits from the value: above in amber (likely to fall back), below in green (a
 * buy under value). */
function Gap(props: { pct: number | null }) {
  return (
    <Show when={props.pct !== null} fallback={<span class="text-fg-faint">—</span>}>
      <span
        class="tabular-nums"
        classList={{
          "text-warning": (props.pct ?? 0) > 10,
          "text-success": (props.pct ?? 0) < -10,
          "text-fg-muted": Math.abs(props.pct ?? 0) <= 10,
        }}
      >
        {(props.pct ?? 0) > 0 ? "+" : ""}
        {props.pct}%
      </span>
    </Show>
  )
}

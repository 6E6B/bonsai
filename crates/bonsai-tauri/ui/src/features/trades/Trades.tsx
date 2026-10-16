import { createEffect, createSignal, For, Show } from "solid-js"
import { useCommandContext, useListCursor, useSubTabs } from "~/lib/commands/hooks"
import { run } from "~/lib/commands/registry"
import { ago, date } from "~/lib/format"
import {
  commands,
  type DraftView,
  type Eligibility,
  type EvalView,
  type Paged,
  type SideView,
  type TradeItemView,
  type TradeRowView,
  type TradeView,
} from "~/lib/ipc/bindings"
import { emptyList, ListState } from "~/lib/list-state"
import { itemMenu, itemTarget } from "~/lib/menus"
import { shell, useLive } from "~/lib/store"
import { thumbUrl } from "~/lib/thumb"
import {
  Alert,
  Avatar,
  Badge,
  Button,
  Card,
  ContextMenu,
  EmptyState,
  Icon,
  IconButton,
  ListItem,
  Pane,
  Robux,
  SectionHeader,
  Skeleton,
  Split,
  showToast,
  Tabs,
  TextField,
  TextLink,
  Thumbnail,
  Tooltip,
  usePrompt,
} from "~/lib/ui"
import { useValuesRefresh, ValueAmount, ValueLine } from "~/lib/value-tags"
import { hasTag, worth } from "~/lib/values"

const sections = [
  { value: "inbound", label: "Inbound" },
  { value: "outbound", label: "Outbound" },
  { value: "completed", label: "Completed" },
  { value: "inactive", label: "Inactive" },
]

const ms = (iso: string) => {
  const t = Date.parse(iso)
  return Number.isNaN(t) ? null : t
}

/** Trades: a list per status on the left, the open trade (or the trade you're making) on the
 * right with both sides, their values and the actions that fit. */
export default function Trades() {
  const [section, setSection] = createSignal("inbound")
  const [cursor, setCursor] = createSignal(0)
  const prompt = usePrompt()
  useValuesRefresh()
  const pick = (v: string) => {
    setSection(v)
    setCursor(0)
  }
  useSubTabs(() => sections, section, pick)
  useCommandContext(() => `trades:${section()}`)
  const rows = useLive(
    () => [shell.active, section()] as const,
    ([, s]) => commands.tradeList(s),
    (): Paged<TradeRowView> => ({ list: emptyList<TradeRowView>(), moreKey: "" }),
  )
  const draft = useLive(
    () => shell.active,
    async () => ({ d: await commands.tradeDraft() }),
    () => ({ d: null as DraftView | null }),
  )
  const can = useLive(
    () => shell.active,
    commands.tradeEligibility,
    (): Eligibility => ({ loading: true, canTrade: true, reason: "" }),
  )
  const open = () => rows.list.items[cursor()]
  useListCursor({ count: () => rows.list.items.length, index: cursor, setIndex: setCursor })

  return (
    <Split>
      <Pane side class="w-88">
        <SectionHeader
          title="Trades"
          size="page"
          actions={
            <>
              <Tooltip content="Robux transactions">
                <IconButton icon="transactions" label="Robux transactions" onClick={() => run("nav.transactions")} />
              </Tooltip>
              <Tooltip content="Copy as CSV">
                <IconButton icon="copy" label="Copy as CSV" onClick={() => run("trade.export")} />
              </Tooltip>
              <Button
                icon="plus"
                onClick={async () => {
                  const user = await prompt({ title: "New trade", label: "Username or ID", submit: "Start" })
                  if (user) void run("trade.new", user)
                }}
              >
                New
              </Button>
            </>
          }
        />
        <Show when={!can.loading && !can.canTrade && can.reason}>
          <Alert variant="warning" title="Can't trade">
            {can.reason}
          </Alert>
        </Show>
        <Tabs aria-label="Trade status" items={sections} value={section()} onChange={pick} />
        <ListState list={rows.list} icon="trades" empty={`No ${section()} trades`} more={rows.moreKey}>
          <div data-cursor-list class="flex flex-col">
            <For each={rows.list.items}>
              {(t, i) => (
                <ListItem
                  data-index={i()}
                  selected={!draft.d && i() === cursor()}
                  leading={<Avatar name={t.partnerDisplay} src={thumbUrl("headshot", t.partnerId)} />}
                  title={t.partnerDisplay || t.partnerName}
                  subtitle={`${t.status} · ${ms(t.created) ? ago(ms(t.created) ?? 0) : ""}`}
                  onClick={() => setCursor(i())}
                />
              )}
            </For>
          </div>
        </ListState>
      </Pane>

      <Pane class="@container">
        <Show
          when={draft.d}
          fallback={
            <Show when={open()} fallback={<EmptyState class="mt-16" icon="trades" title="No trade open" />}>
              {(t) => <Detail row={t()} box={section()} />}
            </Show>
          }
        >
          {(d) => <Composer draft={d()} />}
        </Show>
      </Pane>
    </Split>
  )
}

function Detail(props: { row: TradeRowView; box: string }) {
  const t = useLive(
    () => props.row.id,
    commands.tradeDetail,
    (): TradeView => ({
      loading: true,
      error: null,
      status: "",
      partnerId: null,
      give: null,
      get: null,
      eval: null,
    }),
  )
  const id = () => String(props.row.id)
  return (
    <div class="flex max-w-4xl flex-col gap-4">
      <div class="flex flex-col gap-3 @2xl:flex-row @2xl:items-center">
        <div class="flex min-w-0 items-center gap-3">
          <Avatar size="lg" name={props.row.partnerDisplay} src={thumbUrl("headshot", props.row.partnerId)} />
          <div class="flex min-w-0 flex-col">
            <TextLink tone="name" onClick={() => run("user.open", String(props.row.partnerId))}>
              {props.row.partnerDisplay || props.row.partnerName}
            </TextLink>
            <span class="text-sm text-fg-muted">
              {t.status || props.row.status}
              <Show when={ms(props.row.created)}>{(c) => <> · Sent {ago(c())}</>}</Show>
              <Show when={props.box === "inbound" || props.box === "outbound" ? ms(props.row.expires) : null}>
                {(e) => <> · Expires {date(e())}</>}
              </Show>
            </span>
          </div>
        </div>
        {/* Under the name in a narrow pane, at the far end of the row in a wide one. */}
        <div class="flex gap-1.5 empty:hidden @2xl:ml-auto">
          <Show when={props.box === "inbound"}>
            <Button onClick={() => run("trade.decline", id())}>Decline</Button>
            <Button onClick={() => run("trade.counter", id())}>Counter</Button>
            <Button variant="primary" onClick={() => run("trade.accept", id())}>
              Accept
            </Button>
          </Show>
          <Show when={props.box === "outbound"}>
            <Button variant="danger" onClick={() => run("trade.decline", id())}>
              Cancel trade
            </Button>
          </Show>
        </div>
      </div>
      <Show when={t.give && t.get} fallback={<TradeSkeleton error={t.error} />}>
        <div class="grid grid-cols-1 gap-4 @xl:grid-cols-2">
          <Side title="You give" side={t.give as SideView} />
          <Side title="You get" side={t.get as SideView} />
        </div>
        <Show when={t.eval}>{(e) => <Summary eval={e()} />}</Show>
      </Show>
    </div>
  )
}

function TradeSkeleton(props: { error: string | null }) {
  return (
    <Show when={props.error} fallback={<Skeleton class="h-48 w-full rounded-md" />}>
      {(e) => (
        <Alert variant="danger" title="Couldn't load this trade">
          {e()}
        </Alert>
      )}
    </Show>
  )
}

/** One side of a trade: its items as rows with serial and RAP, then any Robux. `onRemove`
 * shows a remove button per item (the composer). */
function Side(props: { title: string; side: SideView; onRemove?: (i: TradeItemView) => void }) {
  const rap = () => props.side.items.reduce((n, i) => n + (i.rap ?? 0), 0)
  // Shown once any item has a Rolimons value; items without one count at RAP, as in the totals.
  const value = () =>
    props.side.items.some((i) => i.values?.value)
      ? props.side.items.reduce((n, i) => n + (worth(i.values, i.rap) ?? 0), 0)
      : null
  return (
    <Card class="flex flex-col gap-1 p-2">
      <SectionHeader
        class="px-2 pb-1"
        title={props.title}
        actions={
          <span class="flex items-center gap-3">
            <Show when={value()}>{(v) => <ValueAmount class="font-medium" amount={v()} />}</Show>
            <Robux class="font-medium" amount={rap() + props.side.robux} />
          </span>
        }
      />
      <For each={props.side.items} fallback={<span class="px-2 py-2 text-sm text-fg-muted">No items</span>}>
        {(i) => <ItemRow item={i} onRemove={props.onRemove} />}
      </For>
      <Show when={props.side.robux > 0}>
        <ListItem
          leading={
            <span class="flex w-10 justify-center text-fg-muted">
              <Icon name="robux" size={20} />
            </span>
          }
          title="Robux"
          meta={<Robux amount={props.side.robux} />}
        />
      </Show>
    </Card>
  )
}

function ItemRow(props: {
  item: TradeItemView
  onRemove?: (i: TradeItemView) => void
  onClick?: () => void
  selected?: boolean
}) {
  const i = () => props.item
  return (
    <ContextMenu items={itemMenu(i().id, i().bundle)}>
      <ListItem
        selected={props.selected}
        leading={
          <Thumbnail class="w-10" alt="" src={thumbUrl(i().bundle ? "bundle" : "asset", i().id)} fallback="avatar" />
        }
        title={i().name}
        subtitle={
          <span class="flex items-center gap-1.5">
            <Show when={i().serial}>{(n) => `#${n()}`}</Show>
            <Show when={hasTag(i().values) && i().values}>{(t) => <ValueLine tag={t()} />}</Show>
            <Show when={i().onHold}>
              <Badge variant="warning">On hold</Badge>
            </Show>
          </span>
        }
        meta={<Robux amount={i().rap ?? null} />}
        actions={
          <Show when={props.onRemove}>
            {(remove) => <IconButton size="sm" icon="x" label={`Remove ${i().name}`} onClick={() => remove()(i())} />}
          </Show>
        }
        onClick={props.onClick ?? (() => run("market.open", itemTarget(i().id, i().bundle)))}
      />
    </ContextMenu>
  )
}

/** Totals, what you gain or lose, and warnings. */
function Summary(props: { eval: EvalView }) {
  const e = () => props.eval
  return (
    <div class="flex flex-col gap-2">
      <Card class="flex flex-wrap items-center justify-between gap-3 px-4 py-2">
        <span class="text-sm text-fg-muted">
          RAP {(e().giveRap ?? 0).toLocaleString()} for {(e().getRap ?? 0).toLocaleString()}
          <Show when={e().getRobuxAfterFee}>{(r) => <> · Robux after fee {r().toLocaleString()}</>}</Show>
        </span>
        <span class="flex items-center gap-2">
          <span class="text-fg-muted">{(e().net ?? 0) >= 0 ? "You gain" : "You lose"}</span>
          <span
            class="flex items-center gap-1 font-medium"
            classList={{ "text-success": (e().net ?? 0) > 0, "text-danger": (e().net ?? 0) < 0 }}
          >
            <Robux amount={Math.abs(e().net ?? 0)} />
            <Show when={e().netPct !== null}>({e().netPct}%)</Show>
          </span>
        </span>
      </Card>
      <For each={e().warnings}>
        {(w) => (
          <Alert variant="warning" title={w}>
            {""}
          </Alert>
        )}
      </For>
    </div>
  )
}

/** The trade you're putting together: both offers with Robux, both tradable inventories, the
 * valuation and Send. Picking items and Robux are steps inside it. */
function Composer(props: { draft: DraftView }) {
  const d = () => props.draft
  const toggle = async (give: boolean, i: TradeItemView) => {
    const r = await commands.draftToggleItem(give, i.instanceId)
    if (r.status === "error") showToast(r.error)
  }
  const robux = (give: boolean) => (v: string) => {
    const n = Number.parseInt(v || "0", 10)
    if (!Number.isNaN(n) && n >= 0) void commands.draftSetRobux(give, n)
  }
  return (
    <div class="flex max-w-5xl flex-col gap-4">
      <div class="flex flex-col gap-3 @2xl:flex-row @2xl:items-center">
        <div class="flex min-w-0 items-center gap-3">
          <Avatar size="lg" name={d().partnerName} src={thumbUrl("headshot", d().partnerId)} />
          <div class="flex min-w-0 flex-col">
            <span class="text-lg font-medium">
              <Show when={d().counterOf} fallback="New trade with ">
                Counter offer to{" "}
              </Show>
              <TextLink tone="name" onClick={() => run("user.open", String(d().partnerId))}>
                {d().partnerName || "…"}
              </TextLink>
            </span>
            <span class="text-sm text-fg-muted">Up to {d().maxItems} items a side</span>
          </div>
        </div>
        <div class="flex gap-1.5 @2xl:ml-auto">
          <Button onClick={() => commands.discardTradeDraft()}>Discard</Button>
          <Button variant="primary" icon="send" disabled={!d().partnerOk.canTrade} onClick={() => run("trade.send")}>
            Send
          </Button>
        </div>
      </div>
      <Show when={!d().partnerOk.loading && !d().partnerOk.canTrade}>
        <Alert variant="warning" title="Can't trade with them">
          {d().partnerOk.reason || "They can't trade with you right now."}
        </Alert>
      </Show>
      <div class="grid grid-cols-1 gap-4 @3xl:grid-cols-2">
        <div class="flex flex-col gap-2">
          <Side title="You give" side={d().give} onRemove={(i) => toggle(true, i)} />
          <TextField
            size="sm"
            inputMode="numeric"
            label="Add Robux"
            value={d().give.robux ? String(d().give.robux) : ""}
            onChange={robux(true)}
          />
          <Picker
            userId={shell.active ?? 0}
            chosen={d().give.items}
            onPick={(i) => toggle(true, i)}
            title="Your items"
          />
        </div>
        <div class="flex flex-col gap-2">
          <Side title="You get" side={d().get} onRemove={(i) => toggle(false, i)} />
          <TextField
            size="sm"
            inputMode="numeric"
            label="Ask for Robux"
            value={d().get.robux ? String(d().get.robux) : ""}
            onChange={robux(false)}
          />
          <Picker userId={d().partnerId} chosen={d().get.items} onPick={(i) => toggle(false, i)} title="Their items" />
        </div>
      </div>
      <Summary eval={d().eval} />
    </div>
  )
}

/** A user's tradable copies, filterable; click adds or removes one. */
function Picker(props: { userId: number; chosen: TradeItemView[]; onPick: (i: TradeItemView) => void; title: string }) {
  const res = useLive(
    () => props.userId,
    commands.tradableItems,
    (): Paged<TradeItemView> => ({ list: emptyList<TradeItemView>(), moreKey: "" }),
  )
  const [q, setQ] = createSignal("")
  const shown = () => {
    const s = q().trim().toLowerCase()
    return res.list.items.filter(
      (i) => !s || i.name.toLowerCase().includes(s) || (i.serial !== null && `#${i.serial}`.includes(s)),
    )
  }
  createEffect(() => {
    props.userId
    setQ("")
  })
  return (
    <Card class="flex flex-col gap-1 p-2">
      <SectionHeader class="px-2" title={props.title} meta={res.list.items.length || undefined} />
      <TextField
        size="sm"
        icon="search"
        aria-label={`Filter ${props.title}`}
        placeholder="Name or #serial"
        value={q()}
        onChange={setQ}
      />
      <ListState list={res.list} icon="inventory" empty="Nothing tradable" more={res.moreKey}>
        <div class="flex max-h-96 flex-col overflow-y-auto">
          <For each={shown()}>
            {(i) => (
              <ItemRow
                item={i}
                selected={props.chosen.some((c) => c.instanceId === i.instanceId)}
                onClick={() => props.onPick(i)}
              />
            )}
          </For>
        </div>
      </ListState>
    </Card>
  )
}

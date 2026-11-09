import { createMemo, createSignal, createUniqueId, Show } from "solid-js"
import { ago } from "~/lib/format"
import { commands, type PriceHistoryView, type ValueView } from "~/lib/ipc/bindings"
import { useLive } from "~/lib/store"
import { Badge, Card, Divider, Icon, Robux, Skeleton, Stat } from "~/lib/ui"
import { DemandPips, TrendMark, ValueAmount, ValueFlags } from "~/lib/value-tags"
import { rapGap, VALUES_SOURCE, versionLabel } from "~/lib/values"
import { DAY, movingAverage, toDays } from "./history"

const SPAN = 90
const dayFmt = new Intl.DateTimeFormat("en-US", { month: "short", day: "numeric", timeZone: "UTC" })

const blank = (): PriceHistoryView => ({ loading: true, error: null, rap: null, prices: [], volume: [] })

/** A limited's market at a glance, beside the buy button: what Rolimons values it at, RAP and
 * how far it sits from value, demand, trend, and the last 90 days of sales as a sparkline. The
 * Price history tab has the full chart. */
export function MarketCard(props: { collectibleId: string | null; value: ValueView | null }) {
  const history = useLive(
    () => props.collectibleId ?? "",
    (c) => (c ? commands.itemPriceHistory(c) : Promise.resolve({ ...blank(), loading: false })),
    blank,
  )
  const days = createMemo(() => toDays(history.prices, history.volume))
  const v = () => props.value
  const rap = () => v()?.rap ?? history.rap
  const gap = () => {
    const val = v()?.tag.value ?? null
    const r = rap()
    return val !== null && r !== null ? rapGap(val, r) : null
  }
  const sold30 = () => {
    const from = Date.now() - 30 * DAY
    return days().reduce((n, d) => (d.t >= from ? n + d.sold : n), 0)
  }

  return (
    <Card class="flex flex-col">
      <header class="flex flex-wrap items-center gap-1.5 px-4 pt-3.5 text-sm text-fg-muted">
        <Show when={v()} fallback={<span class="font-medium">Market</span>}>
          {(val) => (
            <>
              <Icon name="rolimons" size={14} />
              <span class="font-medium">{VALUES_SOURCE}</span>
              <Show when={val().acronym}>
                <Badge>{val().acronym}</Badge>
              </Show>
              <Show when={versionLabel(val().version)}>{(l) => <Badge>{l()}</Badge>}</Show>
              <ValueFlags tag={val().tag} />
              <Show when={val().fetchedAt}>
                {(t) => <span class="ml-auto text-xs text-fg-faint">Updated {ago(t())}</span>}
              </Show>
            </>
          )}
        </Show>
      </header>
      <dl class="grid grid-cols-2 gap-x-4 gap-y-3 px-4 pt-3 pb-3.5">
        <Show
          when={v()}
          fallback={
            <Stat label="Sold, 30 days">
              <span class="text-lg font-semibold tabular-nums">{sold30()}</span>
            </Stat>
          }
        >
          {(val) => (
            <Stat label="Value">
              <Show when={val().tag.value} fallback={<span class="text-fg-muted">Not valued</span>}>
                {(n) => <ValueAmount class="text-lg font-semibold" amount={n()} />}
              </Show>
            </Stat>
          )}
        </Show>
        <Stat label="RAP">
          <span class="flex flex-wrap items-baseline gap-x-1.5">
            <Show when={rap()} fallback={<span class="text-lg font-semibold text-fg-muted">–</span>}>
              {(r) => <Robux class="text-lg font-semibold" amount={r()} />}
            </Show>
            <Show when={gap()}>
              {(g) => (
                <span
                  class="text-xs tabular-nums"
                  classList={{
                    "text-warning": g() > 10,
                    "text-success": g() < -10,
                    "text-fg-muted": Math.abs(g()) <= 10,
                  }}
                >
                  {g() > 0 ? "+" : "−"}
                  {Math.abs(g())}%
                </span>
              )}
            </Show>
          </span>
        </Stat>
        <Show when={v()?.tag.demand}>
          {(d) => (
            <Stat label="Demand">
              <DemandPips demand={d()} label />
            </Stat>
          )}
        </Show>
        <Show when={v()?.tag.trend}>
          {(t) => (
            <Stat label="Trend">
              <TrendMark trend={t()} label />
            </Stat>
          )}
        </Show>
      </dl>
      <Show when={props.collectibleId}>
        <Divider />
        <Show when={!history.loading || days().length > 0} fallback={<SparkSkeleton />}>
          <Sparkline days={days()} value={v()?.tag.value ?? null} />
        </Show>
      </Show>
    </Card>
  )
}

function SparkSkeleton() {
  return (
    <div class="flex flex-col gap-2 px-4 py-3.5" aria-hidden="true">
      <Skeleton class="h-14 w-full rounded-sm" />
      <Skeleton class="h-3 w-32" />
    </div>
  )
}

const W = 300
const H = 56
const PAD = 4

/** The 7-day average sale price over the last 90 days, with value as a dashed line when it's in
 * range. Hover reads out a day. */
function Sparkline(props: { days: ReturnType<typeof toDays>; value: number | null }) {
  const id = createUniqueId()
  const points = createMemo(() => {
    const avg = movingAverage(props.days, 7)
    const from = Math.max(0, props.days.length - SPAN)
    return props.days.slice(from).map((d, i) => ({ t: d.t, price: avg[from + i] }))
  })
  const priced = () => points().filter((p) => p.price !== null).length
  const domain = createMemo(() => {
    const ys = points().flatMap((p) => (p.price === null ? [] : [p.price]))
    let lo = Math.min(...ys)
    let hi = Math.max(...ys)
    // Value joins the scale only when it's near the prices, so one outlier doesn't flatten them.
    const v = props.value
    const span = Math.max(hi - lo, hi * 0.05)
    if (v !== null && v >= lo - span && v <= hi + span) {
      lo = Math.min(lo, v)
      hi = Math.max(hi, v)
    }
    if (hi === lo) [lo, hi] = [lo * 0.95, hi * 1.05]
    return { lo, hi, value: v !== null && v >= lo && v <= hi ? v : null }
  })
  const n = () => Math.max(1, points().length - 1)
  const x = (i: number) => (i / n()) * W
  const y = (p: number) => {
    const { lo, hi } = domain()
    return PAD + (1 - (p - lo) / (hi - lo || 1)) * (H - PAD * 2)
  }
  // Runs of days with a price; a week without sales breaks the line.
  const runs = createMemo(() => {
    const out: { i: number; p: number }[][] = []
    let cur: { i: number; p: number }[] = []
    points().forEach((pt, i) => {
      if (pt.price === null) {
        if (cur.length) out.push(cur)
        cur = []
      } else cur.push({ i, p: pt.price })
    })
    if (cur.length) out.push(cur)
    return out
  })
  const line = () =>
    runs()
      .map((r) => r.map((pt, k) => `${k ? "L" : "M"}${x(pt.i).toFixed(1)},${y(pt.p).toFixed(1)}`).join(""))
      .join("")
  const area = () =>
    runs()
      .map((r) => {
        const top = r.map((pt, k) => `${k ? "L" : "M"}${x(pt.i).toFixed(1)},${y(pt.p).toFixed(1)}`).join("")
        return `${top}L${x(r[r.length - 1].i).toFixed(1)},${H}L${x(r[0].i).toFixed(1)},${H}Z`
      })
      .join("")

  const [hover, setHover] = createSignal<number | null>(null)
  const onMove = (e: PointerEvent) => {
    const box = (e.currentTarget as HTMLElement).getBoundingClientRect()
    const i = Math.round(((e.clientX - box.left) / box.width) * n())
    // Snap to the nearest day with a price.
    let best: number | null = null
    points().forEach((p, k) => {
      if (p.price !== null && (best === null || Math.abs(k - i) < Math.abs(best - i))) best = k
    })
    setHover(best)
  }
  const at = () => {
    const i = hover()
    if (i === null) return null
    const p = points()[i]
    return p?.price == null ? null : { i, t: p.t, price: p.price }
  }

  return (
    <div class="flex flex-col gap-2 px-4 py-3.5">
      <Show
        when={priced() >= 2}
        fallback={
          <div class="flex h-14 items-center justify-center rounded-sm bg-control text-sm text-fg-faint">
            No sales in 90 days
          </div>
        }
      >
        <div class="relative h-14" onPointerMove={onMove} onPointerLeave={() => setHover(null)}>
          <svg
            class="absolute inset-0 size-full overflow-visible"
            viewBox={`0 0 ${W} ${H}`}
            preserveAspectRatio="none"
            role="img"
            aria-label="Average sale price over the last 90 days"
          >
            <defs>
              <linearGradient id={id} x1="0" x2="0" y1="0" y2="1">
                <stop offset="0" style={{ "stop-color": "var(--color-accent)", "stop-opacity": "0.22" }} />
                <stop offset="1" style={{ "stop-color": "var(--color-accent)", "stop-opacity": "0" }} />
              </linearGradient>
            </defs>
            <path d={area()} fill={`url(#${id})`} />
            <Show when={domain().value}>
              {(v) => (
                <line
                  x1={0}
                  x2={W}
                  y1={y(v())}
                  y2={y(v())}
                  class="stroke-tag-purple"
                  stroke-width="1"
                  stroke-dasharray="3 3"
                  vector-effect="non-scaling-stroke"
                />
              )}
            </Show>
            <path
              d={line()}
              fill="none"
              class="stroke-accent"
              stroke-width="2"
              stroke-linejoin="round"
              stroke-linecap="round"
              vector-effect="non-scaling-stroke"
            />
            <Show when={at()}>
              {(a) => (
                <line
                  x1={x(a().i)}
                  x2={x(a().i)}
                  y1={0}
                  y2={H}
                  class="stroke-line-strong"
                  stroke-width="1"
                  vector-effect="non-scaling-stroke"
                />
              )}
            </Show>
          </svg>
          {/* Drawn in HTML so it stays round when the chart stretches. */}
          <Show when={at()}>
            {(a) => (
              <span
                class="pointer-events-none absolute size-2.5 -translate-x-1/2 -translate-y-1/2 rounded-full border-2 border-canvas bg-accent"
                style={{ left: `${(x(a().i) / W) * 100}%`, top: `${(y(a().price) / H) * 100}%` }}
              />
            )}
          </Show>
        </div>
      </Show>
      <div class="flex h-4 items-center gap-3 text-xs text-fg-muted">
        <Show
          when={at()}
          fallback={
            <>
              <span>Average price · 90 days</span>
              <Show when={domain().value !== null && priced() >= 2}>
                <span class="ml-auto flex items-center gap-1.5">
                  <svg width="12" height="2" aria-hidden="true" class="overflow-visible">
                    <line x1="0" x2="12" y1="1" y2="1" class="stroke-tag-purple" stroke-dasharray="3 3" />
                  </svg>
                  Value
                </span>
              </Show>
            </>
          }
        >
          {(a) => (
            <>
              <span>{dayFmt.format(a().t)}</span>
              <Robux class="ml-auto font-medium text-fg" amount={Math.round(a().price)} />
            </>
          )}
        </Show>
      </div>
    </div>
  )
}

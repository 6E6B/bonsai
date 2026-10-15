import { createMemo, createSignal, createUniqueId, For, type JSX, onCleanup, onMount, Show } from "solid-js"
import { handle, run } from "~/lib/commands/registry"
import { compact, full } from "~/lib/format"
import { commands, type PriceHistoryView } from "~/lib/ipc/bindings"
import { useLive } from "~/lib/store"
import { Button, Card, type Column, DataTable, EmptyState, Robux, SegmentedControl, Skeleton, Stat } from "~/lib/ui"
import {
  DAY,
  type Day,
  localX,
  logTicks,
  MIN_SPAN,
  matchRange,
  movingAverage,
  niceTicks,
  panWindow,
  pctOver,
  presetWindow,
  priceDomain,
  type Range,
  type Reference,
  rangesFor,
  type Summary,
  spread,
  summarize,
  toDays,
  visibleSlice,
  type Window,
  zoomWindow,
} from "./history"

// Days are UTC midnights; local formatting would show the day before west of Greenwich.
const dayFmt = new Intl.DateTimeFormat("en-US", { month: "short", day: "numeric", timeZone: "UTC" })
const dayYearFmt = new Intl.DateTimeFormat("en-US", {
  month: "short",
  day: "numeric",
  year: "numeric",
  timeZone: "UTC",
})
const monthFmt = new Intl.DateTimeFormat("en-US", { month: "short", timeZone: "UTC" })
const longFmt = new Intl.DateTimeFormat("en-US", {
  weekday: "short",
  month: "short",
  day: "numeric",
  year: "numeric",
  timeZone: "UTC",
})

/** Reference line colors. The daily line is the accent; these stay out of its way. */
const REF_STROKE: Record<string, string> = {
  rap: "stroke-fg-muted",
  value: "stroke-tag-purple",
  best: "stroke-tag-green",
}

// Chart geometry in pixels.
const GUTTER = 56 // right side, for y labels and reference tags
const TAG_H = 16
const TOP = 8
const PRICE_H = 220
const GAP = 14
const VOL_H = 52
const AXIS_H = 20
const HEIGHT = TOP + PRICE_H + GAP + VOL_H + AXIS_H
const VOL_TOP = TOP + PRICE_H + GAP
const NAV_H = 36
/** Pointer distance from a navigator edge that grabs the edge instead of the window. */
const EDGE = 6
const ZOOM_STEP = 0.7

type Scale = "linear" | "log"

export interface PriceHistoryProps {
  collectibleId: string
  /** Rolimons value, when values are on and it's valued. */
  value: number | null
  /** The cheapest copy for sale now. */
  best: number | null
}

/** Resale history of a limited: daily average price over copies sold, against RAP, value and
 * the cheapest listing, with a summary of the part on screen. Zooms and pans like a stock
 * chart, with an overview of the whole history underneath. */
export function PriceHistory(props: PriceHistoryProps) {
  const h = useLive(
    () => props.collectibleId,
    commands.itemPriceHistory,
    (): PriceHistoryView => ({ loading: true, error: null, rap: null, prices: [], volume: [] }),
  )
  const all = createMemo(() => toDays(h.prices, h.volume))
  const avgAll = createMemo(() => movingAverage(all()))
  const len = () => all().length

  const [range, setRange] = createSignal<Range>("1y")
  // A zoomed or panned view; null follows the range so new days keep it pinned to today.
  const [zoomed, setZoomed] = createSignal<Window | null>(null)
  const [scale, setScale] = createSignal<Scale>("linear")
  const [view, setView] = createSignal<"chart" | "table">("chart")
  const win = createMemo(() => zoomed() ?? presetWindow(len(), range()))
  const shownRange = () => matchRange(win(), len()) ?? ""

  onCleanup(
    handle("market.history_range", (a) => {
      const opts = rangesFor(len())
      const pick = opts.find((r) => r.value === a?.trim().toLowerCase())
      const i = opts.findIndex((r) => r.value === shownRange())
      setRange(pick?.value ?? opts[(i + 1) % opts.length].value)
      setZoomed(null)
    }),
  )
  onCleanup(
    handle("market.history_zoom", (a) => {
      const w = win()
      const mid = (w.from + w.to) / 2
      const how = a?.trim().toLowerCase()
      if (how === "reset") setZoomed(null)
      else setZoomed(zoomWindow(w, how === "out" ? 1 / ZOOM_STEP : ZOOM_STEP, mid, len()))
    }),
  )
  onCleanup(
    handle("market.history_scale", (a) =>
      setScale(a === "log" || a === "linear" ? a : scale() === "log" ? "linear" : "log"),
    ),
  )
  onCleanup(
    handle("market.history_view", (a) =>
      setView(a === "chart" || a === "table" ? a : view() === "chart" ? "table" : "chart"),
    ),
  )

  const slice = createMemo(() => visibleSlice(win()))
  const days = createMemo(() => all().slice(...slice()))
  const avg = createMemo(() => avgAll().slice(...slice()))
  const sum = createMemo(() => summarize(days()))
  const refs = createMemo(() => {
    const out: Reference[] = []
    if (h.rap) out.push({ key: "rap", label: "RAP", value: h.rap })
    if (props.value) out.push({ key: "value", label: "Value", value: props.value })
    if (props.best) out.push({ key: "best", label: "Best price", value: props.best })
    return out
  })

  return (
    <Show when={!h.error} fallback={<span class="text-fg-muted">{h.error}</span>}>
      <Show
        when={len() > 0 && days().length > 0}
        fallback={
          <Show when={!h.loading} fallback={<Skeleton class="h-80 w-full rounded-md" />}>
            <EmptyState icon="values" title="No recorded sales" />
          </Show>
        }
      >
        <div class="flex flex-col gap-3">
          <div class="flex flex-wrap items-center gap-2">
            <SegmentedControl
              aria-label="Range"
              size="sm"
              value={shownRange()}
              onChange={(v) => run("market.history_range", v)}
              options={rangesFor(len())}
            />
            <Show when={zoomed() && !shownRange()}>
              <Button size="sm" variant="ghost" onClick={() => run("market.history_zoom", "reset")}>
                Reset zoom
              </Button>
            </Show>
            <SegmentedControl
              aria-label="Scale"
              size="sm"
              value={scale()}
              onChange={(v) => run("market.history_scale", v)}
              options={[
                { value: "linear", label: "Linear" },
                { value: "log", label: "Log" },
              ]}
            />
            <SegmentedControl
              aria-label="View"
              size="sm"
              value={view()}
              onChange={(v) => run("market.history_view", v)}
              options={[
                { value: "chart", label: "Chart" },
                { value: "table", label: "Table" },
              ]}
            />
            <span class="ml-auto text-xs text-fg-faint">
              {dayYearFmt.format(days()[0].t)} – {dayYearFmt.format(days()[days().length - 1].t)} · daily, UTC
            </span>
          </div>
          <RangeStats sum={sum()} rap={h.rap} best={props.best} />
          <Show when={view() === "chart"} fallback={<DayTable days={days()} avg={avg()} rap={h.rap} />}>
            <Chart
              all={all()}
              avg={avgAll()}
              win={win()}
              onWindow={setZoomed}
              onReset={() => setZoomed(null)}
              refs={refs()}
              rap={h.rap}
              log={scale() === "log"}
            />
          </Show>
        </div>
      </Show>
    </Show>
  )
}

const round = (v: number | null) => (v === null ? null : Math.round(v))

/** A percent change, signed. Green when it moves the `good` way, red when it doesn't. */
function Delta(props: { pct: number | null; suffix?: string; good?: "up" | "down" }) {
  return (
    <Show when={props.pct !== null ? { pct: props.pct } : null}>
      {(d) => {
        const sign = () => Math.sign(d().pct) * (props.good === "down" ? -1 : 1)
        return (
          <span
            class="text-xs"
            classList={{
              "text-success": props.good !== undefined && sign() > 0,
              "text-danger": props.good !== undefined && sign() < 0,
              "text-fg-muted": props.good === undefined || sign() === 0,
            }}
          >
            {d().pct > 0 ? "+" : ""}
            {d().pct}%{props.suffix ? ` ${props.suffix}` : ""}
          </span>
        )
      }}
    </Show>
  )
}

function Sub(props: { children: JSX.Element }) {
  return <span class="text-xs text-fg-muted">{props.children}</span>
}

function RangeStats(props: { sum: Summary; rap: number | null; best: number | null }) {
  const s = () => props.sum
  return (
    <Card class="p-3">
      <dl class="grid grid-cols-[repeat(auto-fill,minmax(8.5rem,1fr))] gap-4">
        <Stat label="RAP">
          <span class="flex flex-col">
            <Robux class="text-lg font-medium" amount={props.rap} />
            <Delta pct={pctOver(props.rap, s().average)} suffix="vs range avg" />
          </span>
        </Stat>
        <Stat label="Average sale">
          <span class="flex flex-col">
            <Robux amount={round(s().average)} />
            <Sub>weighted by copies</Sub>
          </span>
        </Stat>
        <Stat label="Change">
          <Show when={s().first !== s().last && s().first && s().last ? [s().first, s().last] : null} fallback="—">
            {(fl) => {
              const pct = () => pctOver(fl()[1]?.price ?? null, fl()[0]?.price ?? null) ?? 0
              return (
                <span class="flex flex-col">
                  <span classList={{ "text-success": pct() > 0, "text-danger": pct() < 0 }}>
                    {pct() > 0 ? "+" : ""}
                    {pct()}%
                  </span>
                  <Sub>
                    {compact(fl()[0]?.price ?? 0)} → {compact(fl()[1]?.price ?? 0)}
                  </Sub>
                </span>
              )
            }}
          </Show>
        </Stat>
        <Stat label="High / low">
          <Show when={s().high && s().low ? { high: s().high, low: s().low } : null} fallback="—">
            {(hl) => (
              <span class="flex flex-col">
                <span>
                  {compact(hl().high?.price ?? 0)} / {compact(hl().low?.price ?? 0)}
                </span>
                <Sub>
                  {dayFmt.format(hl().high?.t ?? 0)} / {dayFmt.format(hl().low?.t ?? 0)}
                </Sub>
              </span>
            )}
          </Show>
        </Stat>
        <Stat label="Copies sold">
          <span class="flex flex-col">
            <span>{full(s().sold)}</span>
            <Sub>{(s().sold / Math.max(1, s().days)).toFixed(1)} a day</Sub>
          </span>
        </Stat>
        <Stat label="Days with sales">
          <span class="flex flex-col">
            <span>
              {s().activeDays} of {s().days}
            </span>
            <Show when={s().last}>{(l) => <Sub>last {dayFmt.format(l().t)}</Sub>}</Show>
          </span>
        </Stat>
        <Show when={props.best}>
          {(b) => (
            <Stat label="Best price">
              <span class="flex flex-col">
                <Robux amount={b()} />
                <Delta pct={pctOver(b(), props.rap)} suffix="vs RAP" good="down" />
              </span>
            </Stat>
          )}
        </Show>
      </dl>
    </Card>
  )
}

/** A y scale over lo..hi mapped to the price panel, linear or log. */
function yScale(lo: number, hi: number, log: boolean) {
  const f = log ? (v: number) => Math.log10(Math.max(v, 1e-9)) : (v: number) => v
  const a = f(lo)
  const b = f(hi)
  return {
    y: (v: number) => TOP + PRICE_H - ((f(v) - a) / (b - a)) * PRICE_H,
    value: (py: number) => {
      const u = a + ((TOP + PRICE_H - py) / PRICE_H) * (b - a)
      return log ? 10 ** u : u
    },
  }
}

/** Day spacings for date ticks; past a month, ticks fall on the first of the month. */
const DAY_STEPS = [1, 2, 3, 7, 14]
const MONTH_STEPS = [1, 2, 3, 6, 12, 24]

/** Date ticks for days i0..i1 at `step` pixels a day: day indices and labels. */
function dateTicks(all: readonly Day[], i0: number, i1: number, step: number) {
  const minGap = 72
  const out: { i: number; label: string }[] = []
  const every = DAY_STEPS.find((d) => d * step >= minGap)
  if (every !== undefined) {
    for (let i = i0; i < i1; i++) {
      const d = Math.round(all[i].t / DAY)
      if (d % every === 0) out.push({ i, label: dayFmt.format(all[i].t) })
    }
    return out
  }
  const months = MONTH_STEPS.find((m) => m * 30.4 * step >= minGap) ?? 24
  for (let i = i0; i < i1; i++) {
    const date = new Date(all[i].t)
    if (date.getUTCDate() !== 1) continue
    const m = date.getUTCFullYear() * 12 + date.getUTCMonth()
    if (m % months !== 0) continue
    const label = date.getUTCMonth() === 0 || months >= 12 ? String(date.getUTCFullYear()) : monthFmt.format(date)
    out.push({ i, label })
  }
  return out
}

interface ChartProps {
  all: Day[]
  avg: (number | null)[]
  win: Window
  onWindow: (w: Window) => void
  onReset: () => void
  refs: Reference[]
  rap: number | null
  log: boolean
}

function Chart(props: ChartProps) {
  let box: HTMLDivElement | undefined
  let svg: SVGSVGElement | undefined
  const clip = createUniqueId()
  const [width, setWidth] = createSignal(0)
  const [cursor, setCursor] = createSignal<number | null>(null)
  /** Pointer height in the chart, for the price crosshair; null from the keyboard. */
  const [pointerY, setPointerY] = createSignal<number | null>(null)
  const [drag, setDrag] = createSignal<{ x: number; win: Window; step: number } | null>(null)
  onMount(() => {
    if (!box) return
    const ro = new ResizeObserver(([e]) => setWidth(Math.floor(e.contentRect.width)))
    ro.observe(box)
    onCleanup(() => ro.disconnect())
  })

  const len = () => props.all.length
  const plotW = () => Math.max(0, width() - GUTTER)
  const span = () => props.win.to - props.win.from
  const step = () => plotW() / Math.max(1, span())
  const x = (i: number) => (i - props.win.from + 0.5) * step()
  /** Days at least partly on screen. */
  const i0 = () => Math.max(0, Math.floor(props.win.from))
  const i1 = () => Math.min(len(), Math.ceil(props.win.to))

  const domain = createMemo(() => {
    const vs: number[] = []
    for (let i = i0(); i < i1(); i++) {
      const p = props.all[i].price
      const a = props.avg[i]
      if (p !== null) vs.push(p)
      if (a !== null) vs.push(a)
    }
    return priceDomain(vs, props.refs, props.log)
  })
  const ys = createMemo(() => yScale(domain().lo, domain().hi, props.log))
  const y = (v: number) => ys().y(v)
  const yTicks = createMemo(() =>
    props.log ? logTicks(domain().lo, domain().hi, 5) : niceTicks(domain().lo, domain().hi, 5),
  )

  const volMax = createMemo(() => {
    let m = 1
    for (let i = i0(); i < i1(); i++) m = Math.max(m, props.all[i].sold)
    return m
  })
  const volY = (v: number) => VOL_TOP + VOL_H - (v / volMax()) * VOL_H

  /** Path through the points that have a value, joined across empty days, running on to the
   * nearest point past each edge so the line reaches the side of the plot. */
  const pathOf = (vs: (number | null)[]) => {
    let a = i0() - 1
    while (a > 0 && vs[a] === null) a--
    let b = i1()
    while (b < vs.length - 1 && vs[b] === null) b++
    let d = ""
    for (let i = Math.max(0, a); i <= Math.min(vs.length - 1, b); i++) {
      const v = vs[i]
      if (v !== null) d += `${d ? "L" : "M"}${x(i).toFixed(1)},${y(v).toFixed(1)}`
    }
    return d
  }
  const prices = createMemo(() => props.all.map((d) => d.price))
  const line = createMemo(() => pathOf(prices()))
  const area = createMemo(() => {
    const d = line()
    if (!d) return ""
    const base = (TOP + PRICE_H).toFixed(1)
    const xs = d.match(/[ML](-?[\d.]+),/g) ?? []
    const first = xs[0]?.slice(1, -1) ?? "0"
    const last = xs[xs.length - 1]?.slice(1, -1) ?? "0"
    return `${d}L${last},${base}L${first},${base}Z`
  })
  const avgLine = createMemo(() => pathOf(props.avg))

  // 2px apart, and no wider than a thick bar when there are only a few days.
  const barW = () => Math.max(1, Math.min(step() - 2, 20))
  /** Every volume bar as one path; thousands of rects would be slow to pan. */
  const bars = createMemo(() => {
    const w = barW()
    const base = VOL_TOP + VOL_H
    let d = ""
    for (let i = i0(); i < i1(); i++) {
      const sold = props.all[i].sold
      if (sold <= 0) continue
      const top = volY(sold)
      d += `M${(x(i) - w / 2).toFixed(1)},${base}V${top.toFixed(1)}h${w.toFixed(1)}V${base}Z`
    }
    return d
  })

  const xTicks = createMemo(() =>
    dateTicks(props.all, i0(), i1(), step()).filter((t) => x(t.i) > 16 && x(t.i) < plotW() - 16),
  )

  const refLabels = createMemo(() => {
    const shown = domain().shown
    // Centers of the gutter tags, kept apart so close references don't overlap.
    const tags = spread(
      shown.map((r) => y(r.value)),
      TAG_H + 2,
      TOP + TAG_H / 2,
      TOP + PRICE_H,
    )
    return shown.map((r, i) => ({ ...r, line: y(r.value), tag: tags[i] }))
  })
  /** Axis labels a reference tag would cover are dropped; the grid line stays. */
  const tickFree = (v: number) => refLabels().every((r) => Math.abs(r.tag - y(v)) > TAG_H)

  /** Pointer position in the chart's own pixels; see localX for why it isn't clientX - left. */
  const local = (e: PointerEvent | WheelEvent | MouseEvent) => {
    if (!svg) return { x: 0, y: 0 }
    const r = svg.getBoundingClientRect()
    const k = r.width > 0 ? width() / r.width : 1
    return { x: localX(e.clientX, r, width()), y: (e.clientY - r.top) * k }
  }
  const dayAt = (px: number) => props.win.from + px / Math.max(1e-9, step())

  const onMove = (e: PointerEvent) => {
    const p = local(e)
    const d = drag()
    if (d) {
      props.onWindow(panWindow(d.win, -(p.x - d.x) / d.step, len()))
      return
    }
    const i = Math.floor(dayAt(p.x))
    const inPlot = p.x >= 0 && p.x <= plotW()
    setCursor(inPlot && i >= 0 && i < len() ? i : null)
    setPointerY(inPlot ? p.y : null)
  }
  const onDown = (e: PointerEvent) => {
    if (e.button !== 0) return
    const p = local(e)
    if (p.x > plotW()) return
    svg?.setPointerCapture(e.pointerId)
    setDrag({ x: p.x, win: props.win, step: step() })
  }
  const onUp = (e: PointerEvent) => {
    if (!drag()) return
    svg?.releasePointerCapture(e.pointerId)
    setDrag(null)
  }
  const onWheel = (e: WheelEvent) => {
    const p = local(e)
    if (p.x < 0 || p.x > plotW()) return
    e.preventDefault()
    const unit = e.deltaMode === 1 ? 16 : e.deltaMode === 2 ? plotW() : 1
    const dx = e.deltaX * unit
    const dy = e.deltaY * unit
    // Sideways scrolling (trackpads, or Shift with a wheel) pans; up and down zooms.
    if (Math.abs(dx) > Math.abs(dy) || e.shiftKey) {
      props.onWindow(panWindow(props.win, (Math.abs(dx) > Math.abs(dy) ? dx : dy) / step(), len()))
    } else {
      props.onWindow(zoomWindow(props.win, Math.exp(dy * 0.002), dayAt(p.x), len()))
    }
  }
  onMount(() => {
    // Not passive, so the page doesn't scroll while zooming. Solid's onWheel can't say so.
    const el = box
    el?.addEventListener("wheel", onWheel, { passive: false })
    onCleanup(() => el?.removeEventListener("wheel", onWheel))
  })

  const onKey = (e: KeyboardEvent) => {
    const w = props.win
    const c = cursor()
    if (e.key === "+" || e.key === "=" || e.key === "-") {
      const factor = e.key === "-" ? 1 / ZOOM_STEP : ZOOM_STEP
      props.onWindow(zoomWindow(w, factor, c !== null ? c + 0.5 : (w.from + w.to) / 2, len()))
    } else if (e.key === "0") props.onReset()
    else if (e.shiftKey && (e.key === "ArrowLeft" || e.key === "ArrowRight")) {
      props.onWindow(panWindow(w, (e.key === "ArrowLeft" ? -0.2 : 0.2) * span(), len()))
    } else if (e.key === "Escape") setCursor(null)
    else {
      const at = c ?? Math.min(len(), Math.round(w.to))
      const to = { ArrowLeft: at - 1, ArrowRight: at + 1, Home: 0, End: len() - 1 }[e.key]
      if (to === undefined) return
      const i = Math.max(0, Math.min(len() - 1, to))
      setCursor(i)
      setPointerY(null)
      // Keep the cursor on screen.
      if (i < w.from) props.onWindow(panWindow(w, i - w.from, len()))
      else if (i + 1 > w.to) props.onWindow(panWindow(w, i + 1 - w.to, len()))
    }
    e.preventDefault()
  }

  const hovered = () => {
    const i = cursor()
    return i === null || drag() ? null : { i, day: props.all[i], avg: props.avg[i] }
  }
  /** The price under the pointer, when it's over the price panel. */
  const pointerPrice = () => {
    const py = pointerY()
    return py === null || py < TOP || py > TOP + PRICE_H || drag() ? null : { y: py, v: ys().value(py) }
  }
  const showDots = () => step() >= 12

  return (
    <Card class="flex flex-col gap-2 p-3">
      <div class="flex flex-wrap items-center gap-x-4 gap-y-1">
        <Legend refs={props.refs} off={domain().off} />
        <span class="ml-auto text-xs text-fg-faint">Scroll to zoom · drag to pan · double-click to reset</span>
      </div>
      <div ref={box} class="relative flex w-full flex-col gap-2">
        <Show when={width() > 0}>
          <svg
            ref={svg}
            width={width()}
            height={HEIGHT}
            class="block touch-none outline-none select-none"
            classList={{ "cursor-grabbing": drag() !== null, "cursor-crosshair": drag() === null }}
            tabindex="0"
            role="img"
            aria-label="Average sale price and copies sold by day. Arrow keys step through days, plus and minus zoom, Shift with arrows pans, 0 resets."
            onPointerMove={onMove}
            onPointerDown={onDown}
            onPointerUp={onUp}
            onPointerCancel={onUp}
            onPointerLeave={() => {
              if (drag()) return
              setCursor(null)
              setPointerY(null)
            }}
            onDblClick={() => props.onReset()}
            onKeyDown={onKey}
            onBlur={() => setCursor(null)}
          >
            <defs>
              <clipPath id={clip}>
                <rect x={0} y={0} width={plotW()} height={HEIGHT} />
              </clipPath>
            </defs>

            {/* Price grid and labels */}
            <For each={yTicks()}>
              {(v) => (
                <g>
                  <line x1={0} x2={plotW()} y1={y(v)} y2={y(v)} class="stroke-line" />
                  <Show when={tickFree(v)}>
                    <text x={plotW() + 6} y={y(v) + 4} class="fill-fg-faint text-xs tabular-nums">
                      {compact(v)}
                    </text>
                  </Show>
                </g>
              )}
            </For>

            {/* Reference lines */}
            <For each={refLabels()}>
              {(r) => (
                <g>
                  <line
                    x1={0}
                    x2={plotW()}
                    y1={r.line}
                    y2={r.line}
                    class={REF_STROKE[r.key]}
                    stroke-width="1"
                    stroke-dasharray="3 3"
                  />
                  <path
                    d={`M${plotW()},${r.line}L${plotW() + 2},${r.tag}`}
                    class={REF_STROKE[r.key]}
                    stroke-width="1"
                  />
                  <rect
                    x={plotW() + 2}
                    y={r.tag - TAG_H / 2}
                    width={GUTTER - 4}
                    height={TAG_H}
                    rx="3"
                    class={`fill-surface ${REF_STROKE[r.key]}`}
                    stroke-width="1"
                  />
                  <text x={plotW() + 6} y={r.tag + 4} class="fill-fg text-xs tabular-nums">
                    {compact(r.value)}
                  </text>
                </g>
              )}
            </For>

            <g clip-path={`url(#${clip})`}>
              {/* Prices */}
              <path d={area()} class="fill-accent-soft" opacity="0.6" />
              <path
                d={avgLine()}
                fill="none"
                class="stroke-fg-muted"
                stroke-width="1.5"
                stroke-dasharray="4 3"
                stroke-linejoin="round"
              />
              <path d={line()} fill="none" class="stroke-accent" stroke-width="2" stroke-linejoin="round" />
              <Show when={showDots()}>
                <For each={props.all.slice(i0(), i1())}>
                  {(d, k) => (
                    <Show when={d.price !== null}>
                      <circle cx={x(i0() + k())} cy={y(d.price ?? 0)} r="2.5" class="fill-accent" />
                    </Show>
                  )}
                </For>
              </Show>

              {/* Volume */}
              <path d={bars()} class="fill-fg-faint" opacity="0.55" />
              <Show when={hovered()?.day.sold ? hovered() : null}>
                {(c) => (
                  <rect
                    x={x(c().i) - barW() / 2}
                    y={volY(c().day.sold)}
                    width={barW()}
                    height={VOL_TOP + VOL_H - volY(c().day.sold)}
                    class="fill-accent"
                  />
                )}
              </Show>
            </g>
            <text x={plotW() + 6} y={VOL_TOP + 8} class="fill-fg-faint text-xs tabular-nums">
              {compact(volMax())}
            </text>
            <text x={plotW() + 6} y={VOL_TOP + VOL_H} class="fill-fg-faint text-xs">
              sold
            </text>
            <line x1={0} x2={plotW()} y1={VOL_TOP + VOL_H} y2={VOL_TOP + VOL_H} class="stroke-line-strong" />

            {/* Dates */}
            <For each={xTicks()}>
              {(t) => (
                <text x={x(t.i)} y={HEIGHT - 4} text-anchor="middle" class="fill-fg-faint text-xs">
                  {t.label}
                </text>
              )}
            </For>

            {/* Crosshair */}
            <Show when={hovered()}>
              {(c) => (
                <g pointer-events="none">
                  <line
                    x1={x(c().i)}
                    x2={x(c().i)}
                    y1={TOP}
                    y2={VOL_TOP + VOL_H}
                    class="stroke-line-strong"
                    stroke-width="1"
                  />
                  <Show when={c().day.price}>
                    {(p) => (
                      <circle cx={x(c().i)} cy={y(p())} r="4.5" class="fill-accent stroke-canvas" stroke-width="2" />
                    )}
                  </Show>
                  <Show when={c().avg}>
                    {(a) => (
                      <circle cx={x(c().i)} cy={y(a())} r="3" class="fill-fg-muted stroke-canvas" stroke-width="2" />
                    )}
                  </Show>
                  <rect
                    x={Math.max(0, Math.min(plotW() - 84, x(c().i) - 42))}
                    y={HEIGHT - AXIS_H + 1}
                    width={84}
                    height={AXIS_H - 2}
                    rx="3"
                    class="fill-raised stroke-line-strong"
                  />
                  <text
                    x={Math.max(42, Math.min(plotW() - 42, x(c().i)))}
                    y={HEIGHT - 6}
                    text-anchor="middle"
                    class="fill-fg text-xs tabular-nums"
                  >
                    {dayYearFmt.format(c().day.t)}
                  </text>
                </g>
              )}
            </Show>
            <Show when={pointerPrice()}>
              {(pp) => (
                <g pointer-events="none">
                  <line
                    x1={0}
                    x2={plotW()}
                    y1={pp().y}
                    y2={pp().y}
                    class="stroke-line-strong"
                    stroke-width="1"
                    stroke-dasharray="2 2"
                  />
                  <rect
                    x={plotW() + 2}
                    y={pp().y - TAG_H / 2}
                    width={GUTTER - 4}
                    height={TAG_H}
                    rx="3"
                    class="fill-raised stroke-line-strong"
                  />
                  <text x={plotW() + 6} y={pp().y + 4} class="fill-fg text-xs tabular-nums">
                    {compact(Math.round(pp().v))}
                  </text>
                </g>
              )}
            </Show>
          </svg>

          <Navigator all={props.all} win={props.win} onWindow={props.onWindow} width={plotW()} log={props.log} />
        </Show>

        <Show when={hovered()}>
          {(c) => {
            const TIP_W = 208
            const TIP_H = 120
            const left = () => (x(c().i) > plotW() / 2 ? x(c().i) - TIP_W - 16 : x(c().i) + 16)
            const top = () => {
              const py = pointerY()
              return py === null ? TOP : Math.max(TOP, Math.min(HEIGHT - AXIS_H - TIP_H, py - TIP_H / 2))
            }
            return (
              <div
                class="pointer-events-none absolute flex w-52 flex-col gap-1 rounded-md bg-raised p-2 text-sm shadow-popover"
                style={{ top: `${top()}px`, left: `${left()}px` }}
              >
                <span class="font-medium">{longFmt.format(c().day.t)}</span>
                <Row label="Average price">
                  <Show when={c().day.price} fallback={<span class="text-fg-muted">No sales</span>}>
                    {(p) => <Robux amount={Math.round(p())} />}
                  </Show>
                </Row>
                <Row label="Copies sold">{full(c().day.sold)}</Row>
                <Row label="7-day average">
                  <Robux amount={round(c().avg)} />
                </Row>
                <Show when={props.rap && c().day.price}>
                  <Row label="vs RAP">
                    <Delta pct={pctOver(c().day.price, props.rap)} />
                  </Row>
                </Show>
              </div>
            )
          }}
        </Show>
      </div>
    </Card>
  )
}

/** The whole history in miniature with the visible window on it. Drag the window to pan, its
 * edges to zoom, or click elsewhere to jump there. */
function Navigator(props: { all: Day[]; win: Window; onWindow: (w: Window) => void; width: number; log: boolean }) {
  let svg: SVGSVGElement | undefined
  const [hoverEdge, setHoverEdge] = createSignal<"from" | "to" | null>(null)
  let grab: { mode: "move" | "from" | "to"; x: number; win: Window } | null = null

  const len = () => props.all.length
  const k = () => props.width / Math.max(1, len())
  const line = createMemo(() => {
    const vs = props.all.map((d) => d.price)
    const pos = vs.filter((v): v is number => v !== null && v > 0)
    if (pos.length === 0) return ""
    const f = props.log ? Math.log10 : (v: number) => v
    const lo = f(Math.min(...pos))
    const hi = f(Math.max(...pos))
    const yOf = (v: number) => NAV_H - 3 - ((f(v) - lo) / Math.max(1e-9, hi - lo)) * (NAV_H - 6)
    let d = ""
    vs.forEach((v, i) => {
      if (v !== null && v > 0) d += `${d ? "L" : "M"}${((i + 0.5) * k()).toFixed(1)},${yOf(v).toFixed(1)}`
    })
    return d
  })
  const wx0 = () => props.win.from * k()
  const wx1 = () => props.win.to * k()

  const px = (e: PointerEvent) => (svg ? localX(e.clientX, svg.getBoundingClientRect(), props.width) : 0)
  const edgeAt = (x: number) => (Math.abs(x - wx0()) <= EDGE ? "from" : Math.abs(x - wx1()) <= EDGE ? "to" : null)

  const onDown = (e: PointerEvent) => {
    if (e.button !== 0) return
    const x = px(e)
    const edge = edgeAt(x)
    let win = props.win
    if (!edge && (x < wx0() || x > wx1())) {
      // Jump: center the window on the click, then drag from there.
      const span = win.to - win.from
      win = panWindow(win, x / k() - span / 2 - win.from, len())
      props.onWindow(win)
    }
    grab = { mode: edge ?? "move", x, win }
    svg?.setPointerCapture(e.pointerId)
  }
  const onMove = (e: PointerEvent) => {
    const x = px(e)
    if (!grab) {
      setHoverEdge(edgeAt(x))
      return
    }
    const g = grab
    const day = x / k()
    if (g.mode === "move") props.onWindow(panWindow(g.win, (x - g.x) / k(), len()))
    else if (g.mode === "from") {
      const from = Math.max(0, Math.min(g.win.to - Math.min(MIN_SPAN, len()), day))
      props.onWindow({ from, to: g.win.to })
    } else {
      const to = Math.min(len(), Math.max(g.win.from + Math.min(MIN_SPAN, len()), day))
      props.onWindow({ from: g.win.from, to })
    }
  }
  const onUp = (e: PointerEvent) => {
    if (!grab) return
    grab = null
    svg?.releasePointerCapture(e.pointerId)
  }

  return (
    <svg
      ref={svg}
      width={props.width}
      height={NAV_H}
      class="block touch-none select-none"
      classList={{ "cursor-ew-resize": hoverEdge() !== null, "cursor-pointer": hoverEdge() === null }}
      role="img"
      aria-label="Whole price history. Drag the highlighted window to pan or its edges to zoom."
      onPointerDown={onDown}
      onPointerMove={onMove}
      onPointerUp={onUp}
      onPointerCancel={onUp}
      onPointerLeave={() => setHoverEdge(null)}
    >
      <rect x={0} y={0} width={props.width} height={NAV_H} rx="4" class="fill-canvas stroke-line" />
      <path d={line()} fill="none" class="stroke-fg-faint" stroke-width="1" />
      <rect x={wx0()} y={0} width={Math.max(2, wx1() - wx0())} height={NAV_H} class="fill-accent-soft" opacity="0.5" />
      <line x1={wx0()} x2={wx0()} y1={0} y2={NAV_H} class="stroke-accent" stroke-width="2" />
      <line x1={wx1()} x2={wx1()} y1={0} y2={NAV_H} class="stroke-accent" stroke-width="2" />
      <For each={[wx0(), wx1()]}>
        {(hx) => <rect x={hx - 3} y={NAV_H / 2 - 7} width={6} height={14} rx="2" class="fill-accent" />}
      </For>
    </svg>
  )
}

function Row(props: { label: string; children: JSX.Element }) {
  return (
    <span class="flex items-center justify-between gap-3">
      <span class="text-fg-muted">{props.label}</span>
      <span class="tabular-nums">{props.children}</span>
    </span>
  )
}

/** What each line is. References too far from the prices to draw are marked off chart. */
function Legend(props: { refs: Reference[]; off: Reference[] }) {
  const isOff = (r: Reference) => props.off.some((o) => o.key === r.key)
  return (
    <div class="flex flex-wrap items-center gap-x-4 gap-y-1 text-xs text-fg-muted">
      <LegendItem stroke="stroke-accent" label="Daily average" width={2} />
      <LegendItem stroke="stroke-fg-muted" label="7-day average" dash="4 3" width={1.5} />
      <For each={props.refs}>
        {(r) => (
          <LegendItem stroke={REF_STROKE[r.key]} dash="3 3" width={1} label={r.label}>
            <Robux amount={r.value} compact />
            <Show when={isOff(r)}>
              <span class="text-fg-faint">(off chart)</span>
            </Show>
          </LegendItem>
        )}
      </For>
    </div>
  )
}

function LegendItem(props: { stroke: string; label: string; dash?: string; width: number; children?: JSX.Element }) {
  return (
    <span class="flex items-center gap-1.5">
      <svg width="16" height="8" aria-hidden="true">
        <line
          x1="0"
          x2="16"
          y1="4"
          y2="4"
          class={props.stroke}
          stroke-width={props.width}
          stroke-dasharray={props.dash}
        />
      </svg>
      <span>{props.label}</span>
      {props.children}
    </span>
  )
}

type DayRow = { day: Day; avg: number | null; prev: number | null }

function DayTable(props: { days: Day[]; avg: (number | null)[]; rap: number | null }) {
  const rows = createMemo(() => {
    const out: DayRow[] = []
    let prev: number | null = null
    props.days.forEach((d, i) => {
      if (d.price === null) return
      out.push({ day: d, avg: props.avg[i], prev })
      prev = d.price
    })
    return out.reverse()
  })
  const columns: Column<DayRow>[] = [
    { key: "date", label: "Day", width: "minmax(8rem, 1.2fr)", cell: (r) => longFmt.format(r.day.t) },
    {
      key: "price",
      label: "Average price",
      align: "end",
      cell: (r) => <Robux amount={Math.round(r.day.price ?? 0)} />,
    },
    {
      key: "change",
      label: "vs previous sale",
      align: "end",
      cell: (r) => (
        <Show when={pctOver(r.day.price, r.prev) !== null} fallback="—">
          <Delta pct={pctOver(r.day.price, r.prev)} />
        </Show>
      ),
    },
    { key: "sold", label: "Copies sold", width: "7rem", align: "end", cell: (r) => full(r.day.sold) },
    {
      key: "avg",
      label: "7-day average",
      align: "end",
      cell: (r) => <Robux amount={round(r.avg)} />,
    },
    {
      key: "rap",
      label: "vs RAP",
      width: "6rem",
      align: "end",
      cell: (r) => (
        <Show when={pctOver(r.day.price, props.rap) !== null} fallback="—">
          <Delta pct={pctOver(r.day.price, props.rap)} />
        </Show>
      ),
    },
  ]
  return <DataTable aria-label="Sales by day" columns={columns} rows={rows()} rowKey={(r) => r.day.t} />
}

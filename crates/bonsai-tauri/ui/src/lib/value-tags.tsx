import { createEffect, onCleanup, Show } from "solid-js"
import { commands, type ValueDemand, type ValueTag, type ValueTrend } from "~/lib/ipc/bindings"
import { shell } from "~/lib/store"
import { Badge, Icon, Pips, Robux, Tooltip } from "~/lib/ui"
import {
  demandLabel,
  demandLevel,
  trendIcon,
  trendLabel,
  trendTone,
  VALUES_REFRESH_MS,
  VALUES_SOURCE,
} from "~/lib/values"

/** Keeps values fresh while the calling screen is open: asks every two minutes, and the backend
 * refetches when its copy is that old. Nothing is fetched while values are off or no screen
 * that shows them is open. */
export function useValuesRefresh() {
  createEffect(() => {
    if (!shell.itemValues) return
    const timer = setInterval(() => void commands.valuesStatus(), VALUES_REFRESH_MS)
    onCleanup(() => clearInterval(timer))
  })
}

/** A Rolimons value in Robux. The tooltip says it is a value, not RAP or a price. */
export function ValueAmount(props: { amount: number | null; compact?: boolean; class?: string }) {
  return (
    <Tooltip content={`Value (${VALUES_SOURCE})`} class={`inline-flex items-center ${props.class ?? ""}`}>
      <Robux amount={props.amount} compact={props.compact} />
    </Tooltip>
  )
}

/** Demand as rising bars, colored from red (terrible) to green (amazing). */
export function DemandPips(props: { demand: ValueDemand; label?: boolean }) {
  return (
    <Tooltip content={`Demand: ${demandLabel(props.demand)}`} class="inline-flex items-center gap-1.5">
      <Pips
        label="Demand"
        valueLabel={demandLabel(props.demand)}
        value={demandLevel(props.demand)}
        max={5}
        tone="scale"
      />
      <Show when={props.label}>
        <span>{demandLabel(props.demand)}</span>
      </Show>
    </Tooltip>
  )
}

/** Trend as an arrow: up green, down red, wavy amber, flat muted. `plain` keeps the flat arrow
 * the surrounding color, for use on a picture. */
export function TrendMark(props: { trend: ValueTrend; label?: boolean; plain?: boolean }) {
  const tone = () => (props.plain && props.trend === "stable" ? "" : trendTone(props.trend))
  return (
    <Tooltip content={`Trend: ${trendLabel(props.trend)}`} class={`inline-flex items-center gap-1 ${tone()}`}>
      <Icon name={trendIcon(props.trend)} size={14} aria-label={trendLabel(props.trend)} />
      <Show when={props.label}>
        <span>{trendLabel(props.trend)}</span>
      </Show>
    </Tooltip>
  )
}

/** Projected, rare and hyped as badges. Projected comes first: it is a warning. */
export function ValueFlags(props: { tag: ValueTag }) {
  return (
    <>
      <Show when={props.tag.projected}>
        <Tooltip content="A few sales pushed RAP up; it will likely fall.">
          <Badge variant="warning">Projected</Badge>
        </Tooltip>
      </Show>
      <Show when={props.tag.rare}>
        <Badge variant="accent">
          <Icon name="rare" size={12} class="-ml-0.5 mr-0.5" />
          Rare
        </Badge>
      </Show>
      <Show when={props.tag.hyped}>
        <Badge>
          <Icon name="hyped" size={12} class="-ml-0.5 mr-0.5" />
          Hyped
        </Badge>
      </Show>
    </>
  )
}

/** The chip over an item tile's picture: projected warning, demand bars, trend arrow. Empty
 * (and hidden) when Rolimons says nothing about the item. */
export function ValueMarker(props: { tag: ValueTag }) {
  return (
    <Show when={props.tag.projected || props.tag.demand || props.tag.trend}>
      <Show when={props.tag.projected}>
        <Tooltip content="Projected: RAP will likely fall" class="inline-flex text-warning">
          <Icon name="alert" size={12} aria-label="Projected" />
        </Tooltip>
      </Show>
      <Show when={props.tag.demand}>{(d) => <DemandPips demand={d()} />}</Show>
      <Show when={props.tag.trend}>{(t) => <TrendMark trend={t()} plain />}</Show>
    </Show>
  )
}

/** Value, demand and trend on one line, for list rows such as trade items. */
export function ValueLine(props: { tag: ValueTag }) {
  return (
    <span class="inline-flex items-center gap-2">
      <Show when={props.tag.value}>{(v) => <ValueAmount amount={v()} />}</Show>
      <Show when={props.tag.demand}>{(d) => <DemandPips demand={d()} />}</Show>
      <Show when={props.tag.trend}>{(t) => <TrendMark trend={t()} />}</Show>
      <Show when={props.tag.projected}>
        <Tooltip content="A few sales pushed RAP up; it will likely fall.">
          <Badge variant="warning">Projected</Badge>
        </Tooltip>
      </Show>
    </span>
  )
}

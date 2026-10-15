import { For, Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import { ago, date } from "~/lib/format"
import type { TraderStats, TraderView } from "~/lib/ipc/bindings"
import { Badge, Card, Icon, Robux, Skeleton, Stat, TextLink, Tooltip } from "~/lib/ui"
import { ValueAmount } from "~/lib/value-tags"
import { VALUES_SOURCE } from "~/lib/values"
import {
  badgeDescription,
  badgeIcon,
  badgeRarity,
  badgeVariant,
  rarestFirst,
  type ShownBadge,
  topBadges,
} from "./trader"

/** Rolimons' view of a trader: what their limiteds are worth, their leaderboard rank and the
 * badges Rolimons gave them, with the source and age always shown. While values are off it is
 * one line offering to turn them on. */
export function TradingCard(props: { view: TraderView; enabled: boolean }) {
  return (
    <Card class="flex flex-col gap-3 px-3.5 py-3">
      <div class="flex flex-wrap items-center gap-1.5">
        <h3 class="text-sm font-medium text-fg-muted">Trading</h3>
        <Tooltip content={`Data from ${VALUES_SOURCE}, a third-party site`} class="inline-flex">
          <span class="flex items-center gap-1 text-xs text-fg-faint">
            <Icon name="rolimons" size={12} />
            {VALUES_SOURCE}
          </span>
        </Tooltip>
        <Show when={props.view.stats?.tracked && props.view.stats}>{(s) => <Flags stats={s()} />}</Show>
        <Show when={props.view.stats?.updatedAt}>
          {(t) => (
            <span class="ml-auto text-xs text-fg-faint" title={date(t())}>
              Updated {ago(t())}
            </span>
          )}
        </Show>
      </div>
      <Show
        when={props.enabled}
        fallback={
          <span class="text-sm text-fg-muted">
            Value, rank and badges from {VALUES_SOURCE}.{" "}
            <TextLink onClick={() => run("market.values")}>Turn on</TextLink>
          </span>
        }
      >
        <Show
          when={props.view.stats}
          fallback={
            <Show when={props.view.error} fallback={<Skeleton class="h-12 w-full rounded-md" />}>
              {(e) => <span class="text-sm text-fg-muted">{e()}</span>}
            </Show>
          }
        >
          {(s) => (
            <Show
              when={s().tracked}
              fallback={<span class="text-sm text-fg-muted">{VALUES_SOURCE} hasn't scanned this player.</span>}
            >
              <Numbers stats={s()} />
              <Show when={s().badges.length > 0}>
                <div class="flex flex-wrap gap-1.5">
                  <For each={rarestFirst(topBadges(s().badges))}>{(b) => <RoliBadge shown={b} />}</For>
                </div>
              </Show>
            </Show>
          )}
        </Show>
      </Show>
    </Card>
  )
}

function Flags(props: { stats: TraderStats }) {
  return (
    <>
      <Show when={props.stats.terminated}>
        <Badge variant="danger">Terminated</Badge>
      </Show>
      <Show when={props.stats.private}>
        <Tooltip content="Their inventory is hidden, so these numbers are from before.">
          <Badge variant="warning">
            <Icon name="lock" size={12} class="-ml-0.5 mr-0.5" />
            Private
          </Badge>
        </Tooltip>
      </Show>
      <Show when={props.stats.premium}>
        <Badge>Premium</Badge>
      </Show>
    </>
  )
}

function Numbers(props: { stats: TraderStats }) {
  return (
    <dl class="grid grid-cols-[repeat(auto-fill,minmax(8rem,1fr))] gap-4">
      <Stat label="Value">
        <ValueAmount class="text-lg font-medium" amount={props.stats.value} />
      </Stat>
      <Stat label="RAP">
        <Robux class="text-lg font-medium" amount={props.stats.rap} />
      </Stat>
      <Stat label="Rank">
        <Show when={props.stats.rank} fallback={<span class="text-lg text-fg-muted">Unranked</span>}>
          {(r) => (
            <Tooltip content={`By value, on ${VALUES_SOURCE}`} class="inline-flex">
              <span class="text-lg font-medium">#{r().toLocaleString("en-US")}</span>
            </Tooltip>
          )}
        </Show>
      </Stat>
    </dl>
  )
}

/** One badge; the highest tier of a family stands for the ones below it. Color shows its rarity. */
function RoliBadge(props: { shown: ShownBadge }) {
  const b = () => props.shown.badge
  return (
    <Tooltip
      content={
        <span class="flex flex-col">
          <span>
            {b().group === "value" ? `Inventory worth ${b().name}` : b().name}
            <span class="ml-1 text-fg-muted capitalize">· {badgeRarity(b())}</span>
          </span>
          <Show when={badgeDescription(b())}>{(d) => <span class="text-fg-muted">{d()}</span>}</Show>
          <Show when={b().earned}>{(t) => <span class="text-fg-muted">Earned {date(t())}</span>}</Show>
          <Show when={props.shown.lower.length > 0}>
            <span class="text-fg-muted">Also {props.shown.lower.map((l) => l.name).join(", ")}</span>
          </Show>
        </span>
      }
    >
      <Badge variant={badgeVariant(b())}>
        <Icon name={badgeIcon(b().group)} size={12} class="-ml-0.5 mr-0.5" />
        {b().name}
      </Badge>
    </Tooltip>
  )
}

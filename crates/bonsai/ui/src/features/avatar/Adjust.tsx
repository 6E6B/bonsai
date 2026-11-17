import { createEffect, createSignal, For, Show } from "solid-js"
import { menuItem, run } from "~/lib/commands/registry"
import type { AdjustRuleView, AvatarView, WornView } from "~/lib/ipc/bindings"
import { thumbUrl } from "~/lib/thumb"
import { ItemName } from "~/lib/tiles"
import {
  Button,
  EmptyState,
  IconButton,
  ListItem,
  Menu,
  type MenuEntry,
  SectionHeader,
  Slider,
  Thumbnail,
} from "~/lib/ui"

import {
  type Axis,
  adjustable,
  axisValue,
  copyArg,
  DEFAULT_PLACEMENT,
  formatValue,
  GROUPS,
  type Group,
  isAdjusted,
  resetArg,
  ruleFor,
  setArg,
} from "./adjust"

/** Moving, turning and sizing worn accessories within the bounds Roblox sets per type. */
export function Adjust(props: { view: AvatarView; selected: number | null; onSelect: (id: number) => void }) {
  const items = () => adjustable(props.view)
  const current = () => items().find((w) => w.id === props.selected) ?? items()[0]
  return (
    <Show
      when={current()}
      fallback={
        <EmptyState
          icon="avatar"
          title="Nothing to adjust"
          description="Hats and face, neck, shoulder, front and back accessories can be moved, turned and resized."
        />
      }
    >
      {(item) => (
        <div class="flex flex-col gap-4 md:flex-row">
          <div class="-mx-2 flex shrink-0 flex-col md:w-56">
            <For each={items()}>
              {(w) => (
                <ListItem
                  leading={<Thumbnail class="w-8" alt="" fallback="avatar" src={thumbUrl("asset", w.id)} />}
                  title={<ItemName name={w.name} id={w.id} />}
                  subtitle={isAdjusted(w.placement) ? `${w.typeName} · Adjusted` : w.typeName}
                  selected={w.id === item().id}
                  onClick={() => props.onSelect(w.id)}
                />
              )}
            </For>
          </div>
          <Show when={ruleFor(props.view, item().typeId)}>
            {(rule) => <Editor view={props.view} item={item()} rule={rule()} others={items()} />}
          </Show>
        </div>
      )}
    </Show>
  )
}

function Editor(props: { view: AvatarView; item: WornView; rule: AdjustRuleView; others: WornView[] }) {
  const placement = () => props.item.placement ?? DEFAULT_PLACEMENT
  const copyMenu = (): MenuEntry[] => {
    const others = props.others.filter((o) => o.id !== props.item.id)
    if (!others.length) return [{ kind: "heading", label: "No other adjustable items worn" }]
    return [
      { kind: "heading", label: "Same as" },
      ...others.map((o) => menuItem("avatar.adjust", { arg: copyArg(props.item.id, o.id, false), label: o.name })),
      { kind: "separator" },
      { kind: "heading", label: "Mirror of" },
      ...others.map((o) => menuItem("avatar.adjust", { arg: copyArg(props.item.id, o.id, true), label: o.name })),
    ]
  }
  return (
    <section class="flex min-w-0 flex-1 flex-col gap-4">
      <div class="flex flex-wrap items-center gap-2">
        <SectionHeader title={props.item.name} meta={props.item.typeName} size="group" class="flex-1" />
        <Menu size="sm" icon="copy" label="Copy placement" items={copyMenu()}>
          Copy from
        </Menu>
        <Button
          size="sm"
          icon="undo"
          disabled={!isAdjusted(props.item.placement)}
          onClick={() => void run("avatar.adjust", resetArg(props.item.id))}
        >
          Reset all
        </Button>
      </div>
      <For each={GROUPS}>
        {(g) => (
          <div class="flex flex-col gap-2">
            <div class="flex items-center gap-2">
              <SectionHeader title={g.label} size="group" class="flex-1" />
              <IconButton
                size="sm"
                icon="undo"
                label={`Reset ${g.label.toLowerCase()}`}
                disabled={!isAdjusted(props.item.placement, g.key)}
                onClick={() => void run("avatar.adjust", resetArg(props.item.id, g.key))}
              />
            </div>
            <div class="grid grid-cols-1 gap-x-6 gap-y-3 sm:grid-cols-3">
              <For each={g.axes}>
                {(a) => (
                  <AxisSlider
                    id={props.item.id}
                    group={g.key}
                    axis={a.axis}
                    label={a.label}
                    neutral={g.neutral}
                    value={axisValue(placement(), g.key, a.axis)}
                    min={axisValue(props.rule.min, g.key, a.axis)}
                    max={axisValue(props.rule.max, g.key, a.axis)}
                    step={props.view.adjustSteps[g.key] ?? 0.01}
                  />
                )}
              </For>
            </div>
          </div>
        )}
      </For>
      <p class="text-xs text-fg-muted">
        Roblox limits each axis by accessory type. Double-click a slider or press Delete to reset it.
      </p>
    </section>
  )
}

function AxisSlider(props: {
  id: number
  group: Group
  axis: Axis
  label: string
  neutral: number
  value: number
  min: number
  max: number
  step: number
}) {
  // Follows the slider while dragging; the avatar catches up after the write.
  const [value, setValue] = createSignal(props.value)
  createEffect(() => setValue(props.value))
  const commit = (v: number) => {
    if (Math.abs(v - props.value) < 1e-6) return
    void run("avatar.adjust", setArg(props.id, props.group, props.axis, v))
  }
  return (
    <Slider
      label={props.label}
      value={value()}
      min={props.min}
      max={props.max}
      step={props.step}
      marks={[props.neutral]}
      resetValue={props.neutral}
      format={(v) => formatValue(props.group, v)}
      onChange={setValue}
      onChangeEnd={commit}
    />
  )
}

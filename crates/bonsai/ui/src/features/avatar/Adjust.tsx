import { createEffect, createSignal, For, on, onCleanup, Show } from "solid-js"
import type { AdjustTarget } from "~/lib/avatar3d/viewer"
import { handle, menuItem, run } from "~/lib/commands/registry"
import type { AdjustRuleView, AvatarView, WornView } from "~/lib/ipc/bindings"
import { shell } from "~/lib/store"
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
  SegmentedControl,
  Slider,
  Thumbnail,
} from "~/lib/ui"

import {
  adjustable,
  copyArg,
  formatValue,
  GROUPS,
  isAdjusted,
  type Placement,
  parseTool,
  placementArg,
  resetArg,
  ruleFor,
  type Tool,
  toPlacement,
  withAxis,
} from "./adjust-rules"

const TOOL_OPTIONS = [
  { value: "move", label: "Move", hint: "W" },
  { value: "turn", label: "Turn", hint: "E" },
  { value: "size", label: "Size", hint: "R" },
]

/** How long to wait for the edited look after a change before dropping the dragged placement. */
const SETTLE_MS = 400

/** The accessory being adjusted and its handle, shared by the stage and the Adjust panel. */
export type AdjustState = ReturnType<typeof createAdjust>

/**
 * Moving, turning and sizing worn accessories within the bounds Roblox sets per type. The
 * avatar shows each change as it is dragged; it goes into the edited look when the drag ends.
 */
export function createAdjust(view: () => AvatarView, selected: () => number | null) {
  const items = () => adjustable(view())
  const current = () => items().find((w) => w.id === selected()) ?? items()[0]
  const rule = () => {
    const c = current()
    return c && ruleFor(view(), c.typeId)
  }
  const [tool, setTool] = createSignal<Tool>("move")
  onCleanup(
    handle("avatar.adjust_tool", (arg) => {
      const t = parseTool(arg)
      if (t) setTool(t)
    }),
  )

  // What the avatar shows while a change is dragged or landing in the edited look.
  const [draft, setDraft] = createSignal<{ id: number; placement: Placement } | null>(null)
  const [saving, setSaving] = createSignal(false)
  const saved = () => toPlacement(current()?.placement ?? null)
  const shown = () => {
    const d = draft()
    return d && d.id === current()?.id ? d.placement : saved()
  }
  // A landed change replaces the dragged one as soon as it arrives.
  createEffect(
    on(
      () => JSON.stringify([current()?.id, current()?.placement]),
      () => {
        setDraft(null)
        setSaving(false)
      },
      { defer: true },
    ),
  )
  // A change that failed, or that changed nothing, leaves the placement where it was.
  createEffect(() => {
    if (!saving() || shell.jobs > 0) return
    const t = setTimeout(() => {
      setDraft(null)
      setSaving(false)
    }, SETTLE_MS)
    onCleanup(() => clearTimeout(t))
  })
  const change = (id: number, placement: Placement) => setDraft({ id, placement })
  const commit = (id: number, placement: Placement) => {
    const was = items().find((w) => w.id === id)
    const arg = was && placementArg(id, placement, toPlacement(was.placement))
    if (!arg) return setDraft(null)
    setDraft({ id, placement })
    setSaving(true)
    void run("avatar.adjust", arg).then((ok) => {
      if (!ok) {
        setDraft(null)
        setSaving(false)
      }
    })
  }
  /** The handle the stage draws on the current accessory. */
  const target = (): AdjustTarget | undefined => {
    const item = current()
    const r = rule()
    if (!item || !r) return undefined
    const steps = view().adjustSteps
    return {
      id: item.id,
      placement: shown(),
      tool: tool(),
      min: toPlacement(r.min),
      max: toPlacement(r.max),
      steps: { position: steps.position ?? 0.01, rotation: steps.rotation ?? 0.1, scale: steps.scale ?? 0.01 },
      onChange: (p) => change(item.id, p),
      onCommit: (p) => commit(item.id, p),
    }
  }
  /** The move, turn and size switch for the stage's corner. */
  const toolSwitch = () => (
    <SegmentedControl
      aria-label="Adjust handle"
      size="sm"
      options={TOOL_OPTIONS}
      value={tool()}
      onChange={(v) => void run("avatar.adjust_tool", v)}
    />
  )
  return { items, current, rule, shown, change, commit, target, toolSwitch }
}

/** The Adjust panel: the adjustable items and the sliders for the current one. The handle is on
 * the editor's stage. */
export function Adjust(props: { view: AvatarView; state: AdjustState; onSelect: (id: number) => void }) {
  return (
    <Show
      when={props.state.current()}
      fallback={
        <EmptyState
          icon="avatar"
          title="Nothing to adjust"
          description="Hats and face, neck, shoulder, front and back accessories can be moved, turned and resized."
        />
      }
    >
      {(item) => (
        <div class="flex flex-col gap-4">
          <div class="-mx-2 flex flex-col">
            <For each={props.state.items()}>
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
          <Show when={props.state.rule()}>
            {(r) => (
              <Editor
                item={item()}
                rule={r()}
                others={props.state.items()}
                canUndo={props.view.canUndo}
                canRedo={props.view.canRedo}
                steps={props.view.adjustSteps}
                placement={props.state.shown()}
                onChange={(p) => props.state.change(item().id, p)}
                onCommit={(p) => props.state.commit(item().id, p)}
              />
            )}
          </Show>
        </div>
      )}
    </Show>
  )
}

function Editor(props: {
  item: WornView
  rule: AdjustRuleView
  others: WornView[]
  canUndo: boolean
  canRedo: boolean
  steps: AvatarView["adjustSteps"]
  placement: Placement
  onChange: (p: Placement) => void
  onCommit: (p: Placement) => void
}) {
  const min = () => toPlacement(props.rule.min)
  const max = () => toPlacement(props.rule.max)
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
    <section class="flex min-w-0 flex-col gap-4">
      <div class="flex flex-col gap-2">
        <SectionHeader title={props.item.name} meta={props.item.typeName} size="group" />
        <div class="flex items-center gap-1">
          <IconButton
            size="sm"
            icon="undo"
            label="Undo (Ctrl+Z)"
            disabled={!props.canUndo}
            onClick={() => void run("avatar.undo")}
          />
          <IconButton
            size="sm"
            icon="redo"
            label="Redo (Ctrl+Shift+Z)"
            disabled={!props.canRedo}
            onClick={() => void run("avatar.redo")}
          />
          <span class="flex-1" />
          <Menu size="sm" icon="copy" label="Copy placement" items={copyMenu()}>
            Copy from
          </Menu>
          <Button
            size="sm"
            icon="refresh"
            disabled={!isAdjusted(props.item.placement)}
            onClick={() => void run("avatar.adjust", resetArg(props.item.id))}
          >
            Reset
          </Button>
        </div>
      </div>
      <For each={GROUPS}>
        {(g) => (
          <div class="flex flex-col gap-2">
            <div class="flex items-center gap-2">
              <SectionHeader title={g.label} size="group" class="flex-1" />
              <IconButton
                size="sm"
                icon="refresh"
                label={`Reset ${g.label.toLowerCase()}`}
                disabled={!isAdjusted(props.item.placement, g.key)}
                onClick={() => void run("avatar.adjust", resetArg(props.item.id, g.key))}
              />
            </div>
            <For each={g.axes}>
              {(a) => (
                <Slider
                  label={a.label}
                  value={props.placement[g.key][a.axis]}
                  min={min()[g.key][a.axis]}
                  max={max()[g.key][a.axis]}
                  step={props.steps[g.key] ?? 0.01}
                  marks={[g.neutral]}
                  resetValue={g.neutral}
                  format={(v) => formatValue(g.key, v)}
                  onChange={(v) => props.onChange(withAxis(props.placement, g.key, a.axis, v))}
                  onChangeEnd={(v) => props.onCommit(withAxis(props.placement, g.key, a.axis, v))}
                />
              )}
            </For>
          </div>
        )}
      </For>
      <p class="text-xs text-fg-muted">
        Drag the handle on the avatar or a slider. Changes stay in the editor until you wear them. Roblox limits each
        axis by accessory type.
      </p>
    </section>
  )
}

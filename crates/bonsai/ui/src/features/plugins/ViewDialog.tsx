import { createEffect, createSignal, For, Match, on, Show, Switch } from "solid-js"
import { createStore, reconcile } from "solid-js/store"
import { full } from "~/lib/format"
import type { PluginViewBody, ViewButtonStyle, ViewCell, ViewNode } from "~/lib/ipc/bindings"
import { commands } from "~/lib/ipc/bindings"
import { shell } from "~/lib/store"
import {
  Alert,
  Button,
  type ButtonVariant,
  type Column,
  DataTable,
  Dialog,
  SectionHeader,
  Stat,
  TextField,
} from "~/lib/ui"
import { cellText, mergeValues } from "./view"

type Row = { index: number; cells: ViewCell[] }

/**
 * A plugin's view: a tree of our components a plugin command returned. Opens by itself from
 * `shell.pluginView`. Buttons, Enter in an input and picked table rows go back to the plugin
 * with every input's text; the plugin answers with a new tree, or closes it.
 */
export function ViewDialog() {
  const head = () => shell.pluginView
  const [body, setBody] = createSignal<PluginViewBody | null>(null)
  const [values, setValues] = createStore<Record<string, string>>({})
  const [error, setError] = createSignal("")

  // Fetch the tree when a view opens or the plugin redraws it.
  createEffect(
    on(
      () => (head() ? `${head()?.seq}:${head()?.rev}` : ""),
      async (key) => {
        if (!key) {
          setBody(null)
          setValues(reconcile({}))
          return
        }
        const next = await commands.pluginView()
        // A later redraw fetches its own tree.
        if (!next || `${next.seq}:${next.rev}` !== key) return
        const prev = body()
        const same = prev !== null && prev.seq === next.seq
        setValues(reconcile(mergeValues(same ? prev.root : null, next.root, same ? { ...values } : {})))
        setBody(next)
        setError("")
      },
    ),
  )

  const send = async (action: string, row?: number) => {
    const h = head()
    if (!h || h.busy) return
    setError("")
    const list = Object.entries(values).map(([key, value]) => ({ key, value }))
    const r = await commands.pluginViewEvent(h.seq, action, list, row ?? null)
    if (r.status === "error") setError(r.error)
  }

  const message = () => error() || head()?.error || ""

  return (
    <Dialog
      open={head() !== null}
      onOpenChange={(open) => {
        if (!open) void commands.pluginViewClose()
      }}
      title={head() ? `${head()?.pluginName}: ${head()?.title}` : ""}
      size="lg"
    >
      <div class="flex min-h-0 flex-1 flex-col gap-3 overflow-y-auto p-4" aria-busy={head()?.busy ?? false}>
        <Show when={body()}>
          {(b) => (
            <Part
              node={b().root}
              values={values}
              busy={head()?.busy ?? false}
              onInput={(key, v) => setValues(key, v)}
              onAction={(a, row) => void send(a, row)}
            />
          )}
        </Show>
        <Show when={message()}>
          <Alert variant="danger" title={message()} />
        </Show>
      </div>
    </Dialog>
  )
}

const variants: Record<ViewButtonStyle, ButtonVariant> = {
  default: "secondary",
  primary: "primary",
  danger: "danger",
}

interface PartProps {
  node: ViewNode
  values: Record<string, string>
  busy: boolean
  onInput: (key: string, value: string) => void
  onAction: (action: string, row?: number) => void
}

/** One node and its children. */
function Part(props: PartProps) {
  const children = (list: ViewNode[]) => (
    <For each={list}>
      {(c) => (
        <Part node={c} values={props.values} busy={props.busy} onInput={props.onInput} onAction={props.onAction} />
      )}
    </For>
  )
  return (
    <Switch>
      <Match when={props.node.type === "text" && props.node}>
        {(n) => (
          <p class="whitespace-pre-wrap text-sm" classList={{ "text-fg-muted": n().muted, "font-mono": n().mono }}>
            {n().text}
          </p>
        )}
      </Match>
      <Match when={props.node.type === "heading" && props.node}>
        {(n) => <SectionHeader title={n().text} size="group" />}
      </Match>
      <Match when={props.node.type === "row" && props.node}>
        {(n) => <div class="flex flex-wrap items-end gap-2">{children(n().children)}</div>}
      </Match>
      <Match when={props.node.type === "column" && props.node}>
        {(n) => <div class="flex flex-col gap-3">{children(n().children)}</div>}
      </Match>
      <Match when={props.node.type === "stat" && props.node}>
        {(n) => <Stat label={n().label}>{cellText(n().value, full)}</Stat>}
      </Match>
      <Match when={props.node.type === "table" && props.node}>
        {(n) => {
          const rows = () => n().rows.map((cells, index): Row => ({ index, cells }))
          const columns = () =>
            n().columns.map(
              (c, i): Column<Row> => ({
                key: String(i),
                label: c.label,
                align: c.end ? "end" : "start",
                cell: (r) => cellText(r.cells[i] ?? "", full),
              }),
            )
          const action = () => n().action
          return (
            <DataTable
              aria-label="Plugin table"
              columns={columns()}
              rows={rows()}
              rowKey={(r) => r.index}
              onRowClick={
                action()
                  ? (r) => {
                      const a = action()
                      if (a && !props.busy) props.onAction(a, r.index + 1)
                    }
                  : undefined
              }
              empty={<p class="p-3 text-sm text-fg-muted">Nothing here.</p>}
            />
          )
        }}
      </Match>
      <Match when={props.node.type === "input" && props.node}>
        {(n) => (
          <TextField
            label={n().label || undefined}
            placeholder={n().placeholder}
            value={props.values[n().key] ?? ""}
            onChange={(v) => props.onInput(n().key, v)}
            size="sm"
            onKeyDown={(e) => {
              const a = n().action
              if (e.key === "Enter" && a && !props.busy) {
                e.preventDefault()
                props.onAction(a)
              }
            }}
          />
        )}
      </Match>
      <Match when={props.node.type === "button" && props.node}>
        {(n) => (
          <Button
            variant={variants[n().style]}
            size="sm"
            disabled={props.busy}
            onClick={() => props.onAction(n().action)}
          >
            {n().label}
          </Button>
        )}
      </Match>
    </Switch>
  )
}

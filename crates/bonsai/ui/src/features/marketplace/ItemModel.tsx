import { createEffect, createMemo, on, onCleanup, Show } from "solid-js"
import { createStore, reconcile } from "solid-js/store"
import { useListCursor } from "~/lib/commands/hooks"
import { handle, run, runBackend } from "~/lib/commands/registry"
import { commands, type ModelNodeView, type ModelPropView, type ModelStatusView } from "~/lib/ipc/bindings"
import { itemTarget } from "~/lib/menus"
import { shell, useLive } from "~/lib/store"
import {
  Alert,
  type Column,
  DataTable,
  EmptyState,
  Icon,
  SegmentedControl,
  Skeleton,
  Swatch,
  TextField,
  TextLink,
  VirtualList,
} from "~/lib/ui"
import { allParents, nodePath, visibleRows } from "./model-tree"

const propKey = (p: ModelPropView) => (p.attribute ? `@${p.name}` : p.name)

const FORMATS = [
  { value: "source", label: "Source" },
  { value: "processed", label: "Processed" },
]

/** The decoded instance tree and the selected instance's properties. Mounted only while the
 * tab is open; decoding stays in core and nothing in the model is run. */
export function ItemModel(props: { id: number; assetType: number }) {
  const [ui, setUi] = createStore({
    processed: false,
    query: "",
    collapsed: {} as Record<number, boolean>,
    /** Index into `rows()`. */
    cursor: 0,
    nodes: [] as ModelNodeView[],
    props: [] as ModelPropView[],
    /** Property name under the cursor in the properties table. */
    prop: "Name",
  })
  const status = useLive(
    () => [shell.active, props.id, props.assetType, ui.processed] as const,
    ([, id, type, processed]) => commands.itemModel(id, type, processed),
    (): ModelStatusView => ({ loading: true, error: null, rev: 0, count: 0, hasProcessed: false }),
  )
  // Nodes are fetched once per loaded tree, not on every store change.
  createEffect(
    on(
      () => [status.rev, ui.processed] as const,
      ([rev, processed]) => {
        if (!rev) return setUi("nodes", [])
        void commands.itemModelNodes(props.id, props.assetType, processed).then((nodes) => {
          setUi("nodes", reconcile(nodes))
          setUi("cursor", (c) => Math.min(c, Math.max(0, nodes.length - 1)))
        })
      },
    ),
  )
  const rows = createMemo(() => visibleRows(ui.nodes, ui.collapsed, ui.query))
  const selected = () => rows()[ui.cursor] as number | undefined
  createEffect(
    on(
      () => [selected(), status.rev, ui.processed] as const,
      ([node, rev, processed]) => {
        if (node === undefined || !rev) return setUi("props", [])
        void commands.itemModelProperties(props.id, props.assetType, processed, node).then((p) => {
          setUi("props", reconcile(p, { key: "name" }))
          if (!p.some((x) => propKey(x) === ui.prop)) setUi("prop", "Name")
        })
      },
    ),
  )

  const select = (node: number) => {
    // Reveal a Ref target that sits under a collapsed ancestor.
    for (let at = ui.nodes[node]?.parent ?? null; at !== null; at = ui.nodes[at]?.parent ?? null) {
      if (ui.collapsed[at]) setUi("collapsed", at, false)
    }
    if (!rows().includes(node)) setUi("query", "")
    const i = rows().indexOf(node)
    if (i >= 0) setUi("cursor", i)
  }
  const toggle = (node: number | undefined, open?: boolean) => {
    if (node === undefined || !ui.nodes[node]?.children || ui.query.trim()) return
    setUi("collapsed", node, open === undefined ? !ui.collapsed[node] : !open)
  }

  useListCursor({
    count: () => rows().length,
    index: () => ui.cursor,
    setIndex: (i) => setUi("cursor", i),
    onOpen: () => toggle(selected()),
  })
  const off = [
    // Left collapses, or goes to the parent; Right expands, or goes to the first child.
    handle("list.left", () => {
      const node = selected()
      if (node === undefined) return
      if (ui.nodes[node].children && !ui.collapsed[node] && !ui.query.trim()) return toggle(node, false)
      const parent = ui.nodes[node].parent
      if (parent !== null) select(parent)
    }),
    handle("list.right", () => {
      const node = selected()
      if (node === undefined || !ui.nodes[node].children) return
      if (ui.collapsed[node]) return toggle(node, true)
      setUi("cursor", (c) => Math.min(c + 1, rows().length - 1))
    }),
    handle("market.model_toggle", () => toggle(selected())),
    handle("market.model_expand_all", () => setUi("collapsed", reconcile({}))),
    handle("market.model_collapse_all", () => {
      setUi("collapsed", reconcile(allParents(ui.nodes)))
      setUi("cursor", 0)
    }),
    handle("market.model_format", (arg) => {
      if (!status.hasProcessed) return
      const want = arg?.trim().toLowerCase()
      const processed = want === "processed" ? true : want === "source" ? false : !ui.processed
      setUi({ processed, collapsed: {}, cursor: 0, query: "" })
    }),
    handle("market.model_search", (arg) => {
      if (arg !== null) setUi({ query: arg, cursor: 0 })
      search?.focus()
    }),
    handle("market.model_copy_path", () => {
      const node = selected()
      if (node !== undefined) return runBackend("market.model_copy_path", nodePath(ui.nodes, node))
    }),
    handle("market.model_copy_value", (arg) => {
      const name = (arg?.trim() || ui.prop).toLowerCase()
      const p =
        ui.props.find((p) => propKey(p).toLowerCase() === name) ?? ui.props.find((p) => p.name.toLowerCase() === name)
      if (p) return runBackend("market.model_copy_value", p.value)
    }),
  ]
  onCleanup(() => {
    for (const f of off) f()
  })
  let search: HTMLInputElement | undefined

  const columns: Column<ModelPropView>[] = [
    {
      key: "name",
      label: "Property",
      width: "minmax(8rem, 1fr)",
      cell: (p) => (
        <span class="truncate" classList={{ "text-fg-muted": p.attribute }} title={p.attribute ? "Attribute" : p.name}>
          {p.name}
        </span>
      ),
    },
    {
      key: "value",
      label: "Value",
      width: "minmax(0, 2fr)",
      cell: (p) => (
        <div class="flex min-w-0 items-center gap-2">
          <Show when={p.color}>{(c) => <Swatch display color={c()} label={p.value} />}</Show>
          <Show
            when={p.asset ?? (p.target !== null ? -1 : null)}
            fallback={
              <span class="truncate" title={p.value}>
                {p.value}
                <Show when={p.truncated}>…</Show>
              </span>
            }
          >
            <TextLink
              class="truncate"
              onClick={() => {
                if (p.asset !== null) void run("market.open", itemTarget(p.asset))
                else if (p.target !== null) select(p.target)
              }}
            >
              {p.value}
            </TextLink>
          </Show>
        </div>
      ),
    },
    {
      key: "kind",
      label: "Type",
      width: "minmax(5rem, 0.6fr)",
      cell: (p) => <span class="truncate text-xs text-fg-faint">{p.attribute ? `Attribute · ${p.kind}` : p.kind}</span>,
    },
  ]

  return (
    <div class="flex min-w-0 flex-col gap-3" aria-busy={status.loading}>
      <div class="flex flex-wrap items-center justify-between gap-3">
        <div class="flex flex-col gap-1">
          <span class="text-sm text-fg-muted">
            The instances in this model and their properties. Scripts never run.
          </span>
          <Show when={status.count > 0}>
            <span class="text-xs text-fg-faint" aria-live="polite">
              {status.count} instances
            </span>
          </Show>
        </div>
        <div class="flex items-center gap-2">
          <Show when={status.hasProcessed}>
            <SegmentedControl
              aria-label="Model format"
              size="sm"
              options={FORMATS}
              value={ui.processed ? "processed" : "source"}
              onChange={(v) => void run("market.model_format", v)}
            />
          </Show>
          <Show when={status.count > 0}>
            <TextField
              ref={search}
              class="w-56"
              icon="search"
              aria-label="Search instances"
              placeholder="Name or class"
              value={ui.query}
              onChange={(value) => void run("market.model_search", value)}
            />
          </Show>
        </div>
      </div>
      <Show when={status.error}>
        {(error) => (
          <Alert
            variant={status.count > 0 ? "warning" : "danger"}
            title={status.count > 0 ? "Showing cached model" : "Couldn't load this model"}
          >
            {error()}
          </Alert>
        )}
      </Show>
      <Show
        when={ui.nodes.length > 0}
        fallback={
          <Show when={status.loading && !status.error}>
            <Skeleton class="h-96" />
          </Show>
        }
      >
        <div class="grid h-[32rem] min-h-0 grid-cols-[minmax(14rem,2fr)_3fr] gap-3">
          <div
            class="min-h-0 overflow-y-auto rounded-md border border-line bg-surface p-1"
            data-cursor-list
            role="tree"
            aria-label="Instances"
          >
            <VirtualList
              items={rows()}
              estimateRowHeight={26}
              activeIndex={ui.cursor}
              itemKey={(node) => node}
              empty={
                <EmptyState icon="search" title="No matching instances" description="Try another name or class." />
              }
            >
              {(node, i) => {
                const n = () => ui.nodes[node]
                return (
                  <div
                    role="treeitem"
                    tabIndex={i === ui.cursor ? 0 : -1}
                    aria-selected={i === ui.cursor}
                    aria-expanded={n().children > 0 ? !ui.collapsed[node] : undefined}
                    aria-level={n().depth + 1}
                    data-index={i}
                    class="flex h-[26px] cursor-default items-center gap-1 rounded-sm pr-2 text-sm hover:bg-hover"
                    classList={{ "bg-active": i === ui.cursor }}
                    style={{ "padding-left": `${n().depth * 14 + 4}px` }}
                    onClick={() => setUi("cursor", i)}
                    onDblClick={() => toggle(node)}
                    onKeyDown={(e) => {
                      // Enter and the arrow keys come through the shortcut commands.
                      if (e.key !== " ") return
                      e.preventDefault()
                      toggle(node)
                    }}
                  >
                    <Show when={n().children > 0} fallback={<span class="w-4 shrink-0" />}>
                      <button
                        type="button"
                        class="flex w-4 shrink-0 items-center justify-center text-fg-faint"
                        tabIndex={-1}
                        aria-label={ui.collapsed[node] ? "Expand" : "Collapse"}
                        onClick={(e) => {
                          e.stopPropagation()
                          setUi("cursor", i)
                          void run("market.model_toggle")
                        }}
                      >
                        <Icon
                          name={ui.collapsed[node] && !ui.query.trim() ? "chevron-right" : "chevron-down"}
                          size={12}
                        />
                      </button>
                    </Show>
                    <span class="truncate">{n().name}</span>
                    <span class="shrink-0 text-xs text-fg-faint">{n().class}</span>
                  </div>
                )
              }}
            </VirtualList>
          </div>
          <div class="min-h-0 overflow-y-auto">
            <DataTable
              aria-label="Instance properties"
              columns={columns}
              rows={ui.props}
              rowKey={propKey}
              selected={ui.prop}
              onRowClick={(p) => setUi("prop", propKey(p))}
              rowHeight={32}
            />
          </div>
        </div>
      </Show>
      <Show when={!status.loading && !status.error && status.rev && status.count === 0}>
        <EmptyState icon="inventory" title="Empty model" description="This model has no instances." />
      </Show>
    </div>
  )
}

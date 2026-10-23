import { createMemo, onCleanup, Show } from "solid-js"
import { createStore } from "solid-js/store"
import { assetIcon } from "~/lib/asset-icon"
import { handle, run } from "~/lib/commands/registry"
import { commands, type DependenciesView, type DependencyView } from "~/lib/ipc/bindings"
import { emptyList, ListState } from "~/lib/list-state"
import { entityMenu, itemTarget } from "~/lib/menus"
import { shell, useLive } from "~/lib/store"
import { thumbUrl } from "~/lib/thumb"
import {
  Alert,
  type Column,
  DataTable,
  EmptyState,
  Menu,
  TextField,
  TextLink,
  Thumbnail,
  useDialog,
  VerifiedBadge,
} from "~/lib/ui"
import { filterDependencies, usageLabels } from "./dependencies"

/** Mounted only while the tab is open. Images stay on thumb:// and all model work stays in core. */
export function Dependencies(props: { id: number; assetType: number }) {
  const view = useLive(
    () => [shell.active, props.id, props.assetType] as const,
    ([, id, type]) => commands.itemDependencies(id, type),
    (): DependenciesView => ({ list: emptyList<DependencyView>(), note: null }),
  )
  const [filter, setFilter] = createStore({ query: "" })
  const dialog = useDialog()
  let inspecting = false
  onCleanup(() => {
    if (inspecting) dialog.close()
  })
  onCleanup(
    handle("market.dependency_uses", (arg) => {
      const item = view.list.items.find((item) => item.id === Number(arg))
      if (!item) return
      dialog.show(
        () => (
          <div class="min-h-32 max-h-80 overflow-y-auto p-4">
            <DataTable
              aria-label="Asset references"
              rows={item.uses}
              rowKey={(use) => `${use.path}:${use.class}:${use.property}`}
              columns={[
                {
                  key: "path",
                  label: "Instance",
                  width: "minmax(10rem, 2fr)",
                  cell: (use) => (
                    <span class="truncate" title={use.path}>
                      {use.path}
                    </span>
                  ),
                },
                { key: "class", label: "Class", cell: (use) => <span class="truncate">{use.class}</span> },
                { key: "property", label: "Property", cell: (use) => <span class="truncate">{use.property}</span> },
              ]}
            />
          </div>
        ),
        {
          title: `References to ${item.name ?? `asset ${item.id}`}`,
          size: "lg",
          onClose: () => {
            inspecting = false
          },
        },
      )
      inspecting = true
    }),
  )
  let search: HTMLInputElement | undefined
  onCleanup(
    handle("market.dependencies_search", (arg) => {
      if (arg !== null) setFilter("query", arg)
      search?.focus()
    }),
  )
  const rows = createMemo(() => filterDependencies(view.list.items, filter.query))
  const columns: Column<DependencyView>[] = [
    {
      key: "asset",
      label: "Asset",
      width: "minmax(0, 2fr)",
      cell: (item) => (
        <div class="flex min-w-0 items-center gap-3">
          <Thumbnail
            class="w-10 shrink-0"
            src={thumbUrl("asset", item.id)}
            fallback={assetIcon(item.assetType)}
            alt={item.name ?? `Asset ${item.id}`}
          />
          <div class="flex min-w-0 flex-col gap-0.5">
            <TextLink class="truncate" onClick={() => run("market.open", itemTarget(item.id))}>
              {item.name ?? `Asset ${item.id}`}
            </TextLink>
            <span class="truncate text-xs text-fg-muted">
              {item.name === null ? "Details unavailable" : item.typeName}
            </span>
          </div>
        </div>
      ),
    },
    {
      key: "usage",
      label: "Used for",
      width: "minmax(0, 1.5fr)",
      cell: (item) => (
        <TextLink class="truncate text-sm" onClick={() => run("market.dependency_uses", String(item.id))}>
          {usageLabels(item).join(", ")}
        </TextLink>
      ),
    },
    {
      key: "creator",
      label: "Creator",
      width: "minmax(0, 1fr)",
      cell: (item) => (
        <Show
          when={item.creatorId > 0}
          fallback={<span class="truncate text-sm text-fg-muted">{item.creator || "Unavailable"}</span>}
        >
          <TextLink
            class="truncate"
            tone={item.creatorVerified ? "verified" : "name"}
            onClick={() => run(item.creatorIsGroup ? "group.open" : "user.open", String(item.creatorId))}
          >
            {item.creator || `Creator ${item.creatorId}`}
          </TextLink>
          <Show when={item.creatorVerified}>
            <VerifiedBadge />
          </Show>
        </Show>
      ),
    },
    {
      key: "id",
      label: "Asset ID",
      width: "minmax(0, 1fr)",
      mono: true,
      cell: (item) => (
        <TextLink class="truncate" onClick={() => run("entity.copy_id", itemTarget(item.id))}>
          {item.id}
        </TextLink>
      ),
    },
    {
      key: "actions",
      label: "",
      width: "2rem",
      align: "end",
      cell: (item) => (
        <Menu icon="more" label={`Actions for ${item.name ?? item.id}`} items={entityMenu(itemTarget(item.id))} />
      ),
    },
  ]
  return (
    <div class="flex min-w-0 flex-col gap-3" aria-busy={view.list.loading}>
      <div class="flex flex-wrap items-center justify-between gap-3">
        <div class="flex flex-col gap-1">
          <span class="text-sm text-fg-muted">Meshes, textures and other assets referenced by this model.</span>
          <Show when={view.list.items.length > 0}>
            <span class="text-xs text-fg-faint" aria-live="polite">
              {rows().length} of {view.list.items.length} assets
            </span>
          </Show>
        </div>
        <Show when={view.list.items.length > 0}>
          <TextField
            ref={search}
            class="w-64"
            icon="search"
            aria-label="Search dependencies"
            placeholder="Name, ID, creator or property"
            value={filter.query}
            onChange={(value) => void run("market.dependencies_search", value)}
          />
        </Show>
      </div>
      <Show when={view.note}>{(note) => <Alert title="Partial model scan">{note()}</Alert>}</Show>
      <Show when={view.list.error && view.list.items.length > 0}>
        {(error) => (
          <Alert variant="warning" title="Showing cached dependencies">
            {error()}
          </Alert>
        )}
      </Show>
      <ListState
        list={view.list}
        icon="inventory"
        empty="No referenced assets"
        description="This model has no external asset references. Built-in Roblox resources and embedded data are not listed."
      >
        <Show
          when={rows().length > 0}
          fallback={
            <EmptyState
              icon="search"
              title="No matching dependencies"
              description="Try another name, asset ID, creator or property."
            />
          }
        >
          <DataTable
            aria-label="Item dependencies"
            columns={columns}
            rows={rows()}
            rowKey={(item) => item.id}
            rowHeight={64}
          />
        </Show>
      </ListState>
    </div>
  )
}

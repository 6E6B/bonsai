import { createMemo, createSignal, Match, Switch } from "solid-js"
import { createStore } from "solid-js/store"
import {
  type BookmarkGroup,
  bookmarkFolders,
  bookmarkGroup,
  bookmarkIcons,
  bookmarkRows,
  groupKey,
  groupLabel,
} from "~/lib/bookmarks"
import { menuItem, run } from "~/lib/commands/registry"
import type { BookmarkView } from "~/lib/ipc/bindings"
import { entityMenu, gameMenu, itemMenu, userMenu } from "~/lib/menus"
import { visibleBookmarks } from "~/lib/store"
import { thumbUrl } from "~/lib/thumb"
import {
  Button,
  ContextMenu,
  EmptyState,
  ListItem,
  Menu,
  type MenuEntry,
  TextField,
  Thumbnail,
  VirtualList,
} from "~/lib/ui"

function normalMenu(b: BookmarkView): MenuEntry[] {
  const match = /^\/(games|users|catalog|bundles|passes|communities)\/(\d+)/.exec(b.path)
  if (!match) return []
  const id = Number(match[2])
  switch (match[1]) {
    case "games":
      return gameMenu(id)
    case "users":
      return userMenu(id)
    case "catalog":
      return itemMenu(id)
    case "bundles":
      return itemMenu(id, true)
    case "passes":
      return entityMenu(`pass:${id}`)
    case "communities":
      return entityMenu(`group:${id}`)
    default:
      return []
  }
}

function subtitle(b: BookmarkView): string | undefined {
  const tags = b.tags.map((t) => `#${t}`).join(" ")
  return [b.status, tags].filter(Boolean).join(" · ") || undefined
}

export function BookmarkList() {
  const [filter, setFilter] = createSignal("")
  const [collapsed, setCollapsed] = createStore<Partial<Record<string, boolean>>>({})
  const rows = createMemo(() => bookmarkRows(visibleBookmarks(), filter(), collapsed))
  const folders = createMemo(() => bookmarkFolders(visibleBookmarks()))
  let dragging: BookmarkView | undefined

  const folderMenu = (b: BookmarkView): MenuEntry => ({
    kind: "submenu",
    label: "Move to folder",
    icon: "folder",
    items: [
      ...folders()
        .filter((f) => f !== b.folder)
        .map((f) => menuItem("bookmark.folder", { arg: `${b.id} ${f}`, label: f, icon: "folder" })),
      menuItem("bookmark.folder", { arg: String(b.id), label: "New folder…", icon: "plus" }),
      ...(b.folder ? [menuItem("bookmark.folder", { arg: `${b.id} `, label: "Out of folder", icon: "x" })] : []),
    ],
  })
  const menu = (b: BookmarkView): MenuEntry[] => {
    const normal = normalMenu(b).filter(
      (entry) => !("onSelect" in entry && (entry.icon === "bookmark" || entry.icon === "bookmark-fill")),
    )
    return [
      menuItem("bookmark.rename", { arg: String(b.id), label: "Rename…", icon: "edit" }),
      folderMenu(b),
      menuItem("bookmark.tag", { arg: String(b.id), label: "Tags…", icon: "tag" }),
      menuItem("bookmark.remove", { arg: String(b.id), label: "Remove bookmark", icon: "bookmark-fill" }),
      menuItem("bookmark.move_up", { arg: String(b.id), label: "Move up", icon: "chevron-up" }),
      menuItem("bookmark.move_down", { arg: String(b.id), label: "Move down", icon: "chevron-down" }),
      ...(normal.length ? [{ kind: "separator" } as const, ...normal] : []),
    ]
  }
  const headerMenu = (g: BookmarkGroup): MenuEntry[] =>
    g.folder === null
      ? []
      : [
          menuItem("bookmark.folder_rename", { arg: g.folder, label: "Rename folder…", icon: "edit" }),
          menuItem("bookmark.folder_remove", { arg: g.folder, label: "Remove folder", icon: "trash" }),
        ]
  /** Where a dragged row may land: into a folder, or back out to its own kind. */
  const dropTarget = (g: BookmarkGroup) =>
    !!dragging && (g.folder !== null ? dragging.folder !== g.folder : !!dragging.folder && dragging.kind === g.kind)
  const sameGroup = (a: BookmarkView, b: BookmarkView) => groupKey(bookmarkGroup(a)) === groupKey(bookmarkGroup(b))
  const tools: MenuEntry[] = [
    menuItem("bookmark.export", { label: "Save to file…", icon: "download" }),
    menuItem("bookmark.import", { label: "Load from file…", icon: "upload" }),
  ]

  return (
    <div class="flex flex-col gap-2 pb-2">
      <div class="flex items-center gap-1">
        <TextField
          class="min-w-0 flex-1"
          aria-label="Filter bookmarks"
          placeholder="Filter bookmarks or #tag"
          icon="search"
          size="sm"
          value={filter()}
          onChange={setFilter}
        />
        <Menu icon="more" label="Bookmark file" size="sm" placement="bottom-end" items={tools} />
      </div>
      <VirtualList
        class="-mx-2"
        items={rows()}
        itemKey={(row) => (row.type === "header" ? groupKey(row.group) : row.bookmark.id)}
        estimateRowHeight={48}
        empty={
          <EmptyState
            icon="bookmark"
            title={filter() ? "No matching bookmarks" : "No bookmarks yet"}
            description={filter() ? undefined : "Press Ctrl+D on any page to bookmark it."}
          />
        }
      >
        {(row) => (
          <Switch>
            <Match when={row.type === "header" && row}>
              {(h) => {
                const key = () => groupKey(h().group)
                const open = () => !collapsed[key()] || !!filter().trim()
                return (
                  <ContextMenu items={headerMenu(h().group)}>
                    <Button
                      class="mt-2 w-full justify-start"
                      size="sm"
                      variant="ghost"
                      icon={open() ? "chevron-down" : "chevron-right"}
                      aria-expanded={open()}
                      onClick={() => setCollapsed(key(), !collapsed[key()])}
                      onDragOver={(e) => {
                        if (dropTarget(h().group)) e.preventDefault()
                      }}
                      onDrop={(e) => {
                        e.preventDefault()
                        const b = dragging
                        dragging = undefined
                        if (b && dropTarget(h().group)) void run("bookmark.folder", `${b.id} ${h().group.folder ?? ""}`)
                      }}
                    >
                      {groupLabel(h().group)} · {h().count}
                    </Button>
                  </ContextMenu>
                )
              }}
            </Match>
            <Match when={row.type === "bookmark" && row.bookmark}>
              {(b) => (
                <ContextMenu items={menu(b())}>
                  <fieldset
                    class="m-0 min-w-0 border-0 p-0"
                    aria-label={b().title}
                    draggable={!filter().trim()}
                    onDragStart={() => {
                      dragging = b()
                    }}
                    onDragEnd={() => {
                      dragging = undefined
                    }}
                    onDragOver={(e) => {
                      if (dragging && sameGroup(dragging, b())) e.preventDefault()
                    }}
                    onDrop={(e) => {
                      e.preventDefault()
                      if (dragging && sameGroup(dragging, b()) && dragging.id !== b().id)
                        void run("bookmark.move", `${dragging.id} ${b().id}`)
                      dragging = undefined
                    }}
                  >
                    <ListItem
                      leading={
                        <Thumbnail
                          class="w-8"
                          alt={b().title}
                          fallback={bookmarkIcons[b().kind]}
                          src={(() => {
                            const t = b().thumbnail
                            return t ? thumbUrl(t.kind, t.id) : undefined
                          })()}
                        />
                      }
                      title={b().title}
                      subtitle={subtitle(b())}
                      onClick={() => void run("bookmark.open", String(b().id))}
                    />
                  </fieldset>
                </ContextMenu>
              )}
            </Match>
          </Switch>
        )}
      </VirtualList>
    </div>
  )
}

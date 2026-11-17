import { createMemo, createSignal, Match, Switch } from "solid-js"
import { createStore } from "solid-js/store"
import { bookmarkIcons, bookmarkLabels, bookmarkRows } from "~/lib/bookmarks"
import { menuItem, run } from "~/lib/commands/registry"
import type { BookmarkKindView, BookmarkView } from "~/lib/ipc/bindings"
import { entityMenu, gameMenu, itemMenu, userMenu, withoutCommand } from "~/lib/menus"
import { visibleBookmarks } from "~/lib/store"
import { thumbUrl } from "~/lib/thumb"
import { Button, ContextMenu, EmptyState, ListItem, type MenuEntry, TextField, Thumbnail, VirtualList } from "~/lib/ui"

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
export function BookmarkList() {
  const [filter, setFilter] = createSignal("")
  const [collapsed, setCollapsed] = createStore<Partial<Record<BookmarkKindView, boolean>>>({})
  const rows = createMemo(() => bookmarkRows(visibleBookmarks(), filter(), collapsed))
  let dragging: BookmarkView | undefined
  const menu = (b: BookmarkView): MenuEntry[] => {
    // The thing's own menu first, then the bookmark's. Its bookmark toggle would repeat Remove.
    const normal = withoutCommand(normalMenu(b), "bookmark.toggle")
    const id = String(b.id)
    return [
      ...(normal.length ? [...normal, { kind: "separator" } as const] : []),
      menuItem("bookmark.rename", { arg: id, label: "Rename bookmark…", icon: "edit" }),
      {
        kind: "submenu",
        label: "Move",
        icon: "sort-asc",
        items: [
          menuItem("bookmark.move_up", { arg: id, label: "Up", icon: "chevron-up" }),
          menuItem("bookmark.move_down", { arg: id, label: "Down", icon: "chevron-down" }),
        ],
      },
      menuItem("bookmark.remove", { arg: id, label: "Remove bookmark", icon: "bookmark-fill" }),
    ]
  }
  return (
    <div class="flex flex-col gap-2 pb-2">
      <TextField
        aria-label="Filter bookmarks"
        placeholder="Filter bookmarks"
        icon="search"
        size="sm"
        value={filter()}
        onChange={setFilter}
      />
      <VirtualList
        class="-mx-2"
        items={rows()}
        itemKey={(row) => (row.type === "header" ? row.kind : row.bookmark.id)}
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
              {(h) => (
                <Button
                  class="mt-2 w-full justify-start"
                  size="sm"
                  variant="ghost"
                  icon={collapsed[h().kind] && !filter().trim() ? "chevron-right" : "chevron-down"}
                  aria-expanded={!collapsed[h().kind] || !!filter().trim()}
                  onClick={() => setCollapsed(h().kind, !collapsed[h().kind])}
                >
                  {bookmarkLabels[h().kind]} · {h().count}
                </Button>
              )}
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
                      if (dragging && dragging.kind === b().kind) e.preventDefault()
                    }}
                    onDrop={(e) => {
                      e.preventDefault()
                      if (dragging && dragging.kind === b().kind && dragging.id !== b().id)
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
                      subtitle={b().status || undefined}
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

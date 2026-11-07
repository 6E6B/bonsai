import { createMemo, createResource, createSignal, onCleanup, Show } from "solid-js"
import { createStore } from "solid-js/store"
import { useListCursor } from "~/lib/commands/hooks"
import { handle, run } from "~/lib/commands/registry"
import { ago } from "~/lib/format"
import { commands, type NotificationRow, type Paged } from "~/lib/ipc/bindings"
import { emptyList, ListState } from "~/lib/list-state"
import { shell, useLive } from "~/lib/store"
import { thumbUrl } from "~/lib/thumb"
import { Avatar, Button, Icon, ListItem, Page, SectionHeader, SegmentedControl, Select, VirtualList } from "~/lib/ui"
import { categoryArg, groupByDay, openCommand, shownNotifications } from "./filter"

const ms = (iso: string) => {
  const t = Date.parse(iso)
  return Number.isNaN(t) ? 0 : t
}

/** The notification stream. Enter opens what a notification is about: its page on the website,
 * or the person it's about. Clicking a row, or X, expands text that is cut off; click a row
 * that fits to open it. */
export default function Notifications() {
  const [cursor, setCursor] = createSignal(0)
  const res = useLive(
    () => shell.active,
    commands.notifications,
    (): Paged<NotificationRow> => ({ list: emptyList<NotificationRow>(), moreKey: "" }),
  )
  const [view, setView] = createSignal("all")
  onCleanup(
    handle("notifications.filter", (a) =>
      setView(a === "all" || a === "unread" ? a : view() === "all" ? "unread" : "all"),
    ),
  )
  const [categories] = createResource(() => commands.notificationCategories())
  const [category, setCategory] = createSignal("")
  onCleanup(
    handle("notifications.category", (a) =>
      setCategory(
        categoryArg(
          a,
          (categories() ?? []).map((c) => c.value),
        ),
      ),
    ),
  )
  const shown = createMemo(() => shownNotifications(res.list.items, { view: view(), category: category() }))
  const days = createMemo(() => groupByDay(shown(), (n) => ms(n.when), Date.now()))
  const openAt = (i: number) => {
    const n = shown()[i]
    const c = n && openCommand(n)
    if (c) void run(...c)
  }
  // Rows shown in full, by notification id.
  const [open, setOpen] = createStore<Record<string, boolean>>({})
  const truncated = (i: number) =>
    [
      ...document.querySelectorAll(
        `[data-cursor-list] [data-index="${i}"] :is([data-slot="title"], [data-slot="subtitle"])`,
      ),
    ].some((e) => e.scrollWidth > e.clientWidth)
  /** Expand or collapse a row. Returns false when there's nothing to expand. */
  const toggleAt = (i: number) => {
    const n = shown()[i]
    if (!n || (!open[n.id] && !truncated(i))) return false
    setOpen(n.id, !open[n.id])
    return true
  }
  useListCursor({
    count: () => shown().length,
    index: cursor,
    setIndex: setCursor,
    onOpen: openAt,
    onSelect: toggleAt,
  })

  return (
    <Page
      toolbar={
        <>
          <SegmentedControl
            aria-label="Show"
            size="sm"
            value={view()}
            onChange={setView}
            options={[
              { value: "all", label: "All" },
              { value: "unread", label: "Unread" },
            ]}
          />
          <Select
            class="w-44"
            aria-label="Category"
            size="sm"
            value={category() || "all"}
            onChange={(c) => run("notifications.category", c === "all" ? "" : c)}
            options={[{ value: "all", label: "All categories" }, ...(categories() ?? [])]}
          />
        </>
      }
      actions={
        <>
          <Button onClick={() => run("notifications.mark_read")} icon="check">
            Mark all read
          </Button>
        </>
      }
    >
      <ListState list={res.list} icon="bell" empty="You're all caught up" more={res.moreKey}>
        <div data-cursor-list class="flex flex-col">
          <VirtualList items={days().rows} activeIndex={days().positions[cursor()]} estimateRowHeight={56}>
            {(row) => (
              <Show
                when={row.item !== undefined && row}
                fallback={<SectionHeader class="px-2 pt-3" size="group" title={row.heading ?? ""} />}
              >
                {(r) => {
                  const n = r().item
                  const i = r().index
                  return (
                    <ListItem
                      data-index={i}
                      selected={i === cursor()}
                      unread={!n.read}
                      expanded={open[n.id]}
                      leading={
                        <Show when={n.userId} fallback={<Icon name="bell" class="m-1.5 text-fg-muted" />}>
                          {(id) => <Avatar name={n.title} src={thumbUrl("headshot", id())} />}
                        </Show>
                      }
                      title={n.title}
                      subtitle={n.body || undefined}
                      meta={ago(ms(n.when))}
                      onClick={() => {
                        setCursor(i)
                        if (!toggleAt(i)) openAt(i)
                      }}
                    />
                  )
                }}
              </Show>
            )}
          </VirtualList>
        </div>
      </ListState>
    </Page>
  )
}

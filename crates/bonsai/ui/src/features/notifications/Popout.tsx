import { createSignal, For, onCleanup, Show } from "solid-js"
import { handle, isRunning, run } from "~/lib/commands/registry"
import { ago } from "~/lib/format"
import { commands, type NotificationRow, type Paged } from "~/lib/ipc/bindings"
import { emptyList, ListState } from "~/lib/list-state"
import { shell, useLive } from "~/lib/store"
import { thumbUrl } from "~/lib/thumb"
import { Avatar, Button, Icon, IconButton, ListItem, Popover, Tooltip } from "~/lib/ui"
import { notificationTime, openCommand } from "./filter"

/** How many of the newest notifications the popout shows. */
const RECENT = 10

/** The bell in the title bar. It opens the newest notifications in a popout, with a way to the
 * full list. `notifications.peek` opens it from the keyboard. */
export function NotificationsButton() {
  const [open, setOpen] = createSignal(false)
  onCleanup(handle("notifications.peek", () => setOpen(!open())))
  const go = (...c: [string, string?]) => {
    setOpen(false)
    void run(...c)
  }
  return (
    <Popover
      open={open()}
      onOpenChange={setOpen}
      title="Notifications"
      actions={
        <Button
          size="sm"
          variant="ghost"
          icon="check"
          disabled={shell.unread.notifications === 0}
          loading={isRunning("notifications.mark_read")}
          onClick={() => run("notifications.mark_read")}
        >
          Mark all read
        </Button>
      }
      footer={
        <Button size="sm" variant="ghost" onClick={() => go("nav.notifications")}>
          See all notifications
        </Button>
      }
      trigger={(p) => (
        <Tooltip content={`Notifications (${shell.unread.notifications} unread)`} placement="bottom">
          <IconButton
            {...p}
            icon={shell.unread.notifications > 0 ? "bell-fill" : "bell"}
            label="Notifications"
            variant={open() ? "secondary" : "ghost"}
          />
        </Tooltip>
      )}
    >
      <Recent onOpen={go} />
    </Popover>
  )
}

/** The newest notifications. A row opens what it's about, or the full list when it isn't
 * about anything. Mounted only while the popout is open. */
function Recent(props: { onOpen: (...c: [string, string?]) => void }) {
  const res = useLive(
    () => shell.active,
    commands.notifications,
    (): Paged<NotificationRow> => ({ list: emptyList<NotificationRow>(), moreKey: "" }),
  )
  return (
    <ListState list={res.list} icon="bell" empty="You're all caught up">
      <div class="flex flex-col">
        <For each={res.list.items.slice(0, RECENT)}>
          {(n) => (
            <ListItem
              unread={!n.read}
              leading={
                <Show when={n.userId} fallback={<Icon name="bell" class="m-1.5 text-fg-muted" />}>
                  {(id) => <Avatar name={n.title} src={thumbUrl("headshot", id())} />}
                </Show>
              }
              title={n.title}
              subtitle={n.body || undefined}
              meta={ago(notificationTime(n.when))}
              onClick={() => props.onOpen(...(openCommand(n) ?? ["nav.notifications"]))}
            />
          )}
        </For>
      </div>
    </ListState>
  )
}

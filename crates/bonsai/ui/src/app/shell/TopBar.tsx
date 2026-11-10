import { createSignal, onCleanup, Show } from "solid-js"
import { NotificationsButton } from "~/features/notifications/Popout"
import { findBookmark } from "~/lib/bookmarks"
import { run, shortcutOf } from "~/lib/commands/registry"
import { wroteRecently } from "~/lib/plugins"
import { shell, visibleBookmarks } from "~/lib/store"
import { activeTab, tabs } from "~/lib/tabs"
import { IconButton, SearchButton, Tooltip } from "~/lib/ui"

/** The command bar trigger, centered in the title bar. */
export function TopSearch() {
  const keys = () => shortcutOf("palette.open")?.split("+")
  return (
    <SearchButton
      placeholder="Search, paste a link, or type > for commands"
      keys={keys()}
      onClick={() => run("palette.open")}
    />
  )
}

/** The buttons on the right of the title bar: the plugin kill switch while a plugin has
 * written recently, notifications, Discord and the drawer toggle. Space around them drags
 * the window. */
export function TopBar(props: { drawer: boolean }) {
  const [now, setNow] = createSignal(Date.now())
  const timer = setInterval(() => setNow(Date.now()), 10_000)
  onCleanup(() => clearInterval(timer))
  return (
    <div data-tauri-drag-region class="flex shrink-0 items-center gap-2 px-2">
      <Show when={wroteRecently(shell.pluginWrote, now())}>
        <Tooltip
          content="A plugin wrote recently. Click to stop all plugins."
          keys={[shortcutOf("plugins.stop_all") ?? ""]}
          placement="bottom"
        >
          <IconButton icon="plugin" label="Stop all plugins" variant="danger" onClick={() => run("plugins.stop_all")} />
        </Tooltip>
      </Show>
      <Tooltip content="Bookmarks" keys={[shortcutOf("view.bookmarks") ?? ""]} placement="bottom">
        <IconButton
          icon={findBookmark(visibleBookmarks(), activeTab(tabs)?.path ?? "") ? "bookmark-fill" : "bookmark"}
          label="Show bookmarks"
          onClick={() => run("view.bookmarks")}
        />
      </Tooltip>
      <NotificationsButton />
      <Tooltip content="Join the Bonsai Discord" placement="bottom">
        <IconButton icon="discord" label="Join the Bonsai Discord" onClick={() => run("help.discord")} />
      </Tooltip>
      <Tooltip content="Friends and chat" keys={[shortcutOf("view.drawer") ?? ""]} placement="bottom">
        <IconButton
          icon="friends"
          label="Toggle friends and chat"
          variant={props.drawer ? "secondary" : "ghost"}
          onClick={() => run("view.drawer")}
        />
      </Tooltip>
    </div>
  )
}

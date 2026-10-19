import { createSignal, onCleanup, Show } from "solid-js"
import { run, shortcutOf } from "~/lib/commands/registry"
import { wroteRecently } from "~/lib/plugins"
import { shell } from "~/lib/store"
import { IconButton, SearchButton, Tooltip } from "~/lib/ui"

/** Above the page: sidebar toggle, the command bar trigger, the plugin kill switch while a
 * plugin has written recently, notifications and the drawer toggle. */
export function TopBar(props: { drawer: boolean }) {
  const keys = () => shortcutOf("palette.open")?.split("+")
  const [now, setNow] = createSignal(Date.now())
  const timer = setInterval(() => setNow(Date.now()), 10_000)
  onCleanup(() => clearInterval(timer))
  return (
    <header class="flex h-11 shrink-0 items-center gap-2 border-b border-line bg-deep px-3">
      <Tooltip content="Toggle sidebar" keys={[shortcutOf("view.sidebar") ?? ""]} placement="bottom">
        <IconButton icon="sidebar" label="Toggle sidebar" onClick={() => run("view.sidebar")} />
      </Tooltip>
      <div class="flex flex-1 justify-center">
        <SearchButton
          class="max-w-xl"
          placeholder="Search, paste a link, or type > for commands"
          keys={keys()}
          onClick={() => run("palette.open")}
        />
      </div>
      <Show when={wroteRecently(shell.pluginWrote, now())}>
        <Tooltip
          content="A plugin wrote recently. Click to stop all plugins."
          keys={[shortcutOf("plugins.stop_all") ?? ""]}
          placement="bottom"
        >
          <IconButton icon="plugin" label="Stop all plugins" variant="danger" onClick={() => run("plugins.stop_all")} />
        </Tooltip>
      </Show>
      <Tooltip content={`Notifications (${shell.unread.notifications} unread)`} placement="bottom">
        <IconButton
          icon={shell.unread.notifications > 0 ? "bell-fill" : "bell"}
          label="Notifications"
          onClick={() => run("nav.notifications")}
        />
      </Tooltip>
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
    </header>
  )
}

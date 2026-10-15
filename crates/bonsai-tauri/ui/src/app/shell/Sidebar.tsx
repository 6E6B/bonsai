import { useLocation } from "@solidjs/router"
import { For } from "solid-js"
import { run, shortcutOf } from "~/lib/commands/registry"
import { shell } from "~/lib/store"
import { NavItem } from "~/lib/ui"
import { type Screen, screens } from "../screens"

const badge: Record<string, () => number> = {
  "nav.friends": () => shell.unread.friendRequests,
  "nav.trades": () => shell.unread.trades,
  "nav.messages": () => shell.unread.messages,
  "nav.chat": () => shell.unread.chat,
}

/** Screen navigation. Ctrl+B collapses it to icons. */
export function Sidebar(props: { compact: boolean }) {
  const location = useLocation()
  const isActive = (s: Screen) =>
    s.path === "/"
      ? location.pathname === "/"
      : location.pathname === s.path || location.pathname.startsWith(`${s.path}/`)

  const item = (s: Screen) => (
    <NavItem
      label={s.label}
      icon={s.icon}
      active={isActive(s)}
      count={badge[s.command]?.()}
      shortcut={shortcutOf(s.command)}
      compact={props.compact}
      onClick={() => run(s.command)}
    />
  )

  return (
    <nav
      aria-label="Screens"
      class="flex shrink-0 flex-col gap-0.5 overflow-y-auto border-r border-line bg-deep p-2"
      classList={{ "w-52": !props.compact, "w-12 items-center px-1.5": props.compact }}
    >
      <For each={screens.filter((s) => s.sidebar === "top" && (s.shown?.() ?? true))}>{item}</For>
      <div class="mt-auto pt-2">
        <For each={screens.filter((s) => s.sidebar === "bottom")}>{item}</For>
      </div>
    </nav>
  )
}

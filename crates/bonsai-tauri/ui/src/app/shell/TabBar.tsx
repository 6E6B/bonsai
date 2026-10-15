import { mapArray } from "solid-js"
import { menuItem, run } from "~/lib/commands/registry"
import { tabActions, tabLabel, tabs } from "~/lib/tabs"
import { IconButton, Menu, type MenuEntry, type StripTab, TabStrip } from "~/lib/ui"

/** The open tabs. Every action goes through a `tab.*` command; the Layout's handlers update
 * `tabs` and move the router to the active tab. */
export function TabBar() {
  // One object per tab for as long as the tab lives, so the strip never rebuilds a tab that
  // didn't change. Title and icon are getters, read only where they render.
  const strip = mapArray(
    () => tabs.tabs,
    (t): StripTab => ({
      get key() {
        return t.key
      },
      get title() {
        return tabLabel(t.path).title
      },
      get icon() {
        return tabLabel(t.path).icon
      },
      get pinned() {
        return t.pinned
      },
    }),
  )

  const menu = (key: string): MenuEntry[] => {
    const t = tabs.tabs.find((t) => t.key === key)
    const i = tabs.tabs.findIndex((t) => t.key === key)
    return [
      menuItem("tab.new", { icon: "plus" }),
      menuItem("tab.duplicate", { arg: key, icon: "copy" }),
      menuItem("tab.pin", { arg: key, icon: "pin", label: t?.pinned ? "Unpin tab" : "Pin tab" }),
      { kind: "separator" },
      menuItem("tab.close", { arg: key, icon: "x" }),
      menuItem("tab.close_others", { arg: key, disabled: tabs.tabs.every((o) => o.key === key || o.pinned) }),
      menuItem("tab.close_right", { arg: key, disabled: tabs.tabs.slice(i + 1).every((o) => o.pinned) }),
      { kind: "separator" },
      menuItem("tab.reopen", { icon: "refresh", disabled: tabs.closed.length === 0 }),
    ]
  }

  // Every open tab, for finding one when there are too many to read in the strip.
  const all = (): MenuEntry[] => [
    { kind: "heading", label: `${tabs.tabs.length} open` },
    ...tabs.tabs.map((t) => {
      const l = tabLabel(t.path)
      return menuItem("tab.switch", { arg: t.key, label: l.title, icon: t.pinned ? "pin" : l.icon })
    }),
  ]

  return (
    <TabStrip
      class="shrink-0 bg-deep px-2 pt-1"
      tabs={strip()}
      active={tabs.active}
      onSelect={(key) => void run("tab.switch", key)}
      onClose={(key) => void run("tab.close", key)}
      // Dragging is direct manipulation; Ctrl+Shift+PgUp/PgDn (`tab.move_*`) do it from the keyboard.
      onMove={(key, to) => tabActions.move(key, to)}
      menu={menu}
      actions={
        <>
          <IconButton icon="plus" label="New tab" size="sm" onClick={() => void run("tab.new")} />
          <Menu icon="chevron-down" label="All tabs" size="sm" placement="bottom-end" items={all()} />
        </>
      }
    />
  )
}

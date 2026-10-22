import { ContextMenu as KContextMenu } from "@kobalte/core/context-menu"
import { DropdownMenu as KDropdownMenu } from "@kobalte/core/dropdown-menu"
import { For, type JSX, Match, type ParentProps, Show, Switch, splitProps } from "solid-js"
import { Button, type ButtonProps } from "./button"
import { Icon, type IconName } from "./icon"
import { Kbd } from "./kbd"
import { Tooltip } from "./tooltip"

/** One row of a menu. Build command rows with `menuItem()` from `~/lib/commands/registry`, so
 * the label and shortcut come from the registry. */
export type MenuEntry =
  | {
      kind?: "item"
      label: string
      onSelect: () => void
      icon?: IconName
      /** Shortcut label, like `Ctrl+W`. */
      shortcut?: string
      danger?: boolean
      disabled?: boolean
    }
  | { kind: "submenu"; label: string; icon?: IconName; items: MenuEntry[]; disabled?: boolean }
  | { kind: "heading"; label: string }
  | { kind: "separator" }

// Dropdown and context menus have the same parts; pick one set and render the entries with it.
type Parts = typeof KDropdownMenu | typeof KContextMenu

/** The rows of a menu, for components that build their own Kobalte menu around them. */
export function Entries(props: { parts: Parts; items: MenuEntry[] }) {
  const P = props.parts
  return (
    <For each={props.items}>
      {(e) => (
        <Switch>
          <Match when={e.kind === "separator"}>
            <P.Separator data-slot="separator" />
          </Match>
          <Match when={e.kind === "heading" && e}>{(h) => <div data-slot="heading">{h().label}</div>}</Match>
          <Match when={e.kind === "submenu" && e}>
            {(s) => (
              <P.Sub overlap gutter={4}>
                <P.SubTrigger data-slot="item" disabled={s().disabled}>
                  <ItemBody icon={s().icon} label={s().label} />
                  <Icon data-slot="chevron" name="chevron-right" size={12} />
                </P.SubTrigger>
                <P.Portal>
                  <P.SubContent data-component="menu">
                    <Entries parts={props.parts} items={s().items} />
                  </P.SubContent>
                </P.Portal>
              </P.Sub>
            )}
          </Match>
          <Match when={(e.kind === undefined || e.kind === "item") && e}>
            {(i) => {
              const item = i() as Extract<MenuEntry, { onSelect: () => void }>
              return (
                <P.Item
                  data-slot="item"
                  data-danger={item.danger ? "" : undefined}
                  disabled={item.disabled}
                  onSelect={item.onSelect}
                >
                  <ItemBody icon={item.icon} label={item.label} />
                  <Show when={item.shortcut}>{(s) => <Kbd keys={[s()]} />}</Show>
                </P.Item>
              )
            }}
          </Match>
        </Switch>
      )}
    </For>
  )
}

function ItemBody(props: { icon?: IconName; label: string }) {
  return (
    <>
      <span data-slot="icon">
        <Show when={props.icon}>{(n) => <Icon name={n()} size={14} />}</Show>
      </span>
      <span data-slot="label">{props.label}</span>
    </>
  )
}

export interface MenuProps extends Omit<ButtonProps, "onClick"> {
  items: MenuEntry[]
  /** Accessible name. Needed when the trigger has no text. */
  label?: string
  placement?: "bottom-start" | "bottom-end" | "top-start" | "right-start"
}

/** A button that opens a menu. Takes the same props as `Button`; with no children it is a
 * square icon button (pass `icon` and `label`) with `label` as its tooltip. */
export function Menu(props: MenuProps) {
  const [local, rest] = splitProps(props, ["items", "label", "placement", "children", "variant"])
  const trigger = () => (
    <KDropdownMenu.Trigger
      as={Button}
      variant={local.variant ?? (local.children ? "secondary" : "ghost")}
      aria-label={local.label}
      data-icon-only={local.children ? undefined : ""}
      {...rest}
    >
      {local.children}
    </KDropdownMenu.Trigger>
  )
  return (
    <KDropdownMenu placement={local.placement ?? "bottom-start"} gutter={4}>
      <Show when={!local.children && local.label} fallback={trigger()}>
        {(label) => <Tooltip content={label()}>{trigger()}</Tooltip>}
      </Show>
      <KDropdownMenu.Portal>
        <KDropdownMenu.Content data-component="menu">
          <Entries parts={KDropdownMenu} items={local.items} />
        </KDropdownMenu.Content>
      </KDropdownMenu.Portal>
    </KDropdownMenu>
  )
}

export interface ContextMenuProps extends ParentProps {
  items: MenuEntry[]
  class?: string
  /** Called when the menu opens or closes, e.g. to load what its entries show. */
  onOpenChange?: (open: boolean) => void
}

/** Opens a menu on right-click (or the Menu key) anywhere inside `children`. Everything in it
 * must also be reachable another way: the command bar or a visible button. */
export function ContextMenu(props: ContextMenuProps): JSX.Element {
  return (
    <KContextMenu onOpenChange={props.onOpenChange}>
      <KContextMenu.Trigger as="div" data-slot="context-trigger" class={props.class}>
        {props.children}
      </KContextMenu.Trigger>
      <KContextMenu.Portal>
        <KContextMenu.Content data-component="menu">
          <Entries parts={KContextMenu} items={props.items} />
        </KContextMenu.Content>
      </KContextMenu.Portal>
    </KContextMenu>
  )
}

import { isTauri } from "@tauri-apps/api/core"
import { createSignal } from "solid-js"
import { type CommandInfo, commands as ipc } from "~/lib/ipc/bindings"
import { type ConfirmOptions, type IconName, type MenuEntry, showToast } from "~/lib/ui"
import { confirmations } from "./confirm"
import { isPlain, match, toPress } from "./keys"

/**
 * The UI side of rbx-core's command registry. Every button, menu item and shortcut calls
 * `run` with a command id from the registry. Commands that only change UI state register a
 * local handler with `handle`; the rest go to the backend.
 */

/** `asAccount` is set when a menu runs the command for a given account. */
type Handler = (arg: string | null, asAccount: number | null) => void

const local = new Map<string, Handler>()
let pageContext: string | null = null

/** Sets what "this" means for commands run from now on, like `game:<placeId>` on an
 * experience page. `useCommandContext` sets and clears it with the screen. */
export function setCommandContext(context: string | null) {
  pageContext = context
}
/** What "this" means right now, like `user:1`. */
export const commandContext = () => pageContext

const [list, setList] = createSignal<CommandInfo[]>([])

export const commandList = list

export async function loadCommands() {
  // `bun run dev` in a plain browser has no backend, so no registry.
  if (!isTauri()) return
  setList(await ipc.commands())
}

/** Registers a UI-side handler for a command id. Returns a function that removes it. */
export function handle(id: string, fn: Handler) {
  local.set(id, fn)
  return () => local.delete(id)
}

/** The registry entry for `id`, once loaded. */
export function command(id: string): CommandInfo | undefined {
  return list().find((c) => c.id === id)
}

/** The first shortcut label of `id`, like `Ctrl+W` or `g h`. */
export function shortcutOf(id: string): string | undefined {
  return command(id)?.shortcuts[0]?.label
}

export interface ItemOptions {
  arg?: string | null
  /** Act as this account instead of the active one. */
  asAccount?: number | null
  /** Overrides the registry title, for context like "Join as Alt 2". */
  label?: string
  icon?: IconName
  danger?: boolean
  disabled?: boolean
}

/** A menu row that runs a registry command, titled and labelled from the registry. */
export function menuItem(id: string, opts: ItemOptions = {}): MenuEntry {
  return {
    label: opts.label ?? command(id)?.title ?? id,
    shortcut: shortcutOf(id),
    icon: opts.icon,
    danger: opts.danger,
    disabled: opts.disabled,
    onSelect: () => void run(id, opts.arg ?? null, opts.asAccount ?? null),
  }
}

let confirmer: ((opts: ConfirmOptions) => Promise<boolean>) | null = null

/** Sets how commands in `confirmations` ask first. The layout passes the app's dialog. */
export function setConfirmer(fn: typeof confirmer) {
  confirmer = fn
}

/** Runs a command. False when it was cancelled or the backend refused it (the error is shown). */
export async function run(id: string, arg: string | null = null, asAccount: number | null = null) {
  const ask = confirmations[id]?.(arg)
  if (ask && confirmer && !(await confirmer(ask))) return false
  const h = local.get(id)
  if (h) return h(arg, asAccount)
  return runBackend(id, arg, asAccount)
}

/** Sends a command to the backend, skipping any UI handler. A handler that also needs the
 * backend (like opening a screen after `trade.new`) calls this. True when it ran. */
export async function runBackend(id: string, arg: string | null = null, asAccount: number | null = null) {
  const r = await ipc.runCommand({ id, arg, asAccount, context: pageContext })
  if (r.status === "error") showToast(r.error)
  return r.status === "ok"
}

function isTyping(target: EventTarget | null) {
  if (!(target instanceof HTMLElement)) return false
  return target.isContentEditable || ["INPUT", "TEXTAREA", "SELECT"].includes(target.tagName)
}

/** Returns a keydown listener that runs commands by their shortcuts. */
export function shortcutListener() {
  let pending: string | undefined
  let timer: ReturnType<typeof setTimeout> | undefined
  return (e: KeyboardEvent) => {
    const press = toPress(e)
    if (!press || (isPlain(press) && isTyping(e.target))) return
    let next: string | undefined
    for (const c of list()) {
      const m = match(press, c.shortcuts, pending)
      if (m.kind === "run") {
        e.preventDefault()
        pending = undefined
        void run(c.id)
        return
      }
      if (m.kind === "pending") next = m.first
    }
    pending = next
    clearTimeout(timer)
    if (next) {
      e.preventDefault()
      timer = setTimeout(() => {
        pending = undefined
      }, 1000)
    }
  }
}

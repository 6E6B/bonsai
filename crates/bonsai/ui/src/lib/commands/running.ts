import { createSignal } from "solid-js"
import type { Running } from "~/lib/ipc/bindings"
import { shell } from "~/lib/store"

/**
 * Which commands are still working, for button spinners. bonsai-core lists the commands whose
 * requests haven't finished (`shell.running`). Until the shell shows a command it just ran,
 * `runBackend` keeps it in `local`, so the spinner starts on the click and doesn't flicker.
 */

const [local, setLocal] = createSignal<Running[]>([])

/** True when `list` has command `id` run with `arg` as `account`. */
export function listed(list: readonly Running[], id: string, arg: string | null, account: number | null) {
  return list.some((r) => r.id === id && (r.arg ?? null) === arg && (r.account ?? null) === account)
}

/** Marks `id` as running here until the returned function is called. */
export function begin(id: string, arg: string | null, asAccount: number | null) {
  const entry: Running = { id, arg, account: asAccount ?? shell.active }
  setLocal((l) => [...l, entry])
  return () => setLocal((l) => l.filter((e) => e !== entry))
}

/** True while command `id`, run with `arg` as `asAccount` (default the active account), is
 * still working. Pass the same `arg` the button passes to `run`. */
export function isRunning(id: string, arg: string | null = null, asAccount: number | null = null) {
  const account = asAccount ?? shell.active
  return listed(shell.running, id, arg, account) || listed(local(), id, arg, account)
}

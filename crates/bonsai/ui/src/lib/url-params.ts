import { useSearchParams } from "@solidjs/router"
import { createEffect, createSignal, onCleanup } from "solid-js"

/** A search param as one string; empty when it is missing. */
export const paramOf = (v: string | string[] | undefined) => (typeof v === "string" ? v : (v?.[0] ?? ""))

/** The URL for `path` with `changes` applied. Keeps the other params when already on `path`,
 * since that is where a screen's state lives; elsewhere it starts clean. Empty values drop out. */
export function withParams(
  here: { pathname: string; search: string },
  path: string,
  changes: Record<string, string | null>,
): string {
  const next = new URLSearchParams(here.pathname === path ? here.search : "")
  for (const [k, v] of Object.entries(changes)) {
    if (v) next.set(k, v)
    else next.delete(k)
  }
  const s = next.toString()
  return s ? `${path}?${s}` : path
}

/** Calls `write` once typing stops. `pending` is true until it has. */
export function debouncer(write: (value: string) => void, delay: number) {
  let timer: ReturnType<typeof setTimeout> | undefined
  const cancel = () => {
    clearTimeout(timer)
    timer = undefined
  }
  return {
    pending: () => timer !== undefined,
    cancel,
    schedule(value: string) {
      cancel()
      timer = setTimeout(() => {
        timer = undefined
        write(value)
      }, delay)
    },
  }
}

/** A text field backed by a search param. The field updates at once; the URL, and anything
 * fetched from it, follows when typing stops. */
export function useParamDraft(key: string, delay = 350) {
  const [params, setParams] = useSearchParams()
  const [draft, setDraft] = createSignal(paramOf(params[key]))
  const write = debouncer((v) => setParams({ [key]: v || null }, { replace: true }), delay)
  onCleanup(write.cancel)
  // Follow the URL unless a write is on its way, which would undo what is being typed.
  createEffect(() => {
    const v = paramOf(params[key])
    if (!write.pending()) setDraft(v)
  })
  return {
    value: draft,
    set(v: string) {
      setDraft(v)
      write.schedule(v)
    },
    /** Empties the field and drops any write on its way. The caller clears the param. */
    reset() {
      write.cancel()
      setDraft("")
    },
  }
}

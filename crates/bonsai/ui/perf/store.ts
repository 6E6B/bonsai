// Runs the real screens with synthetic store snapshots, only in --mode performance.
import { createEffect, on, onCleanup } from "solid-js"
import { createStore, reconcile } from "solid-js/store"
import { browserFriends, browserHome, browserShell } from "../src/lib/sample"
import { calls } from "./bindings"
import { games, people } from "./fixtures"

export const [shell] = createStore({ ...browserShell, wiki: false })
export const [friends] = createStore({
  ...browserFriends,
  friends: people.map((p) => ({ ...p, status: "Online", placeId: null, nickname: null, universeId: null })),
})
const [home] = createStore({
  ...browserHome,
  favorites: games,
  sorts: [{ id: "perf", title: "Recommended", subtitle: "", games: games.slice(0, 30) }],
})
const refreshers = new Set<() => Promise<void>>()
let pending = 0
let revision = 0

// The runner waits for explicit store completion and two animation frames, never networkidle.
Object.assign(window, {
  __bonsaiPerf: {
    get pending() {
      return pending
    },
    get revision() {
      return revision
    },
    calls,
    async refresh() {
      await Promise.all([...refreshers].map((refresh) => refresh()))
      revision++
    },
  },
})

export const useHome = () => home
export async function startSync() {}
export function useLive<K, T extends object>(
  source: () => K,
  fetch: (key: K) => Promise<T>,
  initial: () => T,
  opts: { keep?: boolean } = {},
) {
  const [state, setState] = createStore(initial())
  let generation = 0
  let disposed = false
  const refresh = async () => {
    const current = ++generation
    pending++
    try {
      const value = await fetch(source())
      if (!disposed && current === generation) setState(reconcile(value, { key: "id" }))
    } catch (error) {
      // Unsupported fixtures fail the benchmark, never silently time an empty loading screen.
      console.error(error)
    } finally {
      pending--
    }
  }
  createEffect(
    on(source, (_, previous) => {
      if (previous !== undefined && !opts.keep) setState(reconcile(initial()))
      void refresh()
    }),
  )
  refreshers.add(refresh)
  onCleanup(() => {
    disposed = true
    refreshers.delete(refresh)
  })
  return state
}

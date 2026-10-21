// Runs the real screens with synthetic store snapshots, only in --mode performance.
import { createEffect, on, onCleanup } from "solid-js"
import { createStore, reconcile } from "solid-js/store"
import { run } from "../src/lib/commands/registry"
import { browserFriends, browserHome, browserShell } from "../src/lib/sample"
import { calls } from "./bindings"
import { appendChat, games, people, prependChat } from "./fixtures"

export const [shell] = createStore({ ...browserShell, wiki: false })
const playing = new URLSearchParams(location.search).get("playing")
export const [friends] = createStore({
  ...browserFriends,
  friends: people.map((person, index) => {
    const game = playing === "single" ? 1000 : 1000 + Math.floor(index / 5)
    return {
      ...person,
      presence: playing ? ("game" as const) : person.presence,
      status: playing ? `Fixture Game ${game}` : "Online",
      placeId: playing ? game : null,
      nickname: null,
      universeId: playing ? game : null,
    }
  }),
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
    moveCursor: (direction: number) => run(direction > 0 ? "list.down" : "list.up"),
    appendChat,
    prependChat,
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

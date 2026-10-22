import { isTauri } from "@tauri-apps/api/core"
import { createEffect, on, onCleanup } from "solid-js"
import { createStore, reconcile } from "solid-js/store"
import { commands, events, type Friends, type Home, type PlaylistView, type Shell } from "~/lib/ipc/bindings"
import { browserFriends, browserHome, browserShell } from "~/lib/sample"

/**
 * Mirrors of bonsai-core state. bonsai-core is the source of truth; each mirror refetches when the
 * backend sends `storeChanged` (at most once per frame) and merges with `reconcile`, so only
 * the parts that changed re-render.
 */

/** A store that refetches on every `storeChanged` once started. A fetch that lands while
 * another is in flight runs once more afterwards instead of overlapping. */
function mirror<T extends object>(initial: T, fetch: () => Promise<T>, key = "id") {
  const [state, setState] = createStore<T>(initial)
  let inflight = false
  let again = false
  let started = false

  async function refresh() {
    if (inflight) {
      again = true
      return
    }
    inflight = true
    try {
      do {
        again = false
        setState(reconcile(await fetch(), { key }))
      } while (again)
    } finally {
      inflight = false
    }
  }

  return {
    state,
    refresh,
    /** Starts refreshing on store changes. Safe to call more than once. */
    start() {
      if (started) return
      started = true
      void refresh()
    },
    started: () => started,
    set: (v: T) => setState(reconcile(v, { key })),
  }
}

const shellMirror = mirror<Shell>(
  {
    accounts: [],
    active: null,
    offline: false,
    jobs: 0,
    toasts: [],
    keychainWarning: null,
    robux: null,
    unread: { messages: 0, friendRequests: 0, trades: 0, notifications: 0, chat: 0 },
    login: null,
    challenge: null,
    themeImport: null,
    pluginInstall: null,
    pluginActions: null,
    pluginOpen: null,
    pluginRev: 0,
    pluginAutomation: null,
    pluginBudget: null,
    pluginWrote: null,
    pluginView: null,
    security: null,
    approve: null,
    itemValues: false,
    wiki: false,
    avatar3d: false,
    uiScale: 1,
    look: {
      density: "comfortable",
      formatLocale: "",
      motion: "system",
      timeFormat: "auto",
      relativeDates: true,
      statusBar: true,
      startScreen: "home",
      sidebarMode: "auto",
      drawer: false,
      homeFriends: true,
      homeContinue: true,
      homeFavorites: true,
      homeRecommended: true,
      accountAccent: false,
    },
    sidebar: { order: [], hidden: ["messages"] },
  },
  () => commands.shell(),
)

const friendsMirror = mirror<Friends>({ loading: true, error: null, friends: [] }, () => commands.friends(""))

const homeMirror = mirror<Home>({ loading: true, error: null, recent: [], favorites: [], sorts: [] }, () =>
  commands.home(),
)

const playlistsMirror = mirror<{ lists: PlaylistView[] }>({ lists: [] }, async () => ({
  lists: await commands.playlists(),
}))

const mirrors = new Set<{ started: () => boolean; refresh: () => Promise<void> }>([
  shellMirror,
  friendsMirror,
  homeMirror,
  playlistsMirror,
])

export const shell = shellMirror.state

/** The active account's friends, grouped by presence then name. The drawer and Friends page
 * read this. */
export const friends = friendsMirror.state

/** The active account's game playlists. Local data, so it loads at startup for menus and the
 * command bar. */
export const playlists = playlistsMirror.state

/** Home for the active account. Only loads once a screen asks for it, since asking starts the
 * network fetch of the sorts. */
export function useHome() {
  if (isTauri()) homeMirror.start()
  return homeMirror.state
}

/**
 * One screen's data, for the key `source` returns (like a route id). Refetches on store changes
 * while the caller is mounted, and at once when the key changes. Outside the app it stays at
 * `initial`. A new key starts from `initial`, unless `keep` is set (search results, where the
 * old matches should stay until the new ones arrive).
 */
export function useLive<K, T extends object>(
  source: () => K,
  fetch: (key: K) => Promise<T>,
  initial: () => T,
  opts: { keep?: boolean } = {},
) {
  const m = mirror<T>(initial(), () => fetch(source()))
  if (!isTauri()) return m.state
  createEffect(
    on(source, (_, prev) => {
      if (prev !== undefined && !opts.keep) m.set(initial())
      if (m.started()) void m.refresh()
      else m.start()
    }),
  )
  mirrors.add(m)
  onCleanup(() => mirrors.delete(m))
  return m.state
}

/** Loads the first snapshot and keeps it current. Call once at startup. */
export async function startSync() {
  if (!isTauri()) {
    shellMirror.set(browserShell)
    friendsMirror.set(browserFriends)
    homeMirror.set(browserHome)
    return
  }
  await events.storeChanged.listen(() => {
    for (const m of mirrors) if (m.started()) void m.refresh()
  })
  shellMirror.start()
  friendsMirror.start()
  playlistsMirror.start()
}

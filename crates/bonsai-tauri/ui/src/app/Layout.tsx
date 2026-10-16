import { makeEventListener } from "@solid-primitives/event-listener"
import { useLocation, useNavigate } from "@solidjs/router"
import { isTauri } from "@tauri-apps/api/core"
import { createEffect, createSignal, ErrorBoundary, on, onCleanup, onMount, type ParentProps, Show } from "solid-js"
import { NewChat } from "~/features/chat/NewChat"
import { CommandPalette } from "~/features/palette/CommandPalette"
import { ShortcutsDialog } from "~/features/shortcuts/ShortcutsDialog"
import { registerThemeCommands } from "~/features/themes/commands"
import {
  commandContext,
  handle,
  loadCommands,
  runBackend,
  setConfirmer,
  shortcutListener,
} from "~/lib/commands/registry"
import { setClockFormat, setFormatLocale, setRelativeDates } from "~/lib/format"
import { commands } from "~/lib/ipc/bindings"
import { ItemPreview, parseItemTarget } from "~/lib/item-preview"
import { describePath, nudgeScreen, screens } from "~/lib/screens"
import { shell, startSync } from "~/lib/store"
import { activeTab, savedTabs, setTabDescriber, tabActions, tabs } from "~/lib/tabs"
import { showToast, useConfirm, useDialog, usePrompt } from "~/lib/ui"
import { stepZoom } from "~/lib/zoom"
import { useAppearance } from "./appearance"
import { AccountRail } from "./shell/AccountRail"
import { AddAccount, type AddMode } from "./shell/AddAccount"
import { ApproveCode } from "./shell/ApproveCode"
import { Backdrop } from "./shell/Backdrop"
import { ChallengeDialog } from "./shell/ChallengeDialog"
import { Drawer } from "./shell/Drawer"
import { PinForm } from "./shell/PinForm"
import { ShellAlerts } from "./shell/ShellAlerts"
import { Sidebar } from "./shell/Sidebar"
import { StatusBar } from "./shell/StatusBar"
import { TabBar } from "./shell/TabBar"
import { TitleBar } from "./shell/TitleBar"
import { TopBar } from "./shell/TopBar"

/**
 * The window:
 *
 *   title bar (app name, window controls)
 *   rail | top bar (sidebar toggle, command bar, notifications, drawer toggle)
 *        | sidebar | tab strip       | drawer
 *        |         | page            |
 *   status bar
 *
 * It owns the commands that only change UI state: navigation, tabs, panels, zoom, theme.
 */
/** Below this window width (CSS px at zoom 1) the sidebar starts collapsed, until the user
 * toggles it. The drawer (friends) starts hidden at any width. */
const NARROW = 1280

export function Layout(props: ParentProps) {
  setTabDescriber(describePath)
  const navigate = useNavigate()
  const location = useLocation()
  const dialog = useDialog()
  setConfirmer(useConfirm())
  const prompt = usePrompt()
  const [width, setWidth] = createSignal(window.innerWidth)
  makeEventListener(window, "resize", () => setWidth(window.innerWidth))
  const zoom = () => shell.uiScale || 1
  /** The item Ctrl+click or `market.preview` shows beside the page. */
  const [preview, setPreview] = createSignal<{ id: number; bundle: boolean } | null>(null)
  const compact = () => {
    const mode = shell.look.sidebarMode
    return mode === "auto" ? width() / zoom() < NARROW : mode === "collapsed"
  }
  const drawer = () => shell.look.drawer

  const here = () => location.pathname + location.search
  // True while a Ctrl+click is being handled, so commands that open a place open it in a
  // background tab instead. Cleared after the click finishes dispatching.
  let background = false
  makeEventListener(
    window,
    "click",
    (e) => {
      background = e.ctrlKey || e.metaKey
      if (background) setTimeout(() => (background = false))
    },
    { capture: true },
  )
  const visitPath = (path: string) => {
    if (background) tabActions.open(path, { background: true })
    else navigate(path)
  }
  const go = (path: string) => () => visitPath(path)
  const open = (prefix: string) => (arg: string | null) => {
    if (arg) visitPath(`${prefix}/${arg}`)
  }
  /** Zoom is a setting (`ui_scale`), so it is saved and the command bar can set it. */
  const setZoom = (z: number) => runBackend("settings.set", `ui_scale ${Math.round(z * 100)}`)
  const zoomBy = (by: 1 | -1) => setZoom(stepZoom(zoom(), by))
  const syncTab = () => {
    const t = activeTab(tabs)
    if (t && t.path !== here()) navigate(t.path)
  }
  /** A tab command: runs `f` on the tab in `arg` (the active one when null), then shows
   * whichever tab is active. */
  const onTab = (f: (key?: string) => void) => (arg: string | null) => {
    f(arg ?? undefined)
    syncTab()
  }

  /** `user.nickname` and `user.note`: "<user> <text>" saves at once. Otherwise ask, starting
   * from the user on the page and what is saved now. */
  const editAbout = (which: "nickname" | "note") => async (arg: string | null) => {
    const id = `user.${which}`
    const raw = arg?.trim() ?? ""
    if (/\s/.test(raw)) return void runBackend(id, raw)
    let who = raw || /^user:(\d+)$/.exec(commandContext() ?? "")?.[1] || ""
    if (!who) who = (await prompt({ title: "Whose?", label: "Username or ID", submit: "Next" })) ?? ""
    if (!who) return
    const n = Number(who)
    const now = Number.isSafeInteger(n) && n > 0 ? await commands.userNote(n) : null
    const nickname = which === "nickname"
    const text = await prompt({
      title: nickname ? "Nickname" : "Note",
      label: nickname ? "What you call them. Only you see it." : "A note only you see",
      initial: now?.[which] ?? "",
      submit: "Save",
      allowEmpty: true,
      maxLength: nickname ? 32 : 500,
    })
    if (text !== null) void runBackend(id, `${who} ${text}`)
  }

  const addAccount = (mode: AddMode, title: string) => () =>
    dialog.show(() => <AddAccount mode={mode} onDone={() => dialog.close()} />, { title, size: "sm" })

  const off = [
    handle("account.add", addAccount("choose", "Add account")),
    handle("account.replace_cookie", addAccount("paste", "Paste a new cookie")),
    // The dialog starts the backend side (code polling, the sign-in window) and stops it on close.
    handle("account.quick_sign_in", addAccount("quick", "Quick sign-in")),
    handle("account.sign_in_browser", addAccount("browser", "Sign in with Roblox")),
    handle("account.approve_code", (arg, asAccount) => {
      // The attempt to cancel if the dialog closes before it is approved.
      let seq: number | undefined
      dialog.show(
        () => (
          <ApproveCode
            account={asAccount ?? shell.active}
            initial={arg ?? undefined}
            onSeq={(s) => (seq = s)}
            onDone={() => dialog.close()}
          />
        ),
        {
          title: "Sign in another device",
          size: "sm",
          onClose: () => seq !== undefined && void commands.approveClose(seq),
        },
      )
    }),
    handle("account.unlock_pin", () =>
      dialog.show(() => <PinForm onDone={() => dialog.close()} />, { title: "Unlock account PIN", size: "sm" }),
    ),
    ...screens.filter((s) => s.command !== "nav.settings").map((s) => handle(s.command, go(s.path))),
    handle("nav.settings", (section) => visitPath(section ? `/settings/${section}` : "/settings")),
    // The order is saved in rbx-core; the groups that bound a move live in `screens`.
    ...([-1, 1] as const).map((by) =>
      handle(by < 0 ? "view.sidebar_up" : "view.sidebar_down", (arg) => {
        const order = arg ? nudgeScreen(screens, shell.sidebar, arg.trim(), by) : null
        if (order) void runBackend("view.sidebar_order", order.join(" "))
      }),
    ),
    handle("nav.profile", () => shell.active !== null && visitPath(`/users/${shell.active}`)),
    handle("user.open", open("/users")),
    ...(["friends", "followers", "following"] as const).map((list) =>
      handle(`user.${list}`, (who) => {
        if (who?.trim()) visitPath(`/users/${who.trim()}?tab=connections&list=${list}`)
      }),
    ),
    handle("game.open", open("/games")),
    handle("market.open", (arg) => {
      const t = arg?.trim() ?? ""
      if (t.startsWith("bundle:")) visitPath(`/bundles/${t.slice(7)}`)
      else if (t.startsWith("pass:")) visitPath(`/passes/${t.slice(5)}`)
      else if (t) visitPath(`/catalog/${t.replace(/^item:/, "")}`)
    }),
    handle("market.preview", (arg) => {
      const t = parseItemTarget(arg)
      if (t) setPreview(t)
      else showToast("Preview takes an item ID, item:<id> or bundle:<id>")
    }),
    handle("market.search", (q) =>
      navigate(q?.trim() ? `/marketplace?q=${encodeURIComponent(q.trim())}` : "/marketplace"),
    ),
    handle("user.nickname", editAbout("nickname")),
    handle("user.note", editAbout("note")),
    handle("search.open", (q) => {
      const t = q?.trim() ?? ""
      if (t) void runBackend("search.open", t)
      visitPath(t ? `/search?q=${encodeURIComponent(t)}` : "/search")
    }),
    handle("user.message", async (who) => {
      if (await runBackend("user.message", who)) navigate("/chat")
    }),
    handle("message.reply", async (id) => {
      if (await runBackend("message.reply", id)) navigate("/chat")
    }),
    handle("chat.new_group", () =>
      dialog.show(
        () => (
          <NewChat
            onDone={(started) => {
              dialog.close()
              if (started) navigate("/chat")
            }}
          />
        ),
        { title: "New chat", size: "md" },
      ),
    ),
    handle("trade.new", async (who) => {
      if (await runBackend("trade.new", who)) navigate("/trades")
    }),
    handle("trade.counter", async (id) => {
      if (await runBackend("trade.counter", id)) navigate("/trades")
    }),
    handle("inventory.view", (who) =>
      navigate(who?.trim() ? `/inventory/${encodeURIComponent(who.trim())}` : "/inventory"),
    ),
    handle("group.open", open("/communities")),
    handle("group.search", (q) =>
      navigate(q?.trim() ? `/communities?q=${encodeURIComponent(q.trim())}` : "/communities"),
    ),
    handle("palette.open", () =>
      dialog.show(() => <CommandPalette />, { position: "top", label: "Command bar", size: "lg" }),
    ),
    handle("help.shortcuts", () => dialog.show(() => <ShortcutsDialog />, { title: "Keyboard shortcuts", size: "lg" })),
    // Saved, so the window opens the same way next time.
    handle("view.sidebar", () => runBackend("settings.set", `sidebar_mode ${compact() ? "expanded" : "collapsed"}`)),
    handle("view.drawer", () => runBackend("settings.set", "drawer")),
    handle("view.zoom_in", () => zoomBy(1)),
    handle("view.zoom_out", () => zoomBy(-1)),
    handle("view.zoom_reset", () => setZoom(1)),
    handle(
      "tab.new",
      onTab(() => tabActions.open("/")),
    ),
    handle(
      "tab.switch",
      onTab((key) => key && tabActions.activate(key)),
    ),
    handle("tab.close", onTab(tabActions.close)),
    handle("tab.close_others", onTab(tabActions.closeOthers)),
    handle("tab.close_right", onTab(tabActions.closeRight)),
    handle("tab.pin", onTab(tabActions.togglePin)),
    handle("tab.duplicate", onTab(tabActions.duplicate)),
    handle(
      "tab.next",
      onTab(() => tabActions.step(1)),
    ),
    handle(
      "tab.prev",
      onTab(() => tabActions.step(-1)),
    ),
    handle(
      "tab.move_right",
      onTab((key) => tabActions.shift(1, key)),
    ),
    handle(
      "tab.move_left",
      onTab((key) => tabActions.shift(-1, key)),
    ),
    ...[1, 2, 3, 4, 5, 6, 7, 8].map((n) =>
      handle(
        `tab.go_${n}`,
        onTab(() => tabActions.goTo(n)),
      ),
    ),
    handle(
      "tab.go_last",
      onTab(() => tabActions.goTo(Number.POSITIVE_INFINITY)),
    ),
    handle(
      "tab.reopen",
      onTab(() => tabActions.reopen()),
    ),
    ...registerThemeCommands({ visit: visitPath, prompt }),
  ]
  onCleanup(() => {
    for (const f of off) f()
  })

  // Every navigation lands in a tab: screens replace the current tab, entities get their own.
  createEffect(on(here, (path) => tabActions.visit(path)))
  // The start screen, or the tabs from last time. A reload on another page stays there.
  // Tabs are saved only after this, so the single launch tab never replaces the saved ones.
  const [started, setStarted] = createSignal(false)
  onMount(async () => {
    try {
      if (!isTauri() || here() !== "/") return
      const start = await commands.startTabs()
      tabActions.restore(start.tabs, start.active)
      syncTab()
    } finally {
      setStarted(true)
    }
  })
  // Remember the tabs for "Open on launch: Tabs from last time", a moment after they change.
  let saveTimer: ReturnType<typeof setTimeout> | undefined
  createEffect(() => {
    if (!started() || shell.look.startScreen !== "last") return
    const saved = savedTabs(tabs)
    clearTimeout(saveTimer)
    saveTimer = setTimeout(() => void commands.saveTabs(saved.tabs, saved.active), 1000)
  })
  onCleanup(() => clearTimeout(saveTimer))

  // Each tab remembers how far its page was scrolled. Only the active tab's screen exists, so
  // this is all a background tab keeps.
  let main: HTMLElement | undefined
  // The picture runs up into the active tab, so it is measured against the strip.
  let column: HTMLDivElement | undefined
  const [strip, setStrip] = createSignal<HTMLElement>()
  onMount(() => setStrip(column?.querySelector<HTMLElement>('[data-component="tab-strip"]') ?? undefined))
  const scrolls = new Map<string, number>()
  const restoreScroll = (key: string, y: number) => {
    let tries = 0
    const tick = () => {
      if (!main || tabs.active !== key) return
      // Wait for the router to show the tab and the screen to grow tall enough.
      const ready = activeTab(tabs)?.path === here() && main.scrollHeight - main.clientHeight >= y
      if (ready || ++tries > 60) main.scrollTop = y
      else requestAnimationFrame(tick)
    }
    requestAnimationFrame(tick)
  }
  createEffect(
    on(
      () => [tabs.active, activeTab(tabs)?.path] as const,
      ([key], prev) => {
        if (prev && prev[0] === key) scrolls.delete(key) // a new page in the same tab starts at the top
        for (const k of scrolls.keys()) if (!tabs.tabs.some((t) => t.key === k)) scrolls.delete(k)
        restoreScroll(key, scrolls.get(key) ?? 0)
      },
      { defer: true },
    ),
  )
  const appearance = useAppearance()
  // Display settings, applied whenever they change.
  createEffect(() => {
    const root = document.documentElement.dataset
    root.density = shell.look.density
    root.motion = shell.look.motion
    setFormatLocale(shell.look.formatLocale)
    setClockFormat(shell.look.timeFormat)
    setRelativeDates(shell.look.relativeDates)
  })
  // CSS zoom on the root scales everything, including popovers, and keeps layout in px.
  createEffect(() => {
    document.documentElement.style.setProperty("zoom", String(zoom()))
  })

  // Backend toasts show once in the app toaster, then leave rbx-core's list. A refetch that
  // raced the dismissal can bring one back, so remember what was shown.
  const shown = new Set<number>()
  createEffect(() => {
    for (const t of shell.toasts) {
      if (shown.has(t.id)) continue
      shown.add(t.id)
      showToast({
        title: t.text,
        variant: t.error ? "error" : t.retryAfterSecs ? "warning" : "neutral",
        countdownSecs: t.retryAfterSecs ?? undefined,
        key: t.host ?? undefined,
      })
      void commands.dismissToast(t.id)
    }
  })

  makeEventListener(window, "keydown", shortcutListener())

  onMount(() => {
    Promise.all([loadCommands(), startSync()]).catch((e) => showToast(String(e)))
  })

  return (
    <div class="flex h-full min-h-0 flex-col bg-deep text-fg">
      <TitleBar />
      <div class="flex min-h-0 flex-1">
        <AccountRail />
        <div class="flex min-w-0 flex-1 flex-col">
          <TopBar drawer={drawer()} />
          <div class="flex min-h-0 flex-1">
            <Sidebar compact={compact()} />
            <div ref={column} class="relative isolate flex min-w-0 flex-1 flex-col">
              <Backdrop background={appearance.background} strip={strip()} />
              <TabBar />
              <main
                ref={main}
                data-scroller
                class="min-h-0 flex-1 bg-canvas"
                onScroll={(e) => scrolls.set(tabs.active, e.currentTarget.scrollTop)}
              >
                <div class="flex h-full flex-col">
                  <ShellAlerts />
                  <ChallengeDialog />
                  <ErrorBoundary
                    fallback={(err) => (
                      <pre class="whitespace-pre-wrap p-6 font-mono text-sm text-danger">
                        {String(err?.stack ?? err)}
                      </pre>
                    )}
                  >
                    {props.children}
                  </ErrorBoundary>
                </div>
              </main>
            </div>
            <Show when={preview()}>
              {(p) => (
                <aside
                  aria-label="Item preview"
                  data-scroller
                  class="flex w-72 shrink-0 flex-col gap-3 overflow-y-auto border-l border-line bg-canvas p-3"
                >
                  <ItemPreview id={p().id} bundle={p().bundle} onClose={() => setPreview(null)} />
                </aside>
              )}
            </Show>
            <Show when={drawer()}>
              <Drawer />
            </Show>
          </div>
        </div>
      </div>
      <Show when={shell.look.statusBar}>
        <StatusBar zoom={zoom()} />
      </Show>
    </div>
  )
}

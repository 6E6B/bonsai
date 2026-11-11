import { createEffect, createMemo, createSignal, For, Match, onCleanup, onMount, Show, Switch, untrack } from "solid-js"
import { createStore } from "solid-js/store"
import { filterEntries } from "~/lib/commands/filter"
import { highlightParts } from "~/lib/commands/highlight"
import { intentRows, remoteTail } from "~/lib/commands/intent"
import {
  booster,
  completion,
  emptyUsage,
  forget,
  loadUsage,
  recentQueries,
  record,
  rememberArg,
  saveUsage,
  type Usage,
} from "~/lib/commands/learn"
import {
  argEntries,
  argHint,
  argToRemember,
  entityActions,
  flatRows,
  hasActions,
  type PaletteEntity,
  type PaletteEntry,
  type PalettePerson,
  type PaletteSection,
  paletteEntries,
  paletteSections,
  parseScope,
  recentRows,
  refOf,
  remoteEntries,
  remoteQuery,
  type Scope,
  scopes,
  seenEntry,
} from "~/lib/commands/palette"
import { command, commandList, run } from "~/lib/commands/registry"
import { enterVerb, rowIcon } from "~/lib/commands/rowkind"
import { ago } from "~/lib/format"
import { forgetSeen, historyOf, noteUse, openable, type Seen, seenBoost } from "~/lib/history"
import { commands, type PluginView, type PrefView, type RemoteSearch, type ThemeView } from "~/lib/ipc/bindings"
import { screenKey, screens, sidebarGroups, sidebarScreens } from "~/lib/screens"
import { friends, playlists, shell, useLive, visibleBookmarks } from "~/lib/store"
import { tabLabel, tabs } from "~/lib/tabs"
import { thumbUrl } from "~/lib/thumb"
import {
  Avatar,
  Badge,
  Icon,
  Kbd,
  SegmentedControl,
  Skeleton,
  Spinner,
  TextField,
  Thumbnail,
  useDialog,
} from "~/lib/ui"

/** What each account picked for what it typed, loaded from this machine on first use. */
const [usage, setUsage] = createStore<Record<string, Usage>>({})

function usageOf(account: string): Usage {
  if (!usage[account]) setUsage(account, loadUsage(account))
  return usage[account]
}

/** The command bar. Typing searches commands, friends and Roblox (people, experiences,
 * communities, items) at once; ←/→, Tab or a prefix (`@`, `g:`, `#`, `i:`, `>`) narrows it to one.
 * Alt+→ on a result lists what else you can do with it. Commands that take a typed argument ask for
 * it after you pick them. What you picked here and what you looked at anywhere in the app
 * (`~/lib/history`) rank higher, and things you looked at are found from the first letter.
 *
 * Keys: ↑/↓ move, Ctrl+↑/↓ jump between groups, Ctrl+1–9 pick a row, Ctrl+Enter shows all of a
 * cut-short group, Tab or → accepts the faint completion (else changes the scope), Shift+Delete
 * forgets a remembered row. */
export function CommandPalette() {
  const dialog = useDialog()
  const themes = useLive(
    () => "themes",
    () => commands.themes(),
    (): ThemeView[] => [],
  )
  const prefs = useLive(
    () => "prefs",
    () => commands.prefs(),
    (): PrefView[] => [],
  )
  const plugins = useLive(
    () => "plugins",
    () => commands.plugins(),
    (): PluginView[] => [],
  )
  const entries = createMemo(() =>
    paletteEntries(
      commandList(),
      shell.accounts,
      tabs.tabs.map((t) => ({ key: t.key, title: tabLabel(t.path).title })),
      sidebarScreens().map((s) => ({ key: screenKey(s), title: s.label })),
      themes,
      prefs,
      plugins,
      playlists.lists,
      visibleBookmarks(),
    ),
  )
  const people = createMemo(() =>
    friends.friends.map((f) => ({
      id: f.id,
      name: f.name,
      displayName: f.displayName,
      nickname: f.nickname,
      inGame: f.presence === "game",
      presence: f.presence,
      status: f.status,
    })),
  )

  const account = () => String(shell.active ?? 0)
  const places = createMemo(() => [
    ...sidebarGroups(screens, shell.sidebar)
      .flat()
      .map((s) => s.command),
    "nav.settings",
  ])

  const [scope, setScope] = createSignal<Scope>("all")
  const [query, setQuery] = createSignal("")
  /** A command waiting for its argument. */
  const [asking, setAsking] = createSignal<PaletteEntry | null>(null)
  /** A result whose actions are listed, and the query to restore when going back. */
  const [actionsOf, setActionsOf] = createSignal<{ entry: PaletteEntry; query: string } | null>(null)
  const [active, setActive] = createSignal(0)
  /** What was typed before picking the command now asking for its argument. */
  const [askedWith, setAskedWith] = createSignal("")
  const nested = () => asking() !== null || actionsOf() !== null

  const history = () => historyOf(account())
  const currentUsage = () => usage[account()] ?? emptyUsage()
  const saveTo = (a: string, u: Usage) => {
    setUsage(a, u)
    saveUsage(a, u)
  }
  const seenRows = createMemo(() => openable(history()).flatMap((s) => seenEntry(s) ?? []))
  /** Extra score for a row key or ref from what was picked here and looked at anywhere. */
  const boost = createMemo(() => {
    const picked = booster(currentUsage(), query())
    const h = history()
    const now = Date.now()
    return (key: string) => picked(key) + seenBoost(h, key, now)
  })

  /** The argument part of "join tower of hell", searched instead of the whole query. */
  const tail = createMemo(() =>
    scope() === "all" && !asking() && !actionsOf() ? remoteTail(entries(), query()) : null,
  )
  const noResults = (): RemoteSearch => ({ loading: false, rows: [] })
  const remoteFor = () => {
    if (nested()) return asking()?.ask === "user" ? remoteQuery("people", query()) : ""
    return remoteQuery(scope(), tail() ?? query())
  }
  const remote = useLive(
    () => (actionsOf() ? "" : remoteFor()),
    async (q) => (q ? commands.remoteSearch(q) : noResults()),
    noResults,
  )
  const remoteRows = createMemo(() => remoteEntries(remote.rows))

  const sections = createMemo((): PaletteSection[] => {
    const of = actionsOf()
    if (of?.entry.entity) {
      const rows = entityActions(of.entry.entity, (id) => command(id)?.title)
      return [{ title: "", rows: filterEntries(rows, query()) }]
    }
    const cmd = asking()
    if (cmd) {
      const recent = currentUsage().args?.[cmd.id] ?? []
      return [{ title: "", rows: argEntries(cmd, query(), people(), remoteRows(), 8, boost(), recent) }]
    }
    const all = scope() === "all" || scope() === "commands"
    return paletteSections({
      scope: scope(),
      text: query(),
      commands: entries(),
      friends: people(),
      remote: remoteRows(),
      bookmarks: visibleBookmarks(),
      recent: recentRows(
        entries(),
        Object.entries(currentUsage().picks).map(([key, p]) => ({ key, t: p.t })),
        openable(history()),
      ),
      searches: recentQueries(currentUsage(), 3),
      seen: seenRows(),
      places: places(),
      intents: all
        ? intentRows({
            commands: entries(),
            text: query(),
            people: people(),
            remote: remoteRows(),
            remoteFor: tail() ?? undefined,
          })
        : [],
      boost: boost(),
    })
  })
  const rows = createMemo(() => flatRows(sections()))
  /** Index of each section's first row in `rows`. */
  const starts = createMemo(() => {
    let n = 0
    return sections().map((s) => {
      const at = n
      n += s.rows.length
      return at
    })
  })

  /** The section each row is in. */
  const sectionAt = (i: number) => {
    const st = starts()
    let s = 0
    while (s + 1 < st.length && st[s + 1] <= i) s++
    return s
  }
  const activeRow = () => rows()[active()] as PaletteEntry | undefined
  const activeSection = () => sections()[sectionAt(active())] as PaletteSection | undefined
  /** The rest of a query typed before that starts like this one. */
  const ghost = createMemo(() => (nested() || !query() ? "" : (completion(currentUsage(), query()) ?? "")))
  /** Roblox is still answering and nothing from it is shown yet. */
  const waiting = () => remote.loading && remoteRows().length === 0 && remoteFor() !== ""
  /** Ctrl is held: rows show the number that picks them. */
  const [ctrl, setCtrl] = createSignal(false)

  let input: HTMLInputElement | undefined
  let list: HTMLDivElement | undefined

  // A new query starts at the top. Results arriving for the same query (Roblox answering)
  // keep the cursor on the row it was on, so Enter doesn't pick something that just moved in.
  let shownFor = ""
  let shown: PaletteEntry[] = []
  createEffect(() => {
    const next = rows()
    const now = `${scope()}\u0000${query()}\u0000${asking()?.key ?? ""}\u0000${actionsOf()?.entry.key ?? ""}`
    untrack(() => {
      const key = shown[active()]?.key
      const at = now === shownFor && key ? next.findIndex((e) => e.key === key) : -1
      setActive(at >= 0 ? at : 0)
      shownFor = now
      shown = next
    })
  })
  createEffect(() => {
    list?.querySelector(`[data-index="${active()}"]`)?.scrollIntoView({ block: "nearest" })
  })
  onMount(() => {
    usageOf(account())
    input?.focus()
    const held = (ev: KeyboardEvent) => setCtrl(ev.ctrlKey && !ev.altKey && !ev.shiftKey)
    const off = () => setCtrl(false)
    window.addEventListener("keydown", held)
    window.addEventListener("keyup", held)
    window.addEventListener("blur", off)
    onCleanup(() => {
      window.removeEventListener("keydown", held)
      window.removeEventListener("keyup", held)
      window.removeEventListener("blur", off)
    })
  })

  const onInput = (v: string) => {
    const typed = !asking() && !actionsOf() ? parseScope(v) : null
    if (typed) {
      setScope(typed.scope)
      setQuery(typed.text)
    } else setQuery(v)
  }
  const pickScope = (s: Scope) => {
    setScope(s)
    input?.focus()
  }
  const cycleScope = (by: 1 | -1) => {
    const i = scopes.findIndex((s) => s.value === scope())
    setScope(scopes[(i + by + scopes.length) % scopes.length].value)
  }
  const showActions = (e: PaletteEntry | undefined) => {
    if (!e || !hasActions(e)) return false
    setActionsOf({ entry: e, query: query() })
    setQuery("")
    input?.focus()
    return true
  }
  const back = () => {
    const of = actionsOf()
    if (of) {
      setActionsOf(null)
      setQuery(of.query)
    } else {
      setAsking(null)
      setQuery("")
    }
    input?.focus()
  }

  const pick = (e: PaletteEntry | undefined) => {
    if (!e) return
    if (e.fill !== undefined) {
      setQuery(e.fill)
      input?.focus()
      return
    }
    if (e.ask) {
      setAsking(e)
      setAskedWith(query())
      setQuery("")
      input?.focus()
      return
    }
    // Learn the whole path: the command that asked, the result whose actions were listed, and
    // the row picked, each with what was typed for it.
    const a = account()
    const asked = asking()
    const of = actionsOf()
    let u = usageOf(a)
    if (asked) u = record(u, asked.key, askedWith())
    if (of) u = record(u, of.entry.key, of.query)
    u = record(u, e.key, query())
    const about = of?.entry ?? e
    const ref = refOf(e) ?? refOf(about)
    // What a row is about learns the query too, so another row about it ranks higher for it.
    if (ref && ref !== e.key) u = record(u, ref, asked ? askedWith() : (of?.query ?? query()))
    const again = argToRemember(e, asked, of?.entry ?? null, (id) => command(id)?.arg)
    if (again) u = rememberArg(u, again.command, again.arg)
    saveTo(a, u)
    if (ref) {
      const open = about.entity && entityActions(about.entity, () => "")[0]
      noteUse(a, {
        ref,
        title: about.entity ? about.title : undefined,
        thumb: about.entity ? about.thumb : undefined,
        entity: about.entity && { kind: about.entity.kind, id: about.entity.id },
        open: open ? { id: open.id, arg: open.arg ?? "" } : undefined,
      })
    }
    dialog.close()
    void run(e.id, e.arg)
  }

  /** Stops remembering a row picked, searched or looked at before. */
  const forgetRow = (e: PaletteEntry | undefined) => {
    if (!e || !forgettable(e)) return false
    const a = account()
    const ref = refOf(e)
    let u = usageOf(a)
    if (e.fill !== undefined) u = { ...u, queries: u.queries.filter((q) => q.q !== e.fill) }
    u = forget(u, [e.key, ...(ref ? [ref, `seen:${ref}`] : [])])
    saveTo(a, u)
    if (ref) forgetSeen(a, ref)
    return true
  }
  const forgettable = (e: PaletteEntry) => e.recent === true

  /** The first row of the next (`by` 1) or previous section. */
  const jumpSection = (by: 1 | -1) => {
    const st = starts().filter((_, i) => sections()[i].rows.length > 0)
    if (!st.length) return
    const i = active()
    const next = by === 1 ? st.find((at) => at > i) : [...st].reverse().find((at) => at < i)
    setActive(next ?? (by === 1 ? st[0] : st[st.length - 1]))
  }
  const showAll = (s: PaletteSection | undefined) => {
    if (!s?.more || s.more === scope()) return false
    pickScope(s.more)
    return true
  }
  const accept = () => {
    const rest = ghost()
    if (!rest) return false
    setQuery(query() + rest)
    return true
  }

  const atEnd = () => input !== undefined && input.selectionStart === input.value.length
  const atStart = () => input !== undefined && input.selectionStart === 0 && input.selectionEnd === 0

  const onKeyDown = (ev: KeyboardEvent) => {
    const n = rows().length
    const empty = query() === ""
    const digit = /^Digit([1-9])$/.exec(ev.code)
    if (digit && ev.ctrlKey && !ev.altKey) {
      const e = rows()[Number(digit[1]) - 1]
      if (!e) return
      pick(e)
    } else if ((ev.key === "ArrowDown" || ev.key === "ArrowUp") && ev.ctrlKey && n > 0)
      jumpSection(ev.key === "ArrowDown" ? 1 : -1)
    else if (ev.key === "ArrowDown" && n > 0) setActive((i) => (i + 1) % n)
    else if (ev.key === "ArrowUp" && n > 0) setActive((i) => (i - 1 + n) % n)
    else if (ev.key === "Enter" && ev.ctrlKey) {
      if (!showAll(activeSection())) return
    } else if (ev.key === "Enter") pick(activeRow())
    else if (ev.key === "Delete" && ev.shiftKey) {
      if (!forgetRow(activeRow())) return
    } else if (ev.key === "Tab" && !ev.shiftKey && accept()) {
      // Took the completion.
    } else if (ev.key === "Tab" && !nested()) cycleScope(ev.shiftKey ? -1 : 1)
    else if (ev.key === "ArrowRight" && !nested() && ev.altKey) {
      if (!showActions(activeRow())) return
    } else if (ev.key === "ArrowRight" && !nested() && atEnd() && !ev.shiftKey) {
      if (!accept()) cycleScope(1)
    } else if (ev.key === "ArrowLeft" && !nested() && atStart() && !ev.shiftKey) cycleScope(-1)
    else if ((ev.key === "ArrowLeft" || ev.key === "Backspace") && actionsOf() && empty) back()
    else if (ev.key === "Backspace" && asking() && empty) back()
    else if (ev.key === "Backspace" && !nested() && empty && scope() !== "all") setScope("all")
    else if (ev.key === "Escape" && nested()) back()
    else return
    ev.preventDefault()
  }

  const seenOf = (e: PaletteEntry): Seen | undefined => {
    const ref = refOf(e)
    return ref ? history()[ref] : undefined
  }
  const personOf = (e: PaletteEntry): PalettePerson | undefined => {
    const ref = refOf(e)
    return ref?.startsWith("user:") ? people().find((p) => `user:${p.id}` === ref) : undefined
  }
  const titleOf = (id: string) => command(id)?.title

  const placeholder = () => {
    const cmd = asking()
    if (cmd) return argHint(cmd.ask ?? "text")
    if (actionsOf()) return "Filter actions"
    switch (scope()) {
      case "people":
        return "Username, display name or user ID"
      case "experiences":
        return "Experience name or place ID"
      case "communities":
        return "Community name or ID"
      case "items":
        return "Item name or ID"
      case "commands":
        return "Run a command"
      default:
        return "Search players, experiences, items or commands"
    }
  }

  const emptyText = () => {
    if (asking()) return "Type, then press Enter"
    const t = query().trim()
    if (t.length === 0) return "Type to search"
    if (scope() !== "commands" && scope() !== "bookmarks" && t.length > 0 && remoteQuery(scope(), t) === "")
      return "Type 3 letters to search Roblox"
    return "No matches"
  }

  return (
    <div class="flex flex-col gap-2 p-2">
      <Switch>
        <Match when={asking()}>
          {(c) => <span class="px-2 text-sm text-fg-muted">{c().title.replace(/…$/, "")}</span>}
        </Match>
        <Match when={actionsOf()}>
          {(of) => (
            <button type="button" class="flex items-center gap-1 px-2 text-left text-sm text-fg-muted" onClick={back}>
              <Icon name="chevron-left" size={14} />
              {of().entry.title}
            </button>
          )}
        </Match>
      </Switch>
      <TextField
        ref={input}
        size="lg"
        icon="search"
        aria-label={asking() ? "Argument" : "Search"}
        placeholder={placeholder()}
        value={query()}
        completion={ghost()}
        onChange={onInput}
        onKeyDown={onKeyDown}
      />
      <Show when={!nested()}>
        <div class="flex items-center justify-between gap-2">
          <SegmentedControl
            size="sm"
            aria-label="Search in"
            options={scopes.map((s) => ({ value: s.value, label: s.label, hint: s.prefix || undefined }))}
            value={scope()}
            onChange={(v) => pickScope(v as Scope)}
          />
          <Show when={remote.loading}>
            <Spinner size="sm" />
          </Show>
        </div>
      </Show>
      <div class="flex h-[28rem] min-h-0 gap-3">
        <div ref={list} class="flex min-w-0 flex-1 flex-col overflow-y-auto">
          <For each={sections()}>
            {(section, s) => (
              <Show when={section.rows.length > 0}>
                <Show when={section.title}>
                  <div class="flex shrink-0 items-center justify-between gap-2 px-2 pt-2 pb-1 text-xs text-fg-muted">
                    <span>{section.title}</span>
                    <Show when={section.more && section.more !== scope()}>
                      <button
                        type="button"
                        tabIndex={-1}
                        class="text-fg-muted hover:text-fg"
                        onClick={() => showAll(section)}
                      >
                        Show all
                      </button>
                    </Show>
                  </div>
                </Show>
                <For each={section.rows}>
                  {(e, r) => {
                    const i = () => starts()[s()] + r()
                    const on = () => i() === active()
                    return (
                      <button
                        type="button"
                        data-index={i()}
                        class="flex h-8 shrink-0 items-center gap-2 rounded-md px-2 text-left"
                        classList={{ "bg-active": on() }}
                        onMouseMove={() => setActive(i())}
                        onClick={() => pick(e)}
                        onContextMenu={(ev) => {
                          if (showActions(e)) ev.preventDefault()
                        }}
                      >
                        <RowPicture entry={e} />
                        <span class="min-w-0 flex-1 truncate">
                          <Highlighted text={e.title} query={query()} />
                          <Show when={e.subtitle}>
                            {(sub) => <span class="ml-2 text-sm text-fg-muted">{sub()}</span>}
                          </Show>
                        </span>
                        <Show when={e.recent && e.fill === undefined}>
                          <Icon name="clock" size={12} class="shrink-0 text-fg-faint" />
                        </Show>
                        <Show when={e.shortcut}>{(k) => <Kbd keys={[k()]} />}</Show>
                        <Show
                          when={ctrl() && i() < 9}
                          fallback={
                            <Show when={on()}>
                              <span class="shrink-0 text-xs text-fg-muted">{enterVerb(e)}</span>
                              <Show when={!nested() && hasActions(e)}>
                                <Icon name="chevron-right" size={14} class="text-fg-muted" />
                              </Show>
                            </Show>
                          }
                        >
                          <Kbd keys={[String(i() + 1)]} />
                        </Show>
                      </button>
                    )
                  }}
                </For>
              </Show>
            )}
          </For>
          <Show when={waiting()}>
            <div class="px-2 pt-2 pb-1 text-xs text-fg-muted">Roblox</div>
            <For each={[40, 28, 34]}>
              {(w) => (
                <div class="flex h-8 shrink-0 items-center gap-2 px-2">
                  <Skeleton class="size-6 rounded-md" />
                  <Skeleton class={`h-3 ${w === 40 ? "w-40" : w === 28 ? "w-28" : "w-32"}`} />
                </div>
              )}
            </For>
          </Show>
          <Show when={rows().length === 0 && !waiting()}>
            <div class="px-2 py-1.5 text-fg-faint">{emptyText()}</div>
          </Show>
        </div>
        <Show when={activeRow()}>
          {(e) => (
            <Preview
              entry={e()}
              query={query()}
              seen={seenOf(e())}
              person={personOf(e())}
              actions={!nested() && e().entity ? entityActions(e().entity as PaletteEntity, titleOf) : []}
              keywords={command(e().id)?.keywords ?? []}
              onAction={(a) => {
                showActions(e())
                pick(a)
              }}
            />
          )}
        </Show>
      </div>
      <div class="flex flex-wrap items-center gap-x-3 gap-y-1 px-2 text-xs text-fg-muted">
        <span class="flex items-center gap-1">
          <Kbd keys={["↵"]} /> {enterVerb(activeRow())}
        </span>
        <Show when={ghost()}>
          <span class="flex items-center gap-1">
            <Kbd keys={["Tab"]} /> Complete
          </span>
        </Show>
        <Show when={!nested() && activeRow() && hasActions(activeRow() as PaletteEntry)}>
          <span class="flex items-center gap-1">
            <Kbd keys={["Alt", "→"]} /> Actions
          </span>
        </Show>
        <Show when={activeSection()?.more && activeSection()?.more !== scope()}>
          <span class="flex items-center gap-1">
            <Kbd keys={["Ctrl", "↵"]} /> Show all
          </span>
        </Show>
        <Show when={activeRow()?.recent}>
          <span class="flex items-center gap-1">
            <Kbd keys={["Shift", "Del"]} /> Forget
          </span>
        </Show>
        <Show
          when={nested()}
          fallback={
            <span class="flex items-center gap-1">
              <Kbd keys={["←"]} /> <Kbd keys={["→"]} /> Filter
            </span>
          }
        >
          <span class="flex items-center gap-1">
            <Kbd keys={["Esc"]} /> Back
          </span>
        </Show>
        <span class="ml-auto flex items-center gap-1">
          <Kbd keys={["Ctrl", "1–9"]} /> Pick
        </span>
      </div>
    </div>
  )
}

/** A title with the letters the query matched in bold. */
function Highlighted(props: { text: string; query: string }) {
  return (
    <For each={highlightParts(props.text, props.query)}>
      {(p) => (
        <Show when={p.hit} fallback={p.text}>
          <span class="font-semibold">{p.text}</span>
        </Show>
      )}
    </For>
  )
}

const KIND_LABEL: Record<PaletteEntity["kind"], string> = {
  user: "Person",
  experience: "Experience",
  community: "Community",
  item: "Item",
  bundle: "Bundle",
}

/** What kind of row this is, in a word, for the preview. */
function kindLabel(e: PaletteEntry): string {
  if (e.fill !== undefined) return "Recent search"
  if (e.entity) return KIND_LABEL[e.entity.kind]
  if (e.ask) return "Command · asks for input"
  return "Command"
}

/** More about the selected row: a bigger picture, what it is, when you last looked at it, and
 * its other actions (clickable), or a command's shortcut and other names. */
function Preview(props: {
  entry: PaletteEntry
  query: string
  seen?: Seen
  person?: PalettePerson
  actions: PaletteEntry[]
  keywords: string[]
  onAction: (a: PaletteEntry) => void
}) {
  const thumb = () => props.entry.thumb
  const status = () => props.person?.status
  return (
    <aside class="flex w-80 shrink-0 flex-col overflow-hidden rounded-lg border border-line bg-raised text-sm">
      <div class="flex items-center gap-3 border-b border-line p-4">
        <Show
          when={thumb()}
          fallback={
            <span class="flex size-14 shrink-0 items-center justify-center rounded-lg bg-surface text-fg-muted">
              <Icon name={rowIcon(props.entry)} size={24} />
            </span>
          }
        >
          {(t) => (
            <Show
              when={t().kind === "headshot"}
              fallback={<Thumbnail class="size-16 shrink-0 rounded-lg" src={thumbUrl(t().kind, t().id)} alt="" />}
            >
              <Avatar
                size="lg"
                class="shrink-0"
                name={props.entry.title}
                src={thumbUrl("headshot", t().id)}
                presence={props.person?.presence}
              />
            </Show>
          )}
        </Show>
        <div class="flex min-w-0 flex-1 flex-col gap-0.5">
          <span class="text-xs text-fg-faint">{kindLabel(props.entry)}</span>
          <span class="truncate text-base font-semibold" title={props.entry.title}>
            {props.entry.title}
          </span>
          <Show when={props.entry.subtitle}>
            {(sub) => <span class="truncate text-xs text-fg-muted">{sub()}</span>}
          </Show>
        </div>
      </div>
      <Show when={status() || props.seen || props.entry.shortcut}>
        <div class="flex flex-col gap-1.5 border-b border-line px-4 py-3 text-xs text-fg-muted">
          <Show when={status()}>
            {(st) => (
              <span class="flex items-center gap-2">
                <Icon name={props.person?.inGame ? "game" : "user"} size={12} class="shrink-0" />
                <span class="truncate">{st()}</span>
              </span>
            )}
          </Show>
          <Show when={props.seen}>
            {(s) => (
              <span class="flex items-center gap-2">
                <Icon name="clock" size={12} class="shrink-0" />
                Looked at {s().n === 1 ? "once" : `${s().n} times`}, last {ago(s().t)}
              </span>
            )}
          </Show>
          <Show when={props.entry.shortcut}>
            {(k) => (
              <span class="flex items-center gap-2">
                <Icon name="keyboard" size={12} class="shrink-0" />
                Shortcut <Kbd keys={[k()]} />
              </span>
            )}
          </Show>
        </div>
      </Show>
      <div class="flex min-h-0 flex-1 flex-col overflow-y-auto p-2">
        <Show
          when={props.actions.length > 1}
          fallback={
            <Show when={props.keywords.length > 0 && !props.entry.entity}>
              <span class="px-2 pt-1 pb-2 text-xs text-fg-muted">Also found by</span>
              <div class="flex flex-wrap gap-1 px-2">
                <For each={props.keywords.slice(0, 10)}>{(k) => <Badge>{k}</Badge>}</For>
              </div>
            </Show>
          }
        >
          <span class="px-2 pt-1 pb-1 text-xs text-fg-muted">Actions</span>
          <For each={props.actions}>
            {(a, i) => (
              <button
                type="button"
                tabIndex={-1}
                class="flex h-8 shrink-0 items-center gap-2 rounded-md px-2 text-left hover:bg-hover"
                onClick={() => props.onAction(a)}
              >
                <Icon name={rowIcon(a)} size={14} class="shrink-0 text-fg-muted" />
                <span class="min-w-0 flex-1 truncate">{a.title}</span>
                <Show when={i() === 0}>
                  <Kbd keys={["↵"]} />
                </Show>
              </button>
            )}
          </For>
        </Show>
      </div>
      <Show when={props.actions.length > 1}>
        <div class="flex items-center gap-1 border-t border-line px-4 py-2 text-xs text-fg-muted">
          <Kbd keys={["Alt", "→"]} /> Search actions
        </div>
      </Show>
    </aside>
  )
}

/** A result's picture: a round headshot for people, a square thumbnail for the rest, an icon
 * for commands. */
function RowPicture(props: { entry: PaletteEntry }) {
  return (
    <Show
      when={props.entry.thumb}
      fallback={
        <span class="flex size-6 shrink-0 items-center justify-center text-fg-muted">
          <Icon name={rowIcon(props.entry)} size={14} />
        </span>
      }
    >
      {(t) => (
        <Show
          when={t().kind === "headshot"}
          fallback={<Thumbnail class="size-6 shrink-0" src={thumbUrl(t().kind, t().id)} alt="" />}
        >
          <Avatar size="xs" class="shrink-0" name={props.entry.title} src={thumbUrl("headshot", t().id)} />
        </Show>
      )}
    </Show>
  )
}

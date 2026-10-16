import { createEffect, createMemo, createSignal, For, Match, onMount, Show, Switch } from "solid-js"
import { filterEntries } from "~/lib/commands/filter"
import {
  argEntries,
  argHint,
  entityActions,
  flatRows,
  hasActions,
  type PaletteEntry,
  type PaletteSection,
  paletteEntries,
  paletteSections,
  parseScope,
  remoteEntries,
  remoteQuery,
  type Scope,
  scopes,
} from "~/lib/commands/palette"
import { command, commandList, run } from "~/lib/commands/registry"
import { commands, type RemoteSearch, type ThemeView } from "~/lib/ipc/bindings"
import { screenKey, sidebarScreens } from "~/lib/screens"
import { friends, shell, useLive } from "~/lib/store"
import { tabLabel, tabs } from "~/lib/tabs"
import { thumbUrl } from "~/lib/thumb"
import { Avatar, Icon, Kbd, SegmentedControl, Spinner, TextField, Thumbnail, useDialog } from "~/lib/ui"

/** The command bar. Typing searches commands, friends and Roblox (people, experiences,
 * communities, items) at once; ←/→, Tab or a prefix (`@`, `g:`, `#`, `i:`, `>`) narrows it to one.
 * Alt+→ on a result lists what else you can do with it. Commands that take a typed argument ask for
 * it after you pick them. */
export function CommandPalette() {
  const dialog = useDialog()
  const themes = useLive(
    () => "themes",
    () => commands.themes(),
    (): ThemeView[] => [],
  )
  const entries = createMemo(() =>
    paletteEntries(
      commandList(),
      shell.accounts,
      tabs.tabs.map((t) => ({ key: t.key, title: tabLabel(t.path).title })),
      sidebarScreens().map((s) => ({ key: screenKey(s), title: s.label })),
      themes,
    ),
  )
  const people = createMemo(() =>
    friends.friends.map((f) => ({
      id: f.id,
      name: f.name,
      displayName: f.displayName,
      inGame: f.presence === "game",
      presence: f.presence,
      status: f.status,
    })),
  )

  const [scope, setScope] = createSignal<Scope>("all")
  const [query, setQuery] = createSignal("")
  /** A command waiting for its argument. */
  const [asking, setAsking] = createSignal<PaletteEntry | null>(null)
  /** A result whose actions are listed, and the query to restore when going back. */
  const [actionsOf, setActionsOf] = createSignal<{ entry: PaletteEntry; query: string } | null>(null)
  const [active, setActive] = createSignal(0)

  const noResults = (): RemoteSearch => ({ loading: false, rows: [] })
  const remote = useLive(
    () => {
      if (actionsOf()) return ""
      const cmd = asking()
      if (cmd) return cmd.ask === "user" ? remoteQuery("people", query()) : ""
      return remoteQuery(scope(), query())
    },
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
    if (cmd) return [{ title: "", rows: argEntries(cmd, query(), people(), remoteRows()) }]
    return paletteSections({
      scope: scope(),
      text: query(),
      commands: entries(),
      friends: people(),
      remote: remoteRows(),
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

  let input: HTMLInputElement | undefined
  let list: HTMLDivElement | undefined

  createEffect(() => {
    rows()
    setActive(0)
  })
  createEffect(() => {
    list?.querySelector(`[data-index="${active()}"]`)?.scrollIntoView({ block: "nearest" })
  })
  onMount(() => input?.focus())

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
    if (e.ask) {
      setAsking(e)
      setQuery("")
      input?.focus()
      return
    }
    dialog.close()
    void run(e.id, e.arg)
  }

  const atEnd = () => input !== undefined && input.selectionStart === input.value.length
  const atStart = () => input !== undefined && input.selectionStart === 0 && input.selectionEnd === 0
  const nested = () => asking() !== null || actionsOf() !== null

  const onKeyDown = (ev: KeyboardEvent) => {
    const n = rows().length
    const empty = query() === ""
    if (ev.key === "ArrowDown" && n > 0) setActive((i) => (i + 1) % n)
    else if (ev.key === "ArrowUp" && n > 0) setActive((i) => (i - 1 + n) % n)
    else if (ev.key === "Enter") pick(rows()[active()])
    else if (ev.key === "Tab" && !nested()) cycleScope(ev.shiftKey ? -1 : 1)
    else if (ev.key === "ArrowRight" && !nested() && ev.altKey) {
      if (!showActions(rows()[active()])) return
    } else if (ev.key === "ArrowRight" && !nested() && atEnd() && !ev.shiftKey) cycleScope(1)
    else if (ev.key === "ArrowLeft" && !nested() && atStart() && !ev.shiftKey) cycleScope(-1)
    else if ((ev.key === "ArrowLeft" || ev.key === "Backspace") && actionsOf() && empty) back()
    else if (ev.key === "Backspace" && asking() && empty) back()
    else if (ev.key === "Backspace" && !nested() && empty && scope() !== "all") setScope("all")
    else if (ev.key === "Escape" && nested()) back()
    else return
    ev.preventDefault()
  }

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
    if (remote.loading) return "Searching…"
    const t = query().trim()
    if (scope() !== "commands" && t.length > 0 && remoteQuery(scope(), t) === "")
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
        onChange={onInput}
        onKeyDown={onKeyDown}
      />
      <Show when={!nested()}>
        <div class="flex items-center justify-between gap-2">
          <SegmentedControl
            size="sm"
            aria-label="Search in"
            options={scopes.map((s) => ({ value: s.value, label: s.label }))}
            value={scope()}
            onChange={(v) => pickScope(v as Scope)}
          />
          <Show when={remote.loading}>
            <Spinner size="sm" />
          </Show>
        </div>
      </Show>
      <div ref={list} class="flex max-h-[28rem] flex-col overflow-y-auto">
        <Show when={rows().length > 0} fallback={<div class="px-2 py-1.5 text-fg-faint">{emptyText()}</div>}>
          <For each={sections()}>
            {(section, s) => (
              <>
                <Show when={section.title}>
                  <div class="px-2 pt-2 pb-1 text-xs text-fg-muted">{section.title}</div>
                </Show>
                <For each={section.rows}>
                  {(e, r) => {
                    const i = () => starts()[s()] + r()
                    return (
                      <button
                        type="button"
                        data-index={i()}
                        class="flex h-8 shrink-0 items-center gap-2 rounded-md px-2 text-left"
                        classList={{ "bg-active": i() === active() }}
                        onMouseMove={() => setActive(i())}
                        onClick={() => pick(e)}
                        onContextMenu={(ev) => {
                          if (showActions(e)) ev.preventDefault()
                        }}
                      >
                        <RowPicture entry={e} />
                        <span class="min-w-0 flex-1 truncate">
                          {e.title}
                          <Show when={e.subtitle}>
                            {(sub) => <span class="ml-2 text-sm text-fg-muted">{sub()}</span>}
                          </Show>
                        </span>
                        <Show when={e.shortcut}>{(k) => <Kbd keys={[k()]} />}</Show>
                        <Show when={i() === active() && !nested() && hasActions(e)}>
                          <Icon name="chevron-right" size={14} class="text-fg-muted" />
                        </Show>
                      </button>
                    )
                  }}
                </For>
              </>
            )}
          </For>
        </Show>
      </div>
      <div class="flex items-center gap-3 px-2 text-xs text-fg-muted">
        <span class="flex items-center gap-1">
          <Kbd keys={["↵"]} /> Open
        </span>
        <Show
          when={nested()}
          fallback={
            <>
              <span class="flex items-center gap-1">
                <Kbd keys={["Alt", "→"]} /> Actions
              </span>
              <span class="flex items-center gap-1">
                <Kbd keys={["←"]} /> <Kbd keys={["→"]} /> Filter
              </span>
            </>
          }
        >
          <span class="flex items-center gap-1">
            <Kbd keys={["Esc"]} /> Back
          </span>
        </Show>
      </div>
    </div>
  )
}

/** A result's picture: a round headshot for people, a square thumbnail for the rest, a blank
 * slot for commands so titles line up. */
function RowPicture(props: { entry: PaletteEntry }) {
  return (
    <Show when={props.entry.thumb} fallback={<span class="size-6 shrink-0" />}>
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

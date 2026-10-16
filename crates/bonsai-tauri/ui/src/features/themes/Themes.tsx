import { createMemo, createSignal, For, Show } from "solid-js"
import { menuItem, run } from "~/lib/commands/registry"
import { type AppSettings, commands, type ThemeView } from "~/lib/ipc/bindings"
import { useLive } from "~/lib/store"
import { parseTheme, resolve, type Theme } from "~/lib/theme"
import {
  Badge,
  Button,
  ContextMenu,
  EmptyState,
  IconButton,
  Page,
  SegmentedControl,
  TextField,
  ThemeCard,
} from "~/lib/ui"
import { changed, drafts } from "./draft"

/** A row in the grid: an installed theme, or a new one that only exists as a draft. */
type Row = { id: string; name: string; author: string; base: string; builtin: boolean; onDisk: boolean; theme?: Theme }

/** Installed themes and unsaved new ones. Clicking a theme uses it; the pencil opens the editor. */
export default function Themes() {
  const installed = useLive(
    () => "themes",
    () => commands.themes(),
    (): ThemeView[] => [],
  )
  const app = useLive(
    () => "app",
    () => commands.appSettings(),
    (): Pick<AppSettings, "darkTheme" | "lightTheme"> => ({ darkTheme: "", lightTheme: "" }),
  )
  const [query, setQuery] = createSignal("")
  const [base, setBase] = createSignal("all")

  const rows = createMemo((): Row[] => {
    const out: Row[] = installed.map((t) => ({ ...t, onDisk: true, theme: parseTheme(t.doc) }))
    for (const d of Object.values(drafts))
      if (!d.onDisk && changed(d)) out.push({ ...d.theme, builtin: false, onDisk: false, theme: d.theme })
    const q = query().trim().toLowerCase()
    return out.filter(
      (r) =>
        (base() === "all" || r.base === base()) &&
        (!q || r.name.toLowerCase().includes(q) || r.author.toLowerCase().includes(q)),
    )
  })

  const detail = (r: Row) => {
    const d = drafts[r.id]
    if (!r.onDisk) return "Not saved"
    if (d && changed(d)) return "Unsaved changes"
    if (r.id === app.darkTheme) return "Used in dark mode"
    if (r.id === app.lightTheme) return "Used in light mode"
    return r.author || (r.builtin ? "Built in" : "")
  }

  return (
    <Page
      title="Themes"
      actions={
        <>
          <TextField
            size="sm"
            icon="search"
            aria-label="Search themes"
            placeholder="Search"
            class="w-48"
            value={query()}
            onChange={setQuery}
          />
          <SegmentedControl
            size="sm"
            aria-label="Base"
            value={base()}
            onChange={setBase}
            options={[
              { value: "all", label: "All" },
              { value: "dark", label: "Dark" },
              { value: "light", label: "Light" },
            ]}
          />
          <Button size="sm" variant="primary" icon="plus" onClick={() => run("theme.new")}>
            New
          </Button>
        </>
      }
    >
      <Show
        when={rows().length > 0}
        fallback={<EmptyState icon="palette" title="No themes match" description="Try another name or base." />}
      >
        <div class="grid grid-cols-[repeat(auto-fill,minmax(10rem,1fr))] gap-3">
          <For each={rows()}>
            {(r) => {
              const tokens = createMemo(() => {
                const t = drafts[r.id]?.theme ?? r.theme
                return t ? resolve(t) : {}
              })
              return (
                <ContextMenu
                  items={[
                    menuItem("theme.apply", { arg: r.id, icon: "check", disabled: !r.onDisk }),
                    menuItem("theme.edit", { arg: r.id, icon: "edit" }),
                    menuItem("theme.duplicate", { arg: r.id, icon: "copy", disabled: !r.onDisk }),
                    menuItem("theme.rename", { arg: r.id, icon: "edit", disabled: r.builtin }),
                    menuItem("theme.delete", {
                      arg: r.id,
                      icon: "trash",
                      danger: true,
                      disabled: r.builtin || !r.onDisk,
                    }),
                  ]}
                >
                  <div class="group relative">
                    <ThemeCard
                      class="w-full"
                      name={drafts[r.id]?.theme.name ?? r.name}
                      detail={detail(r)}
                      tokens={tokens()}
                      selected={r.id === app.darkTheme || r.id === app.lightTheme}
                      onClick={() => run(r.onDisk ? "theme.apply" : "theme.edit", r.id)}
                    />
                    <Show when={!r.onDisk || (drafts[r.id] && changed(drafts[r.id]))}>
                      <Badge variant="warning" class="pointer-events-none absolute top-2 left-2">
                        Unsaved
                      </Badge>
                    </Show>
                    <div class="absolute top-2 right-2 opacity-0 group-focus-within:opacity-100 group-hover:opacity-100">
                      <IconButton
                        size="sm"
                        variant="secondary"
                        icon="edit"
                        label={r.builtin ? `Edit a copy of ${r.name}` : `Edit ${r.name}`}
                        onClick={() => run("theme.edit", r.id)}
                      />
                    </div>
                  </div>
                </ContextMenu>
              )
            }}
          </For>
        </div>
      </Show>
    </Page>
  )
}

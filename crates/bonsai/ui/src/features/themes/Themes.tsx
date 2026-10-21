import { createMemo, createSignal, onCleanup, Show } from "solid-js"
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
  Menu,
  type MenuEntry,
  Page,
  SectionHeader,
  SegmentedControl,
  TextField,
  ThemeCard,
  VirtualGrid,
} from "~/lib/ui"
import { changed, drafts } from "./draft"
import { isThemeCode } from "./import"

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
      if (!d.onDisk) out.push({ ...d.theme, builtin: false, onDisk: false, theme: d.theme })
    const q = query().trim().toLowerCase()
    return out.filter(
      (r) =>
        (base() === "all" || r.base === base()) &&
        (!q || r.name.toLowerCase().includes(q) || r.author.toLowerCase().includes(q)),
    )
  })

  // Pasting a theme code anywhere on the page, outside a text field, imports it.
  const onPaste = (e: ClipboardEvent) => {
    const t = e.target as HTMLElement | null
    if (t?.closest("input, textarea, [contenteditable]")) return
    const text = e.clipboardData?.getData("text/plain") ?? ""
    if (!isThemeCode(text)) return
    e.preventDefault()
    void run("theme.import", text.trim())
  }
  document.addEventListener("paste", onPaste)
  onCleanup(() => document.removeEventListener("paste", onPaste))

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
          <Button size="sm" onClick={() => run("nav.hub")}>
            Browse hub
          </Button>
          <Button size="sm" icon="download" onClick={() => run("theme.import")}>
            Import
          </Button>
          <Button size="sm" variant="primary" icon="plus" onClick={() => run("theme.new")}>
            Create theme
          </Button>
        </>
      }
    >
      <p class="text-sm text-fg-muted">
        Choose a theme to use it. The pencil edits a theme, making a copy of built-in themes.
      </p>
      <section class="flex flex-col gap-2">
        <SectionHeader
          title="Your library"
          meta={rows().length}
          actions={
            <>
              <TextField
                size="sm"
                icon="search"
                aria-label="Search themes"
                placeholder="Search themes…"
                class="w-64"
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
            </>
          }
        />
        <Show
          when={rows().length > 0}
          fallback={<EmptyState icon="palette" title="No themes match" description="Try another name or base." />}
        >
          <VirtualGrid items={rows()} minItemWidth={208} estimateRowHeight={240} gap={12}>
            {(r) => {
              const tokens = createMemo(() => {
                const t = drafts[r.id]?.theme ?? r.theme
                return t ? resolve(t) : {}
              })
              const actions = (): MenuEntry[] => [
                menuItem("theme.apply", { arg: r.id, icon: "check", disabled: !r.onDisk }),
                menuItem("theme.edit", { arg: r.id, icon: "edit" }),
                menuItem("theme.duplicate", { arg: r.id, icon: "copy", disabled: !r.onDisk }),
                menuItem("theme.rename", { arg: r.id, icon: "edit", disabled: r.builtin }),
                { kind: "separator" },
                menuItem("theme.export", { arg: r.id, icon: "archive", disabled: !r.onDisk }),
                menuItem("hub.publish", {
                  arg: JSON.stringify({ kind: "themes", id: r.id }),
                  label: "Publish to hub",
                  disabled: r.builtin || !r.onDisk || !!(drafts[r.id] && changed(drafts[r.id])),
                }),
                menuItem("theme.copy_code", { arg: r.id, icon: "share", disabled: !r.onDisk }),
                { kind: "separator" },
                menuItem("theme.delete", {
                  arg: r.id,
                  icon: "trash",
                  danger: true,
                  label: r.onDisk ? "Delete theme" : "Discard draft",
                  disabled: r.builtin,
                }),
              ]
              return (
                <ContextMenu items={actions()}>
                  <div class="group relative">
                    <ThemeCard
                      class="w-full"
                      actions={
                        <>
                          <IconButton
                            size="sm"
                            variant="ghost"
                            icon="edit"
                            label={r.builtin ? `Customize ${r.name}` : `Edit ${r.name}`}
                            title={r.builtin ? "Customize a copy" : "Edit theme"}
                            onClick={() => run("theme.edit", r.id)}
                          />
                          <Menu
                            size="sm"
                            variant="ghost"
                            icon="more"
                            label={`Actions for ${r.name}`}
                            items={actions()}
                          />
                        </>
                      }
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
                  </div>
                </ContextMenu>
              )
            }}
          </VirtualGrid>
        </Show>
      </section>
    </Page>
  )
}

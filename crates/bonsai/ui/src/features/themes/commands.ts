/**
 * The theme editor's commands. They act on the editor that is showing (`activeDraft`); with none
 * they say so. The layout registers them, since `theme.new` and `theme.edit` run from anywhere.
 */
import { createStore } from "solid-js/store"
import { handle, runBackend } from "~/lib/commands/registry"
import { commands, type ThemeView } from "~/lib/ipc/bindings"
import { DEFAULT_SEEDS, type Mode, parseTheme, SCHEMA, shownBase, type Theme } from "~/lib/theme"
import { type PromptOptions, showToast } from "~/lib/ui"
import {
  activeDraft,
  copyOf,
  drafts,
  dropDraft,
  edit,
  fixContrast,
  newId,
  plain,
  putDraft,
  redo,
  setInfo,
  startDraft,
  undo,
  update,
} from "./draft"

export const [themeUi, setThemeUi] = createStore({ importing: false })

export const editorPath = (id: string) => `/themes/${id}`

const takenIn = (all: ThemeView[]) => (id: string) => all.some((t) => t.id === id) || id in drafts

export const [saves, setSaves] = createStore<Record<string, { busy: boolean; error: string }>>({})

/** Saves a snapshot; edits made during the write stay dirty. */
export async function saveDraft(id: string) {
  if (saves[id]?.busy) return false
  const theme = plain(id)
  if (!theme) return false
  setSaves(id, { busy: true, error: "" })
  try {
    const ok = await runBackend("theme.save", JSON.stringify(theme))
    if (!ok) {
      setSaves(id, "error", "Theme could not be saved. Your changes are still here. Try again.")
      return false
    }
    update(id, (d) => ({ ...d, base: theme, onDisk: true, from: undefined }))
    showToast(`Saved ${theme.name}.`)
    return true
  } catch {
    setSaves(id, "error", "Theme could not be saved. Your changes are still here. Try again.")
    return false
  } finally {
    setSaves(id, "busy", false)
  }
}

/** The theme on screen now: the one for the base the mode shows. */
async function shownId(): Promise<string> {
  const app = await commands.appSettings()
  return shownBase(app.mode as Mode) === "dark" ? app.darkTheme : app.lightTheme
}

/** Opens a theme in the editor. A built-in one is copied first, since it can't change. */
export async function editTheme(id: string, visit: (path: string) => void) {
  if (id in drafts) return visit(editorPath(id))
  const all = await commands.themes()
  const view = all.find((t) => t.id === id)
  const theme = view && parseTheme(view.doc)
  if (!view || !theme) return void showToast("No theme with that id.")
  if (!view.builtin) return visit(editorPath(id))
  const copy = copyOf(theme, `${theme.name} copy`, takenIn(all))
  putDraft(startDraft(copy, { onDisk: false, from: theme.name, inherit: true }))
  visit(editorPath(copy.id))
}

export function registerThemeCommands(opts: {
  visit: (path: string) => void
  prompt: (o: PromptOptions) => Promise<string | null>
}) {
  /** Runs `f` on the draft in the editor, or says there is none. */
  const onActive = (f: (id: string) => unknown) => () => {
    const id = activeDraft()
    if (id && id in drafts) return void f(id)
    showToast("Open a theme in the editor first.")
  }

  return [
    handle("theme.import", (arg) => {
      if (arg?.trim()) return void runBackend("theme.import", arg)
      setThemeUi("importing", true)
    }),
    handle("theme.new", async () => {
      const name = (
        await opts.prompt({
          title: "Create theme",
          label: "Theme name",
          placeholder: "My theme",
          submit: "Create theme",
          maxLength: 64,
        })
      )?.trim()
      if (!name) return
      const [all, app] = await Promise.all([commands.themes(), commands.appSettings()])
      const base = shownBase(app.mode as Mode)
      const theme: Theme = {
        schema: SCHEMA,
        id: newId(name, takenIn(all)),
        name,
        author: "",
        version: "1.0.0",
        base,
        seeds: DEFAULT_SEEDS[base],
        tokens: {},
      }
      putDraft(startDraft(theme, { onDisk: false }))
      opts.visit(editorPath(theme.id))
    }),
    handle("theme.edit", async (arg) => editTheme(arg?.trim() || (await shownId()), opts.visit)),
    handle("theme.save", (arg) => {
      // A document from the command bar or another caller saves as is.
      if (arg?.trim()) return void runBackend("theme.save", arg)
      return onActive(saveDraft)()
    }),
    handle("theme.delete", async (arg) => {
      const id = arg?.trim() || activeDraft()
      if (id && saves[id]?.busy) return void showToast("Wait for this theme to finish saving.")
      if (id && drafts[id] && !drafts[id].onDisk) {
        dropDraft(id)
        showToast("Theme draft discarded.")
      } else {
        if (!(await runBackend("theme.delete", id ?? arg))) return
        if (id) dropDraft(id)
      }
      if (id === activeDraft()) opts.visit("/themes")
    }),
    handle("theme.rename", async (arg) => {
      const id = arg?.trim() || activeDraft()
      if (!id) return void showToast("Pick a theme to rename.")
      const all = await commands.themes()
      const view = all.find((t) => t.id === id)
      if (view?.builtin) return void showToast("Built-in themes can't be renamed. Duplicate one first.")
      const d = drafts[id]
      const now = d?.theme.name ?? view?.name
      if (now === undefined) return void showToast("No theme with that id.")
      const name = (await opts.prompt({ title: "Rename theme", label: "Name", initial: now, maxLength: 64 }))?.trim()
      if (!name || name === now) return
      if (!d) {
        const theme = view && parseTheme(view.doc)
        if (theme) void runBackend("theme.save", JSON.stringify({ ...theme, name }))
        return
      }
      update(id, (x) => setInfo(x, "name", name))
      // A saved theme is renamed on disk at once; other unsaved edits stay unsaved.
      if (d.onDisk) {
        const base: Theme = { ...(JSON.parse(JSON.stringify(d.base)) as Theme), name }
        if (await runBackend("theme.save", JSON.stringify(base))) update(id, (x) => ({ ...x, base }))
      }
    }),
    handle(
      "theme.revert",
      onActive((id) => update(id, (d) => edit(d, d.base))),
    ),
    handle(
      "theme.undo",
      onActive((id) => update(id, undo)),
    ),
    handle(
      "theme.redo",
      onActive((id) => update(id, redo)),
    ),
    handle(
      "theme.fix_contrast",
      onActive((id) => {
        const d = drafts[id]
        const r = fixContrast(d)
        if (r.fixed > 0) update(id, () => r.draft)
        if (r.failing > 0) showToast(`${r.failing} pairs still fail. Change the colors behind them.`)
        else if (r.fixed === 0) showToast("Everything passes already.")
      }),
    ),
  ]
}

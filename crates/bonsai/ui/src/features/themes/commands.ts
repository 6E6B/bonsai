/**
 * The theme editor's commands. They act on the editor that is showing (`activeDraft`); with none
 * they say so. The layout registers them, since `theme.new` and `theme.edit` run from anywhere.
 */
import { handle, runBackend } from "~/lib/commands/registry"
import { commands, type ThemeView } from "~/lib/ipc/bindings"
import { DEFAULT_SEEDS, type Mode, parseTheme, SCHEMA, shownBase, type Theme } from "~/lib/theme"
import { type PromptOptions, showToast } from "~/lib/ui"
import {
  activeDraft,
  copyOf,
  drafts,
  edit,
  fixContrast,
  inspecting,
  liveDraft,
  newId,
  plain,
  putDraft,
  redo,
  setInfo,
  setInspecting,
  setLiveDraft,
  startDraft,
  undo,
  update,
} from "./draft"

export const editorPath = (id: string) => `/themes/${id}`

const takenIn = (all: ThemeView[]) => (id: string) => all.some((t) => t.id === id) || id in drafts

/** Saves a draft. True when the backend took it. */
export async function saveDraft(id: string) {
  const theme = plain(id)
  if (!theme) return false
  const ok = await runBackend("theme.save", JSON.stringify(theme))
  if (ok) update(id, (d) => ({ ...d, base: theme, onDisk: true, from: undefined }))
  return ok
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
    handle("theme.new", async () => {
      const [all, app] = await Promise.all([commands.themes(), commands.appSettings()])
      const base = shownBase(app.mode as Mode)
      const theme: Theme = {
        schema: SCHEMA,
        id: newId("New theme", takenIn(all)),
        name: "New theme",
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
      "theme.inspect",
      onActive(() => setInspecting(!inspecting())),
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
    handle(
      "theme.live",
      onActive(async (id) => {
        if (liveDraft() === id) return setLiveDraft(undefined)
        setLiveDraft(id)
        const app = await commands.appSettings()
        const base = drafts[id]?.theme.base
        if (base && shownBase(app.mode as Mode) !== base) showToast(`Shows when the app is in ${base} mode.`)
      }),
    ),
  ]
}

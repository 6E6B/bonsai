import { afterEach, beforeEach, expect, it, vi } from "vitest"
import { handle } from "~/lib/commands/registry"
import { builtin } from "~/lib/theme/builtins"
import { registerThemeCommands, saveDraft, saves } from "./commands"
import { drafts, dropDraft, edit, putDraft, startDraft, unsaved, update } from "./draft"

const backend = vi.hoisted(() => vi.fn())
vi.mock("~/lib/commands/registry", () => ({ handle: vi.fn(), runBackend: backend }))
vi.mock("~/lib/ui", () => ({ showToast: vi.fn() }))

const theme = { ...builtin("dark"), id: "save-test", name: "Test theme" }
beforeEach(() => {
  backend.mockReset()
  putDraft(startDraft(theme, { onDisk: false }))
})
afterEach(() => dropDraft(theme.id))

it("shows saving state, rejects overlapping saves and keeps edits made during a save dirty", async () => {
  let finish!: (ok: boolean) => void
  backend.mockImplementation(
    () =>
      new Promise<boolean>((resolve) => {
        finish = resolve
      }),
  )
  const pending = saveDraft(theme.id)
  expect(saves[theme.id].busy).toBe(true)
  expect(await saveDraft(theme.id)).toBe(false)
  update(theme.id, (d) => edit(d, { ...d.theme, name: "Changed while saving" }))
  finish(true)
  expect(await pending).toBe(true)
  expect(saves[theme.id].busy).toBe(false)
  expect(drafts[theme.id].base.name).toBe("Test theme")
  expect(drafts[theme.id].theme.name).toBe("Changed while saving")
  expect(unsaved(drafts[theme.id])).toBe(true)
  expect(backend).toHaveBeenCalledTimes(1)
})

it("retains the draft on failure and clears the error after a successful retry", async () => {
  backend.mockRejectedValueOnce(new Error("transport failed")).mockResolvedValueOnce(true)
  expect(await saveDraft(theme.id)).toBe(false)
  expect(saves[theme.id].busy).toBe(false)
  expect(saves[theme.id].error).toContain("Try again")
  expect(drafts[theme.id].onDisk).toBe(false)
  expect(await saveDraft(theme.id)).toBe(true)
  expect(saves[theme.id].error).toBe("")
  expect(unsaved(drafts[theme.id])).toBe(false)
})

it("discards a new draft without trying to delete a nonexistent file", async () => {
  registerThemeCommands({ visit: vi.fn(), prompt: vi.fn() })
  const handler = vi.mocked(handle).mock.calls.find(([id]) => id === "theme.delete")?.[1]
  if (!handler) throw new Error("Missing theme.delete handler")
  await handler(theme.id, null)
  expect(drafts[theme.id]).toBeUndefined()
  expect(backend).not.toHaveBeenCalled()
})

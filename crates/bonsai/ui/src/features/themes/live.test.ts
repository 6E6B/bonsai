import { afterEach, expect, it } from "vitest"
import { applyAppearance, shownBackground } from "~/lib/theme/apply"
import { builtin } from "~/lib/theme/builtins"
import { BACKGROUND_DEFAULTS } from "~/lib/theme/theme"
import { dropDraft, putDraft, setActiveDraft, startDraft } from "./draft"

const id = "live-background-test"
afterEach(() => {
  setActiveDraft(undefined)
  dropDraft(id)
})

it("automatically previews an active editor draft and restores the saved theme when leaving", async () => {
  applyAppearance("dark", { dark: builtin("dark") })
  putDraft(
    startDraft(
      {
        ...builtin("light"),
        id,
        background: {
          ...BACKGROUND_DEFAULTS,
          url: "https://example.com/background.gif",
        },
      },
      { onDisk: false },
    ),
  )
  setActiveDraft(id)
  await Promise.resolve()
  expect(document.documentElement.dataset.theme).toBe("light")
  expect(shownBackground()?.file).toBe("https://example.com/background.gif")
  setActiveDraft(undefined)
  await Promise.resolve()
  expect(document.documentElement.dataset.theme).toBe("dark")
  expect(shownBackground()).toBeNull()
})

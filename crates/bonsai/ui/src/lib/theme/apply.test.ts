import { afterEach, describe, expect, it, vi } from "vitest"
import { applyAppearance, previewTheme, shownBackground } from "./apply"
import { builtin } from "./builtins"
import { BACKGROUND_DEFAULTS, readTheme } from "./theme"

afterEach(() => {
  previewTheme(undefined)
  applyAppearance("dark", { dark: builtin("dark") })
})

describe("live theme previews", () => {
  it("previews the draft base and background, then restores the saved appearance", () => {
    applyAppearance("dark", { dark: builtin("dark") })
    previewTheme({
      ...builtin("light"),
      background: { ...BACKGROUND_DEFAULTS, url: "https://example.com/a.png?size=2" },
    })
    expect(document.documentElement.dataset.theme).toBe("light")
    expect(shownBackground()?.file).toBe("https://example.com/a.png?size=2")
    previewTheme(undefined)
    expect(document.documentElement.dataset.theme).toBe("dark")
    expect(shownBackground()).toBeNull()
  })

  it("preserves portable background effects through reading and JSON export", () => {
    const input = { ...builtin("dark"), background: { url: "https://example.com/a.gif", blur: 12 } }
    const parsed = readTheme(input)
    expect(parsed.ok).toBe(true)
    if (!parsed.ok) return
    expect(parsed.theme.background).toEqual({ ...BACKGROUND_DEFAULTS, url: input.background.url, blur: 12 })
    expect(readTheme(JSON.parse(JSON.stringify(parsed.theme)))).toEqual(parsed)
  })

  it("refuses local, embedded and invalid backgrounds", () => {
    for (const background of [
      { url: "file:///tmp/a.png" },
      { url: "data:image/png;base64,abc" },
      { url: "http://example.com/a.png" },
      { url: "https://example.com/a.png", blur: 41 },
      { file: "local.png" },
    ])
      expect(readTheme({ ...builtin("dark"), background }).ok).toBe(false)
  })
})

it("does not rewrite root styles when only a theme name changes", () => {
  const theme = { ...builtin("dark"), background: { ...BACKGROUND_DEFAULTS, url: "https://example.com/a.gif" } }
  previewTheme(theme)
  const write = vi.spyOn(document.documentElement.style, "setProperty")
  previewTheme({ ...theme, name: "Renamed" })
  expect(write).not.toHaveBeenCalled()
  write.mockRestore()
})

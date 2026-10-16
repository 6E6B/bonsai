import { createMemo } from "solid-js"
import { run } from "~/lib/commands/registry"
import { FontPicker } from "~/lib/font-picker"
import { commands, type ThemeView } from "~/lib/ipc/bindings"
import { useLive } from "~/lib/store"
import { FONTS, type FontSeed, fontLabel, googleFamily, MONOS, type Mode, parseTheme, shownBase } from "~/lib/theme"
import { Button, SettingRow, SettingsGroup } from "~/lib/ui"
import { PrefRow } from "./prefs"
import { useAppearance, useAppSettings } from "./shared"

/** Theme, fonts and size. Saved on this computer for every account. */
export function AppearanceSection() {
  const app = useAppSettings()
  const themes = useLive(
    () => "themes",
    () => commands.themes(),
    (): ThemeView[] => [],
  )
  const themeName = (id: string) => themes.find((t) => t.id === id)?.name ?? id
  return (
    <>
      <SettingsGroup title="Theme" description="Saved on this computer for every account.">
        <PrefRow name="mode" />
        <SettingRow
          setting="themes"
          label="Themes"
          description={`Dark: ${themeName(app.darkTheme)}. Light: ${themeName(app.lightTheme)}. Pick, edit and make themes in their own tab.`}
        >
          <Button size="sm" icon="palette" onClick={() => run("nav.themes")}>
            Open Themes
          </Button>
        </SettingRow>
      </SettingsGroup>
      <ThemeFonts themes={themes} />
      <SettingsGroup title="Size">
        <PrefRow name="density" />
        <SettingRow setting="zoom" label="Zoom" description="Scales the whole window.">
          <Button size="sm" onClick={() => run("view.zoom_out")}>
            Smaller
          </Button>
          <Button size="sm" onClick={() => run("view.zoom_reset")}>
            Reset
          </Button>
          <Button size="sm" onClick={() => run("view.zoom_in")}>
            Larger
          </Button>
        </SettingRow>
      </SettingsGroup>
    </>
  )
}

/** The fonts of the theme on screen. Picking one changes that theme; a built-in one is copied
 * first. Google fonts download in the background and show once they are on disk. */
function ThemeFonts(props: { themes: ThemeView[] }) {
  // Mode and documents come in one snapshot, so the theme on screen is never out of step.
  const appearance = useAppearance()
  const shown = createMemo(() => {
    const base = shownBase(appearance.mode as Mode)
    return parseTheme(base === "dark" ? appearance.dark : appearance.light)
  })
  const builtin = () => props.themes.find((t) => t.id === shown()?.id)?.builtin ?? false
  const status = (seed: FontSeed<string> | undefined) => {
    const family = seed && googleFamily(seed)
    const f = family && appearance.fonts.find((x) => x.family === family)
    if (!f) return undefined
    if (f.error) return f.error
    if (f.files.length === 0) return `Downloading ${f.family}…`
    return undefined
  }
  const description = (seed: FontSeed<string> | undefined) => {
    const t = shown()
    if (!t) return undefined
    return (
      status(seed) ??
      (builtin() ? `${t.name} is built in, so picking a font makes a copy of it.` : `Saved in ${t.name}.`)
    )
  }
  return (
    <SettingsGroup title="Fonts">
      <SettingRow label="Font" description={description(shown()?.seeds.font)}>
        <FontPicker
          label="Font"
          mono={false}
          value={(() => {
            const seed = shown()?.seeds.font
            return seed ? fontLabel(seed, FONTS) : ""
          })()}
          onChange={(v) => run("theme.font", v)}
        />
      </SettingRow>
      <SettingRow label="Code font" description={description(shown()?.seeds.mono)}>
        <FontPicker
          label="Code font"
          mono
          value={(() => {
            const seed = shown()?.seeds.mono
            return seed ? fontLabel(seed, MONOS) : ""
          })()}
          onChange={(v) => run("theme.mono_font", v)}
        />
      </SettingRow>
    </SettingsGroup>
  )
}

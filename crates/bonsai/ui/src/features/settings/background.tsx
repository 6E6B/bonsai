import { For, Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import { type BackdropLevel, EFFECTS, previewBackdrop, withDraft } from "~/lib/theme"
import { backgroundUrl } from "~/lib/thumb"
import { Button, SettingRow, SettingsGroup, Slider, usePrompt } from "~/lib/ui"
import { useAppearance } from "./shared"

/** A picture or GIF behind every page, from an HTTPS link. The page and cards turn
 * see-through over it; the window chrome and menus stay as the theme draws them. */
export function BackgroundSection() {
  const appearance = useAppearance()
  const prompt = usePrompt()
  const fromLink = async () => {
    const url = await prompt({ title: "Background from a link", label: "Image link (https)", submit: "Use" })
    if (url) void run("view.background", url)
  }
  // Dragging previews at once; the value is saved when the drag ends.
  const level = (key: BackdropLevel, label: string, max: number, step: number, unit: string) => (
    <Show when={appearance.background}>
      {(b) => (
        <Slider
          label={label}
          min={0}
          max={max}
          step={step}
          value={withDraft(b())[key]}
          format={(v) => `${v}${unit}`}
          onChange={(v) => previewBackdrop(key, v)}
          onChangeEnd={(v) =>
            void run("view.background_level", `${key} ${v}`).then((ok) => {
              if (!ok) previewBackdrop(key, undefined)
            })
          }
        />
      )}
    </Show>
  )
  return (
    <>
      <SettingsGroup title="Picture" description="Saved with the current theme and included when shared.">
        <SettingRow label="Picture" description="PNG, JPEG, GIF or WebP, up to 32 MB. Shows behind every page.">
          <Show when={appearance.background && backgroundUrl(appearance.background.file)}>
            {(src) => <img src={src()} alt="" class="h-8 w-14 rounded-sm border border-line object-cover" />}
          </Show>
          <Button size="sm" icon="link" onClick={fromLink}>
            From link
          </Button>
          <Show when={appearance.background}>
            <Button size="sm" variant="ghost" onClick={() => run("view.background_clear")}>
              Remove
            </Button>
          </Show>
        </SettingRow>
        <Show when={appearance.background}>
          <div class="grid grid-cols-2 gap-x-6 gap-y-4 py-4">
            {level("image", "Picture strength", 100, 5, "%")}
            {level("panels", "Page opacity", 100, 5, "%")}
          </div>
        </Show>
      </SettingsGroup>
      <Show when={appearance.background}>
        <SettingsGroup
          title="Effects"
          actions={
            <Button size="sm" variant="ghost" onClick={() => run("view.background_effects_reset")}>
              Reset effects
            </Button>
          }
        >
          <div class="grid grid-cols-2 gap-x-6 gap-y-4 py-4">
            <For each={EFFECTS}>{(e) => level(e.key, e.label, e.max, e.step, e.unit)}</For>
          </div>
        </SettingsGroup>
      </Show>
    </>
  )
}

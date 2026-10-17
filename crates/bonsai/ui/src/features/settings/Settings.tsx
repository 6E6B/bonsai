import { useNavigate, useParams } from "@solidjs/router"
import { type Component, createEffect, createMemo, createSignal, For, on, onCleanup, Show } from "solid-js"
import { Dynamic } from "solid-js/web"
import { type SettingsSectionId, settingsGroups, settingsSection, settingsSections } from "~/lib/sections"
import { shell } from "~/lib/store"
import { EmptyState, NavItem, Pane, SectionHeader, Split, TextField } from "~/lib/ui"
import { ProfileSection, SecuritySection } from "./account"
import { AccountsSection } from "./accounts"
import { AppearanceSection } from "./appearance"
import { BackgroundSection } from "./background"
import { DataSection } from "./data"
import { DesktopSection } from "./desktop"
import { DeveloperSection } from "./developer"
import { GeneralSection } from "./general"
import { LayoutSection } from "./layout"
import { NotificationsSection } from "./notifications"
import { PluginsSection } from "./plugins"
import { PrefsProvider, usePrefsSource } from "./prefs"
import { PrivacySection } from "./privacy-section"
import { entryPath, searchSettings } from "./search"
import { ServersSection } from "./servers"
import { ShortcutsSection } from "./shortcuts"

const pages: Record<SettingsSectionId, Component> = {
  profile: ProfileSection,
  security: SecuritySection,
  privacy: PrivacySection,
  servers: ServersSection,
  general: GeneralSection,
  appearance: AppearanceSection,
  background: BackgroundSection,
  layout: LayoutSection,
  notifications: NotificationsSection,
  shortcuts: ShortcutsSection,
  desktop: DesktopSection,
  accounts: AccountsSection,
  plugins: PluginsSection,
  data: DataSection,
  developer: DeveloperSection,
}

/** Scrolls to the row a deep link names and flashes it. Tries for a few frames while the section
 * and the settings list render. */
function useHighlight(setting: () => string | undefined) {
  let timer: ReturnType<typeof setTimeout> | undefined
  let frame = 0
  const find = (key: string, tries: number) => {
    const row = document.querySelector<HTMLElement>(`[data-setting="${CSS.escape(key)}"]`)
    if (!row) {
      if (tries > 0) frame = requestAnimationFrame(() => find(key, tries - 1))
      return
    }
    row.scrollIntoView({ block: "center" })
    row.setAttribute("data-highlight", "")
    clearTimeout(timer)
    timer = setTimeout(() => row.removeAttribute("data-highlight"), 1200)
  }
  createEffect(
    on(setting, (key) => {
      cancelAnimationFrame(frame)
      if (key) frame = requestAnimationFrame(() => find(key, 30))
    }),
  )
  onCleanup(() => {
    cancelAnimationFrame(frame)
    clearTimeout(timer)
  })
}

/** Settings, one section at a time, split into what is saved on Roblox for the active account and
 * what is saved on this computer. `nav.settings` takes a section name, or `section/setting` to
 * scroll to one row, so the command bar can open any of them directly. */
export default function Settings() {
  const params = useParams()
  const navigate = useNavigate()
  const prefs = usePrefsSource()
  const section = createMemo(() => settingsSection(params.section))
  const current = () => settingsSections.find((s) => s.value === section())
  const active = () => shell.accounts.find((a) => a.id === shell.active)
  const [query, setQuery] = createSignal("")
  const results = createMemo(() => searchSettings(prefs, query()))
  useHighlight(() => params.setting)

  return (
    <Split>
      <Pane side as="nav" class="w-64 gap-0.5" aria-label="Settings sections">
        <SectionHeader title="Settings" size="page" class="mb-2" />
        <TextField
          size="sm"
          icon="search"
          class="mb-2"
          aria-label="Search settings"
          placeholder="Search settings"
          value={query()}
          onChange={setQuery}
          onKeyDown={(e) => {
            if (e.key === "Escape") setQuery("")
            const first = results()[0]
            if (e.key === "Enter" && first) navigate(entryPath(first))
          }}
        />
        <Show when={query().trim()}>
          <For each={results()} fallback={<EmptyState title="No settings match" />}>
            {(r) => (
              <NavItem
                label={r.title}
                icon={settingsSections.find((s) => s.value === r.section)?.icon}
                active={section() === r.section && params.setting === r.setting}
                onClick={() => navigate(entryPath(r))}
              />
            )}
          </For>
        </Show>
        <For each={query().trim() ? [] : settingsGroups}>
          {(g) => (
            <>
              <SectionHeader
                title={g.label}
                size="group"
                meta={g.value === "account" && active() ? `@${active()?.name}` : undefined}
                class="mt-3 px-2.5 first-of-type:mt-0"
              />
              <For each={settingsSections.filter((s) => s.group === g.value)}>
                {(s) => (
                  <NavItem
                    label={s.label}
                    icon={s.icon}
                    active={section() === s.value}
                    onClick={() => navigate(`/settings/${s.value}`)}
                  />
                )}
              </For>
            </>
          )}
        </For>
      </Pane>
      <Pane>
        <div class="flex max-w-2xl flex-col gap-6">
          <SectionHeader title={current()?.label ?? "Settings"} size="page" />
          <PrefsProvider prefs={prefs}>
            <Dynamic component={pages[section()]} />
          </PrefsProvider>
        </div>
      </Pane>
    </Split>
  )
}

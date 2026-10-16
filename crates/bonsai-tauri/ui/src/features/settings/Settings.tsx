import { useNavigate, useParams } from "@solidjs/router"
import { type Component, createMemo, For } from "solid-js"
import { Dynamic } from "solid-js/web"
import { type SettingsSectionId, settingsGroups, settingsSection, settingsSections } from "~/lib/sections"
import { shell } from "~/lib/store"
import { NavItem, Pane, SectionHeader, Split } from "~/lib/ui"
import { ProfileSection, SecuritySection } from "./account"
import { AccountsSection } from "./accounts"
import { AppearanceSection } from "./appearance"
import { BackgroundSection } from "./background"
import { DataSection } from "./data"
import { DeveloperSection } from "./developer"
import { GeneralSection } from "./general"
import { LayoutSection } from "./layout"
import { NotificationsSection } from "./notifications"
import { PrivacySection } from "./privacy-section"
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
  accounts: AccountsSection,
  data: DataSection,
  developer: DeveloperSection,
}

/** Settings, one section at a time, split into what is saved on Roblox for the active account and
 * what is saved on this computer. `nav.settings` takes a section name, so the command bar can open
 * any of them directly. */
export default function Settings() {
  const params = useParams()
  const navigate = useNavigate()
  const section = createMemo(() => settingsSection(params.section))
  const current = () => settingsSections.find((s) => s.value === section())
  const active = () => shell.accounts.find((a) => a.id === shell.active)

  return (
    <Split>
      <Pane side as="nav" class="w-64 gap-0.5" aria-label="Settings sections">
        <SectionHeader title="Settings" size="page" class="mb-2" />
        <For each={settingsGroups}>
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
          <Dynamic component={pages[section()]} />
        </div>
      </Pane>
    </Split>
  )
}

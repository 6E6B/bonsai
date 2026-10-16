import { run } from "~/lib/commands/registry"
import { SettingSwitch, SettingsGroup } from "~/lib/ui"
import { useAppSettings } from "./shared"

/** Which events raise an alert, in the app and on the desktop while it is in the background. */
export function NotificationsSection() {
  const app = useAppSettings()
  const toggle = (kind: string) => () => run("app.notify", kind)
  return (
    <>
      <SettingsGroup title="People" description="Saved on this computer for every account.">
        <SettingSwitch
          label="Friends joining games"
          description="A desktop notification when a friend joins an experience."
          checked={app.notify.friendJoins}
          onChange={toggle("friend_joins")}
        />
        <SettingSwitch
          label="Friend requests"
          checked={app.notify.friendRequests}
          onChange={toggle("friend_requests")}
        />
        <SettingSwitch label="Chat messages" checked={app.notify.chats} onChange={toggle("chats")} />
      </SettingsGroup>
      <SettingsGroup title="Trading and sales">
        <SettingSwitch label="Trade offers" checked={app.notify.trades} onChange={toggle("trades")} />
        <SettingSwitch
          label="Sales"
          description="When someone buys something you sell. Checked every few minutes."
          checked={app.notify.sales}
          onChange={toggle("sales")}
        />
      </SettingsGroup>
      <SettingsGroup title="Experiences">
        <SettingSwitch
          label="Experience updates"
          description="From experiences you turned Notify on for."
          checked={app.notify.experienceUpdates}
          onChange={toggle("experience_updates")}
        />
      </SettingsGroup>
    </>
  )
}

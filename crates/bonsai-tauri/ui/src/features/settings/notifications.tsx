import { SettingsGroup } from "~/lib/ui"
import { PrefRow } from "./prefs"

/** Which events raise an alert, in the app and on the desktop while it is in the background. */
export function NotificationsSection() {
  return (
    <>
      <SettingsGroup title="People" description="Saved on this computer for every account.">
        <PrefRow name="notify.friend_joins" />
        <PrefRow name="notify.friend_requests" />
        <PrefRow name="notify.chats" />
      </SettingsGroup>
      <SettingsGroup title="Trading and sales">
        <PrefRow name="notify.trades" />
        <PrefRow name="notify.sales" />
      </SettingsGroup>
      <SettingsGroup title="Experiences">
        <PrefRow name="notify.experience_updates" />
      </SettingsGroup>
    </>
  )
}

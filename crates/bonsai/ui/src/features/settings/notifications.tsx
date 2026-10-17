import { Show } from "solid-js"
import { SettingsGroup } from "~/lib/ui"
import { PrefRow, usePref } from "./prefs"

/** Which events raise a desktop notification, and when they stay silent. */
export function NotificationsSection() {
  const quiet = usePref("quiet_hours")
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
      <SettingsGroup title="Plugins">
        <PrefRow name="notify.plugins" />
      </SettingsGroup>
      <SettingsGroup title="When">
        <PrefRow name="quiet_hours" />
        <Show when={quiet()?.value === "on"}>
          <PrefRow name="quiet_start" />
          <PrefRow name="quiet_end" />
        </Show>
        <PrefRow name="mute_in_game" />
        <PrefRow name="notify_in_front" />
        <PrefRow name="notify_sound" />
      </SettingsGroup>
    </>
  )
}

import { Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import { type AppearanceView, type AppSettings, commands } from "~/lib/ipc/bindings"
import { useLive } from "~/lib/store"
import { Badge, Button } from "~/lib/ui"

const initialApp = (): AppSettings => ({
  mode: "system",
  darkTheme: "dark",
  lightTheme: "light",
  density: "comfortable",
  playerCommand: "",
  formatLocale: "",
  notify: {
    friendJoins: true,
    trades: true,
    chats: true,
    friendRequests: false,
    experienceUpdates: false,
    sales: true,
    plugins: true,
  },
  cacheData: null,
  cacheImages: null,
})

/** The app's local settings, live. */
export const useAppSettings = () =>
  useLive(
    () => "app",
    () => commands.appSettings(),
    initialApp,
  )

/** Mode, the two theme documents, fonts and background, live and in one snapshot. */
export const useAppearance = () =>
  useLive(
    () => "appearance",
    () => commands.appearance(),
    (): AppearanceView => ({ mode: "system", dark: "", light: "", fonts: [], background: null }),
  )

/** Opens the active account's settings on roblox.com. */
export const WebButton = (props: { label?: string }) => (
  <Button size="sm" icon="external" onClick={() => run("entity.open_website", "section:settings")}>
    {props.label ?? "Manage on website"}
  </Button>
)

/** "Verified" or "Not verified" next to a value Roblox has on file. */
export function Verified(props: { ok: boolean }) {
  return (
    <Show when={props.ok} fallback={<Badge>Not verified</Badge>}>
      <Badge variant="success">Verified</Badge>
    </Show>
  )
}

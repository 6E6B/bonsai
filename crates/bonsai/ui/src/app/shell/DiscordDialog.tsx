import { isTauri } from "@tauri-apps/api/core"
import { createSignal, Show } from "solid-js"
import { runBackend } from "~/lib/commands/registry"
import { Button } from "~/lib/ui"
import inviteCode from "../../../../../../discord-invite.txt?raw"

/** The bundled invite code is shared with the native Discord launcher. */
const available = /^[A-Za-z0-9_-]+$/.test(inviteCode.trim())

export function DiscordDialog() {
  const [opening, setOpening] = createSignal(false)
  const open = async (browser: boolean) => {
    if (opening() || !available || !isTauri()) return
    setOpening(true)
    try {
      await runBackend(browser ? "help.discord_browser" : "help.discord_open")
    } finally {
      setOpening(false)
    }
  }
  return (
    <div class="flex flex-col gap-4 p-4">
      <p class="text-sm text-fg-muted">Get help, share feedback, and follow updates.</p>
      <Show when={!available}>
        <p class="text-xs text-fg-muted">Invite coming soon.</p>
      </Show>
      <div class="flex items-center justify-end gap-2">
        <Button variant="ghost" size="sm" disabled={!available || opening()} onClick={() => void open(true)}>
          Open in browser
        </Button>
        <Button
          variant="primary"
          icon="discord"
          disabled={!available}
          loading={opening()}
          onClick={() => void open(false)}
        >
          Join Discord
        </Button>
      </div>
    </div>
  )
}

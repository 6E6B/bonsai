import { isTauri } from "@tauri-apps/api/core"
import { createSignal, Show } from "solid-js"
import { runBackend } from "~/lib/commands/registry"
import { Button } from "~/lib/ui"
import inviteCode from "../../../../../../discord-invite.txt?raw"

/** The bundled invite code is shared with the browser and clipboard actions. */
const available = /^[A-Za-z0-9_-]+$/.test(inviteCode.trim())
type InviteAction = "help.discord_browser" | "help.discord_copy"

export function DiscordDialog() {
  const [pending, setPending] = createSignal<InviteAction | null>(null)
  const act = async (id: InviteAction) => {
    if (pending() || !available || !isTauri()) return
    setPending(id)
    try {
      await runBackend(id)
    } finally {
      setPending(null)
    }
  }
  return (
    <div class="flex flex-col gap-4 p-4">
      <p class="text-sm text-fg-muted">Get help, share feedback, and follow updates.</p>
      <Show when={!available}>
        <p class="text-xs text-fg-muted">Invite coming soon.</p>
      </Show>
      <div class="flex items-center justify-end gap-2">
        <Button
          variant="ghost"
          icon="copy"
          disabled={!available || pending() !== null}
          loading={pending() === "help.discord_copy"}
          onClick={() => void act("help.discord_copy")}
        >
          Copy invite
        </Button>
        <Button
          variant="primary"
          icon="external"
          disabled={!available || pending() !== null}
          loading={pending() === "help.discord_browser"}
          onClick={() => void act("help.discord_browser")}
        >
          Open in browser
        </Button>
      </div>
    </div>
  )
}

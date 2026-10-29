import { isTauri } from "@tauri-apps/api/core"
import { onCleanup, onMount } from "solid-js"
import { createStore, reconcile } from "solid-js/store"
import { commands, type UpdateStatus } from "~/lib/ipc/bindings"

export const [updates, setUpdates] = createStore<UpdateStatus>({
  phase: "unavailable",
  currentVersion: "",
  version: null,
  notes: null,
  downloaded: 0,
  total: null,
  checkedAt: null,
  error: null,
  unavailableReason: "Updates are available in the desktop app.",
})

export function updateBusy(phase: UpdateStatus["phase"]) {
  return phase === "checking" || phase === "downloading" || phase === "verifying" || phase === "installing"
}

export function updateMessage(s: UpdateStatus): string {
  switch (s.phase) {
    case "unavailable":
      return s.unavailableReason ?? "Updates are unavailable in this build."
    case "checking":
      return "Checking for updates…"
    case "available":
      return `Bonsai ${s.version} is available.`
    case "downloading":
      return `Downloading Bonsai ${s.version}… ${Math.round(s.downloaded / 1048576)} MB`
    case "verifying":
      return "Verifying the update signature…"
    case "ready":
      return `Bonsai ${s.version} is ready to install.`
    case "installing":
      return "Installing update. Bonsai will restart…"
    default:
      if (s.error) return "Check failed. Try again when you’re connected."
      return s.checkedAt ? "You’re up to date." : "Check for a newer version of Bonsai."
  }
}

/** One shell-owned subscription. Serial polling survives transient IPC failures and unmounts. */
export function useUpdates() {
  let stopped = false
  let timer: ReturnType<typeof setTimeout> | undefined
  const refresh = async () => {
    try {
      const next = await commands.updateStatus()
      if (!stopped) setUpdates(reconcile(next))
    } catch {
      // The vault can lock during a poll. Retry after unlock without resetting a pending update.
    }
    if (!stopped) timer = setTimeout(refresh, updateBusy(updates.phase) ? 500 : 2000)
  }
  onMount(() => {
    if (isTauri()) void refresh()
  })
  onCleanup(() => {
    stopped = true
    clearTimeout(timer)
  })
}

import { isTauri } from "@tauri-apps/api/core"
import { createStore, reconcile } from "solid-js/store"
import { commands, type VaultView } from "~/lib/ipc/bindings"

const [vault, setVault] = createStore<VaultView & { ready: boolean }>({
  status: "locked",
  locked: true,
  error: null,
  retrySeconds: 0,
  systemLockWarning: null,
  ready: !isTauri(),
})

export { vault }
export async function refreshVault() {
  if (!isTauri()) {
    setVault({ status: "disabled", locked: false, ready: true })
    return
  }
  try {
    setVault(reconcile({ ...(await commands.vaultStatus()), ready: true }))
  } catch {
    setVault({
      status: "unavailable",
      locked: true,
      ready: true,
      error: "Cannot read vault status. Restart Bonsai to try again.",
    })
  }
}

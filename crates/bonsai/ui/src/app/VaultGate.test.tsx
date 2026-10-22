import { cleanup, render, waitFor } from "@solidjs/testing-library"
import { afterEach, expect, it, vi } from "vitest"
import { refreshVault } from "~/lib/vault"
import { VaultGate } from "./VaultGate"

const { status } = vi.hoisted(() => ({ status: vi.fn() }))
vi.mock("@tauri-apps/api/core", () => ({ isTauri: () => true }))
vi.mock("~/lib/commands/registry", () => ({ handle: () => () => {}, run: vi.fn() }))
vi.mock("~/lib/ipc/bindings", () => ({
  commands: { vaultStatus: status, vaultActivity: vi.fn() },
  events: { storeChanged: { listen: () => Promise.resolve(() => {}) } },
}))
vi.mock("~/lib/ui", () => ({
  Button: () => <button type="button">Check again</button>,
  useDialog: () => ({ show: vi.fn(), close: vi.fn() }),
}))
vi.mock("./shell/TitleBar", () => ({ TitleBar: () => <header>Bonsai</header> }))
vi.mock("./shell/VaultForm", () => ({ VaultForm: () => <form>Unlock</form> }))
afterEach(cleanup)

it("mounts private screens only after backend admission opens", async () => {
  status.mockResolvedValue({ status: "locked", locked: true, error: null, retrySeconds: 0 })
  await refreshVault()
  const mounted = vi.fn()
  function PrivateScreen() {
    mounted()
    return <div>Private account data</div>
  }
  const view = render(() => (
    <VaultGate>
      <PrivateScreen />
    </VaultGate>
  ))
  expect(mounted).not.toHaveBeenCalled()
  // Unwrapped keys alone do not admit access during a storage transition.
  status.mockResolvedValue({ status: "unlocked", locked: true, error: null, retrySeconds: 0 })
  await refreshVault()
  expect(mounted).not.toHaveBeenCalled()
  status.mockResolvedValue({ status: "unlocked", locked: false, error: null, retrySeconds: 0 })
  await refreshVault()
  await waitFor(() => expect(view.queryByText("Private account data")).not.toBeNull())
  expect(mounted).toHaveBeenCalledOnce()
})

it("unmounts private screens when backend status cannot be read", async () => {
  status.mockResolvedValue({ status: "disabled", locked: false, error: null, retrySeconds: 0 })
  await refreshVault()
  const view = render(() => (
    <VaultGate>
      <div>Private account data</div>
    </VaultGate>
  ))
  expect(view.queryByText("Private account data")).not.toBeNull()
  status.mockRejectedValue(new Error("IPC unavailable"))
  await refreshVault()
  await waitFor(() => expect(view.queryByText("Private account data")).toBeNull())
  expect(view.queryByText("Bonsai is locked")).not.toBeNull()
})

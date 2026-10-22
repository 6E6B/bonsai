import { createSignal, onCleanup, Show } from "solid-js"
import { commands } from "~/lib/ipc/bindings"
import { Button, TextField } from "~/lib/ui"
import { refreshVault, vault } from "~/lib/vault"
export type VaultAction = "enable" | "unlock" | "change" | "disable" | "forget"
/** PINs cross dedicated IPC once, and never enter command arguments or history. */
export function VaultForm(props: { action: VaultAction; onDone?: () => void }) {
  const [current, setCurrent] = createSignal("")
  const [next, setNext] = createSignal("")
  const [confirm, setConfirm] = createSignal("")
  const [busy, setBusy] = createSignal(false)
  const [error, setError] = createSignal("")
  const changes = () => props.action === "enable" || props.action === "change"
  const label = () => {
    switch (props.action) {
      case "enable":
        return "Enable protection"
      case "change":
        return "Change PIN"
      case "disable":
        return "Turn off protection"
      case "forget":
        return "Remove all local accounts"
      default:
        return "Unlock"
    }
  }
  onCleanup(() => {
    setCurrent("")
    setNext("")
    setConfirm("")
  })
  const submit = async (event: SubmitEvent) => {
    event.preventDefault()
    if (busy()) return
    if (changes() && next() !== confirm()) return setError("The new PINs do not match.")
    setError("")
    setBusy(true)
    try {
      const result = await commands.vaultSubmit(props.action, current(), next())
      setCurrent("")
      setNext("")
      setConfirm("")
      await refreshVault()
      if (result.status === "error") setError(result.error)
      else props.onDone?.()
    } catch {
      setError("Vault operation failed. Restart Bonsai and check the vault status.")
      await refreshVault()
    } finally {
      setBusy(false)
    }
  }
  return (
    <form class="flex flex-col gap-4 p-4" onSubmit={(e) => void submit(e)}>
      <Show when={vault.systemLockWarning}>
        <p role="status" class="text-sm text-warning">
          {vault.systemLockWarning}
        </p>
      </Show>
      <Show when={props.action === "enable"}>
        <p class="text-sm text-fg-muted">
          One PIN protects every account here. Use 8+ digits or a 12+ character passphrase. Bonsai locks on startup,
          system lock or sleep, and after 5 idle minutes.
        </p>
        <p class="text-sm text-fg-muted">
          There is no recovery. If you forget it, you must sign in to your accounts again.
        </p>
      </Show>
      <Show when={props.action === "disable"}>
        <p class="text-sm text-fg-muted">
          Saved logins will return to your OS keychain, and private cached data will no longer require your Bonsai PIN.
        </p>
      </Show>
      <Show when={props.action === "forget"}>
        <p class="text-sm text-fg-muted">
          This permanently removes every saved login and private cached record from this Bonsai profile. It cannot
          recover or unlock the accounts. You will need to sign in again.
        </p>
        <TextField
          label="Type RESET to continue"
          autocomplete="off"
          autofocus
          value={next()}
          onChange={setNext}
          disabled={busy()}
        />
      </Show>
      <Show when={props.action !== "enable" && props.action !== "forget"}>
        <TextField
          label={props.action === "unlock" ? "PIN or passphrase" : "Current PIN or passphrase"}
          type="password"
          autocomplete="off"
          autofocus
          maxLength={256}
          value={current()}
          onChange={setCurrent}
          disabled={busy()}
        />
      </Show>
      <Show when={changes()}>
        <TextField
          label="New PIN or passphrase"
          type="password"
          autocomplete="new-password"
          autofocus={props.action === "enable"}
          maxLength={256}
          value={next()}
          onChange={setNext}
          disabled={busy()}
        />
        <TextField
          label="Confirm PIN or passphrase"
          type="password"
          autocomplete="new-password"
          maxLength={256}
          value={confirm()}
          onChange={setConfirm}
          disabled={busy()}
        />
      </Show>
      <Show when={error()}>
        <p role="alert" class="text-sm text-danger">
          {error()}
        </p>
      </Show>
      <Show when={vault.retrySeconds > 0 && props.action === "unlock"}>
        <p role="status" class="text-sm text-fg-muted">
          Try again in {vault.retrySeconds} seconds.
        </p>
      </Show>
      <div class="flex justify-end gap-2">
        <Show when={props.onDone}>
          <Button type="button" variant="ghost" disabled={busy()} onClick={() => props.onDone?.()}>
            Cancel
          </Button>
        </Show>
        <Button
          type="submit"
          variant={props.action === "disable" || props.action === "forget" ? "danger" : "primary"}
          loading={busy()}
          disabled={
            (props.action === "forget" ? next() !== "RESET" : changes() ? !next() || !confirm() : !current()) ||
            (props.action === "unlock" && vault.retrySeconds > 0)
          }
        >
          {label()}
        </Button>
      </div>
    </form>
  )
}

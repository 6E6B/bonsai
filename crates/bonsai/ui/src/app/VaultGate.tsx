import { isTauri } from "@tauri-apps/api/core"
import { onCleanup, onMount, type ParentProps, Show } from "solid-js"
import { handle, run } from "~/lib/commands/registry"
import { commands, events } from "~/lib/ipc/bindings"
import { Button, useDialog } from "~/lib/ui"
import { refreshVault, vault } from "~/lib/vault"
import { TitleBar } from "./shell/TitleBar"
import { VaultForm } from "./shell/VaultForm"
/** Private mirrors and account commands mount only after the backend admits access. */
export function VaultGate(props: ParentProps) {
  const dialog = useDialog()
  onMount(() => {
    const unregister = handle("vault.forget", () =>
      dialog.show(() => <VaultForm action="forget" onDone={() => dialog.close()} />, {
        title: "Reset local accounts",
        size: "sm",
      }),
    )
    onCleanup(unregister)
    void refreshVault()
    if (!isTauri()) return
    const unlisten = events.storeChanged.listen(() => void refreshVault())
    const timer = setInterval(() => {
      if (vault.locked || (vault.status !== "disabled" && vault.status !== "unlocked")) void refreshVault()
    }, 1000)
    let last = 0
    const activity = (event: Event) => {
      if (!event.isTrusted || Date.now() - last < 1000) return
      last = Date.now()
      void commands.vaultActivity()
    }
    window.addEventListener("pointerdown", activity, true)
    window.addEventListener("keydown", activity, true)
    window.addEventListener("wheel", activity, true)
    onCleanup(() => {
      clearInterval(timer)
      void unlisten.then((off) => off())
      window.removeEventListener("pointerdown", activity, true)
      window.removeEventListener("keydown", activity, true)
      window.removeEventListener("wheel", activity, true)
    })
  })
  return (
    <Show
      when={vault.ready && !vault.locked && (vault.status === "disabled" || vault.status === "unlocked")}
      fallback={
        <div class="flex h-full flex-col bg-deep text-fg">
          <TitleBar />
          <main class="flex flex-1 items-center justify-center p-6">
            <section class="w-full max-w-md rounded-md border border-line bg-surface">
              <h1 class="px-4 pt-4 text-lg font-semibold">Bonsai is locked</h1>
              <Show when={vault.ready} fallback={<p class="p-4 text-sm text-fg-muted">Checking account protection…</p>}>
                <p class="px-4 pt-2 text-sm text-fg-muted">Unlock once to access all your accounts.</p>
                <Show when={vault.error}>
                  <p role="alert" class="p-4 text-sm text-danger">
                    {vault.error}
                  </p>
                </Show>
                <VaultForm action="unlock" />
                <div class="px-4 pb-4">
                  <Button size="sm" variant="ghost" onClick={() => void refreshVault()}>
                    Check again
                  </Button>
                  <Button size="sm" variant="ghost" onClick={() => void run("vault.forget")}>
                    Forgot PIN?
                  </Button>
                </div>
              </Show>
            </section>
          </main>
        </div>
      }
    >
      {props.children}
    </Show>
  )
}

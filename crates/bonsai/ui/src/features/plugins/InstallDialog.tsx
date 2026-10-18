import { createEffect, createSignal, For, on, Show } from "solid-js"
import type { Asks, Id } from "~/lib/ipc/bindings"
import { commands } from "~/lib/ipc/bindings"
import { byTier, tiers } from "~/lib/plugins"
import { shell } from "~/lib/store"
import { Alert, Button, Checkbox, Dialog, Fact, Facts } from "~/lib/ui"

/**
 * A plugin folder or `.bonsaiplugin` package before it is installed or loaded: its content hash,
 * what it asks for, what it won't get, and which accounts it may use. Opens by itself from
 * `shell.pluginInstall`, which `plugin.install`, `plugin.load_unpacked` and `plugin.review` set.
 */
export function InstallDialog() {
  const pending = () => shell.pluginInstall
  const [accounts, setAccounts] = createSignal<Id[]>([])
  const [busy, setBusy] = createSignal(false)
  const [error, setError] = createSignal("")
  createEffect(
    on(pending, (p) => {
      setAccounts(p?.accounts ?? [])
      setBusy(false)
      setError("")
    }),
  )
  const toggle = (id: Id, on: boolean) =>
    setAccounts((a) => (on ? [...a.filter((x) => x !== id), id] : a.filter((x) => x !== id)))
  const install = async () => {
    if (busy()) return
    setBusy(true)
    const r = await commands.pluginInstallFinish(accounts())
    if (r.status === "error") {
      setBusy(false)
      setError(r.error)
    }
  }
  const title = () => {
    const p = pending()
    if (!p) return ""
    if (p.replaces === null) return p.unpacked ? `Load ${p.name}` : `Install ${p.name}`
    return p.replaces === p.version ? `Review ${p.name}` : `Update ${p.name}`
  }
  return (
    <Dialog
      open={pending() !== null}
      onOpenChange={(open) => {
        if (!open && !busy()) void commands.pluginInstallCancel()
      }}
      title={title()}
      size="md"
    >
      <Show when={pending()}>
        {(p) => (
          <div class="flex flex-col gap-4 p-4">
            <Facts>
              <Fact label="Version">{p().version || "Not given"}</Fact>
              <Show when={p().replaces}>{(v) => <Fact label="Installed">{v() || "Not given"}</Fact>}</Show>
              <Fact label="Author">{p().author || "Not given"}</Fact>
              <Fact label="From">{p().package ? "A .bonsaiplugin package" : p().source}</Fact>
              <Show when={p().hash}>
                {(h) => (
                  <Fact label="SHA-256">
                    <span class="break-all font-mono">{h()}</span>
                  </Fact>
                )}
              </Show>
              <Show when={p().unpacked}>
                <Fact label="Loads">In place, and reloads when its files change</Fact>
              </Show>
            </Facts>
            <Show when={p().replaces !== null && p().added.length > 0}>
              <Alert variant="warning" title="It asks for more than before">
                <ul class="font-mono text-sm">
                  <For each={p().added}>{(k) => <li>{k}</li>}</For>
                </ul>
              </Alert>
            </Show>
            <AsksList asks={p().asks} />
            <Show when={p().asks.needsAccounts}>
              <div class="flex flex-col gap-2">
                <span class="text-sm text-fg-muted">Accounts it may use</span>
                <For each={shell.accounts}>
                  {(a) => (
                    <Checkbox checked={accounts().includes(a.id)} onChange={(on) => toggle(a.id, on)}>
                      {`${a.label ?? a.displayName} (@${a.name})`}
                    </Checkbox>
                  )}
                </For>
                <Show when={shell.accounts.length === 0}>
                  <span class="text-sm text-fg-muted">Add an account first. You can allow it later.</span>
                </Show>
              </div>
            </Show>
            <Show when={error()}>
              <Alert variant="danger" title={error()} />
            </Show>
            <div class="flex justify-end gap-2">
              <Button onClick={() => void commands.pluginInstallCancel()} disabled={busy()}>
                Cancel
              </Button>
              <Button variant="primary" onClick={() => void install()} loading={busy()}>
                {p().replaces === null ? (p().unpacked ? "Load" : "Install") : "Agree and turn on"}
              </Button>
            </div>
          </div>
        )}
      </Show>
    </Dialog>
  )
}

/** Permissions, actions by tier, and what was refused. */
function AsksList(props: { asks: Asks }) {
  return (
    <div class="flex flex-col gap-3">
      <div class="flex flex-col gap-1">
        <span class="text-sm text-fg-muted">Permissions</span>
        <Show when={props.asks.permissions.length > 0} fallback={<span class="text-sm">None</span>}>
          <ul class="text-sm">
            <For each={props.asks.permissions}>{(x) => <li>{x.label}</li>}</For>
          </ul>
        </Show>
      </div>
      <For each={byTier(props.asks.actions)}>
        {(g) => (
          <div class="flex flex-col gap-1">
            <span class="text-sm text-fg-muted">
              {`Tier ${g.tier}, ${tiers[g.tier].label.toLowerCase()}: ${tiers[g.tier].description}`}
            </span>
            <ul class="text-sm">
              <For each={g.actions}>{(a) => <li>{a.title}</li>}</For>
            </ul>
          </div>
        )}
      </For>
      <Show when={props.asks.refused.length > 0}>
        <Alert variant="info" title={`It won't get ${props.asks.refused.length} things it asked for`}>
          <ul class="text-sm">
            <For each={props.asks.refused}>
              {(r) => (
                <li>
                  <span class="font-mono">{r.what}</span>: {r.why}
                </li>
              )}
            </For>
          </ul>
        </Alert>
      </Show>
    </div>
  )
}

import { createSignal, type JSX, Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import { afterPurchase } from "~/lib/purchase"
import { shell } from "~/lib/store"
import { Alert, Button, Robux, Stat, showToast, Thumbnail } from "~/lib/ui"

export interface BuyFormProps {
  name: string
  /** "Roblox", a reseller's name, an experience's creator. */
  seller?: string
  price: number
  thumb?: string
  /** Sends the purchase. Roblox's answer arrives as a toast. */
  buy: () => Promise<{ status: "ok"; data: null } | { status: "error"; error: string }>
  onDone: () => void
  /** A line under the price, like "Serial #12". */
  note?: JSX.Element
  /** Pays as this account instead of the active one. Its balance isn't known here. */
  account?: number
}

/** The confirm step before spending Robux: what, from whom, as which account, the price and
 * the balance after. The price on the button is what Roblox charges; nothing is bought if it
 * changed since. */
export function BuyForm(props: BuyFormProps) {
  const [busy, setBusy] = createSignal(false)
  const payer = () => props.account ?? shell.active
  const me = () => shell.accounts.find((a) => a.id === payer())
  const balance = () => (payer() === shell.active ? shell.robux : null)
  const left = () => afterPurchase(balance(), props.price)
  const buy = async () => {
    setBusy(true)
    const r = await props.buy()
    setBusy(false)
    if (r.status === "error") return showToast(r.error)
    props.onDone()
  }
  return (
    <form
      class="flex flex-col gap-3 p-4"
      onSubmit={(e) => {
        e.preventDefault()
        void buy()
      }}
    >
      <div class="flex items-center gap-3">
        <Show when={props.thumb}>{(t) => <Thumbnail class="w-16 shrink-0" alt={props.name} src={t()} />}</Show>
        <div class="flex min-w-0 flex-col">
          <span class="truncate font-medium">{props.name}</span>
          <Show when={props.seller}>{(s) => <span class="truncate text-sm text-fg-muted">from {s()}</span>}</Show>
        </div>
      </div>
      <div class="grid grid-cols-3 gap-3">
        <Stat label="Price">
          <Show when={props.price > 0} fallback="Free">
            <Robux amount={props.price} />
          </Show>
        </Stat>
        <Stat label="Balance">
          <Robux amount={balance()} />
        </Stat>
        <Stat label="After">
          <Robux amount={left().after} />
        </Stat>
      </div>
      <Show when={props.note}>
        <span class="text-sm text-fg-muted">{props.note}</span>
      </Show>
      <Show when={me()}>
        {(a) => (
          <span class="text-sm text-fg-muted">
            Paid by {a().label ?? a().displayName} (@{a().name})
          </span>
        )}
      </Show>
      <Show when={left().short > 0}>
        <Alert variant="warning" title={`You need ${left().short.toLocaleString("en-US")} more Robux.`} />
      </Show>
      <div class="flex justify-end gap-2">
        <Button variant="ghost" type="button" onClick={() => props.onDone()}>
          Cancel
        </Button>
        <Show
          when={left().short === 0}
          fallback={
            <Button
              variant="primary"
              type="button"
              icon="external"
              onClick={() => {
                void run("entity.open_website", "section:robux")
                props.onDone()
              }}
            >
              Buy Robux on website
            </Button>
          }
        >
          <Button variant="primary" type="submit" disabled={busy()}>
            <Show when={props.price > 0} fallback="Get for free">
              Buy for <Robux amount={props.price} />
            </Show>
          </Button>
        </Show>
      </div>
    </form>
  )
}

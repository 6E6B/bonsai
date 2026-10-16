import { type JSX, Show } from "solid-js"
import { assetIcon } from "~/lib/asset-icon"
import { run } from "~/lib/commands/registry"
import { date } from "~/lib/format"
import { commands, type InspectView } from "~/lib/ipc/bindings"
import { itemTarget } from "~/lib/menus"
import { useLive } from "~/lib/store"
import { thumbUrl } from "~/lib/thumb"
import {
  Button,
  Clamp,
  Fact,
  FactGroup,
  Facts,
  IconButton,
  Robux,
  SectionHeader,
  Skeleton,
  TextLink,
  Thumbnail,
  VerifiedBadge,
} from "~/lib/ui"

/** An item or bundle to preview: `item:5`, `bundle:5` or a bare id, which is an item. */
export function parseItemTarget(arg: string | null): { id: number; bundle: boolean } | null {
  const m = /^(?:(item|bundle):)?(\d+)$/.exec(arg?.trim() ?? "")
  return m ? { id: Number(m[2]), bundle: m[1] === "bundle" } : null
}

/** Ctrl+click previews in a side panel; a plain click opens. */
export const wantsPreview = (e: MouseEvent) => e.ctrlKey || e.metaKey

export function PreviewHeader(props: { title: string; onClose: () => void }) {
  return (
    <SectionHeader
      title={props.title}
      size="section"
      actions={<IconButton icon="x" label="Close details" size="sm" variant="ghost" onClick={props.onClose} />}
    />
  )
}

/** The ID with a copy button. `target` is what `entity.copy_id` takes, like `item:5`. */
export function IdFact(props: { id: number; target: string }) {
  return (
    <Fact label="ID">
      <span class="flex items-center gap-1">
        {props.id}
        <IconButton
          icon="copy"
          label="Copy ID"
          size="sm"
          variant="ghost"
          onClick={() => run("entity.copy_id", props.target)}
        />
      </span>
    </Fact>
  )
}

export function PreviewDescription(props: { text: string }) {
  return (
    <Show when={props.text.trim()}>
      <Clamp lines={8}>
        <p class="whitespace-pre-line text-sm">{props.text}</p>
      </Clamp>
    </Show>
  )
}

const waiting = (): InspectView => ({
  loading: true,
  error: null,
  name: "",
  description: "",
  creator: "",
  creatorId: 0,
  creatorIsGroup: false,
  creatorVerified: false,
  typeName: "",
  limited: null,
  priceLabel: "",
  lowestResale: null,
  created: "",
  inCatalog: false,
})

/** One item's or bundle's details for a side panel: catalog items, and assets the catalog
 * doesn't list (decals, audio, models, plugins...). Screens add their own facts and buttons. */
export function ItemPreview(props: {
  id: number
  bundle: boolean
  /** Picks the fallback icon before the details load. */
  assetType?: number | null
  /** A name to show until the details load. */
  name?: string
  /** Extra rows after the price, like serial or RAP. */
  facts?: JSX.Element
  /** Buttons before "Open page". */
  actions?: JSX.Element
  /** Buttons after "Open page". */
  trailing?: JSX.Element
  onClose: () => void
}) {
  const v = useLive(
    () => [props.id, props.bundle] as const,
    ([id, b]) => commands.inventoryInspect(id, b),
    waiting,
  )
  const target = () => itemTarget(props.id, props.bundle)
  const name = () => v.name || props.name || ""
  return (
    <>
      <PreviewHeader title={name() || "Item"} onClose={props.onClose} />
      <Thumbnail
        class="w-full"
        alt={name()}
        src={thumbUrl(props.bundle ? "bundle" : "asset", props.id)}
        fallback={props.bundle ? "avatar" : assetIcon(props.assetType ?? undefined)}
      />
      <Show when={v.error}>{(e) => <span class="text-sm text-danger">{e()}</span>}</Show>
      <Show when={!v.loading || v.name} fallback={<Skeleton class="h-32" />}>
        <Facts>
          <FactGroup>
            <Show when={v.creator}>
              <Fact label="Creator">
                <span class="flex items-center gap-1">
                  <TextLink
                    tone={v.creatorVerified ? "verified" : "name"}
                    onClick={() => run(v.creatorIsGroup ? "group.open" : "user.open", String(v.creatorId))}
                  >
                    {v.creator}
                  </TextLink>
                  <Show when={v.creatorVerified}>
                    <VerifiedBadge />
                  </Show>
                </span>
              </Fact>
            </Show>
            <Show when={v.typeName}>
              <Fact label="Type">{v.typeName}</Fact>
            </Show>
            <IdFact id={props.id} target={target()} />
            <Show when={v.limited}>{(l) => <Fact label="Limited">{l()}</Fact>}</Show>
            <Show when={v.priceLabel}>
              <Fact label="Price">{v.priceLabel}</Fact>
            </Show>
            <Show when={v.lowestResale}>
              {(r) => (
                <Fact label="Lowest resale">
                  <Robux amount={r()} />
                </Fact>
              )}
            </Show>
            {props.facts}
            <Show when={Date.parse(v.created) || null}>{(ms) => <Fact label="Created">{date(ms())}</Fact>}</Show>
          </FactGroup>
        </Facts>
        <PreviewDescription text={v.description} />
      </Show>
      <div class="flex flex-wrap gap-2">
        {props.actions}
        <Button icon="marketplace" onClick={() => run("market.open", target())}>
          Open page
        </Button>
        {props.trailing}
      </div>
    </>
  )
}

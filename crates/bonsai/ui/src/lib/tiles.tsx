import { For, Match, Show, Switch } from "solid-js"
import { assetIcon } from "~/lib/asset-icon"
import { run } from "~/lib/commands/registry"
import { compact, percent } from "~/lib/format"
import type { GameCard, ValueTag } from "~/lib/ipc/bindings"
import { wantsPreview } from "~/lib/item-preview"
import { gameMenu, type ItemMenuExtras, itemMenu, itemTarget, joinUser } from "~/lib/menus"
import { isSoundHat } from "~/lib/sound-hats"
import { thumbUrl } from "~/lib/thumb"
import {
  Badge,
  Button,
  ContextMenu,
  Icon,
  ListItem,
  type MenuEntry,
  Robux,
  Skeleton,
  Thumbnail,
  Tile,
  Tooltip,
} from "~/lib/ui"
import { ValueAmount, ValueMarker } from "~/lib/value-tags"

/** Grid classes for tiles, so every screen's tiles are the same size. `tileGrid` for square
 * tiles, `wideTileGrid` for 16:9 game thumbnails. */
export const tileGrid = "grid grid-cols-[repeat(auto-fill,minmax(8rem,1fr))] gap-x-3 gap-y-4"
export const wideTileGrid = "grid grid-cols-[repeat(auto-fill,minmax(12rem,1fr))] gap-x-3 gap-y-4"
/** `minItemWidth` for a `VirtualGrid` of square tiles; matches `tileGrid`. */
export const TILE_WIDTH = 128

/** A tile's shape while it loads: the image, a title line and a subtitle line. */
export function TileSkeleton(props: { shape?: "square" | "wide" }) {
  return (
    <div class="flex flex-col gap-1.5">
      <Skeleton class={`${props.shape === "wide" ? "aspect-video" : "aspect-square"} w-full rounded-md`} />
      <Skeleton class="mt-0.5 h-3.5 w-4/5" />
      <Skeleton class="h-3 w-1/2" />
    </div>
  )
}

/** Placeholder sections shaped like loaded shelves: a heading over one row of tiles. The row
 * clips, so it fills any width with whole tiles. */
export function ShelfSkeleton(props: { shelves?: number; tiles?: number }) {
  return (
    <div class="flex flex-col gap-6" aria-hidden="true">
      <For each={Array.from({ length: props.shelves ?? 3 })}>
        {() => (
          <section class="flex flex-col gap-2">
            <Skeleton class="h-5 w-40" />
            <div class={`${tileGrid} grid-rows-[auto] auto-rows-[0] gap-y-0 overflow-hidden`}>
              <For each={Array.from({ length: props.tiles ?? 12 })}>{() => <TileSkeleton />}</For>
            </div>
          </section>
        )}
      </For>
    </div>
  )
}

/** A game in a grid. Click opens it; right-click for join and copy, plus `menu` rows. */
export function GameTile(props: { game: GameCard; shape?: "square" | "wide"; menu?: MenuEntry[] }) {
  return (
    <ContextMenu items={[...gameMenu(props.game.placeId), ...(props.menu ?? [])]}>
      <Tile
        title={props.game.name}
        subtitle={
          <span class="flex items-center gap-2">
            <Show when={props.game.rating}>
              {(r) => (
                <Tooltip content="Rating" class="items-center gap-1">
                  <Icon name="thumbs-up" size={12} />
                  {percent(r() / 100)}
                </Tooltip>
              )}
            </Show>
            <Tooltip content="Playing" class="items-center gap-1">
              <Icon name="user" size={12} />
              {compact(props.game.playing)}
            </Tooltip>
          </span>
        }
        src={props.game.universeId ? thumbUrl("game_icon", props.game.universeId, true) : undefined}
        shape={props.shape ?? "square"}
        fallback="game"
        onClick={() => run("game.open", String(props.game.placeId))}
      />
    </ContextMenu>
  )
}

/** An experience's icon from its universe id, or the fallback when it is unknown. */
export function GameIcon(props: { universeId: number | null; class?: string }) {
  return (
    <Thumbnail
      class={props.class ?? "w-8 rounded-md"}
      alt=""
      src={props.universeId ? thumbUrl("game_icon", props.universeId) : undefined}
      fallback="game"
    />
  )
}

/** The experience a user is in: click opens it, Join follows them into their server.
 * Right-click for the experience's menu. */
export function PlayingRow(props: {
  userId: number
  placeId: number
  universeId: number | null
  name: string
  /** Hide Join, e.g. on your own profile. */
  noJoin?: boolean
  class?: string
}) {
  return (
    <ContextMenu items={gameMenu(props.placeId)}>
      <ListItem
        class={props.class}
        leading={<GameIcon universeId={props.universeId} />}
        title={props.name}
        subtitle="Playing now"
        onClick={() => run("game.open", String(props.placeId))}
        actions={
          props.noJoin ? undefined : (
            <Button size="sm" variant="primary" icon="play" onClick={() => run("game.join", joinUser(props.userId))}>
              Join
            </Button>
          )
        }
      />
    </ContextMenu>
  )
}

/** What an item tile shows. `ItemCard` from the backend has these. */
export interface TileItem {
  id: number
  name: string
  price: number | null
  /** Copies owned, for inventory tiles. */
  copies?: number
  bundle?: boolean
  /** Adds template downloads to the menu for clothing. */
  assetType?: number
  limited?: boolean
  serial?: number | null
  rap?: number | null
  /** Rolimons' data, when values are on. */
  values?: ValueTag | null
}

/** The flair on a sound hat's tile. Check `isSoundHat` first. */
export function SoundHatBadge() {
  return (
    <Badge variant="purple">
      <Icon name="audio-fill" size={12} class="-ml-0.5 mr-1" />
      Sound Hat
    </Badge>
  )
}

/** A music-notes icon with a tooltip, for spots too small for `SoundHatBadge`. Renders
 * nothing unless the item is a sound hat. */
export function SoundHatMark(props: { id: number; bundle?: boolean; size?: number; class?: string }) {
  return (
    <Show when={isSoundHat(props.id, props.bundle)}>
      <Tooltip content="Sound Hat" class={`inline-flex shrink-0 text-tag-purple ${props.class ?? ""}`}>
        <Icon name="audio-fill" size={props.size ?? 14} aria-label="Sound Hat" />
      </Tooltip>
    </Show>
  )
}

/** An item's name that truncates, with its `SoundHatMark` after it. For list rows and cells. */
export function ItemName(props: { name: string; id: number; bundle?: boolean }) {
  return (
    <span class="flex min-w-0 items-center gap-1.5">
      <span class="truncate">{props.name}</span>
      <SoundHatMark id={props.id} bundle={props.bundle} />
    </span>
  )
}

const marker = (t: ValueTag | null | undefined) => !!t && (t.projected || !!t.demand || !!t.trend)

/** A catalog or inventory item in a grid. With values on, limiteds show their value next to
 * RAP, and demand, trend and a projected warning in a chip over the picture. */
export function ItemTile(props: {
  item: TileItem
  selected?: boolean
  onClick?: (e: MouseEvent) => void
  showSerial?: boolean
  /** Show recent average price instead of the list price. */
  showRap?: boolean
  /** This screen's rows in the right-click menu. */
  menu?: ItemMenuExtras
  /** See `Tile`'s `fixed`. */
  fixed?: boolean
}) {
  return (
    <ContextMenu items={itemMenu(props.item.id, props.item.bundle, props.item.assetType, props.menu)}>
      <Tile
        title={props.item.name}
        subtitle={
          <span class="flex items-center gap-2">
            <Show when={props.showRap && props.item.rap} fallback={<Price amount={props.item.price} />}>
              {(rap) => <Robux amount={rap()} />}
            </Show>
            <Show when={props.item.values?.value}>{(v) => <ValueAmount amount={v()} compact />}</Show>
          </span>
        }
        src={thumbUrl(props.item.bundle ? "bundle" : "asset", props.item.id, true)}
        fallback={props.item.bundle ? "avatar" : assetIcon(props.item.assetType)}
        selected={props.selected}
        fixed={props.fixed}
        marker={marker(props.item.values) ? <ValueMarker tag={props.item.values as ValueTag} /> : undefined}
        badge={
          props.item.limited ? (
            <Badge>{props.showSerial && props.item.serial ? `#${props.item.serial}` : "Limited"}</Badge>
          ) : props.showSerial && (props.item.copies ?? 1) > 1 ? (
            <Badge>{`x${props.item.copies}`}</Badge>
          ) : undefined
        }
        flair={isSoundHat(props.item.id, props.item.bundle) ? <SoundHatBadge /> : undefined}
        onClick={
          props.onClick ??
          ((e) => run(wantsPreview(e) ? "market.preview" : "market.open", itemTarget(props.item.id, props.item.bundle)))
        }
      />
    </ContextMenu>
  )
}

/** A catalog price: Robux, "Free" at 0, or "Off sale" when the item can't be bought. */
export function Price(props: { amount: number | null; class?: string }) {
  return (
    <Switch fallback={<Robux class={props.class} amount={props.amount} />}>
      <Match when={props.amount === null}>
        <span class={props.class}>Off sale</span>
      </Match>
      <Match when={props.amount === 0}>
        <span class={props.class}>Free</span>
      </Match>
    </Switch>
  )
}

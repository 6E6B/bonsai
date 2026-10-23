import { Button as KButton } from "@kobalte/core/button"
import { type JSX, Show } from "solid-js"
import type { IconName } from "./icon"
import { Thumbnail } from "./thumbnail"

export interface TileProps {
  title: string
  /** One muted line under the title: creator, player count, price. */
  subtitle?: JSX.Element
  /** A `thumbUrl()`. */
  src?: string
  shape?: "square" | "wide"
  fallback?: IconName
  /** Corner badge over the image, like "Limited" or a serial number. */
  badge?: JSX.Element
  /** A small dark chip in the other top corner, for glanceable signals like demand and trend. */
  marker?: JSX.Element
  /** Ring around the tile, for items being worn or picked for a trade. */
  selected?: boolean
  /** Always as tall as two title lines and a subtitle, so rows of tiles keep one height as
   * their contents change. Short text leaves the space under it, not between lines. */
  fixed?: boolean
  /** Opens the thing. Put other actions in a ContextMenu around the tile. The event says
   * whether Ctrl was held, for screens that preview on Ctrl+click. */
  onClick: (e: MouseEvent) => void
  class?: string
}

/** A picture with a title under it: games, items, outfits, badges, communities. Lay tiles out
 * with a grid, or `VirtualGrid` when there can be many. */
export function Tile(props: TileProps) {
  return (
    <KButton
      data-component="tile"
      data-selected={props.selected ? "" : undefined}
      data-fixed={props.fixed ? "" : undefined}
      class={props.class}
      onClick={props.onClick}
    >
      <span data-slot="media">
        <Thumbnail src={props.src} alt={props.title} shape={props.shape} fallback={props.fallback} />
        <Show when={props.badge}>
          <span data-slot="badge">{props.badge}</span>
        </Show>
        <Show when={props.marker}>
          <span data-slot="marker">{props.marker}</span>
        </Show>
      </span>
      <span data-slot="text">
        <span data-slot="title">{props.title}</span>
        <Show when={props.subtitle}>
          <span data-slot="subtitle">{props.subtitle}</span>
        </Show>
      </span>
    </KButton>
  )
}

import { Image as KImage } from "@kobalte/core/image"
import { type ComponentProps, Show, splitProps } from "solid-js"

import { imageKey } from "~/lib/image-actions"
import { ImageMenu, useImageMenu } from "./menu"

/** Roblox presence. `game` is in an experience, `studio` is in Roblox Studio. */
export type Presence = "offline" | "online" | "game" | "studio"

export interface AvatarProps extends ComponentProps<"span"> {
  /** Name used for the initials fallback and the alt text. */
  name: string
  src?: string
  size?: "xs" | "sm" | "md" | "lg" | "xl"
  /** Shows a presence dot in the bottom-right corner. */
  presence?: Presence
  /** Shows an unread count in the top-right corner. Hidden when 0. */
  count?: number
}

/** A round user or account picture, with initials while it loads or when it fails. */
export function Avatar(props: AvatarProps) {
  const standalone = !useImageMenu()
  const [local, rest] = splitProps(props, ["name", "src", "size", "presence", "count"])
  const initials = () =>
    local.name
      .split(/[\s_]+/)
      .filter(Boolean)
      .slice(0, 2)
      .map((w) => w[0]?.toUpperCase())
      .join("")
  return (
    <ImageMenu src={local.src}>
      <span
        data-image-src={local.src}
        data-image-name={local.name}
        aria-haspopup={standalone && imageKey(local.src) ? "menu" : undefined}
        tabIndex={standalone && imageKey(local.src) ? 0 : undefined}
        data-component="avatar"
        data-size={local.size ?? "md"}
        {...rest}
      >
        <KImage data-slot="frame" fallbackDelay={0}>
          <Show when={local.src}>{(src) => <KImage.Img data-slot="image" src={src()} alt={local.name} />}</Show>
          <KImage.Fallback data-slot="fallback">{initials()}</KImage.Fallback>
        </KImage>
        <Show when={local.presence}>
          {(p) => <span data-slot="presence" data-presence={p()} role="img" aria-label={p()} />}
        </Show>
        <Show when={local.count}>{(n) => <span data-slot="count">{n() > 99 ? "99+" : n()}</span>}</Show>
      </span>
    </ImageMenu>
  )
}

import { Image as KImage } from "@kobalte/core/image"
import { type ComponentProps, Show, splitProps } from "solid-js"
import { imageKey } from "~/lib/image-actions"
import { Icon, type IconName } from "./icon"
import { ImageMenu, useImageMenu } from "./menu"

export interface ThumbnailProps extends ComponentProps<"span"> {
  /** A `thumbUrl()`. Without one, or while loading, the fallback icon shows. */
  src?: string
  alt: string
  /** `square` for icons and items, `wide` (16:9) for game thumbnails. */
  shape?: "square" | "wide"
  /** Icon shown when there is no image. */
  fallback?: IconName
}

/** An image from `thumb://` with a fixed aspect ratio. Set the width with Tailwind; the height
 * follows. */
export function Thumbnail(props: ThumbnailProps) {
  const standalone = !useImageMenu()
  const [local, rest] = splitProps(props, ["src", "alt", "shape", "fallback"])
  return (
    <ImageMenu src={local.src}>
      <span
        data-image-src={local.src}
        data-image-name={local.alt}
        aria-haspopup={standalone && imageKey(local.src) ? "menu" : undefined}
        tabIndex={standalone && imageKey(local.src) ? 0 : undefined}
        data-component="thumbnail"
        data-shape={local.shape ?? "square"}
        {...rest}
      >
        <KImage data-slot="frame" fallbackDelay={0}>
          <Show when={local.src}>{(src) => <KImage.Img data-slot="image" src={src()} alt={local.alt} />}</Show>
          <KImage.Fallback data-slot="fallback">
            <Icon name={local.fallback ?? "game"} size={20} />
          </KImage.Fallback>
        </KImage>
      </span>
    </ImageMenu>
  )
}

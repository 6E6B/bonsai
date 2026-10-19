import { Image as KImage } from "@kobalte/core/image"
import { type JSX, Show } from "solid-js"
import { Icon } from "./icon"

export interface FigureProps {
  /** An image URL from one of the app's protocols. Without one, or while loading, an icon shows. */
  src?: string
  alt: string
  /** The image's size, so the frame keeps its shape before it loads. */
  width?: number
  height?: number
  caption?: JSX.Element
  class?: string
}

/** A picture of any shape with an optional caption under it, loaded when scrolled near. For
 * article pictures; fixed-shape thumbnails use `Thumbnail`. */
export function Figure(props: FigureProps) {
  const ratio = () => (props.width && props.height ? `${props.width} / ${props.height}` : undefined)
  return (
    <figure data-component="figure" class={props.class}>
      <KImage data-slot="frame" fallbackDelay={0} style={{ "aspect-ratio": ratio() }}>
        <Show when={props.src}>
          {(src) => <KImage.Img data-slot="image" src={src()} alt={props.alt} loading="lazy" decoding="async" />}
        </Show>
        <KImage.Fallback data-slot="fallback">
          <Icon name="image" size={20} />
        </KImage.Fallback>
      </KImage>
      <Show when={props.caption}>
        <figcaption data-slot="caption">{props.caption}</figcaption>
      </Show>
    </figure>
  )
}

import { Image as KImage } from "@kobalte/core/image"
import type { JSX } from "solid-js"
import { Show } from "solid-js"

export interface HeroProps {
  /** An `Avatar`, usually `size="xl"`. */
  avatar: JSX.Element
  /** The name, with any badges after it. Wrap long text in a `truncate` span. */
  title: JSX.Element
  /** A muted line under the title, like the @username. */
  subtitle?: JSX.Element
  /** Buttons at the far end of the title row. */
  actions?: JSX.Element
  /** `Stat`s in a row under the title, split by lines. */
  stats?: JSX.Element
  /** A `HeroBanner` across the top. */
  banner?: JSX.Element
  class?: string
}

/** The subject at the top of a screen, like a user on their profile: a card with the avatar,
 * names and actions, and their figures under them. */
export function Hero(props: HeroProps) {
  return (
    <section data-component="hero" class={props.class}>
      {props.banner}
      <div data-slot="body">
        <div data-slot="avatar">{props.avatar}</div>
        <div data-slot="head">
          <div data-slot="names">
            <h1 data-slot="title">{props.title}</h1>
            <Show when={props.subtitle}>
              <span data-slot="subtitle">{props.subtitle}</span>
            </Show>
          </div>
          <Show when={props.actions}>
            <div data-slot="actions">{props.actions}</div>
          </Show>
        </div>
        <Show when={props.stats}>
          <dl data-slot="stats">{props.stats}</dl>
        </Show>
      </div>
    </section>
  )
}

export interface HeroBannerProps {
  /** A wide `thumbUrl()`, like a game screenshot, cropped to fill and darkened toward the
   * bottom. Without one the band is plain. */
  src?: string
  /** Small line above the title ("Playing now"). */
  label: string
  title: JSX.Element
  /** A button after the text, like Join. */
  action?: JSX.Element
}

/** A band across the top of a `Hero` for a live status, like the game someone is in. */
export function HeroBanner(props: HeroBannerProps) {
  return (
    <div data-slot="banner">
      <KImage data-slot="backdrop" fallbackDelay={0}>
        <Show when={props.src}>
          {(src) => <KImage.Img data-slot="backdrop-image" src={src()} alt="" decoding="async" />}
        </Show>
      </KImage>
      <div data-slot="banner-content">
        <div data-slot="banner-text">
          <span data-slot="banner-label">{props.label}</span>
          <span data-slot="banner-title">{props.title}</span>
        </div>
        {props.action}
      </div>
    </div>
  )
}

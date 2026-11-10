import { For, type JSX, Show } from "solid-js"
import { Icon, type IconName } from "./icon"
import { Thumbnail } from "./thumbnail"

export interface ShowcaseImage {
  /** A wide `thumbUrl()`. */
  src?: string
  /** Accessible name for the picture and its button in the strip. */
  label: string
}

/** One figure for the narrow banner, like 16,400 playing. */
export interface ShowcaseFigure {
  value: string
  label: string
  icon?: IconName
}

export interface ShowcaseProps {
  /** The pictures. The shown one fills the banner; with more than one, a strip of them sits in
   * the bottom-right corner (top-left when narrow) and arrows step through them. Empty shows a plain band. */
  images: ShowcaseImage[]
  /** Zero-based index of the shown picture. */
  index: number
  onSelect: (index: number) => void
  /** Short text in the top-left corner of the picture, like a trailer's title. May hold a
   * `TextLink`. */
  caption?: JSX.Element
  captionIcon?: IconName
  /** Short words above the title on the picture: genre, maturity. */
  tags?: string[]
  title: JSX.Element
  /** A square picture before the title, like a community's icon: a `Thumbnail`. */
  emblem?: JSX.Element
  /** Nothing to show: a short band instead of the full-height stage. */
  plain?: boolean
  /** The line under the title on the picture: the creator. */
  byline?: JSX.Element
  /** Figures shown in a row under the byline on the picture, in place of `stats`, when the
   * card is too narrow for both the actions and the stats on one bar. */
  summary?: ShowcaseFigure[]
  /** Buttons on the bar under the picture, starting with the main action. */
  actions?: JSX.Element
  /** `Stat`s at the far end of the bar, split by lines. */
  stats?: JSX.Element
  class?: string
}

/** How many pictures the strip shows at once. It follows the shown one. */
const STRIP = 5

/** An entity's pictures as a wide banner with its name laid over them, and a bar of actions
 * and figures under it. The top of an experience's page. */
export function Showcase(props: ShowcaseProps) {
  const count = () => props.images.length
  const shown = () => props.images[props.index] ?? props.images[0]
  const step = (by: number) => props.onSelect((props.index + by + count()) % count())
  const strip = () => {
    const start = Math.max(0, Math.min(props.index - 1, count() - STRIP))
    return props.images.slice(start, start + STRIP).map((image, i) => ({ image, i: start + i }))
  }
  return (
    <section data-component="showcase" data-plain={props.plain ? "" : undefined} class={props.class}>
      <div data-slot="stage">
        {/* Keyed so a new picture remounts and fades in. */}
        <Show when={shown()} keyed>
          {(image) => <Thumbnail data-slot="backdrop" shape="wide" src={image.src} alt={image.label} />}
        </Show>
        <div data-slot="overlay">
          <div data-slot="top">
            <Show when={props.caption}>
              <span data-slot="caption">
                <Show when={props.captionIcon}>{(icon) => <Icon name={icon()} size={12} />}</Show>
                <span data-slot="caption-text">{props.caption}</span>
              </span>
            </Show>
            <Show when={count() > STRIP}>
              <span data-slot="count">
                {props.index + 1} / {count()}
              </span>
            </Show>
          </div>
          <div data-slot="foot">
            <div data-slot="heading">
              <Show when={props.emblem}>
                <div data-slot="emblem">{props.emblem}</div>
              </Show>
              <div data-slot="names">
                <Show when={props.tags?.length}>
                  <div data-slot="tags">
                    <For each={props.tags}>{(t) => <span data-slot="tag">{t}</span>}</For>
                  </div>
                </Show>
                <h1 data-slot="title">{props.title}</h1>
                <Show when={props.byline}>
                  <div data-slot="byline">{props.byline}</div>
                </Show>
                <Show when={props.summary?.length}>
                  <ul data-slot="summary">
                    <For each={props.summary}>
                      {(f) => (
                        <li data-slot="figure">
                          <Show when={f.icon}>{(icon) => <Icon name={icon()} size={14} />}</Show>
                          <span data-slot="figure-value">{f.value}</span>
                          <span data-slot="figure-label">{f.label}</span>
                        </li>
                      )}
                    </For>
                  </ul>
                </Show>
              </div>
            </div>
            <Show when={count() > 1}>
              <div data-slot="strip">
                <For each={strip()}>
                  {(t) => (
                    <button
                      type="button"
                      data-slot="thumb"
                      aria-label={t.image.label}
                      aria-pressed={props.index === t.i}
                      onClick={() => props.onSelect(t.i)}
                    >
                      <Thumbnail shape="wide" src={t.image.src} alt="" />
                    </button>
                  )}
                </For>
              </div>
            </Show>
          </div>
        </div>
        <Show when={count() > 1}>
          <button type="button" data-slot="prev" aria-label="Previous image" onClick={() => step(-1)}>
            <Icon name="chevron-left" size={16} />
          </button>
          <button type="button" data-slot="next" aria-label="Next image" onClick={() => step(1)}>
            <Icon name="chevron-right" size={16} />
          </button>
        </Show>
      </div>
      <Show when={props.actions || props.stats}>
        <div data-slot="bar">
          <div data-slot="actions">{props.actions}</div>
          <Show when={props.stats}>
            <dl data-slot="stats">{props.stats}</dl>
          </Show>
        </div>
      </Show>
    </section>
  )
}

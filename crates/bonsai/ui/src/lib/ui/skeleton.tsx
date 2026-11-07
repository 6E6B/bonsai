export interface SkeletonProps {
  /** Size and shape with Tailwind: `h-4 w-32`, `size-10 rounded-full`. */
  class?: string
}

/** A placeholder block shown while something loads for the first time. Prefer showing cached
 * data; use this only when there is none yet.
 *
 * A plain element on purpose: Kobalte's Skeleton writes an inline `width: 100%; height: auto`
 * that overrides the size classes, which left most placeholders 0px tall. */
export function Skeleton(props: SkeletonProps) {
  return <div data-component="skeleton" data-animate aria-hidden="true" class={props.class} />
}

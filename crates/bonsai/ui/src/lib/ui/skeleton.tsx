import { Skeleton as KSkeleton } from "@kobalte/core/skeleton"

export interface SkeletonProps {
  /** Size and shape with Tailwind: `h-4 w-32`, `size-10 rounded-full`. */
  class?: string
}

/** A placeholder block shown while something loads for the first time. Prefer showing cached
 * data; use this only when there is none yet. */
export function Skeleton(props: SkeletonProps) {
  return <KSkeleton data-component="skeleton" class={props.class} visible animate />
}

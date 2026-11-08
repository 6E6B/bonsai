import { For } from "solid-js"
import { Card, Skeleton } from "~/lib/ui"

/** The item and pass pages while they load: the same thumbnail, title, price, buttons and facts
 * card as the loaded page. */
export function DetailSkeleton() {
  return (
    <div class="flex flex-wrap gap-6" aria-hidden="true">
      <Skeleton class="aspect-square w-72 rounded-md" />
      <div class="flex min-w-64 flex-1 flex-col gap-3">
        <div class="flex flex-col gap-2">
          <Skeleton class="h-7 w-2/3 max-w-md rounded-md" />
          <Skeleton class="h-4 w-40" />
        </div>
        <Skeleton class="h-6 w-24" />
        <div class="flex gap-1.5">
          <Skeleton class="h-8 w-36 rounded-md" />
          <Skeleton class="h-8 w-20 rounded-md" />
        </div>
        <div class="flex max-w-prose flex-col gap-1.5">
          <Skeleton class="h-4 w-full" />
          <Skeleton class="h-4 w-5/6" />
        </div>
        <Card class="flex flex-wrap gap-x-10 gap-y-3 p-3">
          <For each={[0, 1, 2, 3]}>
            {() => (
              <div class="flex flex-col gap-1.5">
                <Skeleton class="h-3 w-14" />
                <Skeleton class="h-4 w-20" />
              </div>
            )}
          </For>
        </Card>
      </div>
    </div>
  )
}

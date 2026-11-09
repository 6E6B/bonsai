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

/** The item page while it loads: the stage, the buy box beside it and the facts below. */
export function ItemSkeleton() {
  return (
    <div class="@container" aria-hidden="true">
      <div class="flex flex-col gap-4 @3xl:grid @3xl:grid-cols-[minmax(0,1fr)_21rem] @3xl:items-start">
        <div class="contents @3xl:flex @3xl:min-w-0 @3xl:flex-col @3xl:gap-4">
          <Skeleton class="order-1 aspect-[4/3] max-h-[30rem] w-full rounded-lg" />
          <Card class="order-3 flex flex-col gap-1.5 px-4 py-3.5">
            <Skeleton class="h-3.5 w-20" />
            <Skeleton class="h-4 w-full" />
            <Skeleton class="h-4 w-4/6" />
          </Card>
        </div>
        <Card class="order-2 flex flex-col gap-3 px-4 py-3.5">
          <Skeleton class="h-5 w-16 rounded-full" />
          <Skeleton class="h-7 w-4/5 rounded-md" />
          <Skeleton class="h-4 w-36" />
          <Skeleton class="mt-2 h-3 w-12" />
          <Skeleton class="h-8 w-28 rounded-md" />
          <div class="flex gap-1.5">
            <Skeleton class="h-10 flex-1 rounded-md" />
            <Skeleton class="h-10 w-24 rounded-md" />
          </div>
          <div class="flex gap-1.5">
            <Skeleton class="h-8 flex-1 rounded-md" />
            <Skeleton class="h-8 flex-1 rounded-md" />
          </div>
        </Card>
      </div>
    </div>
  )
}

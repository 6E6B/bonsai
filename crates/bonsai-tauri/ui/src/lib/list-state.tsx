import { createEffect, createSignal, For, type JSX, onCleanup, Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import type { List } from "~/lib/ipc/bindings"
import { Alert, Button, EmptyState, type IconName, Skeleton } from "~/lib/ui"

/** A `List` that is still loading: the initial value for `useLive`. */
export function emptyList<T>(): List<T> {
  return { loading: true, error: null, items: [], more: false }
}

/** Shows a `List`: skeletons while the first page loads, the error, an empty state, or the
 * children with a "Load more" button when more pages exist. */
export function ListState<T>(props: {
  list: List<T>
  empty: string
  icon: IconName
  description?: string
  /** The `list.more` argument that loads the next page. */
  more?: string
  /** Also load the next page when "Load more" scrolls near the view, for long lists. */
  autoMore?: boolean
  children: JSX.Element
}) {
  const [near, setNear] = createSignal(false)
  const watch = (el: HTMLElement) => {
    if (!props.autoMore) return
    const io = new IntersectionObserver(([e]) => setNear(e.isIntersecting), { rootMargin: "400px" })
    io.observe(el)
    onCleanup(() => io.disconnect())
  }
  // Runs again when a page lands, so a short page that leaves the button in view loads the next.
  createEffect(() => {
    if (near() && props.more && props.list.more && !props.list.loading) void run("list.more", props.more)
  })
  return (
    <Show
      when={props.list.items.length > 0}
      fallback={
        <Show
          when={!props.list.loading}
          fallback={
            <div class="flex flex-col gap-2">
              <For each={[1, 2, 3]}>{() => <Skeleton class="h-12 w-full rounded-md" />}</For>
            </div>
          }
        >
          <Show
            when={props.list.error}
            fallback={<EmptyState icon={props.icon} title={props.empty} description={props.description} />}
          >
            {(e) => (
              <Alert variant="danger" title="Couldn't load this">
                {e()}
              </Alert>
            )}
          </Show>
        </Show>
      }
    >
      {props.children}
      <Show when={props.list.more && props.more}>
        {(key) => (
          <div ref={watch} class="self-start">
            <Button variant="ghost" disabled={props.list.loading} onClick={() => run("list.more", key())}>
              Load more
            </Button>
          </div>
        )}
      </Show>
    </Show>
  )
}

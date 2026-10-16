import { type ParentProps, Show } from "solid-js"
import { Icon, type IconName } from "./icon"

export interface EmptyStateProps extends ParentProps {
  title: string
  description?: string
  icon?: IconName
  class?: string
}

/** What a list or screen shows when it has loaded and is empty. Put actions in `children`. */
export function EmptyState(props: EmptyStateProps) {
  return (
    <div data-component="empty-state" class={props.class}>
      <Show when={props.icon}>
        {(n) => (
          <span data-slot="icon">
            <Icon name={n()} size={24} />
          </span>
        )}
      </Show>
      <p data-slot="title">{props.title}</p>
      <Show when={props.description}>
        <p data-slot="description">{props.description}</p>
      </Show>
      <Show when={props.children}>
        <div data-slot="actions">{props.children}</div>
      </Show>
    </div>
  )
}

import { run } from "~/lib/commands/registry"
import { Button, EmptyState } from "~/lib/ui"

/** What streamer mode covers until the user asks to see it. */
const covers = {
  friends: { title: "Friends are hidden in streamer mode", button: "Show friends", command: "streamer.show_friends" },
  messages: {
    title: "Messages are hidden in streamer mode",
    button: "Show messages",
    command: "streamer.show_messages",
  },
} as const

/** Stands in for a list streamer mode hides. Showing it lasts until streamer mode turns off. */
export function StreamerCover(props: { what: keyof typeof covers; class?: string }) {
  const c = () => covers[props.what]
  return (
    <EmptyState
      class={props.class}
      icon="eye"
      title={c().title}
      description="Showing it keeps it visible until streamer mode turns off."
    >
      <Button onClick={() => run(c().command)}>{c().button}</Button>
    </EmptyState>
  )
}

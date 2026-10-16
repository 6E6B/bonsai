import { type ComponentProps, For, splitProps } from "solid-js"

export interface KbdProps extends ComponentProps<"span"> {
  /** One entry per key cap, like `["Ctrl", "K"]`. */
  keys: string[]
}

/** Shows a keyboard shortcut. */
export function Kbd(props: KbdProps) {
  const [local, rest] = splitProps(props, ["keys"])
  return (
    <span data-component="kbd" {...rest}>
      <For each={local.keys}>{(k) => <kbd data-slot="key">{k}</kbd>}</For>
    </span>
  )
}
